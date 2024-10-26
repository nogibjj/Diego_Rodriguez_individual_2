"""
extract a dataset from URL
"""

import requests
import os
from main import measure_time_and_memory


@measure_time_and_memory
def extract(
    url="https://media.githubusercontent.com/media/nickeubank/MIDS_Data/master/World_Development_Indicators/wdi_small_tidy_2015.csv",
    file_path="data/wdi.csv",
    directory="data",
):
    """Extract a URL to a file path."""
    if not os.path.exists(directory):
        os.makedirs(directory)
    with requests.get(url) as r:
        with open(file_path, "wb") as f:
            f.write(r.content)
    return file_path


# Run the function with time and memory measurements
extract()
