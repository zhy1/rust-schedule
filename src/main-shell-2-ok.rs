use std::process::Command; // Command::new(command)
use std::io::stdin; // stdin().read_line
use std::io::Write; // stdout().flush()
use std::io::stdout; // stdout()

fn main(){
    loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!("> ");
        stdout().flush().expect("some error message");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        let mut child = Command::new(command)
            .spawn()
            .unwrap();

        // don't accept another command until this one completes
        child.wait().expect("some error message");
    }
}