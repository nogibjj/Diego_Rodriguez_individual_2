"""
Query the database
"""

import sqlite3


def querycreate():
    conn = sqlite3.connect("wdi.db")
    cursor = conn.cursor()
    # create execution
    cursor.execute(
        "INSERT INTO wdi (country,fertility_rate,viral,battle,cpia_1) VALUES(1,1,1,1,1)"
    )
    conn.close()
    return "Create Success"


def queryRead():
    conn = sqlite3.connect("wdi.db")
    cursor = conn.cursor()
    # read execution
    cursor.execute("SELECT * FROM wdi LIMIT 10")
    conn.close()
    return "Read Success"


def queryUpdate():
    conn = sqlite3.connect("wdi.db")
    cursor = conn.cursor()
    # update execution
    cursor.execute("UPDATE wdi SET viral = 1 WHERE id = 1 ")
    conn.close()
    return "Update Success"


def queryDelete():
    conn = sqlite3.connect("wdi.db")
    cursor = conn.cursor()
    # delete execution
    cursor.execute("DELETE FROM wdi WHERE id = 3")
    conn.close()
    return "Delete Success"
