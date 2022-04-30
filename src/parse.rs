use std::process::Command;

pub fn get_commit_times() -> Vec<String> {
    let output = Command::new("git")
        .arg("log")
        .arg("--pretty=format:%ci")
        .output()
        .expect("Failed to run `git log`");

    let commit_times = String::from_utf8_lossy(&output.stdout);
    let commit_times_vec: Vec<_> = commit_times
        .lines()
        .map(|timestamp| timestamp.split_whitespace().nth(1).unwrap().to_string())
        .collect();
    return commit_times_vec;
}
