---
type: ClosedItems
title: Closed features
description: The features that are done or cancelled.
status: stable
generated:
  by: prd.py regen
---

# Closed features

## MILESTONE-01: Make it work

* [FEATURE-01: Store a key one time](FEATURE-01.md) - The user stores a TypeSafe key one time; jag finds it on each search, never shows it, and tells a new user where to get one.
* [FEATURE-04: Rank matching lines by a question](FEATURE-04.md) - The user runs `jag --ask "QUESTION" TERM [PATH...]`, and jag ranks the lines that match the term by how well they answer the question, and labels weak results.
* [FEATURE-05: Search a folder as rg does](FEATURE-05.md) - jag searches each subfolder of a folder, skips binary, hidden, and ignored files, and with --ask or --json says how many files it skipped.
* [FEATURE-06: Cost limit on scoring](FEATURE-06.md) - When a term matches too many lines, jag scores only the best part up to a cost limit that the user can change, and says how many matches it skipped.
* [FEATURE-08: File, line, and snippet for each result](FEATURE-08.md) - Each result shows the file, the line range, and the exact source text, with each matching line marked, so the person or the agent can open the source at that line.
* [FEATURE-09: Size limit on the text that jag returns](FEATURE-09.md) - jag never writes more characters than the size limit, leaves out whole results with the lowest rank first, and says how many it left out and how to raise the limit.
* [FEATURE-11: Text output for a person and structured output for an agent](FEATURE-11.md) - An option turns on structured output that an agent can parse, with the same results as the text, a named field for each item, structured errors, and a form version.
* [FEATURE-17: Block private data in the repository](FEATURE-17.md) - A check on each pull request fails when the change adds a key, case content, private evaluation data, or an internal project name.
* [FEATURE-27: README examples for people](FEATURE-27.md) - The README gives worked examples that show a person how to search with jag at the shell, with the command to copy and what jag shows.

## MILESTONE-02: Make it good

* [FEATURE-02: Readiness check](FEATURE-02.md) - One command examines five items, shows pass or fail for each one, and names the fix for each failure, with one call that costs almost nothing.
* [FEATURE-03: Clear failure when the model is not available](FEATURE-03.md) - When the key is missing or out of credit, or the model service is down, jag stops with a message that names the cause and the fix, and exits with the code of that cause.
* [FEATURE-12: Plain rg with no question](FEATURE-12.md) - With a term and no question, jag returns the matching lines as rg does, makes no model call, and costs nothing.
* [FEATURE-13: Common rg options and exit codes](FEATURE-13.md) - jag keeps the rg options that people use most and the rg exit codes 0, 1, and 2; any other option gives the rg text unrecognized flag and exit 2.
* [FEATURE-14: Files, folders, and piped text](FEATURE-14.md) - With no path, jag searches the current folder or reads the pipe; the user can also give any mix of files and folders.
* [FEATURE-15: Good behavior in a terminal and in a pipe](FEATURE-15.md) - jag shows progress and color only in a terminal, and it stops at once with no partial results when the user stops it or the pipe closes.
* [FEATURE-19: Go public with a clean history](FEATURE-19.md) - A scan lists the private data in the history. A new public repository gets a clean history, and a second scan finds none. The private repository stays private.
* [FEATURE-20: Show the cost of a search](FEATURE-20.md) - After each search, jag reports how many results it scored and skipped, and the billed cost in US dollars, as a text line or a structured field.
* [FEATURE-21: Send the model calls through an AI gateway](FEATURE-21.md) - The user can set a base URL and a model name, so that jag sends the same model calls through an AI gateway. With no setting, jag calls TypeSafe directly.
* [FEATURE-22: Check that jag is ready before the first publish](FEATURE-22.md) - Before the first publish, the user uses jag on real work and fixes the bugs, and audits find no open high security finding and no license or compliance problem.
* [FEATURE-23: Give the same output as rg for the same arguments](FEATURE-23.md) - An rg command and a jag command with the same arguments and no question give the same bytes, the same color, and the same exit code, and e2e tests check this in CI.
* [FEATURE-24: Install the jag skill with one README command](FEATURE-24.md) - The agent builder runs one command from the README, and the jag skill teaches the agent the four behaviors and what to tell the user when jag fails.
* [FEATURE-25: Copy the same teaching as a snippet](FEATURE-25.md) - For an agent that can't install a skill, the agent builder copies a snippet that holds the same teaching as the jag skill.

## MILESTONE-03: Make it fast

* [FEATURE-16: Fast enough to replace grep](FEATURE-16.md) - A typical search returns in under 5 seconds, so that a shell user does not go back to grep.
