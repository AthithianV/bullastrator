---
name: bullastrator-pr-review
description: Review Bullastrator GitHub pull requests against their base branch and, when explicitly requested, publish concrete inline review comments on changed lines.
metadata:
  short-description: Review Bullastrator PRs and comment inline
---

# Bullastrator PR review

Use this skill for a pull-request review of this repository. The review must be evidence-based and focused on defects, regressions, security problems, migration risks, and missing validation. Do not add comments for personal style preferences or changes that are merely optional.

## Repository context

- Read the repository `AGENTS.md` before reviewing.
- This is a Svelte 5/SvelteKit/Tauri application with Rust workspace crates under `crates/`.
- Frontend validation uses `vp check`/the repository's configured check command; Rust validation runs from `crates/` with Cargo.
- Preserve existing user changes and do not modify the worktree while reviewing.

## Review workflow

1. Resolve the PR from the current branch with `gh pr view`, or use the PR number supplied by the user. Confirm the repository, base branch, head SHA, and working tree before proceeding.
2. Read the base-to-head diff, including renames and deleted files. Inspect surrounding code and relevant tests or migrations before making a finding.
3. Review in this order:
   - correctness and behavior regressions;
   - data safety, authorization, secrets, and destructive operations;
   - database migration ordering, repeatability, rollback/failure behavior, and existing-data compatibility;
   - Rust async/concurrency/error handling;
   - Svelte reactivity, lifecycle cleanup, keyboard accessibility, and event propagation;
   - API/command compatibility and cache invalidation;
   - tests and validation proportional to the changed risk.
4. Run focused, read-only checks when useful. Do not “fix” the PR while reviewing.
5. Report only findings that are actionable and supported by the code. Every finding must identify the failure mode, impact, and a concise suggested direction.

## Inline comment rules

When the user asks to add comments to GitHub:

- Post only comments tied to changed lines in the PR (`side: RIGHT`). If the defect is exposed by unchanged code, anchor the comment to the nearest relevant changed line and explain the relationship.
- Use one comment per distinct issue. Do not create duplicate comments for the same root cause.
- Start each comment with a priority, for example `[P1]`, `[P2]`, or `[P3]`, where P1 blocks safe merge, P2 is a significant correctness or reliability issue, and P3 is a lower-impact but actionable defect. Do not use priority labels for praise.
- Keep comments short and specific: state what is wrong, why it matters, and what should change. Avoid vague questions and avoid prescribing an implementation when multiple safe fixes exist.
- Do not post findings about formatting, naming, or architecture preference unless they cause a concrete defect.
- Do not approve or request changes unless the user explicitly asks for that review event. The default event is a normal `COMMENT` review.
- If there are no actionable findings, do not create an empty review; report that the PR has no findings instead.

## GitHub posting

Use the authenticated `gh` CLI and GitHub REST API. Before posting, retrieve existing review comments and avoid repeating an already-reported issue. Construct a single review payload containing all new inline comments:

```json
{
  "body": "Automated review findings",
  "event": "COMMENT",
  "comments": [
    {
      "path": "src/example.ts",
      "line": 42,
      "side": "RIGHT",
      "body": "[P1] Explain the concrete defect and its impact."
    }
  ]
}
```

Post it with `gh api --method POST repos/OWNER/REPO/pulls/NUMBER/reviews --input payload.json`. Resolve `OWNER/REPO` and `NUMBER` from `gh pr view`; never guess them. If authentication, permissions, or line anchoring prevents posting, do not silently claim success—return the findings and the exact blocker.

After posting, verify the returned review/comment IDs and report the number of inline comments created, with links when available.

## Final report

Summarize findings first, ordered by priority, with file and line references. Then state checks run, whether comments were posted, and any posting or validation blockers. Do not include a long description of the review process.
