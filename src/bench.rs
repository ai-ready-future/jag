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
use crate::search::snippet::{
    ContextLines, Snippet, SnippetLine, Window, match_texts, plan_windows, read_snippets,
};
use crate::search::{ScanOptions, ScanReport, Source, scan};

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
    /// The highest model score of the windows of the case. `None` when the
    /// case has no window, so no model call.
    pub top: Option<f64>,
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
        top: None,
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

/// The path of a source, relative to the folder of the case.
fn relative(case: &Case, source: &Source) -> Option<PathBuf> {
    let path = source.path()?;
    path.strip_prefix(&case.folder).ok().map(Path::to_path_buf)
}

/// The steps of a case up to the model call: the scan, the windows, and the
/// pre-rank when the windows are more than the cost limit.
struct Planned {
    report: ScanReport,
    /// The windows that go to the model, in the order of the search flow.
    kept: Vec<Window>,
    /// The windows that the pre-rank dropped. Empty with no pre-rank.
    removed: Vec<Window>,
    /// The count of windows before the pre-rank.
    total: usize,
}

/// Make the windows of one case, with the values of the default search.
fn plan_case(case: &Case) -> Result<Planned, Failure> {
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
    let windows = plan_windows(&report.files, ContextLines::DEFAULT);
    let total = windows.len();
    let (kept, removed) = if total > DEFAULT_COST_LIMIT {
        let texts = match_texts(&report.files, &windows);
        let best = prerank::best(&texts, &case.question, DEFAULT_COST_LIMIT);
        let mut slots: Vec<Option<Window>> = windows.into_iter().map(Some).collect();
        let kept = best
            .into_iter()
            .filter_map(|index| slots[index].take())
            .collect();
        (kept, slots.into_iter().flatten().collect())
    } else {
        (windows, Vec::new())
    };
    Ok(Planned {
        report,
        kept,
        removed,
        total,
    })
}

/// Run one case with an answer: the search of jag (with the model) and the
/// blind grep, on the same scan.
pub fn run_case(
    case: &Case,
    setup: &Setup,
    on_call_end: &(dyn Fn(usize) + Sync),
) -> Result<CaseScore, Failure> {
    let planned = plan_case(case)?;

    // The blind grep: each matching line is one result, by path and by line.
    let mut lines: Vec<(PathBuf, u64)> = Vec::new();
    for file in &planned.report.files {
        if let Some(path) = relative(case, &file.source) {
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
    let (snippets, _warnings) = read_snippets(&planned.kept, None);
    let (jag, top): (Vec<Hit>, Option<f64>) = if snippets.is_empty() {
        (Vec::new(), None)
    } else {
        let scores = score_snippets(&snippets, case, setup, on_call_end)?;
        let top = top_score(&scores);
        let hits = rank(snippets, scores)
            .into_iter()
            .take(TOP)
            .filter_map(|(snippet, _)| {
                Some(Hit {
                    file: relative(case, &snippet.source)?,
                    match_lines: match_lines(&snippet.lines),
                })
            })
            .collect();
        (hits, top)
    };
    Ok(CaseScore {
        top,
        ..score_case(&jag, &grep, &case.answers)
    })
}

/// The model scores of the results of a case.
fn score_snippets(
    snippets: &[Snippet],
    case: &Case,
    setup: &Setup,
    on_call_end: &(dyn Fn(usize) + Sync),
) -> Result<Vec<f64>, Failure> {
    score_with_usage(
        snippets,
        &case.question,
        setup.key,
        setup.settings,
        setup.stop,
        on_call_end,
        setup.usage,
    )
    .map_err(|error| error.failure(setup.settings))
}

/// The highest of the scores, or `None` for no score.
fn top_score(scores: &[f64]) -> Option<f64> {
    scores.iter().copied().reduce(f64::max)
}

/// Run one decoy (a case with no answer): the search of jag on the same scan,
/// with no rank and no ndcg@10. Gives its top model score, or `None` when the
/// case has no window (`--decoy`).
pub fn run_decoy(
    case: &Case,
    setup: &Setup,
    on_call_end: &(dyn Fn(usize) + Sync),
) -> Result<Option<f64>, Failure> {
    let planned = plan_case(case)?;
    let (snippets, _warnings) = read_snippets(&planned.kept, None);
    if snippets.is_empty() {
        return Ok(None);
    }
    Ok(top_score(&score_snippets(
        &snippets,
        case,
        setup,
        on_call_end,
    )?))
}

/// The cuts that the rule looks at: 0.05 to 0.95 in steps of 0.05. The cut
/// of `k` is `k / 20`.
const CUT_STEPS: std::ops::RangeInclusive<u32> = 1..=19;
/// The step of the current threshold, 0.4 (`THRESHOLD`).
const BASE_STEP: u32 = 8;
/// The best cut must have a balanced accuracy this much higher than 0.4 to
/// replace it (about the effect of 1 decoy in 9).
const CUT_MARGIN: f64 = 0.06;

/// The decoy AUC: over each pair of a case with an answer and a decoy, the
/// share of pairs where the case with an answer has the higher top score. A
/// tie counts one half. `None` when there is no pair.
pub fn decoy_auc(answered: &[f64], decoys: &[f64]) -> Option<f64> {
    let pairs = answered.len() * decoys.len();
    if pairs == 0 {
        return None;
    }
    let mut wins = 0.0;
    for with_answer in answered {
        for decoy in decoys {
            wins += match with_answer.partial_cmp(decoy) {
                Some(std::cmp::Ordering::Greater) => 1.0,
                Some(std::cmp::Ordering::Equal) => 0.5,
                _ => 0.0,
            };
        }
    }
    Some(wins / pairs as f64)
}

/// The mean of 2 shares at the cut `step / 20`: the cases with an answer whose
/// top score is the cut or more, and the decoys whose top score is below it.
fn balanced_accuracy(answered: &[f64], decoys: &[f64], step: u32) -> f64 {
    let cut = f64::from(step) / 20.0;
    let share = |count: usize, of: usize| count as f64 / of as f64;
    let kept = answered.iter().filter(|top| **top >= cut).count();
    let dropped = decoys.iter().filter(|top| **top < cut).count();
    (share(kept, answered.len()) + share(dropped, decoys.len())) / 2.0
}

/// The cut of the rule of STORY-81 (decision 7): the cut with the best
/// balanced accuracy, if it is `CUT_MARGIN` or more above that of 0.4. Else
/// 0.4. On a tie, the cut nearest to 0.4, then the lower cut. 0.4 when a group
/// has no top score.
pub fn best_cut(answered: &[f64], decoys: &[f64]) -> f64 {
    let base = f64::from(BASE_STEP) / 20.0;
    if answered.is_empty() || decoys.is_empty() {
        return base;
    }
    let accuracy = |step: u32| balanced_accuracy(answered, decoys, step);
    let best = CUT_STEPS
        .max_by(|a, b| {
            accuracy(*a)
                .total_cmp(&accuracy(*b))
                .then(a.abs_diff(BASE_STEP).cmp(&b.abs_diff(BASE_STEP)).reverse())
                .then(b.cmp(a))
        })
        .unwrap_or(BASE_STEP);
    // The accuracies are sums of small whole shares: a tiny slack keeps a gap
    // of exactly 0.06 from falling under the margin by a rounding error.
    if accuracy(best) - accuracy(BASE_STEP) >= CUT_MARGIN - 1e-9 {
        f64::from(best) / 20.0
    } else {
        base
    }
}

/// The decoy measure of a run (`--decoy`): the top scores of the cases with
/// an answer, and of the decoys. A case with no window has no top score and is
/// in neither group.
#[derive(Clone, Debug, PartialEq)]
pub struct DecoyResult {
    pub auc: Option<f64>,
    /// The decoys with a top score.
    pub scored: usize,
    /// The decoys.
    pub decoys: usize,
    pub cut: f64,
}

impl DecoyResult {
    pub fn new(answered: &[Option<f64>], decoys: &[Option<f64>]) -> DecoyResult {
        let answered: Vec<f64> = answered.iter().flatten().copied().collect();
        let scored: Vec<f64> = decoys.iter().flatten().copied().collect();
        DecoyResult {
            auc: decoy_auc(&answered, &scored),
            scored: scored.len(),
            decoys: decoys.len(),
            cut: best_cut(&answered, &scored),
        }
    }

    /// The 3 lines of the output. They hold no text of a case. The AUC is
    /// `n/a` when no pair of cases has a top score.
    pub fn lines(&self) -> String {
        let auc = self
            .auc
            .map_or_else(|| "n/a".to_owned(), |auc| format!("{auc:.3}"));
        format!(
            "jag-bench: decoy auc {auc}\n\
             jag-bench: decoys with a model score: {} of {}\n\
             jag-bench: cut {:.2}\n",
            self.scored, self.decoys, self.cut
        )
    }
}

/// The best score of one case, with no model call (`--best-order`).
#[derive(Clone, Debug, PartialEq)]
pub struct BestScore {
    /// ndcg@10 of the best order of the kept windows.
    pub ndcg: f64,
    /// The count of windows is more than `DEFAULT_COST_LIMIT`: the pre-rank ran.
    pub over_limit: bool,
    /// The labeled lines that are a match line of a window that the pre-rank
    /// dropped.
    pub removed: usize,
}

/// The labeled lines that are match lines of a window.
fn labeled<'a>(window: &'a Hit, answers: &'a [Answer]) -> impl Iterator<Item = &'a Answer> {
    answers
        .iter()
        .filter(|answer| answer.file == window.file && window.match_lines.contains(&answer.line))
}

/// The gain of a window: the highest grade of its labeled lines, or 0.
fn gain(window: &Hit, answers: &[Answer]) -> u8 {
    labeled(window, answers)
        .map(|answer| answer.grade)
        .max()
        .unwrap_or(0)
}

/// The best ndcg@10 that an order of the kept windows can get, and the count
/// of labeled lines in the removed windows. Each matching line is in one
/// window, so the windows do not compete for a labeled line. So the best
/// order is by gain, highest first, and the sort is stable (the scan order
/// breaks a tie).
fn best_of(kept: &[Hit], removed: &[Hit], answers: &[Answer]) -> (f64, usize) {
    let mut order = kept.to_vec();
    order.sort_by_key(|window| std::cmp::Reverse(gain(window, answers)));
    let lost = removed
        .iter()
        .map(|window| labeled(window, answers).count())
        .sum();
    (ndcg(&credit(&order, answers), answers), lost)
}

/// The best score of one case: the scan and the windows of the default
/// search, then the best order (`--best-order`).
pub fn best_case(case: &Case) -> Result<BestScore, Failure> {
    let planned = plan_case(case)?;
    let hits = |windows: &[Window]| -> Vec<Hit> {
        windows
            .iter()
            .filter_map(|window| {
                Some(Hit {
                    file: relative(case, &window.source)?,
                    match_lines: window.matches.clone(),
                })
            })
            .collect()
    };
    let (ndcg, removed) = best_of(&hits(&planned.kept), &hits(&planned.removed), &case.answers);
    Ok(BestScore {
        ndcg,
        over_limit: planned.total > DEFAULT_COST_LIMIT,
        removed,
    })
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

    #[test]
    fn best_order_known_value() {
        // Grades 1, 3 and 2 in 3 windows, and a window with no labeled line:
        // the best order is 3, 2, 1, so the score is 1.
        let answers = [answer("a", 1, 1), answer("b", 1, 3), answer("c", 1, 2)];
        let kept = [
            hit("a", &[1]),
            hit("z", &[1]),
            hit("b", &[1]),
            hit("c", &[1]),
        ];
        let (score, lost) = best_of(&kept, &[], &answers);
        assert!(close(score, 1.0));
        assert_eq!(lost, 0);
        // Window c removed by the pre-rank: DCG = 3/log2(2) + 1/log2(3). The
        // ideal list is 3, 2, 1.
        let kept = [hit("a", &[1]), hit("b", &[1])];
        let (score, lost) = best_of(&kept, &[hit("c", &[1])], &answers);
        let dcg = 3.0 + 1.0 / 3f64.log2();
        let ideal = 3.0 + 2.0 / 3f64.log2() + 1.0 / 4f64.log2();
        assert!(close(score, dcg / ideal));
        assert_eq!(lost, 1);
    }

    #[test]
    fn best_order_one_gain_per_window() {
        // One window holds a grade 3 and a grade 2 labeled match line: it
        // gives the gain 3 only.
        let answers = [answer("a", 1, 3), answer("a", 2, 2)];
        let (score, _) = best_of(&[hit("a", &[1, 2])], &[], &answers);
        assert!(close(score, 3.0 / (3.0 + 2.0 / 3f64.log2())));
    }

    #[test]
    fn best_order_context_line() {
        // The labeled line is a context line of the window, not a match line.
        let answers = [answer("a", 5, 3)];
        let (score, lost) = best_of(&[hit("a", &[4])], &[], &answers);
        assert!(close(score, 0.0));
        assert_eq!(lost, 0);
    }

    #[test]
    fn best_order_removed_count() {
        // A labeled line in a removed window counts 1. A labeled line in a
        // kept window, and a labeled line in no window, count 0.
        let answers = [answer("a", 1, 1), answer("b", 1, 1), answer("c", 1, 1)];
        let (_, lost) = best_of(&[hit("a", &[1])], &[hit("b", &[1])], &answers);
        assert_eq!(lost, 1);
    }

    #[test]
    fn best_order_keeps_the_scan_order_on_a_tie() {
        // Equal gains keep the order of the scan: window b has the grade 2
        // line, and windows x and y have none.
        let answers = [answer("b", 1, 2)];
        let kept = [hit("x", &[1]), hit("b", &[1]), hit("y", &[1])];
        let (score, _) = best_of(&kept, &[], &answers);
        assert!(close(score, 1.0));
    }

    #[test]
    fn the_decoy_auc_has_a_known_value() {
        // 0.9 beats 0.5 and 0.1, and 0.5 ties 0.5 and beats 0.1: 3.5 of 4.
        assert!(close(
            decoy_auc(&[0.9, 0.5], &[0.5, 0.1]).expect("pairs"),
            0.875
        ));
    }

    #[test]
    fn the_decoy_auc_counts_a_tie_as_one_half() {
        assert!(close(decoy_auc(&[0.5], &[0.5]).expect("pairs"), 0.5));
        assert!(close(decoy_auc(&[0.1], &[0.9]).expect("pairs"), 0.0));
    }

    #[test]
    fn the_decoy_auc_has_no_value_with_no_pair() {
        assert_eq!(decoy_auc(&[], &[0.5]), None);
        assert_eq!(decoy_auc(&[0.5], &[]), None);
    }

    #[test]
    fn the_cut_stays_0_4_when_the_gain_is_under_the_margin() {
        // 9 cases at 0.9, 1 decoy at 0.45 and 8 at 0.1. At 0.4 the balanced
        // accuracy is (1 + 8/9) / 2, and at 0.5 it is 1: a gain of 1/18.
        let answered = [0.9; 9];
        let mut decoys = vec![0.1; 8];
        decoys.push(0.45);
        assert!(close(best_cut(&answered, &decoys), 0.4));
    }

    #[test]
    fn the_cut_moves_when_the_gain_is_the_margin_or_more() {
        // 2 decoys at 0.45 and 7 at 0.1: a gain of 1/9 at 0.5. Each cut from
        // 0.5 to 0.9 gives 1, so the cut nearest to 0.4 is the best.
        let answered = [0.9; 9];
        let mut decoys = vec![0.1; 7];
        decoys.extend([0.45, 0.45]);
        assert!(close(best_cut(&answered, &decoys), 0.5));
    }

    #[test]
    fn the_cut_stays_0_4_on_a_tie_at_0_4() {
        assert!(close(best_cut(&[0.9; 3], &[0.1; 3]), 0.4));
    }

    #[test]
    fn the_cut_on_an_equal_distance_is_the_lower_cut() {
        // The cuts 0.35 and 0.45 give 0.5, and 0.4 gives 0.25.
        let answered = [0.36, 0.95];
        let decoys = [0.42, 0.99];
        assert!(close(best_cut(&answered, &decoys), 0.35));
    }

    #[test]
    fn the_cut_is_0_4_when_a_group_has_no_score() {
        assert!(close(best_cut(&[], &[]), 0.4));
        assert!(close(best_cut(&[0.9], &[]), 0.4));
        assert!(close(best_cut(&[], &[0.1]), 0.4));
    }

    #[test]
    fn the_decoy_result_counts_the_decoys_with_a_score() {
        let result = DecoyResult::new(&[Some(0.9), None, Some(0.8)], &[Some(0.1), None, Some(0.2)]);
        assert_eq!((result.scored, result.decoys), (2, 3));
        assert!(close(result.auc.expect("pairs"), 1.0));
        assert_eq!(
            result.lines(),
            "jag-bench: decoy auc 1.000\n\
             jag-bench: decoys with a model score: 2 of 3\n\
             jag-bench: cut 0.40\n"
        );
    }

    #[test]
    fn the_decoy_result_with_no_pair_gives_n_a() {
        let result = DecoyResult::new(&[Some(0.9)], &[None]);
        assert_eq!(result.auc, None);
        assert!(result.lines().starts_with("jag-bench: decoy auc n/a\n"));
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
