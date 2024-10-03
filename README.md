## Diego_Rodriguez_Miniproject5
[![CI/CD run](https://github.com/nogibjj/Diego_Rodriguez_Miniproject5/actions/workflows/hello.yml/badge.svg)](https://github.com/nogibjj/Diego_Rodriguez_Miniproject5/actions/workflows/hello.yml)
### File Structure
```
Diego_Rodriguez_Miniproject5/
├── .devcontainer/
│   ├── devcontainer.json
│   └── Dockerfile
├── .github/
│   └── workflows/hello.yml
├── .gitignore
├── data/
│   └── wdi.csv
├── mylib/
│   ├── extract.py
│   ├── query.py
│   └── transform_load.py
├── .gitignore
├── main.py
├── Makefile
├── README.md
├── requirements.txt
├── test_main.py
└── wdi.db
```

### Purpose of project
The purpose of this project is to build an ETL-Query pipeline. I use World Bank, World Development Indicators dataset to extract it into a local csv file, tranfrom the csv file by cleaning it, loading it into a .db ready for SQL queries.



### Database Connection
1. Under `mylib/` directory `exract.py` extract raw data from an online source. 
2. Under `mylib/` directory `transform_and_load.py` clean and transform raw data from `csv` to `db` and builds connections. 

### CRUD Operations
Under `mylib/` directory `query.py` performs QRUD operations. For exxample: 
1. Create: `querycreate()`
2. Read: `queryRead()`
3. Update: `queryUpdate()`
4. Delete: `queryDelete()`

