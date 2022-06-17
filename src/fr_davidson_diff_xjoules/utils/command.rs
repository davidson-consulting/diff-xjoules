use std::process::{Command, ExitStatus, Stdio};
use std::fs::File;
use std::os::unix::io::{FromRawFd, IntoRawFd};

pub fn run_command(str_command: &str) -> ExitStatus {
    println!("{}", str_command);
    let mut splitted_command = str_command.split(" ");
    let mut command = Command::new(splitted_command.next().unwrap());
    splitted_command.for_each(|arg| {
        command.arg(arg);
    });
    return command
        .spawn()
        .expect("command failed to start")
        .wait()
        .expect("Could not wait for the command!");
}

pub fn run_command_redirect_to_file(str_command: &str, path_file: &str) -> ExitStatus {
    println!("{}", str_command);
    let file = File::create(path_file).expect("couldn't create file");
    let mut splitted_command = str_command.split(" ");
    let mut command = Command::new(splitted_command.next().unwrap());
    splitted_command.for_each(|arg| {
        command.arg(arg);
    });
    return command
        .stdout(unsafe { Stdio::from_raw_fd(file.into_raw_fd()) })
        .spawn()
        .expect("command failed to start")
        .wait()
        .expect("Could not wait for the command!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_command() {
        let exit_status = run_command("ls -a");
        assert!(exit_status.success());
    }

    #[test]
    fn test_run_command_redirect_to_file() {
        let exit_status = run_command_redirect_to_file("ls -a", "target/cmd_results");
        assert!(exit_status.success());
        let content = std::fs::read_to_string("target/cmd_results").unwrap();
        let mut splitted_content = content.split("\n");
        assert!(splitted_content.any(|element| element.eq("Cargo.toml")));
        assert!(splitted_content.any(|element| element.eq(".git")));
        assert!(splitted_content.any(|element| element.eq("src")));
        
    }
}
