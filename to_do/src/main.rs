use std::env;
use std::io;

use to_do::{ToDo,PrintList};


fn main() {
    let to_do = ToDo::new(env::args());

    loop {
        println!("\n Pick the option you want, use lowercase letters:");
        println!("1. add \n2. rm \n3. help \n4. look\n5. done \n else leave");

        let mut bruh = String::new();
        io::stdin().read_line(&mut bruh).expect("failed to read line");
        let input = bruh.trim();

        match input {
            "add" | "1" => {
                println!("What do you want to add?");
                let mut yap = String::new();
                io::stdin().read_line(&mut yap).expect("failed to read line");
                let item = yap.trim().to_string();
                to_do.add(item);
            }
            "rm" | "2" => {
                to_do.print_list();
                println!("What do you want to remove?");
                let mut yap = String::new();
                io::stdin().read_line(&mut yap).expect("failed to read line");
                let item = yap.trim().to_string();
                to_do.rm(item);
            }
            "look" | "4" => {
                to_do.print_list();
            }
            "help" | "3" => {
                println!("Available commands:");
                println!("add - Add a new to-do item");
                println!("rm - Remove an existing to-do item");
                println!("look - View all to-do items");
                println!("help - Show this help message");
                println!("leave - Exit the application");
            }
            "done" | "5" => {
                to_do.print_list();
                println!("What do you want done? ");
                let mut yap = String::new();
                io::stdin().read_line(&mut yap).expect("failed to read line");
                let item = yap.trim().to_string();
                let _ = to_do.done(item);
            }
            _ => break,
        }
    }
}
