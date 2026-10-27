//! The run tool of the held-out evaluation (STORY-78): the case file, the run of
//! one case with jag and with a blind grep, and the score.
//!
//! This module is built only with the Cargo feature `bench`. Releases do not
//! have it. No message of this module holds the text of a case, a folder
//! path, a line, or the key.

use std::collections::HashSet;
use std::path::{Component, Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;

use serde_json::{Map, Value};

use crate::cli::{MatchOptions, build_matcher};
use crate::config::{ApiKey, DEFAULT_COST_LIMIT};
use crate::failure::Failure;
use crate::prerank;
use crate::scorer::cost::Usage;
use crate::scorer::{ScoreSettings, rank, score_with_usage};
use crate::search::snippet::{ContextLines, SnippetLine, match_texts, plan_windows, read_snippets};
use crate::search::{ScanOptions, Source, scan};

/// The score that jag needs, and the value that grep must stay under.
pub const BAR: f64 = 0.70;
/// The count of results that the score looks at.
pub const TOP: usize = 10;

/// One labeled line of a case.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Answer {
    /// The path of the file, relative to the folder of the case.
    pub file: PathBuf,
    pub line: u64,
    /// 1, 2, or 3. A higher grade is a better answer.
    pub grade: u8,
}

/// One case: a folder, a search, a question, and the labeled lines.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Case {
    /// The folder, as an absolute path.
    pub folder: PathBuf,
    pub term: String,
    pub ignore_case: bool,
    pub fixed_strings: bool,
    pub question: String,
    /// Empty: a case with no answer.
    pub answers: Vec<Answer>,
}

impl Case {
    fn matching(&self) -> MatchOptions {
        MatchOptions {
            ignore_case: self.ignore_case,
            word: false,
            fixed: self.fixed_strings,
        }
    }
}

/// One result of a ranking: a file, and the lines of the result that match
/// the term. Context lines are not here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hit {
    /// Relative to the folder of the case.
    pub file: PathBuf,
    pub match_lines: Vec<u64>,
}

/// The score of one case.
#[derive(Clone, Debug, PartialEq)]
pub struct CaseScore {
    pub jag: f64,
    pub grep: f64,
    /// For each answer, in the order of the case file: the rank in the
    /// results of jag, and the rank in the results of grep (1 is the first).
    pub ranks: Vec<(Option<usize>, Option<usize>)>,
}

fn case_failure(number: usize, field: &str, cause: &str, fix: &str) -> Failure {
    Failure {
        cause: format!("case {number}, {field}: {cause}"),
        fix: fix.to_owned(),
        exit_code: 2,
    }
}

fn file_failure(cause: &str, fix: &str) -> Failure {
    Failure {
        cause: cause.to_owned(),
        fix: fix.to_owned(),
        exit_code: 2,
    }
}

/// Remove the `.` parts of a relative path: `./a/b` gives `a/b`.
fn plain(path: &Path) -> PathBuf {
    path.components()
        .filter(|part| !matches!(part, Component::CurDir))
        .collect()
}

const CASE_FIELDS: [&str; 6] = [
    "folder",
    "term",
    "ignore_case",
    "fixed_strings",
    "question",
    "answers",
];
const ANSWER_FIELDS: [&str; 3] = ["file", "line", "grade"];

/// Read the text of a case file. `base` is the folder of the case file: a
/// relative `folder` is relative to it. Each failure is a usage failure (exit
/// 2) that names the case by number and the field, never by text.
pub fn parse_cases(text: &str, base: &Path) -> Result<Vec<Case>, Failure> {
    let value: Value = serde_saphyr::from_str(text).map_err(|_| {
        file_failure(
            "the case file is not valid YAML.",
            "Check the syntax of the case file.",
        )
    })?;
    let list = value
        .as_object()
        .filter(|map| map.keys().all(|key| key == "cases"))
        .and_then(|map| map.get("cases"))
        .and_then(Value::as_array)
        .ok_or_else(|| {
            file_failure(
                "the case file needs one list named cases.",
                "Write cases: and then the list of cases.",
            )
        })?;
    if list.is_empty() {
        return Err(file_failure(
            "the case file has no case.",
            "Add at least one case to the list cases.",
        ));
    }
    list.iter()
        .enumerate()
        .map(|(index, item)| parse_case(index + 1, item, base))
        .collect()
}

fn parse_case(number: usize, item: &Value, base: &Path) -> Result<Case, Failure> {
    let fix_field = "Use the fields that docs/bench.md lists.";
    let Some(map) = item.as_object() else {
        return Err(case_failure(number, "case", "is not a mapping.", fix_field));
    };
    check_fields(number, "case", map, &CASE_FIELDS)?;
    let folder = text_field(number, map, "folder")?;
    let folder = base.join(folder);
    let folder = folder.canonicalize().ok().filter(|path| path.is_dir());
    let Some(folder) = folder else {
        return Err(case_failure(
            number,
            "folder",
            "is not a folder that exists.",
            "Give a folder that exists, relative to the case file or absolute.",
        ));
    };
    let term = text_field(number, map, "term")?.to_owned();
    let question = text_field(number, map, "question")?.to_owned();
    let ignore_case = bool_field(number, map, "ignore_case")?;
    let fixed_strings = bool_field(number, map, "fixed_strings")?;
    let case = Case {
        folder,
        term,
        ignore_case,
        fixed_strings,
        question,
        answers: Vec::new(),
    };
    if build_matcher(&[case.term.as_str()], case.matching()).is_err() {
        return Err(case_failure(
            number,
            "term",
            "is not a valid regular expression.",
            "Check the term. Set fixed_strings to true for a plain text.",
        ));
    }
    let answers = match map.get("answers") {
        None => {
            return Err(case_failure(
                number,
                "answers",
                "is missing.",
                "Write answers: [] for a case with no answer.",
            ));
        }
        Some(Value::Array(items)) => items,
        Some(_) => {
            return Err(case_failure(
                number,
                "answers",
                "is not a list.",
                "Write a list of answers, or answers: [].",
            ));
        }
    };
    let answers = answers
        .iter()
        .map(|item| parse_answer(number, item, &case.folder))
        .collect::<Result<Vec<_>, _>>()?;
    let mut seen = HashSet::new();
    for answer in &answers {
        if !seen.insert((answer.file.clone(), answer.line)) {
            return Err(case_failure(
                number,
                "answers",
                "has two answers for the same file and line.",
                "Remove one of the two answers.",
            ));
        }
    }
    Ok(Case { answers, ..case })
}

fn parse_answer(number: usize, item: &Value, folder: &Path) -> Result<Answer, Failure> {
    let Some(map) = item.as_object() else {
        return Err(case_failure(
            number,
            "answers",
            "holds an item that is not a mapping.",
            "Give each answer a file, a line, and a grade.",
        ));
    };
    check_fields(number, "answers", map, &ANSWER_FIELDS)?;
    let file = map
        .get("file")
        .and_then(Value::as_str)
        .filter(|text| !text.is_empty());
    let Some(file) = file else {
        return Err(case_failure(
            number,
            "answers.file",
            "is missing or is not a text.",
            "Give the path of the file, relative to the folder.",
        ));
    };
    let file = plain(Path::new(file));
    if file.is_absolute() || !folder.join(&file).is_file() {
        return Err(case_failure(
            number,
            "answers.file",
            "is not a file in the folder.",
            "Give the path of a file in the folder, relative to the folder.",
        ));
    }
    let line = map
        .get("line")
        .and_then(Value::as_u64)
        .filter(|line| *line >= 1);
    let Some(line) = line else {
        return Err(case_failure(
            number,
            "answers.line",
            "is missing or is not a whole number of 1 or more.",
            "Give the number of the line. The first line is 1.",
        ));
    };
    let grade = match map.get("grade") {
        None => 1,
        Some(value) => match value.as_u64() {
            Some(grade @ 1..=3) => grade as u8,
            _ => {
                return Err(case_failure(
                    number,
                    "answers.grade",
                    "is not 1, 2, or 3.",
                    "Use 1, 2, or 3. Leave the field out for 1.",
                ));
            }
        },
    };
    Ok(Answer { file, line, grade })
}

fn check_fields(
    number: usize,
    field: &str,
    map: &Map<String, Value>,
    known: &[&str],
) -> Result<(), Failure> {
    if map.keys().any(|key| !known.contains(&key.as_str())) {
        return Err(case_failure(
            number,
            field,
            "has a field that jag-bench does not know.",
            "Use the fields that docs/bench.md lists.",
        ));
    }
    Ok(())
}

fn text_field<'a>(
    number: usize,
    map: &'a Map<String, Value>,
    field: &str,
) -> Result<&'a str, Failure> {
    map.get(field)
        .and_then(Value::as_str)
        .filter(|text| !text.trim().is_empty())
        .ok_or_else(|| {
            case_failure(
                number,
                field,
                "is missing, is blank, or is not a text.",
                "Give a text. Put quotes around a text that looks like a number.",
            )
        })
}

fn bool_field(number: usize, map: &Map<String, Value>, field: &str) -> Result<bool, Failure> {
    match map.get(field) {
        None => Ok(false),
        Some(Value::Bool(value)) => Ok(*value),
        Some(_) => Err(case_failure(
            number,
            field,
            "is not true or false.",
            "Write true or false.",
        )),
    }
}

/// For each answer, the rank (1 is the first) of the result that credits it.
/// A result credits the answer of the highest grade among the answers that
/// are match lines of the result and that no earlier result credited. A
/// context line gives no credit. An answer is credited once.
pub fn credit(hits: &[Hit], answers: &[Answer]) -> Vec<Option<usize>> {
    let mut ranks: Vec<Option<usize>> = vec![None; answers.len()];
    for (index, hit) in hits.iter().take(TOP).enumerate() {
        let best = answers
            .iter()
            .enumerate()
            .filter(|(at, answer)| {
                ranks[*at].is_none()
                    && answer.file == hit.file
                    && hit.match_lines.contains(&answer.line)
            })
            .max_by_key(|(at, answer)| (answer.grade, std::cmp::Reverse(*at)));
        if let Some((at, _)) = best {
            ranks[at] = Some(index + 1);
        }
    }
    ranks
}

/// The discount of a rank: `1 / log2(rank + 1)`.
fn discount(rank: usize) -> f64 {
    1.0 / ((rank + 1) as f64).log2()
}

/// ndcg@10 with the grade as the gain. The ideal list is all labeled lines by
/// grade, highest first, cut at 10.
pub fn ndcg(ranks: &[Option<usize>], answers: &[Answer]) -> f64 {
    let gain: f64 = ranks
        .iter()
        .zip(answers)
        .filter_map(|(rank, answer)| rank.map(|rank| f64::from(answer.grade) * discount(rank)))
        .sum();
    let mut grades: Vec<u8> = answers.iter().map(|answer| answer.grade).collect();
    grades.sort_unstable_by(|a, b| b.cmp(a));
    let ideal: f64 = grades
        .iter()
        .take(TOP)
        .enumerate()
        .map(|(index, grade)| f64::from(*grade) * discount(index + 1))
        .sum();
    if ideal == 0.0 { 0.0 } else { gain / ideal }
}

/// The score of a case from the results of jag and of grep.
pub fn score_case(jag: &[Hit], grep: &[Hit], answers: &[Answer]) -> CaseScore {
    let jag_ranks = credit(jag, answers);
    let grep_ranks = credit(grep, answers);
    CaseScore {
        jag: ndcg(&jag_ranks, answers),
        grep: ndcg(&grep_ranks, answers),
        ranks: jag_ranks.into_iter().zip(grep_ranks).collect(),
    }
}

/// Round to 3 places, as the summary and the detail file show it.
pub fn round3(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

/// The mean of the scores of the cases with answers.
pub fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f64>() / values.len() as f64
    }
}

/// True when jag gets the bar: `BAR` or more, and more than grep. The values
/// are not rounded.
pub fn bar_passes(jag: f64, grep: f64) -> bool {
    jag >= BAR && jag > grep
}

/// What a run of cases needs. The made-up key of a test and the real key go
/// the same way.
pub struct Setup<'a> {
    pub key: &'a ApiKey,
    pub settings: &'a ScoreSettings,
    /// Set by a Ctrl-C.
    pub stop: &'a AtomicBool,
    pub usage: &'a Mutex<Usage>,
}

/// The lines of a result that match the term.
fn match_lines(lines: &[SnippetLine]) -> Vec<u64> {
    lines
        .iter()
        .filter(|line| line.is_match)
        .map(|line| line.number)
        .collect()
}

/// Run one case with an answer: the search of jag (with the model) and the
/// blind grep, on the same scan.
pub fn run_case(
    case: &Case,
    setup: &Setup,
    on_call_end: &(dyn Fn(usize) + Sync),
) -> Result<CaseScore, Failure> {
    let matcher = build_matcher(&[case.term.as_str()], case.matching()).map_err(|_| {
        file_failure(
            "a term is not a valid regular expression.",
            "Check the terms of the case file.",
        )
    })?;
    let report = scan(
        std::slice::from_ref(&case.folder),
        &matcher,
        &ScanOptions::default(),
    );
    let relative = |source: &Source| -> Option<PathBuf> {
        let path = source.path()?;
        path.strip_prefix(&case.folder).ok().map(Path::to_path_buf)
    };

    // The blind grep: each matching line is one result, by path and by line.
    let mut lines: Vec<(PathBuf, u64)> = Vec::new();
    for file in &report.files {
        if let Some(path) = relative(&file.source) {
            lines.extend(
                file.lines
                    .iter()
                    .map(|line| (path.clone(), line.line_number)),
            );
        }
    }
    lines.sort_by(|a, b| {
        a.0.as_os_str()
            .as_encoded_bytes()
            .cmp(b.0.as_os_str().as_encoded_bytes())
            .then(a.1.cmp(&b.1))
    });
    let grep: Vec<Hit> = lines
        .into_iter()
        .take(TOP)
        .map(|(file, line)| Hit {
            file,
            match_lines: vec![line],
        })
        .collect();

    // The search of jag, in the order of the search flow.
    let windows = plan_windows(&report.files, ContextLines::DEFAULT);
    let total = windows.len();
    let kept: Vec<_> = if total > DEFAULT_COST_LIMIT {
        let texts = match_texts(&report.files, &windows);
        let best = prerank::best(&texts, &case.question, DEFAULT_COST_LIMIT);
        let mut windows: Vec<Option<_>> = windows.into_iter().map(Some).collect();
        best.into_iter()
            .filter_map(|index| windows[index].take())
            .collect()
    } else {
        windows
    };
    let (snippets, _warnings) = read_snippets(&kept, None);
    let jag: Vec<Hit> = if snippets.is_empty() {
        Vec::new()
    } else {
        let scores = score_with_usage(
            &snippets,
            &case.question,
            setup.key,
            setup.settings,
            setup.stop,
            on_call_end,
            setup.usage,
        )
        .map_err(|error| error.failure(setup.settings))?;
        rank(snippets, scores)
            .into_iter()
            .take(TOP)
            .filter_map(|(snippet, _)| {
                Some(Hit {
                    file: relative(&snippet.source)?,
                    match_lines: match_lines(&snippet.lines),
                })
            })
            .collect()
    };
    Ok(score_case(&jag, &grep, &case.answers))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn answer(file: &str, line: u64, grade: u8) -> Answer {
        Answer {
            file: PathBuf::from(file),
            line,
            grade,
        }
    }

    fn hit(file: &str, lines: &[u64]) -> Hit {
        Hit {
            file: PathBuf::from(file),
            match_lines: lines.to_vec(),
        }
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn ndcg_perfect_order() {
        let answers = [answer("a", 1, 3), answer("b", 2, 2), answer("c", 3, 1)];
        let hits = [hit("a", &[1]), hit("b", &[2]), hit("c", &[3])];
        let ranks = credit(&hits, &answers);
        assert_eq!(ranks, vec![Some(1), Some(2), Some(3)]);
        assert!(close(ndcg(&ranks, &answers), 1.0));
    }

    #[test]
    fn ndcg_known_value() {
        // Grades 2, 0, 1 by hand: the labeled lines are grade 2 at rank 1
        // and grade 1 at rank 3. DCG = 2/log2(2) + 1/log2(4) = 2.5. The ideal
        // list is 2, 1: 2/log2(2) + 1/log2(3).
        let answers = [answer("a", 1, 2), answer("c", 3, 1)];
        let hits = [hit("a", &[1]), hit("b", &[2]), hit("c", &[3])];
        let ranks = credit(&hits, &answers);
        assert_eq!(ranks, vec![Some(1), Some(3)]);
        let ideal = 2.0 + 1.0 / 3f64.log2();
        assert!(close(ndcg(&ranks, &answers), 2.5 / ideal));
    }

    #[test]
    fn ndcg_credit_once() {
        // Two results hold the same labeled line: only the first gets the
        // credit. A result with two labeled lines credits the higher grade.
        let answers = [answer("a", 1, 1), answer("a", 2, 3)];
        let hits = [hit("a", &[1, 2]), hit("a", &[1, 2]), hit("a", &[1])];
        let ranks = credit(&hits, &answers);
        assert_eq!(ranks, vec![Some(2), Some(1)]);
    }

    #[test]
    fn ndcg_context_lines() {
        // The labeled line is a context line of the result, not a match line.
        let answers = [answer("a", 5, 3)];
        let hits = [hit("a", &[4])];
        let ranks = credit(&hits, &answers);
        assert_eq!(ranks, vec![None]);
        assert!(close(ndcg(&ranks, &answers), 0.0));
    }

    #[test]
    fn ndcg_ideal_cut() {
        // 12 labeled lines of grade 1: the ideal list has 10 of them.
        let answers: Vec<Answer> = (1..=12).map(|line| answer("a", line, 1)).collect();
        let hits: Vec<Hit> = (1..=12).map(|line| hit("a", &[line])).collect();
        let ranks = credit(&hits, &answers);
        assert_eq!(ranks.iter().filter(|rank| rank.is_some()).count(), 10);
        assert!(close(ndcg(&ranks, &answers), 1.0));
    }

    #[test]
    fn bar_uses_unrounded_values() {
        assert!(bar_passes(0.70, 0.69));
        assert!(!bar_passes(0.6999, 0.1));
        assert!(!bar_passes(0.9, 0.9));
        assert_eq!(round3(0.12345), 0.123);
    }

    fn parse(text: &str) -> Result<Vec<Case>, Failure> {
        parse_cases(text, Path::new("."))
    }

    fn case_text(extra: &str) -> String {
        format!(
            "cases:\n  - folder: .\n    term: needle\n    question: where is the needle?\n{extra}"
        )
    }

    #[test]
    fn case_file_errors() {
        let ok = parse(&case_text("    answers: []\n")).expect("a case");
        assert_eq!(ok.len(), 1);
        assert!(ok[0].answers.is_empty());

        let cause = |text: String| parse(&text).expect_err("a failure").cause;
        assert!(cause("cases: []\n".into()).contains("no case"));
        assert!(cause("other: 1\n".into()).contains("cases"));
        assert!(cause("cases: [\n".into()).contains("YAML"));
        assert!(cause(case_text("    answers: []\n    extra: 1\n")).starts_with("case 1, case:"));
        assert!(cause(case_text("")).starts_with("case 1, answers:"));
        assert!(
            cause(case_text("    answers: []\n").replace("folder: .", "folder: no-such-folder"))
                .starts_with("case 1, folder:")
        );
        assert!(
            cause(case_text("    answers: []\n").replace("needle\n", "'a('\n"))
                .starts_with("case 1, term:")
        );
        assert!(
            cause(case_text("    answers: []\n").replace("where is the needle?", "' '"))
                .starts_with("case 1, question:")
        );
        let with = |answer: &str| case_text(&format!("    answers:\n      - {answer}\n"));
        assert!(cause(with("{line: 1}")).starts_with("case 1, answers.file:"));
        assert!(cause(with("{file: no-such-file, line: 1}")).starts_with("case 1, answers.file:"));
        assert!(cause(with("{file: Cargo.toml, line: 0}")).starts_with("case 1, answers.line:"));
        assert!(
            cause(with("{file: Cargo.toml, line: 1, grade: 4}"))
                .starts_with("case 1, answers.grade:")
        );
        assert!(
            cause(case_text(
                "    answers:\n      - {file: Cargo.toml, line: 1}\n      - {file: ./Cargo.toml, line: 1, grade: 2}\n"
            ))
            .contains("same file and line")
        );
        // No message holds the text of the case.
        assert!(!cause(with("{file: Cargo.toml, line: 0}")).contains("needle"));
    }
}
