use std::{fmt::Display, process::Command};

#[derive(PartialEq, Clone)]
pub struct Branch {
    pub name: String,
    pub title: String,
    pub date: String,
    pub author: String,
    pub email: String,
    pub remote: String,
}

impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let branch = format!(
            "{}{}{}\n{:>170}\n{:>170}\n{:>170}\n",
            self.name,
            " ".repeat(170 - self.name.len() - self.title.len()),
            self.title,
            self.date,
            self.author,
            self.email,
        );
        write!(f, "{}", branch)
    }
}

pub fn get_current_branches() -> Vec<Branch> {
    let command = Command::new("git")
        .args([
            "branch",
            "--format",
            "%(refname:short)---%(subject)---%(authordate:format:%c)---%(authorname)---%(authoremail:trim)---%(upstream:lstrip=-2)",
            "--sort",
            "-authordate",
        ])
        .output()
        .expect("failed to run git branch command");
    let branches = String::from_utf8(command.stdout).expect("failed to parse git branch command");
    branches.lines().map(parse_branch).collect()
}

fn parse_branch(line: &str) -> Branch {
    let sections = line.split("---").collect::<Vec<&str>>();
    Branch {
        name: sections[0].to_string(),
        title: sections[1].to_string(),
        date: sections[2].to_string(),
        author: sections[3].to_string(),
        email: sections[4].to_string(),
        remote: sections[5].to_string(),
    }
}

pub fn delete_branch(branch: &Branch, author_email: &str) {
    Command::new("git")
        .args(["branch", "-D", &branch.name])
        .output()
        .expect(&format!(
            "failed to execute 'git branch -D {}'",
            branch.name
        ));
    if !branch.remote.is_empty() && branch.email == author_email {
        let remote: Vec<&str> = branch.remote.split("/").collect();
        let remote_name = remote[0];
        let remote_branch = remote[1];
        Command::new("git")
            .args(["push", remote_name, "-d", remote_branch])
            .output()
            .expect(&format!(
                "failed to execute 'git push {} -d {}'",
                remote_name, remote_branch
            ));
    }
}
