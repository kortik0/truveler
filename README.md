# Truveler 🌍
This is a Rust program that traverses through a directory and creates a JSON file containing the hash values of all the files in the directory. It detects duplicate files and stores them in the JSON file.

# Installation
Clone the repository:  `git clone https://github.com/kortik0/truveler.git`

Navigate to the project directory: `cd truveler`

# Usage
Build the program: 
`
cargo build --release
`

Run the program: 
`
./target/release/truveler [path_to_scan_directory]
`

# Credits
This program uses the following third-party libraries:

xxhash-rust for file hashing 

serde for JSON serialization and deserialization

# Contributing 🐱‍💻
Contributions are welcome! Feel free to open an issue or submit a pull request.
