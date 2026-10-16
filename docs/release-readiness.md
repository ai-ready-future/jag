# Release readiness

This file is the record of the checks that must pass before the first
publish of jag (FEATURE-22). The spec is
`docs/superpowers/specs/2026-10-15-STORY-62-check-that-jag-is-ready-before-the-first-publish-design.md`.

Each section gives the commit, the date, the result, and the evidence (run
URLs and issue numbers). This file holds no key, no address, and no steps of
an attack.

## Use period

Criterion: CRITERION-105 (STORY-62).

Not done.

## Dependency audit

Criterion: CRITERION-106 (STORY-63).

Done for the first part: the audit tools run on `main`. The criterion is
checked again on the commit of the first tag (see "Before the first tag").

- Commit: `16b5cc7` (`main`, PR #146).
- Date: 2026-10-16 (UTC).
- Result: the run of `deps.yml` is green. It runs `scripts/deps/check.sh`:
  `cargo-deny` (advisories, licenses, bans, and sources), `npm audit`, and
  `release.py check-npm-deps`. No open finding of high or critical severity.
- Evidence: https://github.com/ai-ready-future/jag/actions/runs/38837824825
- `ignore` entries in `deny.toml`: none, so no issue with the label
  `security` is necessary.
- The fix of the 4 advisories of `rustls` 0.23.45 (RUSTSEC-2026-0346 to
  RUSTSEC-2026-0349): `Cargo.lock` has `rustls` 0.23.46 (PR #146).

## Security review

Criterion: CRITERION-107 (STORY-63).

Not done.

## Licenses

Criterion: CRITERION-108 (STORY-64).

Not done.

## Notices and metadata

Criterion: CRITERION-109 (STORY-64).

Not done.

## Before the first tag

The list of the spec, section "Before the first tag". Chores does it on the
commit of the tag.

Not done.
