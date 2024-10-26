"""
Main cli or app entry point
"""

from mylib.utils import measure_time_and_memory
from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import querycreate, queryRead, queryUpdate, queryDelete

# Extract
extract()

# Transform and Load
load()

# Query
querycreate()
queryRead()
queryUpdate()
queryDelete()


def main_res():
    results = {
        "extract_to": extract(),
        "transform_db": load(),
        "create": querycreate(),
        "read": queryRead(),
        "update": queryUpdate(),
        "delete": queryDelete(),
    }

    return results
