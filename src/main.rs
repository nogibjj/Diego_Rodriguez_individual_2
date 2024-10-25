// Cargo.toml dependencies:
// [dependencies]
// reqwest = { version = "0.11", features = ["blocking"] }
// csv = "1.1"
// rusqlite = "0.26"
// memory-stats = "0.2"

use std::{fs, io::Write, path::Path, time::Instant};
use reqwest::blocking::get;
use csv::ReaderBuilder;
use rusqlite::{params, Connection, Result};
use memory_stats::memory_stats;
use std::error::Error;

fn append_to_md_file(
    file_name: &str,
    message: &str,

) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)?;

    let mut file = std::io::BufWriter::new(file);
    let e_d = if encrypt { "encryption" } else { "decryption" };
    writeln!(file,"{}\n\n", message)?;
    println!("Content appended to {} successfully!", file_name);

    Ok(())
}




fn extract(url: &str, file_path: &str, directory: &str) -> Result<String, Box<dyn Error>> {
    if !Path::new(directory).exists() {
        fs::create_dir_all(directory)?;
    }

    let start = Instant::now();
    let initial_mem = memory_stats().map(|m| m.physical_mem).unwrap_or(0);

    let response = get(url)?.bytes()?;
    let mut file = fs::File::create(file_path)?;
    file.write_all(&response)?;

    let duration = start.elapsed();
    let final_mem = memory_stats().map(|m| m.physical_mem).unwrap_or(0);

    println!(
        "Extract completed in {:.2?} seconds, memory used: {} KB",
        duration,
        (final_mem - initial_mem) / 1024
    );

    Ok(file_path.to_string())
}

fn load(dataset: &str) -> Result<String, Box<dyn Error>> {
    let start = Instant::now();
    let initial_mem = memory_stats().map(|m| m.physical_mem).unwrap_or(0);

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

    let duration = start.elapsed();
    let final_mem = memory_stats().map(|m| m.physical_mem).unwrap_or(0);

    println!(
        "Load completed in {:.2?} seconds, memory used: {} KB",
        duration,
        (final_mem - initial_mem) / 1024
    );

    Ok("wdi.db".to_string())
}

fn measure_time_and_memory<F>(operation_name: &str, f: F) -> Result<String, Box<dyn Error>>
where
    F: Fn() -> Result<String, Box<dyn Error>>,
{
    let start = Instant::now();
    let initial_mem = memory_stats().map(|m| m.physical_mem).unwrap_or(0);

    let result = f()?;

    let duration = start.elapsed();
    let final_mem = memory_stats().map(|m| m.physical_mem).unwrap_or(0);

    println!(
        "{} completed in {:.2?} seconds, memory used: {} KB",
        operation_name,
        duration,
        (final_mem - initial_mem) / 1024
    );

    Ok(result)
}

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

    append_to_md_file("rust_vs_python.md", measure_time_and_memory("Extract", || extract(url, file_path, directory)).unwrap());
    append_to_md_file("rust_vs_python.md", measure_time_and_memory("Load", || load(file_path)).unwrap());
    append_to_md_file("rust_vs_python.md", measure_time_and_memory("Query Create", query_create).unwrap());
    append_to_md_file("rust_vs_python.md", measure_time_and_memory("Query Read", query_read).unwrap());
    append_to_md_file("rust_vs_python.md", measure_time_and_memory("Query Update", query_update).unwrap());
    append_to_md_file("rust_vs_python.md", measure_time_and_memory("Query Delete", query_delete).unwrap());
}

