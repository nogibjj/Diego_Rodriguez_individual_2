// Cargo.toml dependencies:
// [dependencies]
// reqwest = { version = "0.11", features = ["blocking"] }
// tokio = { version = "1", features = ["full"] }
// csv = "1.1"
// rusqlite = "0.26"

use std::{fs, io::Write, path::Path};
use reqwest::blocking::get;
use csv::ReaderBuilder;
use rusqlite::{params, Connection, Result};
use std::error::Error;

// Extracts data from a URL and saves it locally
fn extract(url: &str, file_path: &str, directory: &str) -> Result<String, Box<dyn Error>> {
    if !Path::new(directory).exists() {
        fs::create_dir_all(directory)?;
    }

    let response = get(url)?.bytes()?;
    let mut file = fs::File::create(file_path)?;
    file.write_all(&response)?;

    Ok(file_path.to_string())
}

// Loads data from CSV into the SQLite database
fn load(dataset: &str) -> Result<String, Box<dyn Error>> {
    let file = fs::File::open(dataset)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let conn = Connection::open("wdi.db")?;

    conn.execute("DROP TABLE IF EXISTS wdi", [])?;
    conn.execute(
        "CREATE TABLE wdi (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            country TEXT,
            fertility_rate INTEGER,
            viral INTEGER,
            battle INTEGER,
            cpia_1 INTEGER,
            cpia_2 INTEGER,
            debt INTEGER
        )",
        [],
    )?;

    let mut stmt = conn.prepare(
        "INSERT INTO wdi (country, fertility_rate, viral, battle, cpia_1, cpia_2, debt) 
        VALUES (?, ?, ?, ?, ?, ?, ?)",
    )?;

    for result in rdr.records().skip(1) {
        let record = result?;
        stmt.execute(params![
            record.get(0),
            record.get(1).and_then(|s| s.parse::<i32>().ok()),
            record.get(2).and_then(|s| s.parse::<i32>().ok()),
            record.get(3).and_then(|s| s.parse::<i32>().ok()),
            record.get(4).and_then(|s| s.parse::<i32>().ok()),
            record.get(5).and_then(|s| s.parse::<i32>().ok()),
            record.get(6).and_then(|s| s.parse::<i32>().ok())
        ])?;
    }

    Ok("wdi.db".to_string())
}

// CRUD Operations
fn query_create() -> Result<String> {
    let conn = Connection::open("wdi.db")?;
    conn.execute(
        "INSERT INTO wdi (country, fertility_rate, viral, battle, cpia_1) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![1, 1, 1, 1, 1],
    )?;
    Ok("Create Success".to_string())
}

fn query_read() -> Result<String> {
    let conn = Connection::open("wdi.db")?;
    let mut stmt = conn.prepare("SELECT * FROM wdi LIMIT 10")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, Option<i32>>(0)?,
            row.get::<_, Option<String>>(1)?,
            row.get::<_, Option<i32>>(2)?,
            row.get::<_, Option<i32>>(3)?,
            row.get::<_, Option<i32>>(4)?,
            row.get::<_, Option<i32>>(5)?,
            row.get::<_, Option<i32>>(6)?,
            row.get::<_, Option<i32>>(7)?,
        ))
    })?;

    for row in rows {
        println!("{:?}", row?);
    }
    Ok("Read Success".to_string())
}

fn query_update() -> Result<String> {
    let conn = Connection::open("wdi.db")?;
    conn.execute("UPDATE wdi SET viral = 1 WHERE id = 1", [])?;
    Ok("Update Success".to_string())
}

fn query_delete() -> Result<String> {
    let conn = Connection::open("wdi.db")?;
    conn.execute("DELETE FROM wdi WHERE id = 3", [])?;
    Ok("Delete Success".to_string())
}

fn main() {
    let url = "https://media.githubusercontent.com/media/nickeubank/MIDS_Data/master/World_Development_Indicators/wdi_small_tidy_2015.csv";
    let file_path = "data/wdi.csv";
    let directory = "data";

    if let Ok(path) = extract(url, file_path, directory) {
        println!("File saved to {}", path);
    } else {
        eprintln!("Error saving file.");
    }

    if let Ok(db_path) = load(file_path) {
        println!("Database saved as {}", db_path);
    } else {
        eprintln!("Error loading database.");
    }

    if let Ok(msg) = query_create() {
        println!("{}", msg);
    }
    if let Ok(msg) = query_read() {
        println!("{}", msg);
    }
    if let Ok(msg) = query_update() {
        println!("{}", msg);
    }
    if let Ok(msg) = query_delete() {
        println!("{}", msg);
    }
}


mod tests {
    use super::*;
    use std::path::Path;
    use std::fs;

    #[test]
    fn test_extract() {
        let url = "https://media.githubusercontent.com/media/nickeubank/MIDS_Data/master/World_Development_Indicators/wdi_small_tidy_2015.csv";
        let file_path = "data/test_wdi.csv";
        let directory = "data";
        let result = extract(url, file_path, directory);
        assert!(result.is_ok());
        assert!(Path::new(file_path).exists());

        // Cleanup
        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_load() {
        let dataset = "data/wdi.csv";
        let result = load(dataset);
        assert!(result.is_ok());
        assert!(Path::new("wdi.db").exists());

        // Cleanup
        let _ = fs::remove_file("wdi.db");
    }

    #[test]
    fn test_query_create() {
        let result = query_create();
        assert_eq!(result.unwrap(), "Create Success");
    }

    #[test]
    fn test_query_read() {
        let result = query_read();
        assert_eq!(result.unwrap(), "Read Success");
    }

    #[test]
    fn test_query_update() {
        let result = query_update();
        assert_eq!(result.unwrap(), "Update Success");
    }

    #[test]
    fn test_query_delete() {
        let result = query_delete();
        assert_eq!(result.unwrap(), "Delete Success");
    }
}