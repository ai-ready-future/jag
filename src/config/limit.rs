//! The cost limit: the most results that the model scores (STORY-19), and the
//! size limit: the most characters that jag writes to standard output
//! (STORY-25, STORY-28).
//!
//! No message here shows a value of `--max-scored`, `JAG_MAX_SCORED`,
//! `--max-chars`, or `JAG_MAX_CHARS` (CV-6).

use super::model::SettingSource;
use super::{ConfigError, ConfigFile, Env, file_setting};

pub const COST_LIMIT_OPTION: &str = "--max-scored";
pub const COST_LIMIT_VARIABLE: &str = "JAG_MAX_SCORED";
pub const DEFAULT_COST_LIMIT: usize = 200;
pub const MAX_COST_LIMIT: usize = 2_000;

pub const SIZE_LIMIT_OPTION: &str = "--max-chars";
pub const SIZE_LIMIT_OFF_OPTION: &str = "--no-max-chars";
pub const SIZE_LIMIT_VARIABLE: &str = "JAG_MAX_CHARS";
pub const DEFAULT_SIZE_LIMIT: usize = 20_000;
pub const MIN_SIZE_LIMIT: usize = 1_000;

/// The size limit of a search (CAPABILITY-04).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeLimit {
    /// The most characters on standard output.
    Chars(usize),
    /// `--no-max-chars`: no limit.
    Off,
}

impl SizeLimit {
    /// The count of characters, or `None` for no limit.
    pub fn chars(self) -> Option<usize> {
        match self {
            SizeLimit::Chars(count) => Some(count),
            SizeLimit::Off => None,
        }
    }
}

/// A whole number from 1 to 2,000: 1 to 4 ASCII digits and nothing else. The
/// error never holds the text.
pub fn parse_cost_limit(text: &str, source: SettingSource) -> Result<usize, ConfigError> {
    let text = text.trim();
    let bad = ConfigError::CostLimitBad { source };
    if text.is_empty() || text.len() > 4 || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(bad);
    }
    match text.parse::<usize>() {
        Ok(count) if (1..=MAX_COST_LIMIT).contains(&count) => Ok(count),
        _ => Err(bad),
    }
}

/// The option, else `JAG_MAX_SCORED`, else `config.max-scored` in the config
/// file, else the default. A blank variable, or a blank value in the file,
/// counts as not set.
pub fn cost_limit(
    env: &Env,
    file: &ConfigFile,
    option: Option<usize>,
) -> Result<usize, ConfigError> {
    if let Some(option) = option {
        return Ok(option);
    }
    if let Some(value) = (env.vars)(COST_LIMIT_VARIABLE) {
        let bad = ConfigError::CostLimitBad {
            source: SettingSource::Variable,
        };
        let text = value.to_str().ok_or(bad)?;
        if !text.trim().is_empty() {
            return parse_cost_limit(text, SettingSource::Variable);
        }
    }
    match file_setting(file, |section| section.max_scored.as_ref())? {
        Some((path, text)) => parse_cost_limit(text, SettingSource::Variable).map_err(|_| {
            ConfigError::FileCostLimitBad {
                path: path.to_owned(),
            }
        }),
        None => Ok(DEFAULT_COST_LIMIT),
    }
}

/// A whole number of 1,000 or more: ASCII digits and nothing else, after a
/// trim. A number too large for `usize` gives `usize::MAX`. The error never
/// holds the text.
pub fn parse_size_limit(text: &str, source: SettingSource) -> Result<usize, ConfigError> {
    let text = text.trim();
    let bad = ConfigError::SizeLimitBad { source };
    if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(bad);
    }
    let count = text.parse::<usize>().unwrap_or(usize::MAX);
    if count < MIN_SIZE_LIMIT {
        return Err(bad);
    }
    Ok(count)
}

/// The limit that `JAG_MAX_CHARS` gives. A blank or missing variable counts as
/// not set.
fn size_limit_variable(env: &Env) -> Result<Option<usize>, ConfigError> {
    let Some(value) = (env.vars)(SIZE_LIMIT_VARIABLE) else {
        return Ok(None);
    };
    let bad = ConfigError::SizeLimitBad {
        source: SettingSource::Variable,
    };
    let text = value.to_str().ok_or(bad)?;
    if text.trim().is_empty() {
        return Ok(None);
    }
    parse_size_limit(text, SettingSource::Variable).map(Some)
}

/// The option, else `JAG_MAX_CHARS`, else `config.max-chars` in the config
/// file, else the default. A blank variable, or a blank value in the file,
/// counts as not set. The file has no value for no limit (decision 10).
pub fn size_limit(
    env: &Env,
    file: &ConfigFile,
    option: Option<SizeLimit>,
) -> Result<SizeLimit, ConfigError> {
    if let Some(option) = option {
        return Ok(option);
    }
    if let Some(count) = size_limit_variable(env)? {
        return Ok(SizeLimit::Chars(count));
    }
    match file_setting(file, |section| section.max_chars.as_ref())? {
        Some((path, text)) => parse_size_limit(text, SettingSource::Variable)
            .map(SizeLimit::Chars)
            .map_err(|_| ConfigError::FileSizeLimitBad {
                path: path.to_owned(),
            }),
        None => Ok(SizeLimit::Chars(DEFAULT_SIZE_LIMIT)),
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::super::RealFs;
    use super::super::testing::{home_with, vars, write_file};
    use super::*;

    const SOURCES: [SettingSource; 2] = [SettingSource::Option, SettingSource::Variable];

    fn limit(pairs: &[(&str, &str)], option: Option<usize>) -> Result<usize, ConfigError> {
        limit_at(pairs, Path::new("/nowhere"), option)
    }

    fn limit_at(
        pairs: &[(&str, &str)],
        home: &Path,
        option: Option<usize>,
    ) -> Result<usize, ConfigError> {
        let vars = vars(pairs);
        let env = Env {
            vars: &vars,
            home: Some(home.to_owned()),
        };
        cost_limit(&env, &ConfigFile::new(&env, &RealFs), option)
    }

    #[test]
    fn whole_numbers_from_1_to_2000_pass() {
        for (text, count) in [
            ("1", 1),
            ("200", 200),
            ("2000", 2000),
            ("0400", 400),
            (" 10 ", 10),
        ] {
            for source in SOURCES {
                assert_eq!(parse_cost_limit(text, source), Ok(count), "{text:?}");
            }
        }
    }

    #[test]
    fn other_values_are_refused_for_both_sources() {
        for text in [
            "0",
            "2001",
            "02000",
            "1.5",
            "-1",
            "+1",
            "1e3",
            "2,000",
            "200 results",
            "abc",
            "",
            "  ",
        ] {
            for source in SOURCES {
                assert_eq!(
                    parse_cost_limit(text, source),
                    Err(ConfigError::CostLimitBad { source }),
                    "{text:?}"
                );
            }
        }
    }

    #[test]
    fn the_default_is_200() {
        assert_eq!(limit(&[], None), Ok(DEFAULT_COST_LIMIT));
        assert_eq!(DEFAULT_COST_LIMIT, 200);
    }

    #[test]
    fn the_option_wins_over_the_variable() {
        assert_eq!(limit(&[(COST_LIMIT_VARIABLE, "20")], Some(10)), Ok(10));
        // An option hides a bad variable.
        assert_eq!(limit(&[(COST_LIMIT_VARIABLE, "abc")], Some(10)), Ok(10));
    }

    #[test]
    fn the_variable_is_used_and_a_blank_one_is_not_set() {
        assert_eq!(limit(&[(COST_LIMIT_VARIABLE, " 20 ")], None), Ok(20));
        assert_eq!(limit(&[(COST_LIMIT_VARIABLE, " ")], None), Ok(200));
    }

    #[test]
    fn a_bad_variable_is_refused() {
        assert_eq!(
            limit(&[(COST_LIMIT_VARIABLE, "2001")], None),
            Err(ConfigError::CostLimitBad {
                source: SettingSource::Variable
            })
        );
    }

    #[test]
    fn a_variable_that_is_not_text_is_refused() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;
        let vars = |_: &str| Some(OsString::from_vec(vec![0xff]));
        let env = Env {
            vars: &vars,
            home: None,
        };
        let result = cost_limit(&env, &ConfigFile::new(&env, &RealFs), None);
        assert_eq!(
            result,
            Err(ConfigError::CostLimitBad {
                source: SettingSource::Variable
            })
        );
    }

    fn size(pairs: &[(&str, &str)], option: Option<SizeLimit>) -> Result<SizeLimit, ConfigError> {
        size_at(pairs, Path::new("/nowhere"), option)
    }

    fn size_at(
        pairs: &[(&str, &str)],
        home: &Path,
        option: Option<SizeLimit>,
    ) -> Result<SizeLimit, ConfigError> {
        let vars = vars(pairs);
        let env = Env {
            vars: &vars,
            home: Some(home.to_owned()),
        };
        size_limit(&env, &ConfigFile::new(&env, &RealFs), option)
    }

    fn size_bad(source: SettingSource) -> ConfigError {
        ConfigError::SizeLimitBad { source }
    }

    #[test]
    fn size_limits_of_1000_or_more_pass_for_both_sources() {
        for (text, count) in [
            ("1000", 1_000),
            ("20000", 20_000),
            ("01000", 1_000),
            (" 40000 ", 40_000),
            ("99999999999999999999999", usize::MAX),
        ] {
            for source in SOURCES {
                assert_eq!(parse_size_limit(text, source), Ok(count), "{text:?}");
            }
        }
    }

    #[test]
    fn other_size_limits_are_refused_for_both_sources() {
        for text in [
            "0",
            "999",
            "0999",
            "-1",
            "+1000",
            "1e4",
            "20,000",
            "1000.0",
            "20000 chars",
            "off",
            "none",
            "abc",
            "",
            "  ",
        ] {
            for source in SOURCES {
                assert_eq!(
                    parse_size_limit(text, source),
                    Err(size_bad(source)),
                    "{text:?}"
                );
            }
        }
    }

    #[test]
    fn the_size_limit_defaults_to_20000_and_the_smallest_is_1000() {
        assert_eq!(size(&[], None), Ok(SizeLimit::Chars(20_000)));
        assert_eq!(DEFAULT_SIZE_LIMIT, 20_000);
        assert_eq!(MIN_SIZE_LIMIT, 1_000);
        assert_eq!(SizeLimit::Chars(5).chars(), Some(5));
        assert_eq!(SizeLimit::Off.chars(), None);
    }

    #[test]
    fn the_size_option_wins_over_the_variable_and_hides_a_bad_one() {
        let variable = (SIZE_LIMIT_VARIABLE, "40000");
        assert_eq!(
            size(&[variable], Some(SizeLimit::Chars(2_000))),
            Ok(SizeLimit::Chars(2_000))
        );
        assert_eq!(
            size(&[(SIZE_LIMIT_VARIABLE, "abc")], Some(SizeLimit::Off)),
            Ok(SizeLimit::Off)
        );
    }

    #[test]
    fn the_size_variable_is_used_and_a_blank_one_is_not_set() {
        assert_eq!(
            size(&[(SIZE_LIMIT_VARIABLE, " 40000 ")], None),
            Ok(SizeLimit::Chars(40_000))
        );
        assert_eq!(
            size(&[(SIZE_LIMIT_VARIABLE, " ")], None),
            Ok(SizeLimit::Chars(DEFAULT_SIZE_LIMIT))
        );
        assert_eq!(
            size(&[(SIZE_LIMIT_VARIABLE, "999")], None),
            Err(size_bad(SettingSource::Variable))
        );
    }

    #[test]
    fn a_size_variable_that_is_not_text_is_refused() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;
        let vars = |name: &str| {
            (name == SIZE_LIMIT_VARIABLE).then(|| OsString::from_vec(vec![0xff, 0xfe]))
        };
        let env = Env {
            vars: &vars,
            home: Some(PathBuf::from("/nowhere")),
        };
        assert_eq!(
            size_limit(&env, &ConfigFile::new(&env, &RealFs), None),
            Err(size_bad(SettingSource::Variable))
        );
    }

    fn file_cost_bad(home: &Path) -> ConfigError {
        ConfigError::FileCostLimitBad {
            path: home.join(".config").join("jag").join("config.yml"),
        }
    }

    fn file_size_bad(home: &Path) -> ConfigError {
        ConfigError::FileSizeLimitBad {
            path: home.join(".config").join("jag").join("config.yml"),
        }
    }

    #[test]
    fn the_cost_limit_comes_from_the_file_when_no_option_or_variable_sets_it() {
        let home = home_with("config:\n  max-scored: 10\n");
        assert_eq!(limit_at(&[], home.path(), None), Ok(10));
        // The variable and the option win over the file.
        let variable = [(COST_LIMIT_VARIABLE, "20")];
        assert_eq!(limit_at(&variable, home.path(), None), Ok(20));
        assert_eq!(limit_at(&variable, home.path(), Some(30)), Ok(30));
        // A blank variable counts as not set.
        assert_eq!(
            limit_at(&[(COST_LIMIT_VARIABLE, " ")], home.path(), None),
            Ok(10)
        );
    }

    #[test]
    fn a_bad_cost_limit_in_the_file_is_refused_and_a_higher_source_hides_it() {
        for text in [
            "abc", "0", "2001", "1.5", "4.0", "0x10", "-1", "1e3", "2_00",
        ] {
            let home = home_with(&format!("config:\n  max-scored: {text}\n"));
            assert_eq!(
                limit_at(&[], home.path(), None),
                Err(file_cost_bad(home.path())),
                "{text}"
            );
            assert_eq!(limit_at(&[], home.path(), Some(5)), Ok(5));
            assert_eq!(
                limit_at(&[(COST_LIMIT_VARIABLE, "7")], home.path(), None),
                Ok(7)
            );
        }
    }

    #[test]
    fn the_cost_limit_text_rules_of_the_file_are_the_rules_of_the_variable() {
        let home = home_with("config:\n  max-scored: 0400\n");
        assert_eq!(limit_at(&[], home.path(), None), Ok(400));
        for text in [
            "max-scored:",
            "max-scored: ~",
            "max-scored: \"  \"",
            "# max-scored: 9",
        ] {
            let home = home_with(&format!("config:\n  {text}\n"));
            assert_eq!(
                limit_at(&[], home.path(), None),
                Ok(DEFAULT_COST_LIMIT),
                "{text}"
            );
        }
    }

    #[test]
    fn the_size_limit_comes_from_the_file_when_no_option_or_variable_sets_it() {
        let home = home_with("config:\n  max-chars: 5000\n");
        assert_eq!(size_at(&[], home.path(), None), Ok(SizeLimit::Chars(5_000)));
        let variable = [(SIZE_LIMIT_VARIABLE, "40000")];
        assert_eq!(
            size_at(&variable, home.path(), None),
            Ok(SizeLimit::Chars(40_000))
        );
        assert_eq!(
            size_at(&variable, home.path(), Some(SizeLimit::Chars(2_000))),
            Ok(SizeLimit::Chars(2_000))
        );
        // `--no-max-chars` wins over the file.
        assert_eq!(
            size_at(&[], home.path(), Some(SizeLimit::Off)),
            Ok(SizeLimit::Off)
        );
        // A number that is too large for `usize` gives `usize::MAX`.
        let home = home_with("config:\n  max-chars: 99999999999999999999999\n");
        assert_eq!(
            size_at(&[], home.path(), None),
            Ok(SizeLimit::Chars(usize::MAX))
        );
    }

    #[test]
    fn a_bad_size_limit_in_the_file_is_refused_and_a_higher_source_hides_it() {
        for text in [
            "abc", "0", "999", "off", "none", "1000.0", "-1", "0x3E8", "20_000",
        ] {
            let home = home_with(&format!("config:\n  max-chars: {text}\n"));
            assert_eq!(
                size_at(&[], home.path(), None),
                Err(file_size_bad(home.path())),
                "{text}"
            );
            assert_eq!(
                size_at(&[], home.path(), Some(SizeLimit::Off)),
                Ok(SizeLimit::Off)
            );
            assert_eq!(
                size_at(&[(SIZE_LIMIT_VARIABLE, "3000")], home.path(), None),
                Ok(SizeLimit::Chars(3_000))
            );
        }
    }

    #[test]
    fn a_higher_source_does_not_open_the_file() {
        // A file that other users can read is an error only when jag opens it.
        let home = tempfile::tempdir().expect("temporary folder");
        write_file(home.path(), "config:\n  max-chars: 5000\n", 0o644);
        assert_eq!(
            size_at(&[(SIZE_LIMIT_VARIABLE, "3000")], home.path(), None),
            Ok(SizeLimit::Chars(3_000))
        );
        assert!(matches!(
            size_at(&[], home.path(), None),
            Err(ConfigError::KeyFileOpen { .. })
        ));
    }

    #[test]
    fn the_file_error_texts_name_the_key_and_never_show_the_value() {
        use crate::failure::Failure;
        let home = home_with("config:\n  max-scored: SENTINEL-1\n  max-chars: SENTINEL-2\n");
        for (error, key) in [
            (
                limit_at(&[], home.path(), None).unwrap_err(),
                "config.max-scored",
            ),
            (
                size_at(&[], home.path(), None).unwrap_err(),
                "config.max-chars",
            ),
        ] {
            let failure = Failure::from(error);
            assert_eq!(failure.exit_code, 2);
            assert!(failure.cause.starts_with(key), "{}", failure.cause);
            assert!(failure.fix.contains(key), "{}", failure.fix);
            assert!(!failure.cause.contains("SENTINEL") && !failure.fix.contains("SENTINEL"));
        }
    }
}
