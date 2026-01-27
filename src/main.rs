//! claude-lint: validate .claude/ directory structure
//!
//! Enforces a layering model where global context is non-procedural,
//! agents express perspective without workflows, skills describe
//! capabilities without success criteria, and references are elective.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let path = match args.get(1) {
        Some(p) => PathBuf::from(p),
        None => PathBuf::from(".claude"),
    };

    if !path.is_dir() {
        eprintln!("error: {} is not a directory", path.display());
        return ExitCode::from(1);
    }

    let mut errors = Vec::new();
    
    check_claude_md(&path, &mut errors);
    check_agents(&path, &mut errors);
    check_skills(&path, &mut errors);

    if errors.is_empty() {
        println!("ok: {} passes all checks", path.display());
        ExitCode::SUCCESS
    } else {
        for e in &errors {
            eprintln!("error: {}", e);
        }
        eprintln!("\n{} error(s)", errors.len());
        ExitCode::from(1)
    }
}

fn read_file(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}

fn count_lines(s: &str) -> usize {
    s.lines().count()
}

// CLAUDE.md checks.

fn check_claude_md(root: &Path, errors: &mut Vec<String>) {
    let claude_md = root.join("CLAUDE.md");
    if !claude_md.is_file() {
        errors.push(format!("missing {}", claude_md.display()));
        return;
    }

    let content = match read_file(&claude_md) {
        Some(c) => c,
        None => {
            errors.push(format!("cannot read {}", claude_md.display()));
            return;
        }
    };

    // No workflow verbs.
    let workflow_verbs = [
        "must then", "next,", "step 1", "step 2", "first,",
        "second,", "finally,", "afterward", "subsequently",
    ];
    let lower = content.to_lowercase();
    for verb in workflow_verbs {
        if lower.contains(verb) {
            errors.push(format!(
                "{}: contains workflow verb '{}'",
                claude_md.display(), verb
            ));
        }
    }

    // No fenced code blocks.
    if content.contains("```") {
        errors.push(format!(
            "{}: contains fenced code block",
            claude_md.display()
        ));
    }
}

// Agent checks.

fn check_agents(root: &Path, errors: &mut Vec<String>) {
    let agents_dir = root.join("agents");
    if !agents_dir.is_dir() {
        return; // agents/ is optional
    }

    let entries = match fs::read_dir(&agents_dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let content = match read_file(&path) {
            Some(c) => c,
            None => continue,
        };

        // Must have frontmatter.
        if !content.starts_with("---\n") {
            errors.push(format!("{}: missing YAML frontmatter", path.display()));
        }

        // Max 120 lines.
        let lines = count_lines(&content);
        if lines > 120 {
            errors.push(format!(
                "{}: too long ({} lines, max 120)",
                path.display(), lines
            ));
        }

        // No fenced code blocks.
        if content.contains("```") {
            errors.push(format!(
                "{}: contains fenced code block",
                path.display()
            ));
        }

        // No step-by-step patterns.
        let procedural = ["## procedure", "## workflow", "## steps"];
        let lower = content.to_lowercase();
        for p in procedural {
            if lower.contains(p) {
                errors.push(format!(
                    "{}: contains procedural section '{}'",
                    path.display(), p
                ));
            }
        }
    }
}

// Skill checks.

fn check_skills(root: &Path, errors: &mut Vec<String>) {
    let skills_dir = root.join("skills");
    if !skills_dir.is_dir() {
        return; // skills/ is optional
    }

    let entries = match fs::read_dir(&skills_dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let skill_dir = entry.path();
        if !skill_dir.is_dir() {
            continue;
        }

        let skill_md = skill_dir.join("SKILL.md");
        if !skill_md.is_file() {
            errors.push(format!("{}: missing SKILL.md", skill_dir.display()));
            continue;
        }

        let content = match read_file(&skill_md) {
            Some(c) => c,
            None => continue,
        };

        // Must have frontmatter.
        if !content.starts_with("---\n") {
            errors.push(format!("{}: missing YAML frontmatter", skill_md.display()));
        }

        // Must have Capability section.
        if !content.contains("\n## Capability\n") {
            errors.push(format!(
                "{}: missing '## Capability' section",
                skill_md.display()
            ));
        }

        // Max 500 lines.
        let lines = count_lines(&content);
        if lines > 500 {
            errors.push(format!(
                "{}: too long ({} lines, max 500)",
                skill_md.display(), lines
            ));
        }

        // No fenced code blocks.
        if content.contains("```") {
            errors.push(format!(
                "{}: contains fenced code block",
                skill_md.display()
            ));
        }

        // No success criteria terms.
        let success_terms = [
            "success criteria", "must ensure", "must verify",
            "requirement:", "requirements:", "you must",
        ];
        let lower = content.to_lowercase();
        for term in success_terms {
            if lower.contains(term) {
                errors.push(format!(
                    "{}: contains success criteria term '{}'",
                    skill_md.display(), term
                ));
            }
        }

        // Check references if they exist.
        let refs_dir = skill_dir.join("references");
        if refs_dir.is_dir() {
            // SKILL.md should have References section.
            if !content.contains("\n## References\n") {
                errors.push(format!(
                    "{}: has references/ but no '## References' section",
                    skill_md.display()
                ));
            }

            check_references(&refs_dir, errors);
        }
    }
}

fn check_references(refs_dir: &Path, errors: &mut Vec<String>) {
    let entries = match fs::read_dir(refs_dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let content = match read_file(&path) {
            Some(c) => c,
            None => continue,
        };

        // Must say "optional" near the top.
        let head: String = content.lines().take(15).collect::<Vec<_>>().join("\n");
        if !head.to_lowercase().contains("optional") {
            errors.push(format!(
                "{}: should state 'optional' near the top",
                path.display()
            ));
        }
    }
}
