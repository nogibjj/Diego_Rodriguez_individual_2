// tests.rs asserting each function
mod tests{
    use std::path::Path;
    use std::fs;
    use crate::{extract, load, query_create, query_read, query_update, query_delete};

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