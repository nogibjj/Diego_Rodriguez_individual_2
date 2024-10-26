import time
import psutil


def measure_time_and_memory(func):
    """time and memory usage of a f_x and save to rust_vs_python.md."""

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
            f"Python Function '{func.__name__}' completed in {duration:.4f} seconds\n"
            f"Memory used: {memory_used:.4f} KB\n\n"
        )

        # Append the log entry to rust_vs_python.md
        with open("rust_vs_python.md", "a") as f:
            f.write(log_entry)

        print(log_entry)  # Print to console for immediate feedback
        return result

    return wrapper
