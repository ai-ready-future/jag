# The agent test

This test shows how well an agent uses jag. It gives Claude Code, with the jag
skill, nine fixed tasks. For each task it records what the agent did:

- Did the agent run jag, or did it use rg or grep?
- Did the agent set a limit on the text (`--max-chars`)?
- Did the agent use the file and the line of a result?
- When jag could not run, did the agent tell the user the cause and the fix?

The test only records. It never fails because of a choice of the agent.

## Run it

One command runs all nine tasks:

```
cargo test --locked --test agent_use -- --ignored --nocapture
```

The test prints one table when it ends. It also writes the full record in
`target/agent-use/<unix seconds>/`: the file `record.json`, one transcript for
each task, and the calls that the agent made to jag.

## What the test needs

- Claude Code (`claude`) on your PATH, with your normal login.
- Node.js with `npx`. The test installs the skill with `skills@1.7.4`.
- A network connection.

The test does not need an API key. If `ANTHROPIC_API_KEY` is set, the test
removes it for the agent.

## What the test uses

The test uses the login and the subscription of the person who runs it. Each
task is one new agent process of at most 30 turns and at most 10 minutes. The
test never reads your login files. Do not run it in CI.

The setup of your Claude Code loads as usual: your instructions, plugins,
skills, hooks, and MCP servers. The record lists what loaded, so you can see
which setup the agent had. A personal skill `jag` in `~/.claude/skills/jag`
that is different from `skills/jag/SKILL.md` stops the test. Update it or
remove it, then run the command again.

## What is fake

The agent never gets a real key. Each task has its own fake model service on
your machine. A wrapper named `jag` records each call that the agent makes, and
it runs the jag of this checkout. Task `not-installed` has no jag on the PATH.

## The tasks

The file `tasks.yaml` holds the tasks. The text of each task is given to the
agent as it is. Change the file only when you want a new test, and compare
records that used the same file (the record holds a copy of it).
