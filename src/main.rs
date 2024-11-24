use clap::{Arg, Command};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashSet;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn compare_directories(dir1: &str, dir2: &str) -> io::Result<()> {
    let mut differences = Vec::new();

    // Validate that both directories exist
    if !Path::new(dir1).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Directory not found: {}", dir1),
        ));
    }
    if !Path::new(dir2).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Directory not found: {}", dir2),
        ));
    }

    // Collect files from the first directory
    let dir1_files: Vec<_> = WalkDir::new(dir1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .collect();

    // Create the progress bar
    let progress_bar = ProgressBar::new(dir1_files.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .expect("Failed to set progress bar template")
            .progress_chars("##-"),
    );

    // Compare files from dir1 to dir2
    for entry in &dir1_files {
        progress_bar.set_message(format!("Processing: {}", entry.path().display()));
        let relative_path = entry
            .path()
            .strip_prefix(dir1)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let file2_path = Path::new(dir2).join(relative_path);

        if !file2_path.exists() {
            differences.push(format!(
                "File present in {} but missing in {}: {}",
                dir1,
                dir2,
                relative_path.display()
            ));
        }
        progress_bar.inc(1);
    }

    progress_bar.finish_with_message("Comparison complete!");

    // Collect files from dir1 into a HashSet for quick lookup
    let dir1_files_set: HashSet<PathBuf> = dir1_files
        .iter()
        .map(|e| e.path().strip_prefix(dir1).unwrap().to_path_buf())
        .collect();

    // Compare files in dir2 to files in dir1
    for entry in WalkDir::new(dir2).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            let relative_path = entry
                .path()
                .strip_prefix(dir2)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            if !dir1_files_set.contains(relative_path) {
                differences.push(format!(
                    "File present in {} but missing in {}: {}",
                    dir2,
                    dir1,
                    relative_path.display()
                ));
            }
        }
    }

    // Print the results
    if differences.is_empty() {
        println!("The directories are identical.");
    } else {
        println!("\nFound {} differences:", differences.len());
        for diff in differences {
            println!("{}", diff);
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let matches = Command::new("Directory Comparison Script")
        .version("1.0")
        .author("Matthew Urrea")
        .about("Compares two directories and lists files that exist in one but not in the other")
        .arg(
            Arg::new("dir1")
                .help("First directory to compare")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("dir2")
                .help("Second directory to compare")
                .required(true)
                .index(2),
        )
        .get_matches();

    let dir1 = matches.get_one::<String>("dir1").unwrap();
    let dir2 = matches.get_one::<String>("dir2").unwrap();

    compare_directories(dir1, dir2)?;
    Ok(())
}
