// use std::cmp::Ordering;
// use rand::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    example()
}
fn main2() {
    loop {
        println!("type in 1 to read file or type in 2 to write files or 3 to end");
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Error reading line");
        let output: i32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter 1, 2, or 3.");
                continue; // Restart the loop if the input is invalid
            }
        };
        match output {
            1 => println!("Path to file to read"),
            2 => println!("Path to file to readto write"),
            3 => {
                println!("Quitting...");
                break;
            }
            _ => {
                println!("Wrong option\nKindly input 1 or 2 ");
                continue;
            }
        };

        let mut filename = String::new();

        io::stdin()
            .read_line(&mut filename)
            .expect("Error reading line");

        if output == 1 {
            // Read from file
            match read_file_contents(&filename) {
                Ok(_) => println!("File successfully read!"),
                Err(err) => eprintln!("Error reading from file: {}", err),
            }
        } else {
            // Write to file
            println!("contents to write");
            let mut content_to_write = String::new();
            io::stdin()
                .read_line(&mut content_to_write)
                .expect("Error reading line");
            match write_to_file(&filename, &content_to_write) {
                Ok(_) => println!("File successfully written!"),
                Err(err) => eprintln!("Error writing to file: {}", err),
            }
        }
    }
}

fn read_file_contents(filename: &str) -> io::Result<()> {
    let file_path = filename.trim();
    let file_path = Path::new(file_path);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    println!("Contents of {}: ", filename);
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn write_to_file(filename: &str, content: &str) -> io::Result<()> {
    println!("filename : {}", filename);
    let file_path = filename.trim();
    let file_path = Path::new(file_path);
    let mut file = File::create(file_path)?;

    file.write_all(content.as_bytes())?;

    println!("Data written to {}", filename);
    Ok(())
}

fn example() {
    let mut s = String::from("Hello");
    s.push_str(",World");
    println!("{}", s);
}

// fn guessing_number() {
//       let  rng = rand::thread_rng().gen_range(1..=10);

// println!("Guess a number from 1-10!");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
//     let guess: u32 = guess.trim().parse().expect("Please type a number!");
//     match guess.cmp(&rng) {
//         Ordering::Less => println!("you guessed lesser"),
//         Ordering::Greater => println!("You guessed higher"),
//         Ordering::Equal=> println!("you guessed correctly")
//     }
// println!("Secret Number: {rng}");
//     println!("You guessed: {guess}");
// }
