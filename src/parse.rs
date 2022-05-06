use std::process::Command;

pub fn get_commit_times(repo_path: String, author: Option<String>) -> Result<Vec<String>, String> {
    let mut command = Command::new("git");

    command
        .arg("-C")
        .arg(repo_path)
        .arg("log")
        .arg("--pretty=format:%ci");

    if !author.is_none() {
        command.arg(format!("--author={}", author.unwrap()));
    }

    let output = command.output().expect("Failed to run `git log`");

    if output.status.success() {
        let commit_times = String::from_utf8_lossy(&output.stdout);
        let commit_times_vec: Vec<_> = commit_times
            .lines()
            .map(|timestamp| timestamp.split_whitespace().nth(1).unwrap().to_string())
            .collect();
        return Ok(commit_times_vec);
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(error_msg.to_string());
    }
}
