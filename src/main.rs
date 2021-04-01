use reqwest::blocking::get;
#[allow(unused_imports)]
use std::env::args;
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use console::style;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let data_to_dl:Vec<String> = args().collect();
    
    if data_to_dl.len() == 1 {
        println!("{}", style("Error: Not enough arguments!").red());
        return Ok(());
    }

    let data_string = format!("{}", data_to_dl[1]);

    let data = match get(&data_string) {
        Ok(file) => file,
        Err(err) => {
             println!("{}", style(&format!("Error: {}", err)).red());
             return Ok(());
        }
    };
    let data = match data.text() {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to process data: {}", err);
            return Ok(());
        }
    };

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
    Ok(())
}
