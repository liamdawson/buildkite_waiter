#[path = "src/cli.rs"]
mod cli;

use clap::CommandFactory;
use clap_complete::Shell;
use cli::Cli;
use std::ffi::OsStr;
use std::{env, path::PathBuf};

fn main() {
    if env::var("PROFILE") != Ok("release".to_string()) {
        // only generate completions on release builds
        return;
    }

    let target_dir = match env::var_os("CARGO_MANIFEST_DIR") {
        // if no CARGO_MANIFEST_DIR, don't completions
        None => return,
        Some(outdir) => PathBuf::from(outdir).join("completions"),
    };

    if !target_dir.exists() {
        std::fs::create_dir_all(&target_dir).expect("should find or create completions directory");
    }

    generate_completions(target_dir.as_os_str());
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
            panic!(
                "failed to generate completions for {:?} in {:?}: {:?}",
                shell, target_dir, err
            );
        }
    }

    if let Err(err) =
        clap_complete::generate_to(clap_complete_fig::Fig, &mut cmd, bin_name, target_dir)
    {
        panic!(
            "failed to generate completions for fig in {:?}: {:?}",
            target_dir, err
        );
    }
}
