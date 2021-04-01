use reqwest::blocking::get;

use std::env::args;
use std::io::prelude::*;
use std::fs::File;

use console::style;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let data_to_dl:Vec<String> = args().collect();
    
    let data_string = format!("{}", data_to_dl[1]);

    let data = get(&data_string)?
        .text()?;

    if data_to_dl.len() < 2 {

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
