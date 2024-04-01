use std::process::Command;

fn default_city() -> Result<String, std::io::Error> {
    let output = Command::new("sh")
                         .arg("-c")
                         .arg("timedatectl | awk '/Time zone/ {split($3, a, \"/\"); print a[2]}'")
                         .output()?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(result)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Command execution failed"))
    }
}
