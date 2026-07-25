use clap::{Parser, Subcommand};
use dialoguer::{Select, Confirm, theme::ColorfulTheme};

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
    project_type: String,
    styling: String,
    include_js: bool,
    backend_language: Option<String>,
    backend_framework: Option<String>,
    install_dependencies: bool,
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

fn run_initialize() {
    let theme = ColorfulTheme::default();

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
        project_type,
        styling,
        include_js,
        backend_language,
        backend_framework,
        install_dependencies,
    };

    println!("\n--- Config Summary ---");
    println!("Project type: {}", config.project_type);
    println!("Styling: {}", config.styling);
    println!("Include JS: {}", config.include_js);
    println!("Backend language: {:?}", config.backend_language);
    println!("Backend framework: {:?}", config.backend_framework);
    println!("Install dependencies: {}", config.install_dependencies);
}