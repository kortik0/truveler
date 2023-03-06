use std::time::Instant;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::{File, self};
use std::io::{BufReader, Read, Write};
use xxhash_rust::xxh3::{Xxh3};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Serialize, Deserialize, Debug)]
struct FileHash {
    path: PathBuf,
    hash: String,
    size: f64,
    dub: Option<PathBuf>,
}

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let home_dir = env::var("USERPROFILE").unwrap();
    let documents_path = format!("{home_dir}\\Documents");
    let mut file_queue: HashSet<String> = HashSet::new();

    traverse_fs(path.to_str().unwrap().to_string(), &mut file_queue);

    let mut map: HashMap<String, FileHash> = HashMap::new();
    let mut duplicates: Vec<FileHash> = Vec::new();

    for file in file_queue {
        let path = &file;

        let hash = hash_file(path);

        let size_in_mb = (PathBuf::from(path).metadata().unwrap().len() as f64) / 1_000_000.0;

        if let Some(entry) = map.get(&hash) {
            let dub_path = &entry.path;

            let file_hash = FileHash {
                path: path.clone().parse().unwrap(),
                hash: hash.clone(),
                size: size_in_mb,
                dub: Some(dub_path.to_owned()),
            };

            duplicates.push(file_hash);

            println!("\x1b[0;33mWARNING: There is a duplicate file: {:?}\x1b[0;39m", path);
            println!("\t CACHE: {}", entry.hash);
        } else {
            let file_hash = FileHash {
                path: path.clone().parse().unwrap(),
                hash: hash.clone(),
                size: size_in_mb,
                dub: None,
            };

            map.insert(hash.clone(), file_hash);
            println!("FILE: {} \n\t CACHE: {}\n", path, hash);
        }
    }

    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp = since_epoch.as_secs();

    let dir_name = format!("{documents_path}\\traveler\\");

    if Path::new(&dir_name).exists() {
        println!("Directory already exists");
    } else {
        match fs::create_dir(dir_name) {
            Ok(()) => println!("Directory created successfully"),
            Err(e) => println!("Error creating directory: {}", e),
        }
    }

    let mut file = File::create(format!("{documents_path}\\traveler\\{timestamp}.json")).unwrap();
    let json = serde_json::to_string(&duplicates).expect("TODO: panic message");
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

    let hash_value = state.digest();
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

                    //TODO: Everywhere path must be reduced to PathBuf
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