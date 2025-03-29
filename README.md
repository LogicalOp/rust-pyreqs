
# PyReqs

PyReqs is a simple Rust-based command-line tool for generating a `requirements.txt` file for Python projects. It scans a specified directory for Python imports and resolves them into a list of dependencies.

## Features

- Scans Python files in a directory to detect imports.
- Filters out standard library modules and personal imports.
- Deduplicates submodules (e.g., `sklearn.neighbors` becomes `sklearn`).
- Writes the resolved dependencies to a `requirements.txt` file.
- Supports overwriting the `requirements.txt` file with the `--force` flag.

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/rust-pyreqs.git
   ```

2. Build the tool using Cargo:

    ```bash
    cargo build --release
    ```

3. Run the compiled binary:

    ```bash
    ./target/release/rust-pyreqs
    ```

4. Optional add to path

    ```bash
    cargo install --path .
    ```

## Usage/Examples

Basic Command
To scan a directory and generate a requirements.txt file:

```bash
    rust-pyreqs <directory>
```

Example:

```bash
    rust-pyreqs ./my-project
```

Overwrite requirements.txt
To force overwrite the requirements.txt file, use the --force flag:

```bash
    rust pyreqs ./my-project --force
```

Options:

--force or -f: Overwrite the requirements.txt file if it already exists.

`<directory>`: The directory to scan for Python imports. Defaults to the current directory (.).

## Limitations

- This tool does not resolve version conflicts between dependencies.
- It assumes that all imports are available on PyPI.

## Contributing

Contributions are welcome. Feel free to open issues or submit pull requests.

## License

[MIT](https://choosealicense.com/licenses/mit/)
