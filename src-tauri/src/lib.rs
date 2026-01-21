mod ai;
mod storage;

use crate::ai::{MockAiAdapter, AiApiAdapter, OpenAiAdapter};
use crate::storage::{WallpaperRecord, Resolution};
use uuid::Uuid;
use chrono::Utc;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;

use tauri::http::{Response, StatusCode};
use std::fs;
use std::path::PathBuf;

const APP_PASSWORD: &str = "123456";

fn get_mime_type(path: &PathBuf) -> String {
    match path.extension().and_then(|e| e.to_str()) {
        Some("png") => "image/png".to_string(),
        Some("jpg") | Some("jpeg") => "image/jpeg".to_string(),
        Some("gif") => "image/gif".to_string(),
        Some("svg") => "image/svg+xml".to_string(),
        Some("webp") => "image/webp".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

#[tauri::command]
fn verify_password(password: String) -> bool {
    password == APP_PASSWORD
}

#[tauri::command]
async fn generate_wallpaper(prompt: String, width: u32, height: u32, api_key: String, model: String) -> Result<String, String> {
    println!("Frontend called generate_wallpaper: prompt={}, size={}x{}, model={}", prompt, width, height, model);
    
    let image_data = if api_key.is_empty() {
        println!("No API Key provided, using Mock Adapter");
        let adapter = MockAiAdapter;
        adapter.generate_image(&prompt, width, height).await?
    } else {
        println!("API Key provided, using OpenAiAdapter");
        let adapter = OpenAiAdapter { api_key, model };
        adapter.generate_image(&prompt, width, height).await?
    };

    println!("Image generated, size: {} bytes", image_data.len());
    
    let id = Uuid::new_v4().to_string();
    println!("Saving image with ID: {}", id);
    
    match storage::save_image(&id, &image_data) {
        Ok(path) => {
            println!("Image saved to: {}", path);
            
            let record = WallpaperRecord {
                id,
                prompt,
                image_path: path.clone(),
                created_at: Utc::now().timestamp_millis(),
                resolution: Resolution { width, height },
            };
            
            if let Err(e) = storage::save_record(record) {
                println!("Failed to save record: {}", e);
                // We don't fail the whole request if history save fails
            }
            
            Ok(path)
        }
        Err(e) => {
            println!("Failed to save image: {}", e);
            Err(e)
        }
    }
}

#[tauri::command]
fn get_screen_resolution(window: tauri::Window) -> Result<(u32, u32), String> {
    if let Ok(Some(monitor)) = window.current_monitor() {
        let size = monitor.size();
        Ok((size.width, size.height))
    } else {
        Ok((1920, 1080))
    }
}

#[tauri::command]
async fn set_wallpaper(image_path: String) -> Result<(), String> {
    println!("尝试设置壁纸，路径: {}", image_path);

    // 使用 wallpaper crate 设置壁纸 (跨平台，支持 Windows)
    println!("使用 wallpaper crate 设置壁纸...");
    match wallpaper::set_from_path(&image_path) {
        Ok(_) => {
            println!("wallpaper crate 设置成功");
            if let Err(e) = wallpaper::set_mode(wallpaper::Mode::Crop) {
                println!("设置显示模式失败 (非致命): {}", e);
            }
            Ok(())
        }
        Err(e) => {
            let err_msg = format!("自动设置壁纸失败: {}", e);
            println!("{}", err_msg);
            
            // Windows Fallback: 尝试在资源管理器中打开
            println!("尝试在资源管理器中显示文件...");
            #[cfg(target_os = "windows")]
            {
                let _ = std::process::Command::new("explorer")
                    .arg("/select,")
                    .arg(&image_path)
                    .output();
            }
            
            Err(format!("无法自动设置壁纸。已为您在文件管理器中打开图片，请右键手动设置为桌面背景。"))
        }
    }
}

#[tauri::command]
fn get_wallpaper_history() -> Result<Vec<WallpaperRecord>, String> {
    storage::get_history()
}

#[tauri::command]
fn clear_cache() -> Result<(), String> {
    storage::clear_cache()
}

#[tauri::command]
async fn import_image(file_path: String) -> Result<String, String> {
    println!("Importing image from: {}", file_path);
    
    // 读取原始图片文件
    let data = std::fs::read(&file_path).map_err(|e| format!("Failed to read file at {}: {}", file_path, e))?;
    
    // 生成新的ID并保存到应用缓存
    let id = Uuid::new_v4().to_string();
    let saved_path = storage::save_image(&id, &data).map_err(|e| format!("Failed to save image: {}", e))?;
    
    // 获取图片尺寸 (简单起见，这里先假设为屏幕尺寸，或者使用默认值)
    // 在生产环境中，应该使用 image crate 读取实际尺寸
    let (width, height) = (1920, 1080);
    
    let record = WallpaperRecord {
        id,
        prompt: "Imported Image".to_string(),
        image_path: saved_path.clone(),
        created_at: Utc::now().timestamp_millis(),
        resolution: Resolution { width, height },
    };
    
    storage::save_record(record).map_err(|e| format!("Failed to save record: {}", e))?;
    
    Ok(saved_path)
}

#[tauri::command]
fn get_storage_path() -> Result<String, String> {
    Ok(storage::get_storage_dir().to_string_lossy().to_string())
}

#[tauri::command]
fn set_storage_path(path: String) -> Result<(), String> {
    storage::set_custom_storage_dir(path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .register_uri_scheme_protocol("wallpaper-image", |_, request| {
            let uri = request.uri();
            let path_str = uri.path();
            
            // URL decode
            let decoded_path = percent_encoding::percent_decode_str(path_str).decode_utf8_lossy();
            
            // Handle Windows path (remove leading slash if it looks like /C:/...)
            let path_to_read = if cfg!(windows) && decoded_path.starts_with('/') {
                &decoded_path[1..]
            } else {
                // For macOS/Linux, we need to ensure it's treated as an absolute path
                // If path_str starts with "localhost", we should ignore it if it was added by the frontend
                if decoded_path.starts_with("localhost") {
                     // This shouldn't happen with our frontend logic but good for safety
                     &decoded_path[9..] 
                } else {
                    &decoded_path
                }
            };
            
            let file_path = PathBuf::from(path_to_read.to_string());
            println!("Protocol reading: {:?}", file_path);

            match fs::read(&file_path) {
                Ok(data) => {
                    let mime_type = get_mime_type(&file_path);
                    Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", mime_type)
                        .header("Access-Control-Allow-Origin", "*")
                        .body(data)
                        .unwrap()
                },
                Err(e) => {
                    println!("Protocol read error: {}", e);
                    Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .header("Access-Control-Allow-Origin", "*")
                        .body("File not found".as_bytes().to_vec())
                        .unwrap()
                }
            }
        })
        .setup(|app| {
            // 创建退出菜单项
            let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;
            
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            generate_wallpaper,
            get_screen_resolution,
            set_wallpaper,
            get_wallpaper_history,
            clear_cache,
            import_image,
            get_storage_path,
            set_storage_path,
            verify_password
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
