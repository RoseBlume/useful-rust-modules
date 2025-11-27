
#[cfg(any(feature = "music-collection", feature = "rosary"))]
use utils::collect_music_files;
#[cfg(any(feature = "music-collection", feature = "rosary"))]
use std::path::PathBuf;
#[cfg(any(feature = "music-collection", feature = "rosary"))]
#[test]
fn test_music_files_collection() {

    let music_files: Vec<PathBuf> = collect_music_files();

    for music_file in music_files {
        match music_file.to_str() {
            Some(s) => {
                #[cfg(debug_assertions)]
                println!("Found File in music: {}", s)
            },
            None => println!("Path contains invalid UTF-8"),
        }
    }
}