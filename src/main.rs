use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashSet;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn compare_directories(dir1: &str, dir2: &str) -> io::Result<()> {
    let mut differences = Vec::new();

    // Collect files from the first directory
    let dir1_files: Vec<_> = WalkDir::new(dir1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file()) // Only files, not directories
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

        // Create relative path based on dir1's root
        let relative_path = entry
            .path()
            .strip_prefix(dir1)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let file2_path = Path::new(dir2).join(relative_path);

        // If file doesn't exist in dir2, add to differences
        if !file2_path.exists() {
            differences.push(format!(
                "File present in {} but missing in {}: {}",
                dir1,
                dir2,
                relative_path.display()
            ));
            progress_bar.inc(1);
            continue;
        }

        progress_bar.inc(1);
    }

    // Finish progress bar with message
    progress_bar.finish_with_message("Comparison complete!");

    // Collect files from dir1 into a HashSet for quick lookup
    let dir1_files_set: HashSet<PathBuf> = dir1_files
        .iter() // Borrow dir1_files here
        .map(|e| e.path().strip_prefix(dir1).unwrap().to_path_buf())
        .collect();

    // Compare files in dir2 to files in dir1
    for entry in WalkDir::new(dir2).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            // Create relative path based on dir2's root
            let relative_path = entry
                .path()
                .strip_prefix(dir2)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            // If the file from dir2 doesn't exist in dir1, add to differences
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
        for diff in differences {
            println!("{}", diff);
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let dir1 = "I:/Media"; // Replace with your path
    let dir2 = "G:/Media"; // Replace with your path

    // Call the compare_directories function to compare the directories
    compare_directories(dir1, dir2)?;

    Ok(())
}
