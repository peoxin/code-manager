use std::fmt::{self, Display};
use std::path::Path;
use std::process::Command;

pub struct GitStatus {
    branch: String,
    uncommited: bool,
}

impl Display for GitStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let uncommited = if self.uncommited { "*" } else { "" };
        write!(f, "{} {}", self.branch, uncommited)
    }
}

pub fn git_current_branch(dir: &Path) -> Option<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .current_dir(dir)
        .output()
        .expect("failed to execute git rev-parse");

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).expect("stdout not utf8");
    Some(stdout.trim().to_string())
}

pub fn git_uncommit_status(dir: &Path) -> Option<bool> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(dir)
        .output()
        .expect("failed to execute git status");

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).expect("stdout not utf8");
    Some(stdout.lines().count() > 0)
}

pub fn get_git_status(dir: &Path) -> Option<GitStatus> {
    if let Some(branch) = git_current_branch(dir) {
        if let Some(uncommited) = git_uncommit_status(dir) {
            return Some(GitStatus { branch, uncommited });
        }
        return None;
    }
    None
}
