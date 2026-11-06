---
title: "The config file holds each non-secret setting, and setup shows each default: the file keys, and the comment lines of setup"
feature: FEATURE-29
stories: [STORY-84, STORY-85]
kind: build
gh_issue: 349
lifecycle_state: draft
inherits: [COMPONENT-01, COMPONENT-02, COMPONENT-07, CONVENTION-06, CONVENTION-07]
session: b90dc59a-2eb4-43ef-9199-b45e4d4604b0
---

# The config file holds each non-secret setting, and setup shows each default: the file keys, and the comment lines of setup

## Problem

The agent builder sets each non-secret setting of jag one time, in
`~/.config/jag/config.yml`, and not in each shell or job (STORY-84). After
`jag --setup`, the file shows each setting that it does not set, as a comment
line with its default (STORY-85).

The acceptance criteria:

- CRITERION-151: with `config.timeout` in the file, and with no `--timeout`
  and no `JAG_TIMEOUT`, one model call uses the time in the file. The order
  is the option, the variable, the file, then the default. A bad value gives
  exit 2 with the words of a bad `JAG_TIMEOUT`, and the message names
  `config.timeout` and the file.
- CRITERION-154: the rule of CRITERION-151 is also true for
  `config.retry-wait`, `config.max-scored`, and `config.max-chars`.
  `--setup` and `--forget-key` accept a file with these keys, and keep them.
- CRITERION-152: after `jag --setup` on a new file, or on an older file, the
  file has 1 comment line with the default for each non-secret setting that
  the file did not have as a value or as a comment line. Values and keys stay
  as they are. With only the comment line, jag uses the default.
  `--forget-key` keeps each comment line and each setting.
- CRITERION-153: a second `jag --setup` with the same answers does not change
  the bytes of the file.

Today, only an option and a `JAG_` variable set the 2 times and the 2
limits (`src/config/model.rs`, `src/config/limit.rs`). `file_text` in
`src/config/change.rs` writes the whole file again from the values, so each
comment line goes away at the next `--setup` or `--forget-key`. ADR-0029
(accepted, the answer of the user through hitl, 2026-11-05) gives the rule.
This spec gives the code.

## Decisions

| # | Decision | Choice | Source |
|---|---|---|---|
| 1 | Spec grouping | 1 spec for STORY-84 and STORY-85. STORY-85 shows the keys of STORY-84, and both change the same write of the file | `docs/adlc.md` and the specifier role file: 1 feature gives 1 spec; STORY-85 `blocked_by` STORY-84 |
| 2 | The settings and their keys | 6 settings, in this order: `base-url`, `model`, `timeout`, `retry-wait`, `max-scored`, `max-chars`, under `config:`. This spec adds the last 4. The defaults come from the code constants: `DEFAULT_BASE_URL`, `DEFAULT_MODEL`, `DEFAULT_TIMEOUT`, `DEFAULT_RETRY_WAIT`, `DEFAULT_COST_LIMIT`, `DEFAULT_SIZE_LIMIT` | 0029-read-each-setting-from-the-config-file-and-let-setup-write.md (accepted); STORY-84 scope |
| 3 | The order of the sources | For each setting by itself: the option, then the variable, then the file, then the default. The first source that is set wins. A source that is set but bad stops the lookup. A source that wins hides a bad value of a lower source of the same setting | 0029 (accepted); CONVENTION-07; FEATURE-21 spec decision 5 |
| 4 | The text of a file value | jag reads the scalar text of the key, and gives it to the parse of the variable: `parse_seconds`, `parse_cost_limit`, or `parse_size_limit`. So `04` passes, and `4.0`, `0x0A`, `20_000`, `1e9`, and `-1` give exit 2, as for the variable. A blank text, an empty value (`timeout:`), and a YAML null (`~`, `null`) count as not set | 0029 ("the same text rules"); FEATURE-21 spec decision 9 (a blank value in the file counts as not set); the probe of Assumption A1 |
| 5 | The type of the file fields | `Option<String>`, the type of the other keys. serde-saphyr 2.0.0 gives the text of a plain scalar (`4`) to a `String` field. No custom type. A tagged scalar (`!!int 4`) or a list (`[4]`) gives a YAML error, exit 3, as `api-key: [x]` does now. A unit test keeps this behavior | 0029 Consequences ("for example as text"); 0012-use-clap-serde-saphyr-and-rpassword-as-the-outside-crates.md; Assumption A1 |
| 6 | A bad value in the file | Exit 2. The cause has the words of the cause for the variable, and names `config.<key> in <path>`. The fix has the form of the fix of `config.base-url`. No text shows the value. The texts are in "Errors" below | CRITERION-151; CRITERION-154; 0029; `FileBaseUrlBad` in `src/config.rs`; CONVENTION-06 |
| 7 | The flows that read a key | Each flow that reads the variable reads the key, at the same step: a search with `--ask` (all 4), `--json` with no question (`max-chars` only), `--check` and `jag-bench` (the 2 times, through `model_settings`). A search with no question and no `--json` reads no variable, so it reads no key. `--setup` and `--forget-key` read no value of the 4 keys | 0029 ("in each flow that reads the variable"); FEATURE-23 spec decision 32 (`own_options` in `src/cli.rs`); README "With no question" |
| 8 | When jag reads the file | At most 1 time in each run. The settings, the key, and the 2 limits share 1 `ConfigFile`. jag reads the file only when a setting or the key has no source above the file | FEATURE-21 spec decision 12; 0015-read-the-base-url-and-the-model-name-from-config-base-url.md Consequences |
| 9 | `--json` with no question | jag reads the file only when `--max-chars`, `--no-max-chars`, and `JAG_MAX_CHARS` are not set. Then the file checks apply: a file that other users can read, that jag cannot read, that is not a regular file, or that is not valid YAML gives exit 3, as a JSON error document. No `HOME` or no file gives the default. This replaces FEATURE-12 spec decision 4 ("a key file that other users can read does not stop plain grep") for `--json` with no question only. A search with no question and no `--json` keeps decision 4 | 0029 Consequences ("`--json` with no `--ask` ... the file checks apply to it"); FEATURE-12 spec decision 4; DECISION-07 |
| 10 | "No limit" for `config.max-chars` | No file form. `config.max-chars` takes the text rules of `JAG_MAX_CHARS`: a whole number of 1,000 or more. Only `--no-max-chars` turns the limit off, for 1 search | Inferred: 0029 says that a key works as its variable with the same text rules, and lists `--no-max-chars` with the options of 1 search that are not settings. `JAG_MAX_CHARS` has no value for no limit (`src/config/limit.rs`). DECISION-25 and CRITERION-43: `0`, a negative number, or a text that is not a number is a usage error, so `0` cannot mean no limit. DECISION-20: the user turns the limit off for a search with `--ask` or `--json` |
| 11 | Strict parse and the other-keys text | `StrictSection` knows the 8 keys. The text of `KeyFileHasOtherKeys` names the 8 keys, in the order of the file | 0029 Consequences; FEATURE-01 spec decision 18 |
| 12 | The form of a comment line | `<indent># <name>: <default>`, with the indent of the section (row 19). The default is plain text, not the output of `yaml_scalar`: `# timeout: 4`, not `# timeout: "4"`. The comment lines go in the order of row 2 | 0029 Decision (`# timeout: 4`, `# model: jev-1.13.0`); STORY-85 scope 1 |
| 13 | When a setting is in the file | When the section under `config:` has a value line of the key, or a comment line whose text after `#` and the spaces starts with `<name>:`. Then setup adds no line for it | STORY-85 scope 2; 0029 Decision |
| 14 | An old default in a comment line | A later `--setup` does not change a comment line, also one that shows the default of an older release. So CRITERION-153 is true also for a second setup of a later release | Inferred: 0029 Decision (the write "adds only the missing comment lines, and changes only the line of a key that it writes"); STORY-85 scope 2; FEATURE-01 spec decision 18 (jag never deletes text that the user wrote) |
| 15 | The setup paths that add comment lines | Each path of `--setup` that ends with exit 0 adds the missing comment lines: the store path (a key is typed), and the keep path (the answer "no" to "Replace it?"). The keep path writes the file only when a line is missing. A path that fails writes nothing, as now | Inferred: CRITERION-152 ("an older file" has a key, so setup asks "Replace it?"); CRITERION-153; FEATURE-01 spec ("Each error stores nothing and changes nothing", `load_for_change`) |
| 16 | Setup output | No new line. Setup prints the lines of today | Inferred: STORY-85 and CRITERION-152 name no message; the output of `setup` in `src/config/setup.rs` |
| 17 | The write keeps the lines | `--setup` and `--forget-key` edit the lines of the file. They keep each line that they do not change, also each comment line. They change only the line of the key that they write or remove, and they add only the missing comment lines and a missing `config:` line. serde-saphyr stays the reader. jag does not write the file with serde-saphyr | 0029 Consequences; COMPONENT-02 ("with no loss of the other keys and the comment lines of the file") |
| 18 | The form that jag can change | jag changes only a file in the block form: `config:` on its own line, and each value on 1 line under it. For each other form, `--setup` and `--forget-key` stop with exit 2 before the first question, and the file does not change. After each edit, jag parses the new text again. If the values are not the expected values, jag stops with exit 2 and writes nothing | Inferred: FEATURE-01 spec decision 18 (refuse to change a file, so that jag never deletes text that the user wrote) and decision 9 (exit 2 for a file that setup refuses); 0029 Consequences |
| 19 | Where new lines go | A new key line goes directly under the `config:` line. The missing comment lines go after the last line of the section that is not blank. The indent is the indent of the value lines of the section, or 2 spaces when the section has no value line. With no `config:` line, jag adds `config:` at the end of the file | Inferred: `file_text` in `src/config/change.rs` (the key first, 2 spaces); 0029 ("indented under `config:`") |
| 20 | Line endings | Each line keeps its line ending. A new line takes `\r\n` when the `config:` line (or, with no `config:` line, the first line) ends with `\r\n`, else `\n`. When the last line has no line ending, jag adds one before a new line | Inferred: row 17 (keep each line) |
| 21 | `--forget-key` and the file | `--forget-key` removes the line of the key of the route. It removes the file only when the file then holds no value and no comment line, as now for a file that holds only the key. Else it writes the edited text. It adds no comment line | CRITERION-152 ("`jag --forget-key` keeps each comment line and each setting"); STORY-85 scope 5; `forget_key` step 5 in `src/config/setup.rs` |
| 22 | Secrecy | No message shows a value of the file. The text of the file is a key holder: its type has no `Display`, and its `Debug` shows no line text | CONVENTION-06; FEATURE-01 spec decision 10; `FileValues` in `src/config/change.rs` |
| 23 | Test strategy | Unit tests in `src/config` with real temporary folders and the fake prompter. End-to-end tests in `tests/` with the fake model server and a temporary `HOME`. The successful setup output stays in the unit tests, because an end-to-end test cannot type a key | FEATURE-01 spec decision 17; FEATURE-21 spec; `tests/setup.rs` |
| 24 | Docs | README: a table of the 6 settings, the order, the comment lines of setup, and the changed lines about the size limit and about a search with no question. `--help` does not change | 0029; README "Use an AI gateway" (the table form) |
| 25 | Outside crates | No new crate. serde-saphyr reads the file. The line edit is plain Rust in `src/config/change.rs`, as `file_text` is now | 0012 (accepted): the crates; 0029 Consequences: "jag does not write the file with `serde-saphyr`" and "`serde-saphyr` stays the reader". The text of 0012 says that serde-saphyr reads and writes the file, and rejects a hand-written read and write. 0029 is later, keeps the typed read, and decides the write. The architect links the 2 ADRs (a `TASK` suggestion of this spec); this spec does not wait for it |

## Design

### Data: the keys and the file structs

`src/config.rs`:

- `Section` (the loose parse) and `StrictSection` (the strict parse) each get
  4 fields, `Option<String>`: `timeout`, `retry_wait` (`#[serde(rename =
  "retry-wait")]`), `max_scored` (`max-scored`), and `max_chars`
  (`max-chars`). `StrictSection` keeps `deny_unknown_fields` (row 11).
- `KeyFileHasOtherKeys`: the cause is "`<path>` holds settings other than
  config.api-key, config.gateway-api-key, config.base-url, config.model,
  config.timeout, config.retry-wait, config.max-scored, and
  config.max-chars." The fix does not change.

`src/config/change.rs`:

- 1 table of the settings, in the order of row 2: the name of the key and its
  default text. The defaults come from the constants of `model.rs` and
  `limit.rs` (`DEFAULT_TIMEOUT.as_secs()`, and so on), so a release that
  changes a constant changes the comment line of a new file. The comment
  lines and the other-keys text use this table. A unit test checks that a
  file with each key of the table passes the strict parse, and that each key
  of the table has a field in `FileValues`. A later story that adds a
  setting adds 1 row to this table (STORY-84: "a setting that a later story
  adds ... also gets a file key and a comment line").
- `FileValues` gets the 4 texts, as the file gives them, with the same
  rule as now: a blank or empty value counts as not set. `Debug` shows only
  "set" or "not set" for each field.

### Read: the order, the text rules, and the errors

`src/config/model.rs`:

- `time()` takes the `ConfigFile` and the key name. The steps: the option;
  else the variable (a blank variable counts as not set, a value that is not
  text is bad); else the text of the key in the file (a blank text counts as
  not set); else the default. jag calls `file.get()` only at the third step
  (row 8). A bad file text gives `FileTimeBad { setting, path }`.
- `model_settings` reads in the order of today: the base URL, the model, the
  timeout, then the retry wait. So with 2 bad values, the first one in this
  order gives the error, as for the variables.

`src/config/limit.rs`:

- `cost_limit(env, file, option)` and `size_limit(env, file, option)` take the
  `ConfigFile`, with the same steps as `time()`. A bad file text gives
  `FileCostLimitBad { path }` or `FileSizeLimitBad { path }`.
- A number too large for `usize` gives `usize::MAX`, as for the variable
  (`parse_size_limit`).

`src/cli.rs`:

- `search`: pass the `ConfigFile` of the run to `cost_limit` and
  `size_limit`. The order of the steps stays: the settings, the key, the cost
  limit, the size limit.
- `plain_json`: make 1 `ConfigFile` and pass it to `size_limit` (row 9).
- `own_options` (a search with no question and no `--json`) does not change.
- `check` and `bench_main.rs` get the 2 times from the file through
  `model_settings`. A bad time in the file is not a file error
  (`is_file_error` is false), so it stops `--check` with exit 2, as a bad
  `JAG_TIMEOUT` does.

### Errors

Each new error has exit 2. No text shows the value (row 22).

| Error | Cause | Fix |
|---|---|---|
| `FileTimeBad`, timeout | `config.timeout in <path> is not a whole number of seconds from 1 to 60.` | `Set config.timeout in <path> to a number such as 10, or remove it.` |
| `FileTimeBad`, retry wait | `config.retry-wait in <path> is not a whole number of seconds from 1 to 60.` | `Set config.retry-wait in <path> to a number such as 2, or remove it.` |
| `FileCostLimitBad` | `config.max-scored in <path> is not a whole number from 1 to 2,000.` | `Set config.max-scored in <path> to a number such as 400, or remove it.` |
| `FileSizeLimitBad` | `config.max-chars in <path> is not a whole number of 1,000 or more.` | `Set config.max-chars in <path> to a number such as 40000, or remove it.` |
| `KeyFileNotEditable` | `jag cannot change <path>, because it is not in the form that jag writes.` | `Edit the file by hand. Put config: on its own line, and each setting on its own line under it.` |

The examples are the examples of the fix for the variable. In `--json` mode,
each error is a JSON error document (DECISION-07), as now.

### The flows

| Flow | Keys that it reads from the file |
|---|---|
| Search with `--ask` | `base-url`, `model`, `timeout`, `retry-wait`, `max-scored`, `max-chars`, and the key of the route |
| `--json` with no question | `max-chars` only |
| Search with no question and no `--json` | none, and jag does not open the file |
| `--check` | `base-url`, `model`, `timeout`, `retry-wait`, and the key of the route |
| `jag-bench` | the settings of `model_settings`, as now |
| `--setup`, `--forget-key` | `base-url` for the route, as now. They keep the other values as text, and check none of them |

### Write: the line edit (STORY-85)

`load_for_change` keeps the text of the file in the state, in a type with no
`Display` and a `Debug` that shows no line text (row 22). It also gives the
line model of the text, or `KeyFileNotEditable` when the text is not in the
editable form. `setup` stops at once on `KeyFileNotEditable`, before the
first question. `forget_key` stops on it only when it must edit the file
(row 21).

The line model of a file:

1. Lines: the text in lines, each line with its line ending.
2. The `config:` line: the 1 line that starts at column 0 with `config:`,
   then only spaces and an optional `#` comment.
3. Before the `config:` line: only blank lines, comment lines, and `---`.
4. The section: each line after the `config:` line, to the end of the file.
   Each line of the section is blank, a comment line (any indent), or a value
   line.
5. A value line: an indented line whose text after the indent is
   `<name>:` and then a space, a tab, or the end of the line, where `<name>`
   is 1 of the 8 keys. Each value line has the same indent, and each key has
   at most 1 value line.
6. A file with no line that is not blank, a comment line, or `---` has no
   `config:` line. That is the editable form too.

Each other form is not editable: for example `config: {api-key: k}`,
`config: null`, a value on 2 lines or more (a block scalar `|` or `>`, or a
plain or quoted scalar that goes on to the next line), a document end `...`,
or a second document. For such a file, `--setup` and `--forget-key` give
`KeyFileNotEditable` (exit 2), and the file does not change. Exception: when
`--forget-key` removes the file (row 21), it needs no edit, so the form does
not matter. A comment line, for this rule and for row 21, is a line whose
first character that is not a space is `#`.

The edits:

- Set a key: replace its value line with `<indent><name>: <yaml_scalar(key)>`.
  With no value line, add that line directly under the `config:` line. A
  comment at the end of the old line goes away with the line.
- Remove a key: remove its value line.
- Add the missing comment lines (row 13): after the last line of the section
  that is not blank, in the order of row 2.
- With no `config:` line: add `config:` at the end of the file, then the
  lines.

The check after each edit: parse the new text with the loose and the strict
structs. Its `FileValues` must be the expected values: the old values with
only the change of the key. If 1 check fails, give `KeyFileNotEditable`, and
write nothing.

`write_key_file` takes the new text in place of the values. The write
through a temporary file with mode `0o600` and a rename stays as it is.
`file_text` goes away.

The text of a new file from `--setup` on the direct route, with a key `K`
that `yaml_scalar` writes as it is:

```yaml
config:
  api-key: K
  # base-url: https://api.typesafe.ai/v1
  # model: jev-1.13.0
  # timeout: 4
  # retry-wait: 1
  # max-scored: 200
  # max-chars: 20000
```

On the gateway route, the first line under `config:` is
`  gateway-api-key: K`, and the 6 comment lines are the same. The keys
`api-key` and `gateway-api-key` get no comment line.

### Setup

`setup` in `src/config/setup.rs`:

1. Steps 1 to 4 stay. Step 2 (`load_for_change`) also checks the form. A
   file that is not editable stops setup before the question.
2. The keep path (the answer "no"): when a comment line is missing, write
   the file with the missing comment lines. Then print "Kept the stored
   key." as now. When no line is missing, write nothing.
3. The store path: set the key line, add the missing comment lines, and
   write the file, as now.
4. A failure (no terminal, a typed key with a bad format, a failed prompt,
   an interrupt) writes nothing, as now.

A second setup with the same answers gives the same bytes: on the keep path,
no line is missing, so jag writes nothing. On the store path, the key line
gets the same text, and no comment line is missing.

### Forget-key

`forget_key` in `src/config/setup.rs`, step 5:

1. Remove the value line of the key of the route.
2. When the file then has no value and no comment line, remove the file, as
   now.
3. Else, write the edited text. The other values and each comment line
   stay.

`--forget-key` still does not check the mode. A write gives a new file with
mode `0o600`.

### Tests

| Criterion or rule | Test |
|---|---|
| CRITERION-151, the file sets the time | `tests/failure.rs`: copy `the_timeout_variable_sets_the_time_of_one_call` with `config:\n  timeout: 1\n` in a temporary `HOME` and no `JAG_TIMEOUT`: the call times out after 2 calls. Also: `JAG_TIMEOUT=1` wins over `timeout: 30`, and `--timeout 1` wins over `timeout: 30` |
| CRITERION-151, the order | `src/config/model.rs` unit: a table of the option, the variable, the file, and the default for the timeout, as `criterion_96_...` in `tests/gateway.rs` does for the model. An option or a variable hides a bad file value |
| CRITERION-151, a bad value | `tests/usage.rs`: for each bad text of the list of the `JAG_TIMEOUT` test (`abc`, `0`, `61`, `1.5`, and also `4.0`, `0x0A`, `-1`), `config.timeout` gives exit 2, the cause and the fix of "Errors", and the stderr has no copy of the value |
| CRITERION-151, the text rules | `src/config/model.rs` unit: `timeout: 4` (a plain YAML number) reads as 4 seconds; `timeout: 04` passes; `timeout:`, `timeout: ~`, and `timeout: "  "` give the default |
| CRITERION-154, retry wait | `tests/failure.rs`: `retry-wait` in the file sets the wait before the retry, with the pattern of the `JAG_RETRY_WAIT` test. `tests/usage.rs`: bad values, as for the timeout |
| CRITERION-154, max-scored | `tests/cost_limit.rs`: `max-scored: 10` in the file limits the scored results, as `JAG_MAX_SCORED=10` does; the variable wins over the file. `tests/usage.rs`: bad values |
| CRITERION-154, max-chars | `tests/size_limit.rs`: `max-chars: 1000` in the file sets the limit for `--ask` and for `--json` with no question; `JAG_MAX_CHARS` and `--max-chars` win over the file; `--no-max-chars` wins over the file. `tests/usage.rs`: bad values |
| Row 9 | `tests/plain_grep.rs`, next to `criterion_68_an_open_key_file_stops_a_question_search_and_not_plain_grep`: with a file of mode `0o644`, `--json needle dir` gives exit 3 and a JSON error document; `--json --max-chars 1000 needle dir` gives exit 0; `needle dir` gives exit 0 (the existing test stays) |
| CRITERION-154, setup and forget-key keep the keys | `tests/setup.rs`: a file `config:\n  api-key: k\n  timeout: 9\n  max-chars: 5000\n` and `--forget-key` gives the file `config:\n  timeout: 9\n  max-chars: 5000\n`. `src/config/setup.rs` unit: setup on the same file keeps the 2 lines |
| Row 5 | `src/config.rs` unit: the loose and the strict parse take `timeout: 4`; `timeout: !!int 4` and `timeout: [4]` give `KeyFileBad` (exit 3) |
| Row 11 | `src/config/change.rs` unit: a file with each key of the table passes the strict parse; `other: 1` gives `KeyFileHasOtherKeys` with the new text |
| CRITERION-152, a new file | `src/config/setup.rs` unit, fake prompter: setup with no file gives the bytes of "Write" above, on the direct route and on the gateway route |
| CRITERION-152, an older file | `src/config/setup.rs` unit: a file `config:\n  api-key: k\n  base-url: "https://openrouter.ai/api"\n` and the answer "no" keeps the 2 lines and adds the 5 comment lines that are missing, not `# base-url`. With the answer "yes" and a new key, only the key line changes |
| CRITERION-152, a comment line is not a value | `src/config/model.rs` unit: a file with only the 6 comment lines gives each default, with the origin `Default` |
| CRITERION-152, forget-key keeps the comment lines | `tests/setup.rs`: a file with a key line and the 6 comment lines, and `--forget-key`, gives the same file with no key line. A file with only a key line is removed (`forget_key_removes_the_stored_key` stays as it is) |
| CRITERION-153 | `src/config/setup.rs` unit: 2 setups with the same answers ("no", and the same key) give the same bytes. A setup on the keep path with no missing line does not write (a fake file system counts the writes) |
| Row 14 | `src/config/setup.rs` unit: a file with `# timeout: 3` keeps that line, and gets no `# timeout: 4` |
| Rows 17 to 20 | `src/config/change.rs` unit: the edit keeps comment lines at column 0 and indented, comments at the end of a line, blank lines, `---` at the start, `config:  # note`, `\r\n` line endings, and a file with no line ending at the end. A file with only comments gets `config:` at the end |
| Row 18 | `src/config/change.rs` unit: each form of the list in "Write" gives `KeyFileNotEditable`, exit 2, the file stays byte for byte, and no temporary file stays. An edit whose check fails writes nothing |
| Row 22 | `tests/secrecy.rs`: a bad file value `SENTINEL-...` for each of the 4 keys, and a key `SENTINEL-...` in a file that is not editable: no output shows `SENTINEL`. `src/config/change.rs` unit: the `Debug` of the state shows no line text |

### Docs

`README.md`:

- A new section "Settings", after "Use an AI gateway": a table of the 6
  settings with the option, the variable, the file key, and the default. The
  order of the sources. The comment lines of `jag --setup`: remove `# ` to
  set a value. A comment line shows the default of the release that wrote
  it; with only a comment line, jag uses the default of the release that
  runs.
- "The size limit": "you can also set the variable `JAG_MAX_CHARS`, or
  `config.max-chars` in the config file". Only `--no-max-chars` turns the
  limit off.
- "With no question": jag reads none of the variables and none of the file
  keys of these settings.
- "Use an AI gateway": the 2 rows of the base URL and the model stay. A link
  to "Settings" for the other settings.

The system design (the designer) and the ADRs (the architect) need no change
for this spec. DF-1 is closed, and COMPONENT-02 names ADR-0029.

## Build order

1. STORY-84, data: the 4 fields of `Section` and `StrictSection`, the table
   of the settings, the 4 texts of `FileValues`, and the other-keys text.
   Until step 4, `file_text` also writes the 4 values, so that `--setup` and
   `--forget-key` keep them (STORY-84 scope 4). Tests of rows 5 and 11.
2. STORY-84, read: `time()`, `cost_limit`, `size_limit`, the 4 new errors,
   and `search` and `plain_json` in `src/cli.rs`. Tests of CRITERION-151,
   CRITERION-154, and row 9.
3. STORY-84, docs: the README lines about the keys and the size limit.
4. STORY-85, write: the line model, the edits, the check after the edit,
   `KeyFileNotEditable`, and `write_key_file` with the text. Remove
   `file_text`. Tests of rows 17 to 20 and 18.
5. STORY-85, setup and forget-key: the keep path, the store path, and the
   rule of row 21. Tests of CRITERION-152, CRITERION-153, and row 14.
6. STORY-85, docs: the README lines about the comment lines.

## Assumptions

- A1 (rows 4 and 5). A probe on 2026-11-05, with serde-saphyr `=2.0.0` and
  serde 1.0.229 in Docker (`rust:1.99`), parsed these texts into a struct with
  `Option<String>` fields: `timeout: 4` gave `Some("4")`; `04`, `0x0A`,
  `4.0`, `true`, `-1`, `1e9`, `20_000`, `.inf`, and
  `99999999999999999999999` each gave their text; `"4"` gave `Some("4")`;
  `timeout:` and `timeout: ~` gave `None`; `timeout: [4]`, `timeout: !!int 4`,
  and a key 2 times gave an error; `config:` with only comment lines gave
  `config: None`; `config: {timeout: 4}` gave `Some("4")`; `\r\n` line
  endings parsed. The source: `deserialize_string` in
  `src/de/deserializer.rs` of serde-saphyr 2.0.0 takes the scalar text when
  `no_schema` is false, and the default of `no_schema` is false
  (`src/de/options.rs`). The unit test of row 5 keeps this behavior, so an
  update of serde-saphyr that changes it fails the test.
- A2 (row 10). The text rules of 0029 decide the file form of no limit. A
  number of 1,000 or more that is too large for `usize` gives `usize::MAX`
  (`parse_size_limit` in `src/config/limit.rs`), so a large number in the
  file works as no practical limit.
- A3 (row 14). 0029 Decision and STORY-85 scope 2 decide that a comment line
  stays as it is.
- A4 (row 15). CRITERION-152 names "an older file", and a file of an older
  release has a key, so setup asks "Replace it?". So the keep path adds the
  comment lines.
- A5 (row 16). No story or criterion asks for a message.
- A6 (rows 18 to 20). The form of the file that jag writes (`file_text` in
  `src/config/change.rs`) and the refusal of FEATURE-01 spec decision 18
  decide the editable form and the place of a new line.

## Risks

- A user with a file in another YAML form (a flow map, a value on 2 lines)
  can no longer run `--setup`, and `--forget-key` works only when it removes
  the file. Signal: an issue or a report of exit 2 with "not in the form that
  jag writes". Fix: a later release can edit more forms.
- After an update that changes a default, a comment line shows the old
  default. Signal: a user report that the file and the behavior do not
  agree. The README says that a comment line shows the default of the
  release that wrote it.
- `--json` with no question now stops with exit 3 on a config file that
  other users can read, when no option and no variable sets the size limit.
  Signal: an agent report of exit 3 in a search with no question. The fix in
  the message (`chmod 600`) stays.
- A comment line of the user that holds an old key (for example
  `# api-key: <old key>`) now stays after `--forget-key`, because jag keeps
  each comment line. Before this change, `--forget-key` removed a file that
  held only the key, with its comments. Signal: a key text in a comment line
  after `--forget-key`. jag never writes a key in a comment line.
- An older jag release that reads a file with the new keys: a search ignores
  them, but `--setup` and `--forget-key` of that release refuse the file
  (exit 2). Signal: a report after a downgrade.
- A change of serde-saphyr that gives a number in place of its text breaks
  row 5. Signal: the unit test of row 5 fails.

## Out of scope

- The place of the private patterns of DECISION-68 in the file: the spec of
  CAPABILITY-14 decides it (0029).
- The setting of DECISION-80: it gets a key and a comment line by the table of
  row 2 when its story adds it.
- A file form of no limit for `config.max-chars` (row 10).
- A change of a comment line that shows an old default (row 14).
- The fix text of a timeout failure ("give each call more time with
  `--timeout`", `src/scorer.rs`) does not change.
- `--help` does not change.
