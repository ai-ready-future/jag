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
| 9 | The lines of `--check` with no gateway key | **Pending decision** (`BLOCKED · decision`, see "Pending decision"). The recommended option is in the Design. The Build order puts this part last | DECISION-75 and QUESTION-04 do not map the 5 items of DECISION-04 to the 2 server items |
| 10 | The key-file item of `--check` | No change on each route: the item checks the mode of the config file, also when the file holds no key | Inferred: DECISION-04 item 5; 0015-read-the-base-url-and-the-model-name-from-config-base-url.md Consequences (the mode check applies to each read of the file); CONVENTION-06 |
| 11 | The new status words | `not needed` and `not available`. An item with one of these words does not fail the check. `jag is ready.` shows when no item fails and no item is `not checked`. The exit code is the code of the first item that fails, as now | DECISION-75 ("A key item that is not needed does not fail the check"); CRITERION-157 ("the check passes"); DECISION-48 |
| 12 | The status column | It stays 12 characters wide (`{:<12}` in `Report::lines`). A word of 12 characters or more (`not available`) gets 1 space after it. The lines of the other routes do not change | Inferred: `Report::lines` in `src/check.rs`; the check output of the other routes is released (FEATURE-02) |
| 13 | `jag-bench` | It uses `find_key` and the client, so on the gateway route with no key it also calls with no key | Inferred: `src/bench_main.rs` |
| 14 | Secrecy | No change. With no key, jag sends no secret. The proxy rule of the client stays | CONVENTION-06; 0028 Consequences ("The call sends no key, so no secret leaves the machine") |
| 15 | Test strategy | End-to-end tests with the fake model server, which records each header of each request. Unit tests for `find_key`, the client, the texts, and the check report | FEATURE-21 spec; `tests/common/fake_server.rs` (`Request::header`); 0028 Consequences (an end-to-end case for the gateway route with no key) |
| 16 | Docs | README: the gateway section and the check section. `--help` does not change | 0028; README "Use an AI gateway" ("With no gateway key, a search stops with exit 3 before the call") |
| 17 | Outside crates | No new crate | 0012-use-clap-serde-saphyr-and-rpassword-as-the-outside-crates.md |

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

`src/check.rs`. The recommended option of the pending decision (row 9):
on the gateway route with no gateway key, the check keeps its 5 items, and
item 2 examines the answer to the scoring call in place of the key:

| Item | Text on this route | Result |
|---|---|---|
| 1 | `a key is present` (` (no gateway key is set)`) | `not needed` |
| 2 | `the server answers a scoring call` | The call: `pass` for a `200` with the scores. `KeyNeeded` fails it (exit 4). `NoCredit` gives `pass` (the server answered). Each cause of exit 6 gives `not checked` (` (the call to the service failed)`) |
| 3 | `the account has credit` (` (jag sends no key, so there is no account)`) | `not available`. `NoCredit` (`402`) fails it (exit 5) |
| 4 | `the service is reachable` | As now: `pass` for an answer from the server (also `401`, `402`, and `403`); each cause of exit 6 fails it |
| 5 | `other users cannot read the key file` | No change (row 10) |

So each cause of exit 4 fails item 2, the cause of exit 5 fails item 3, and
each cause of exit 6 fails item 4, as FEATURE-02 spec decision 12 says. When
item 5 fails and no gateway key variable is set, item 1 shows `not checked`
(` (jag cannot use the key file: see the last item)`), and jag makes no call,
as now (FEATURE-02 spec decision 10).

The output with a server that answers, and no config file:

```
not needed  a key is present (no gateway key is set)
pass        the server answers a scoring call
not available the account has credit (jag sends no key, so there is no account)
pass        the service is reachable
pass        other users cannot read the key file (no key file)
jag is ready.
```

`Status` gets `NotNeeded` and `NotAvailable`. The call closure of
`check::run` takes `Option<&ApiKey>`. The other routes keep their lines and
their bytes.

### Tests

| Criterion or rule | Test |
|---|---|
| CRITERION-155 | `tests/gateway.rs`: change `a_typesafe_key_is_not_a_gateway_key`. With `JAG_TYPESAFE_API_KEY`, `TYPESAFE_API_KEY`, and `config.api-key` set, `JAG_BASE_URL` to the fake server, and no gateway key: exit 0, 1 request, `request.header("authorization")` is `None`, and no output and no request holds the TypeSafe key. The same with `--base-url` |
| CRITERION-155, the normal result | `tests/gateway.rs`: a search with no gateway key gives the same stdout as the same search with a gateway key |
| CRITERION-156 | `tests/gateway.rs`: a fake server that answers `401`, and one that answers `403`, to a call with no key: exit 4, 1 request (no retry), and the cause and the fix of "Errors", for `JAG_BASE_URL` and for `--base-url`. With `--json`: a JSON error document with `exit_code` 4 |
| CRITERION-156, a call with a key | `tests/gateway.rs`: with `JAG_GATEWAY_API_KEY` set, a `401` keeps the `KeyNotAccepted` text of today |
| Row 2 | `src/config.rs` unit: the gateway route with no source gives `Ok(None)`; a blank `JAG_GATEWAY_API_KEY` gives `Ok(None)`; a bad gateway key still gives `KeyFormatBad`; the direct route with no source still gives `KeyMissing` |
| Row 5 | `src/scorer/client.rs` unit, or `tests/score.rs`: a client with no key sends the same headers as a client with a key, less `Authorization` |
| CRITERION-157 | `tests/check.rs`: change `the_key_of_the_direct_route_does_not_count_on_the_gateway_route`. With a fake server that answers, the output is the block in "The check", exit 0. A `401` server: item 2 fails, exit 4, and the error stream has the `KeyNeeded` texts. A server that does not answer: item 4 fails, exit 6. A `402` server: item 3 fails, exit 5 |
| Row 11 | `src/check.rs` unit: `jag is ready.` shows with items that are `not needed` and `not available`, and does not show with an item that is `not checked` |
| Row 12 | `src/check.rs` unit: the lines of the direct route and of a gateway with a key keep their bytes |
| Row 8 | Remove the unit tests of `GatewayKeyMissing` in `src/config.rs` and `src/check.rs`, and change the expected texts at about `src/check.rs` line 618 and line 716 |

### Docs

`README.md`:

- "Use an AI gateway": replace "With no gateway key, a search stops with
  exit 3 before the call." with: with no gateway key, jag calls the gateway
  with no key. When the gateway refuses the call, jag exits with 4, and the
  message names `JAG_GATEWAY_API_KEY`.
- "Check that jag is ready": the lines for a server with no key, and the 2
  new status words.

## Pending decision

Row 9 waits for 1 `BLOCKED · decision`: how does `--check` show a server with
no key?

- **5 lines, item 2 changes its words:** the recommended option, in the
  Design. It keeps the 5 items of DECISION-04 and DECISION-48, and the rule of
  FEATURE-02 spec decision 12 (exit 4 fails item 2). Only item 1 says "not
  needed".
- **6 lines, a new item:** items 1 and 2 say "not needed", item 3 says "not
  available", and a new item `the server answers a scoring call` comes after
  item 4. Each key item says "not needed", as DECISION-75 says. The check has
  6 items on this route, and a refusal for a key fails the new item.

When the answer is the other option, only "The check", its tests, and the
README lines of the check change. Until the answer, STORY-86 stays in
`backlog`.

## Build order

1. Config: `find_key` and the removal of `GatewayKeyMissing`. Tests of row 2.
2. Scorer: the client with no key, `KeyNeeded`, and its texts. Tests of row 5
   and of CRITERION-156.
3. Search and `jag-bench`: pass the key or no key. Tests of CRITERION-155.
4. Check: the status words, the column rule, and the items of the answer of
   row 9. Tests of CRITERION-157 and of rows 11 and 12.
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

## Risks

- A user who forgets the gateway key for a gateway that needs one now sees
  the error after 1 call, not before (0028 Consequences). Signal: exit 4 with
  the `KeyNeeded` text. The call sends no key.
- With no gateway key, the snippets of the search now go to the base URL.
  Before this change, jag stopped before the call. The user chose the base
  URL, so this is the intended behavior. Signal: a report of a call to a base
  URL with a typing error.
- A gateway that answers a call with no key with a status other than `401` or
  `403` (for example `404`) gives the failure of that status, not
  `KeyNeeded`. Signal: a report of exit 6 from a gateway that needs a key.

## Out of scope

- The other parts of CAPABILITY-13: DECISION-65 and DECISION-76 (a call or a
  snippet that is too long for the server), and DECISION-74 (the report names
  the host). They are in SCOPE-04 and have no roadmap item.
- CAPABILITY-14 (private patterns).
- A change of the texts of `KeyNotAccepted` (a call with a key).
