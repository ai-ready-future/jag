# Conventions

* [CONVENTION-01: Each failure gives its cause, its fix, and its exit code](CONVENTION-01.md) - Each failure gives its cause, its fix, and its exit code of IF-4, in the error stream or in the JSON document.
* [CONVENTION-02: A model failure fails the full search](CONVENTION-02.md) - A model failure fails the full search, and jag writes no partial result.
* [CONVENTION-03: In text mode, results go to standard output and notices go to the error stream](CONVENTION-03.md) - In text mode, results go to standard output and notices go to the error stream. In structured mode, all output is in the document.
* [CONVENTION-04: On Ctrl-C, jag starts no new model call and exits with 130](CONVENTION-04.md) - On Ctrl-C, jag starts no new model call and exits with 130. When the reader closes the pipe, jag exits with no message.
* [CONVENTION-05: Each Jev call has a timeout and 1 retry, and 1 deadline covers the search](CONVENTION-05.md) - Each Jev call times out after 4 seconds and gets 1 retry, and 1 deadline covers all calls of a search.
* [CONVENTION-06: Only C-config reads the key, and the key never shows](CONVENTION-06.md) - Only C-config reads the key, the key never shows, the config file must be private, and jag sends each key only on its own route.
* [CONVENTION-07: A setting takes its value from the option, the variable, the file, or the default](CONVENTION-07.md) - A setting takes its value from the option, then the variable, then the config file, then the default.
* [CONVENTION-08: jag sends no telemetry and writes no log file](CONVENTION-08.md) - jag sends no telemetry and writes no log file.
