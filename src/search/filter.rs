//! The globs of `-g` and `--glob` (FEATURE-23, decision 20).
//!
//! The rules are the rules of rg: the globs are the override globs of the
//! `ignore` crate. A glob with `!` in front leaves out the files that it
//! matches. When one glob has no `!`, a file must match a glob of that kind.
//! The last glob that matches decides.

use std::path::{Path, PathBuf};

use ignore::overrides::{Override, OverrideBuilder};

/// The globs of `-g`, in the order of the command line.
#[derive(Clone, Debug)]
pub struct FileFilter {
    globs: Override,
}

impl Default for FileFilter {
    fn default() -> Self {
        FileFilter {
            globs: Override::empty(),
        }
    }
}

impl FileFilter {
    /// The filter of `globs`, relative to the folder `cwd`. A glob that does
    /// not parse gives the error of the `ignore` crate, as rg does.
    pub fn new(globs: &[String], cwd: &Path) -> Result<FileFilter, ignore::Error> {
        if globs.is_empty() {
            return Ok(FileFilter::default());
        }
        let mut builder = OverrideBuilder::new(cwd);
        for glob in globs {
            builder.add(glob)?;
        }
        Ok(FileFilter {
            globs: builder.build()?,
        })
    }

    /// The `Override` for the walk of `ignore`.
    pub fn overrides(&self) -> Override {
        self.globs.clone()
    }

    /// True when jag searches the file at `path`.
    pub fn allows(&self, path: &Path) -> bool {
        !self.globs.matched(path, false).is_ignore()
    }
}

/// The current folder, or `.` when jag cannot read it. rg uses `PWD` as a
/// second source (`current_dir` of `hiargs.rs`).
pub fn current_dir() -> PathBuf {
    std::env::current_dir()
        .ok()
        .or_else(|| std::env::var_os("PWD").filter(|pwd| !pwd.is_empty()).map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn filter(globs: &[&str]) -> FileFilter {
        let globs: Vec<String> = globs.iter().map(|glob| (*glob).to_owned()).collect();
        FileFilter::new(&globs, Path::new("/")).expect("valid globs")
    }

    fn allows(filter: &FileFilter, name: &str) -> bool {
        filter.allows(Path::new(name))
    }

    #[test]
    fn no_glob_allows_each_name() {
        let filter = FileFilter::default();
        assert!(allows(&filter, "a.txt"));
        assert!(allows(&filter, ".hidden"));
    }

    #[test]
    fn a_glob_with_no_bang_allows_only_the_files_that_it_matches() {
        let filter = filter(&["*.rs"]);
        assert!(!allows(&filter, "a.txt"));
        assert!(allows(&filter, "c.rs"));
        assert!(allows(&filter, "sub/c.rs"));
    }

    #[test]
    fn a_glob_with_a_bang_leaves_out_the_files_that_it_matches() {
        let filter = filter(&["!*.md"]);
        assert!(allows(&filter, "a.txt"));
        assert!(!allows(&filter, "d.md"));
    }

    #[test]
    fn the_last_glob_that_matches_decides() {
        let filter = filter(&["*.rs", "!c.*"]);
        assert!(!allows(&filter, "c.rs"));
        assert!(allows(&filter, "b.rs"));
    }

    #[test]
    fn a_glob_uses_the_rules_of_ripgrep_and_is_case_sensitive() {
        let filter = filter(&["*.{rs,md}"]);
        assert!(allows(&filter, "d.md"));
        assert!(allows(&filter, "c.rs"));
        assert!(!allows(&filter, "c.RS"));
    }

    #[test]
    fn a_glob_that_does_not_parse_gives_the_error_of_the_ignore_crate() {
        let error = FileFilter::new(&["[".to_owned()], Path::new("/")).expect_err("error");
        assert_eq!(
            error.to_string(),
            "error parsing glob '[': unclosed character class; missing ']'"
        );
    }
}
