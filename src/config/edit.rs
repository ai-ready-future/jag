//! Change the lines of the key file. A change keeps each line that it does
//! not change, also each comment line (ADR-0029, STORY-85).
//!
//! The file is in the form that jag can change when it has the `config:` line
//! on its own, and each value on 1 line under it. For each other form,
//! `Lines::parse` gives `None`.

use std::fmt;

use std::path::Path;

use super::ConfigError;
use super::change::{KEY_NAMES, setting_table, yaml_scalar};

/// The text of the key file. It can hold a key, so it has no `Display`, and
/// its `Debug` shows no text (CV-6).
#[derive(Clone, Default, PartialEq, Eq)]
pub struct FileText(pub(super) String);

impl FileText {
    /// The lines of the file, to change them. A file that is not in the form
    /// that jag writes gives `KeyFileNotEditable`.
    pub fn lines(&self, path: &Path) -> Result<Lines, ConfigError> {
        Lines::parse(self).ok_or_else(|| ConfigError::KeyFileNotEditable {
            path: path.to_owned(),
        })
    }

    /// True when a line of the file is a comment line.
    pub fn has_comment_line(&self) -> bool {
        self.0.lines().any(is_comment)
    }
}

impl fmt::Debug for FileText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileText")
            .field("lines", &self.0.lines().count())
            .finish()
    }
}

/// The line with no line ending.
fn body(line: &str) -> &str {
    let line = line.strip_suffix('\n').unwrap_or(line);
    line.strip_suffix('\r').unwrap_or(line)
}

/// The line ending of a line: `\r\n`, `\n`, or nothing.
fn ending(line: &str) -> &str {
    &line[body(line).len()..]
}

fn is_blank(line: &str) -> bool {
    body(line).trim().is_empty()
}

/// A comment line: the first character that is not a space is `#`.
fn is_comment(line: &str) -> bool {
    body(line).trim_start().starts_with('#')
}

/// The `config:` line: it starts at column 0, then only spaces and a comment.
fn is_config_line(line: &str) -> bool {
    let Some(rest) = body(line).strip_prefix("config:") else {
        return false;
    };
    let rest = rest.trim_start_matches(' ');
    rest.is_empty() || (rest.len() < body(line).len() - "config:".len() && rest.starts_with('#'))
}

/// A value line: the width of the indent, and the name of the key.
fn value_line(line: &str) -> Option<(usize, &'static str)> {
    let line = body(line);
    let rest = line.trim_start_matches(' ');
    let indent = line.len() - rest.len();
    if indent == 0 {
        return None;
    }
    KEY_NAMES.into_iter().find_map(|name| {
        let after = rest.strip_prefix(name)?.strip_prefix(':')?;
        (after.is_empty() || after.starts_with([' ', '\t'])).then_some((indent, name))
    })
}

/// Where the lines of a file are.
#[derive(Default)]
struct Layout {
    /// The `config:` line.
    config: Option<usize>,
    /// The indent of the value lines.
    indent: Option<usize>,
    /// The value line of each key.
    values: Vec<(&'static str, usize)>,
    /// The settings that a comment line of the section names.
    commented: Vec<&'static str>,
    /// The last line of the section that is not blank, or the `config:` line.
    last: usize,
}

impl Layout {
    /// The layout of the lines, or `None` when the lines are not in the form
    /// that jag can change.
    fn of(lines: &[String]) -> Option<Layout> {
        let mut layout = Layout::default();
        let setting_names = setting_table().map(|(name, _)| name);
        for (index, line) in lines.iter().enumerate() {
            if layout.config.is_none() {
                if is_config_line(line) {
                    layout.config = Some(index);
                    layout.last = index;
                } else if !(is_blank(line) || is_comment(line) || body(line).trim_end() == "---") {
                    return None;
                }
            } else if is_blank(line) {
            } else if is_comment(line) {
                layout.last = index;
                let text = body(line)
                    .trim_start()
                    .strip_prefix('#')
                    .unwrap_or_default();
                let text = text.trim_start();
                for name in setting_names {
                    if text.strip_prefix(name).is_some_and(|r| r.starts_with(':')) {
                        layout.commented.push(name);
                    }
                }
            } else if let Some((indent, name)) = value_line(line) {
                if layout.indent.is_some_and(|known| known != indent)
                    || layout.values.iter().any(|(known, _)| *known == name)
                {
                    return None;
                }
                layout.indent = Some(indent);
                layout.values.push((name, index));
                layout.last = index;
            } else {
                return None;
            }
        }
        Some(layout)
    }

    fn value_line(&self, name: &str) -> Option<usize> {
        self.values
            .iter()
            .find(|(known, _)| *known == name)
            .map(|(_, index)| *index)
    }

    /// The setting is in the file as a value or as a comment line.
    fn has(&self, name: &str) -> bool {
        self.value_line(name).is_some() || self.commented.contains(&name)
    }

    fn indent(&self) -> String {
        " ".repeat(self.indent.unwrap_or(2))
    }
}

/// The lines of a key file that jag can change. There is no `Display`, and
/// `Debug` shows no line text (CV-6).
#[derive(Clone)]
pub struct Lines {
    lines: Vec<String>,
}

impl fmt::Debug for Lines {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lines")
            .field("lines", &self.lines.len())
            .finish()
    }
}

impl Lines {
    /// The lines of the text, or `None` when the text is not in the form that
    /// jag can change.
    pub fn parse(text: &FileText) -> Option<Lines> {
        let lines: Vec<String> = text.0.split_inclusive('\n').map(str::to_owned).collect();
        Layout::of(&lines)?;
        Some(Lines { lines })
    }

    fn layout(&self) -> Layout {
        // A change keeps the form. If it did not, the check after the edit
        // finds it.
        Layout::of(&self.lines).unwrap_or_default()
    }

    /// The line ending of a new line: `\r\n` when the `config:` line (or, with
    /// no `config:` line, the first line) ends with `\r\n`.
    fn new_ending(&self, layout: &Layout) -> &'static str {
        let probe = self.lines.get(layout.config.unwrap_or(0));
        if probe.is_some_and(|line| ending(line) == "\r\n") {
            "\r\n"
        } else {
            "\n"
        }
    }

    /// Give the line at `index` a line ending, before a new line follows it.
    fn close(&mut self, index: usize, new_ending: &str) {
        if let Some(line) = self.lines.get_mut(index)
            && ending(line).is_empty()
        {
            line.push_str(new_ending);
        }
    }

    /// Add `new` after the last line of the section, or at the end with a
    /// `config:` line first when the file has none.
    fn add_lines(&mut self, layout: &Layout, new: Vec<String>) {
        let eol = self.new_ending(layout);
        match layout.config {
            Some(_) => {
                self.close(layout.last, eol);
                let at = layout.last + 1;
                self.lines.splice(at..at, new);
            }
            None => {
                if let Some(last) = self.lines.len().checked_sub(1) {
                    self.close(last, eol);
                }
                self.lines.push(format!("config:{eol}"));
                self.lines.extend(new);
            }
        }
    }

    /// Set the key `name` to the value. The line of the key changes; a key
    /// that is not in the file goes directly under the `config:` line.
    pub fn set(&mut self, name: &str, value: &str) {
        let layout = self.layout();
        let eol = self.new_ending(&layout);
        let text = format!("{}{name}: {}", layout.indent(), yaml_scalar(value));
        if let Some(index) = layout.value_line(name) {
            let old = ending(&self.lines[index]).to_owned();
            self.lines[index] = format!("{text}{old}");
        } else if let Some(config) = layout.config {
            self.close(config, eol);
            self.lines.insert(config + 1, format!("{text}{eol}"));
        } else {
            self.add_lines(&layout, vec![format!("{text}{eol}")]);
        }
    }

    /// Remove the line of the key `name`.
    pub fn remove(&mut self, name: &str) {
        if let Some(index) = self.layout().value_line(name) {
            self.lines.remove(index);
        }
    }

    /// Add 1 comment line with the default for each setting that the file
    /// does not have as a value or as a comment line. Returns true when a line
    /// is added.
    pub fn add_missing_comments(&mut self) -> bool {
        let layout = self.layout();
        let eol = self.new_ending(&layout);
        let indent = layout.indent();
        let new: Vec<String> = setting_table()
            .into_iter()
            .filter(|(name, _)| !layout.has(name))
            .map(|(name, default)| format!("{indent}# {name}: {default}{eol}"))
            .collect();
        if new.is_empty() {
            return false;
        }
        self.add_lines(&layout, new);
        true
    }

    /// The text of the lines.
    pub fn text(&self) -> FileText {
        FileText(self.lines.concat())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(text: &str) -> Lines {
        Lines::parse(&FileText(text.to_owned())).expect("an editable form")
    }

    fn text_of(lines: &Lines) -> String {
        lines.text().0
    }

    const COMMENTS: &str = "  # base-url: https://api.typesafe.ai/v1\n  # model: jev-1.13.0\n  # timeout: 4\n  # retry-wait: 1\n  # max-scored: 200\n  # max-chars: 20000\n";

    // Rows 17 to 20: the form that jag can change.
    #[test]
    fn these_forms_are_editable() {
        for text in [
            "",
            "\n",
            "# only a comment\n",
            "---\n# note\n",
            "config:\n",
            "config:  # note\n  api-key: k\n",
            "# note\n---\nconfig:\n  api-key: k  # key\n\n  # note\n  model: m\n",
            "config:\r\n  api-key: k\r\n",
            "config:\n  api-key: k",
            "config:\n  api-key:\n",
            "config:\n    api-key: k\n    model: m\n",
            "config:\n  api-key: k\n# a comment at column 0\n",
        ] {
            assert!(
                Lines::parse(&FileText(text.to_owned())).is_some(),
                "{text:?}"
            );
        }
    }

    #[test]
    fn these_forms_are_not_editable() {
        for text in [
            "config: {api-key: k}\n",
            "config: null\n",
            "config:\n  api-key: |\n    k\n",
            "config:\n  api-key: \"a\n    b\"\n",
            "config:\n  api-key: k\n...\n",
            "config:\n  api-key: k\n---\nconfig:\n  model: m\n",
            "config:\n  api-key: k\n  api-key: j\n",
            "config:\n  api-key: k\n    model: m\n",
            "config:\n  \"api-key\": k\n",
            "other: 1\nconfig:\n  api-key: k\n",
            "config:\n  api-key: k\nconfig:\n",
            "config:\n  other: 1\n",
            "api-key: k\n",
            "config:#x\n",
        ] {
            assert!(
                Lines::parse(&FileText(text.to_owned())).is_none(),
                "{text:?}"
            );
        }
    }

    #[test]
    fn a_new_key_goes_directly_under_the_config_line_with_the_indent_of_the_section() {
        let mut file = lines("config:\n    model: m\n");
        file.set("api-key", "k");
        assert_eq!(text_of(&file), "config:\n    api-key: k\n    model: m\n");
        let mut file = lines("config:\n");
        file.set("api-key", "k");
        assert_eq!(text_of(&file), "config:\n  api-key: k\n");
    }

    #[test]
    fn a_key_that_is_set_changes_its_own_line_only() {
        let mut file =
            lines("# top\nconfig:  # note\n  # c1\n  api-key: old  # mine\n\n  model: m\n");
        file.set("api-key", "new");
        assert_eq!(
            text_of(&file),
            "# top\nconfig:  # note\n  # c1\n  api-key: new\n\n  model: m\n"
        );
    }

    #[test]
    fn a_file_with_no_config_line_gets_one_at_the_end() {
        let mut file = lines("# only a comment\n");
        file.set("api-key", "k");
        assert_eq!(text_of(&file), "# only a comment\nconfig:\n  api-key: k\n");
        let mut file = lines("---\n");
        assert!(file.add_missing_comments());
        assert_eq!(text_of(&file), format!("---\nconfig:\n{COMMENTS}"));
        let mut file = lines("");
        file.set("api-key", "k");
        assert_eq!(text_of(&file), "config:\n  api-key: k\n");
    }

    #[test]
    fn the_missing_comment_lines_go_after_the_last_line_that_is_not_blank() {
        let mut file = lines("config:\n  api-key: k\n  base-url: \"https://x.io\"\n\n\n");
        assert!(file.add_missing_comments());
        assert_eq!(
            text_of(&file),
            "config:\n  api-key: k\n  base-url: \"https://x.io\"\n  # model: jev-1.13.0\n  # timeout: 4\n  # retry-wait: 1\n  # max-scored: 200\n  # max-chars: 20000\n\n\n"
        );
    }

    // Rows 13 and 14: a setting in the file as a value or as a comment line
    // gets no line. A comment line stays as it is.
    #[test]
    fn a_setting_in_the_file_as_a_value_or_a_comment_gets_no_line() {
        let mut file = lines(
            "config:\n  # timeout: 3\n  #retry-wait: 9\n#   max-scored: 7\n  model:\n  base-url: x\n    # max-chars: 5\n",
        );
        assert!(!file.add_missing_comments());
        assert_eq!(
            text_of(&file),
            "config:\n  # timeout: 3\n  #retry-wait: 9\n#   max-scored: 7\n  model:\n  base-url: x\n    # max-chars: 5\n"
        );
    }

    #[test]
    fn a_comment_that_only_starts_like_a_name_is_not_the_setting() {
        let mut file = lines("config:\n  # timeouts: 3\n  ## retry-wait: 2\n");
        assert!(file.add_missing_comments());
        let text = text_of(&file);
        assert!(text.contains("# timeout: 4\n"), "{text}");
        assert!(text.contains("# retry-wait: 1\n"), "{text}");
    }

    #[test]
    fn a_comment_before_the_config_line_is_not_in_the_section() {
        let mut file = lines("# timeout: 3\nconfig:\n  api-key: k\n");
        assert!(file.add_missing_comments());
        assert!(text_of(&file).contains("  # timeout: 4\n"));
    }

    // Row 20: line endings.
    #[test]
    fn a_new_line_takes_the_line_ending_of_the_config_line() {
        let mut file = lines("config:\r\n  model: m\r\n");
        file.set("api-key", "k");
        assert!(file.add_missing_comments());
        let text = text_of(&file);
        assert!(text.starts_with("config:\r\n  api-key: k\r\n  model: m\r\n  # base-url:"));
        assert_eq!(text.matches("\r\n").count(), text.matches('\n').count());
        // A file with a `config:` line with `\n` and other lines with `\r\n`.
        let mut file = lines("config:\n  model: m\r\n");
        file.set("api-key", "k");
        assert_eq!(text_of(&file), "config:\n  api-key: k\n  model: m\r\n");
    }

    #[test]
    fn a_last_line_with_no_ending_gets_one_before_a_new_line() {
        let mut file = lines("config:\n  model: m");
        assert!(file.add_missing_comments());
        assert!(text_of(&file).starts_with("config:\n  model: m\n  # base-url:"));
        let mut file = lines("config:");
        file.set("api-key", "k");
        assert_eq!(text_of(&file), "config:\n  api-key: k\n");
        let mut file = lines("# note");
        file.set("api-key", "k");
        assert_eq!(text_of(&file), "# note\nconfig:\n  api-key: k\n");
        // A line that changes keeps its missing ending.
        let mut file = lines("config:\n  api-key: old");
        file.set("api-key", "new");
        assert_eq!(text_of(&file), "config:\n  api-key: new");
    }

    #[test]
    fn remove_takes_the_line_of_the_key_and_keeps_the_rest() {
        let mut file = lines("config:\n  # c\n  api-key: k  # mine\n\n  model: m\n");
        file.remove("api-key");
        assert_eq!(text_of(&file), "config:\n  # c\n\n  model: m\n");
        file.remove("api-key");
        assert_eq!(text_of(&file), "config:\n  # c\n\n  model: m\n");
    }

    #[test]
    fn the_text_of_a_full_new_file() {
        let mut file = lines("");
        file.set("api-key", "K");
        assert!(file.add_missing_comments());
        assert_eq!(text_of(&file), format!("config:\n  api-key: K\n{COMMENTS}"));
        // The second time adds nothing.
        assert!(!file.add_missing_comments());
    }

    #[test]
    fn a_comment_line_is_a_line_with_a_hash_as_its_first_character_that_is_not_a_space() {
        let has = |text: &str| FileText(text.to_owned()).has_comment_line();
        assert!(has("config:\n  # x\n"));
        assert!(has("# x\nconfig:\n"));
        assert!(!has("config:\n  api-key: k  # x\n"));
        assert!(!has("config:  # note\n"));
        assert!(!has(""));
    }

    #[test]
    fn debug_shows_no_line_text() {
        let text = FileText("config:\n  api-key: SENTINEL-KEY\n".to_owned());
        assert!(!format!("{text:?}").contains("SENTINEL"));
        let file = Lines::parse(&text).unwrap();
        assert!(!format!("{file:?}").contains("SENTINEL"));
    }
}
