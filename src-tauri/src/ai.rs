use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct GenerateRequest {
    pub prompt: String,
    pub width: u32,
    pub height: u32,
}

pub trait AiApiAdapter {
    async fn generate_image(&self, prompt: &str, width: u32, height: u32) -> Result<Vec<u8>, String>;
}

pub struct MockAiAdapter;

impl AiApiAdapter for MockAiAdapter {
    async fn generate_image(&self, prompt: &str, width: u32, height: u32) -> Result<Vec<u8>, String> {
        // Mock实现：获取占位图片
        
        // 如果提示词是 "test"，则尝试读取项目根目录下的 test.png
        if prompt == "test" {
            // 打印当前工作目录，方便调试
            if let Ok(current_dir) = std::env::current_dir() {
                println!("当前工作目录: {:?}", current_dir);
                
                // 尝试多个可能的路径
                let possible_paths = vec![
                    current_dir.join("test.png"), // 如果在项目根目录运行
                    current_dir.parent().unwrap_or(&current_dir).join("test.png"), // 如果在 src-tauri 运行
                    current_dir.join("../test.png"), // 相对路径尝试
                ];

                for path in possible_paths {
                    println!("尝试寻找测试图片: {:?}", path);
                    if path.exists() {
                        println!("找到测试图片: {:?}", path);
                        if let Ok(bytes) = std::fs::read(&path) {
                            return Ok(bytes);
                        }
                    }
                }
                println!("未找到 test.png");
            }
        }

        // 使用 AI+Wallpaper 作为占位文字，避免中文编码问题
        let url = format!("https://placehold.co/{}x{}/png?text=AI+Wallpaper", width, height);
        let bytes = reqwest::get(&url)
            .await
            .map_err(|e| e.to_string())?
            .bytes()
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(bytes.to_vec())
    }
}
