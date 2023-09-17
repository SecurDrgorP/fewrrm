use std::io::{self, Write};
use std::fs::{self, OpenOptions};
use std::path::Path;
use std::process::Command;
use log::error;


pub fn file_reader(ncp: String) {
    let path = Path::new(&ncp);

    if Path::exists(path) {
        let result = fs::read_to_string(path);

        match result {
            Ok(data) => {
                let contents = data;
                println!("The Content of your file located in {}: \n\n {}", ncp, contents);
            },
            Err(e) => {
                println!("Error reading file: {}", e);
            },
	}
    } else {
	println!("File does not exist");
    }
}

pub fn file_writter(ncp: String) {

       let input = ">>".to_string().input();

       let path = Path::new(&ncp);

       if !Path::exists(path) {

          // Create a file
          let mut data_file = OpenOptions::new()
             .write(true)
             .create(true)
             .open(path)
             .expect("creation failed");

          // Write contents to the file
          write!(data_file, "{input}").expect("write failed");

          println!("Created a file in {ncp}");
       }else {
          // Open a file with append option
          let mut data_file = OpenOptions::new()
             .append(true)
             .open(path)
             .expect("cannot open file");

          // Write to a file
          data_file
             .write(input.as_bytes())
             .expect("write failed");

          println!("Appended content to a file");
       }

}

pub fn file_editing(ncp: String) {
    // Create a new Command object to run the nano command line.
    let mut nano = Command::new("nano");

    // Add the file to be edited to the command line arguments.
    nano.arg(ncp);

    // Execute the command and check the exit status.
    match nano.status() {
        Ok(status) => {
            if status.success() {
                // Print the output of the command.
                println!("the #File created succcessfully.");
            } else {
                println!("The command failed to execute.");
            }
	}
	Err(err) => {
            // The command failed to execute due to an error.
            error!("Command failed to execute: {}", err);
        }
    }

}

pub fn file_remover(ncp: String) {
    let path = Path::new(&ncp);

    // Remove a file
    fs::remove_file(path).expect("could not remove file");

    println!("Removed file located in {}", ncp);
}

pub trait SE {
 fn is_exists(&self);
 fn el_line(&self) -> String;
 fn input(&self) -> String;
}

impl SE for String {

 fn is_exists(&self) {

    let path = Path::new(&self);

    let mut answer = String::from("");
    if Path::exists(path) {
       answer.push_str("is");
    }else{
       answer.push_str("is not");
    }

    println!("`NOTE: The #File {answer} exist.`");

 }

 fn el_line(&self) -> String {

    let mut nstr = String::new();
    for str in self.chars() {
        if str != '\n' {
           nstr.push(str);
        }
    }
    nstr

 }

 fn input(&self) -> String {
    println!("{}", self);

    let mut cp = String::new();
    io::stdin().read_line(&mut cp)
    .expect("Failed to read line");

    cp
 }

}

