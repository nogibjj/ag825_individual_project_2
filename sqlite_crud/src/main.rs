
use sqlite_crud::extract;
use sqlite_crud::load;
use sqlite_crud::crud;

// use clap::{Parser, ValueEnum};
use std::error::Error;

// #[derive(ValueEnum, Clone, Debug)]
// enum CliAction {
//     Extract,
//     Load,
//     Crud,
// }

// #[derive(Parser)]
// #[command(name = "CRUD Script", version = "1.0", about = "A CLI for handling data extraction, transformation, and CRUD operations.")]
// struct Cli {
//     #[arg(value_enum)]
//     action: CliAction, // Use the CliAction enum for the action field

//     /// SQL query for 'general_query' action
//     query: Option<String>, // Optional SQL query parameter
// }

fn main() -> Result<(), Box<dyn Error>> {

    let url = "https://raw.githubusercontent.com/nogibjj/ag825_sqlite_lab/refs/heads/main/Cancer_Data.csv";
    let file_path = "Cancer_Data.csv";

    println!("Extracting data...");
    extract(url, file_path)?;
    println!("Transforming and loading data...");
    load()?;
    println!("Performing CRUD operations...");
    crud(file_path)?;
        



    // Handle each action
    // match args.action {
    //     CliAction::Extract => {
    //         println!("Extracting data...");
    //         extract(url, file_path)?;
    //     }
    //     CliAction::Load => {
    //         println!("Transforming and loading data...");
    //         load()?;
    //     }
    //     CliAction::Crud => {
    //         println!("Performing CRUD operations...");
    //         crud(file_path)?;
    //     }
    // }

    Ok(())
}





