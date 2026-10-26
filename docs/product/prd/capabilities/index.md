# capabilities

* [CAPABILITY-01: Guided key setup and health check](CAPABILITY-01.md) - The user stores a TypeSafe key one time, and one command confirms that jag is ready or says what to fix.
* [CAPABILITY-02: Term and question search](CAPABILITY-02.md) - The user gives a folder, a term, and a plain-language question; jag returns the matching lines ranked by how well they answer the question.
* [CAPABILITY-03: File and line for each result](CAPABILITY-03.md) - Each result shows the file, the line number, and the snippet, so the person or the agent can open it.
* [CAPABILITY-04: Limit on the text that jag returns](CAPABILITY-04.md) - The user sets a limit on how much text jag returns to the agent; jag never goes over it and says what it left out.
* [CAPABILITY-05: Text output for people and structured output for agents](CAPABILITY-05.md) - jag gives readable text to a person and structured output that an agent can parse.
* [CAPABILITY-06: Feels like grep](CAPABILITY-06.md) - jag works in a pipe, uses grep-like options and exit codes, and is fast enough to replace grep.
* [CAPABILITY-07: Show the cost of a search](CAPABILITY-07.md) - jag reports how many results it scored and what the search cost.
* [CAPABILITY-08: A repeated search costs nothing](CAPABILITY-08.md) - When the user asks the same question of the same text again, jag does not pay for it again.
* [CAPABILITY-09: Say which text leaves the machine](CAPABILITY-09.md) - jag tells the user which text it sends to the model service, so the user can decide before a search.
* [CAPABILITY-10: Question without a term](CAPABILITY-10.md) - The user gives only a question, and jag chooses the candidate lines itself.
* [CAPABILITY-11: Agent instructions to install or copy](CAPABILITY-11.md) - The agent builder installs a jag skill into their agent, or copies a snippet; either one teaches the agent how jag works and how to use it as an enhanced rg.
* [CAPABILITY-12: Measure jag on your own questions](CAPABILITY-12.md) - The user runs jag and grep on their own questions and sees which one finds the answer more often.
* [CAPABILITY-13: Score with a Clef server that the user runs](CAPABILITY-13.md) - The user sets the base URL to a Clef or Clef-Flash server, and jag scores each search there, with no TypeSafe key, so that no text goes to TypeSafe.
* [CAPABILITY-14: Hold private text back from the model](CAPABILITY-14.md) - Before each model call, jag checks each snippet for keys, tokens, passwords, and the patterns of the user, and it does not send a snippet that matches.
