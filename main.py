"""
Main cli or app entry point
"""

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


def measure_time_and_memory(func):
    """Decorator to measure time and memory usage of a function and save to rust_vs_python.md."""

    def wrapper(*args, **kwargs):
        process = psutil.Process()
        initial_mem = process.memory_info().rss  # Initial memory in bytes
        start_time = time.time()  # Start time

        result = func(*args, **kwargs)  # Run the function

        duration = time.time() - start_time  # Time elapsed
        final_mem = process.memory_info().rss  # Final memory in bytes
        memory_used = (final_mem - initial_mem) / 1024  # Convert to KB

        # Prepare the log entry
        log_entry = (
            f"Function '{func.__name__}' completed in {duration:.2f} seconds\n"
            f"Memory used: {memory_used:.2f} KB\n\n"
        )

        # Append the log entry to rust_vs_python.md
        with open("rust_vs_python.md", "a") as f:
            f.write(log_entry)

        print(log_entry)  # Print to console for immediate feedback
        return result

    return wrapper
