use rayon::prelude::*;
use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

const EXCLUDED_DIRS: [&str; 5] = ["venv", "__pycache__", "env", ".tox", ".git"];

/// Runs a Python command to fetch system stdlib modules
fn get_stdlib_modules() -> HashSet<String> {
    let output = Command::new("python")
        .arg("-c")
        .arg("import sys, json; print(json.dumps(list(sys.stdlib_module_names)))")
        .output();

    match output {
        Ok(output) if output.status.success() => {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                if let Ok(Value::Array(modules)) = serde_json::from_str::<Value>(&stdout) {
                    return modules
                        .into_iter()
                        .filter_map(|m| m.as_str().map(String::from))
                        .collect();
                }
            }
        }
        _ => eprintln!("Warning: Failed to fetch system stdlib modules. Some stdlib modules may be incorrectly marked as dependencies."),
    }

    HashSet::new()
}

/// Checks if a directory should be skipped
fn is_excluded(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|name| EXCLUDED_DIRS.contains(&name))
        .unwrap_or(false)
}

/// Detects local modules (directories with __init__.py and .py files)
fn find_local_modules(dir: &PathBuf) -> HashSet<String> {
    let mut local_modules = HashSet::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if is_excluded(&entry) {
            continue;
        }

        if path.extension().map_or(false, |ext| ext == "py") || path.join("__init__.py").exists() {
            if let Some(relative_path) = path.strip_prefix(dir).ok() {
                let module_name = relative_path
                    .with_extension("")
                    .to_string_lossy()
                    .replace("/", ".")
                    .replace("\\", ".");

                local_modules.insert(module_name.clone());

                let mut parts = module_name.split('.').collect::<Vec<_>>();
                while !parts.is_empty() {
                    local_modules.insert(parts.join("."));
                    parts.pop();
                }
            }
        }
    }
    local_modules
}

/// Regex patterns for extracting imports
fn get_import_regex() -> [Regex; 2] {
    [
        Regex::new(r"^import\s+([\w\.]+)").unwrap(),
        Regex::new(r"^from\s+([\w\.]+)\s+import").unwrap(),
    ]
}

/// Reads a Python file and extracts unique imports, excluding local and stdlib modules
fn extract_imports(
    file_path: &PathBuf,
    regex_patterns: &[Regex],
    local_modules: &HashSet<String>,
    stdlib_modules: &HashSet<String>,
) -> HashSet<String> {
    let mut imports = HashSet::new();

    if let Ok(content) = fs::read_to_string(file_path) {
        for line in content.lines() {
            for regex in regex_patterns {
                if let Some(captures) = regex.captures(line) {
                    if let Some(module) = captures.get(1) {
                        let module_name = module.as_str().to_string();
                        
                        if !local_modules.contains(&module_name) && !stdlib_modules.contains(&module_name) {
                            imports.insert(module_name);
                        }
                    }
                }
            }
        }
    }

    imports
}

/// Scans a directory for `.py` files and extracts imports, excluding local and stdlib modules
pub fn find_python_imports(dir: &PathBuf) -> HashSet<String> {
    let regex_patterns = get_import_regex();
    let local_modules = find_local_modules(dir);
    let stdlib_modules = get_stdlib_modules();

    WalkDir::new(dir)
        .into_iter()
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|entry| !is_excluded(entry))
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "py"))
        .flat_map(|entry| {
            extract_imports(
                &entry.path().to_path_buf(),
                &regex_patterns,
                &local_modules,
                &stdlib_modules,
            )
        })
        .collect()
}