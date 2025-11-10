use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use notify_rust::{Notification, Timeout};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// Configuration structure
#[derive(Debug, Default)]
struct Config {
    api_key: String,
    model: String,
    temperature: f32,
    max_tokens: u32,
    extract_metadata: bool,
    metadata_fields: String,
    custom_prompt: Option<String>,
    system_prompt_file: Option<String>,
}

// API request structures
#[derive(Serialize)]
struct ApiRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
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
#[derive(Deserialize)]
struct ApiResponse {
    choices: Option<Vec<Choice>>,
    error: Option<ApiError>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

#[derive(Deserialize)]
struct ApiError {
    message: String,
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
        temperature: 0.1,
        max_tokens: 8000,
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
                    "TEMPERATURE" => config.temperature = value.parse().unwrap_or(0.1),
                    "MAX_TOKENS" => config.max_tokens = value.parse().unwrap_or(8000),
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

fn get_prompt_text(config: &Config) -> Result<String> {
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

    if config.extract_metadata {
        let metadata_prompt_path = prompts_dir.join("metadata_prompt.txt");
        if metadata_prompt_path.exists() {
            let mut prompt = fs::read_to_string(&metadata_prompt_path)?;
            prompt = prompt.replace("{METADATA_FIELDS}", &config.metadata_fields);
            return Ok(prompt);
        }

        // Fallback inline metadata prompt
        Ok(format!(
            "Please analyze this PDF document and extract both metadata and content.\n\
            \n\
            First, extract the following metadata fields if present in the document: {}. If a field cannot be found, use 'N/A'.\n\
            \n\
            Then, extract all text content from the PDF and convert it to clean, well-formatted Markdown.\n\
            \n\
            IMPORTANT: Transcribe the content EXACTLY as it appears in the document, including any typos, spelling errors, or grammatical mistakes. Do not correct or modify the actual text content.\n\
            \n\
            You may improve the formatting by:\n\
            - Using appropriate Markdown syntax (headings, lists, tables, etc.)\n\
            - Adding code blocks for code snippets\n\
            - Using emphasis (bold, italic) for highlighted text\n\
            - Preserving document structure\n\
            \n\
            Format your response as follows:\n\
            1. Start with YAML frontmatter containing the metadata (enclosed in --- delimiters)\n\
            2. Follow with the main content in Markdown format\n\
            \n\
            Only return the formatted output without any explanations or additional commentary.\n\
            \n\
            Example format:\n\
            ---\n\
            Title: Document Title or N/A\n\
            Author: Author Name or N/A\n\
            Course: Course Name or N/A\n\
            Due Date: Date or N/A\n\
            ---\n\
            \n\
            # Document Content Starts Here\n\
            ....",
            config.metadata_fields
        ))
    } else {
        let default_prompt_path = prompts_dir.join("default_prompt.txt");
        if default_prompt_path.exists() {
            return fs::read_to_string(&default_prompt_path)
                .context("Failed to read default prompt file");
        }

        // Fallback inline default prompt
        Ok("Please extract all text content from this PDF document and convert it to clean, well-formatted Markdown.\n\
            \n\
            IMPORTANT: Transcribe the content EXACTLY as it appears in the document, including any typos, spelling errors, or grammatical mistakes. Do not correct or modify the actual text content.\n\
            \n\
            You may improve the formatting by:\n\
            - Using appropriate Markdown syntax (headings, lists, tables, etc.)\n\
            - Adding code blocks for code snippets\n\
            - Using emphasis (bold, italic) for highlighted text\n\
            - Preserving document structure\n\
            \n\
            Only return the Markdown text without any explanations or additional commentary.".to_string())
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

fn convert_pdf(pdf_path: &Path, config: &Config, notif: &NotificationManager) -> Result<String> {
    notif.update("Encoding PDF file...");

    // Read and encode PDF
    let pdf_data =
        fs::read(pdf_path).with_context(|| format!("Failed to read PDF file: {:?}", pdf_path))?;
    let pdf_base64 = STANDARD.encode(&pdf_data);

    // Get prompt
    notif.update("Preparing API request...");
    let prompt_text = get_prompt_text(config)?;

    // Build request
    let pdf_filename = pdf_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("document.pdf")
        .to_string();

    let request = ApiRequest {
        model: config.model.clone(),
        temperature: config.temperature,
        max_tokens: config.max_tokens,
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
    notif.update("Sending request to OpenRouter API...");

    let client = Client::new();
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
        .context("Failed to send request to OpenRouter API")?;

    let api_response: ApiResponse = response.json().context("Failed to parse API response")?;

    // Check for errors
    if let Some(error) = api_response.error {
        anyhow::bail!("API Error: {}", error.message);
    }

    // Extract content
    notif.update("Extracting markdown content...");

    let content = api_response
        .choices
        .and_then(|choices| choices.into_iter().next())
        .map(|choice| choice.message.content)
        .ok_or_else(|| anyhow::anyhow!("Failed to extract content from API response. The response may be empty or malformed."))?;

    if content.is_empty() {
        anyhow::bail!("API returned empty content");
    }

    Ok(content)
}

fn main() -> Result<()> {
    // Check dependencies
    check_dependencies()?;

    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("No PDF file provided. Usage: kmarkdownify <pdf-file>");
    }

    let pdf_path = Path::new(&args[1]);

    // Verify PDF
    verify_pdf(pdf_path)?;

    // Load configuration
    let config = load_config()?;

    // Generate output filename
    let output_path = pdf_path.with_extension("md");

    // Check if output exists and get user confirmation
    if !check_overwrite(&output_path)? {
        println!("Conversion cancelled by user.");
        return Ok(());
    }

    // Initialize notification manager
    let mut notif = NotificationManager::new();
    notif.show_loading(
        "KMarkdownify",
        "Converting PDF to Markdown...\nThis may take a moment.",
    );

    // Convert PDF
    match convert_pdf(pdf_path, &config, &notif) {
        Ok(markdown_content) => {
            notif.update(&format!("Saving to file: {:?}", output_path));
            fs::write(&output_path, markdown_content)
                .with_context(|| format!("Failed to write output file: {:?}", output_path))?;

            notif.success(&format!(
                "✓ Conversion complete!\n\nSaved to: {:?}",
                output_path
            ));
            Ok(())
        }
        Err(e) => {
            notif.error(&format!("{:#}", e));
            Err(e)
        }
    }
}
