use std::process::Command;

use chrono::{DateTime, FixedOffset};

fn main() {
    let output = Command::new("git")
        .args(["branch", "-a", "-v"])
        .output()
        .unwrap();
    let str_output = String::from_utf8(output.stdout).unwrap();
    let mut hashes: Vec<CommitLog> = str_output
        .lines()
        .map(|l| &l[2..])
        .filter(|l| !l.starts_with("remotes/"))
        .map(get_last_commit_info)
        .collect();
    hashes.sort_by(|a, b| b.date.timestamp().cmp(&a.date.timestamp()));
    println!(
        "{}",
        hashes
            .iter()
            .map(format_commit_info)
            .collect::<Vec<_>>()
            .join("\n\n")
    )
}

// fn branches(line: &str) -> Vec<&str> {
//     line.split(" ").filter(|w| !w.is_empty()).collect()
// }
struct CommitLog {
    branch_name: String,
    hash_and_commit: String,
    author: String,
    date: DateTime<FixedOffset>,
}
fn get_last_commit_info(line: &str) -> CommitLog {
    let branch: Vec<_> = line.split(" ").filter(|w| !w.is_empty()).collect();
    let commit_hash = branch[1];
    let commit_log = String::from_utf8(
        Command::new("git")
            .args(["log", "-n", "1", commit_hash])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    // let vec = commit_log.lines().collect::<Vec<_>>();
    // if vec.len() == 0 {
    //     println!("{line}");
    // }
    // println!("{commit_log}");
    // let commit_log = &commit_log
    //     .lines()
    //     .filter(|l| !l.is_empty())
    //     .collect::<Vec<_>>()[1..];
    let commit_log: Vec<_> = commit_log
        .lines()
        .filter(|l| l.starts_with("Author") || l.starts_with("Date"))
        .collect();
    // println!("{}\n{:#?}", line, commit_log);
    CommitLog {
        branch_name: String::from(branch[0]),
        hash_and_commit: branch[1..].join(" "),
        author: String::from(commit_log[0]),
        date: DateTime::parse_from_str(commit_log[1], "Date:   %a %b %e %T %Y %z")
            .expect("Date parsing failed"),
    }
}

fn format_commit_info(commit_log: &CommitLog) -> String {
    format!(
        "{:<80}{}\n{:<80}{}\n{:<80}Date: {}",
        commit_log.branch_name,
        commit_log.hash_and_commit,
        "",
        commit_log.author,
        "",
        commit_log.date,
    )
}
