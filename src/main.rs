use clap::{Parser, Subcommand};
use code_manager::{CodeDir, Lang};
use git::get_git_status;
use std::path::Path;

mod git;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List all code directories in the given directory
    List {
        dir: String,

        /// The programming language to search for
        #[arg(short, long)]
        lang: Option<String>,

        /// Only show directories that are git repositories
        #[arg(short, long)]
        git: bool,
    },
}

fn match_lang(lang_str: &str) -> Option<Lang> {
    match lang_str {
        "rust" => Some(Lang::Rust),
        "python" => Some(Lang::Python),
        "c" => Some(Lang::C),
        "cpp" => Some(Lang::Cpp),
        "java" => Some(Lang::Java),
        "javascript" => Some(Lang::JavaScript),
        "typescript" => Some(Lang::TypeScript),
        "go" => Some(Lang::Go),
        "shell" => Some(Lang::Shell),
        _ => None,
    }
}

fn find_code_dir(root_dir: &Path, lang_exts: &[&str], git: &bool) -> Vec<CodeDir> {
    fn search_dir(dir: &Path, lang_exts: &[&str], git: &bool, code_dirs: &mut Vec<CodeDir>) {
        for entry in dir.read_dir().expect("read_dir call failed") {
            let entry = entry.expect("entry failed");
            let path = entry.path();
            if path.is_dir() {
                if let Some(code_dir) = is_code_dir(&path, lang_exts, git) {
                    code_dirs.push(code_dir);
                } else {
                    search_dir(&path, lang_exts, git, code_dirs);
                }
            }
        }
    }

    fn is_code_dir(dir: &Path, lang_exts: &[&str], need_git: &bool) -> Option<CodeDir> {
        let mut found_git = false;
        let mut code_dir: Option<CodeDir> = None;
        for entry in dir.read_dir().expect("read_dir call failed") {
            let entry = entry.expect("entry failed");
            let path = entry.path();
            if path.is_dir() && path.ends_with(".git") {
                found_git = true;
            } else {
                match path.extension() {
                    Some(path_ext) => {
                        let path_ext = path_ext.to_str().unwrap();
                        if lang_exts.contains(&path_ext) {
                            code_dir = Some(CodeDir {
                                dir: dir.to_path_buf(),
                                lang: Lang::from_extension(path_ext),
                                with_git: false,
                            });
                        }
                    }
                    None => continue,
                }
            }
        }

        if *need_git && !found_git {
            return None;
        } else if let Some(code_dir) = code_dir {
            return Some(CodeDir {
                with_git: found_git,
                ..code_dir
            });
        } else {
            return None;
        }
    }

    let mut code_dirs: Vec<CodeDir> = Vec::new();
    search_dir(root_dir, lang_exts, git, &mut code_dirs);
    code_dirs
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::List { dir, lang, git } => {
            // Match the language string to the Lang enum.
            let language = match lang {
                Some(lang_str) => match match_lang(&lang_str) {
                    Some(lang) => lang.extensions(),
                    None => {
                        eprintln!("Invalid language: {}", lang_str);
                        return;
                    }
                },
                None => Lang::all_extensions(),
            };

            // Find code directories in the given directory.
            // If the path is not a directory, print an error message.
            // If the path is a directory, print the code directories found in it.
            let dir_path = Path::new(&dir);
            match dir_path.is_dir() {
                true => {
                    let code_dirs = find_code_dir(&dir_path, &language, &git);
                    for code_dir in code_dirs {
                        if code_dir.with_git {
                            if let Some(git_status) = get_git_status(&code_dir.dir) {
                                println!("{} {}", code_dir, git_status);
                            } else {
                                println!("{}", code_dir);
                            }
                        } else {
                            println!("{}", code_dir);
                        }
                    }
                }

                false => {
                    eprintln!("{} is not a directory", dir);
                }
            }
        }
    }
}
