//! The rg parity cases that need a file system set-up (ADR-0021, FEATURE-23):
//! files with no read permission, symbolic links, a folder that git marks, an
//! ignore file in a parent folder, and a file name that is not UTF-8.
//!
//! A fixture tree of `tests/parity/cases.yaml` cannot hold these, because git
//! cannot store them on each system. Each scenario here makes its files in a
//! new temporary folder, and then it follows the same rules as
//! `tests/rg_parity.rs`: the pinned rg and jag run with the same arguments in
//! three modes, and stdout, stderr, and the exit code are the same.
//!
//! The test finds rg at the path in `JAG_PARITY_RG`, as `tests/rg_parity.rs`
//! does.

#[path = "parity/harness.rs"]
mod harness;

use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};

use harness::{Invocation, RunMode, check_rg_version, compare_with_rg, find_rg, pinned_version};

/// One case: how to set up the files, where to run, and the arguments.
struct Scenario {
    id: &'static str,
    /// The folder to run in, relative to the folder of the set-up.
    cwd: &'static str,
    args: &'static [&'static str],
    setup: fn(&Path),
}

fn write(root: &Path, relative: &str, bytes: &[u8]) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().expect("a file has a parent")).expect("make a folder");
    fs::write(&path, bytes).expect("write a file");
}

fn chmod(root: &Path, relative: &str, mode: u32) {
    fs::set_permissions(root.join(relative), fs::Permissions::from_mode(mode))
        .expect("set the permissions");
}

/// Give back the permissions, so that the temporary folder can go.
fn restore_permissions(dir: &Path) {
    let Ok(metadata) = fs::symlink_metadata(dir) else {
        return;
    };
    if metadata.file_type().is_symlink() {
        return;
    }
    let _ = fs::set_permissions(dir, fs::Permissions::from_mode(0o755));
    if metadata.is_dir()
        && let Ok(entries) = fs::read_dir(dir)
    {
        for entry in entries.flatten() {
            restore_permissions(&entry.path());
        }
    }
}

fn locked_file(root: &Path) {
    write(root, "a.txt", b"foo a\n");
    write(root, "locked.txt", b"foo locked\n");
    chmod(root, "locked.txt", 0o000);
}

fn locked_folder(root: &Path) {
    write(root, "a.txt", b"foo a\n");
    write(root, "sub/x.txt", b"foo x\n");
    chmod(root, "sub", 0o000);
}

fn links(root: &Path) {
    write(root, "a.txt", b"foo a\n");
    write(root, "sub/c.txt", b"foo c\n");
    symlink("a.txt", root.join("link.txt")).expect("make a link");
    symlink("sub", root.join("linkdir")).expect("make a link");
    symlink("nowhere", root.join("broken")).expect("make a link");
}

fn git_folder(root: &Path) {
    fs::create_dir_all(root.join(".git")).expect("make a folder");
    write(root, ".gitignore", b"ignored.txt\n*.log\n");
    write(root, "a.txt", b"foo a\n");
    write(root, "ignored.txt", b"foo ignored\n");
    write(root, "sub/x.log", b"foo log\n");
    write(root, "sub/y.txt", b"foo y\n");
}

fn parent_ignore(root: &Path) {
    write(root, ".ignore", b"*.txt\n");
    write(root, "work/a.txt", b"foo a\n");
    write(root, "work/b.rs", b"foo b\n");
}

fn parent_gitignore_no_repo(root: &Path) {
    write(root, ".gitignore", b"*.txt\n");
    write(root, "work/a.txt", b"foo a\n");
    write(root, "work/b.rs", b"foo b\n");
}

fn parent_gitignore_repo(root: &Path) {
    fs::create_dir_all(root.join(".git")).expect("make a folder");
    write(root, ".gitignore", b"*.txt\n");
    write(root, "work/a.txt", b"foo a\n");
    write(root, "work/b.rs", b"foo b\n");
}

fn hidden_folder(root: &Path) {
    write(root, ".hidden/x.txt", b"foo x\n");
    write(root, "a.txt", b"foo a\n");
}

#[cfg(target_os = "linux")]
fn not_utf8_name(root: &Path) {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    fs::write(root.join(OsStr::from_bytes(b"n\xffme.txt")), b"foo odd\n").expect("write a file");
    write(root, "a.txt", b"foo a\n");
}

fn scenarios() -> Vec<Scenario> {
    #[allow(unused_mut)]
    let mut list = vec![
        Scenario {
            id: "locked-file-walk",
            cwd: "",
            args: &["--sort", "path", "foo", "."],
            setup: locked_file,
        },
        Scenario {
            id: "locked-file-path",
            cwd: "",
            args: &["foo", "locked.txt"],
            setup: locked_file,
        },
        Scenario {
            id: "locked-folder-walk",
            cwd: "",
            args: &["--sort", "path", "foo", "."],
            setup: locked_folder,
        },
        Scenario {
            id: "locked-folder-path",
            cwd: "",
            args: &["foo", "sub"],
            setup: locked_folder,
        },
        Scenario {
            id: "link-path",
            cwd: "",
            args: &["foo", "link.txt"],
            setup: links,
        },
        Scenario {
            id: "link-folder-path",
            cwd: "",
            args: &["--sort", "path", "foo", "linkdir"],
            setup: links,
        },
        Scenario {
            id: "link-walk",
            cwd: "",
            args: &["--sort", "path", "foo", "."],
            setup: links,
        },
        Scenario {
            id: "link-broken-path",
            cwd: "",
            args: &["foo", "broken"],
            setup: links,
        },
        Scenario {
            id: "link-count",
            cwd: "",
            args: &["--sort", "path", "-c", "foo", "."],
            setup: links,
        },
        Scenario {
            id: "git-walk",
            cwd: "",
            args: &["--sort", "path", "foo", "."],
            setup: git_folder,
        },
        Scenario {
            id: "git-no-path",
            cwd: "",
            args: &["--sort", "path", "foo"],
            setup: git_folder,
        },
        Scenario {
            id: "git-subfolder",
            cwd: "sub",
            args: &["--sort", "path", "foo"],
            setup: git_folder,
        },
        Scenario {
            id: "parent-ignore",
            cwd: "work",
            args: &["--sort", "path", "foo"],
            setup: parent_ignore,
        },
        Scenario {
            id: "parent-gitignore-no-repo",
            cwd: "work",
            args: &["--sort", "path", "foo"],
            setup: parent_gitignore_no_repo,
        },
        Scenario {
            id: "parent-gitignore-repo",
            cwd: "work",
            args: &["--sort", "path", "foo"],
            setup: parent_gitignore_repo,
        },
        Scenario {
            id: "hidden-folder-path",
            cwd: "",
            args: &["--sort", "path", "foo", ".hidden"],
            setup: hidden_folder,
        },
    ];
    #[cfg(target_os = "linux")]
    list.push(Scenario {
        id: "not-utf8-name",
        cwd: "",
        args: &["--sort", "path", "foo", "."],
        setup: not_utf8_name,
    });
    list
}

#[test]
fn jag_gives_the_same_output_as_rg_on_special_files() {
    let Some(rg) = find_rg() else { return };
    check_rg_version(&rg);
    let jag = PathBuf::from(env!("CARGO_BIN_EXE_jag"));
    let no_env = BTreeMap::new();

    let mut details = Vec::new();
    let mut summary = Vec::new();
    let mut runs = 0;
    for scenario in scenarios() {
        let root = tempfile::Builder::new()
            .prefix("jag-parity-fs-")
            .tempdir()
            .expect("make a folder");
        (scenario.setup)(root.path());
        let home = tempfile::Builder::new()
            .prefix("jag-parity-home-")
            .tempdir()
            .expect("make a folder");
        let args: Vec<String> = scenario.args.iter().map(|a| a.to_string()).collect();
        let invocation = Invocation {
            args: &args,
            stdin: None,
            env: &no_env,
        };
        let cwd = root.path().join(scenario.cwd);
        for mode in RunMode::ALL {
            runs += 1;
            let label = format!("{} [{}]", scenario.id, mode.name());
            if let Err((line, detail)) =
                compare_with_rg(&label, &rg, &jag, &invocation, mode, &cwd, home.path())
            {
                summary.push(line);
                details.push(detail);
            }
        }
        restore_permissions(root.path());
    }
    assert!(
        summary.is_empty(),
        "{} of {runs} runs differ from rg {}:\n\n{}\n\nSUMMARY ({} runs)\n{}\n",
        summary.len(),
        pinned_version(),
        details.join("\n"),
        summary.len(),
        summary.join("\n")
    );
}
