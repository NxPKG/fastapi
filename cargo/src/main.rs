use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

fn main() {
    // Initialize the Python interpreter
    Python::with_gil(|py| {
        // Import the Python module
        let sys = py.import("sys").expect("Failed to import sys module");

        // Call a Python function
        let version: String = sys
            .getattr("version")?
            .extract()?;
            .extract()
            .expect("Failed to extract version");
        println!("Python version: {}", version);
    });
}
