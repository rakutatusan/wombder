use std::env;
use std::fs;
use std::io::{self, Read, Write};

fn embed_text_into_image(text_file: &str, image_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the text file
    let mut text = String::new();
    fs::File::open(text_file)?.read_to_string(&mut text)?;

    // Prompt the user to create a password
    let mut password = String::new();
    println!("Create a password for the stego message:");
    io::stdin().read_line(&mut password)?;

    // Embed the text into the image
    let stego_message = format!("PASSWORD:{}\n{}", password.trim(), text);
    fs::write(image_file, stego_message)?;

    println!("Steganography message embedded successfully.");
    Ok(())
}

fn extract_text_from_image(image_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the steganography image
    let mut stego_message = String::new();
    fs::File::open(image_file)?.read_to_string(&mut stego_message)?;

    // Extract the password and text from the stego message
    let mut lines = stego_message.lines();
    let first_line = lines.next().ok_or("Invalid stego message")?;
    let (password_keyword, password) = first_line.split_once(':').ok_or("Invalid stego message")?;
    let text = lines.collect::<Vec<_>>().join("\n");

    if password_keyword.trim().to_lowercase() != "password" {
        return Err("Invalid stego message: Missing password".into());
    }

    // Prompt the user to enter the password
    let mut entered_password = String::new();
    println!("Enter the password to extract the steganography message:");
    io::stdin().read_line(&mut entered_password)?;

    if entered_password.trim() != password.trim() {
        println!("Wrong password. Extraction failed.");
        return Ok(());
    }

    // Write the extracted text to the output file
    fs::write(output_file, text)?;

    println!("Steganography message extracted successfully.");
    Ok(())
}

fn print_help() {
    println!("Available commands:");
    println!("-sm <text_file> -sr <image_file> : Embed a text file into an image.");
    println!("-sf <image_file> -esf <output_file> : Extract steganography message from an image.");
    println!("-h : Display help (this message).");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Invalid number of arguments.");
        print_help();
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "-sm" => {
            if args.len() < 5 {
                println!("Invalid number of arguments for embedding.");
                print_help();
                return;
            }
            let text_file = &args[2];
            let image_file = &args[4];
            embed_text_into_image(text_file, image_file).unwrap();
        }
        "-sf" => {
            if args.len() < 5 {
                println!("Invalid number of arguments for extraction.");
                print_help();
                return;
            }
            let image_file = &args[2];
            let output_file = &args[4];
            extract_text_from_image(image_file, output_file).unwrap();
        }
        "-h" => {
            print_help();
        }
        _ => {
            println!("Invalid command.");
            print_help();
        }
    }
}
