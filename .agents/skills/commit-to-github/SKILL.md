---
name: commit-to-github
description: Automatically stage working tree changes and create a clean git commit. Triggers on /commit-to-github, /commit, /Commit, or when user asks to commit to git/github.
---

# Commit to GitHub

Automates staging changes and committing them cleanly to the local Git repository before pushing.

## Workflow

1. **Inspect Status**:
   Run `git status` to see unstaged changes and untracked files.
   Verify that sensitive files (.env, credentials, secrets, binary archives) are not being staged.

2. **Stage Changes**:
   Stage tracked modifications and new relevant source/asset files (`git add -A` or specific paths).

3. **Generate Commit Message**:
   - If user provided a message (e.g. `/commit-to-github <message>`), use their message.
   - Otherwise, craft a concise Conventional Commit message based on the staged diff (e.g., `feat: ...`, `fix: ...`, `refactor: ...`).

4. **Commit**:
   Run `git commit -m "<message>"`.
   If `.git` is read-only in sandbox, execute with `BypassSandbox: true`.

5. **Report Result**:
   Print the commit hash, branch, and summary of changes committed.
