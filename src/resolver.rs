use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

/// Reads the mapping file and returns a dictionary of import names to PyPI package names.
fn load_mapping_file(mapping_file: &str) -> Result<HashMap<String, String>, io::Error> {
    let mut mapping = HashMap::new();
    let file = File::open(mapping_file)?;

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();
            mapping.insert(key, value);
        }
    }

    Ok(mapping)
}

/// Resolves package names by looking them up in the mapping file.
/// If a mapping isn't found, the original import name is returned.
/// Accepts a HashSet<String> for import names to ensure uniqueness.
pub fn get_pkg_names(
    pkgs: HashSet<String>,
    mapping_file: &str,
) -> Result<HashSet<String>, io::Error> {
    let mapping = load_mapping_file(mapping_file)?;

    let mut result = HashSet::new();

    for pkg in pkgs {
        let resolved_pkg = mapping.get(&pkg).unwrap_or(&pkg).to_string();
        result.insert(resolved_pkg);
    }

    Ok(result)
}
