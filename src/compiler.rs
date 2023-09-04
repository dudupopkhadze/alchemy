use std::fs;
use std::process::Command;

#[derive(Serialize)]
pub struct CompileResult {
    pub result: String,
    pub error: String,
}

pub fn compile_and_run(js_code: &str) -> CompileResult {
    // Initialize CompileResult
    let mut compile_result = CompileResult {
        result: String::new(),
        error: String::new(),
    };

    // Write the JavaScript code to "index.js"
    if let Err(e) = fs::write("index.js", js_code) {
        compile_result.error = format!("Failed to write to index.js: {}", e);
        return compile_result;
    }

    // Execute `node index.js`
    let output = Command::new("node")
        .arg("index.js")
        .output();

    // Clear "index.js" after running the code
    if let Err(e) = fs::File::create("index.js") {
        compile_result.error = format!("Failed to clear index.js: {}", e);
        return compile_result;
    }

    match output {
        Ok(output_data) => {
            let stdout = String::from_utf8_lossy(&output_data.stdout);
            compile_result.result = format!("{}", stdout);
        }
        Err(_) => {
            compile_result.error = "Failed to execute command".to_string();
        }
    }

    compile_result
}

