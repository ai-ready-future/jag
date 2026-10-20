# jag

Grep with a model that ranks the matches by how well they answer your
question. The package on crates.io is `jag-grep`. The command is `jag`.

## Install

jag runs on Linux (x86_64 and arm64) and on macOS (arm64 and x86_64). Each
way gives the same version.

### 1. Install the jag program

With npm (Node.js 18 or later):

```
$ npm install -g @airf/jag
```

### 2. Add the jag skill to your agent

A skill tells a coding agent how to use jag. To add it to your agent, after
step 1:

```
$ npx skills add ai-ready-future/jag
```

The tool asks which agents get the skill and where it goes. To install for
Claude Code with no question, add `--agent claude-code -y`. The tool writes
`skills-lock.json` in the project. You write no instructions and edit no
file.

jag sends no usage data. The `skills` tool, not jag, sends anonymous usage
data and a security-audit request to skills.sh. `DO_NOT_TRACK=1` stops both:
`DO_NOT_TRACK=1 npx skills add ai-ready-future/jag`.

### Other ways to install

With cargo (Rust 1.99 or later). This builds jag from source in some
minutes:

```
$ cargo install --locked jag-grep
```

From GitHub Releases: download the archive for your system and its
`.sha256` file from the latest release. Then check the archive, open it, and
move `jag` to a folder in your `PATH`:

```
$ sha256sum -c jag-<version>-<target>.tar.gz.sha256        # Linux
$ shasum -a 256 -c jag-<version>-<target>.tar.gz.sha256    # macOS
$ tar -xzf jag-<version>-<target>.tar.gz
$ mv jag-<version>-<target>/jag ~/.local/bin/
```

The targets are `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`,
`aarch64-apple-darwin`, and `x86_64-apple-darwin`.

On macOS, a browser marks each file that it downloads, and macOS then stops
`jag`. Download with `curl -LO`, or remove the mark:
`xattr -d com.apple.quarantine jag`.

Or run jag with no install:

```
$ npx @airf/jag --help
```

The jag skill runs the command `jag`, so an agent needs one of the installs
above.

On Linux, a question to the model needs the CA certificates of the system
(the package `ca-certificates`).

## Store your TypeSafe key

Get a key at <https://console.typesafe.ai/keys>. Then run:

```
$ jag --setup
```

`jag --setup` asks for the key. The terminal shows no echo. It stores the key
in `~/.config/jag/config.yml`. It needs a terminal. With no terminal, set
`JAG_TYPESAFE_API_KEY` instead.

To remove the stored key:

```
$ jag --forget-key
```

## Where jag looks for the key

jag uses the first key that it finds, in this order:

1. the variable `JAG_TYPESAFE_API_KEY`
2. the variable `TYPESAFE_API_KEY`
3. the file `~/.config/jag/config.yml`

The file has this form. The key goes under `config:`, not at the top level:

```yaml
config:
  api-key: <your key>
```

jag refuses a config file that other users can read. Run `chmod 600` on it.
jag never shows the key.

This order is for the TypeSafe key. To use an AI gateway in place of the
TypeSafe API, see "Use an AI gateway".

## Use an AI gateway

jag can send the model calls to an AI gateway. Set the address of the gateway
and the model name that the gateway uses. For each of the two settings, jag
uses the first source that is set:

| Setting | Option | Variable | Config file |
|---|---|---|---|
| The address | `--base-url URL` | `JAG_BASE_URL` | `config.base-url` |
| The model name | `--model NAME` | `JAG_MODEL` | `config.model` |

The default address is `https://api.typesafe.ai/v1`. The default model is
`jev-1.13.0`. A blank value counts as not set. The address must be an `https`
address, with no user name and no `?` or `#`. `http` is allowed only for
`127.0.0.1`, `localhost`, and `[::1]`. jag adds `/v1` when the path of the
address does not end with `/v1`.

These gateways take the TypeSafe model:

| Gateway | `JAG_BASE_URL` | `JAG_MODEL` |
|---|---|---|
| OpenRouter | `https://openrouter.ai/api` | `~typesafe/jev-latest` |
| Vercel AI Gateway | `https://ai-gateway.vercel.sh/typesafe` | `typesafe-ai/jev` |
| Pydantic AI Gateway | `https://gateway-us.pydantic.dev/proxy/typesafe` | `jev-latest` |

The TypeSafe key goes only to the TypeSafe API. For any other address, jag
sends a gateway key, and it never sends the TypeSafe key. jag uses the first
gateway key that it finds:

1. the variable `JAG_GATEWAY_API_KEY`
2. `config.gateway-api-key` in `~/.config/jag/config.yml`

```yaml
config:
  api-key: <your TypeSafe key>
  gateway-api-key: <your gateway key>
  base-url: https://openrouter.ai/api
  model: "~typesafe/jev-latest"
```

Put a model name that starts with `~` in quotes, in the shell and in the file.
An example search through a gateway:

```
$ jag --base-url https://openrouter.ai/api --model "~typesafe/jev-latest" --ask "how long is the wait" retry src
```

When `JAG_BASE_URL` or `config.base-url` names a gateway, `jag --setup` and
`jag --forget-key` work on the gateway key. They keep the other values of the
file. With no gateway key, a search stops with exit 3 before the call.
`--check` follows the same settings and the same key as a search, and it
accepts `--base-url` and `--model`.

To store or remove the key of the other route, name that route for the one
command. For example, to store the TypeSafe key while a gateway is set:

```
$ JAG_BASE_URL=https://api.typesafe.ai jag --setup
```

## Check that jag is ready

```
$ jag --check [--base-url URL] [--model NAME]
```

The check shows `pass` or `fail` for five items, in this order:

1. a key is present
2. the service accepts the key
3. the account has credit
4. the service is reachable
5. other users cannot read the key file

An item that jag cannot examine, because an earlier item failed, shows
`not checked`. Each item that fails has its cause and its fix on the error
stream. The check makes one small call to the model service. The call costs
less than $0.00001. `--timeout`, `--retry-wait`, `--base-url`, and `--model` apply to the call. On a
gateway, item 1 names the gateway key, and the texts say "gateway".

When all five items pass, the check shows this and exits with 0:

```
pass        a key is present (JAG_TYPESAFE_API_KEY)
pass        the service accepts the key
pass        the account has credit
pass        the service is reachable
pass        other users cannot read the key file (no key file)
jag is ready.
```

Exit codes:

| Code | Cause |
|---|---|
| 0 | All five items pass |
| 2 | The command line or a setting is not valid |
| 3 | No key, a bad key, or a config file that others can read or that jag cannot use |
| 4 | The service does not accept the key |
| 5 | The account has no credit |
| 6 | The service is not reachable |

When more than one item fails, the code is the code of the first item that
fails.

## Examples

The output below is what jag shows in a terminal, with no color. It comes from a test of jag. The test searches a small sample folder, and a test service takes the place of the model. The scores, the order of the results, and the cost come from that test service. With the real model, the order of your results and your cost can be different. In a pipe, the layout and the number of results can be different.

An operator reads the log of a deploy that failed. The log goes into jag through a pipe, and the question asks which step failed:

<!-- example: operator-pipe -->
```
$ cat logs/deploy.log | jag --ask "which step made the deploy fail" fail
<stdin>-6-10:00:10 step 3 of 6: update the database
<stdin>-7-10:00:14 step 3 of 6 done
<stdin>-8-10:00:15 step 4 of 6: start the new service
<stdin>:9:10:00:19 health check failed once, will try again
<stdin>-10-10:00:24 step 4 of 6 done
<stdin>-11-10:00:25 step 5 of 6: warm the cache
<stdin>-12-10:00:31 step 5 of 6 done
<stdin>-13-10:00:32 step 6 of 6: send traffic to the new service
<stdin>:14:10:00:36 step 6 of 6 failed: the new service has no free disk space
<stdin>-15-10:00:37 roll back started
<stdin>-16-10:00:40 traffic is back on the old service
<stdin>-17-10:00:41 roll back done
--
<stdin>:18:10:00:42 deploy of build 412 failed
jag: cost $0.000063: scored 2 of 2 results, skipped 0.
```

A new employee reads the notes of the company and asks about leave:

<!-- example: notes-question -->
```
$ jag --ask "how many days of leave do new staff get" days notes
notes/leave.md-1-# Leave
notes/leave.md-2-
notes/leave.md:3:New staff get 25 days of paid leave each year.
notes/leave.md-4-Leave starts to count on the first day of work.
notes/leave.md:5:Ask your manager before you book more than 10 days in a row.
--
notes/onboarding.md-1-# First weeks
notes/onboarding.md-2-
notes/onboarding.md:3:In the first 30 days, a new person meets the team and reads the guides.
notes/onboarding.md-4-They get a laptop on the first day.
notes/onboarding.md-5-Their manager checks in each week.
--
notes/expenses.md-1-# Expenses
notes/expenses.md-2-
notes/expenses.md-3-Send a receipt with each claim.
notes/expenses.md:4:The company pays a claim within 14 days.
notes/expenses.md-5-A claim without a receipt waits until the receipt comes.
jag: cost $0.000095: scored 3 of 3 results, skipped 0.
```

A developer looks for the place where the code waits before it tries a request again:

<!-- example: code-question -->
```
$ jag --ask "where does the code wait before it tries a request again" retry src
src/client.py-12-        # Wait before the next try.
src/client.py:13:        time.sleep(retry_wait)
src/client.py-14-    raise RuntimeError("the service did not answer")
--
src/queue.py-3-        try:
src/queue.py-4-            job.run()
src/queue.py-5-        except TimeoutError:
src/queue.py:6:            job.retry_count += 1
src/queue.py-7-            jobs.append(job)
--
src/client.py-1-import time
src/client.py-2-
src/client.py:3:max_retry = 3
src/client.py:4:retry_wait = 2
src/client.py-5-
src/client.py-6-
src/client.py-7-def send(request):
src/client.py:8:    for attempt in range(max_retry):
src/client.py-9-        reply = call_service(request)
src/client.py-10-        if reply.ok:
src/client.py-11-            return reply
--
src/settings.py:1:# How many times to retry a job.
src/settings.py:2:job_retry_limit = 5
jag: cost $0.000126: scored 4 of 4 results, skipped 0.
```

A developer who does not need the model gives no question. jag then gives the same output as `rg`, with no cost line:

<!-- example: no-question -->
```
$ jag retry src
src/client.py
3:max_retry = 3
4:retry_wait = 2
8:    for attempt in range(max_retry):
13:        time.sleep(retry_wait)

src/queue.py
6:            job.retry_count += 1

src/settings.py
1:# How many times to retry a job.
2:job_retry_limit = 5
```

An employee asks a question that the notes do not answer. jag exits with exit code 1 and shows the label before the results. Ask a better question, or search for a broader term:

<!-- example: no-good-answer -->
```
$ jag --ask "what is the refund policy" policy notes
jag: no good answer. The results below do not answer the question well.
notes/security.md-1-# Security
notes/security.md-2-
notes/security.md:3:Read the security policy before you use a company laptop.
notes/security.md-4-Lock the screen when you leave your desk.
notes/security.md-5-Report a lost laptop on the same day.
--
notes/travel.md-1-# Travel
notes/travel.md-2-
notes/travel.md:3:The travel policy says to book trains before planes.
notes/travel.md-4-Book at least one week before the trip.
notes/travel.md-5-Keep the tickets for the expense claim.
jag: cost $0.000063: scored 2 of 2 results, skipped 0.
```

A technician looks for the reason that a backup stopped, and sets a small size limit. jag leaves out some results, exits with exit code 8, and tells how to raise the limit. Use a larger `--max-chars`:

<!-- example: size-limit -->
```
$ jag --max-chars 1000 --ask "why did the backup stop" error logs
jag: size limit reached: showed 1 of 6 results, left out 5. To see more, use --max-chars with a number larger than 1000, or --no-max-chars.
logs/backup.log-29-02:04:10 copied 10 percent of volume 4
logs/backup.log-30-02:04:20 copied 40 percent of volume 4
logs/backup.log-31-02:04:30 copied 70 percent of volume 4
logs/backup.log:32:02:04:40 error 4: the backup disk answered too slowly
logs/backup.log:33:02:04:41 error 4: the copy of volume 4 was cut off
logs/backup.log-34-02:04:50 copy of volume 4 started again
logs/backup.log-35-02:04:55 copied 100 percent of volume 4
logs/backup.log-36-02:04:59 check of volume 4 passed
jag: cost $0.000189: scored 6 of 6 results, skipped 0.
```

A developer asks a question before jag has a key. jag exits with exit code 3 and tells what to do. Run `jag --setup` (see "Store your TypeSafe key"):

<!-- example: missing-key -->
```
$ jag --ask "where does the code wait before it tries a request again" retry src
jag: no TypeSafe API key found.
Get a key at https://console.typesafe.ai/keys, then run `jag --setup`,
or set JAG_TYPESAFE_API_KEY.
```

## Search with no question

With no question, jag gives the same output as `rg` (ripgrep) 15.2.0 gives
for the same arguments:

```
$ jag TERM [PATH...]
```

The lines, the colors, the layout, the error texts, and the exit codes are
those of `rg`. An error text starts with `jag:` in the place of `rg:`. jag
makes no call to the model and reads no key and no setting of the model
service. The files come in the order of the paths on the command line. Inside
one path, the files come in the order of their names, as `rg --sort path`
does.

To rank the matches by a question, add `--ask`:

```
$ jag --ask "how long is the wait" retry src
```

A question that starts with `-` needs `=`:

```
$ jag --ask="-v or --verbose?" verbose src
```

- In a terminal, `rg` shows the path above its lines, with the line numbers.
  In a pipe, `rg` shows `path:text` for more than one file, and only the text
  for one file. Add `-n` for line numbers, `-H` for the path, or `--heading`.
- jag does not cut a line, and it writes no cost line and no notice of
  skipped files. jag skips binary files, hidden files, and ignored files, as
  `rg` does.
- A few things are not the same as in `rg`: `--max-chars` and
  `--no-max-chars` (the size limit, below), `--json`, `--ask`, `--help`,
  `--version`, and the options of the model. The texts for a term that uses
  look-around or a new line, for an option that jag does not know, and for the
  value `modified` of `--sort`, do not have the hints of `rg`. The option
  `--colour` does not exist.
- With no question, `--timeout`, `--retry-wait`, `--max-scored`,
  `--base-url`, and `--model` have no effect on the search. jag still checks
  their values before the search: a bad value is a usage error with exit code
  2, and jag needs no key. jag reads none of the variables of these settings.

## Where jag searches

- jag searches each file and each folder that you give. It searches a folder
  as `rg` does. You can give any mix of files and folders.
- With no path, jag searches the text of a pipe, and the path of each result
  is `<stdin>`. An empty pipe gives no match and exit 1, as in `rg`.
- With no path and no pipe, for example in a terminal or with `/dev/null` as
  standard input, jag searches the current folder, and the paths have no `./`
  at the start.
- The path `-` means standard input, in each mode, also with other paths.
  jag searches each path in the order of the command line. In the JSON
  document, the `path` of a result of standard input is `null`. An empty path
  is a path that does not exist.
- With `--ask`, jag reads all piped text before it ranks the results, so it
  waits until the pipe closes.
- An agent gives a path, for example `.`. Some tools keep standard input open,
  and then jag waits for text that does not come. Other tools give an empty
  pipe, and then jag finds no match.
- A path that does not exist gives an error text that names the path. jag
  searches the other paths, and it exits with 2 at the end, as `rg` does. With
  `--ask`, the text is `jag: the path P does not exist`, and the results of
  the other paths cost model calls.

```
$ printf 'one\nneedle here\n' | jag needle
needle here
$ printf 'one\nneedle here\n' | jag -n needle
2:needle here
```

## The size limit

jag never writes more characters to standard output than a size limit. The
default limit is 20,000 characters with `--ask` or `--json`. A search with no
question has no limit, as `rg` has none, unless you give `--max-chars`. Then
jag writes whole results up to the limit and the notice below. To change the
limit:

```
$ jag --max-chars 40000 --ask "QUESTION" TERM PATH...
$ jag --no-max-chars TERM PATH...
```

With `--ask` or `--json`, you can also set the variable `JAG_MAX_CHARS`. The
option wins over the variable. With no question, jag does not read the
variable. The smallest limit is 1,000. A value that is not a whole number of
1,000 or more is a usage error with exit code 2. `--no-max-chars` turns the
limit off.

- jag counts characters, not bytes. Each character that jag writes to standard
  output counts: the paths, the line numbers, the `--` lines, the label, and
  the color codes. The error stream does not count.
- To stay inside the limit, jag leaves out whole results, with the lowest
  rank first. It never writes a lower result in place of a higher result that
  does not fit.
- When the top result alone is larger than the limit, jag cuts it. It keeps
  the first matching line, and it ends the last line with ` [+N chars]`,
  where N is the count of characters that jag does not show.
- jag then writes one notice on the error stream. It gives the count of the
  results that jag left out, and how to raise the limit. For example:

```
jag: size limit reached: showed 8 of 30 results, left out 22. To see more, use --max-chars with a number larger than 1000, or --no-max-chars.
```

- When jag leaves out a result or cuts the top result, it exits with 8. When
  the cost limit is also reached, the exit code is 7, and the notice still
  shows. When no result answers well, the exit code is 1.
- With `--json`, jag fits the whole document in the limit, and it never cuts a
  document. The results fit first, then the warnings. The fields
  `left_out`, `top_result_cut`, `size_limit_fix`, and `warnings_left_out` hold
  what the limit did. The error stream holds no size notice.

## Output for an agent

By default, jag writes text, also when the output goes to a pipe. The
results go to standard output. The notices go to the error stream.

For an agent, add `--json`:

```
$ jag --json --ask "QUESTION" TERM PATH...
```

jag then writes one JSON document on one line, with `\n` at the end, on
standard output. The document holds the results, the counts, the notices, and
each error. jag writes nothing on the error stream, unless it cannot write the
document to standard output: then it writes that failure there, as text, and
then the cost line. Only the option turns on
this form: no variable and no setting does. `--json` does not work with
`--setup`, `--forget-key`, or `--check`.

Each document has these fields:

| Field | Meaning |
|---|---|
| `version` | The version of this form. It is `1` |
| `prompt_version` | The version of the scoring prompt, for example `jag-p1` |
| `exit_code` | The exit code of jag |
| `error` | `null`, or an object with `cause` and `fix` |
| `no_good_answer` | `true` when the text mode shows the no-good-answer label |
| `results_found`, `results_scored`, `results_skipped` | The three numbers of the cost-limit notice. With no question, `results_found` is the count of matching lines, and the other two are `0` |
| `left_out` | The count of results that the size limit left out |
| `top_result_cut` | The count of characters that the size limit cut from the top result, or `0` |
| `size_limit_fix` | The text that tells how to raise the limit, or `""` when the limit left out nothing |
| `warnings_left_out` | The count of warnings that the size limit left out |
| `cost` | An object with `micro_usd` (the cost in millionths of a US dollar, or `null` when it is not available), `kind` (`billed`, `estimate`, `not_available`, or `no_call`), and `reason` (`null`, or the reason when `kind` is `no_call`: `no_question`, `no_match`, `no_readable_match`, or `not_started`) |
| `files_skipped` | An object with `total`, `binary`, `hidden`, and `ignored` |
| `warnings` | A list of objects with `path` (text or `null`) and `text` |
| `results` | A list in rank order |

A later version can add a kind or a reason to `cost`, so agent code must
accept a value that it does not know.

Each result has `rank` (1 for the best result), `path` (`null` for standard
input), `first_line`, `last_line`, and `lines`. Each line has `number`,
`text`, `is_match`, and `cut` (the count of characters that jag cut from the
end of the line, or `0`). The document has no score. With no question, each
result has one line, and the results come in the order of the text output.

When no line matches, the document has an empty `results` list and an
`exit_code` of 1. When a path does not exist, `exit_code` is 2, `error` is `null`,
`warnings` names the path, and `results` holds the results of the other
paths. When jag fails, `error` holds the cause and the fix, `results` is empty,
and the other fields hold what jag counted before the failure.

In one major version of jag, jag does not remove or rename a field. It can add
a field, so agent code must accept a field that it does not know. The order of
the fields can change: a new field goes before `results`. `version` changes
only when a field is removed or renamed.

## In a terminal and in a pipe

- In a terminal, jag shows `jag: the model scored K of N results` on the
  error stream while the model scores, and removes it when the results come.
  In a pipe or a file, jag shows no progress. With `--json`, jag never shows
  progress.
- jag colors the matches, the paths, the line numbers, and the separators as
  `rg --color=auto` does: only when the output is a terminal, `TERM` is set
  and is not `dumb`, and `NO_COLOR` is not set. `--color=always` colors also
  in a pipe, for example for `less -R`. `--color=never` never colors.
  `--color` takes a value after a space or after `=`: `never`, `auto`,
  `always`, or `ansi`. When you give the option more than one time, the last
  one wins. With `--json`, the document has no color.
- Ctrl-C stops jag at once. jag starts no new model call and shows no
  results. jag writes `jag: the search was stopped.` and ends by the signal
  SIGINT, so a shell shows the exit code 130 and a script stops. With
  `--json`, the output is an error document with `exit_code` 130. When jag
  already counted the results, it also writes the cost line after the
  message, and the document holds the cost of the calls that ended before
  the stop. With no question, jag ends by the signal SIGINT with no message,
  as `rg` does.
- When the reader of the output closes the pipe, jag stops at once, starts no
  new model call, and ends by the signal SIGPIPE with no message. A shell
  shows the exit code 141.

## Search options

jag takes these options of `rg`, with their meaning in `rg`: `-i`, `-w`, `-F`,
`-e`, `-A`, `-B`, `-C`, `-l`, `-c`, `-q`, `-g`, `-n`, `-N`, `-H`, `-I`,
`--heading`, `--no-heading`, `--sort`, and `--color`. Each one also has its
long name, for example `--ignore-case`. Any other option of `rg` is an
unknown option: jag gives the error text of `rg` for it and exit code 2.
`-h` and `--help` show the help of jag.

- `-e TERM` gives a term. With more than one `-e`, a line matches one of the
  terms. With `-e`, each word after the options is a path.
- A short option that takes a value reads one `=` after its letter as the
  separator, as `rg` does: `-e=foo` gives the term `foo`, and `-A=2` gives 2.
  To search `=foo`, type `-e '=foo'`.
- `-g GLOB` (or `--glob`) searches only the files that match the glob. With
  `!` at the start, jag leaves the files out. The glob rules are those of
  ripgrep, for example `*.{rs,toml}`. The last glob that matches decides.
  A glob does not apply to a path that you name. A glob that leaves out a
  folder leaves out the files in it. A glob that matches a hidden file
  searches it, as `rg` does. This is the same with no question, with `--ask`,
  and with `--json`.
- `-n` and `-N` turn the line numbers on and off. `-H` and `-I` turn the
  paths on and off. `--heading` and `--no-heading` put the path above its
  lines, or on each line. The last of each pair wins. These options work only
  with no `--ask` and no `--json`.
- `--sort path` (the default) and `--sort none` set the order of the files.
  The value `modified` is not in jag.
- With `--ask`, `-A`, `-B`, and `-C` take 0 to 12 lines, and a result has at
  most 25 lines. With no option, a result has 3 lines before and 3 lines after
  the match, and at most 12 lines. With no `--ask`, the options take each
  number, and `--` stands between two groups of lines, as in `rg`.
- `-l`, `-c`, and `-q` work only with no `--ask` and with no `--json`. `-c`
  writes `path:N` for more than one file, and `N` for one file. A file with no
  match has no line. If you give more than one of the three, `-q` wins. Of
  `-l` and `-c`, the last one wins.
- The size limit applies to `-l` and `-c` only when you set it: each line is
  one whole item. When the limit leaves out lines, jag writes the size notice
  and exits with 8. `-q` writes nothing, so the limit has no effect on it.
- jag skips binary files, so `-l` and `-c` do not show them.
- The exit codes with no question are those of `rg`: 0 for a match, 1 for no
  match, and 2 for an error, also when a line matches. With `--ask`, 1 also
  means no good answer.

## The cost of a search

After each search with `--ask` in text mode, jag writes one cost line as the last line on
the error stream. It does this also when the output goes to a pipe:

```
jag: cost $0.002016: scored 33 of 33 results, skipped 0.
```

The line gives the cost in US dollars, the count of results that the model
scored, and the count that the cost limit skipped. jag adds the input tokens
that the model service reports for each answered call, at 0.042 US dollars
for 1 million input tokens, the price of jev-1.13.0. Output tokens are free.
A call that gets no answer, for example after a timeout, adds nothing.
Through an AI gateway, the amount is the price of TypeSafe. It does not
include a fee of the gateway.

- `(estimate)` after the amount: the service did not report the tokens of
  some calls, so jag estimated them from the size of the request.
- `not available` in place of the amount: jag does not know the price of the
  model that `--model`, `JAG_MODEL`, or `config.model` names. The search
  still works.
- `$0.000000 (no match)` or `(no readable match)`: jag made no model call.
- When a search fails or you stop it, the cost line comes after the failure
  text and gives the cost of the calls that the service answered. A search
  that ends before jag counts the results, and `--check`, write no cost line.
  When the reader closes the pipe, jag writes no cost line.
- With no question, jag writes no cost line, as `rg` does not.
- With `--json`, jag writes no cost line. The `cost` field of the document
  gives the same amount (see "Output for an agent").

## Speed

A typical search returns in under 5 seconds. A typical search is one search
with a question, at the default limit of 200 scored results, on a code
repository of about 15,000 files, while the model service works.

- jag finds the matches in one pass. On Go 1.25.0, with 14,490 files, that
  takes about 0.1 s on a machine with 12 CPUs. Then the model scores 200
  results in 7 calls, 6 at the same time, so the search waits for two rounds
  of calls. The calls take most of the time.
- A larger `--max-scored` needs one more round of calls for each 192
  results.
- A long list of files can be too long for the system. macOS allows 1 MiB for
  the arguments of a command and its environment, so a shell glob of a
  repository with about 1.5 times the files of Go can fail with `argument
  list too long`. Then give the folder in the place of the list.
- To measure jag on your machine, run
  `python3 scripts/speed/measure.py --jag PATH` in a clone of this
  repository, with the path of the jag that you use, for example
  `target/release/jag` after `cargo build --release --locked`. The script
  clones Go 1.25.0 (a download of about 36 MB) and runs 5 searches 6 times
  each: 30 searches. Each search costs at most about $0.02, so the measure
  costs at most about $0.60. jag finds your TypeSafe key as it always does: the
  script sets `JAG_BASE_URL` to the TypeSafe address and removes the other
  settings of the model service, so it never uses a gateway. The script
  exits with 0 when each search returns in under 5 seconds, with 1 when it
  does not, and with 2 when jag fails or an argument is wrong.
- A test in the CI (`tests/speed.rs`, a release build) keeps the local part
  of a search under 1 s, and a search with a model service that needs 2 s for
  each call under 5 s.

Measured results, with the TypeSafe model service:

- 2026-10-14: jag 0.1.0 on Linux x86_64, 12 CPUs, Go 1.25.0 (14,490 files): medians 0.29 to 0.37 s, slowest run 0.56 s.
