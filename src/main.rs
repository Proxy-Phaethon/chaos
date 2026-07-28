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

// ---------- Config structs ----------

struct ProjectConfig {
    project_name: String,
    project_type: String,
    frontend_language: Option<String>,
    backend_language: Option<String>,
    frontend: Option<FrontendConfig>,
    backend: Option<BackendConfig>,
    database: Option<DatabaseConfig>,
    background_jobs: Option<String>,
    auth: Option<String>,
    tooling: ToolingConfig,
    infra: InfraConfig,
    install_dependencies: bool,
}

struct FrontendConfig {
    framework: String,
    meta_framework: Option<String>,
    styling: String,
    component_library: Option<String>,
    include_js: bool,
    state_management: Option<String>,
    data_fetching: Option<String>,
    forms: Option<String>,
}

struct BackendConfig {
    framework: String,
}

struct DatabaseConfig {
    engine: String,
    provider: Option<String>,
    orm: Option<String>,
}

struct ToolingConfig {
    linting: Option<String>,
    testing: Option<String>,
    git_hooks: Option<String>,
}

struct InfraConfig {
    docker: bool,
    hosting: Option<String>,
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
        Ok(output) => {
            let combined_output = format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );

            if output.status.success() && !combined_output.contains("is not currently installed") {
                true
            } else {
                println!("\n '{}' isn't installed or isn't on your PATH.", command);
                println!("   {}", install_hint);
                false
            }
        }
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

    let project_types = vec!["Static Website", "Web Application"];
    let selection = Select::with_theme(&theme)
        .with_prompt("What are you building?")
        .items(&project_types)
        .default(0)
        .interact()
        .unwrap();
    let project_type = project_types[selection].to_string();

    let config = if project_type == "Static Website" {
        build_static_website_config(&theme, project_name, project_type)
    } else {
        build_web_application_config(&theme, project_name, project_type)
    };

    print_summary(&config);
    generate_project(&config);
}

fn build_static_website_config(
    theme: &ColorfulTheme,
    project_name: String,
    project_type: String,
) -> ProjectConfig {
    let (styling, _) = ask_styling(theme, "None");

    let include_js = Confirm::with_theme(theme)
        .with_prompt("Include JavaScript?")
        .default(true)
        .interact()
        .unwrap();

    let install_dependencies = Confirm::with_theme(theme)
        .with_prompt("Install dependencies now?")
        .default(true)
        .interact()
        .unwrap();

    ProjectConfig {
        project_name,
        project_type,
        frontend_language: None,
        backend_language: None,
        frontend: Some(FrontendConfig {
            framework: "None".to_string(),
            meta_framework: None,
            styling,
            component_library: None,
            include_js,
            state_management: None,
            data_fetching: None,
            forms: None,
        }),
        backend: None,
        database: None,
        background_jobs: None,
        auth: None,
        tooling: ToolingConfig {
            linting: None,
            testing: None,
            git_hooks: None,
        },
        infra: InfraConfig {
            docker: false,
            hosting: None,
        },
        install_dependencies,
    }
}

fn build_web_application_config(
    theme: &ColorfulTheme,
    project_name: String,
    project_type: String,
) -> ProjectConfig {
    let (frontend_language, backend_language) = ask_languages(theme);
    let (framework, meta_framework) = ask_frontend_framework(theme);
    let backend_framework = ask_backend_framework(theme, &backend_language);
    let database = ask_database(theme, &backend_language);
    let background_jobs = ask_background_jobs(theme, &backend_language);
    let (styling, component_library) = ask_styling(theme, &framework);
    let state_management = ask_state_management(theme, &framework);
    let data_fetching = ask_data_fetching(theme, &framework, &frontend_language, &backend_language);
    let forms = ask_forms(theme, &framework);
    let auth = ask_auth(theme, &database, &meta_framework);
    let tooling = ask_tooling(theme, &frontend_language, &backend_language);
    let infra = ask_infra(theme);

    let install_dependencies = Confirm::with_theme(theme)
        .with_prompt("Install dependencies now?")
        .default(true)
        .interact()
        .unwrap();

    ProjectConfig {
        project_name,
        project_type,
        frontend_language: Some(frontend_language),
        backend_language: Some(backend_language),
        frontend: Some(FrontendConfig {
            framework,
            meta_framework,
            styling,
            component_library,
            include_js: true,
            state_management,
            data_fetching,
            forms,
        }),
        backend: Some(BackendConfig {
            framework: backend_framework,
        }),
        database,
        background_jobs,
        auth,
        tooling,
        infra,
        install_dependencies,
    }
}

fn ask_languages(theme: &ColorfulTheme) -> (String, String) {
    let lang_options = vec!["TypeScript", "JavaScript"];
    let selection = Select::with_theme(theme)
        .with_prompt("Frontend language?")
        .items(&lang_options)
        .default(0)
        .interact()
        .unwrap();
    let frontend_language = lang_options[selection].to_string();

    let backend_options = vec![
        "TypeScript", "Go", "Python", "Rust", "PHP", "Ruby", "Java", "C#", "Elixir",
    ];
    let selection = Select::with_theme(theme)
        .with_prompt("Backend language?")
        .items(&backend_options)
        .default(0)
        .interact()
        .unwrap();
    let backend_language = backend_options[selection].to_string();

    (frontend_language, backend_language)
}

fn ask_frontend_framework(theme: &ColorfulTheme) -> (String, Option<String>) {
    let frameworks = vec![
        "None", "React", "Vue.js", "Angular", "Svelte", "SolidJS", "Preact", "Qwik",
    ];
    let selection = Select::with_theme(theme)
        .with_prompt("Frontend framework?")
        .items(&frameworks)
        .default(0)
        .interact()
        .unwrap();
    let framework = frameworks[selection].to_string();

    let meta_options: Vec<&str> = match framework.as_str() {
        "React" => vec!["None", "Next.js", "Remix"],
        "Vue.js" => vec!["None", "Nuxt"],
        "Svelte" => vec!["None", "SvelteKit"],
        "SolidJS" => vec!["None", "SolidStart"],
        _ => vec![],
    };

    let meta_framework = if !meta_options.is_empty() {
        let selection = Select::with_theme(theme)
            .with_prompt("Meta-framework?")
            .items(&meta_options)
            .default(0)
            .interact()
            .unwrap();
        let choice = meta_options[selection].to_string();
        if choice == "None" { None } else { Some(choice) }
    } else {
        None
    };

    (framework, meta_framework)
}

fn ask_backend_framework(theme: &ColorfulTheme, backend_language: &str) -> String {
    let frameworks: Vec<&str> = match backend_language {
        "TypeScript" => vec!["Express", "NestJS", "Fastify", "Elysia", "Hono"],
        "Go" => vec!["Gin", "Fiber", "Chi", "Echo"],
        "Python" => vec!["FastAPI", "Django", "Flask", "FastHTML"],
        "Rust" => vec!["Axum", "Actix-web", "Rocket", "Poem"],
        "PHP" => vec!["Laravel", "Symfony", "CodeIgniter", "Slim", "Flight"],
        "Ruby" => vec!["Rails", "Sinatra", "Hanami"],
        "Java" => vec!["Spring Boot", "Quarkus", "Micronaut", "Ktor"],
        "C#" => vec!["ASP.NET Minimal API", "ASP.NET Web API", "ASP.NET MVC", "Razor Pages"],
        "Elixir" => vec!["Phoenix"],
        _ => vec![],
    };

    let selection = Select::with_theme(theme)
        .with_prompt("Backend framework?")
        .items(&frameworks)
        .default(0)
        .interact()
        .unwrap();
    frameworks[selection].to_string()
}

fn ask_database(theme: &ColorfulTheme, backend_language: &str) -> Option<DatabaseConfig> {
    let wants_db = Confirm::with_theme(theme)
        .with_prompt("Do you need a database?")
        .default(true)
        .interact()
        .unwrap();

    if !wants_db {
        return None;
    }

    let engines = vec![
        "PostgreSQL", "MySQL", "SQLite", "MS SQL Server", "MariaDB",
        "MongoDB", "CouchDB", "Redis", "Cassandra", "DynamoDB", "Neo4j",
        "Firebase Firestore",
    ];
    let selection = Select::with_theme(theme)
        .with_prompt("Database engine?")
        .items(&engines)
        .default(0)
        .interact()
        .unwrap();
    let engine = engines[selection].to_string();

    let provider = if engine == "Firebase Firestore" {
        Some("Firebase".to_string())
    } else {
        let provider_options: Vec<&str> = match engine.as_str() {
            "PostgreSQL" => vec!["Self-hosted", "Supabase", "Neon", "AWS RDS", "CockroachDB"],
            "MySQL" => vec!["Self-hosted", "PlanetScale", "AWS RDS"],
            "MongoDB" => vec!["Self-hosted", "MongoDB Atlas"],
            _ => vec!["Self-hosted"],
        };
        let selection = Select::with_theme(theme)
            .with_prompt("Hosting/provider?")
            .items(&provider_options)
            .default(0)
            .interact()
            .unwrap();
        let choice = provider_options[selection].to_string();
        if choice == "Self-hosted" { None } else { Some(choice) }
    };

    let orm = if engine == "Firebase Firestore" {
        None
    } else {
        let orm_options: Vec<&str> = match backend_language {
            "TypeScript" => vec!["Prisma", "Drizzle", "TypeORM", "Sequelize", "Kysely", "Mongoose"],
            "Go" => vec!["SQLC", "GORM", "Ent", "Pgx"],
            "Python" => vec!["SQLAlchemy", "SQLModel", "Tortoise ORM"],
            "Rust" => vec!["Diesel", "SeaORM", "SQLx"],
            "PHP" => vec!["Doctrine"],
            "Java" => vec!["Hibernate"],
            "C#" => vec!["Entity Framework Core"],
            _ => vec![],
        };
        if !orm_options.is_empty() {
            let selection = Select::with_theme(theme)
                .with_prompt("ORM / query layer?")
                .items(&orm_options)
                .default(0)
                .interact()
                .unwrap();
            Some(orm_options[selection].to_string())
        } else {
            None
        }
    };

    Some(DatabaseConfig { engine, provider, orm })
}

fn ask_background_jobs(theme: &ColorfulTheme, backend_language: &str) -> Option<String> {
    let wants_jobs = Confirm::with_theme(theme)
        .with_prompt("Do you need background jobs / task queues?")
        .default(false)
        .interact()
        .unwrap();

    if !wants_jobs {
        return None;
    }

    let mut options: Vec<&str> = match backend_language {
        "TypeScript" => vec!["BullMQ"],
        "Python" => vec!["Celery"],
        "Go" => vec!["Asynq"],
        _ => vec![],
    };
    options.extend(vec!["Redis (raw)", "RabbitMQ", "Apache Kafka"]);

    let selection = Select::with_theme(theme)
        .with_prompt("Background job system?")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    Some(options[selection].to_string())
}

fn ask_styling(theme: &ColorfulTheme, framework: &str) -> (String, Option<String>) {
    let styling_options = vec![
        "Tailwind CSS", "Plain CSS", "Sass/SCSS", "Bootstrap", "Bulma",
        "Foundation", "Semantic UI", "Materialize", "Pure.css", "UIKit", "Pico.css",
    ];
    let selection = Select::with_theme(theme)
        .with_prompt("CSS / styling approach?")
        .items(&styling_options)
        .default(0)
        .interact()
        .unwrap();
    let styling = styling_options[selection].to_string();

    let component_options: Vec<&str> = match framework {
        "React" => vec!["None", "Chakra UI", "Radix UI", "Headless UI", "Shadcn/ui"],
        "Vue.js" => vec!["None", "PrimeVue"],
        _ => vec![],
    };

    let component_library = if !component_options.is_empty() {
        let selection = Select::with_theme(theme)
            .with_prompt("Component library?")
            .items(&component_options)
            .default(0)
            .interact()
            .unwrap();
        let choice = component_options[selection].to_string();
        if choice == "None" { None } else { Some(choice) }
    } else {
        None
    };

    (styling, component_library)
}

fn ask_state_management(theme: &ColorfulTheme, framework: &str) -> Option<String> {
    if framework == "None" {
        return None;
    }

    let mut options: Vec<&str> = match framework {
        "React" => vec!["Zustand", "Redux Toolkit", "Recoil", "Jotai"],
        "Vue.js" => vec!["Pinia"],
        "Svelte" => vec!["Svelte Stores", "Svelte Runes"],
        _ => vec![],
    };
    options.extend(vec!["MobX", "XState", "Nano Stores", "None"]);

    let selection = Select::with_theme(theme)
        .with_prompt("State management?")
        .items(&options)
        .default(options.len() - 1)
        .interact()
        .unwrap();
    let choice = options[selection].to_string();
    if choice == "None" { None } else { Some(choice) }
}

fn ask_data_fetching(
    theme: &ColorfulTheme,
    framework: &str,
    frontend_language: &str,
    backend_language: &str,
) -> Option<String> {
    let mut options = vec!["None", "TanStack Query"];
    if framework == "React" {
        options.push("SWR");
    }
    if frontend_language == "TypeScript" && backend_language == "TypeScript" {
        options.push("tRPC");
        options.push("ts-rest");
    }

    let selection = Select::with_theme(theme)
        .with_prompt("Data fetching approach?")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    let choice = options[selection].to_string();
    if choice == "None" { None } else { Some(choice) }
}

fn ask_forms(theme: &ColorfulTheme, framework: &str) -> Option<String> {
    let mut options: Vec<&str> = match framework {
        "React" => vec!["React Hook Form", "Formik"],
        "Vue.js" => vec!["FormKit"],
        _ => vec![],
    };
    options.extend(vec!["Zod", "Valibot", "Yup", "None"]);

    let selection = Select::with_theme(theme)
        .with_prompt("Forms & validation?")
        .items(&options)
        .default(options.len() - 1)
        .interact()
        .unwrap();
    let choice = options[selection].to_string();
    if choice == "None" { None } else { Some(choice) }
}

fn ask_auth(
    theme: &ColorfulTheme,
    database: &Option<DatabaseConfig>,
    meta_framework: &Option<String>,
) -> Option<String> {
    let mut options = vec!["None"];

    if let Some(db) = database {
        if db.provider.as_deref() == Some("Supabase") {
            options.push("Supabase Auth");
        }
        if db.provider.as_deref() == Some("Firebase") {
            options.push("Firebase Auth");
        }
    }
    if meta_framework.as_deref() == Some("Next.js") {
        options.push("Auth.js/NextAuth");
    }
    options.extend(vec!["Clerk", "Auth0", "Kinde", "Stytch", "Lucia", "Passport.js"]);

    let selection = Select::with_theme(theme)
        .with_prompt("Auth provider?")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    let choice = options[selection].to_string();
    if choice == "None" { None } else { Some(choice) }
}

fn ask_tooling(theme: &ColorfulTheme, frontend_language: &str, backend_language: &str) -> ToolingConfig {
    let js_involved = frontend_language == "TypeScript"
        || frontend_language == "JavaScript"
        || backend_language == "TypeScript";

    let mut lint_options: Vec<&str> = vec![];
    if js_involved {
        lint_options.extend(vec!["ESLint + Prettier", "Biome", "Oxlint"]);
    }
    match backend_language {
        "Python" => lint_options.extend(vec!["Ruff", "Black + Flake8"]),
        "Go" => lint_options.push("Golangci-lint + Gofmt"),
        "Rust" => lint_options.push("Clippy + Rustfmt"),
        _ => {}
    }
    lint_options.push("None");

    let selection = Select::with_theme(theme)
        .with_prompt("Linting/formatting?")
        .items(&lint_options)
        .default(lint_options.len() - 1)
        .interact()
        .unwrap();
    let choice = lint_options[selection].to_string();
    let linting = if choice == "None" { None } else { Some(choice) };

    let mut test_options: Vec<&str> = vec![];
    if js_involved {
        test_options.extend(vec!["Vitest", "Jest", "Playwright", "Cypress"]);
    }
    match backend_language {
        "Python" => test_options.push("PyTest"),
        "Go" => test_options.push("Go Test"),
        _ => {}
    }
    test_options.push("None");

    let selection = Select::with_theme(theme)
        .with_prompt("Testing framework?")
        .items(&test_options)
        .default(test_options.len() - 1)
        .interact()
        .unwrap();
    let choice = test_options[selection].to_string();
    let testing = if choice == "None" { None } else { Some(choice) };

    let hook_options: Vec<&str> = if js_involved {
        vec!["Husky + lint-staged", "Lefthook", "None"]
    } else {
        vec!["Lefthook", "simple-git-hooks", "None"]
    };
    let selection = Select::with_theme(theme)
        .with_prompt("Git hooks?")
        .items(&hook_options)
        .default(hook_options.len() - 1)
        .interact()
        .unwrap();
    let choice = hook_options[selection].to_string();
    let git_hooks = if choice == "None" { None } else { Some(choice) };

    ToolingConfig { linting, testing, git_hooks }
}

fn ask_infra(theme: &ColorfulTheme) -> InfraConfig {
    let docker = Confirm::with_theme(theme)
        .with_prompt("Containerize with Docker?")
        .default(false)
        .interact()
        .unwrap();

    let hosting_options = vec!["None", "Vercel", "Netlify", "Render", "AWS"];
    let selection = Select::with_theme(theme)
        .with_prompt("Hosting target?")
        .items(&hosting_options)
        .default(0)
        .interact()
        .unwrap();
    let choice = hosting_options[selection].to_string();
    let hosting = if choice == "None" { None } else { Some(choice) };

    InfraConfig { docker, hosting }
}

fn print_summary(config: &ProjectConfig) {
    println!("\n--- Config Summary ---");
    println!("Project name: {}", config.project_name);
    println!("Project type: {}", config.project_type);
    println!("Frontend language: {:?}", config.frontend_language);
    println!("Backend language: {:?}", config.backend_language);

    if let Some(fe) = &config.frontend {
        println!("\nFrontend:");
        println!("  Framework: {}", fe.framework);
        println!("  Meta-framework: {:?}", fe.meta_framework);
        println!("  Styling: {}", fe.styling);
        println!("  Component library: {:?}", fe.component_library);
        println!("  Include JS: {}", fe.include_js);
        println!("  State management: {:?}", fe.state_management);
        println!("  Data fetching: {:?}", fe.data_fetching);
        println!("  Forms: {:?}", fe.forms);
    }

    if let Some(be) = &config.backend {
        println!("\nBackend:");
        println!("  Framework: {}", be.framework);
    }

    match &config.database {
        Some(db) => {
            println!("\nDatabase:");
            println!("  Engine: {}", db.engine);
            println!("  Provider: {:?}", db.provider);
            println!("  ORM: {:?}", db.orm);
        }
        None => println!("\nDatabase: None"),
    }

    println!("\nBackground jobs: {:?}", config.background_jobs);
    println!("Auth: {:?}", config.auth);

    println!("\nTooling:");
    println!("  Linting: {:?}", config.tooling.linting);
    println!("  Testing: {:?}", config.tooling.testing);
    println!("  Git hooks: {:?}", config.tooling.git_hooks);

    println!("\nInfra:");
    println!("  Docker: {}", config.infra.docker);
    println!("  Hosting: {:?}", config.infra.hosting);

    println!("\nInstall dependencies: {}", config.install_dependencies);
    println!();
}

// ---------- Shared content helpers ----------

fn readme_content(config: &ProjectConfig) -> String {
    format!("# {}\n\nGenerated with Chaos.\n", config.project_name)
}

// ---------- Frontend feature contributors ----------

fn add_html_boilerplate(plan: &mut BuildPlan, frontend: &FrontendConfig, project_name: &str) {
    let css_link = match frontend.styling.as_str() {
        "Tailwind CSS" => "<link rel=\"stylesheet\" href=\"dist/output.css\">".to_string(),
        "Sass/SCSS" => "<link rel=\"stylesheet\" href=\"dist/output.css\">".to_string(),
        "Bootstrap" => {
            "<link rel=\"stylesheet\" href=\"node_modules/bootstrap/dist/css/bootstrap.min.css\">"
                .to_string()
        }
        _ => "<link rel=\"stylesheet\" href=\"styles/style.css\">".to_string(),
    };

    let script_tag = if frontend.include_js {
        "<script src=\"script.js\"></script>"
    } else {
        ""
    };

    let html = format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <title>{}</title>\n    {}\n</head>\n<body>\n    <h1>Welcome to {}</h1>\n    {}\n</body>\n</html>",
        project_name, css_link, project_name, script_tag
    );

    plan.files.push(("index.html".to_string(), html));
}

fn add_tailwind(plan: &mut BuildPlan, project_name: &str) {
    let package_json = format!(
        "{{\n  \"name\": \"{}\",\n  \"scripts\": {{ \"build\": \"tailwindcss -i src/input.css -o dist/output.css\" }}\n}}",
        project_name
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
    plan.npm_packages.push("bootstrap".to_string());
    plan.gitignore_entries.push("node_modules/".to_string());
}

fn add_sass(plan: &mut BuildPlan, project_name: &str) {
    let package_json = format!(
        "{{\n  \"name\": \"{}\",\n  \"scripts\": {{ \"build\": \"sass src/input.scss dist/output.css\" }}\n}}",
        project_name
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

fn generate_frontend_plan(frontend: &FrontendConfig, project_name: &str) -> BuildPlan {
    let mut plan = BuildPlan::new();

    add_html_boilerplate(&mut plan, frontend, project_name);

    match frontend.styling.as_str() {
        "Tailwind CSS" => add_tailwind(&mut plan, project_name),
        "Bootstrap" => add_bootstrap(&mut plan),
        "Sass/SCSS" => add_sass(&mut plan, project_name),
        _ => add_plain_css(&mut plan),
    }

    if frontend.include_js {
        add_javascript(&mut plan);
    }

    plan
}

// ---------- Static Website ----------

fn generate_static_webpage(config: &ProjectConfig) {
    let frontend = config.frontend.as_ref().unwrap();
    let mut plan = generate_frontend_plan(frontend, &config.project_name);
    plan.files.push(("README.md".to_string(), readme_content(config)));

    execute_plan(&config.project_name, &plan);

    if config.install_dependencies && !plan.npm_packages.is_empty() {
        run_npm_install(&config.project_name, &plan.npm_packages);
    }
}

// ---------- Web Application ----------

fn generate_project(config: &ProjectConfig) {
    if config.project_type == "Static Website" {
        generate_static_webpage(config);
        return;
    }

    fs::create_dir_all(&config.project_name).expect("Failed to create project folder");

    let docs_folder = format!("{}/docs", config.project_name);
    fs::create_dir_all(&docs_folder).expect("Failed to create docs folder");
    fs::write(format!("{}/.gitkeep", docs_folder), "").expect("Failed to create docs placeholder");

    let frontend_cfg = config.frontend.as_ref().unwrap();
    let frontend_folder = format!("{}/frontend", config.project_name);
    let mut root_gitignore_entries: Vec<String> = Vec::new();

    if frontend_cfg.framework == "None" {
        let frontend_plan = generate_frontend_plan(frontend_cfg, &config.project_name);
        execute_plan(&frontend_folder, &frontend_plan);

        if config.install_dependencies && !frontend_plan.npm_packages.is_empty() {
            run_npm_install(&frontend_folder, &frontend_plan.npm_packages);
        }

        root_gitignore_entries.extend(
            frontend_plan
                .gitignore_entries
                .iter()
                .map(|entry| format!("frontend/{}", entry)),
        );
    } else {
        let frontend_entries = generate_frontend(config);
        root_gitignore_entries.extend(frontend_entries);
    }

    if config.install_dependencies {
        let backend_entries = generate_backend(config);
        root_gitignore_entries.extend(backend_entries);
    } else {
        fs::create_dir_all(format!("{}/backend", config.project_name))
            .expect("Failed to create backend folder");
        println!(
            "\n Backend wasn't scaffolded — every supported backend requires installing \
             the framework/toolchain before its project files can be generated. Run \
             'chaos initialize' again with dependency installation enabled to build a \
             real backend."
        );
    }

    for (path, content) in generate_docker_files(config) {
        let full_path = format!("{}/{}", config.project_name, path);
        fs::write(&full_path, content).expect("Failed to write docker file");
        println!("Created: {}", full_path);
    }

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

    println!(
        "\nNote: database, background jobs, auth, and tooling choices were captured \
         in the config summary above but aren't wired into file generation yet — \
         that's the next layer of work."
    );
}

// ---------- Frontend dispatcher (framework/meta-framework scaffolding) ----------

fn generate_frontend(config: &ProjectConfig) -> Vec<String> {
    let frontend_folder = format!("{}/frontend", config.project_name);
    fs::create_dir_all(&frontend_folder).expect("Failed to create frontend folder");

    let frontend = config.frontend.as_ref().unwrap();

    if !config.install_dependencies {
        fs::create_dir_all(&frontend_folder).expect("Failed to create frontend folder");
        println!(
            "\n Frontend wasn't scaffolded — {} requires installing its toolchain \
             before project files can be generated. Run 'chaos initialize' again with \
             dependency installation enabled to build a real frontend.",
            frontend.framework
        );
        return vec![];
    }

    let frontend_folder_abs =
        fs::canonicalize(&frontend_folder).expect("Failed to resolve frontend folder path");

    let ts = config.frontend_language.as_deref() == Some("TypeScript");

    println!(
        "\n🔧 Setting up {} frontend...",
        frontend.meta_framework.as_deref().unwrap_or(&frontend.framework)
    );

    match frontend.meta_framework.as_deref() {
        Some("Next.js") => {
            generate_nextjs_frontend(&frontend_folder_abs, ts);
            return vec!["frontend/node_modules/".to_string(), "frontend/.next/".to_string()];
        }
        Some("Nuxt") => {
            generate_nuxt_frontend(&frontend_folder_abs);
            return vec![
                "frontend/node_modules/".to_string(),
                "frontend/.nuxt/".to_string(),
                "frontend/.output/".to_string(),
            ];
        }
        Some("SvelteKit") => {
            generate_sveltekit_frontend(&frontend_folder_abs);
            return vec![
                "frontend/node_modules/".to_string(),
                "frontend/.svelte-kit/".to_string(),
                "frontend/build/".to_string(),
            ];
        }
        Some(other) => {
            println!("Meta-framework {} isn't built yet — captured in config only.", other);
            return vec![];
        }
        None => {}
    }

    match frontend.framework.as_str() {
        "React" => {
            let template = if ts { "react-ts" } else { "react" };
            generate_vite_frontend(&frontend_folder_abs, template);
            vec!["frontend/node_modules/".to_string(), "frontend/dist/".to_string()]
        }
        "Vue.js" => {
            let template = if ts { "vue-ts" } else { "vue" };
            generate_vite_frontend(&frontend_folder_abs, template);
            vec!["frontend/node_modules/".to_string(), "frontend/dist/".to_string()]
        }
        "Svelte" => {
            let template = if ts { "svelte-ts" } else { "svelte" };
            generate_vite_frontend(&frontend_folder_abs, template);
            vec!["frontend/node_modules/".to_string(), "frontend/dist/".to_string()]
        }
        "Preact" => {
            let template = if ts { "preact-ts" } else { "preact" };
            generate_vite_frontend(&frontend_folder_abs, template);
            vec!["frontend/node_modules/".to_string(), "frontend/dist/".to_string()]
        }
        "SolidJS" => {
            let template = if ts { "solid-ts" } else { "solid" };
            generate_vite_frontend(&frontend_folder_abs, template);
            vec!["frontend/node_modules/".to_string(), "frontend/dist/".to_string()]
        }
        "Angular" => {
            generate_angular_frontend(&frontend_folder_abs, &config.project_name);
            vec![
                "frontend/node_modules/".to_string(),
                "frontend/dist/".to_string(),
                "frontend/.angular/".to_string(),
            ]
        }
        other => {
            println!("Frontend framework {} isn't built yet — captured in config only.", other);
            vec![]
        }
    }
}

fn generate_vite_frontend(frontend_folder_abs: &Path, template: &str) -> bool {
    if !require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") {
        return false;
    }

    let status = Command::new("npx")
        .arg("--yes")
        .arg("create-vite@latest")
        .arg(".")
        .arg("--template")
        .arg(template)
        .current_dir(frontend_folder_abs)
        .status()
        .expect("Failed to run create-vite");

    if !status.success() {
        println!("create-vite failed");
        return false;
    }
    println!("Generated Vite + {} project", template);

    let status = Command::new("npm")
        .arg("install")
        .current_dir(frontend_folder_abs)
        .status()
        .expect("Failed to run npm install");

    if status.success() {
        println!("Installed frontend dependencies");
    } else {
        println!("npm install failed");
    }

    status.success()
}

fn generate_angular_frontend(frontend_folder_abs: &Path, project_name: &str) -> bool {
    if !require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") {
        return false;
    }

    let status = Command::new("npx")
        .arg("--yes")
        .arg("@angular/cli@latest")
        .arg("new")
        .arg(project_name)
        .arg("--directory")
        .arg(".")
        .arg("--skip-git")
        .arg("--defaults")
        .current_dir(frontend_folder_abs)
        .status()
        .expect("Failed to run ng new");

    if status.success() {
        println!("Generated Angular project");
    } else {
        println!("ng new failed");
    }

    status.success()
}

fn generate_nextjs_frontend(frontend_folder_abs: &Path, typescript: bool) -> bool {
    if !require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") {
        return false;
    }

    let mut cmd = Command::new("npx");
    cmd.arg("--yes")
        .arg("create-next-app@latest")
        .arg(".")
        .arg("--eslint")
        .arg("--tailwind")
        .arg("--app")
        .arg("--no-src-dir")
        .arg("--import-alias")
        .arg("@/*")
        .arg("--use-npm")
        .arg("--yes");

    if typescript {
        cmd.arg("--ts");
    } else {
        cmd.arg("--js");
    }

    let status = cmd
        .current_dir(frontend_folder_abs)
        .status()
        .expect("Failed to run create-next-app");

    if status.success() {
        println!("Generated Next.js project");
    } else {
        println!("create-next-app failed");
    }

    status.success()
}

fn generate_nuxt_frontend(frontend_folder_abs: &Path) -> bool {
    if !require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") {
        return false;
    }

    let status = Command::new("npx")
        .arg("--yes")
        .arg("nuxi@latest")
        .arg("init")
        .arg(".")
        .arg("--force")
        .arg("--packageManager")
        .arg("npm")
        .current_dir(frontend_folder_abs)
        .status()
        .expect("Failed to run nuxi init");

    if !status.success() {
        println!("nuxi init failed");
        return false;
    }
    println!("Generated Nuxt project");

    let status = Command::new("npm")
        .arg("install")
        .current_dir(frontend_folder_abs)
        .status()
        .expect("Failed to run npm install");

    if status.success() {
        println!("Installed frontend dependencies");
    } else {
        println!("npm install failed");
    }

    status.success()
}

fn generate_sveltekit_frontend(frontend_folder_abs: &Path) -> bool {
    if !require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") {
        return false;
    }

    let status = Command::new("npx")
        .arg("--yes")
        .arg("sv")
        .arg("create")
        .arg(".")
        .arg("--template")
        .arg("minimal")
        .arg("--types")
        .arg("ts")
        .arg("--no-add-ons")
        .current_dir(frontend_folder_abs)
        .status()
        .expect("Failed to run sv create");

    if status.success() {
        println!("Generated SvelteKit project");
    } else {
        println!("sv create failed");
    }

    status.success()
}

fn generate_docker_files(config: &ProjectConfig) -> Vec<(String, String)> {
    if !config.infra.docker {
        return vec![];
    }

    let dockerfile = "# Generated by Chaos — generic starter, adjust for your actual stack.\nFROM node:20-alpine\nWORKDIR /app\nCOPY . .\nRUN npm install\nCMD [\"npm\", \"start\"]\n".to_string();

    let compose = "# Generated by Chaos — generic starter, adjust for your actual stack.\nservices:\n  app:\n    build: .\n    ports:\n      - \"3000:3000\"\n".to_string();

    vec![
        ("Dockerfile".to_string(), dockerfile),
        ("docker-compose.yml".to_string(), compose),
    ]
}

// ---------- Backend dispatcher ----------

fn generate_backend(config: &ProjectConfig) -> Vec<String> {
    let backend_folder = format!("{}/backend", config.project_name);
    fs::create_dir_all(&backend_folder).expect("Failed to create backend folder");

    let backend_folder_abs =
        fs::canonicalize(&backend_folder).expect("Failed to resolve backend folder path");

    let language = config.backend_language.as_deref();
    let framework = config.backend.as_ref().map(|b| b.framework.as_str());

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
            println!(
                "⚠️  Backend combination ({:?}, {:?}) isn't built yet — captured in config only.",
                language, framework
            );
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

    println!("\n📦 Installing dependencies...");

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