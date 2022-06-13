use std::process::{Command, ExitStatus};

pub fn run_command(str_command: String) -> ExitStatus {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_command() {
        let exit_status = run_command(String::from("ls -a"));
        assert!(exit_status.success());
    }
}
