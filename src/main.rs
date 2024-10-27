use rust_vs_python::{
    extract, load, measure_time_and_memory, query_create, query_delete, query_read, query_update,
};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument is provided
    if args.len() < 2 {
        println!("Usage: cargo run <operation>");
        println!("Available operations: extract, load, query_create, query_read, query_update, query_delete");
        return;
    }

    let operation = &args[1];

    // Define the URL, file path, and directory
    let url = "https://media.githubusercontent.com/media/nickeubank/MIDS_Data/master/World_Development_Indicators/wdi_small_tidy_2015.csv";
    let file_path = "data/wdi.csv";
    let directory = "data";

    match operation.as_str() {
        "extract" => {
            measure_time_and_memory("Extract", || extract(url, file_path, directory)).unwrap();
        }
        "load" => {
            measure_time_and_memory("Load", || load(file_path)).unwrap();
        }
        "query_create" => {
            measure_time_and_memory("Query Create", query_create).unwrap();
        }
        "query_read" => {
            measure_time_and_memory("Query Read", query_read).unwrap();
        }
        "query_update" => {
            measure_time_and_memory("Query Update", query_update).unwrap();
        }
        "query_delete" => {
            measure_time_and_memory("Query Delete", query_delete).unwrap();
        }
        _ => {
            println!("Unknown operation: {}", operation);
            println!("Available operations: extract, load, query_create, query_read, query_update, query_delete");
        }
    }
}


// Test functions
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

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
