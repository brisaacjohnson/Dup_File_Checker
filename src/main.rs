//TODO: create a duplicate file checker that can scan multiple directories and prompts the user before deleting them. 
//Add a gui as well that will present these choices to the user. But try to get it working in the temrinal first.
//Impports:
#![allow(unused)]
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use walkdir::WalkDir;
use std::process;

//Convert bytes to megabytes
pub fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / 1_000_000.0 // For MB (10^6)
}

// Function to find and handle duplicate files and prompt user for deletion
pub fn dup_finder(directories: &Path, cb: &dyn Fn(&DirEntry)) {
let directories: Vec<String> = vec![String::from(directories.to_str().unwrap())];
let mut checker = dupefinder::DupeFinder::new_recursive(directories);
let mut results = checker.run();
let total_duplicates = results.len();
println!("Found {} sets of duplicate files.", total_duplicates);
    for key in results.keys() {
        let result = results.get(key);
        if let Some(details) = result {
            println!("{} files of size {:.2} MB found with hash {}", details.files.len(), bytes_to_mb(details.size), details.hash);
            for file in details.files.iter() {
                println!("{}", file);
            }
            // Prompt user for deletion
            println!("Do you want to delete these files? (y/n): ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line, please input y or n.");
            if input.trim().to_lowercase() == "y" {
                for file in details.files.iter().skip(1) { // Keep one file, delete the rest
                    match fs::remove_file(file) {
                        Ok(_) => println!("Deleted file(s): {}", file),
                        Err(e) => eprintln!("Failed to delete file(s) {}: {}", file, e),       
        }
    }
} 
            else {
                println!("No files were deleted.");
            }
        }
    }   

}


fn main() {
    let path = Path::new("/Users/brisaacjohnson/Documents/Photos"); //find out how to prompt user for this. 

    let cb = |entry: &DirEntry| {
        println!("{:?}", entry.path());
    }; //needed to have .path method. Come back and find out why this is needed. And better understand closures in rust.

    dup_finder(path, &cb);
} 