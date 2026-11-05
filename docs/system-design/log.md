# Log

## 2026-11-05
* **Learn**: data: DF-1 closed by ADR-0029 (proposed, hitl ANSWER 2026-11-05, REQ-config-timeout): new decision row data-3, open row 'not migrated: DF-1' removed., by claude-code/opus-5.5, at 2026-11-05T20:35:34Z
* **Report**: set of the body failed: set changes only the frontmatter. I edited components/COMPONENT-02.md: the body sentence now says 'with no loss of the other keys and the comment lines of the file', the same as the description. Reason: designer review of PR #352, item 5 (ADR-0029)., by claude-code/opus-5.5, at 2026-11-05T20:58:06Z
* **Report**: set of the body failed: set changes only the frontmatter. I edited conventions/CONVENTION-01.md to CONVENTION-08.md: the heading now has the new title, and the body now has the full rule text in place of the first line only that the migration of 2026-11-03 kept. title, description and rule changed through set. Reason: designer review of PR #352 (the rule text of the conventions was cut short)., by claude-code/opus-5.5, at 2026-11-05T21:21:22Z
* **Learn**: contracts: new decision row contracts-12 (ADR-0028, accepted 2026-10-29). ADR-0028 partly supersedes ADR-0017 for the case with no gateway key. Row contracts-10 (ADR-0017) stays: its key sources and the route rule do not change. COMPONENT-02 is governed by ADR-0028., by claude-code/opus-5.5, at 2026-11-05T21:33:26Z
* **Learn**: deployment: new decision row deployment-7 (ADR-0027, accepted 2026-10-26, model status out-of-scope, a row as for ADR-0019, ADR-0021, ADR-0023, ADR-0024 and ADR-0025)., by claude-code/opus-5.5, at 2026-11-05T21:33:28Z

## 2026-11-03
* **Setup**: made the bundle docs/system-design, by system-design/0.2.3, at 2026-11-03T11:37:02Z
* **Migrate**: migrated docs/product/system-design.md (27 items, 6 not migrated); the old file stays, by claude-code/opus-5.5, at 2026-11-03T11:37:50Z
* **Settle**: system accepted -> draft: the design covers release 1 only. CAPABILITY-08, 09, 10, 13 and 14 of later or wont scopes have no component (owner choice, human:404pilo, 2026-11-03)., by claude-code/opus-5.5, at 2026-11-03T11:59:30Z
* **Report**: Recommendation: the GAP rule X4 counts each include of each release scope, also an include with priority wont (CAPABILITY-08 in SCOPE-01) and the scopes of later releases. An accepted design for release 1 cannot pass. Skip wont includes, and let system.md name the scopes it covers (destination)., by claude-code/opus-5.5, at 2026-11-03T11:59:30Z
* **Report**: Recommendation: migrate-file copies design_state accepted from the old file, but the new GAP rule then makes each gap an ERROR. Migrate as draft, or warn that the accepted state turns gaps into errors., by claude-code/opus-5.5, at 2026-11-03T11:59:30Z
* **Report**: Recommendation: resolve does not find a page by its slug field (resolve agent-skill failed for COMPONENT-11, slug agent-skill). new should print the new ID, or resolve should match slug., by claude-code/opus-5.5, at 2026-11-03T11:59:30Z
* **Report**: Recommendation: the X6 rule asks for a decision row for each ADR whose parent_spec names an area, but init does not offer to run migrate-file when docs/product/system-design.md exists. Setup ends with X6 errors until the person runs learn., by claude-code/opus-5.5, at 2026-11-03T11:59:30Z
