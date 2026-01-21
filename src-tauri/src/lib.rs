mod ai;
mod storage;

use crate::ai::{MockAiAdapter, AiApiAdapter};
use crate::storage::{WallpaperRecord, Resolution};
use uuid::Uuid;
use chrono::Utc;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;

#[tauri::command]
async fn generate_wallpaper(prompt: String, width: u32, height: u32) -> Result<String, String> {
    println!("Frontend called generate_wallpaper: prompt={}, size={}x{}", prompt, width, height);
    
    let adapter = MockAiAdapter;
    let image_data = adapter.generate_image(&prompt, width, height).await?;
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

    // 尝试多种方法设置壁纸
    
    // 方法1: 使用 macOS AppleScript (通常最可靠)
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        println!("使用 AppleScript 设置壁纸...");
        
        // 使用 Finder 而不是 System Events，通常权限要求更低
        let script = format!(
            "tell application \"Finder\" to set desktop picture to POSIX file \"{}\"",
            image_path
        );
        
        match Command::new("osascript").arg("-e").arg(&script).output() {
            Ok(output) => {
                if output.status.success() {
                    println!("AppleScript (Finder) 设置成功");
                    return Ok(());
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);
                    println!("AppleScript (Finder) 设置失败: {}", error);
                    
                    // 再次尝试 System Events，但这次使用 POSIX file 格式
                     let script_sys = format!(
                        "tell application \"System Events\" to set picture of every desktop to POSIX file \"{}\"",
                        image_path
                    );
                    if let Ok(out_sys) = Command::new("osascript").arg("-e").arg(&script_sys).output() {
                        if out_sys.status.success() {
                             println!("AppleScript (System Events) 设置成功");
                             return Ok(());
                        }
                    }
                }
            }
            Err(e) => {
                println!("执行 AppleScript 错误: {}", e);
            }
        }
    }

    // 方法2: 使用 wallpaper crate
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
            
            // Fallback: 如果自动设置失败（通常是权限问题），则在 Finder 中显示该文件
            println!("尝试在 Finder 中显示文件...");
            #[cfg(target_os = "macos")]
            {
                let _ = std::process::Command::new("open")
                    .arg("-R")
                    .arg(&image_path)
                    .output();
            }
            
            Err(format!("由于 macOS 权限限制，无法自动设置壁纸。已为您在 Finder 中打开图片，请右键手动设置为桌面背景。"))
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
        .invoke_handler(tauri::generate_handler![
            generate_wallpaper,
            get_screen_resolution,
            set_wallpaper,
            get_wallpaper_history,
            clear_cache
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
