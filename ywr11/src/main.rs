use std::process::Command;

fn main() {
    // Create a Command struct to represent the process you want to run
    let mut cmd = Command::new("Explorer.exe");

    // Optionally, you can pass arguments to the program:
    // cmd.arg("/select,C:\\path\\to\\file\\or\\folder");

    // Execute the command and handle errors
    match cmd.status() {
        Ok(status) => {
            if status.success() {
                println!("explorer.exe was launched successfully!");
            } else {
                eprintln!("explorer.exe failed to run with status: {:?}", status.code());
            }
        }
        Err(err) => {
            eprintln!("Failed to run explorer.exe: {:?}", err);
        }
    }
}

