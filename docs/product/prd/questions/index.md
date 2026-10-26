# questions

* [QUESTION-01: Does jag serve users whose text must not leave the machine?](QUESTION-01.md) - jag sends text to an outside model service; some operators and knowledge seekers cannot send their text out.
* [QUESTION-02: What target fits setup failures?](QUESTION-02.md) - jag has no telemetry, so setup failures come only from public issues; the target is not set.
* [QUESTION-03: Which exit code wins when both limits apply?](QUESTION-03.md) - A search can reach the cost limit and also leave out results; each has its own exit code.
* [QUESTION-04: What does the readiness check report for a server with no key?](QUESTION-04.md) - The readiness check examines five items, and most are about the key; a Clef server can have no key.
* [QUESTION-05: What happens when one snippet alone is too long for the server?](QUESTION-05.md) - jag sends smaller calls when a call is too long, but one snippet can be too long by itself.
* [QUESTION-06: Which exit code applies when the private-pattern check skips lines?](QUESTION-06.md) - A search can skip lines for private patterns, leave out results, and reach the cost limit; each has or can have its own exit code.
* [QUESTION-07: Does turning the private-pattern check off also turn off the built-in list?](QUESTION-07.md) - The user can turn the check off; it is not decided if that turns off the built-in list, the list of the user, or both.
