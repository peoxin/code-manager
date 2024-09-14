use std::fmt::{self, Display};
use std::path::PathBuf;

pub enum Lang {
    Rust,
    Python,
    C,
    Cpp,
    Java,
    JavaScript,
    TypeScript,
    Go,
    Shell,
}

impl Lang {
    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            Lang::Rust => &["rs"],
            Lang::Python => &["py"],
            Lang::C => &["c"],
            Lang::Cpp => &["cpp", "cc", "cxx"],
            Lang::Java => &["java"],
            Lang::JavaScript => &["js"],
            Lang::TypeScript => &["ts"],
            Lang::Go => &["go"],
            Lang::Shell => &["sh"],
        }
    }

    pub fn all_extensions() -> &'static [&'static str] {
        &[
            "rs", "py", "c", "cpp", "cc", "cxx", "java", "js", "ts", "go", "sh",
        ]
    }

    pub fn from_extension(ext: &str) -> Option<Lang> {
        match ext {
            "rs" => Some(Lang::Rust),
            "py" => Some(Lang::Python),
            "c" => Some(Lang::C),
            "cpp" | "cc" | "cxx" => Some(Lang::Cpp),
            "java" => Some(Lang::Java),
            "js" => Some(Lang::JavaScript),
            "ts" => Some(Lang::TypeScript),
            "go" => Some(Lang::Go),
            "sh" => Some(Lang::Shell),
            _ => None,
        }
    }
}

impl Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lang::Rust => write!(f, "Rust"),
            Lang::Python => write!(f, "Python"),
            Lang::C => write!(f, "C"),
            Lang::Cpp => write!(f, "C++"),
            Lang::Java => write!(f, "Java"),
            Lang::JavaScript => write!(f, "JavaScript"),
            Lang::TypeScript => write!(f, "TypeScript"),
            Lang::Go => write!(f, "Go"),
            Lang::Shell => write!(f, "Shell"),
        }
    }
}

pub struct CodeDir {
    pub dir: PathBuf,
    pub lang: Option<Lang>,
    pub with_git: bool,
}

impl Display for CodeDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lang_str = match &self.lang {
            Some(lang) => lang.to_string(),
            None => "".to_string(),
        };
        let git_str = if self.with_git { "Git" } else { "" };

        write!(f, "{}: {} {}", self.dir.display(), lang_str, git_str)
    }
}
