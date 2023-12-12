use std::{fmt::Display, process::Command};

#[derive(PartialEq, Clone)]
pub struct Branch {
    pub name: String,
    pub title: String,
    pub date: String,
    pub author: String,
    pub email: String,
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
            "%(refname:short)---%(subject)---%(authordate:format:%c)---%(authorname)---%(authoremail:trim)",
            "--sort",
            "-authordate",
        ])
        .output()
        .expect("failed to run git branch command");
    let branches = String::from_utf8(command.stdout).expect("failed to parse git branch command");
    branches.lines().map(parse_branch).collect()
}

fn parse_branch(line: &str) -> Branch {
    let sections: Vec<_> = line.split("---").collect();
    Branch {
        name: sections[0].to_string(),
        title: sections[1].to_string(),
        date: sections[2].to_string(),
        author: sections[3].to_string(),
        email: sections[4].to_string(),
    }
}

pub fn delete_branch(branch: Branch) {
    Command::new("git")
        .args(["branch", "-D", &branch.name])
        .output()
        .expect(&format!(
            "failed to execute 'git branch -D {}'",
            branch.name
        ));
}
