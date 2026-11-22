---
name: push-to-github
description: Automatically push local commits and tags to GitHub, creating the repository if it doesn't exist yet. Triggers on /push-to-github, /puhs-to-github, /push, or when user asks to push to github.
---

# Push to GitHub

Pushes commits and tags to GitHub smoothly using GitHub CLI (`gh`) and Git.

## Workflow

1. **Verify Git State**:
   Run `git status` to ensure working tree is clean. If uncommitted changes exist, run the commit workflow first or ask user.

2. **Check Remote**:
   Run `git remote -v`.
   - If remote `origin` exists: verify branch with `git branch --show-current`.
   - If remote `origin` does NOT exist:
     Create repository on GitHub using:
     `gh repo create $(basename "$PWD") --public --source=. --push`

3. **Push Commits and Tags**:
   Run with `BypassSandbox: true`:
   `git push -u origin $(git branch --show-current) --tags`

4. **Verify and Link**:
   Verify with `gh repo view --json url,name,owner` and provide the clickable GitHub URL to the user.
