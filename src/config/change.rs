//! Read the key file for a change, and write it.

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use std::fmt;

use super::fs::TEMP_PREFIX;
use super::limit::{DEFAULT_COST_LIMIT, DEFAULT_SIZE_LIMIT};
use super::model::{DEFAULT_BASE_URL, DEFAULT_MODEL, DEFAULT_RETRY_WAIT, DEFAULT_TIMEOUT};
use super::{
    ConfigError, Env, File, Fs, StrictFile, has_yaml_content, open_key_file, parse_loose,
    read_open_file,
};

/// The non-secret settings that the file can hold under `config:`, in the
/// order of the file, each with its default (ADR-0029). A later story that
/// adds a setting adds 1 row here. STORY-85 writes the comment lines from it.
pub(crate) fn setting_table() -> [(&'static str, String); 6] {
    [
        ("base-url", DEFAULT_BASE_URL.to_owned()),
        ("model", DEFAULT_MODEL.to_owned()),
        ("timeout", DEFAULT_TIMEOUT.as_secs().to_string()),
        ("retry-wait", DEFAULT_RETRY_WAIT.as_secs().to_string()),
        ("max-scored", DEFAULT_COST_LIMIT.to_string()),
        ("max-chars", DEFAULT_SIZE_LIMIT.to_string()),
    ]
}

/// The 8 keys that the file can hold, as the text of a list: `config.api-key,
/// config.gateway-api-key, ..., and config.max-chars`.
pub(crate) fn key_list_text() -> String {
    let mut names = vec!["api-key", "gateway-api-key"];
    names.extend(setting_table().map(|(name, _)| name));
    let (last, first) = names.split_last().expect("the list has keys");
    let first: Vec<String> = first.iter().map(|name| format!("config.{name}")).collect();
    format!("{}, and config.{last}", first.join(", "))
}

/// The values of the file, to write them again. A blank or empty value counts
/// as not set. `Debug` shows only which values are set, because two of them
/// are keys (CV-6). There is no `Display` and no `Serialize`.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct FileValues {
    pub api_key: Option<String>,
    pub gateway_api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: Option<String>,
    pub timeout: Option<String>,
    pub retry_wait: Option<String>,
    pub max_scored: Option<String>,
    pub max_chars: Option<String>,
}

impl FileValues {
    /// Each key of the file with its value, in the order of the file.
    fn entries(&self) -> [(&'static str, &Option<String>); 8] {
        [
            ("api-key", &self.api_key),
            ("gateway-api-key", &self.gateway_api_key),
            ("base-url", &self.base_url),
            ("model", &self.model),
            ("timeout", &self.timeout),
            ("retry-wait", &self.retry_wait),
            ("max-scored", &self.max_scored),
            ("max-chars", &self.max_chars),
        ]
    }

    /// The values of a parse, as the file gives them: no trim and no check.
    fn of(file: &File) -> FileValues {
        let Some(section) = &file.config else {
            return FileValues::default();
        };
        let kept = |value: &Option<String>| {
            value
                .as_ref()
                .filter(|text| !text.trim().is_empty())
                .cloned()
        };
        FileValues {
            api_key: kept(&section.api_key),
            gateway_api_key: kept(&section.gateway_api_key),
            base_url: kept(&section.base_url),
            model: kept(&section.model),
            timeout: kept(&section.timeout),
            retry_wait: kept(&section.retry_wait),
            max_scored: kept(&section.max_scored),
            max_chars: kept(&section.max_chars),
        }
    }

    /// True when no value is set.
    pub fn is_empty(&self) -> bool {
        *self == FileValues::default()
    }
}

impl fmt::Debug for FileValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let set = |value: &Option<String>| if value.is_some() { "set" } else { "not set" };
        f.debug_struct("FileValues")
            .field("api_key", &set(&self.api_key))
            .field("gateway_api_key", &set(&self.gateway_api_key))
            .field("base_url", &set(&self.base_url))
            .field("model", &set(&self.model))
            .field("timeout", &set(&self.timeout))
            .field("retry_wait", &set(&self.retry_wait))
            .field("max_scored", &set(&self.max_scored))
            .field("max_chars", &set(&self.max_chars))
            .finish()
    }
}

/// What `--setup` and `--forget-key` find in the key file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeState {
    /// The file exists.
    pub exists: bool,
    /// The mode of the open file. `0` when there is no file.
    pub mode: u32,
    /// The values of the file.
    pub values: FileValues,
}

/// Read the key file for a change. Each error stores nothing and changes
/// nothing. This function does not check the mode (`--setup` does).
pub fn load_for_change(env: &Env, fs: &dyn Fs) -> Result<(PathBuf, ChangeState), ConfigError> {
    // 1. No home.
    let path = env.key_path().ok_or(ConfigError::NoHome)?;
    // 2. A symbolic link (decision 20).
    if fs
        .is_symlink(&path)
        .map_err(|_| ConfigError::KeyFileUnreadable { path: path.clone() })?
    {
        return Err(ConfigError::KeyFileIsLink { path });
    }
    // 3. Open and read from the same handle.
    let Some(mut file) = open_key_file(fs, &path)? else {
        let state = ChangeState {
            exists: false,
            mode: 0,
            values: FileValues::default(),
        };
        return Ok((path, state));
    };
    let mode = file
        .mode()
        .map_err(|_| ConfigError::KeyFileUnreadable { path: path.clone() })?;
    let text = read_open_file(file.as_mut(), &path)?;
    // 4. Parse two times (decision 18). The loose parse finds a YAML error.
    //    When it passes, an error of the strict parse means an unknown key.
    let loose = parse_loose(&text, &path)?;
    if has_yaml_content(&text) && serde_saphyr::from_str::<StrictFile>(&text).is_err() {
        return Err(ConfigError::KeyFileHasOtherKeys { path });
    }
    let state = ChangeState {
        exists: true,
        mode,
        values: FileValues::of(&loose),
    };
    Ok((path, state))
}

/// True for a character that goes as `\uXXXX` in a double-quoted scalar.
fn needs_escape(c: char) -> bool {
    matches!(c, '\u{0}'..='\u{1f}' | '\u{7f}'..='\u{9f}' | '\u{feff}' | '\u{fffe}' | '\u{ffff}')
}

/// The text of the YAML scalar for a value. A plain value goes as it is. Any
/// other value goes in double quotes, so that it reads back as the same
/// string.
fn yaml_scalar(key: &str) -> String {
    const RESERVED: [&str; 10] = [
        "true", "false", "null", "yes", "no", "on", "off", "y", "n", "nan",
    ];
    let plain = key.starts_with(|c: char| c.is_ascii_alphabetic())
        && key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "_.-/+=".contains(c))
        && !RESERVED.contains(&key.to_ascii_lowercase().as_str())
        && !key.eq_ignore_ascii_case("inf");
    if plain {
        key.to_owned()
    } else {
        let mut text = String::with_capacity(key.len() + 2);
        text.push('"');
        for c in key.chars() {
            match c {
                '\\' => text.push_str("\\\\"),
                '"' => text.push_str("\\\""),
                c if needs_escape(c) => text.push_str(&format!("\\u{:04X}", u32::from(c))),
                c => text.push(c),
            }
        }
        text.push('"');
        text
    }
}

const TEMP_ATTEMPTS: u32 = 5;

/// A random number from the random seed of the standard library.
fn random_u64() -> u64 {
    RandomState::new().build_hasher().finish()
}

/// The text of the file: `config:`, then one line for each value that is
/// set, in a fixed order.
fn file_text(values: &FileValues) -> String {
    let mut text = String::from("config:\n");
    for (name, value) in values.entries() {
        if let Some(value) = value {
            text.push_str(&format!("  {name}: {}\n", yaml_scalar(value)));
        }
    }
    text
}

/// Write the key file (decision 8) through a temporary file and a rename.
/// Returns the path of `config.yml`.
pub fn write_key_file(
    fs: &dyn Fs,
    home: &Path,
    values: &FileValues,
) -> Result<PathBuf, ConfigError> {
    let write_error = |path: &Path, error: std::io::Error| ConfigError::KeyFileWrite {
        path: path.to_owned(),
        kind: error.kind(),
    };
    let config_dir = home.join(".config");
    let jag_dir = config_dir.join("jag");
    let path = jag_dir.join("config.yml");
    // 1. `~/.config` with the default mode. Never change a mode that exists.
    fs.create_dir_all(&config_dir)
        .map_err(|e| write_error(&config_dir, e))?;
    // 2. `~/.config/jag` with mode 0o700, when it does not exist.
    fs.create_private_dir(&jag_dir)
        .map_err(|e| write_error(&jag_dir, e))?;
    // 3. A new temporary file with mode 0o600. The name holds the process id
    //    and a random part. When the name is taken, try again a few times, so
    //    that a stale temporary file does not block setup.
    let text = file_text(values);
    let mut attempt = 1;
    let temp = loop {
        let temp = jag_dir.join(format!(
            "{TEMP_PREFIX}{}-{:016x}",
            std::process::id(),
            random_u64()
        ));
        match fs.create_private_file(&temp, &text) {
            Ok(()) => break temp,
            Err(error) if error.kind() == ErrorKind::AlreadyExists && attempt < TEMP_ATTEMPTS => {
                attempt += 1;
            }
            Err(error) => {
                // The file is not ours when the name was taken.
                if error.kind() != ErrorKind::AlreadyExists {
                    let _ = fs.remove_file(&temp);
                }
                return Err(write_error(&path, error));
            }
        }
    };
    // 4. Rename it over `config.yml`.
    if let Err(error) = fs.rename(&temp, &path) {
        let _ = fs.remove_file(&temp);
        return Err(write_error(&path, error));
    }
    // 5. Remove each stale temporary copy of an old write that stopped. It can
    //    hold a key that the user has removed since. The file is written, so a
    //    failure here does not fail the write.
    let _ = fs.remove_stale_temporary_files(&jag_dir);
    Ok(path)
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::os::unix::fs::PermissionsExt;

    use super::*;
    use crate::config::testing::*;
    use crate::config::{ConfigFile, KeySource, Origin, RealFs, Route, find_key};

    fn only_key(text: &str) -> FileValues {
        FileValues {
            api_key: Some(text.to_owned()),
            ..FileValues::default()
        }
    }

    fn load(home: Option<&Path>) -> Result<(PathBuf, ChangeState), ConfigError> {
        let vars = vars(&[]);
        let env = Env {
            vars: &vars,
            home: home.map(Path::to_owned),
        };
        load_for_change(&env, &RealFs)
    }

    #[test]
    fn no_home_gives_no_home() {
        assert_eq!(load(None).unwrap_err(), ConfigError::NoHome);
        assert_eq!(load(Some(Path::new(""))).unwrap_err(), ConfigError::NoHome);
        assert_eq!(
            crate::failure::Failure::from(ConfigError::NoHome).exit_code,
            2
        );
    }

    #[test]
    fn no_file_and_no_key_and_a_key() {
        let home = tempfile::tempdir().unwrap();
        let (_, state) = load(Some(home.path())).unwrap();
        assert!(!state.exists && state.values.is_empty());

        write_file(home.path(), "# comment only\n", 0o600);
        let (_, state) = load(Some(home.path())).unwrap();
        assert!(state.exists && state.values.is_empty());

        write_file(home.path(), "config:\n  api-key: k\n", 0o600);
        let (_, state) = load(Some(home.path())).unwrap();
        assert!(state.exists && state.values.api_key.as_deref() == Some("k"));
        assert_eq!(state.mode & 0o777, 0o600);
    }

    #[test]
    fn a_bad_yaml_file_gives_key_file_bad() {
        let home = tempfile::tempdir().unwrap();
        write_file(home.path(), "config:\n  api-key: [SENTINEL\n", 0o600);
        assert!(matches!(
            load(Some(home.path())).unwrap_err(),
            ConfigError::KeyFileBad { .. }
        ));
    }

    #[test]
    fn other_keys_give_key_file_has_other_keys() {
        for text in [
            "config:\n  api-key: k\nother: 1\n",
            "config:\n  api-key: k\n  other: 1\n",
        ] {
            let home = tempfile::tempdir().unwrap();
            write_file(home.path(), text, 0o600);
            let error = load(Some(home.path())).unwrap_err();
            assert!(
                matches!(error, ConfigError::KeyFileHasOtherKeys { .. }),
                "{text}"
            );
            assert_eq!(crate::failure::Failure::from(error).exit_code, 2);
        }
    }

    #[test]
    fn a_link_gives_key_file_is_link_and_leaves_the_target() {
        use std::os::unix::fs::symlink;
        let home = tempfile::tempdir().unwrap();
        let target_dir = tempfile::tempdir().unwrap();
        let target = target_dir.path().join("real.yml");
        std::fs::write(&target, "config:\n  api-key: k\n").unwrap();
        let dir = home.path().join(".config").join("jag");
        std::fs::create_dir_all(&dir).unwrap();
        symlink(&target, dir.join("config.yml")).unwrap();
        assert!(matches!(
            load(Some(home.path())).unwrap_err(),
            ConfigError::KeyFileIsLink { .. }
        ));
        assert_eq!(
            std::fs::read_to_string(&target).unwrap(),
            "config:\n  api-key: k\n"
        );
    }

    #[test]
    fn an_unreadable_file_gives_key_file_unreadable() {
        if is_root() {
            return;
        }
        let home = tempfile::tempdir().unwrap();
        write_file(home.path(), "config:\n  api-key: k\n", 0o000);
        assert!(matches!(
            load(Some(home.path())).unwrap_err(),
            ConfigError::KeyFileUnreadable { .. }
        ));
    }

    #[test]
    fn write_creates_the_folders_and_a_private_file() {
        let home = tempfile::tempdir().unwrap();
        let key = only_key(SENTINEL_KEY);
        let path = write_key_file(&RealFs, home.path(), &key).unwrap();
        let mode = |p: &Path| std::fs::metadata(p).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode(&path), 0o600);
        assert_eq!(mode(path.parent().unwrap()), 0o700);
        // `~/.config` keeps the default mode, not 0o700.
        assert_ne!(mode(&home.path().join(".config")), 0o700);
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            format!("config:\n  api-key: {SENTINEL_KEY}\n")
        );
    }

    #[test]
    fn write_does_not_change_the_mode_of_a_folder_that_exists() {
        let home = tempfile::tempdir().unwrap();
        let dir = home.path().join(".config").join("jag");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o755)).unwrap();
        write_key_file(&RealFs, home.path(), &only_key("k")).unwrap();
        assert_eq!(
            std::fs::metadata(&dir).unwrap().permissions().mode() & 0o777,
            0o755
        );
    }

    #[test]
    fn every_valid_key_reads_back_as_the_same_text() {
        for text in [
            "plain",
            "sk-abc_123.x/y+z=",
            "true",
            "False",
            "null",
            "~",
            "123",
            "1e3",
            "0x1F",
            "a#b",
            "#start",
            "[x",
            "{x",
            "*x",
            "&x",
            "!x",
            "'q'",
            "\"q\"",
            "a\\b",
            "%x",
            "@x",
            "`x",
            "|x",
            ">x",
            "-x",
            "?x",
            ":x",
            "a:b",
            "a:",
            "yes",
            "No",
            "On",
            "inf",
            ".nan",
        ] {
            let home = tempfile::tempdir().unwrap();
            let key = only_key(text);
            let path = write_key_file(&RealFs, home.path(), &key).unwrap();
            assert_eq!(
                std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
                0o600
            );
            let vars = vars(&[]);
            let env = Env {
                vars: &vars,
                home: Some(home.path().to_owned()),
            };
            let file = ConfigFile::new(&env, &RealFs);
            let (found, source) = find_key(&env, &file, Route::Direct, &Origin::Default).unwrap();
            assert_eq!(found.expose(), text, "key {text:?}");
            assert_eq!(source, KeySource::ConfigFile(path));
        }
    }

    fn all_values() -> FileValues {
        FileValues {
            api_key: Some(SENTINEL_KEY.to_owned()),
            gateway_api_key: Some("gateway-key".to_owned()),
            base_url: Some("https://openrouter.ai/api".to_owned()),
            model: Some("~typesafe/jev-latest".to_owned()),
            timeout: Some("9".to_owned()),
            retry_wait: Some("3".to_owned()),
            max_scored: Some("50".to_owned()),
            max_chars: Some("5000".to_owned()),
        }
    }

    #[test]
    fn the_file_text_has_one_line_for_each_value_in_a_fixed_order() {
        assert_eq!(
            file_text(&all_values()),
            format!(
                "config:\n  api-key: {SENTINEL_KEY}\n  gateway-api-key: gateway-key\n  base-url: \"https://openrouter.ai/api\"\n  model: \"~typesafe/jev-latest\"\n  timeout: \"9\"\n  retry-wait: \"3\"\n  max-scored: \"50\"\n  max-chars: \"5000\"\n"
            )
        );
        assert_eq!(
            file_text(&FileValues {
                base_url: Some("https://openrouter.ai/api".to_owned()),
                ..FileValues::default()
            }),
            "config:\n  base-url: \"https://openrouter.ai/api\"\n"
        );
    }

    // Row 5: the 4 numbers are plain text. A tagged number or a list is a YAML
    // error, as for the other keys.
    #[test]
    fn a_number_key_is_read_as_text_and_a_tagged_number_or_a_list_is_refused() {
        let home = tempfile::tempdir().unwrap();
        write_file(
            home.path(),
            "config:\n  timeout: 4\n  max-chars: 05000\n",
            0o600,
        );
        let (_, state) = load(Some(home.path())).unwrap();
        assert_eq!(state.values.timeout.as_deref(), Some("4"));
        assert_eq!(state.values.max_chars.as_deref(), Some("05000"));
        for text in ["config:\n  timeout: !!int 4\n", "config:\n  timeout: [4]\n"] {
            write_file(home.path(), text, 0o600);
            let error = load(Some(home.path())).unwrap_err();
            assert!(matches!(error, ConfigError::KeyFileBad { .. }), "{text}");
            assert_eq!(crate::failure::Failure::from(error).exit_code, 3);
        }
    }

    // Row 11: the table of the settings, the strict parse, and `FileValues`.
    #[test]
    fn each_key_of_the_table_passes_the_strict_parse_and_has_a_file_value() {
        let table = setting_table();
        assert_eq!(
            table.iter().map(|(name, _)| *name).collect::<Vec<_>>(),
            [
                "base-url",
                "model",
                "timeout",
                "retry-wait",
                "max-scored",
                "max-chars"
            ]
        );
        let entries: Vec<&str> = FileValues::default()
            .entries()
            .iter()
            .map(|e| e.0)
            .collect();
        for (name, default) in &table {
            assert!(entries.contains(name), "{name} has no field in FileValues");
            let home = tempfile::tempdir().unwrap();
            write_file(home.path(), &format!("config:\n  {name}: x\n"), 0o600);
            let (_, state) = load(Some(home.path())).unwrap();
            let value = state
                .values
                .entries()
                .iter()
                .find(|entry| entry.0 == *name)
                .and_then(|entry| entry.1.clone());
            assert_eq!(value.as_deref(), Some("x"), "{name}");
            assert!(!default.is_empty(), "{name}");
        }
        assert_eq!(table[2].1, "4");
        assert_eq!(table[3].1, "1");
        assert_eq!(table[4].1, "200");
        assert_eq!(table[5].1, "20000");
    }

    #[test]
    fn the_other_keys_text_names_the_8_keys() {
        let home = tempfile::tempdir().unwrap();
        write_file(home.path(), "config:\n  other: 1\n", 0o600);
        let error = load(Some(home.path())).unwrap_err();
        let cause = crate::failure::Failure::from(error).cause;
        let keys = "config.api-key, config.gateway-api-key, config.base-url, config.model, config.timeout, config.retry-wait, config.max-scored, and config.max-chars.";
        assert!(cause.ends_with(keys), "{cause}");
    }

    #[test]
    fn every_value_reads_back_as_the_same_text() {
        for text in [
            "https://openrouter.ai/api",
            "~typesafe/jev-latest",
            "typesafe-ai/jev",
            "jev-latest",
            "null",
            "a \"b\" \\c",
            "tab\there",
            "line\nfeed",
            "\u{feff}mark",
            "\u{7f}\u{85}",
            "é",
            "a: b",
        ] {
            for slot in 0..4 {
                let mut values = FileValues::default();
                let value = Some(text.to_owned());
                match slot {
                    0 => values.api_key = value,
                    1 => values.gateway_api_key = value,
                    2 => values.base_url = value,
                    _ => values.model = value,
                }
                let home = tempfile::tempdir().unwrap();
                write_key_file(&RealFs, home.path(), &values).unwrap();
                let (_, state) = load(Some(home.path())).unwrap();
                assert_eq!(state.values, values, "slot {slot} text {text:?}");
            }
        }
    }

    #[test]
    fn the_values_of_a_whole_file_read_back() {
        let home = tempfile::tempdir().unwrap();
        write_key_file(&RealFs, home.path(), &all_values()).unwrap();
        let (_, state) = load(Some(home.path())).unwrap();
        assert_eq!(state.values, all_values());
    }

    #[test]
    fn debug_shows_which_values_are_set_and_no_value() {
        let text = format!("{:?}", all_values());
        assert!(!text.contains(SENTINEL_KEY) && !text.contains("gateway-key"));
        assert!(!text.contains("openrouter") && !text.contains("jev-latest"));
        assert!(text.contains("set"));
        let empty = format!("{:?}", FileValues::default());
        assert!(empty.contains("not set") && !empty.contains("\"set\""));
    }

    /// A file system with two faults: the rename fails, or the first
    /// `taken` names for a temporary file are already taken.
    struct Faulty {
        rename_fails: bool,
        taken: std::cell::Cell<u32>,
        creates: std::cell::Cell<u32>,
    }

    impl Faulty {
        fn new(rename_fails: bool, taken: u32) -> Faulty {
            Faulty {
                rename_fails,
                taken: std::cell::Cell::new(taken),
                creates: std::cell::Cell::new(0),
            }
        }
    }

    impl Fs for Faulty {
        fn open(&self, path: &Path) -> io::Result<Box<dyn crate::config::OpenFile>> {
            RealFs.open(path)
        }
        fn is_symlink(&self, path: &Path) -> io::Result<bool> {
            RealFs.is_symlink(path)
        }
        fn is_special(&self, path: &Path) -> io::Result<bool> {
            RealFs.is_special(path)
        }
        fn create_dir_all(&self, path: &Path) -> io::Result<()> {
            RealFs.create_dir_all(path)
        }
        fn create_private_dir(&self, path: &Path) -> io::Result<()> {
            RealFs.create_private_dir(path)
        }
        fn create_private_file(&self, path: &Path, text: &str) -> io::Result<()> {
            self.creates.set(self.creates.get() + 1);
            if self.taken.get() > 0 {
                self.taken.set(self.taken.get() - 1);
                return Err(io::Error::from(io::ErrorKind::AlreadyExists));
            }
            RealFs.create_private_file(path, text)
        }
        fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
            if self.rename_fails {
                Err(io::Error::from(io::ErrorKind::PermissionDenied))
            } else {
                RealFs.rename(from, to)
            }
        }
        fn remove_file(&self, path: &Path) -> io::Result<()> {
            RealFs.remove_file(path)
        }
        fn remove_stale_temporary_files(&self, dir: &Path) -> io::Result<usize> {
            RealFs.remove_stale_temporary_files(dir)
        }
    }

    #[test]
    fn a_taken_temporary_name_is_tried_again() {
        let home = tempfile::tempdir().unwrap();
        let key = only_key("k");
        let fs = Faulty::new(false, TEMP_ATTEMPTS - 1);
        write_key_file(&fs, home.path(), &key).unwrap();
        assert_eq!(fs.creates.get(), TEMP_ATTEMPTS);
        // After the retries are spent, setup fails and no file stays.
        let home = tempfile::tempdir().unwrap();
        let fs = Faulty::new(false, TEMP_ATTEMPTS);
        assert!(matches!(
            write_key_file(&fs, home.path(), &key).unwrap_err(),
            ConfigError::KeyFileWrite {
                kind: ErrorKind::AlreadyExists,
                ..
            }
        ));
        assert_eq!(fs.creates.get(), TEMP_ATTEMPTS);
    }

    #[test]
    fn a_stale_temporary_file_does_not_block_setup_and_a_write_removes_it() {
        let home = tempfile::tempdir().unwrap();
        let dir = home.path().join(".config").join("jag");
        std::fs::create_dir_all(&dir).unwrap();
        let stale = dir.join(format!("{TEMP_PREFIX}{}", std::process::id()));
        std::fs::write(&stale, "stale").unwrap();
        write_key_file(&RealFs, home.path(), &only_key("k")).unwrap();
        assert!(!stale.exists());
        assert!(dir.join("config.yml").exists());
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
    }

    #[test]
    fn a_failed_rename_leaves_a_stale_temporary_file_of_an_old_write_alone() {
        // A failed write changes nothing that it did not make.
        let home = tempfile::tempdir().unwrap();
        let dir = home.path().join(".config").join("jag");
        std::fs::create_dir_all(&dir).unwrap();
        let stale = dir.join(format!("{TEMP_PREFIX}old"));
        std::fs::write(&stale, "stale").unwrap();
        write_key_file(&Faulty::new(true, 0), home.path(), &only_key("k")).unwrap_err();
        assert!(stale.exists());
    }

    #[test]
    fn a_failed_rename_gives_key_file_write_and_leaves_no_temporary_file() {
        let home = tempfile::tempdir().unwrap();
        let key = only_key(SENTINEL_KEY);
        let error = write_key_file(&Faulty::new(true, 0), home.path(), &key).unwrap_err();
        assert!(matches!(
            error,
            ConfigError::KeyFileWrite {
                kind: ErrorKind::PermissionDenied,
                ..
            }
        ));
        let failure = crate::failure::Failure::from(error);
        assert_eq!(failure.exit_code, 3);
        assert!(!failure.render().contains("SENTINEL"));
        let dir = home.path().join(".config").join("jag");
        assert_eq!(std::fs::read_dir(dir).unwrap().count(), 0);
    }

    #[test]
    fn a_failed_write_gives_key_file_write() {
        // `.config` is a file, so the folder cannot be made.
        let home = tempfile::tempdir().unwrap();
        std::fs::write(home.path().join(".config"), "x").unwrap();
        let key = only_key("k");
        assert!(matches!(
            write_key_file(&RealFs, home.path(), &key).unwrap_err(),
            ConfigError::KeyFileWrite { .. }
        ));
    }
}
