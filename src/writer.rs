use std::fs::{File, OpenOptions};
use std::io::{self, Write};

pub fn write_requirements(packages: Vec<(String, String)>, path: &str, force: bool) -> io::Result<()> {
    let req_file_path = format!("{}/requirements.txt", path);

    let file = if force {
        File::create(&req_file_path)?
    } else {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&req_file_path)?
    };

    let mut file = io::BufWriter::new(file);

    for (pkg, version) in packages {
        writeln!(file, "{}=={}", pkg, version)?;
    }

    Ok(())
}
