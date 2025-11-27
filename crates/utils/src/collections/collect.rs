use std::path::PathBuf;
use std::fs;
use std::path::Path;

#[cfg(feature = "music")]
use crate::MUSIC_FOLDER_PATH;

#[cfg(feature = "documents")]
use crate::DOCUMENTS_FOLDER_PATH;

#[cfg(feature = "videos")]
use crate::VIDEOS_FOLDER_PATH;

#[cfg(feature = "downloads")]
use crate::DOWNLOADS_FOLDER_PATH;

#[cfg(feature = "pictures")]
use crate::PICTURES_FOLDER_PATH;

#[cfg(feature = "rosary-collection")]
pub fn collect_music_files() -> Vec<PathBuf> {
    let supported = ["mp3", "m4a", "wav", "flac"];
    // Check if path exists and is a directory
    let path = Path::new(&*MUSIC_FOLDER_PATH);
    
    if !path.exists() {
        eprintln!("Error: Path '{}' does not exist.", &*MUSIC_FOLDER_PATH);
    }
    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory.", &*MUSIC_FOLDER_PATH);
    }
    
    // Read directory entries
    let mut music_files: Vec<PathBuf> = Vec::new();
    for entry_result in fs::read_dir(path).expect("Failed to read directory") {
        match entry_result {
            Ok(entry) => {
                let file_type = entry.file_type().expect("Could not find file type");
                if file_type.is_file() {
                    let extension:Option<String> = Path::new(&entry.path().display().to_string()).extension()
                         .and_then(|ext| ext.to_str()) // Convert OsStr to &str
                         .map(|ext_str| ext_str.to_lowercase());
                    match extension {
                        Some(n) => {
                            if supported.contains(&n.as_str()) {
                                music_files.push(Path::new(&entry.path().display().to_string()).to_path_buf());
                            }
                            else {
                                #[cfg(debug_assertions)]
                                println!("Skipped File: {}\nFor Reason: Unsupported extension", entry.path().display());
                            }
                        },
                        None => {
                            #[cfg(debug_assertions)]
                            println!("Skipped File: {}\nFor Reason: Unsupported extension", entry.path().display());
                        },
                    }
                    
                } else if file_type.is_dir() {
                    println!("(Skipping directory) {}", entry.path().display());
                } else {
                    println!("(Other) {}", entry.path().display());
                }
            }
            Err(e) => eprintln!("Error reading entry: {}", e),
        }
    }
    music_files
}

#[cfg(feature = "music-collection")]
pub fn collect_music_files() -> Vec<PathBuf> {
    let path = Path::new(&*MUSIC_FOLDER_PATH);

    if !path.exists() {
        eprintln!("Error: Path '{}' does not exist.", &*MUSIC_FOLDER_PATH);
    }
    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory.", &*MUSIC_FOLDER_PATH);
    }

    let mut files: Vec<PathBuf> = Vec::new();

    for entry_result in fs::read_dir(path).expect("Failed to read directory") {
        match entry_result {
            Ok(entry) => {
                let file_type = entry.file_type().expect("Could not find file type");

                if file_type.is_file() {
                    files.push(entry.path());
                } else if file_type.is_dir() {
                    #[cfg(debug_assertions)]
                    println!("(Skipping directory) {}", entry.path().display());
                } else {
                    #[cfg(debug_assertions)]
                    println!("(Other) {}", entry.path().display());
                }
            }
            Err(e) => eprintln!("Error reading entry: {}", e),
        }
    }

    files
}

#[cfg(feature = "documents-collection")]
pub fn collect_document_files() -> Vec<PathBuf> {
    let path = Path::new(&*DOCUMENTS_FOLDER_PATH);

    if !path.exists() {
        eprintln!("Error: Path '{}' does not exist.", &*DOCUMENTS_FOLDER_PATH);
    }
    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory.", &*DOCUMENTS_FOLDER_PATH);
    }

    let mut files: Vec<PathBuf> = Vec::new();

    for entry_result in fs::read_dir(path).expect("Failed to read directory") {
        match entry_result {
            Ok(entry) => {
                let file_type = entry.file_type().expect("Could not find file type");

                if file_type.is_file() {
                    files.push(entry.path());
                } else if file_type.is_dir() {
                    #[cfg(debug_assertions)]
                    println!("(Skipping directory) {}", entry.path().display());
                } else {
                    #[cfg(debug_assertions)]
                    println!("(Other) {}", entry.path().display());
                }
            }
            Err(e) => eprintln!("Error reading entry: {}", e),
        }
    }

    files
}


#[cfg(feature = "downloads-collection")]
pub fn collect_download_files() -> Vec<PathBuf> {
    let path = Path::new(&*DOWNLOADS_FOLDER_PATH);

    if !path.exists() {
        eprintln!("Error: Path '{}' does not exist.", &*DOWNLOADS_FOLDER_PATH);
    }
    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory.", &*DOWNLOADS_FOLDER_PATH);
    }

    let mut files: Vec<PathBuf> = Vec::new();

    for entry_result in fs::read_dir(path).expect("Failed to read directory") {
        match entry_result {
            Ok(entry) => {
                let file_type = entry.file_type().expect("Could not find file type");

                if file_type.is_file() {
                    files.push(entry.path());
                } else if file_type.is_dir() {
                    #[cfg(debug_assertions)]
                    println!("(Skipping directory) {}", entry.path().display());
                } else {
                    #[cfg(debug_assertions)]
                    println!("(Other) {}", entry.path().display());
                }
            }
            Err(e) => eprintln!("Error reading entry: {}", e),
        }
    }

    files
}

#[cfg(feature = "pictures-collection")]
pub fn collect_picture_files() -> Vec<PathBuf> {
    let path = Path::new(&*PICTURES_FOLDER_PATH);

    if !path.exists() {
        eprintln!("Error: Path '{}' does not exist.", &*PICTURES_FOLDER_PATH);
    }
    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory.", &*PICTURES_FOLDER_PATH);
    }

    let mut files: Vec<PathBuf> = Vec::new();

    for entry_result in fs::read_dir(path).expect("Failed to read directory") {
        match entry_result {
            Ok(entry) => {
                let file_type = entry.file_type().expect("Could not find file type");

                if file_type.is_file() {
                    files.push(entry.path());
                } else if file_type.is_dir() {
                    #[cfg(debug_assertions)]
                    println!("(Skipping directory) {}", entry.path().display());
                } else {
                    #[cfg(debug_assertions)]
                    println!("(Other) {}", entry.path().display());
                }
            }
            Err(e) => eprintln!("Error reading entry: {}", e),
        }
    }

    files
}

#[cfg(feature = "videos-collection")]
pub fn collect_video_files() -> Vec<PathBuf> {
    let path = Path::new(&*VIDEOS_FOLDER_PATH);

    if !path.exists() {
        eprintln!("Error: Path '{}' does not exist.", &*VIDEOS_FOLDER_PATH);
    }
    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory.", &*VIDEOS_FOLDER_PATH);
    }

    let mut files: Vec<PathBuf> = Vec::new();

    for entry_result in fs::read_dir(path).expect("Failed to read directory") {
        match entry_result {
            Ok(entry) => {
                let file_type = entry.file_type().expect("Could not find file type");

                if file_type.is_file() {
                    files.push(entry.path());
                } else if file_type.is_dir() {
                    #[cfg(debug_assertions)]
                    println!("(Skipping directory) {}", entry.path().display());
                } else {
                    #[cfg(debug_assertions)]
                    println!("(Other) {}", entry.path().display());
                }
            }
            Err(e) => eprintln!("Error reading entry: {}", e),
        }
    }

    files
}