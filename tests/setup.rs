//! End-to-end tests of `--setup` and `--forget-key`.
//!
//! Each test sets HOME to a temp folder and removes the key variables, so a
//! key on the test machine does not change the result. The keys are made up.

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use jag::config::{ConfigError, ConfigFile, Env, RealFs, Route, find_key};
use predicates::prelude::*;

fn jag(home: &Path) -> Command {
    let mut command = Command::new(assert_cmd::cargo::cargo_bin!("jag"));
    command
        .env("HOME", home)
        .env_remove("JAG_TYPESAFE_API_KEY")
        .env_remove("TYPESAFE_API_KEY")
        .env_remove("JAG_GATEWAY_API_KEY")
        .env_remove("JAG_BASE_URL");
    command
}

fn key_path(home: &Path) -> PathBuf {
    home.join(".config").join("jag").join("config.yml")
}

fn write_key_file(home: &Path, text: &str) {
    let path = key_path(home);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, text).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
}

// CRITERION-72: setup with no terminal fails clearly.
#[test]
fn setup_with_no_terminal_exits_2_and_stores_nothing() {
    let home = tempfile::tempdir().unwrap();
    jag(home.path())
        .arg("--setup")
        .write_stdin("")
        .assert()
        .code(2)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains("--setup needs a terminal"))
        .stderr(predicate::str::contains("JAG_TYPESAFE_API_KEY"));
    assert!(!key_path(home.path()).exists());
}

#[test]
fn setup_with_a_piped_key_does_not_read_it() {
    let home = tempfile::tempdir().unwrap();
    jag(home.path())
        .arg("--setup")
        .write_stdin("SENTINEL-SENTINEL-SENTINEL\n")
        .assert()
        .code(2)
        .stdout(predicate::str::is_empty());
    assert!(!key_path(home.path()).exists());
}

// CRITERION-04: one command removes the stored key.
#[test]
fn forget_key_removes_the_stored_key() {
    let home = tempfile::tempdir().unwrap();
    write_key_file(
        home.path(),
        "config:\n  api-key: SENTINEL-SENTINEL-SENTINEL\n",
    );
    jag(home.path())
        .arg("--forget-key")
        .assert()
        .code(0)
        .stdout(predicate::str::contains("Removed the stored key"));
    assert!(!key_path(home.path()).exists());

    let vars = |_: &str| None;
    let env = Env {
        vars: &vars,
        home: Some(home.path().to_owned()),
    };
    let file = ConfigFile::new(&env, &RealFs);
    assert_eq!(
        find_key(&env, &file, Route::Direct).unwrap_err(),
        ConfigError::KeyMissing
    );
}

/// A temporary copy of the key file that an old write left, as a write that
/// stops between the file and the rename does. It holds the made-up key.
fn write_stale_copy(home: &Path, name: &str) -> PathBuf {
    let path = key_path(home).parent().unwrap().join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, "config:\n  api-key: SENTINEL-SENTINEL-SENTINEL\n").unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
    path
}

/// The names of the files in the folder of the key file.
fn names_in_jag_folder(home: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(key_path(home).parent().unwrap())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    names
}

// Issue 162: the removal of the key leaves no copy in the config folder.
#[test]
fn forget_key_removes_a_stale_temporary_copy_of_the_key() {
    let home = tempfile::tempdir().unwrap();
    write_key_file(
        home.path(),
        "config:\n  api-key: SENTINEL-SENTINEL-SENTINEL\n",
    );
    let stale = write_stale_copy(home.path(), "config.yml.tmp-4242-00000000000000aa");
    jag(home.path())
        .arg("--forget-key")
        .assert()
        .code(0)
        .stdout(predicate::str::contains("Removed the stored key"))
        .stdout(predicate::str::contains("SENTINEL").not());
    assert!(!stale.exists());
    assert!(names_in_jag_folder(home.path()).is_empty());
}

#[test]
fn forget_key_removes_a_stale_copy_when_other_values_stay() {
    let home = tempfile::tempdir().unwrap();
    write_key_file(
        home.path(),
        "config:\n  api-key: SENTINEL-SENTINEL-SENTINEL\n  model: jev-1.13.0\n",
    );
    write_stale_copy(home.path(), "config.yml.tmp-1-0000000000000001");
    write_stale_copy(home.path(), "config.yml.tmp-2-0000000000000002");
    jag(home.path()).arg("--forget-key").assert().code(0);
    assert_eq!(names_in_jag_folder(home.path()), ["config.yml"]);
    let text = std::fs::read_to_string(key_path(home.path())).unwrap();
    assert_eq!(text, "config:\n  model: jev-1.13.0\n");
}

#[test]
fn forget_key_removes_a_stale_copy_when_there_is_no_key_file() {
    let home = tempfile::tempdir().unwrap();
    // A first setup stopped before the rename: no `config.yml`, one copy.
    let stale = write_stale_copy(home.path(), "config.yml.tmp-7-0000000000000007");
    jag(home.path())
        .arg("--forget-key")
        .assert()
        .code(0)
        .stdout(predicate::str::contains("No stored key"));
    assert!(!stale.exists());
}

#[test]
fn forget_key_leaves_a_link_a_folder_and_other_files_alone() {
    use std::os::unix::fs::symlink;
    let home = tempfile::tempdir().unwrap();
    write_key_file(
        home.path(),
        "config:\n  api-key: SENTINEL-SENTINEL-SENTINEL\n",
    );
    let outside = tempfile::tempdir().unwrap();
    let target = outside.path().join("target.txt");
    std::fs::write(&target, "not ours").unwrap();
    let jag_dir = key_path(home.path()).parent().unwrap().to_owned();
    symlink(&target, jag_dir.join("config.yml.tmp-link")).unwrap();
    std::fs::create_dir(jag_dir.join("config.yml.tmp-folder")).unwrap();
    std::fs::write(jag_dir.join("notes.txt"), "keep").unwrap();
    jag(home.path()).arg("--forget-key").assert().code(0);
    assert_eq!(
        names_in_jag_folder(home.path()),
        ["config.yml.tmp-folder", "config.yml.tmp-link", "notes.txt"]
    );
    assert_eq!(std::fs::read_to_string(&target).unwrap(), "not ours");
}

#[test]
fn forget_key_with_no_file_exits_0() {
    let home = tempfile::tempdir().unwrap();
    jag(home.path())
        .arg("--forget-key")
        .assert()
        .code(0)
        .stdout(predicate::str::contains("No stored key"));
}

#[test]
fn forget_key_notes_a_variable_that_is_still_set() {
    let home = tempfile::tempdir().unwrap();
    write_key_file(home.path(), "config:\n  api-key: from-file\n");
    jag(home.path())
        .env("TYPESAFE_API_KEY", "SENTINEL-SENTINEL-SENTINEL")
        .arg("--forget-key")
        .assert()
        .code(0)
        .stdout(predicate::str::contains("TYPESAFE_API_KEY is still set"))
        .stdout(predicate::str::contains("SENTINEL").not());
}

#[test]
fn setup_and_forget_key_are_exclusive() {
    let home = tempfile::tempdir().unwrap();
    jag(home.path())
        .args(["--setup", "--forget-key"])
        .assert()
        .code(2);
    jag(home.path()).args(["--setup", "term"]).assert().code(2);
}

#[test]
fn no_option_gives_the_error_text_of_rg_and_exits_2() {
    let home = tempfile::tempdir().unwrap();
    jag(home.path())
        .assert()
        .code(2)
        .stdout(predicate::str::is_empty())
        .stderr("jag: jag requires at least one pattern to execute a search\n");
}

#[test]
fn help_and_version_work() {
    let home = tempfile::tempdir().unwrap();
    jag(home.path())
        .arg("--help")
        .assert()
        .code(0)
        .stdout(predicate::str::contains("--setup"))
        .stdout(predicate::str::contains("--forget-key"));
    jag(home.path())
        .arg("--version")
        .assert()
        .code(0)
        .stdout(predicate::str::starts_with("jag "));
}

#[test]
fn no_option_takes_the_key() {
    let home = tempfile::tempdir().unwrap();
    jag(home.path())
        .args(["--setup", "--key", "SENTINEL-SENTINEL-SENTINEL"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("SENTINEL").not());
}

// FEATURE-21: `--forget-key` on the direct route keeps the two settings.
#[test]
fn forget_key_keeps_the_two_settings_on_the_direct_route() {
    let home = tempfile::tempdir().unwrap();
    write_key_file(
        home.path(),
        "config:\n  api-key: SENTINEL-SENTINEL-SENTINEL\n  base-url: \"https://api.typesafe.ai\"\n  model: jev-1.13.0\n",
    );
    jag(home.path()).arg("--forget-key").assert().code(0);
    let text = std::fs::read_to_string(key_path(home.path())).unwrap();
    assert_eq!(
        text,
        "config:\n  base-url: \"https://api.typesafe.ai\"\n  model: jev-1.13.0\n"
    );
    assert_eq!(
        std::fs::metadata(key_path(home.path()))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
}

// FEATURE-21: on a gateway, `--forget-key` removes the gateway key only.
#[test]
fn forget_key_removes_the_gateway_key_only_on_a_gateway() {
    let home = tempfile::tempdir().unwrap();
    write_key_file(
        home.path(),
        "config:\n  api-key: SENTINEL-SENTINEL-SENTINEL\n  gateway-api-key: another-made-up-key\n",
    );
    jag(home.path())
        .env("JAG_BASE_URL", "https://openrouter.ai/api")
        .arg("--forget-key")
        .assert()
        .code(0)
        .stdout(predicate::str::contains(format!(
            "Removed the stored gateway key from {}.",
            key_path(home.path()).display()
        )));
    let text = std::fs::read_to_string(key_path(home.path())).unwrap();
    assert_eq!(text, "config:\n  api-key: SENTINEL-SENTINEL-SENTINEL\n");
}

// STORY-84, CRITERION-154: `--forget-key` keeps the keys of the settings.
#[test]
fn forget_key_keeps_the_keys_of_the_settings() {
    let home = tempfile::tempdir().unwrap();
    write_key_file(
        home.path(),
        "config:\n  api-key: SENTINEL-SENTINEL-SENTINEL\n  timeout: 9\n  max-chars: 5000\n",
    );
    jag(home.path()).arg("--forget-key").assert().code(0);
    let text = std::fs::read_to_string(key_path(home.path())).unwrap();
    assert!(
        !text.contains("api-key") && !text.contains("SENTINEL"),
        "{text}"
    );
    assert!(
        text.contains("timeout: ") && text.contains("max-chars: "),
        "{text}"
    );
    // A search still reads the two values (exit 0 needs no model call here).
    let vars = |_: &str| None;
    let env = Env {
        vars: &vars,
        home: Some(home.path().to_owned()),
    };
    let file = ConfigFile::new(&env, &RealFs);
    let settings =
        jag::config::model_settings(&env, &file, &jag::config::SettingOptions::default()).unwrap();
    assert_eq!(settings.times.timeout, std::time::Duration::from_secs(9));
}

const COMMENT_LINES: &str = "  # base-url: https://api.typesafe.ai/v1\n  # model: jev-1.13.0\n  # timeout: 4\n  # retry-wait: 1\n  # max-scored: 200\n  # max-chars: 20000\n";

// CRITERION-152: `--forget-key` keeps each comment line and adds none.
#[test]
fn forget_key_keeps_each_comment_line() {
    let home = tempfile::tempdir().unwrap();
    write_key_file(
        home.path(),
        &format!("config:\n  api-key: SENTINEL-SENTINEL-SENTINEL\n{COMMENT_LINES}"),
    );
    jag(home.path()).arg("--forget-key").assert().code(0);
    let text = std::fs::read_to_string(key_path(home.path())).unwrap();
    assert_eq!(text, format!("config:\n{COMMENT_LINES}"));
    // A file with only the key line is removed.
    write_key_file(
        home.path(),
        "config:\n  api-key: SENTINEL-SENTINEL-SENTINEL\n",
    );
    jag(home.path()).arg("--forget-key").assert().code(0);
    assert!(!key_path(home.path()).exists());
}

// STORY-85 row 18: a file in another form stops `--forget-key` with exit 2,
// and the file stays as it is.
#[test]
fn a_file_in_another_form_stops_forget_key_with_exit_2() {
    let home = tempfile::tempdir().unwrap();
    let text = "config: {api-key: SENTINEL-SENTINEL-SENTINEL, model: m}\n";
    write_key_file(home.path(), text);
    jag(home.path())
        .arg("--forget-key")
        .assert()
        .code(2)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains("not in the form that jag writes"))
        .stderr(predicate::str::contains("SENTINEL").not());
    assert_eq!(
        std::fs::read_to_string(key_path(home.path())).unwrap(),
        text
    );
}
