"""
Transforms and Loads data into the local SQLite3 database
"""

import csv
import sqlite3
from mylib.utils import measure_time_and_memory


@measure_time_and_memory
def load(dataset="data/wdi.csv"):
    """Transforms and Loads data into the local SQLite3 database"""
    # Open and read the CSV file
    with open(dataset, newline="") as file:
        payload = csv.reader(file, delimiter=",")

        # Skip the header of CSV
        next(payload)

        # Filter rows to ensure only the first 7 columns are selected
        filtered_payload = [row[:7] for row in payload]

    # Connect to SQLite database
    conn = sqlite3.connect("wdi.db")
    c = conn.cursor()

    # Drop the table if it already exists
    c.execute("DROP TABLE IF EXISTS wdi")

    # Create the table with the required schema
    c.execute(
        """
        CREATE TABLE wdi (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            country TEXT,
            fertility_rate INTEGER,
            viral INTEGER,
            battle INTEGER,
            cpia_1 INTEGER,
            cpia_2 INTEGER,
            debt INTEGER
        )
        """
    )

    # Insert the filtered data into the table
    c.executemany(
        """
        INSERT INTO wdi (
            country, 
            fertility_rate, 
            viral, battle, cpia_1, 
            cpia_2, 
            debt
        ) 
        VALUES (?, ?, ?, ?, ?, ?, ?)
        """,
        filtered_payload,
    )

    # Commit the transaction and close the connection
    conn.commit()
    conn.close()

    return "wdi.db"
