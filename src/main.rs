use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(["branch", "-a", "-v"])
        .output()
        .expect("bla");
    println!("{:?}", output.stdout);
}
