use std::env;
use std::fs::{ File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write, Read};
pub struct ToDo {
    name: String,
}

impl ToDo {
    pub fn new(mut args: env::Args) -> ToDo {
        args.next(); // Skip the first argument (the program name)

        match args.next().as_deref() {

            Some(filename) => {
                let mut f = File::open(filename).expect("Failed to open file");
		match
                let mut contents = String::new();
                f.read_to_string(&mut contents).expect("Failed to read file");
                contents.print_list();

                ToDo { name: filename.to_string() }
            }
             _ => {
                println!("Create a new to_do_list file");
                let mut name = String::new();
                println!("what should be the name");
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_string();
                File::create(&name).expect("Failed to create file");
                ToDo { name }
            }

        }
    }
    

    pub fn add(&self, input: String) {
        let mut file = OpenOptions::new()
            .append(true)
            .write(true)
            .open(&self.name)
            .expect("Failed to open file");
        writeln!(file, "{} ▫", input).expect("Failed to write to file");
    }

    pub fn rm(&self, string: String) {
        if !string.chars().all(|c| c.is_digit(10))
            {
                self.remove(string);
            }
        else {
            let index = string.parse().expect("number");
            let file = File::open(&self.name).expect("Failed to open file");
            let reader = BufReader::new(file);
            let mut remove = String::from("qwertyuiopasdfghjklzxcvbnmmnbvcxzlkjhgfdsapoiuytrewq");
            let mut count=1;
            for line in reader.lines() {
            if count==index {remove=line.expect("gyatt");
            break; }
            count += 1;
                   }
            if remove=="qwertyuiopasdfghjklzxcvbnmmnbvcxzlkjhgfdsapoiuytrewq" { println!("Couldnt not find") }
            else{
                        self.remove(remove);
            }
            }
        }
        fn remove(&self,string: String) {
            let file = File::open(&self.name).expect("Failed to open file");
                        let reader = BufReader::new(file);
                        let mut lines: Vec<String> = Vec::new();
            
                        for line in reader.lines() {
                            let line = line.expect("Failed to read line");
                            let (first, second) = find(line.clone());
                             if first.trim() != string.trim() || second.is_empty() {
                                lines.push(line);
                            }
                        }
                        self.write(lines);
        }
    pub fn write(&self,lines: Vec<String>) {

                        let mut file = OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .open(&self.name)
                            .expect("Failed to open file");
                        for line in lines {
                            writeln!(file, "{}", line).expect("Failed to write to file");
                        } 
        }
        pub fn done(&self, string: String )-> io::Result<()> {
        let file = File::open(&self.name)?;
        let reader = BufReader::new(file);
        let mut lines: Vec<String> = Vec::new();

        for line in reader.lines() {
            let mut line = line.expect("Failed to read line");
            let (first, second) = find(line.clone());
            if first.trim() == string.trim() && second == "▫" {
                line = format!("{} ▪", first); // Change open square to closed square
            }
            lines.push(line);
        }
        self.write(lines);
        Ok(())
    }
}

pub trait PrintList {
    fn print_list(&self);
}

impl PrintList for String {
    fn print_list(&self) {
        let mut index = 1;
        for line in self.lines() {
            println!("{} {}", index, line);
            index += 1;
        }
    }
}

impl PrintList for ToDo {
    fn print_list(&self) {
        let mut f = File::open(&self.name).expect("Failed to open file");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Failed to read file");
        contents.print_list();
    }
}
pub fn find(input: String) -> (String, String) {
    if let Some(pos) = input.rfind(|c| c == '▫' || c == '▪') {
        let (first, second_with_char) = input.split_at(pos);
        let second = second_with_char.to_string();
        let first = first.to_string();
        (first, second)
    } else {
        (input, String::new())
    }
}


