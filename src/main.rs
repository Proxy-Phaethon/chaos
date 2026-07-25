use clap::{Parser, Subcommand};
use dialoguer::{Select, Confirm, Input, theme::ColorfulTheme};
use std::fs;
use std::process::Command;

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

    let project_types = vec!["Static Webpage", "Basic Webapp"];
    let selection = Select::with_theme(&theme)
        .with_prompt("What are you building?")
        .items(&project_types)
        .default(0)
        .interact()
        .unwrap();
    let project_type = project_types[selection].to_string();

    let styling_options = vec!["Tailwind CSS", "Plain CSS"];
    let selection = Select::with_theme(&theme)
        .with_prompt("Choose your styling approach")
        .items(&styling_options)
        .default(0)
        .interact()
        .unwrap();
    let styling = styling_options[selection].to_string();

    let include_js = Confirm::with_theme(&theme)
        .with_prompt("Include JavaScript?")
        .default(true)
        .interact()
        .unwrap();

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
        generate_basic_webapp(&config);
    }
}

// ---------- Shared content helpers ----------

fn readme_content(config: &ProjectConfig) -> String {
    format!("# {}\n\nGenerated with Chaos.\n", config.project_name)
}

// ---------- Frontend feature contributors ----------

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

fn generate_frontend_plan(config: &ProjectConfig) -> BuildPlan {
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

    plan
}

// ---------- Static Webpage ----------

fn generate_static_webpage(config: &ProjectConfig) {
    let mut plan = generate_frontend_plan(config);
    plan.files.push(("README.md".to_string(), readme_content(config)));

    execute_plan(&config.project_name, &plan);

    if config.install_dependencies && !plan.npm_packages.is_empty() {
        run_npm_install(&config.project_name, &plan.npm_packages);
    }
}

// ---------- Basic Webapp ----------

fn generate_basic_webapp(config: &ProjectConfig) {
    fs::create_dir_all(&config.project_name).expect("Failed to create project folder");

    let docs_folder = format!("{}/docs", config.project_name);
    fs::create_dir_all(&docs_folder).expect("Failed to create docs folder");
    fs::write(format!("{}/.gitkeep", docs_folder), "").expect("Failed to create docs placeholder");

    // Frontend
    let frontend_plan = generate_frontend_plan(config);
    let frontend_folder = format!("{}/frontend", config.project_name);
    execute_plan(&frontend_folder, &frontend_plan);

    if config.install_dependencies && !frontend_plan.npm_packages.is_empty() {
        run_npm_install(&frontend_folder, &frontend_plan.npm_packages);
    }

    let mut root_gitignore_entries: Vec<String> = frontend_plan
        .gitignore_entries
        .iter()
        .map(|entry| format!("frontend/{}", entry))
        .collect();

    // Backend
    if config.install_dependencies {
        let backend_entries = generate_backend(config);
        root_gitignore_entries.extend(backend_entries);
    } else {
        fs::create_dir_all(format!("{}/backend", config.project_name))
            .expect("Failed to create backend folder");
        println!(
            "Backend wasn't scaffolded — Django/Express both require installing \
             the framework before their project files can be generated. Run \
             'chaos initialize' again with dependency installation enabled to \
             build a real backend."
        );
    }

    // Root README + .gitignore
    fs::write(
        format!("{}/README.md", config.project_name),
        readme_content(config),
    )
    .expect("Failed to write README");
    println!("Created: {}/README.md", config.project_name);

    if !root_gitignore_entries.is_empty() {
        let content = root_gitignore_entries.join("\n");
        fs::write(format!("{}/.gitignore", config.project_name), content)
            .expect("Failed to write .gitignore");
        println!("Created: {}/.gitignore", config.project_name);
    }
}

fn generate_backend(config: &ProjectConfig) -> Vec<String> {
    let backend_folder = format!("{}/backend", config.project_name);
    fs::create_dir_all(&backend_folder).expect("Failed to create backend folder");

    let mut gitignore_entries = Vec::new();

    match config.backend_language.as_deref() {
        Some("Python") => {
            gitignore_entries.push("backend/venv/".to_string());
            gitignore_entries.push("backend/__pycache__/".to_string());

            println!("\n Setting up Python backend...");

            let status = Command::new("python3")
                .arg("-m")
                .arg("venv")
                .arg("venv")
                .current_dir(&backend_folder)
                .status()
                .expect("Failed to create virtual environment — is python3 installed?");

            if !status.success() {
                println!("Failed to create virtual environment");
                return gitignore_entries;
            }
            println!("Created virtual environment");

            let pip_path = format!("{}/venv/bin/pip", backend_folder);
            let status = Command::new(&pip_path)
                .arg("install")
                .arg("django")
                .current_dir(&backend_folder)
                .status()
                .expect("Failed to run pip install");

            if !status.success() {
                println!("Failed to install Django");
                return gitignore_entries;
            }
            println!("Installed Django");

            let django_admin_path = format!("{}/venv/bin/django-admin", backend_folder);
            let status = Command::new(&django_admin_path)
                .arg("startproject")
                .arg("backend")
                .arg(".")
                .current_dir(&backend_folder)
                .status()
                .expect("Failed to run django-admin startproject");

            if status.success() {
                println!("Generated Django project");
            } else {
                println!("django-admin startproject failed");
            }
        }
        Some("TypeScript") => {
            gitignore_entries.push("backend/node_modules/".to_string());

            println!("\n🟦 Setting up TypeScript backend...");

            let status = Command::new("npx")
                .arg("--yes")
                .arg("express-generator")
                .arg("--no-view")
                .current_dir(&backend_folder)
                .status()
                .expect("Failed to run express-generator — is npm/npx installed?");

            if !status.success() {
                println!("express-generator failed");
                return gitignore_entries;
            }
            println!("Generated Express project");

            let status = Command::new("npm")
                .arg("install")
                .current_dir(&backend_folder)
                .status()
                .expect("Failed to run npm install");

            if status.success() {
                println!("Installed backend dependencies");
            } else {
                println!("npm install failed");
            }
        }
        _ => {}
    }

    gitignore_entries
}

// ---------- Shared execution ----------

fn execute_plan(folder: &str, plan: &BuildPlan) {
    fs::create_dir_all(folder).expect("Failed to create project folder");

    for (relative_path, content) in &plan.files {
        let full_path = format!("{}/{}", folder, relative_path);

        if let Some(parent) = std::path::Path::new(&full_path).parent() {
            fs::create_dir_all(parent).expect("Failed to create subfolder");
        }

        fs::write(&full_path, content).expect("Failed to write file");
        println!(" Created: {}", full_path);
    }

    if !plan.gitignore_entries.is_empty() {
        let gitignore_content = plan.gitignore_entries.join("\n");
        let gitignore_path = format!("{}/.gitignore", folder);
        fs::write(&gitignore_path, gitignore_content).expect("Failed to write .gitignore");
        println!(" Created: {}", gitignore_path);
    }
}

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

    let status = cmd
        .status()
        .expect("Failed to run npm install — is npm installed?");

    if status.success() {
        println!("Dependencies installed successfully");
    } else {
        println!("npm install failed — you can run it manually inside the project folder");
    }
}