use sqlite_crud::crud;
use sqlite_crud::extract;
use sqlite_crud::load;
use std::error::Error;

mod test;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://raw.githubusercontent.com/nogibjj/ag825_individual_project_2/refs/heads/main/sqlite_crud/Cancer_Data.csv";
    let file_path = "Cancer_Data.csv";
    let database = "CancerDB.db";

    println!("Extracting data...");
    extract(url, file_path)?;
    println!("Transforming and loading data...");
    load()?;
    println!("Performing CRUD operations...");
    crud(database)?;

    Ok(())
}
