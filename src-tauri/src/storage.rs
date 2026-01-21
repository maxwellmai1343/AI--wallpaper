use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WallpaperRecord {
    pub id: String,
    pub prompt: String,
    #[serde(rename = "imagePath")]
    pub image_path: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    pub resolution: Resolution,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Default)]
struct History {
    records: Vec<WallpaperRecord>,
}

pub fn get_app_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        // macOS 开发环境沙箱限制，临时使用 /tmp
        Path::new("/tmp").join("ai-wallpaper-app")
    }
    #[cfg(not(target_os = "macos"))]
    {
        // Windows 和 Linux 使用标准数据目录
        if let Some(data_dir) = dirs::data_local_dir() {
            return data_dir.join("ai-wallpaper-app");
        }
        // 回退到临时目录
        std::env::temp_dir().join("ai-wallpaper-app")
    }
}

pub fn save_image(id: &str, data: &[u8]) -> Result<String, String> {
    let app_dir = get_app_dir();
    let images_dir = app_dir.join("cache").join("images");
    fs::create_dir_all(&images_dir).map_err(|e| e.to_string())?;
    
    let file_path = images_dir.join(format!("{}.png", id));
    let mut file = fs::File::create(&file_path).map_err(|e| e.to_string())?;
    file.write_all(data).map_err(|e| e.to_string())?;
    
    Ok(file_path.to_string_lossy().to_string())
}

pub fn save_record(record: WallpaperRecord) -> Result<(), String> {
    let app_dir = get_app_dir();
    let history_file = app_dir.join("cache").join("history.json");
    
    let mut history = if history_file.exists() {
        let content = fs::read_to_string(&history_file).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        History::default()
    };
    
    history.records.push(record);
    
    let content = serde_json::to_string_pretty(&history).map_err(|e| e.to_string())?;
    fs::write(history_file, content).map_err(|e| e.to_string())?;
    
    Ok(())
}

pub fn get_history() -> Result<Vec<WallpaperRecord>, String> {
    let app_dir = get_app_dir();
    let history_file = app_dir.join("cache").join("history.json");
    
    if history_file.exists() {
        let content = fs::read_to_string(&history_file).map_err(|e| e.to_string())?;
        let history: History = serde_json::from_str(&content).unwrap_or_default();
        Ok(history.records)
    } else {
        Ok(vec![])
    }
}

pub fn clear_cache() -> Result<(), String> {
    let app_dir = get_app_dir();
    let cache_dir = app_dir.join("cache");
    if cache_dir.exists() {
        fs::remove_dir_all(&cache_dir).map_err(|e| e.to_string())?;
    }
    Ok(())
}
