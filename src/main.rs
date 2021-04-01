use reqwest::blocking::get;
use std::env::args;
use std::io::prelude::*;
use std::fs::File;
use console::style;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Arguments [binary (./qdl), uri (sh.rustup.rs), *optional* file to write to (rustupsetup.sh)]
    let data_to_dl:Vec<String> = args().collect();
    // Guard clause. Stops code from running if no arguments are provided.
    if data_to_dl.len() == 1 {
        println!("{}", style("Error: Not enough arguments!").red());
        return Ok(());
    }
    // Format the data_to_dl in a separate reusable variable for readability and speed purposes.
    let data_string = format!("{}", data_to_dl[1]);

    // Error handling for if the application fails to retrive the URL. Gives a nice, stylized error message instead of Thread MAIN panicked
    let data = match get(&data_string) {
        Ok(file) => file,
        Err(err) => {
             println!("{}", style(&format!("Error: {}", err)).red());
             return Ok(());
        }
    };

    // Nice stylized error message
    let data = match data.text() {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to process data: {}", err);
            return Ok(());
        }
    };

    // Guard clause to check if the arguments provided
    if data_to_dl.len() < 3 {
        println!("{}", data);
        return Ok(());
    }
    
    let file_name = format!("{}",data_to_dl[2]);

    let mut position = 0;
    let mut buffer = File::create(&file_name)?;

    let data = data.into_bytes();

    println!("{}", style(
        format!(
            "Downloading data from {1} to {0}...",
            file_name,
            data_string
        )
    ).cyan());

    while position < data.len() {
        let byte_write = buffer.write(&data[position..])?;
        position += byte_write;
    }

    println!("{}", style("Done! ✔").green());
    if data_to_dl.len() > 3 {
        println!("{}", style(
            "Warning: You included excess arguments. File names can only have spaces if they are enclosed in \"\" (\"my file.txt\")"
        ).
        yellow())
    }
    Ok(())
}
