# claude-lint: Global Context

## Purpose

A CLI tool that validates .claude/ directory structure against the
layering model: context shapes reasoning, the model decides when to act.

## Repo Structure

- src/main.rs — single-file CLI, no dependencies beyond std.
- README.md — skeeto-style, minimal documentation.

## Quality Bar

- All comments end with a period.
- No panics in normal operation; errors go to stderr.
- Exit 0 on success, exit 1 on validation failure.

## Epistemics

- This tool enforces opinions, not universal truths.
- The layering model is one valid approach, not the only one.
