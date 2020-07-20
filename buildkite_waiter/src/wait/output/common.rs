use buildkite_rust::Build;
use console::style;
use std::io::Write;

pub fn print_build_info(build: &Build) {
    let mut line = format!("{} {}", style("Waiting for").dim(), style(format!("{}/{}", &build.pipeline.slug, &build.number)).green());
    if let Some(creator) = &build.creator {
        if let Some(name) = &creator.name {
            line = format!("{} {} {}", line, style("by").dim(), style(name).cyan());
        }
    }
    line = format!("{} {}", line, format!("{} {}", style("on branch").dim(), style(&build.branch).cyan()));

    // use writeln! and .ok(), because it's fine if the output couldn't be written
    writeln!(std::io::stderr(), "{}", line).ok();

    if let Some(message) = &build.message {
        if let Some(first_line) = message.lines().next() {
            writeln!(std::io::stderr(), "  {}", first_line).ok();
        }
    }
}
