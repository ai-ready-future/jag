---
type: Decision
title: Release 1 keeps the rg options that people use most
description: jag keeps a fixed set of rg options that people use most, in the rg form;
  any other option gives the rg text unrecognized flag and exit code 2.
status: stable
decided_at: '2026-10-15'
alternatives:
- The same set, and jag ignores an unknown option with a warning
- All GNU grep options, with a usage error for options that have no meaning in a ranked
  search
- Keep grep as the reference for the options
- Keep --include and --exclude in the grep form, with no new options
about:
- /capabilities/CAPABILITY-06.md
- /decisions/DECISION-05.md
---

# DECISION-34: Release 1 keeps the rg options that people use most

Release 1 keeps these rg options, in the rg form: `-i`, `-w`, `-F`, `-e`, `-A`, `-B`, `-C`, `-l`, `-c`, `-q`, `-g` and `--glob`, `-n`, `-N`, `-H`, `-I`, `--heading`, `--no-heading`, `--sort` with `path` or `none`, and `--color`. `-g` and `--glob` replace `--include` and `--exclude`. `--color` takes its value as rg does (`--color always` and `--color=always`), and jag has no `--colour`. Any other value of `--sort` (`modified`, `accessed`, or `created`) gives `jag: error parsing flag --sort: choice 'VALUE' is unrecognized`, with exit code 2 (card C). jag also keeps its own options (card A): `--ask`, `--json`, `--max-chars`, `--no-max-chars`, `--max-scored`, `--timeout`, `--retry-wait`, `--base-url`, `--model`, `--setup`, `--check`, `--forget-key`, `-h`, `--help`, `-V`, and `--version`. Any other option gives the rg text `unrecognized flag`, with the name `jag`, and exit code 2.

Source of the change from grep to rg: the user, hitl answer of 2026-10-15 (FEATURE-23, Q1 option 1: rg is the reference for jag with no question). Source of the option set: the user, hitl answer of 2026-10-15 (FEATURE-23, card C option 1: the current set in the rg form, with `-g` and `--glob` in place of `--include` and `--exclude`). Source of the own options of jag: the user, hitl answer of 2026-10-15 (FEATURE-23, card A option 1: jag keeps its name, its help, its version, and its own options).

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-06: Feels like grep](../capabilities/CAPABILITY-06.md)
- about: [DECISION-05: jag keeps rg exit codes and adds one code for each model failure](DECISION-05.md)
- subject of: [DECISION-50: The -A, -B, and -C options set the context lines of a result](DECISION-50.md)
- traced by: [FEATURE-13: Common rg options and exit codes](../../roadmap/features/FEATURE-13.md)
- traced by: [STORY-35: Use the common rg options](../../roadmap/stories/STORY-35.md)
- traced by: [STORY-37: Get a usage error for an unknown option](../../roadmap/stories/STORY-37.md)
<!-- LINKS:END -->
