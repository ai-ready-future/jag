---
type: Decision
title: Release 1 keeps the options that people use most
description: jag keeps a fixed set of options that people use most; any other option
  is a usage error that names it.
status: stable
decided_at: '2026-10-07'
alternatives:
- The same set, and jag ignores an unknown option with a warning
- All GNU grep options, with a usage error for options that have no meaning in a ranked
  search
about:
- /capabilities/CAPABILITY-06.md
- /decisions/DECISION-05.md
---

# DECISION-34: Release 1 keeps the options that people use most

Release 1 keeps these options: `-i`, `-w`, `-F`, `-e`, `-A`, `-B`, `-C`, `-l`, `-c`, `-q`, `--include`, and `--exclude`. Any other option is a usage error. jag names the option and exits with code 2.

Source of the change from grep to rg: the user, hitl answer of 2026-10-15 (FEATURE-23, Q1 option 1: rg is the reference for jag with no question). rg 15.1.0 has no `--include` and no `--exclude` option. It uses `-g` or `--glob`. The FEATURE-23 spec decides what jag does with these 2 options.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-06: Feels like grep](../capabilities/CAPABILITY-06.md)
- about: [DECISION-05: jag keeps grep exit codes and adds one code for each model failure](DECISION-05.md)
- subject of: [DECISION-50: The -A, -B, and -C options set the context lines of a result](DECISION-50.md)
- traced by: [FEATURE-13: Common grep options and exit codes](../../roadmap/features/FEATURE-13.md)
- traced by: [STORY-35: Use the common grep options](../../roadmap/stories/STORY-35.md)
- traced by: [STORY-37: Get a usage error for an unknown option](../../roadmap/stories/STORY-37.md)
<!-- LINKS:END -->
