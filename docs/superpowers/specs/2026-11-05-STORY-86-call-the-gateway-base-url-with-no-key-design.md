---
title: "Send the model calls through an AI gateway: call the gateway base URL with no key when the user sets no gateway key"
feature: FEATURE-21
stories: [STORY-86]
kind: build
gh_issue: 356
lifecycle_state: draft
inherits: [COMPONENT-01, COMPONENT-02, COMPONENT-05, COMPONENT-07, COMPONENT-10, INTERFACE-02, CONVENTION-06]
session: b90dc59a-2eb4-43ef-9199-b45e4d4604b0
---

# Send the model calls through an AI gateway: call the gateway base URL with no key when the user sets no gateway key

## Problem

The operator runs a model server, for example a Clef server, that has no key
check. The operator sets the base URL and no gateway key. jag must call the
server with no key, so that the server works with no extra setting
(STORY-86, CAPABILITY-13).

Today, `find_key` in `src/config.rs` stops the gateway route with
`GatewayKeyMissing` (exit 3) before the call. ADR-0028 (accepted
2026-10-29) changes this rule of ADR-0017, and DECISION-59, DECISION-71, and
DECISION-75 give the behavior. The code does not follow them yet.

The acceptance criteria:

- CRITERION-155: with a base URL that is not TypeSafe, and with no
  `JAG_GATEWAY_API_KEY` and no `config.gateway-api-key`, a search calls the
  base URL with no `Authorization` header, and does not stop before the
  call. A server that answers gives the normal result. A TypeSafe key is not
  sent to that base URL.
- CRITERION-156: when the server answers `HTTP 401` or `HTTP 403` to a call
  with no key, jag exits with the key-not-accepted code of DECISION-05, and
  the message names `JAG_GATEWAY_API_KEY`. The `GatewayKeyMissing` stop is
  gone for this route.
- CRITERION-157: `jag --check` with a base URL that is not TypeSafe and no
  gateway key reports whether the server is reachable and whether it answers
  a scoring call. The key items say "not needed", and the credit item says
  "not available". With a server that is reachable and answers, the check
  passes.

## Decisions

| # | Decision | Choice | Source |
|---|---|---|---|
| 1 | Spec grouping | 1 spec for STORY-86, the only open story of FEATURE-21 | `docs/adlc.md` and the specifier role file: 1 feature gives 1 spec; STORY-57 and STORY-58 are done |
| 2 | The key of the gateway route | `find_key` looks in `JAG_GATEWAY_API_KEY`, then in `config.gateway-api-key`, as now. When neither is set, it gives "no key", not an error. A blank variable or a blank value in the file counts as not set. A key that is set but has a bad format still stops with `KeyFormatBad` (exit 3) | 0028-call-a-base-url-with-no-key-when-the-user-sets-no-gateway-key.md (accepted); DECISION-71; FEATURE-21 spec decisions 9 and 11 |
| 3 | The direct route | No change: with no TypeSafe key, `KeyMissing`, exit 3, before the call | 0028 ("the direct route" stays); DECISION-03 |
| 4 | No TypeSafe key to another host | No change: on the gateway route, `find_key` reads only the 2 gateway sources | 0017-use-a-separate-key-for-an-ai-gateway.md; 0028; DECISION-59 |
| 5 | The call with no key | The client sends no `Authorization` header. The other headers, the body, the times, the retry, and the proxy rule do not change | 0028 Decision |
| 6 | A refusal of a call with no key | `HTTP 401` or `HTTP 403` to a call with no key gives a new error `ScoreError::KeyNeeded { status }`, exit 4, with no retry, as `KeyNotAccepted` now. The texts are in "Errors" | 0028 Decision (exit of DECISION-05, the message names `JAG_GATEWAY_API_KEY`); DECISION-71; the client of `src/scorer/client.rs` (401 and 403 stop the attempt) |
| 7 | The words of the refusal | Cause: `the gateway did not accept a call with no key (HTTP <status>).` Fix with no `--base-url`: the 2 lines of the old `GatewayKeyMissing` fix. Fix with `--base-url`: `Set JAG_GATEWAY_API_KEY to the key of the gateway.` | Inferred: the cause of `KeyNotAccepted` on the gateway route (`gateway_text` in `src/scorer.rs`); the fixes of `GatewayKeyMissing` (FEATURE-21 spec decision 18 and its error table); with `--base-url`, `jag --setup` does not see the gateway, so that fix does not name `jag --setup` (FEATURE-21 spec, "Errors") |
| 8 | `GatewayKeyMissing` | It goes away, because no path gives it now | 0028; CRITERION-156 |
| 9 | The lines of `--check` with no gateway key | D-017: 6 lines. `a key is present` and `the service accepts the key` say `not needed`. `the account has credit` says `not available`. A new line `the server answers a scoring call` comes after `the service is reachable`. The key-file line does not change. A refusal of a call with no key (`KeyNeeded`) fails the new line (exit 4). A `402` fails the credit line (exit 5), and each cause of exit 6 fails `the service is reachable`, as now. The direct route and a gateway with a key keep their 5 lines | D-017 (decision-judge, 2026-11-06; detail, medium, reversible, visible): "the key items" of DECISION-75 is plural; FEATURE-02 spec (STORY-06) decision 16 (the line form is not a contract); FEATURE-02 spec decision 12 (the rules of exit 5 and exit 6 stay) |
| 10 | The key-file item of `--check` | No change on each route: the item checks the mode of the config file, also when the file holds no key | Inferred: DECISION-04 item 5; 0015-read-the-base-url-and-the-model-name-from-config-base-url.md Consequences (the mode check applies to each read of the file); CONVENTION-06 |
| 11 | The new status words | `not needed` and `not available`. An item with one of these words does not fail the check. `jag is ready.` shows when no item fails and no item is `not checked`. The exit code is the code of the first item that fails, as now | DECISION-75 ("A key item that is not needed does not fail the check"); CRITERION-157 ("the check passes"); DECISION-48 |
| 12 | The status column | It stays 12 characters wide (`{:<12}` in `Report::lines`). A word of 12 characters or more (`not available`) gets 1 space after it. The lines of the other routes do not change | Inferred: `Report::lines` in `src/check.rs`; the check output of the other routes is released (FEATURE-02) |
| 13 | `jag-bench` | It uses `find_key` and the client, so on the gateway route with no key it also calls with no key | Inferred: `src/bench_main.rs` |
| 14 | Secrecy | No change. With no key, jag sends no secret. The proxy rule of the client stays | CONVENTION-06; 0028 Consequences ("The call sends no key, so no secret leaves the machine") |
| 15 | Test strategy | End-to-end tests with the fake model server, which records each header of each request. Unit tests for `find_key`, the client, the texts, and the check report | FEATURE-21 spec; `tests/common/fake_server.rs` (`Request::header`); 0028 Consequences (an end-to-end case for the gateway route with no key) |
| 16 | Docs | README: the gateway section and the check section. `--help` does not change | 0028; README "Use an AI gateway" ("With no gateway key, a search stops with exit 3 before the call") |
| 17 | Outside crates | No new crate | 0012-use-clap-serde-saphyr-and-rpassword-as-the-outside-crates.md |
| 18 | When the 6 lines show | Only when `find_key` gives no key. When `find_key` gives an error (for example bad YAML), or when item 5 fails and no gateway key variable is set, the check shows the 5 lines of today and makes no call, because jag cannot know if the file holds a gateway key | Inferred: FEATURE-02 spec decision 10; `criterion_10_an_open_key_file_that_is_the_source_shows_its_block_one_time`, `a_file_with_bad_yaml_fails_item_1_and_makes_no_call`, and `an_open_file_that_the_settings_need_fails_item_5_and_makes_no_call` in `tests/check.rs` |
| 19 | The new line when the call gives no scores | A `402` and each cause of exit 6 give `not checked` with the note ` (the call to the service failed)`. Only a `200` with the scores gives `pass`, and only `KeyNeeded` gives `fail` | Inferred: FEATURE-02 spec decision 11 (an item that jag cannot examine shows `not checked` and the reason); `CALL_FAILED_NOTE` in `src/check.rs` |
| 20 | A note after `not needed` and `not available` | None. The status word gives the reason, and the 3 lines keep the words of the other routes | Inferred: FEATURE-02 spec decision 11 (a note gives the reason of `not checked`); the D-017 card ("no line changes its words") |

## Design

### Config: the key of the route

`src/config.rs`:

- `find_key` gives `Result<Option<(ApiKey, KeySource)>, ConfigError>`. On the
  direct route it never gives `Ok(None)`: no key is `KeyMissing`, as now. On
  the gateway route, `Ok(None)` means that no gateway source is set.
- Remove `ConfigError::GatewayKeyMissing` and its texts.
- The callers: `search` in `src/cli.rs`, `check::run`, and
  `src/bench_main.rs`.

### Scorer: the call and the refusal

`src/scorer/client.rs`:

- `Client::new` takes `Option<&ApiKey>`. With `None`, the client keeps no
  `Authorization` value, and `call` sends no `Authorization` header.
- `call` gives `ScoreError::KeyNeeded { status }` for `401` and `403` when
  the client has no key, and `KeyNotAccepted { status }` when it has a key.
  Both stop the attempt, with no retry.

`src/scorer.rs`:

- `score`, `score_with_usage`, and `probe` take `Option<&ApiKey>`.
- `KeyNeeded` has exit 4. It comes only on the gateway route, because the
  direct route always has a key.

### Errors

| Error | Cause | Fix | Exit |
|---|---|---|---|
| `KeyNeeded`, base URL from `JAG_BASE_URL` or `config.base-url` | `the gateway did not accept a call with no key (HTTP <status>).` | 2 lines: ``Get a key from the gateway, then run `jag --setup`,`` and `or set JAG_GATEWAY_API_KEY.` | 4 |
| `KeyNeeded`, base URL from `--base-url` | `the gateway did not accept a call with no key (HTTP <status>).` | `Set JAG_GATEWAY_API_KEY to the key of the gateway.` | 4 |

In `--json` mode, the error is a JSON error document (DECISION-07), as now.
The texts of `KeyNotAccepted` (a call with a key) do not change.

### The flows

| Flow | Gateway route, no gateway key |
|---|---|
| Search with `--ask` | 1 call with no `Authorization` header. A `200` gives the normal result. `401` or `403` gives `KeyNeeded`, exit 4. Each other answer gives the failure of today |
| Search with no question | No change: no key and no call |
| `--check` | See "The check" |
| `jag-bench` | The calls go with no `Authorization` header |
| `--setup`, `--forget-key` | No change |

### The check

`src/check.rs`. Row 9 (D-017): on the gateway route, when `find_key` gives
no key, the check shows 6 lines:

| Line | Text | Result |
|---|---|---|
| 1 | `a key is present` | `not needed` |
| 2 | `the service accepts the key` | `not needed` |
| 3 | `the account has credit` | `not available`. A `402` (`NoCredit`) fails it (exit 5) |
| 4 | `the service is reachable` | As now: `pass` for an answer from the server (also `401`, `402`, and `403`). Each cause of exit 6 fails it |
| 5 | `the server answers a scoring call` | `pass` for a `200` with the scores. `KeyNeeded` (`401`, `403`) fails it (exit 4). A `402` and each cause of exit 6 give `not checked` (` (the call to the service failed)`) (row 19) |
| 6 | `other users cannot read the key file` | No change (row 10) |

So the cause of exit 5 fails the credit line, and each cause of exit 6
fails the reachable line, as FEATURE-02 spec decision 12 says. Exit 4 fails
line 5. The exit code is the code of the first line that fails
(DECISION-48). Lines 1 to 3 have no note (row 20).

When `find_key` gives an error, or when item 5 fails and no gateway key
variable is set, the check shows the 5 lines of today and makes no call
(row 18).

A server that answers, and no config file. Exit 0:

```
not needed  a key is present
not needed  the service accepts the key
not available the account has credit
pass        the service is reachable
pass        the server answers a scoring call
pass        other users cannot read the key file (no key file)
jag is ready.
```

A server that answers `401`, the base URL from `JAG_BASE_URL`, and no config
file. Exit 4:

```
not needed  a key is present
not needed  the service accepts the key
not available the account has credit
pass        the service is reachable
fail        the server answers a scoring call
pass        other users cannot read the key file (no key file)
```

The error stream:

```
jag: the gateway did not accept a call with no key (HTTP 401).
Get a key from the gateway, then run `jag --setup`,
or set JAG_GATEWAY_API_KEY.
```

`Status` gets `NotNeeded` and `NotAvailable`. `Report` holds a list of items
in place of `[Item; 5]`. The call closure of `check::run` takes
`Option<&ApiKey>`. The other routes keep their 5 lines and their bytes.

### Tests

| Criterion or rule | Test |
|---|---|
| CRITERION-155 | `tests/gateway.rs`: change `a_typesafe_key_is_not_a_gateway_key`. With `JAG_TYPESAFE_API_KEY`, `TYPESAFE_API_KEY`, and `config.api-key` set, `JAG_BASE_URL` to the fake server, and no gateway key: exit 0, 1 request, `request.header("authorization")` is `None`, and no output and no request holds the TypeSafe key. The same with `--base-url` |
| CRITERION-155, the normal result | `tests/gateway.rs`: a search with no gateway key gives the same stdout as the same search with a gateway key |
| CRITERION-156 | `tests/gateway.rs`: a fake server that answers `401`, and one that answers `403`, to a call with no key: exit 4, 1 request (no retry), and the cause and the fix of "Errors", for `JAG_BASE_URL` and for `--base-url`. With `--json`: a JSON error document with `exit_code` 4 |
| CRITERION-156, a call with a key | `tests/gateway.rs`: with `JAG_GATEWAY_API_KEY` set, a `401` keeps the `KeyNotAccepted` text of today |
| Row 2 | `src/config.rs` unit: the gateway route with no source gives `Ok(None)`; a blank `JAG_GATEWAY_API_KEY` gives `Ok(None)`; a bad gateway key still gives `KeyFormatBad`; the direct route with no source still gives `KeyMissing` |
| Row 5 | `src/scorer/client.rs` unit, or `tests/score.rs`: a client with no key sends the same headers as a client with a key, less `Authorization` |
| CRITERION-157 | `tests/check.rs`: change `the_key_of_the_direct_route_does_not_count_on_the_gateway_route`. With a fake server that answers: the first block of "The check", exit 0, and 1 request with no `Authorization` header. A `401` server and a `403` server: the second block, exit 4, and the error stream has the `KeyNeeded` texts. A server that does not answer: line 4 fails, line 5 is `not checked`, exit 6. A `402` server: line 3 fails, line 5 is `not checked`, exit 5 |
| CRITERION-10, no key | `tests/check.rs`: `criterion_10_no_key_names_the_problem_and_the_fix_and_makes_no_call` runs on the gateway route, so it becomes a case of CRITERION-157 with no key (exit 0 and 1 call with no `Authorization` header). The case of the direct route with no key (exit 3, no call) stays in the `src/check.rs` unit test `the_lines_have_a_status_field_of_12_characters` |
| Row 11 | `src/check.rs` unit: `jag is ready.` shows with items that are `not needed` and `not available`, and does not show with an item that is `not checked` |
| Row 12 | `src/check.rs` unit: the lines of the direct route and of a gateway with a key keep their 5 lines and their bytes |
| Row 8 | Remove the unit tests of `GatewayKeyMissing` in `src/config.rs` (at about line 1016) and `src/check.rs` (at about line 618). In `tests/gateway.rs`, the expected texts at about lines 383 and 400 (`a_typesafe_key_is_not_a_gateway_key`) change with the CRITERION-155 row. `the_lines_have_a_status_field_of_12_characters` keeps its bytes (row 12) |
| Row 18 | `tests/check.rs`: `criterion_10_an_open_key_file_that_is_the_source_shows_its_block_one_time`, `a_file_with_bad_yaml_fails_item_1_and_makes_no_call`, and `an_open_file_that_the_settings_need_fails_item_5_and_makes_no_call` keep their 5 lines, their exit code, and no call |

### Docs

`README.md`:

- "Use an AI gateway": replace "With no gateway key, a search stops with
  exit 3 before the call." with: with no gateway key, jag calls the gateway
  with no key. When the gateway refuses the call, jag exits with 4, and the
  message names `JAG_GATEWAY_API_KEY`.
- "Check that jag is ready": on a gateway with no gateway key, the check
  shows 6 lines. The 2 key items say `not needed`, the credit item says
  `not available`, and the line `the server answers a scoring call` comes
  after `the service is reachable`. Add the first block of "The check". In
  the exit code table, the cause of `0` becomes "No item fails".
  The cause of `4` becomes "The service does not accept the key, or a gateway refuses a call with no key". The sentence "The check shows `pass` or `fail` for five items, in this order:" becomes "The check shows a status for five items, in this order (six on a gateway with no gateway key):".

## Build order

1. Config: `find_key` and the removal of `GatewayKeyMissing`. Tests of row 2.
2. Scorer: the client with no key, `KeyNeeded`, and its texts. Tests of row 5
   and of CRITERION-156.
3. Search and `jag-bench`: pass the key or no key. Tests of CRITERION-155.
4. Check: the status words, the column rule, the 6 lines of row 9, and rows
   18 to 20. Tests of CRITERION-157, of CRITERION-10, and of rows 11, 12,
   and 18.
5. README.

## Assumptions

- A1 (row 7). The words of `KeyNeeded` come from the 2 texts of the gateway
  route that exist: the cause of `KeyNotAccepted` and the fixes of
  `GatewayKeyMissing`.
- A2 (row 10). The config file can hold `config.api-key`, so the mode check
  of item 5 stays also on a route that sends no key.
- A3 (row 12). The width of the status column stays, so that the released
  output of the other routes keeps its bytes.
- A4 (row 13). `jag-bench` follows the search, because it uses the same
  `find_key` and the same client.
- A5 (row 18). When jag cannot read the config file, it cannot know if the
  file holds a gateway key, so the check keeps the lines of today.
- A6 (row 19). A `402` or a failed call gives no scores, so jag cannot
  examine line 5.
- A7 (row 20). The status words `not needed` and `not available` give the
  reason, so the 3 lines need no note.

## Risks

- A user who forgets the gateway key for a gateway that needs one now sees
  the error after 1 call, not before (0028 Consequences). Signal: exit 4 with
  the `KeyNeeded` text. The call sends no key.
- With no gateway key, the snippets of the search now go to the base URL.
  Before this change, jag stopped before the call. The user chose the base
  URL, so this is the intended behavior. Signal: a report of a call to a base
  URL with a typing error.
- On a gateway with no key, the check has 6 lines, and on the other routes 5.
  A script that counts the lines can fail. The exit code is the contract
  (FEATURE-02 spec decision 16). Signal: a report of a script that reads the
  lines.
- A gateway that answers a call with no key with a status other than `401` or
  `403` (for example `404`) gives the failure of that status, not
  `KeyNeeded`. Signal: a report of exit 6 from a gateway that needs a key.

## Out of scope

- The other parts of CAPABILITY-13: DECISION-65 and DECISION-76 (a call or a
  snippet that is too long for the server), and DECISION-74 (the report names
  the host). They are in SCOPE-04 and have no roadmap item.
- CAPABILITY-14 (private patterns).
- A change of the texts of `KeyNotAccepted` (a call with a key).
