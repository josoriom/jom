use std::process::Command;

pub fn execute_bash(name: &str,instruction: &str) -> Result<(), String> {
    let mut command = Command::new("bash");
    command.arg("-c").arg(instruction);
    match command.status() {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("Command {} executed successfully!", name);
                Ok(())
            } else {
                Err(format!("Execution failed with error code: {:?}", exit_status.code()))
            }
        },
        Err(err) => Err(format!("Failed to execute command: {}\r\n", err)),
    }
}