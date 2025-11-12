use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use eventsource_stream::Eventsource;
use futures_util::StreamExt;
use notify_rust::{Notification, Timeout};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Duration;

// Configuration structure
#[derive(Debug, Default)]
struct Config {
    api_key: String,
    model: String,                   // Fallback/default model
    ocr_model: Option<String>,       // Model for convert mode
    reasoning_model: Option<String>, // Model for solve mode
    temperature: f32,
    max_tokens_per_page: u32,
    timeout_per_page: u64,
    extract_metadata: bool,
    metadata_fields: String,
    custom_prompt: Option<String>,
    system_prompt_file: Option<String>,
}

// Operation mode
#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    Convert,
    Solve,
}

// API request structures
#[derive(Serialize)]
struct ApiRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
    stream: bool,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: Vec<ContentItem>,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum ContentItem {
    Text { text: String },
    File { file: FileData },
}

#[derive(Serialize)]
struct FileData {
    filename: String,
    file_data: String,
}

// API response structures
#[allow(dead_code)]
#[derive(Deserialize)]
struct ApiResponse {
    choices: Option<Vec<Choice>>,
    error: Option<ApiError>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

#[derive(Deserialize, Debug)]
struct ApiError {
    message: String,
}

// Streaming response structures
#[derive(Deserialize, Debug)]
struct StreamResponse {
    #[allow(dead_code)]
    id: Option<String>,
    choices: Option<Vec<StreamChoice>>,
    error: Option<ApiError>,
}

#[derive(Deserialize, Debug)]
struct StreamChoice {
    delta: StreamDelta,
    #[serde(default)]
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
struct StreamDelta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    reasoning_content: Option<String>,
}

// Notification manager
struct NotificationManager {
    id: Option<u32>,
    title: String,
}

impl NotificationManager {
    fn new() -> Self {
        Self {
            id: None,
            title: String::new(),
        }
    }

    fn show_loading(&mut self, title: &str, message: &str) {
        self.title = title.to_string();
        println!("{}", message);

        if let Ok(handle) = Notification::new()
            .summary(title)
            .body(message)
            .icon("process-working")
            .timeout(Timeout::Never)
            .show()
        {
            self.id = Some(handle.id());
        }
    }

    fn update(&self, message: &str) {
        println!("{}", message);

        if let Some(id) = self.id {
            let _ = Notification::new()
                .summary(&self.title)
                .body(message)
                .icon("process-working")
                .timeout(Timeout::Never)
                .id(id)
                .show();
        }
    }

    fn update_with_reasoning(&self, main_message: &str, reasoning: &str) {
        println!("{}", main_message);
        if !reasoning.is_empty() {
            println!("Reasoning: {}", reasoning);
        }

        if let Some(id) = self.id {
            let body = if reasoning.is_empty() {
                main_message.to_string()
            } else {
                format!("{}\n\n💭 {}", main_message, reasoning)
            };

            let _ = Notification::new()
                .summary(&self.title)
                .body(&body)
                .icon("process-working")
                .timeout(Timeout::Never)
                .id(id)
                .show();
        }
    }

    fn success(&mut self, message: &str) {
        println!("✓ {}", message);

        if let Some(id) = self.id {
            let _ = Notification::new()
                .summary("Task Complete")
                .body(message)
                .icon("dialog-ok-apply")
                .timeout(Timeout::Milliseconds(3000))
                .id(id)
                .show();
            self.id = None;
        }
    }

    fn error(&mut self, message: &str) {
        eprintln!("Error: {}", message);

        if let Some(id) = self.id {
            let _ = Notification::new()
                .summary("Task Failed")
                .body(message)
                .icon("dialog-error")
                .timeout(Timeout::Milliseconds(5000))
                .id(id)
                .show();
            self.id = None;
        }
    }
}

impl Drop for NotificationManager {
    fn drop(&mut self) {
        if self.id.is_some() {
            self.error("Script interrupted or failed.");
        }
    }
}

fn get_config_dir() -> PathBuf {
    let xdg_config = env::var("XDG_CONFIG_HOME").ok().and_then(|p| {
        if p.is_empty() {
            None
        } else {
            Some(PathBuf::from(p))
        }
    });

    xdg_config
        .or_else(|| dirs::home_dir().map(|p| p.join(".config")))
        .unwrap_or_else(|| PathBuf::from(".config"))
        .join("kmarkdownify")
}

fn get_prompts_dir() -> PathBuf {
    let system_paths = [
        PathBuf::from("/usr/share/kmarkdownify/prompts"),
        PathBuf::from("/usr/local/share/kmarkdownify/prompts"),
    ];

    for path in &system_paths {
        if path.exists() {
            return path.clone();
        }
    }

    // Fallback to current directory
    PathBuf::from(".")
}

fn load_config() -> Result<Config> {
    let config_dir = get_config_dir();
    let api_key_file = config_dir.join("api_key");
    let config_file = config_dir.join("config");

    // Load API key
    let api_key = fs::read_to_string(&api_key_file)
        .with_context(|| {
            format!(
            "API key not found at {:?}\n\nYou can get an API key from: https://openrouter.ai/keys",
            api_key_file
        )
        })?
        .trim()
        .to_string();

    if api_key.is_empty() {
        anyhow::bail!("API key file is empty: {:?}", api_key_file);
    }

    // Default configuration
    let mut config = Config {
        api_key,
        model: "mistralai/pixtral-large-latest".to_string(),
        ocr_model: None,
        reasoning_model: None,
        temperature: 0.1,
        max_tokens_per_page: 8000,
        timeout_per_page: 60,
        extract_metadata: true,
        metadata_fields: "Title,Author,Course,Due Date".to_string(),
        custom_prompt: None,
        system_prompt_file: None,
    };

    // Load optional config file
    if config_file.exists() {
        let content = fs::read_to_string(&config_file)?;
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "MODEL" => config.model = value.to_string(),
                    "OCR_MODEL" => config.ocr_model = Some(value.to_string()),
                    "REASONING_MODEL" => config.reasoning_model = Some(value.to_string()),
                    "TEMPERATURE" => config.temperature = value.parse().unwrap_or(0.1),
                    "MAX_TOKENS" => config.max_tokens_per_page = value.parse().unwrap_or(8000), // Backwards compatibility
                    "MAX_TOKENS_PER_PAGE" => {
                        config.max_tokens_per_page = value.parse().unwrap_or(8000)
                    }
                    "TIMEOUT_PER_PAGE" => config.timeout_per_page = value.parse().unwrap_or(60),
                    "EXTRACT_METADATA" => config.extract_metadata = value == "true",
                    "METADATA_FIELDS" => config.metadata_fields = value.to_string(),
                    "CUSTOM_PROMPT" => config.custom_prompt = Some(value.to_string()),
                    "SYSTEM_PROMPT_FILE" => config.system_prompt_file = Some(value.to_string()),
                    _ => {}
                }
            }
        }
    }

    Ok(config)
}

fn get_prompt_text(config: &Config, mode: Mode) -> Result<String> {
    // Priority 1: Custom inline prompt
    if let Some(ref prompt) = config.custom_prompt {
        return Ok(prompt.clone());
    }

    // Priority 2: Custom prompt file
    if let Some(ref prompt_file) = config.system_prompt_file {
        let config_dir = get_config_dir();
        let path = if Path::new(prompt_file).is_absolute() {
            PathBuf::from(prompt_file)
        } else {
            config_dir.join(prompt_file)
        };

        let mut prompt = fs::read_to_string(&path)
            .with_context(|| format!("System prompt file not found: {:?}", path))?;

        prompt = prompt.replace("{METADATA_FIELDS}", &config.metadata_fields);
        return Ok(prompt);
    }

    // Priority 3: Built-in prompts
    let prompts_dir = get_prompts_dir();

    // If solve mode, use solve prompt
    if mode == Mode::Solve {
        let solve_prompt_path = prompts_dir.join("solve_prompt.txt");
        let mut prompt = fs::read_to_string(&solve_prompt_path)
            .with_context(|| format!("Solve prompt file not found at {:?}. Please ensure kmarkdownify is properly installed.", solve_prompt_path))?;
        prompt = prompt.replace("{METADATA_FIELDS}", &config.metadata_fields);
        Ok(prompt)
    } else if config.extract_metadata {
        let metadata_prompt_path = prompts_dir.join("metadata_prompt.txt");
        let mut prompt = fs::read_to_string(&metadata_prompt_path)
            .with_context(|| format!("Metadata prompt file not found at {:?}. Please ensure kmarkdownify is properly installed.", metadata_prompt_path))?;
        prompt = prompt.replace("{METADATA_FIELDS}", &config.metadata_fields);
        Ok(prompt)
    } else {
        let default_prompt_path = prompts_dir.join("default_prompt.txt");
        fs::read_to_string(&default_prompt_path)
            .with_context(|| format!("Default prompt file not found at {:?}. Please ensure kmarkdownify is properly installed.", default_prompt_path))
    }
}

fn verify_pdf(path: &Path) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("File not found: {:?}", path);
    }

    let output = Command::new("file")
        .arg(path)
        .output()
        .context("Failed to execute 'file' command")?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    if !output_str.contains("PDF") {
        anyhow::bail!("File is not a PDF: {:?}", path);
    }

    Ok(())
}

fn check_dependencies() -> Result<()> {
    for cmd in &["curl", "file"] {
        Command::new("which").arg(cmd).output().with_context(|| {
            format!(
                "Required command not found: {}. Please install it first.",
                cmd
            )
        })?;
    }
    Ok(())
}

fn count_pdf_pages(pdf_path: &Path) -> Result<u32> {
    // Method 1: Try pdfinfo (from poppler-utils)
    if let Ok(output) = Command::new("pdfinfo").arg(pdf_path).output() {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.starts_with("Pages:") {
                    if let Some(count_str) = line.split_whitespace().nth(1) {
                        if let Ok(count) = count_str.parse::<u32>() {
                            if count > 0 {
                                return Ok(count);
                            }
                        }
                    }
                }
            }
        }
    }

    // Method 2: Try qpdf
    if let Ok(output) = Command::new("qpdf")
        .args(["--show-npages", &pdf_path.to_string_lossy()])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Ok(count) = stdout.parse::<u32>() {
                if count > 0 {
                    return Ok(count);
                }
            }
        }
    }

    // Method 3: Try gs (Ghostscript)
    if let Ok(output) = Command::new("gs")
        .args([
            "-q",
            "-dNODISPLAY",
            "-c",
            &format!(
                "({}) (r) file runpdfbegin pdfpagecount = quit",
                pdf_path.display()
            ),
        ])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Ok(count) = stdout.parse::<u32>() {
                if count > 0 {
                    return Ok(count);
                }
            }
        }
    }

    // Fallback: Return 1 page with warning
    eprintln!("Warning: Could not determine PDF page count. Using 1 page for token calculation.");
    eprintln!("Install 'poppler-utils' (pdfinfo) for accurate page counting.");
    Ok(1)
}

fn check_overwrite(output_path: &Path) -> Result<bool> {
    if !output_path.exists() {
        return Ok(true);
    }

    // Try to use kdialog for confirmation
    let output = Command::new("kdialog")
        .arg("--yesno")
        .arg(format!(
            "Output file already exists: {:?}\n\nDo you want to overwrite it?",
            output_path
        ))
        .arg("--title")
        .arg("KMarkdownify")
        .output();

    match output {
        Ok(result) => Ok(result.status.success()),
        Err(_) => {
            // kdialog not available, just proceed
            Ok(true)
        }
    }
}

async fn convert_pdf_streaming(
    pdf_path: &Path,
    output_path: &Path,
    config: &Config,
    mode: Mode,
    notif: Arc<Mutex<NotificationManager>>,
) -> Result<()> {
    let action = match mode {
        Mode::Convert => "Converting",
        Mode::Solve => "Solving",
    };

    // Count PDF pages
    {
        let notif_guard = notif.lock().unwrap();
        notif_guard.update("Analyzing PDF file...");
    }
    let page_count = count_pdf_pages(pdf_path)?;
    let max_tokens = page_count * config.max_tokens_per_page;
    println!(
        "PDF has {} page(s). Using {} max tokens.",
        page_count, max_tokens
    );

    {
        let notif_guard = notif.lock().unwrap();
        notif_guard.update(&format!(
            "Encoding PDF file for {}...",
            action.to_lowercase()
        ));
    }

    // Read and encode PDF
    let pdf_data =
        fs::read(pdf_path).with_context(|| format!("Failed to read PDF file: {:?}", pdf_path))?;
    let pdf_base64 = STANDARD.encode(&pdf_data);

    // Get prompt
    {
        let notif_guard = notif.lock().unwrap();
        notif_guard.update("Preparing API request...");
    }
    let prompt_text = get_prompt_text(config, mode)?;

    // Select appropriate model based on mode
    let selected_model = match mode {
        Mode::Convert => config.ocr_model.as_ref().unwrap_or(&config.model),
        Mode::Solve => config.reasoning_model.as_ref().unwrap_or(&config.model),
    };

    println!("Using model: {}", selected_model);

    // Build request
    let pdf_filename = pdf_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("document.pdf")
        .to_string();

    let request = ApiRequest {
        model: selected_model.clone(),
        temperature: config.temperature,
        max_tokens,
        stream: true,
        messages: vec![Message {
            role: "user".to_string(),
            content: vec![
                ContentItem::Text { text: prompt_text },
                ContentItem::File {
                    file: FileData {
                        filename: pdf_filename,
                        file_data: format!("data:application/pdf;base64,{}", pdf_base64),
                    },
                },
            ],
        }],
    };

    // Make API request
    {
        let notif_guard = notif.lock().unwrap();
        notif_guard.update("Sending request to OpenRouter API...");
    }

    // Calculate timeout based on page count
    let timeout_seconds = page_count as u64 * config.timeout_per_page;
    let timeout = Duration::from_secs(timeout_seconds);
    println!(
        "Using timeout of {} seconds ({} seconds per page × {} pages)",
        timeout_seconds, config.timeout_per_page, page_count
    );

    let client = Client::builder()
        .timeout(timeout)
        .build()
        .context("Failed to build HTTP client")?;

    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header(
            "HTTP-Referer",
            "https://github.com/profiluefter/KMarkdownify",
        )
        .header("X-Title", "KMarkdownify")
        .json(&request)
        .send()
        .await
        .context("Failed to send request to OpenRouter API")?;

    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        anyhow::bail!("API request failed: {}", error_text);
    }

    // Process streaming response
    {
        let notif_guard = notif.lock().unwrap();
        notif_guard.update("Receiving and saving streamed response...");
    }

    let mut file = File::create(output_path)
        .with_context(|| format!("Failed to create output file: {:?}", output_path))?;

    let mut stream = response.bytes_stream().eventsource();
    let mut content_buffer = String::new();
    let mut reasoning_buffer = String::new();
    let mut last_update = std::time::Instant::now();

    while let Some(event_result) = stream.next().await {
        match event_result {
            Ok(event) => {
                if event.data == "[DONE]" {
                    break;
                }

                match serde_json::from_str::<StreamResponse>(&event.data) {
                    Ok(stream_resp) => {
                        // Check for errors
                        if let Some(error) = stream_resp.error {
                            anyhow::bail!("API Error: {}", error.message);
                        }

                        // Process choices
                        if let Some(choices) = stream_resp.choices {
                            for choice in choices {
                                // Handle regular content
                                if let Some(content) = choice.delta.content {
                                    content_buffer.push_str(&content);
                                    file.write_all(content.as_bytes())
                                        .context("Failed to write to output file")?;
                                    file.flush().context("Failed to flush output file")?;
                                }

                                // Handle reasoning content
                                if let Some(reasoning) = choice.delta.reasoning_content {
                                    reasoning_buffer.push_str(&reasoning);
                                }

                                // Update notification periodically (every 2 seconds)
                                if last_update.elapsed() > Duration::from_secs(2) {
                                    let words_written = content_buffer.split_whitespace().count();
                                    let notif_guard = notif.lock().unwrap();

                                    // Extract a brief summary from reasoning for notification
                                    let reasoning_summary = if !reasoning_buffer.is_empty() {
                                        // Get the last line or first 50 chars of reasoning
                                        reasoning_buffer
                                            .lines()
                                            .last()
                                            .unwrap_or(&reasoning_buffer)
                                            .chars()
                                            .take(50)
                                            .collect::<String>()
                                    } else {
                                        String::new()
                                    };

                                    notif_guard.update_with_reasoning(
                                        &format!("Streaming... ({} words written)", words_written),
                                        &reasoning_summary,
                                    );
                                    last_update = std::time::Instant::now();
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to parse stream event: {} (data: {})",
                            e, event.data
                        );
                        // Continue processing other events
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: Stream error: {}", e);
                // Try to preserve partial output
                if !content_buffer.is_empty() {
                    eprintln!("Partial content was saved to the output file.");
                }
                anyhow::bail!("Stream error: {}", e);
            }
        }
    }

    if content_buffer.is_empty() {
        anyhow::bail!("API returned empty content");
    }

    // Final flush
    file.flush().context("Failed to flush output file")?;

    let words_written = content_buffer.split_whitespace().count();
    println!(
        "✓ Streaming complete! {} words written to {:?}",
        words_written, output_path
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Check dependencies
    check_dependencies()?;

    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("No PDF file provided. Usage: kmarkdownify [--solve] <pdf-file>");
    }

    // Check for mode flag
    let (mode, pdf_arg_index) = if args.len() >= 3 && args[1] == "--solve" {
        (Mode::Solve, 2)
    } else if args[1] == "--solve" {
        anyhow::bail!("No PDF file provided. Usage: kmarkdownify [--solve] <pdf-file>");
    } else {
        (Mode::Convert, 1)
    };

    let pdf_path = Path::new(&args[pdf_arg_index]);

    // Verify PDF
    verify_pdf(pdf_path)?;

    // Load configuration
    let config = load_config()?;

    // Generate output filename with appropriate suffix
    let output_path = if mode == Mode::Solve {
        let stem = pdf_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        pdf_path.with_file_name(format!("{}_solved.md", stem))
    } else {
        pdf_path.with_extension("md")
    };

    // Check if output exists and get user confirmation
    if !check_overwrite(&output_path)? {
        println!("Operation cancelled by user.");
        return Ok(());
    }

    // Initialize notification manager
    let mut notif = NotificationManager::new();
    let action_title = match mode {
        Mode::Convert => "Converting PDF to Markdown",
        Mode::Solve => "Solving PDF Assignments",
    };
    notif.show_loading(
        "KMarkdownify",
        &format!("{}...\nThis may take a moment.", action_title),
    );

    let notif_arc = Arc::new(Mutex::new(notif));
    let notif_clone = Arc::clone(&notif_arc);

    // Convert/Solve PDF with streaming
    match convert_pdf_streaming(pdf_path, &output_path, &config, mode, notif_clone).await {
        Ok(()) => {
            let mut notif_guard = notif_arc.lock().unwrap();
            let success_msg = match mode {
                Mode::Convert => format!("✓ Conversion complete!\n\nSaved to: {:?}", output_path),
                Mode::Solve => format!("✓ Solutions complete!\n\nSaved to: {:?}", output_path),
            };
            notif_guard.success(&success_msg);
            // Prevent drop handler from showing error
            notif_guard.id = None;
            Ok(())
        }
        Err(e) => {
            let mut notif_guard = notif_arc.lock().unwrap();
            notif_guard.error(&format!("{:#}", e));
            Err(e)
        }
    }
}
