---
name: jag
description: "jag is a code and text search tool to use in place of rg. It takes a term and a path as rg does, and with --ask it ranks the matching lines by how well they answer a plain-language question. Use it for each search of code or text files."
---

# Search with jag

This file tells you what jag is, how to run it, how to read its output, and
what to do when it cannot run.

## What jag is

jag is an enhanced rg. It has the same command form as rg: a term and a
path. Use jag in place of rg for each search of code or text files.

jag adds the option `--ask`. With `--ask` and a question in plain language,
jag ranks the matching lines by how well they answer the question. Without
`--ask`, jag finds the lines as rg does.

## How to run jag

Run the command in a shell. There are two forms.

Without a question, jag finds the lines as rg does. It makes no call to the
model and it costs nothing:

```
jag --json --max-chars 8000 retry .
```

With a question, jag ranks the matching lines by how well they answer it. It
makes a call to the model, and the call costs money:

```
jag --json --max-chars 8000 --ask "Where does the code retry a failed request?" retry .
```

It is up to you to use `--ask` or not.

- `--json` is present in each command. jag then writes one JSON document on
  standard output.
- The term is one broad word or a short regular expression, for example
  `retry`.
- The question is one clear sentence in plain language. A question that
  starts with `-` needs the form `--ask="QUESTION"`.
- The path is present in each command, for example `.`. Without a path, jag
  can wait for text on standard input that does not come.
- `--max-chars N` is present in each command. It sets the size limit of the
  output. The limit counts characters, not tokens. Choose N from the text
  that you can read now. The smallest value is 1,000. The default is 20,000.
- The options `-i`, `-w`, `-F`, and `-g GLOB` work as they do in rg.
- Do not pipe the output into `head` or a similar command. The limit already
  keeps the output short, and a cut document is not valid JSON.
- Do not run `jag --check` before a search. It makes a paid call. Run the
  search and read the exit code.

## How to read the output

Read `exit_code` first. Then read `results` in the order that jag gives.
The first result is the best.

- Each result has `path`, `first_line`, and `last_line`. Open `path` from
  `first_line` to `last_line` before you change code.
- Each line of a result has `number`, `text`, and `is_match`. `is_match` is
  true for the lines that hold the term. Cite a line as `path:number`.
- `no_good_answer` is true when no result answers the question well.
  `results_found` is the count of results that matched the term.
- `left_out` is the count of results that the size limit left out. When it is
  more than 0, `size_limit_fix` tells how to get the rest. Raise the number of
  `--max-chars`, or use a narrower term or path.
- `warnings` lists paths that jag could not search.
- When `error` is not null, the search failed. `error.cause` tells why and
  `error.fix` tells what to do. `results` is then empty. Do not use it as a
  result.

## What each exit code means

| Code | Meaning | What to do |
|---|---|---|
| 0 | Results found | Read `results` |
| 1 | No match, or no good answer | If `no_good_answer` is true, the term matched, but no result answers the question: say so. If `results_found` is 0, the term matched nothing: try a broader term, or another path |
| 2 | Usage error, or a path that does not exist | Read `error.fix`. If `error` is null, read `warnings`. Fix the command and run it again |
| 3 | The key is missing or cannot be used | jag cannot run: see the next section |
| 4 | The service does not accept the key | jag cannot run: see the next section |
| 5 | The account has no credit | jag cannot run: see the next section |
| 6 | The service is down | jag cannot run: see the next section |
| 7 | The cost limit was reached | The results are valid. Use them. For more, use a narrower term or path |
| 8 | The size limit left out results | The results are valid. Use them. For more, follow `size_limit_fix` |
| 126 | jag did not start | jag cannot run: see the next section |
| 127 | jag was not found | jag cannot run: see the next section |
| 130 | The user stopped the search | Stop. Do not run jag again until the user asks |

## When jag cannot run

jag cannot run when the exit code is 3, 4, 5, 6, 126, or 127, or when the
shell says `command not found`. Do this one time in the task:

- For 3, 4, 5, and 6, give `error.cause` and `error.fix` to the user as jag
  writes them.
- For 126, 127, and `command not found`, tell the user that jag is not
  installed. Do not install it yourself. Give the user the install steps at
  https://github.com/ai-ready-future/jag#install. One way is
  `npm install -g @airf/jag`.
- For 3 and 4, tell the user to run `jag --setup` in their own terminal, or
  to set the variable `JAG_TYPESAFE_API_KEY`.

Do not run jag again in this task, unless the user says that the fix is
done. For 6, do not run jag again at once: jag already tried again in the
same run.

The key is secret. Never ask the user for the key in the chat. Never put a
key in a command. Never read or print the files in `~/.config/jag/`.