#[path = "src/cli.rs"]
mod cli;

use clap::CommandFactory;
use clap_complete::Shell;
use cli::Cli;
use std::ffi::OsStr;
use std::fs::File;
use std::{env, path::PathBuf};

fn main() {
    let target_dir = match env::var_os("OUT_DIR") {
        // if no OUT_DIR, no need to write man/completions
        None => return,
        Some(outdir) => PathBuf::from(outdir),
    };

    let mut completion_dirs = vec![target_dir.clone()];

    if Ok("release".to_string()) == env::var("PROFILE") {
        if let Some(manifest_dir) = env::var_os("CARGO_MANIFEST_DIR") {
            completion_dirs.push(PathBuf::from(manifest_dir).join("target").join("release"));
        }
    };

    for target_dir in completion_dirs {
        generate_completions(target_dir.as_os_str());
    }

    let anchor = PathBuf::from(&target_dir).join("buildkite_waiter-stamp");

    File::create(anchor).unwrap();
}

fn generate_completions(target_dir: &OsStr) {
    let bin_name = env!("CARGO_PKG_NAME");
    let mut cmd = Cli::command();

    for shell in &[
        Shell::Bash,
        Shell::Elvish,
        Shell::Fish,
        Shell::PowerShell,
        Shell::Zsh,
    ] {
        if let Err(err) = clap_complete::generate_to(*shell, &mut cmd, bin_name, target_dir) {
            panic!("failed to generate completions for {:?}: {:?}", shell, err);
        }
    }

    if let Err(err) =
        clap_complete::generate_to(clap_complete_fig::Fig, &mut cmd, bin_name, target_dir)
    {
        panic!("failed to generate completions for fig: {:?}", err);
    }
}
