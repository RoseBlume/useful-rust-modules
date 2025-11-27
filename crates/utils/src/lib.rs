use std::sync::LazyLock;
mod collections;
#[cfg(any(feature="music-collection", feature="rosary-collection"))]
pub use collections::collect_music_files;


#[cfg(target_os = "windows")]
pub static USERNAME: LazyLock<String> = LazyLock::new(|| {
    std::env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string())
});

#[cfg(not(target_os = "windows"))]
pub static USERNAME: LazyLock<String> = LazyLock::new(|| {
    std::env::var("USER").unwrap_or_else(|_| "unknown".to_string())
});

#[cfg(feature = "music")]
pub static MUSIC_FOLDER_PATH: LazyLock<String> = LazyLock::new(|| {
    #[cfg(target_os = "android")]
    {
        "/storage/emulated/0/Music".to_string()
    }
    #[cfg(target_os = "windows")]
    {
        format!("C:\\Users\\{}\\Music", *USERNAME)
    }
    #[cfg(target_os = "linux")]
    {
        format!("/home/{}/Music", *USERNAME)
    }
});

#[cfg(feature = "documents")]
pub static DOCUMENTS_FOLDER_PATH: LazyLock<String> = LazyLock::new(|| {
    #[cfg(target_os = "android")]
    {
        "/storage/emulated/0/Documents".to_string()
    }
    #[cfg(target_os = "windows")]
    {
        format!("C:\\Users\\{}\\Documents", *USERNAME)
    }
    #[cfg(target_os = "linux")]
    {
        format!("/home/{}/Documents", *USERNAME)
    }
});

#[cfg(feature = "downloads")]
pub static DOWNLOADS_FOLDER_PATH: LazyLock<String> = LazyLock::new(|| {
    #[cfg(target_os = "android")]
    {
        "/storage/emulated/0/Downloads".to_string()
    }
    #[cfg(target_os = "windows")]
    {
        format!("C:\\Users\\{}\\Downloads", *USERNAME)
    }
    #[cfg(target_os = "linux")]
    {
        format!("/home/{}/Downloads", *USERNAME)
    }
});

#[cfg(feature = "pictures")]
pub static PICTURES_FOLDER_PATH: LazyLock<String> = LazyLock::new(|| {
    #[cfg(target_os = "android")]
    {
        "/storage/emulated/0/Pictures".to_string()
    }
    #[cfg(target_os = "windows")]
    {
        format!("C:\\Users\\{}\\Pictures", *USERNAME)
    }
    #[cfg(target_os = "linux")]
    {
        format!("/home/{}/Pictures", *USERNAME)
    }
});

#[cfg(feature = "videos")]
pub static VIDEOS_FOLDER_PATH: LazyLock<String> = LazyLock::new(|| {
    #[cfg(target_os = "android")]
    {
        "/storage/emulated/0/Movies".to_string()
    }
    #[cfg(target_os = "windows")]
    {
        format!("C:\\Users\\{}\\Videos", *USERNAME)
    }
    #[cfg(target_os = "linux")]
    {
        format!("/home/{}/Videos", *USERNAME)
    }
});

#[cfg(feature = "appdata")]
pub static APPDATA_FOLDER_PATH: LazyLock<String> = LazyLock::new(|| {
    #[cfg(target_os = "windows")]
    {
        format!("C:\\Users\\{}\\AppData", *USERNAME)
    }
    #[cfg(target_os = "linux")]
    {
        format!("/home/{}/.local/share", *USERNAME)
    }
});

#[cfg(feature = "local-appdata")]
pub static LOCAL_APPDATA_FOLDER_PATH: LazyLock<String> = LazyLock::new(|| {
    #[cfg(target_os = "windows")]
    {
        format!("C:\\Users\\{}\\AppData\\Local", *USERNAME)
    }
    #[cfg(target_os = "linux")]
    {
        format!("/home/{}/.local/share", *USERNAME)
    }
});

#[cfg(feature = "local-low-appdata")]
pub static LOCAL_LOW_APPDATA_FOLDER_PATH: LazyLock<String> = LazyLock::new(|| {
    #[cfg(target_os = "windows")]
    {
        format!("C:\\Users\\{}\\AppData\\LocalLow", *USERNAME)
    }
    #[cfg(target_os = "linux")]
    {
        format!("/home/{}/.local/share", *USERNAME)
    }
});

#[cfg(feature = "local-roaming-appdata")]
pub static LOCAL_ROAMING_APPDATA_FOLDER_PATH: LazyLock<String> = LazyLock::new(|| {
    #[cfg(target_os = "windows")]
    {
        format!("C:\\Users\\{}\\AppData\\Roaming", *USERNAME)
    }
    #[cfg(target_os = "linux")]
    {
        format!("/home/{}/.local/share", *USERNAME)
    }
});

#[cfg(feature = "rosary")]
pub static SCANFILE_PATH: LazyLock<String> = LazyLock::new(|| {
    #[cfg(target_os = "windows")]
    {
        format!("C:\\Users\\{}\\AppData\\Local\\Rosary Music\\scan.json", *USERNAME)
    }
    #[cfg(target_os = "macos")]
    {
        format!("/Users/{}/Library/Application Support/RosaryMusic/scan.json", *USERNAME)
    }
    #[cfg(target_os = "linux")]
    {
        format!("/home/{}/.config/Rosary Music/scan.json", *USERNAME)
    }
    #[cfg(target_os = "android")]
    {
        "/storage/emulated/0/Documents/scan.json".to_string()
    }
});



#[cfg(feature = "rosary")]
pub fn is_roman_alphabet(s: String) -> bool {
    let x = s.as_str();
    x.chars().all(|c| {
        c.is_ascii_alphabetic() ||
        c.is_ascii_digit() ||
        c.is_ascii_whitespace() ||
        c.is_ascii_punctuation()
    })
}


