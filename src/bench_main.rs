//! `jag-bench`: the run tool of the held-out evaluation (STORY-78). It runs jag
//! and a blind grep on the cases of one case file, and prints four summary
//! lines. Releases do not have it: build it with `--features bench`.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use clap::Parser;
use jag::bench::{BAR, Case, CaseScore, Setup, bar_passes, mean, parse_cases, round3, run_case};
use jag::config::{ConfigFile, Env, RealFs, SettingOptions, find_key, model_settings};
use jag::failure::Failure;
use jag::scorer::cost::Usage;
use jag::scorer::{PROMPT_VERSION, ScoreSettings, THRESHOLD};
use serde_json::json;
use signal_hook::consts::SIGINT;

/// Score jag and a blind grep on a labeled set of cases.
#[derive(Parser)]
#[command(name = "jag-bench", version)]
struct Args {
    /// Write one JSON line for each case to this file. The file holds no text
    /// of a case and no path.
    #[arg(long, value_name = "FILE")]
    detail: Option<PathBuf>,
    /// The case file (YAML). See docs/bench.md.
    cases: PathBuf,
}

fn failure_of(cause: &str, fix: &str) -> Failure {
    Failure {
        cause: cause.to_owned(),
        fix: fix.to_owned(),
        exit_code: 2,
    }
}

fn render(failure: &Failure) -> String {
    format!("jag-bench: {}\n{}\n", failure.cause, failure.fix)
}

/// The absolute path of a file that can be new: the folder is resolved, the
/// name is not.
fn resolved(path: &Path) -> PathBuf {
    let absolute = std::env::current_dir()
        .map(|dir| dir.join(path))
        .unwrap_or_else(|_| path.to_path_buf());
    if let Ok(real) = absolute.canonicalize() {
        return real;
    }
    match (absolute.parent(), absolute.file_name()) {
        (Some(parent), Some(name)) => parent
            .canonicalize()
            .map_or_else(|_| absolute.clone(), |parent| parent.join(name)),
        _ => absolute,
    }
}

/// Exit 2 when a file is in the folder of the source that built this tool
/// (the held-out cases stay out of the repository).
fn guard(path: &Path, what: &str) -> Result<(), Failure> {
    let source = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = source
        .canonicalize()
        .unwrap_or_else(|_| source.to_path_buf());
    if resolved(path).starts_with(source) {
        return Err(failure_of(
            &format!("the {what} is in the folder of the source of jag-bench."),
            "Keep the held-out cases and results outside the source folder.",
        ));
    }
    Ok(())
}

fn read_cases(path: &Path) -> Result<Vec<Case>, Failure> {
    let text = fs::read_to_string(path).map_err(|_| {
        failure_of(
            "cannot read the case file.",
            "Check that the file exists and is UTF-8 text.",
        )
    })?;
    let base = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let cases = parse_cases(&text, base)?;
    if cases.iter().all(|case| case.answers.is_empty()) {
        return Err(failure_of(
            "no case has an answer.",
            "Label at least one line in the case file.",
        ));
    }
    Ok(cases)
}

fn settings() -> Result<(jag::config::ApiKey, ScoreSettings), Failure> {
    let vars = |name: &str| std::env::var_os(name);
    let home = std::env::var_os("HOME")
        .filter(|home| !home.is_empty())
        .map(PathBuf::from);
    let env = Env { vars: &vars, home };
    let file = ConfigFile::new(&env, &RealFs);
    let options = SettingOptions {
        base_url: None,
        model: None,
        times: (None, None),
    };
    let settings = model_settings(&env, &file, &options).map_err(Failure::from)?;
    let (key, _source) =
        find_key(&env, &file, settings.route, &settings.base_url_origin).map_err(Failure::from)?;
    Ok((key, ScoreSettings::new(settings)))
}

/// Run all cases. Gives the summary lines and the detail lines, or the
/// failure that stopped the run.
fn run(args: &Args) -> Result<(bool, String, String), Failure> {
    guard(&args.cases, "case file")?;
    if let Some(detail) = &args.detail {
        guard(detail, "detail file")?;
    }
    let cases = read_cases(&args.cases)?;
    let (key, score_settings) = settings()?;

    let stop = Arc::new(AtomicBool::new(false));
    let _ = signal_hook::flag::register(SIGINT, Arc::clone(&stop));
    let usage = Mutex::new(Usage::default());
    let setup = Setup {
        key: &key,
        settings: &score_settings,
        stop: &stop,
        usage: &usage,
    };

    let total = cases.len();
    let mut scores: Vec<Option<CaseScore>> = Vec::new();
    for (index, case) in cases.iter().enumerate() {
        eprintln!("jag-bench: case {} of {total}", index + 1);
        if case.answers.is_empty() {
            scores.push(None);
            continue;
        }
        scores.push(Some(run_case(case, &setup, &|_| {})?));
    }

    let scored: Vec<&CaseScore> = scores.iter().flatten().collect();
    let jag = mean(&scored.iter().map(|score| score.jag).collect::<Vec<_>>());
    let grep = mean(&scored.iter().map(|score| score.grep).collect::<Vec<_>>());
    let passes = bar_passes(jag, grep);
    let summary = format!(
        "jag-bench: {total} cases, {} with no answer\n\
         jag-bench: ndcg@10 jag {jag:.3}, grep {grep:.3}\n\
         jag-bench: bar {} (jag {BAR:.2} or more, and higher than grep)\n\
         jag-bench: prompt {PROMPT_VERSION}, threshold {THRESHOLD}, model {}, jag {}\n",
        total - scored.len(),
        if passes { "pass" } else { "fail" },
        score_settings.model,
        env!("CARGO_PKG_VERSION"),
    );
    let mut detail = String::new();
    for (index, score) in scores.iter().enumerate() {
        let line = match score {
            Some(score) => json!({
                "case": index + 1,
                "jag": round3(score.jag),
                "grep": round3(score.grep),
                "answers": score.ranks.iter()
                    .map(|(jag, grep)| json!({"jag_rank": jag, "grep_rank": grep}))
                    .collect::<Vec<_>>(),
            }),
            None => json!({"case": index + 1, "jag": null, "grep": null, "answers": []}),
        };
        detail.push_str(&line.to_string());
        detail.push('\n');
    }
    Ok((passes, summary, detail))
}

fn main() -> ExitCode {
    let args = Args::parse();
    match run(&args) {
        Ok((passes, summary, detail)) => {
            if let Some(path) = &args.detail
                && fs::write(path, detail).is_err()
            {
                eprint!(
                    "{}",
                    render(&failure_of(
                        "cannot write the detail file.",
                        "Check that the folder exists and that you can write to it."
                    ))
                );
                return ExitCode::from(2);
            }
            let _ = std::io::stdout().write_all(summary.as_bytes());
            ExitCode::from(u8::from(!passes))
        }
        // A Ctrl-C ends the run with no message.
        Err(failure) if failure.exit_code == 130 => ExitCode::from(130),
        Err(failure) => {
            eprint!("{}", render(&failure));
            ExitCode::from(failure.exit_code)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_file_in_the_source_folder_is_refused() {
        let inside = Path::new(env!("CARGO_MANIFEST_DIR")).join("not-yet/detail.jsonl");
        assert!(guard(&inside, "detail file").is_err());
        assert!(guard(Path::new("/"), "detail file").is_ok());
    }
}
