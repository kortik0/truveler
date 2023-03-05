use std::time::Instant;
use std::path::{Path};
use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry;
use std::env;
use std::fs::{File, self};
use std::io::{BufReader, Read, Write};
use xxhash_rust::xxh3::{Xxh3};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Serialize, Deserialize, Debug)]
struct FileHash {
    path: String,
    hash: String,
}

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let mut file_queue: HashSet<String> = HashSet::new();

    traverse_fs(path.to_str().unwrap().to_string(), &mut file_queue);

    let mut map: HashMap<String, FileHash> = HashMap::new();
    let mut duplicate: Vec<FileHash> = Vec::new();

    for file in file_queue {
        let file = &file;
        let hash = hash_file(&file);

        //TODO: Erase duplication of hash creation

        match map.entry(hash.clone()) {
            Entry::Occupied(entry) => {
                let file_hash = FileHash {
                    path: String::from(file),
                    hash: String::from(hash),
                };

                duplicate.push(file_hash);
                println!("\x1b[0;33mWARNING: There is a duplicate file: {:?}\x1b[0;39m", file);
                println!("\t CACHE: {}", entry.get().hash);
            }
            Entry::Vacant(_) => {
                let file_hash = FileHash {
                    path: String::from(file),
                    hash: String::from(hash),
                };

                println!("FILE: {} \n\t CACHE: {}\n", &file_hash.path, &file_hash.hash);

                map.insert(file_hash.hash.clone(), file_hash);
            }
        }
    }

    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp = since_epoch.as_secs();

    let mut file = File::create(format!("C:\\Users\\Usver\\Documents\\traveler\\{timestamp}.json")).unwrap();
    let json = serde_json::to_string(&duplicate).expect("TODO: panic message");
    file.write_all(json.as_bytes()).expect("TODO: panic message2");

    println!("{:?}", Instant::now() - start);
}

fn hash_file(path: &String) -> String {
    const CHUNK_SIZE: usize = 1024 * 32;
    let mut file = BufReader::new(File::open(path).unwrap());
    let mut state = Xxh3::new();

    // Read the file contents into a buffer and feed it to the hash state
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_bytes = file.read(&mut buffer).unwrap();
        if num_bytes == 0 {
            break;
        }
        state.update(&buffer[..num_bytes]);
    }

    // Get the hash value as a u64
    let hash_value = state.digest();

    // Convert the hash value to a hexadecimal string
    let hash_value_hex = format!("{:016x}", hash_value);

    return hash_value_hex;
}

fn traverse_fs(initial_path: String, file_queue: &mut HashSet<String>) {
    let mut directory: VecDeque<String> = VecDeque::new();
    directory.push_back(initial_path);

    while let Some(path) = directory.pop_front() {
        match fs::read_dir(&path) {
            Ok(_) => {
                for dir_entry in fs::read_dir(&path).unwrap() {
                    let dir_entry = dir_entry.unwrap();
                    let entry_path = dir_entry.path();

                    let file_name = entry_path.file_name().unwrap().to_str().unwrap();
                    if file_name.contains("node_modules")
                        || file_name.contains("$RECYCLE.BIN")
                        || file_name.contains("packages")
                        || file_name.contains("maven-dependencies")
                        || file_name.starts_with('.')
                        || file_name.starts_with("Library")
                    {
                        continue;
                    };

                    if entry_path.is_file() {
                        file_queue.insert(entry_path.to_str().unwrap().to_string());
                    } else {
                        directory.push_back(entry_path.to_str().unwrap().to_string());
                    }
                }
            }
            Err(e) => {
                println!("ERROR: {}", e);
                continue;
            }
        }
    }
}