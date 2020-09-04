extern crate structopt;

use std::ffi::OsStr;
use std::fs::File;
use std::{env, path::PathBuf};
use structopt::clap::Shell;

include!("src/cli.rs");

fn main() {
    let target_dir = match env::var_os("OUT_DIR") {
        // if no OUT_DIR, no need to write man/completions
        None => return,
        Some(outdir) => outdir,
    };

    generate_completions(&target_dir);

    let anchor = PathBuf::from(&target_dir).join("buildkite_waiter-stamp");

    File::create(&anchor).unwrap();
}

fn generate_completions(target_dir: &OsStr) {
    for shell in &[Shell::Bash, Shell::Fish, Shell::Zsh, Shell::PowerShell] {
        Command::clap().gen_completions(env!("CARGO_PKG_NAME"), *shell, target_dir);
    }
}
