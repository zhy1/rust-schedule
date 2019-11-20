use std::process::Command;
use std::io::stdin;


fn main(){
    println!("step1");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    // read_line leaves a trailing newline, which trim removes
    let command = input.trim();

    Command::new(command)
        .spawn()
        .unwrap();
    println!("step2");
}