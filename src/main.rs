use rusqlite::Connection;
use rust_vs_python::{extract, load, query_create, query_read, query_update, query_delete, measure_time_and_memory};
use std::path::Path;
use std::fs;

// Function to set up the database and create the necessary table
fn setup_database() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("wdi.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS wdi (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            country TEXT,
            fertility_rate REAL,
            viral REAL,
            battle REAL,
            cpia_1 REAL,
            cpia_2 REAL,
            debt REAL
        )",
        [],
    )?;
    Ok(())
}

fn main() {
    let url = "https://media.githubusercontent.com/media/nickeubank/MIDS_Data/master/World_Development_Indicators/wdi_small_tidy_2015.csv";
    let file_path = "data/wdi.csv";
    let directory = "data";

    measure_time_and_memory("Extract", || extract(url, file_path, directory)).unwrap();
    measure_time_and_memory("Load", || load(file_path)).unwrap();
    measure_time_and_memory("Query Create", query_create).unwrap();
    measure_time_and_memory("Query Read", query_read).unwrap();
    measure_time_and_memory("Query Update", query_update).unwrap();
    measure_time_and_memory("Query Delete", query_delete).unwrap();
}

// Test functions
#[cfg(test)]
mod tests {
    use super::*;
    use rust_vs_python::{extract, load, query_create, query_read, query_update, query_delete};

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
        setup_database().expect("Failed to set up the database");

        let dataset = "data/wdi.csv";
        let result = load(dataset);
        assert!(result.is_ok());
        assert!(Path::new("wdi.db").exists());

        // Cleanup
        let _ = fs::remove_file("wdi.db");
    }

    #[test]
    fn test_query_create() {
        setup_database().expect("Failed to set up the database");

        let result = query_create();
        assert_eq!(result.unwrap(), "Create Success");
    }

    #[test]
    fn test_query_read() {
        setup_database().expect("Failed to set up the database");

        let result = query_read();
        assert_eq!(result.unwrap(), "Read Success");
    }

    #[test]
    fn test_query_update() {
        setup_database().expect("Failed to set up the database");

        let result = query_update();
        assert_eq!(result.unwrap(), "Update Success");
    }

    #[test]
    fn test_query_delete() {
        setup_database().expect("Failed to set up the database");

        let result = query_delete();
        assert_eq!(result.unwrap(), "Delete Success");
    }
}
