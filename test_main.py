"""
Test goes here
"""

import os
import shutil

from main import main_res
from mylib.extract import extract


def test_func():
    results = main_res()
    assert results == {
        "extract_to": "data/wdi.csv",
        "transform_db": "wdi.db",
        "create": "Create Success",
        "read": "Read Success",
        "update": "Update Success",
        "delete": "Delete Success",
    }


def test_extract():  # to test in case of if condition
    # Ensure clean state before testing
    if os.path.exists("data"):
        shutil.rmtree("data")  # Remove directory to test directory creation

    # Run the extract function
    result = extract()

    # Check if the directory and file are created
    assert result == "data/wdi.csv"
    assert os.path.exists("data/wdi.csv")

    # Clean up after test
    shutil.rmtree("data")


if __name__ == "__main__":
    test_func()
    test_extract()
