mod cli;
mod scanner;
mod resolver;
mod version;
mod writer;

use cli::get_cli_args;
use scanner::find_python_imports;
use resolver::get_pkg_names;
use version::get_package_version;
use writer::write_requirements;
use std::process::exit;

fn main() {
    let args = get_cli_args();
    let path = args.directory;
    let force = args.force;

    if !path.exists() || !path.is_dir() {
        eprintln!("Error: '{}' is not a valid directory.", path.display());
        exit(1);
    }

    let imports = find_python_imports(&path);

    if imports.is_empty() {
        println!("No external dependencies found in '{}'", path.display());
    } else {

        match get_pkg_names(imports, "src/mapping") {
            Ok(resolved_pkgs) => {
                let mut packages_with_versions = Vec::new();
                for pkg in resolved_pkgs {
                    match get_package_version(&pkg, path.to_str().unwrap()) {
                        Ok(version) => {
                            packages_with_versions.push((pkg, version));
                        }
                        Err(e) => {
                            eprintln!("Error fetching version for {}: {:?}", pkg, e);
                        }
                    }
                }

                if let Err(e) = write_requirements(packages_with_versions, path.to_str().unwrap(), force) {
                    eprintln!("Error writing to requirements.txt: {}", e);
                    exit(1);
                }

                println!("requirements.txt has been successfully written.");
            }
            Err(err) => {
                eprintln!("Error resolving package names: {}", err);
                exit(1);
            }
        }
    }
}
