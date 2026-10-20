//! Content tests of the jag agent skill (FEATURE-24, CRITERION-115 and
//! CRITERION-117 to CRITERION-123). They read `skills/jag/SKILL.md` and
//! `README.md` at run time, because the crate package does not hold them.
//!
//! The tests prove that the text holds each instruction. The install test
//! (CRITERION-116) is `tests/agent_skill/install.test.js`. The test of what an
//! agent does with the text is FEATURE-26.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// The repository name in the install command (ADR-0024). This is the one place
/// in the tests that holds it.
const REPOSITORY: &str = "ai-ready-future/jag";

/// The install command of the README.
fn install_command() -> String {
    format!("$ npx skills add {REPOSITORY}")
}

/// The link that the skill gives for the install steps of jag.
fn install_link() -> String {
    format!("https://github.com/{REPOSITORY}#install")
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(relative: &str) -> String {
    fs::read_to_string(root().join(relative)).unwrap_or_else(|error| panic!("{relative}: {error}"))
}

fn skill() -> String {
    read("skills/jag/SKILL.md")
}

/// The frontmatter and the body of a file that starts with `---`.
fn split(text: &str) -> (&str, &str) {
    let rest = text
        .strip_prefix("---\n")
        .expect("the file starts with ---");
    let end = rest.find("\n---\n").expect("the frontmatter ends with ---");
    (&rest[..end], &rest[end + "\n---\n".len()..])
}

/// The text of one section of the body: from its `## ` heading to the next one.
fn section<'a>(body: &'a str, heading: &str) -> &'a str {
    let start = body
        .find(&format!("\n## {heading}\n"))
        .unwrap_or_else(|| panic!("no section {heading}"));
    let rest = &body[start + 1..];
    match rest[3..].find("\n## ") {
        Some(end) => &rest[..end + 3],
        None => rest,
    }
}

/// The text with each run of white space as one space, so that a phrase that
/// the file wraps on two lines still matches.
fn flat(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn body() -> String {
    let text = skill();
    split(&text).1.to_owned()
}

// CRITERION-115: the README shows one command that installs the skill.
#[test]
fn the_readme_gives_one_install_command() {
    let readme = read("README.md");
    let mut inside = false;
    let mut commands = Vec::new();
    for line in readme.lines() {
        if line.starts_with("```") {
            inside = !inside;
        } else if inside && line.starts_with("$ npx skills add") {
            commands.push(line.to_owned());
        }
    }
    assert_eq!(commands, [install_command()]);

    // In `## Install` the program comes first, then the skill, then the
    // other ways to install (spec row 20).
    let text = section(&readme, "Install");
    let position = |line: &str| {
        text.lines()
            .position(|found| found == line)
            .unwrap_or_else(|| panic!("`## Install` lacks the line {line}"))
    };
    let npm = position("$ npm install -g @airf/jag");
    let skill = position(&install_command());
    let cargo = position("$ cargo install --locked jag-cli");
    assert!(npm < skill && skill < cargo);

    // The text after the skill command says what the tool does and sends.
    for needed in [
        "--agent claude-code -y",
        "skills-lock.json",
        "DO_NOT_TRACK=1",
        "jag sends no usage data",
    ] {
        assert!(text.contains(needed), "`## Install` lacks {needed}");
    }
    assert!(!readme.contains("This release has the key setup only"));
}

// Decisions 3 to 5 of the spec, and ADR-0024: one file, for each agent.
#[test]
fn the_skill_frontmatter_is_for_each_agent() {
    let skills = root().join("skills");
    let names: Vec<_> = fs::read_dir(&skills)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    assert_eq!(names, ["jag"]);
    let files: Vec<_> = fs::read_dir(skills.join("jag"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    assert_eq!(files, ["SKILL.md"]);

    let text = skill();
    let (frontmatter, body) = split(&text);
    let lines: Vec<_> = frontmatter.lines().collect();
    assert_eq!(lines.len(), 2, "only name and description: {frontmatter}");
    assert_eq!(lines[0], "name: jag");
    let description = lines[1]
        .strip_prefix("description: \"")
        .and_then(|rest| rest.strip_suffix('"'))
        .expect("the description is one line in double quotes");
    assert!(!description.contains('"') && !description.contains('\\'));
    assert!((1..=1024).contains(&description.chars().count()));

    // The body is complete alone: no relative link (ADR-0025).
    for (index, _) in body.match_indices("](") {
        let target = &body[index + 2..];
        assert!(
            target.starts_with("https://"),
            "relative link: {target:.40}"
        );
    }
    // No word that only one agent knows (spec decision 6).
    for word in [
        "Claude",
        "Cursor",
        "Codex",
        "Copilot",
        "Gemini",
        "allowed-tools",
    ] {
        assert!(!body.contains(word), "the body names {word}");
    }
}

// ADR-0024: no other SKILL.md goes in the repository, because the tool
// installs each one that it finds.
#[test]
fn no_other_skill_file_is_in_the_repository() {
    fn walk(folder: &Path, found: &mut Vec<PathBuf>) {
        for entry in fs::read_dir(folder).unwrap() {
            let entry = entry.unwrap();
            let name = entry.file_name().to_string_lossy().to_lowercase();
            let path = entry.path();
            if entry.file_type().unwrap().is_dir() {
                // Other folders can hold the copies of other sessions.
                if ![".git", ".claude", "target", "node_modules"].contains(&name.as_str()) {
                    walk(&path, found);
                }
            } else if name == "skill.md" {
                found.push(path);
            }
        }
    }
    let mut found = Vec::new();
    walk(&root(), &mut found);
    assert_eq!(found, [root().join("skills/jag/SKILL.md")]);
    assert!(!root().join(".claude/skills").exists());
}

// CRITERION-117: the skill says to use jag in place of rg, and it gives no
// advice on when to use which tool (DECISION-54).
#[test]
fn the_skill_says_to_use_jag_in_place_of_rg() {
    let body = body();
    let text = flat(section(&body, "What jag is"));
    assert!(text.contains("in place of rg"));
    assert!(text.contains("--ask"));
    let all = skill().to_lowercase();
    for phrase in [
        "plain search",
        "too many lines",
        "too much text",
        "more lines than",
        "when to use",
    ] {
        assert!(!all.contains(phrase), "the skill holds the phrase {phrase}");
    }
}

/// The code spans and the lines of code blocks of the skill that start with
/// `prefix`.
fn commands_starting_with(prefix: &str) -> Vec<String> {
    let text = skill();
    let mut found = Vec::new();
    let mut in_block = false;
    for line in text.lines() {
        if line.starts_with("```") {
            in_block = !in_block;
            continue;
        }
        let pieces: Vec<&str> = if in_block {
            vec![line.trim()]
        } else {
            line.split('`').skip(1).step_by(2).collect()
        };
        found.extend(
            pieces
                .into_iter()
                .filter(|piece| piece.starts_with(prefix))
                .map(String::from),
        );
    }
    found
}

// CRITERION-118 and CRITERION-119: the two forms of the command. The agent
// decides about `--ask`, so no sentence says that the agent always asks.
#[test]
fn the_skill_gives_the_command_form() {
    let forms = commands_starting_with("jag --json --max-chars");
    let with_question: Vec<_> = forms.iter().filter(|form| form.contains("--ask")).collect();
    let without_question: Vec<_> = forms
        .iter()
        .filter(|form| !form.contains("--ask"))
        .collect();
    assert!(!with_question.is_empty(), "no form with --ask");
    assert!(!without_question.is_empty(), "no form with no question");
    // Each form ends with a path.
    for form in &forms {
        assert!(form.ends_with(" ."), "the form has no path: {form}");
    }

    let body = body();
    let text = flat(section(&body, "How to run jag"));
    for needed in [
        "--ask=\"QUESTION\"",
        "--json",
        "--max-chars",
        "1,000",
        "20,000",
        "characters, not tokens",
        "no call to the model",
    ] {
        assert!(text.contains(needed), "the section lacks {needed}");
    }

    // The text of each paragraph is one line, so that a line break does not
    // hide a sentence. The code blocks are not text.
    let text = skill();
    let mut prose = String::new();
    let mut in_block = false;
    for line in text.lines() {
        if line.starts_with("```") {
            in_block = !in_block;
        } else if !in_block {
            prose.push_str(line);
            prose.push(' ');
        }
    }
    for sentence in prose.split(". ") {
        let sentence = sentence.to_lowercase();
        assert!(
            !(sentence.contains("always") && sentence.contains("--ask")),
            "a sentence says that the agent always asks: {sentence}"
        );
    }
}

// CRITERION-121: one row for each exit code of IF-4.
#[test]
fn the_skill_has_one_row_for_each_exit_code() {
    let body = body();
    let text = section(&body, "What each exit code means");
    let codes: BTreeSet<u32> = text
        .lines()
        .filter(|line| line.starts_with("| "))
        .filter_map(|line| line.split('|').nth(1))
        .filter_map(|cell| cell.trim().parse().ok())
        .collect();
    let want: BTreeSet<u32> = [0, 1, 2, 3, 4, 5, 6, 7, 8, 126, 127, 130].into();
    assert_eq!(codes, want);
    let header = text
        .lines()
        .find(|line| line.starts_with("| Code"))
        .unwrap();
    assert_eq!(header, "| Code | Meaning | What to do |");
}

// CRITERION-122: the user hears the cause and the fix.
#[test]
fn the_skill_tells_the_user_the_cause_and_the_fix() {
    let body = body();
    let text = flat(section(&body, "When jag cannot run"));
    for needed in [
        "`error.cause`",
        "`error.fix`",
        "jag --setup",
        "JAG_TYPESAFE_API_KEY",
        "command not found",
        "npm install -g @airf/jag",
        install_link().as_str(),
        "Never ask the user for the key",
        "~/.config/jag/",
    ] {
        assert!(text.contains(needed), "the section lacks {needed}");
    }
    // The skill does not copy a failure text of jag (spec decision 14).
    assert!(!body.contains("the TypeSafe API"));
}

// CRITERION-123: the skill names no other search tool for a failure of jag
// (DECISION-55). The section and the rows of the codes where jag cannot run
// hold none of these words and phrases.
#[test]
fn the_skill_names_no_other_search_tool_after_a_failure() {
    let body = body();
    let mut text = section(&body, "When jag cannot run").to_owned();
    for line in section(&body, "What each exit code means").lines() {
        let code = line.split('|').nth(1).map(str::trim).unwrap_or_default();
        if ["3", "4", "5", "6", "126", "127"].contains(&code) {
            text.push_str(line);
            text.push('\n');
        }
    }
    let text = text.to_lowercase();
    let words: Vec<&str> = text
        .split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .collect();
    for word in ["grep", "rg", "ripgrep", "ag", "ack"] {
        assert!(!words.contains(&word), "the text names {word}");
    }
    let flat = flat(&text);
    for phrase in ["fall back", "fallback"] {
        assert!(!flat.contains(phrase), "the text holds {phrase}");
    }
    // The rows of the table are in the text.
    assert!(flat.contains("jag cannot run: see the next section"));
}
