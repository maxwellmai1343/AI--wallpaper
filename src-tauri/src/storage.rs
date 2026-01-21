use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
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

#[derive(Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub storage_path: Option<String>,
}

pub fn get_config_dir() -> PathBuf {
    if let Some(config_dir) = dirs::config_dir() {
        config_dir.join("ai-wallpaper-app")
    } else {
        // Fallback
        std::env::temp_dir().join("ai-wallpaper-app-config")
    }
}

pub fn load_config() -> AppConfig {
    let config_file = get_config_dir().join("config.json");
    if config_file.exists() {
        if let Ok(content) = fs::read_to_string(&config_file) {
            return serde_json::from_str(&content).unwrap_or_default();
        }
    }
    AppConfig::default()
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let config_dir = get_config_dir();
    fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    
    let config_file = config_dir.join("config.json");
    let content = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(config_file, content).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_storage_dir() -> PathBuf {
    // 1. Check config
    let config = load_config();
    if let Some(path_str) = config.storage_path {
        let path = PathBuf::from(path_str);
        if path.exists() || fs::create_dir_all(&path).is_ok() {
            return path;
        }
    }

    // 2. Default to Pictures/ai-wallpaper-app
    if let Some(pic_dir) = dirs::picture_dir() {
        return pic_dir.join("ai-wallpaper-app");
    }

    // 3. Fallback to Data Local
    if let Some(data_dir) = dirs::data_local_dir() {
        return data_dir.join("ai-wallpaper-app");
    }

    // 4. Final Fallback
    std::env::temp_dir().join("ai-wallpaper-app")
}

pub fn set_custom_storage_dir(path: String) -> Result<(), String> {
    let mut config = load_config();
    config.storage_path = Some(path);
    save_config(&config)
}

pub fn save_image(id: &str, data: &[u8]) -> Result<String, String> {
    let storage_dir = get_storage_dir();
    // Use "images" subdir to keep it organized
    let images_dir = storage_dir.join("images");
    fs::create_dir_all(&images_dir).map_err(|e| e.to_string())?;
    
    let file_path = images_dir.join(format!("{}.png", id));
    let mut file = fs::File::create(&file_path).map_err(|e| e.to_string())?;
    file.write_all(data).map_err(|e| e.to_string())?;
    
    Ok(file_path.to_string_lossy().to_string())
}

pub fn save_record(record: WallpaperRecord) -> Result<(), String> {
    let storage_dir = get_storage_dir();
    fs::create_dir_all(&storage_dir).map_err(|e| e.to_string())?;
    let history_file = storage_dir.join("history.json");
    
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
    let storage_dir = get_storage_dir();
    let history_file = storage_dir.join("history.json");
    
    if history_file.exists() {
        let content = fs::read_to_string(&history_file).map_err(|e| e.to_string())?;
        let history: History = serde_json::from_str(&content).unwrap_or_default();
        Ok(history.records)
    } else {
        Ok(vec![])
    }
}

pub fn clear_cache() -> Result<(), String> {
    let storage_dir = get_storage_dir();
    
    // Clear images
    let images_dir = storage_dir.join("images");
    if images_dir.exists() {
        fs::remove_dir_all(&images_dir).map_err(|e| e.to_string())?;
    }
    
    // Clear history.json
    let history_file = storage_dir.join("history.json");
    if history_file.exists() {
        fs::remove_file(&history_file).map_err(|e| e.to_string())?;
    }
    
    Ok(())
}
