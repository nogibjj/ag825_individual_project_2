
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let content = response.text()?;

    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    println!("- Extraction completed from .csv file");

    Ok(())
}

pub fn load() -> Result<(), Box<dyn std::error::Error>> {
    let dataset = "Cancer_Data.csv";

    // let current_dir = std::env::current_dir()?;

    let mut rdr = csv::Reader::from_path(dataset)?;

    let conn = rusqlite::Connection::open("CancerDB.db")?;
    conn.execute("DROP TABLE IF EXISTS CancerDB", [])?;
    conn.execute(
        "CREATE TABLE CancerDB (
            id INTEGER,
            diagnosis TEXT,
            radius_mean REAL,
            texture_mean REAL,
            perimeter_mean REAL,
            area_mean REAL,
            smoothness_mean REAL
        )",
        [],
    )?;

    for result in rdr.deserialize() {
        let (id, diagnosis, radius_mean, texture_mean, perimeter_mean, area_mean, smoothness_mean): (i32, String, f64, f64, f64,  f64, f64) = result?;
        conn.execute(
            "INSERT INTO CancerDB (id, diagnosis, radius_mean, texture_mean, perimeter_mean, area_mean, smoothness_mean) VALUES (?1, ?2, ?3,?4, ?5, ?6, ?7)",
            params![id, diagnosis, radius_mean, texture_mean, perimeter_mean, area_mean, smoothness_mean],
        )?;
    }

    println!("- Data loaded into CancerDB.db successfully.");
    Ok(())
}

// Function to create and insert a record into the CancerDB table
pub fn create(database: &str) -> Result<(), Box<dyn Error>> {
    let conn = rusqlite::Connection::open(database)?;
    conn.execute(
        "INSERT INTO CancerDB (id, diagnosis, radius_mean, texture_mean, perimeter_mean, area_mean, smoothness_mean)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![123123123, "M", 12.3123312, 21.312312312, 12.3213123, 21.3123213, 23.1231312],
    )?;
    println!("- (CREATE) The following record has been inserted into the database:");

    let mut stmt = conn.prepare("SELECT * FROM CancerDB WHERE id = ?1 LIMIT 5")?;
    let rows = stmt.query_map(["123123123"], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, f64>(2)?,
            row.get::<_, f64>(3)?,
            row.get::<_, f64>(4)?,
            row.get::<_, f64>(5)?,
            row.get::<_, f64>(6)?,
        ))
    })?;

    for row in rows {
        println!("{:?}", row?);
    }
    Ok(())
}

// Function to read the first 5 rows of the CancerDB table
pub fn read(database: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(database)?;
    println!("- (READ) Top 5 rows of the CancerDB table:");

    let mut stmt = conn.prepare("SELECT * FROM CancerDB LIMIT 5")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, f64>(2)?,
            row.get::<_, f64>(3)?,
            row.get::<_, f64>(4)?,
            row.get::<_, f64>(5)?,
            row.get::<_, f64>(6)?,
        ))
    })?;

    for row in rows {
        println!("{:?}", row?);
    }

    Ok(())
}

// Function to update a specific record in the CancerDB table
pub fn update(database: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(database)?;
    conn.execute(
        "UPDATE CancerDB SET radius_mean = ?1 WHERE id = ?2",
        params!["32.234234423", "123123123"],
    )?;
    println!("- (UPDATE) The following record has been updated in the database:");

    let mut stmt = conn.prepare("SELECT * FROM CancerDB WHERE id = ?1 LIMIT 5")?;
    let rows = stmt.query_map(["123123123"], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, f64>(2)?,
            row.get::<_, f64>(3)?,
            row.get::<_, f64>(4)?,
            row.get::<_, f64>(5)?,
            row.get::<_, f64>(6)?,
        ))
    })?;

    for row in rows {
        println!("{:?}", row?);
    }

    Ok(())
}

// Function to delete a specific record from the CancerDB table
pub fn delete(database: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(database)?;
    conn.execute("DELETE FROM CancerDB WHERE id = ?1", params!["123123123"])?;
    println!(
        "- (DELETE) The record has been deleted from the database (no output will be seen here):"
    );

    let mut stmt = conn.prepare("SELECT * FROM CancerDB WHERE id = ?1 LIMIT 5")?;
    let rows = stmt.query_map(["123123123"], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, f64>(2)?,
            row.get::<_, f64>(3)?,
            row.get::<_, f64>(4)?,
            row.get::<_, f64>(5)?,
            row.get::<_, f64>(6)?,
        ))
    })?;

    for row in rows {
        println!("{:?}", row?);
    }

    Ok(())
}

// CRUD function to perform create, read, update, and delete operations
pub fn crud(database: &str) -> Result<String, Box<dyn std::error::Error>> {
    create(database)?;
    read(database)?;
    update(database)?;
    delete(database)?;

    Ok("SUCCESS".to_string())
}
