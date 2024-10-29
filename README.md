## Diego_Rodriguez_Individual_2: Rust CLI Binary with SQLite
[![Python CI/CD Pipeline](https://github.com/nogibjj/Diego_Rodriguez_individual_2/actions/workflows/hello.yml/badge.svg)](https://github.com/nogibjj/Diego_Rodriguez_individual_2/actions/workflows/hello.yml)
[![Rust CI/CD Pipeline](https://github.com/nogibjj/Diego_Rodriguez_individual_2/actions/workflows/bye.yml/badge.svg)](https://github.com/nogibjj/Diego_Rodriguez_individual_2/actions/workflows/bye.yml)
### File Structure
```
Diego_Rodriguez_individual_2/
├── .devcontainer/
│   ├── devcontainer.json
│   └── Dockerfile
├── .github/
│   ├── workflows/bye.yml
│   └── workflows/hello.yml
├── .gitignore
├── data/
│   └── wdi.csv
├── mylib/
│   ├── extract.py
│   ├── query.py
│   └── transform_load.py
├── src/
│   ├── lib.rs
│   └── main.rs
├── .gitignore
├── Cargo.toml
├── main.py
├── Makefile
├── README.md
├── requirements.txt
├── rust_vs_python.txt
├── test_main.py
└── wdi.db
```

### :bulb: Purpose of project
The purpose of this project is to build an ETL-Query pipeline comparing Rust and Python operations. I use World Bank, World Development Indicators dataset to extract it into a local csv file, transform the csv file by cleaning it, loading it into a .db ready for SQL queries from the command line.

### :movie_camera: Youtube Video
This [Demo](https://youtu.be/LlTd3onfDZs) is to guide you on how Rust its used in this repository and how you can run the functions from the command line. 

### :open_file_folder: Database Connection - Python
1. Under `mylib/` directory `extract.py` extract raw data from an online source. 
2. Under `mylib/` directory `transform_load.py` clean and transform raw data from `csv` to `db` and builds connections.
3. Under `mylib/` directory `utils.py` create a decorator to measure time and performance of python operations.
   
### :open_file_folder: Database Connection - Rust
1. Under `src/` directory `main.rs` holds CRUD operations to be perform on the CL and tests.
2. Under `src/` directory `lib.rs` define functions for extraction, loading, CRUD operations, and measure of time and performance of Rust operations. 

### :arrows_counterclockwise: CRUD Operations
Under `mylib/` directory `query.py` performs CRUD operations. For example: 
  1. Create: `querycreate()`
  2. Read: `queryRead()`
  3. Update: `queryUpdate()`
  4. Delete: `queryDelete()`

Under `src/` directory `main.rs` performs CRUD operations through the command line with `cargo run <operations>` . For example: 
  1. `extract` save the `.csv` file under `data/` directory
  2. `load` transform `.csv` file to a `.db` 
  3. `query_create` create a a new row for the `.db` file.
  4. `query_read` read the first 10 rows of the `.db` file.
  5. `query_update` update a given row of `.db` file.
  6. `query_delete` delete a given row of `.db` file.

### :chart_with_upwards_trend: Rust vs Python Performance
|Python | Extract | Load | Create | Read | Update | Delete |
| -------- | ------------ | ------------ | ------------ | ------------ | ------------ | ------------ |  
|Time (micro seconds)  | 53700.00 | 642.00 | 30.0 | 10.0 | 20.0 | 20.0 |
|Memory (kb)| 3328 | 512 | 0.0 | 0.0 | 0.0 | 0.0 |


|Rust | Extract | Load | Create | Read | Update | Delete |
| -------- | ------------ | ------------ | ------------ | ------------ | ------------ | ------------ |  
|Time (micro seconds)  | 72.00 | 233.01 | 1.95 | 2.30 | 1.0 | 2.0 |
|Memory (kb)| 9860 | 1232 | 0.0 | 0.0 | 0.0 | 0.0 |

###  :closed_book: Testing - Python
<img width="1090" alt="Testing" src="https://github.com/user-attachments/assets/aafafcc2-307a-4ab5-a204-9cc9e2a977fc">

###  :closed_book: Testing - Rust
<img width="673" alt="Screenshot 2024-10-27 at 5 36 55 PM" src="https://github.com/user-attachments/assets/6125e5bf-e2e0-4e84-943a-5e6b4409a582">

### :bangbang: Usage of LLMs
This project used OpenAI to translate Pyhton Syntax into Rust. Specifically the CRUD operations found under `mylib/` folder.
Specification of formulas and pipelines for CI/CD are author own creation are adjusted to work with this subset of information from the World Development Indicators dataset from the World Bank. 
