use dirs::{cache_dir, config_dir};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::process::Command;

#[no_mangle]
pub fn recognize(
    _base64: &str,
    _lang: &str,
    _needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let config_dir_path = config_dir().unwrap();

    let plugin_path = config_dir_path
        .join("com.pot-app.desktop")
        .join("plugins")
        .join("recognize")
        .join("[plugin].com.pot-app.paddle");
    let paddle_dir_path = plugin_path.join("PaddleOCR-json_v.1.3.0");
    let paddle_exe_path = paddle_dir_path.join("PaddleOCR-json.exe");

    let cache_dir_path = cache_dir().unwrap();
    let image_path = cache_dir_path
        .join("com.pot-app.desktop")
        .join("pot_screenshot_cut.png");

    let mut cmd = Command::new("cmd");
    let cmd = cmd.creation_flags(0x08000000);
    let cmd = cmd.args(["/c", &paddle_exe_path.to_str().unwrap()]);

    let output = cmd
        .current_dir(paddle_dir_path)
        .arg(&format!("--image_path={}", image_path.to_str().unwrap()))
        .output()?;

    let result = String::from_utf8_lossy(&output.stdout);
    let result = result.split("OCR init completed.\r\n").last().unwrap();
    let json = serde_json::from_str(result.trim())?;

    fn parse_json(res: Value) -> Option<String> {
        let mut target = String::new();
        let data = res.as_object()?.get("data")?.as_array()?;
        for line in data {
            let str = line.as_object()?.get("text")?.as_str()?;
            target.push_str(str);
            target.push_str("\n");
        }
        Some(target)
    }
    match parse_json(json) {
        Some(text) => Ok(Value::String(text)),
        None => Err("Parse Error".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let needs = HashMap::new();
        let result = recognize("", "", needs);
        println!("{result:?}");
    }
}
