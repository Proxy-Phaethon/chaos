use clap::{Parser, Subcommand};
use dialoguer::{Select, Confirm, Input, theme::ColorfulTheme};
use std::fs;

#[derive(Parser)]
#[command(name = "chaos")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Initialize,
    Write,
    End,
    Run,
}

struct ProjectConfig {
    project_name: String,
    project_type: String,
    styling: String,
    include_js: bool,
    backend_language: Option<String>,
    backend_framework: Option<String>,
    install_dependencies: bool,
}

struct BuildPlan {
    files: Vec<(String, String)>,
    gitignore_entries: Vec<String>,
    npm_packages: Vec<String>,
}

impl BuildPlan {
    fn new() -> Self {
        BuildPlan {
            files: Vec::new(),
            gitignore_entries: Vec::new(),
            npm_packages: Vec::new(),
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Initialize => run_initialize(),
        Commands::Write => println!("Running: chaos write"),
        Commands::End => println!("Running: chaos end"),
        Commands::Run => println!("Running: chaos run"),
    }
}

fn sanitize_project_name(name: &str) -> String {
    let mut result = String::new();
    let mut last_was_separator = false;

    for c in name.trim().chars() {
        if c.is_alphanumeric() || c == '-' || c == '_' {
            result.push(c);
            last_was_separator = false;
        } else if c.is_whitespace() {
            if !last_was_separator && !result.is_empty() {
                result.push('-');
                last_was_separator = true;
            }
        }
    }

    result.trim_end_matches(['-', '_']).to_string()
}

fn run_initialize() {
    let theme = ColorfulTheme::default();

    let raw_name: String = Input::with_theme(&theme)
        .with_prompt("What's your project called?")
        .interact_text()
        .unwrap();

    let project_name = sanitize_project_name(&raw_name);
    println!("Project folder will be named: {}", project_name);

    // Step 1
    let project_types = vec!["Static Webpage", "Basic Webapp"];
    let selection = Select::with_theme(&theme)
        .with_prompt("What are you building?")
        .items(&project_types)
        .default(0)
        .interact()
        .unwrap();
    let project_type = project_types[selection].to_string();

    // Step 2
    let styling_options = vec!["Tailwind CSS", "Plain CSS"];
    let selection = Select::with_theme(&theme)
        .with_prompt("Choose your styling approach")
        .items(&styling_options)
        .default(0)
        .interact()
        .unwrap();
    let styling = styling_options[selection].to_string();

    // Step 3
    let include_js = Confirm::with_theme(&theme)
        .with_prompt("Include JavaScript?")
        .default(true)
        .interact()
        .unwrap();

    // Step 4 & 5 — only if Basic Webapp
    let mut backend_language: Option<String> = None;
    let mut backend_framework: Option<String> = None;

    if project_type == "Basic Webapp" {
        let backend_options = vec!["Python", "TypeScript"];
        let selection = Select::with_theme(&theme)
            .with_prompt("Choose your backend language")
            .items(&backend_options)
            .default(0)
            .interact()
            .unwrap();
        let language = backend_options[selection].to_string();

        let frameworks: Vec<&str> = match language.as_str() {
            "Python" => vec!["Django"],
            "TypeScript" => vec!["Express"],
            _ => vec![],
        };

        let selection = Select::with_theme(&theme)
            .with_prompt("Choose your backend framework")
            .items(&frameworks)
            .default(0)
            .interact()
            .unwrap();
        let framework = frameworks[selection].to_string();

        backend_language = Some(language);
        backend_framework = Some(framework);
    }

    // Step 6
    let install_dependencies = Confirm::with_theme(&theme)
        .with_prompt("Install dependencies now?")
        .default(true)
        .interact()
        .unwrap();

    let config = ProjectConfig {
        project_name,
        project_type,
        styling,
        include_js,
        backend_language,
        backend_framework,
        install_dependencies,
    };

    println!("\n--- Config Summary ---");
    println!("Project name: {}", config.project_name);
    println!("Project type: {}", config.project_type);
    println!("Styling: {}", config.styling);
    println!("Include JS: {}", config.include_js);
    println!("Backend language: {:?}", config.backend_language);
    println!("Backend framework: {:?}", config.backend_framework);
    println!("Install dependencies: {}", config.install_dependencies);
    println!();

    if config.project_type == "Static Webpage" {
        generate_static_webpage(&config);
    } else {
        println!("Basic Webapp generation isn't built yet — coming soon.");
    }
}

fn add_html_boilerplate(plan: &mut BuildPlan, config: &ProjectConfig) {
    let css_link = if config.styling == "Tailwind CSS" {
        "<link rel=\"stylesheet\" href=\"dist/output.css\">"
    } else {
        "<link rel=\"stylesheet\" href=\"styles/style.css\">"
    };

    let script_tag = if config.include_js {
        "<script src=\"script.js\"></script>"
    } else {
        ""
    };

    let html = format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <title>{}</title>\n    {}\n</head>\n<body>\n    <h1>Welcome to {}</h1>\n    {}\n</body>\n</html>",
        config.project_name, css_link, config.project_name, script_tag
    );

    plan.files.push(("index.html".to_string(), html));
}

fn add_tailwind(plan: &mut BuildPlan, config: &ProjectConfig) {
    let package_json = format!(
        "{{\n  \"name\": \"{}\",\n  \"scripts\": {{ \"build\": \"tailwindcss -i src/input.css -o dist/output.css\" }}\n}}",
        config.project_name
    );
    plan.files.push(("package.json".to_string(), package_json));

    plan.files.push((
        "tailwind.config.js".to_string(),
        "module.exports = {\n  content: [\"./index.html\"],\n  theme: { extend: {} },\n  plugins: [],\n};".to_string(),
    ));
    plan.files.push((
        "src/input.css".to_string(),
        "@tailwind base;\n@tailwind components;\n@tailwind utilities;".to_string(),
    ));

    plan.gitignore_entries.push("node_modules/".to_string());
    plan.gitignore_entries.push("dist/".to_string());

    plan.npm_packages.push("tailwindcss".to_string());
}

fn add_plain_css(plan: &mut BuildPlan) {
    plan.files.push((
        "styles/style.css".to_string(),
        "/* Your styles here */".to_string(),
    ));
}

fn add_javascript(plan: &mut BuildPlan) {
    plan.files.push((
        "script.js".to_string(),
        "// Your JavaScript here".to_string(),
    ));
}

fn generate_static_webpage(config: &ProjectConfig) {
    let mut plan = BuildPlan::new();

    add_html_boilerplate(&mut plan, config);

    if config.styling == "Tailwind CSS" {
        add_tailwind(&mut plan, config);
    } else {
        add_plain_css(&mut plan);
    }

    if config.include_js {
        add_javascript(&mut plan);
    }

    execute_plan(&config.project_name, &plan);

    if config.install_dependencies && !plan.npm_packages.is_empty() {
        run_npm_install(&config.project_name, &plan.npm_packages);
    }
}

fn execute_plan(folder: &str, plan: &BuildPlan) {
    fs::create_dir_all(folder).expect("Failed to create project folder");

    for (relative_path, content) in &plan.files {
        let full_path = format!("{}/{}", folder, relative_path);

        if let Some(parent) = std::path::Path::new(&full_path).parent() {
            fs::create_dir_all(parent).expect("Failed to create subfolder");
        }

        fs::write(&full_path, content).expect("Failed to write file");
        println!("Created: {}", full_path);
    }

    if !plan.gitignore_entries.is_empty() {
        let gitignore_content = plan.gitignore_entries.join("\n");
        let gitignore_path = format!("{}/.gitignore", folder);
        fs::write(&gitignore_path, gitignore_content).expect("Failed to write .gitignore");
        println!("Created: {}", gitignore_path);
    }
}

use std::process::Command;

fn run_npm_install(folder: &str, packages: &[String]) {
    if packages.is_empty() {
        return;
    }

    println!("\n Installing dependencies...");

    let mut cmd = Command::new("npm");
    cmd.arg("install").arg("-D");

    for pkg in packages {
        cmd.arg(pkg);
    }

    cmd.current_dir(folder);

    let status = cmd.status().expect("Failed to run npm install — is npm installed?");

    if status.success() {
        println!("Dependencies installed successfully");
    } else {
        println!("npm install failed — you can run it manually inside the project folder");
    }
}