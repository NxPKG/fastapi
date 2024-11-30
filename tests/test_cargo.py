import subprocess
import unittest


class TestCargoIntegration(unittest.TestCase):
    def test_cargo_build(self):
        result = subprocess.run(
            ["cargo", "build", "--release"], capture_output=True, text=True
        )
        self.assertEqual(result.returncode, 0, f"Cargo build failed: {result.stderr}")

    def test_cargo_run(self):
        result = subprocess.run(["cargo", "run"], capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, f"Cargo run failed: {result.stderr}")

    def test_python_functionality_in_build(self):
        result = subprocess.run(
            ["cargo", "build", "--release"], capture_output=True, text=True
        )
        self.assertEqual(result.returncode, 0, f"Cargo build failed: {result.stderr}")
        # Check for Python functionality during build
        self.assertIn(
            "Python", result.stderr, "Python functionality not found during build"
        )

    def test_python_functionality_in_run(self):
        result = subprocess.run(["cargo", "run"], capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, f"Cargo run failed: {result.stderr}")
        # Check for Python functionality during run
        self.assertIn(
            "Python version:",
            result.stdout,
            "Python functionality not found during run",
        )


if __name__ == "__main__":
    unittest.main()
