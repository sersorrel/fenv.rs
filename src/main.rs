use std::io::prelude::*;

use aho_corasick::AhoCorasick;

// design goals:
// 1. be as fast as physically possible
// 2. at least be faster than plugin-foreign-env

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let divider = "---DIVIDER---"; // TODO: make this unpredictable

    // TODO: does any of this work with multiline environment variables? how portable is `env -0`?
    let previous_env = std::process::Command::new("bash")
        .arg("-c")
        .arg("env")
        .output()
        .unwrap();

    let output_and_new_env = std::process::Command::new("bash")
        .arg("-c")
        .arg(format!(
            "{} && echo && echo '{}' && env",
            args.join(" "), // TODO: uhhh this seems Suspicious from an escaping perspective
            divider
        ))
        .output()
        .unwrap();

    // TODO: is there an even faster search we can do, since we only have one pattern?
    let ac = AhoCorasick::new_auto_configured(&[divider]);

    match ac.earliest_find(&output_and_new_env.stdout) {
        None => {
            std::io::stdout()
                .write_all(&output_and_new_env.stdout)
                .unwrap();
            std::process::exit(output_and_new_env.status.code().unwrap_or(1))
        }
        Some(divider_position) => {
            if !output_and_new_env.status.success() {
                std::io::stdout()
                    .write_all(&output_and_new_env.stdout[..divider_position.start()])
                    .unwrap();
                std::process::exit(output_and_new_env.status.code().unwrap_or(1))
            }
            // TODO: can we avoid using (utf-8) strings here?
            diff_and_apply(
                std::str::from_utf8(&previous_env.stdout).unwrap(),
                std::str::from_utf8(&output_and_new_env.stdout[divider_position.end()..]).unwrap(),
            );
        }
    }
}

// TODO: there is significant room for optimisation here!
fn diff_and_apply(prev: &str, new: &str) {
    for assignment in new.split('\n') {
        if !prev.contains(assignment) {
            let (name, value) = assignment.split_once('=').unwrap();
            print!("{}\0{}\0", name, value);
        }
    }
}
