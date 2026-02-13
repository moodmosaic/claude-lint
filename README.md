# claude-lint

Validate `.claude/` directory structure.

    $ claude-lint .claude
    ok: .claude passes all checks

    $ claude-lint /path/to/.claude
    error: /path/to/.claude/CLAUDE.md: contains workflow verb 'step 1'
    error: /path/to/.claude/skills/foo/SKILL.md: contains fenced code block
    2 error(s)

## What it checks

| Layer | Allowed | Rejected |
|-------|---------|----------|
| `CLAUDE.md` | Norms, facts | Workflow verbs, code blocks |
| `agents/*.md` | Perspective, values (≤120 lines) | Procedures, code blocks |
| `skills/*/SKILL.md` | Capabilities (≤500 lines) | Success criteria, code blocks |
| `references/*.md` | Playbooks | Missing "optional" declaration |

## Install

    cargo install --path .

## Run without Rust

If you have Docker but no Rust toolchain:

    ./run.sh

Run from the project directory you want to lint (it checks `.claude/` by
default), or pass a relative path:

    ./run.sh path/to/.claude

The first run builds the image (~1 min). Subsequent runs take under a
second. Containers are removed automatically; only the cached image
remains. To remove it:

    docker rmi claude-lint

## Why

Context should shape reasoning, not script behavior. If your `.claude/`
has workflows embedded, the model follows scripts instead of thinking.
