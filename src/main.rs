use clap::{Parser, Subcommand};
use dialoguer::{Select, Confirm, Input, theme::ColorfulTheme};
use std::fs;
use std::path::Path;
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

// ---------- Shared guardrail ----------

fn require_tool(command: &str, install_hint: &str) -> bool {
    let check = Command::new(command).arg("--version").output();

    match check {
        Ok(_) => true,
        Err(_) => {
            println!("\n '{}' isn't installed or isn't on your PATH.", command);
            println!("   {}", install_hint);
            false
        }
    }
}

// ---------- Question flow ----------

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

    let styling_options = vec!["Tailwind CSS", "Plain CSS", "Bootstrap", "Sass/SCSS"];
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
        let backend_options = vec!["Python", "TypeScript", "Ruby", "PHP", "Go"];
        let selection = Select::with_theme(&theme)
            .with_prompt("Choose your backend language")
            .items(&backend_options)
            .default(0)
            .interact()
            .unwrap();
        let language = backend_options[selection].to_string();

        let frameworks: Vec<&str> = match language.as_str() {
            "Python" => vec!["Django", "Flask"],
            "TypeScript" => vec!["Express", "Fastify", "NestJS"],
            "Ruby" => vec!["Rails"],
            "PHP" => vec!["Laravel"],
            "Go" => vec!["Gin"],
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
    let css_link = match config.styling.as_str() {
        "Tailwind CSS" => "<link rel=\"stylesheet\" href=\"dist/output.css\">".to_string(),
        "Sass/SCSS" => "<link rel=\"stylesheet\" href=\"dist/output.css\">".to_string(),
        "Bootstrap" => {
            "<link rel=\"stylesheet\" href=\"node_modules/bootstrap/dist/css/bootstrap.min.css\">"
                .to_string()
        }
        _ => "<link rel=\"stylesheet\" href=\"styles/style.css\">".to_string(), // Plain CSS
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

fn add_bootstrap(plan: &mut BuildPlan) {
    // Bootstrap ships pre-compiled CSS — no build step needed, just install and link directly.
    plan.npm_packages.push("bootstrap".to_string());
    plan.gitignore_entries.push("node_modules/".to_string());
}

fn add_sass(plan: &mut BuildPlan, config: &ProjectConfig) {
    let package_json = format!(
        "{{\n  \"name\": \"{}\",\n  \"scripts\": {{ \"build\": \"sass src/input.scss dist/output.css\" }}\n}}",
        config.project_name
    );
    plan.files.push(("package.json".to_string(), package_json));
    plan.files.push((
        "src/input.scss".to_string(),
        "// Your Sass/SCSS here\nbody {\n  font-family: sans-serif;\n}".to_string(),
    ));

    plan.gitignore_entries.push("node_modules/".to_string());
    plan.gitignore_entries.push("dist/".to_string());
    plan.npm_packages.push("sass".to_string());
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

    match config.styling.as_str() {
        "Tailwind CSS" => add_tailwind(&mut plan, config),
        "Bootstrap" => add_bootstrap(&mut plan),
        "Sass/SCSS" => add_sass(&mut plan, config),
        _ => add_plain_css(&mut plan), // Plain CSS
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
            "Backend wasn't scaffolded — every supported backend requires installing \
             the framework/toolchain before its project files can be generated. Run \
             'chaos initialize' again with dependency installation enabled to build a \
             real backend."
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

// ---------- Backend dispatcher ----------

fn generate_backend(config: &ProjectConfig) -> Vec<String> {
    let backend_folder = format!("{}/backend", config.project_name);
    fs::create_dir_all(&backend_folder).expect("Failed to create backend folder");

    let backend_folder_abs =
        fs::canonicalize(&backend_folder).expect("Failed to resolve backend folder path");

    let language = config.backend_language.as_deref();
    let framework = config.backend_framework.as_deref();

    println!(
        "\n🔧 Setting up {} backend ({})...",
        language.unwrap_or("?"),
        framework.unwrap_or("?")
    );

    match (language, framework) {
        (Some("Python"), Some("Django")) => {
            generate_django_backend(&backend_folder_abs);
            vec!["backend/venv/".to_string(), "backend/__pycache__/".to_string()]
        }
        (Some("Python"), Some("Flask")) => {
            generate_flask_backend(&backend_folder_abs);
            vec!["backend/venv/".to_string(), "backend/__pycache__/".to_string()]
        }
        (Some("TypeScript"), Some("Express")) => {
            generate_express_backend(&backend_folder_abs);
            vec!["backend/node_modules/".to_string()]
        }
        (Some("TypeScript"), Some("Fastify")) => {
            generate_fastify_backend(&backend_folder_abs);
            vec!["backend/node_modules/".to_string()]
        }
        (Some("TypeScript"), Some("NestJS")) => {
            generate_nestjs_backend(&backend_folder_abs);
            vec!["backend/node_modules/".to_string(), "backend/dist/".to_string()]
        }
        (Some("Ruby"), Some("Rails")) => {
            generate_rails_backend(&backend_folder_abs);
            vec!["backend/log/".to_string(), "backend/tmp/".to_string()]
        }
        (Some("PHP"), Some("Laravel")) => {
            generate_laravel_backend(&backend_folder_abs);
            vec!["backend/vendor/".to_string(), "backend/.env".to_string()]
        }
        (Some("Go"), Some("Gin")) => {
            generate_go_backend(&backend_folder_abs, &config.project_name);
            vec![]
        }
        _ => {
            println!("Unknown backend combination — skipping.");
            vec![]
        }
    }
}

// ---------- Individual backend generators ----------

fn generate_django_backend(backend_folder_abs: &Path) -> bool {
    if !require_tool("python3", "Install Python from https://python.org") {
        return false;
    }

    let status = Command::new("python3")
        .arg("-m")
        .arg("venv")
        .arg("venv")
        .current_dir(backend_folder_abs)
        .status()
        .expect("Failed to create virtual environment");

    if !status.success() {
        println!("Failed to create virtual environment");
        return false;
    }
    println!("Created virtual environment");

    let pip_path = backend_folder_abs.join("venv/bin/pip");
    let status = Command::new(&pip_path)
        .arg("install")
        .arg("django")
        .current_dir(backend_folder_abs)
        .status()
        .expect("Failed to run pip install");

    if !status.success() {
        println!("Failed to install Django");
        return false;
    }
    println!("Installed Django");

    let django_admin_path = backend_folder_abs.join("venv/bin/django-admin");
    let status = Command::new(&django_admin_path)
        .arg("startproject")
        .arg("backend")
        .arg(".")
        .current_dir(backend_folder_abs)
        .status()
        .expect("Failed to run django-admin startproject");

    if status.success() {
        println!("Generated Django project");
    } else {
        println!("django-admin startproject failed");
    }

    status.success()
}

fn generate_flask_backend(backend_folder_abs: &Path) -> bool {
    if !require_tool("python3", "Install Python from https://python.org") {
        return false;
    }

    let status = Command::new("python3")
        .arg("-m")
        .arg("venv")
        .arg("venv")
        .current_dir(backend_folder_abs)
        .status()
        .expect("Failed to create virtual environment");

    if !status.success() {
        println!("Failed to create virtual environment");
        return false;
    }
    println!("Created virtual environment");

    let pip_path = backend_folder_abs.join("venv/bin/pip");
    let status = Command::new(&pip_path)
        .arg("install")
        .arg("flask")
        .current_dir(backend_folder_abs)
        .status()
        .expect("Failed to run pip install");

    if !status.success() {
        println!("Failed to install Flask");
        return false;
    }
    println!("Installed Flask");

    let app_py = "from flask import Flask\n\napp = Flask(__name__)\n\n@app.route(\"/\")\ndef home():\n    return \"Hello from Flask!\"\n\nif __name__ == \"__main__\":\n    app.run(debug=True)";
    fs::write(backend_folder_abs.join("app.py"), app_py).expect("Failed to write app.py");
    fs::write(backend_folder_abs.join("requirements.txt"), "flask\n")
        .expect("Failed to write requirements.txt");

    println!("Generated Flask project");
    true
}

fn generate_express_backend(backend_folder_abs: &Path) -> bool {
    if !require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") {
        return false;
    }

    let status = Command::new("npx")
        .arg("--yes")
        .arg("express-generator")
        .arg("--no-view")
        .current_dir(backend_folder_abs)
        .status()
        .expect("Failed to run express-generator");

    if !status.success() {
        println!("express-generator failed");
        return false;
    }
    println!("Generated Express project");

    let status = Command::new("npm")
        .arg("install")
        .current_dir(backend_folder_abs)
        .status()
        .expect("Failed to run npm install");

    if status.success() {
        println!("Installed backend dependencies");
    } else {
        println!("npm install failed");
    }

    status.success()
}

fn generate_fastify_backend(backend_folder_abs: &Path) -> bool {
    if !require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") {
        return false;
    }

    let status = Command::new("npx")
        .arg("--yes")
        .arg("fastify-cli")
        .arg("generate")
        .arg(".")
        .current_dir(backend_folder_abs)
        .status()
        .expect("Failed to run fastify-cli generate");

    if status.success() {
        println!("Generated Fastify project");
    } else {
        println!("fastify-cli generate failed");
    }

    status.success()
}

fn generate_nestjs_backend(backend_folder_abs: &Path) -> bool {
    if !require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") {
        return false;
    }

    let status = Command::new("npx")
        .arg("--yes")
        .arg("@nestjs/cli")
        .arg("new")
        .arg(".")
        .arg("--package-manager")
        .arg("npm")
        .arg("--skip-git")
        .current_dir(backend_folder_abs)
        .status()
        .expect("Failed to run nest new");

    if status.success() {
        println!("Generated NestJS project");
    } else {
        println!("nest new failed");
    }

    status.success()
}

fn generate_go_backend(backend_folder_abs: &Path, project_name: &str) -> bool {
    if !require_tool("go", "Install Go from https://go.dev/dl/") {
        return false;
    }

    let status = Command::new("go")
        .arg("mod")
        .arg("init")
        .arg(project_name)
        .current_dir(backend_folder_abs)
        .status()
        .expect("Failed to run go mod init");

    if !status.success() {
        println!("go mod init failed");
        return false;
    }
    println!("Initialized Go module");

    let status = Command::new("go")
        .arg("get")
        .arg("github.com/gin-gonic/gin")
        .current_dir(backend_folder_abs)
        .status()
        .expect("Failed to run go get");

    if !status.success() {
        println!("Failed to fetch Gin");
        return false;
    }
    println!("Installed Gin");

    let main_go = "// Backend generated with Chaos.\n// Uses Gin (https://github.com/gin-gonic/gin), an open source Go web framework.\npackage main\n\nimport \"github.com/gin-gonic/gin\"\n\nfunc main() {\n\tr := gin.Default()\n\tr.GET(\"/\", func(c *gin.Context) {\n\t\tc.JSON(200, gin.H{\"message\": \"Hello from Gin!\"})\n\t})\n\tr.Run()\n}";
    fs::write(backend_folder_abs.join("main.go"), main_go).expect("Failed to write main.go");

    println!("Generated Gin project");
    true
}

fn generate_rails_backend(backend_folder_abs: &Path) -> bool {
    if !require_tool("ruby", "Install Ruby from https://www.ruby-lang.org/en/downloads/") {
        return false;
    }
    if !require_tool("rails", "Install Rails by running: gem install rails") {
        return false;
    }

    let status = Command::new("rails")
        .arg("new")
        .arg(".")
        .arg("--skip-git")
        .current_dir(backend_folder_abs)
        .status()
        .expect("Failed to run rails new");

    if status.success() {
        println!("Generated Rails project");
    } else {
        println!("rails new failed");
    }

    status.success()
}

fn generate_laravel_backend(backend_folder_abs: &Path) -> bool {
    if !require_tool("php", "Install PHP from https://www.php.net/downloads") {
        return false;
    }
    if !require_tool("composer", "Install Composer from https://getcomposer.org/download/") {
        return false;
    }

    let status = Command::new("composer")
        .arg("create-project")
        .arg("laravel/laravel")
        .arg(".")
        .current_dir(backend_folder_abs)
        .status()
        .expect("Failed to run composer create-project");

    if status.success() {
        println!("Generated Laravel project");
    } else {
        println!("composer create-project failed");
    }

    status.success()
}

// ---------- Shared execution ----------

fn execute_plan(folder: &str, plan: &BuildPlan) {
    fs::create_dir_all(folder).expect("Failed to create project folder");

    for (relative_path, content) in &plan.files {
        let full_path = format!("{}/{}", folder, relative_path);

        if let Some(parent) = Path::new(&full_path).parent() {
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

fn run_npm_install(folder: &str, packages: &[String]) {
    if packages.is_empty() {
        return;
    }

    if !require_tool("npm", "Install Node.js (which includes npm) from https://nodejs.org") {
        return;
    }

    println!("\n Installing dependencies...");

    let mut cmd = Command::new("npm");
    cmd.arg("install").arg("-D");

    for pkg in packages {
        cmd.arg(pkg);
    }

    cmd.current_dir(folder);

    let status = cmd.status().expect("Failed to run npm install");

    if status.success() {
        println!("Dependencies installed successfully");
    } else {
        println!("npm install failed — you can run it manually inside the project folder");
    }
}