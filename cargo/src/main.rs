use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

fn main() {
    // Initialize the Python interpreter
    Python::with_gil(|py| {
        // Import the Python module
        let sys = py.import("sys").expect("Failed to import sys module");

        // Call a Python function
        let version: String = sys.get("version").expect("Failed to get version").extract().expect("Failed to extract version");
        println!("Python version: {}", version);

        // Execute a Python script
        let locals = [("os", py.import("os").expect("Failed to import os module"))].into_py_dict(py);
        py.run("print(os.getcwd())", None, Some(&locals)).expect("Failed to execute Python script");
    });
}
