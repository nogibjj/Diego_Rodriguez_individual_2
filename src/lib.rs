use csv::ReaderBuilder;
use memory_stats::memory_stats;
use reqwest::blocking::get;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::{fs, io::Write, path::Path, time::Instant};

pub fn extract(url: &str, file_path: &str, directory: &str) -> Result<String, Box<dyn Error>> {
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

pub fn load(dataset: &str) -> Result<String, Box<dyn Error>> {
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
            fertility_rate REAL,
            viral REAL,
            battle REAL,
            cpia_1 REAL,
            cpia_2 REAL,
            debt REAL
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

pub fn measure_time_and_memory<F>(operation_name: &str, f: F) -> Result<String, Box<dyn Error>>
where
    F: Fn() -> Result<String, Box<dyn Error>>,
{
    let start = Instant::now();
    let initial_mem = memory_stats().map(|m| m.physical_mem).unwrap_or(0);

    let result = f()?;

    let duration = start.elapsed();
    let final_mem = memory_stats().map(|m| m.physical_mem).unwrap_or(0);

    let output = format!(
        "{} Rust completed in {:.4?} seconds, memory used: {} KB\n",
        operation_name,
        duration,
        (final_mem - initial_mem) / 1024
    );
    println!("{}", output);

    // Append the output to rust_vs_python.md
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("rust_vs_python.md")?;
    println!("Attempting to write to rust_vs_python.md");
    file.write_all(output.as_bytes())?;
    println!("Write operation to rust_vs_python.md succeeded.");

    Ok(result)
}

pub fn query_create() -> Result<String, Box<dyn Error>> {
    let conn = Connection::open("wdi.db")?;
    conn.execute(
        "INSERT INTO wdi (country, fertility_rate, viral, battle, cpia_1) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![1, 1, 1, 1, 1],
    )?;
    Ok("Create Success".to_string())
}

pub fn query_read() -> Result<String, Box<dyn Error>> {
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

pub fn query_update() -> Result<String, Box<dyn Error>> {
    let conn = Connection::open("wdi.db")?;
    conn.execute("UPDATE wdi SET viral = 1 WHERE id = 1", [])?;
    Ok("Update Success".to_string())
}

pub fn query_delete() -> Result<String, Box<dyn Error>> {
    let conn = Connection::open("wdi.db")?;
    conn.execute("DELETE FROM wdi WHERE id = 3", [])?;
    Ok("Delete Success".to_string())
}