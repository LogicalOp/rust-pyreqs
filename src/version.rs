use reqwest::blocking::Client;
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum VersionError {
    PyPiError(String),
}

impl std::fmt::Display for VersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionError::PyPiError(msg) => write!(f, "PyPiError: {}", msg),
        }
    }
}

fn get_latest_version_from_pypi(package: &str) -> Result<String, VersionError> {
    let url = format!("https://pypi.org/pypi/{}/json", package);
    let client = Client::new();
    match client.get(&url).send() {
        Ok(response) => {
            if response.status().is_success() {
                let json: Value = response.json().unwrap();
                if let Some(version) = json["info"]["version"].as_str() {
                    return Ok(version.to_string());
                } else {
                    return Err(VersionError::PyPiError(format!("No version info found for {}", package)));
                }
            } else {
                return Err(VersionError::PyPiError(format!("Failed to fetch from PyPI for {}", package)));
            }
        }
        Err(err) => Err(VersionError::PyPiError(format!("Request failed: {}", err))),
    }
}

/// Read version from `requirements.txt` or `pyproject.toml` if it exists
fn get_version_from_file(package: &str, dir: &str) -> Option<String> {
    let req_file_path = format!("{}/requirements.txt", dir);
    if Path::new(&req_file_path).exists() {
        if let Ok(content) = fs::read_to_string(req_file_path) {
            for line in content.lines() {
                if let Some((pkg, version)) = parse_requirements_line(line) {
                    if pkg == package {
                        return Some(version);
                    }
                }
            }
        }
    }

    let pyproject_path = format!("{}/pyproject.toml", dir);
    if Path::new(&pyproject_path).exists() {
        if let Ok(content) = fs::read_to_string(pyproject_path) {
            if let Some(version) = parse_toml_for_version(&content, package) {
                return Some(version);
            }
        }
    }

    None
}

/// Parse `requirements.txt` line for package and version
fn parse_requirements_line(line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = line.split("==").collect();
    if parts.len() == 2 {
        return Some((parts[0].to_string(), parts[1].to_string()));
    }
    None
}

/// Parse `pyproject.toml` for version (basic version)
fn parse_toml_for_version(toml_content: &str, package: &str) -> Option<String> {
    // A basic search for a version in `[tool.poetry.dependencies]` section of pyproject.toml
    if let Some(start) = toml_content.find(package) {
        let sub_str = &toml_content[start..];
        if let Some(end) = sub_str.find('\n') {
            let version_line = &sub_str[..end];
            let parts: Vec<&str> = version_line.split("=").collect();
            if parts.len() == 2 {
                return Some(parts[1].trim().to_string());
            }
        }
    }
    None
}

/// Fetches the version of a package, checking both project files and PyPI
pub fn get_package_version(package: &str, dir: &str) -> Result<String, VersionError> {
    if let Some(version) = get_version_from_file(package, dir) {
        return Ok(version);
    }

    // If no version is found in project files, fetch from PyPI
    get_latest_version_from_pypi(package)
}

