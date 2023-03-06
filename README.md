# Truveler 🌍
🎲 Welcome to Truveler! 🌎

Are you tired of losing track of all your files and getting lost in the endless maze of folders? 🤔

Fear no more! With Truveler, you'll be able to easily identify and organize all your files with just a few clicks. 🙌

🕵️‍♂️ Truveler uses state-of-the-art algorithms to traverse your file system and locate all your files, even those pesky hidden ones. 🕵️‍♀️

🔍 It then calculates their unique hash values and compares them to identify duplicate files. Truveler is so smart, it can even detect files with the same content but different names. 😎

🗂️ Once all your files have been analyzed, Truveler creates a neat JSON report containing all the duplicates found, so you can easily delete them and free up valuable disk space. 📈

🎉 Say goodbye to cluttered folders and hello to a more organized and efficient workflow with Truveler! 🎉

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
