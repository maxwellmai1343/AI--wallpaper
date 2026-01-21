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

pub struct OpenAiAdapter {
    pub api_key: String,
    pub model: String,
}

impl AiApiAdapter for OpenAiAdapter {
    async fn generate_image(&self, prompt: &str, width: u32, height: u32) -> Result<Vec<u8>, String> {
        println!("Using OpenAiAdapter with model: {}", self.model);
        
        let client = reqwest::Client::new();
        let api_url = "https://api.siliconflow.cn/v1/images/generations";

        let request_body = serde_json::json!({
            "model": self.model,
            "prompt": prompt,
            "image_size": format!("{}x{}", width, height),
            "batch_size": 1,
            "num_inference_steps": 20,
            "guidance_scale": 7.5
        });

        println!("Sending request to SiliconFlow...");
        
        let response = client.post(api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| format!("Network request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("API Error: {}", error_text));
        }

        let response_json: serde_json::Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        // Extract image URL from response
        // SiliconFlow (OpenAI format) returns: { "data": [ { "url": "..." } ] }
        if let Some(data) = response_json.get("data").and_then(|d| d.as_array()).and_then(|arr| arr.first()) {
            if let Some(url) = data.get("url").and_then(|u| u.as_str()) {
                println!("Image URL received: {}", url);
                
                // Download the image
                let image_bytes = client.get(url)
                    .send()
                    .await
                    .map_err(|e| format!("Failed to download image: {}", e))?
                    .bytes()
                    .await
                    .map_err(|e| format!("Failed to read image bytes: {}", e))?;
                
                return Ok(image_bytes.to_vec());
            }
        }

        Err("Failed to extract image URL from response".to_string())
    }
}
