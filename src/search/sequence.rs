//! The haystacks of a plain text search, in the order of rg (FEATURE-23,
//! decisions 8, 9, 14, 15, and 16).
//!
//! The walk uses the settings of `HiArgs::walk_builder` of rg 15.2.0
//! (`crates/core/flags/hiargs.rs`), one thread, and the order of
//! `sort_by_file_name`. The paths of the command line keep their order. A
//! haystack is a file or the text of standard input.

use std::path::{Path, PathBuf};

use ignore::{DirEntry, Walk, WalkBuilder};

use super::Source;
use super::filter::FileFilter;
use super::input::stdin_can_give_text;

/// The path that `ignore` gives to the text of standard input.
const STDIN_PATH: &str = "-";

/// What the user gave as the places to search.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Places {
    /// The paths to search: those of the command line, or one of `-` and `./`.
    pub paths: Vec<PathBuf>,
    /// True when the user gave no path (`has_implicit_path` of rg). The paths
    /// that the walk gives then have no `./` at the start.
    pub implicit: bool,
    /// True when only one file is the haystack: one path that is `-` or is
    /// not a folder (`is_one_file` of rg).
    pub one_file: bool,
}

impl Places {
    /// The places of `given`. With no path, jag searches the text of standard
    /// input when it can give text (`is_readable_stdin` of rg), else the
    /// current folder.
    pub fn new(given: Vec<PathBuf>) -> Places {
        if !given.is_empty() {
            let one_file =
                given.len() == 1 && (given[0] == Path::new(STDIN_PATH) || !given[0].is_dir());
            return Places {
                paths: given,
                implicit: false,
                one_file,
            };
        }
        if stdin_can_give_text() {
            Places {
                paths: vec![PathBuf::from(STDIN_PATH)],
                implicit: true,
                one_file: true,
            }
        } else {
            Places {
                paths: vec![PathBuf::from("./")],
                implicit: true,
                one_file: false,
            }
        }
    }

    /// True when standard input is the only haystack.
    pub fn only_stdin(&self) -> bool {
        self.paths.len() == 1 && self.paths[0] == Path::new(STDIN_PATH)
    }
}

/// One thing that the walk gives, in order.
#[derive(Debug)]
pub enum Item {
    /// A file or the text of standard input to search.
    Haystack(Haystack),
    /// An error of the walk. It makes the exit code 2. The text has no `jag: `.
    Error(String),
    /// A message about an ignore file. It does not change the exit code.
    IgnoreMessage(String),
}

/// A thing to search.
#[derive(Clone, Debug)]
pub struct Haystack {
    /// The path to show. `<stdin>` for standard input.
    pub path: PathBuf,
    /// The path that the user gave, not a file that the walk found. The binary
    /// rule "convert" applies, not "quit" (decision 15).
    pub explicit: bool,
    /// The text of standard input.
    pub stdin: bool,
}

/// The haystacks, as rg gives them with one thread and `--sort path`.
pub struct Sequence {
    walk: Walk,
    implicit: bool,
}

impl Sequence {
    /// Start the walk. `cwd` is the current folder.
    pub fn new(places: &Places, files: &FileFilter, cwd: &Path) -> Sequence {
        let mut builder = WalkBuilder::new(&places.paths[0]);
        for path in places.paths.iter().skip(1) {
            builder.add(path);
        }
        builder
            .follow_links(false)
            .threads(1)
            .skip_stdout(true)
            .overrides(files.overrides())
            .hidden(true)
            .parents(true)
            .ignore(true)
            .git_global(true)
            .git_ignore(true)
            .git_exclude(true)
            .require_git(true)
            .ignore_case_insensitive(false)
            .current_dir(cwd)
            .add_custom_ignore_filename(".rgignore")
            .sort_by_file_name(|a, b| a.cmp(b));
        Sequence {
            walk: builder.build(),
            implicit: places.implicit,
        }
    }
}

impl Iterator for Sequence {
    type Item = Item;

    /// The next haystack, error, or message. A folder, and a link that the
    /// walk does not follow, give nothing.
    fn next(&mut self) -> Option<Item> {
        loop {
            let entry = match self.walk.next()? {
                Ok(entry) => entry,
                Err(error) => return Some(Item::Error(error.to_string())),
            };
            if let Some(error) = entry.error() {
                // `ignore_message` of rg: the message goes out, and the entry
                // goes on.
                return Some(Item::IgnoreMessage(error.to_string()));
            }
            if let Some(haystack) = self.haystack(&entry) {
                return Some(Item::Haystack(haystack));
            }
        }
    }
}

impl Sequence {
    /// `HaystackBuilder::build` of rg 15.2.0.
    fn haystack(&self, entry: &DirEntry) -> Option<Haystack> {
        let stdin = entry.is_stdin();
        let is_dir = entry.file_type().is_some_and(|kind| {
            kind.is_dir() || (entry.path_is_symlink() && entry.path().is_dir())
        });
        let explicit = stdin || (entry.depth() == 0 && !is_dir);
        let is_file = entry.file_type().is_some_and(|kind| kind.is_file());
        if !explicit && !is_file {
            return None;
        }
        let path = entry.path();
        let path = if stdin {
            Path::new(Source::STANDARD_INPUT_LABEL)
        } else if self.implicit {
            path.strip_prefix("./").unwrap_or(path)
        } else {
            path
        };
        Some(Haystack {
            path: path.to_path_buf(),
            explicit,
            stdin,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(root: &Path, name: &str, text: &str) {
        let path = root.join(name);
        std::fs::create_dir_all(path.parent().expect("parent")).expect("folders");
        std::fs::write(path, text).expect("write");
    }

    fn names(root: &Path, paths: Vec<PathBuf>) -> Vec<String> {
        let places = Places {
            paths,
            implicit: false,
            one_file: false,
        };
        Sequence::new(&places, &FileFilter::default(), root)
            .filter_map(|item| match item {
                Item::Haystack(haystack) => Some(haystack.path.display().to_string()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_folder_gives_its_files_by_name_folder_by_folder() {
        let dir = tempfile::tempdir().expect("folder");
        let root = dir.path();
        write(root, "sub/x.txt", "");
        write(root, "sub.txt", "");
        write(root, "B.txt", "");
        write(root, "a.txt", "");
        let found = names(root, vec![root.to_path_buf()]);
        let found: Vec<&str> = found
            .iter()
            .map(|path| {
                path.strip_prefix(root.to_str().expect("text"))
                    .expect("under root")
            })
            .collect();
        assert_eq!(found, ["/B.txt", "/a.txt", "/sub/x.txt", "/sub.txt"]);
    }

    #[test]
    fn the_paths_of_the_command_line_keep_their_order() {
        let dir = tempfile::tempdir().expect("folder");
        let root = dir.path();
        write(root, "a.txt", "");
        write(root, "b.txt", "");
        let b = root.join("b.txt");
        let a = root.join("a.txt");
        let found = names(root, vec![b.clone(), a.clone()]);
        assert_eq!(found, [b.display().to_string(), a.display().to_string()]);
    }

    #[test]
    fn a_hidden_file_of_a_walk_is_skipped_but_one_that_the_user_names_is_not() {
        let dir = tempfile::tempdir().expect("folder");
        let root = dir.path();
        write(root, ".hidden.txt", "");
        write(root, "a.txt", "");
        assert_eq!(names(root, vec![root.to_path_buf()]).len(), 1);
        assert_eq!(names(root, vec![root.join(".hidden.txt")]).len(), 1);
    }

    #[test]
    fn a_file_named_rgignore_hides_files() {
        let dir = tempfile::tempdir().expect("folder");
        let root = dir.path();
        write(root, ".rgignore", "r.txt\n");
        write(root, "r.txt", "");
        write(root, "k.txt", "");
        let found = names(root, vec![root.to_path_buf()]);
        assert_eq!(found.len(), 1);
        assert!(found[0].ends_with("k.txt"));
    }

    #[test]
    fn a_path_that_does_not_exist_gives_the_error_text_of_rg() {
        let dir = tempfile::tempdir().expect("folder");
        let places = Places {
            paths: vec![PathBuf::from("nope.txt")],
            implicit: false,
            one_file: true,
        };
        let items: Vec<Item> = Sequence::new(&places, &FileFilter::default(), dir.path()).collect();
        match items.as_slice() {
            [Item::Error(text)] => assert_eq!(
                text,
                "nope.txt: IO error for operation on nope.txt: No such file or directory (os error 2)"
            ),
            other => panic!("{other:?}"),
        }
    }
}
