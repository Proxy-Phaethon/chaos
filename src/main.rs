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
    validator: Option<String>,
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
    container_tool: Option<String>, // "Docker" | "Podman" | None
    hosting: Option<String>,
}

struct BuildPlan {
    files: Vec<(String, String)>,
    gitignore_entries: Vec<String>,
    npm_dependencies: Vec<String>,
    npm_dev_dependencies: Vec<String>,
}

impl BuildPlan {
    fn new() -> Self {
        BuildPlan {
            files: Vec::new(),
            gitignore_entries: Vec::new(),
            npm_dependencies: Vec::new(),
            npm_dev_dependencies: Vec::new(),
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
        } else if c.is_whitespace() || !c.is_alphanumeric() {
            if !last_was_separator && !result.is_empty() {
                result.push('-');
                last_was_separator = true;
            }
        }
    }
    result.trim_end_matches(['-', '_']).to_string()
}

fn require_tool(command: &str, install_hint: &str) -> bool {
    let check = Command::new(command).arg("--version").output();
    match check {
        Ok(output) => {
            let combined = format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            if output.status.success() && !combined.contains("is not currently installed") {
                true
            } else {
                println!("\n '{}' isn't installed or isn't on your PATH.\n   {}", command, install_hint);
                false
            }
        }
        Err(_) => {
            println!("\n '{}' isn't installed or isn't on your PATH.\n   {}", command, install_hint);
            false
        }
    }
}

fn ext(config: &ProjectConfig) -> &'static str {
    if config.frontend_language.as_deref() == Some("TypeScript") { "ts" } else { "js" }
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

fn build_static_website_config(theme: &ColorfulTheme, project_name: String, project_type: String) -> ProjectConfig {
    let (styling, _) = ask_styling(theme, "None");
    let include_js = Confirm::with_theme(theme).with_prompt("Include JavaScript?").default(true).interact().unwrap();
    let install_dependencies = Confirm::with_theme(theme).with_prompt("Install dependencies now?").default(true).interact().unwrap();

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
            validator: None,
        }),
        backend: None,
        database: None,
        background_jobs: None,
        auth: None,
        tooling: ToolingConfig { linting: None, testing: None, git_hooks: None },
        infra: InfraConfig { container_tool: None, hosting: None },
        install_dependencies,
    }
}

fn build_web_application_config(theme: &ColorfulTheme, project_name: String, project_type: String) -> ProjectConfig {
    let (frontend_language, backend_language) = ask_languages(theme);
    let (framework, meta_framework) = ask_frontend_framework(theme);
    let backend_framework = ask_backend_framework(theme, &backend_language);
    let database = ask_database(theme, &backend_language);
    let background_jobs = ask_background_jobs(theme, &backend_language);
    let (styling, component_library) = ask_styling(theme, &framework);
    let state_management = ask_state_management(theme, &framework);
    let data_fetching = ask_data_fetching(theme, &framework, &frontend_language, &backend_language);
    let (forms, validator) = ask_forms(theme, &framework);
    let auth = ask_auth(theme, &database, &meta_framework);
    let tooling = ask_tooling(theme, &frontend_language, &backend_language);
    let infra = ask_infra(theme);
    let install_dependencies = Confirm::with_theme(theme).with_prompt("Install dependencies now?").default(true).interact().unwrap();

    ProjectConfig {
        project_name,
        project_type,
        frontend_language: Some(frontend_language),
        backend_language: Some(backend_language),
        frontend: Some(FrontendConfig {
            framework, meta_framework, styling, component_library,
            include_js: true, state_management, data_fetching, forms, validator,
        }),
        backend: Some(BackendConfig { framework: backend_framework }),
        database, background_jobs, auth, tooling, infra, install_dependencies,
    }
}

fn ask_languages(theme: &ColorfulTheme) -> (String, String) {
    let lang_options = vec!["TypeScript", "JavaScript"];
    let s = Select::with_theme(theme).with_prompt("Frontend language?").items(&lang_options).default(0).interact().unwrap();
    let frontend_language = lang_options[s].to_string();

    let backend_options = vec!["TypeScript", "Go", "Python", "Rust", "PHP", "Ruby", "Java", "C#", "Elixir"];
    let s = Select::with_theme(theme).with_prompt("Backend language?").items(&backend_options).default(0).interact().unwrap();
    let backend_language = backend_options[s].to_string();

    (frontend_language, backend_language)
}

fn ask_frontend_framework(theme: &ColorfulTheme) -> (String, Option<String>) {
    let frameworks = vec!["None", "React", "Vue.js", "Angular", "Svelte", "SolidJS", "Preact", "Qwik"];
    let s = Select::with_theme(theme).with_prompt("Frontend framework?").items(&frameworks).default(0).interact().unwrap();
    let framework = frameworks[s].to_string();

    let meta_options: Vec<&str> = match framework.as_str() {
        "React" => vec!["None", "Next.js", "Remix"],
        "Vue.js" => vec!["None", "Nuxt"],
        "Svelte" => vec!["None", "SvelteKit"],
        "SolidJS" => vec!["None", "SolidStart"],
        _ => vec![],
    };
    let meta_framework = if !meta_options.is_empty() {
        let s = Select::with_theme(theme).with_prompt("Meta-framework?").items(&meta_options).default(0).interact().unwrap();
        let choice = meta_options[s].to_string();
        if choice == "None" { None } else { Some(choice) }
    } else { None };

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
    let s = Select::with_theme(theme).with_prompt("Backend framework?").items(&frameworks).default(0).interact().unwrap();
    frameworks[s].to_string()
}

fn ask_database(theme: &ColorfulTheme, backend_language: &str) -> Option<DatabaseConfig> {
    let wants_db = Confirm::with_theme(theme).with_prompt("Do you need a database?").default(true).interact().unwrap();
    if !wants_db { return None; }

    let engines = vec![
        "PostgreSQL", "MySQL", "SQLite", "MS SQL Server", "MariaDB",
        "MongoDB", "CouchDB", "Redis", "Cassandra", "DynamoDB", "Neo4j", "Firebase Firestore",
    ];
    let s = Select::with_theme(theme).with_prompt("Database engine?").items(&engines).default(0).interact().unwrap();
    let engine = engines[s].to_string();

    let provider = if engine == "Firebase Firestore" {
        Some("Firebase".to_string())
    } else {
        let provider_options: Vec<&str> = match engine.as_str() {
            "PostgreSQL" => vec!["Self-hosted", "Supabase", "Neon", "AWS RDS", "CockroachDB"],
            "MySQL" => vec!["Self-hosted", "PlanetScale", "AWS RDS"],
            "MariaDB" => vec!["Self-hosted", "AWS RDS"],
            "MS SQL Server" => vec!["Self-hosted", "AWS RDS"],
            "MongoDB" => vec!["Self-hosted", "MongoDB Atlas"],
            _ => vec!["Self-hosted"],
        };
        let s = Select::with_theme(theme).with_prompt("Hosting/provider?").items(&provider_options).default(0).interact().unwrap();
        let choice = provider_options[s].to_string();
        if choice == "Self-hosted" { None } else { Some(choice) }
    };

    let orm = if engine == "Firebase Firestore" { None } else {
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
            let s = Select::with_theme(theme).with_prompt("ORM / query layer?").items(&orm_options).default(0).interact().unwrap();
            Some(orm_options[s].to_string())
        } else { None }
    };

    Some(DatabaseConfig { engine, provider, orm })
}

fn ask_background_jobs(theme: &ColorfulTheme, backend_language: &str) -> Option<String> {
    let wants_jobs = Confirm::with_theme(theme).with_prompt("Do you need background jobs / task queues?").default(false).interact().unwrap();
    if !wants_jobs { return None; }

    let mut options: Vec<&str> = match backend_language {
        "TypeScript" => vec!["BullMQ"],
        "Python" => vec!["Celery"],
        "Go" => vec!["Asynq"],
        _ => vec![],
    };
    options.extend(vec!["Redis (raw)", "RabbitMQ", "Apache Kafka"]);
    let s = Select::with_theme(theme).with_prompt("Background job system?").items(&options).default(0).interact().unwrap();
    Some(options[s].to_string())
}

fn ask_styling(theme: &ColorfulTheme, framework: &str) -> (String, Option<String>) {
    let styling_options = vec![
        "Tailwind CSS", "Plain CSS", "Sass/SCSS", "Bootstrap", "Bulma",
        "Foundation", "Semantic UI", "Materialize", "Pure.css", "UIKit", "Pico.css",
    ];
    let s = Select::with_theme(theme).with_prompt("CSS / styling approach?").items(&styling_options).default(0).interact().unwrap();
    let styling = styling_options[s].to_string();

    let component_options: Vec<&str> = match framework {
        "React" => vec!["None", "Chakra UI", "Radix UI", "Headless UI", "Shadcn/ui"],
        "Vue.js" => vec!["None", "PrimeVue", "Headless UI"],
        _ => vec!["None", "DaisyUI"],
    };
    let s = Select::with_theme(theme).with_prompt("Component library?").items(&component_options).default(0).interact().unwrap();
    let choice = component_options[s].to_string();
    let component_library = if choice == "None" { None } else { Some(choice) };

    (styling, component_library)
}

fn ask_state_management(theme: &ColorfulTheme, framework: &str) -> Option<String> {
    if framework == "None" { return None; }
    let mut options: Vec<&str> = match framework {
        "React" => vec!["Zustand", "Redux Toolkit", "Recoil", "Jotai"],
        "Vue.js" => vec!["Pinia", "Vuex"],
        "Svelte" => vec!["Svelte Stores", "Svelte Runes"],
        _ => vec![],
    };
    options.extend(vec!["MobX", "Nano Stores", "None"]);
    let s = Select::with_theme(theme).with_prompt("State management?").items(&options).default(options.len() - 1).interact().unwrap();
    let choice = options[s].to_string();
    if choice == "None" { None } else { Some(choice) }
}

fn ask_data_fetching(theme: &ColorfulTheme, framework: &str, frontend_language: &str, backend_language: &str) -> Option<String> {
    let mut options = vec!["None", "TanStack Query"];
    if framework == "React" { options.push("SWR"); options.push("RTK Query"); }
    if frontend_language == "TypeScript" && backend_language == "TypeScript" {
        options.push("tRPC");
        options.push("ts-rest");
    }
    let s = Select::with_theme(theme).with_prompt("Data fetching approach?").items(&options).default(0).interact().unwrap();
    let choice = options[s].to_string();
    if choice == "None" { None } else { Some(choice) }
}

fn ask_forms(theme: &ColorfulTheme, framework: &str) -> (Option<String>, Option<String>) {
    let mut form_options: Vec<&str> = match framework {
        "React" => vec!["React Hook Form", "Formik"],
        "Vue.js" => vec!["FormKit"],
        _ => vec![],
    };
    form_options.push("None");
    let s = Select::with_theme(theme).with_prompt("Form library?").items(&form_options).default(form_options.len() - 1).interact().unwrap();
    let choice = form_options[s].to_string();
    let forms = if choice == "None" { None } else { Some(choice) };

    let validator_options = vec!["None", "Zod", "Valibot", "Yup", "TypeBox", "ArkType"];
    let s = Select::with_theme(theme).with_prompt("Schema validator?").items(&validator_options).default(0).interact().unwrap();
    let choice = validator_options[s].to_string();
    let validator = if choice == "None" { None } else { Some(choice) };

    (forms, validator)
}

fn ask_auth(theme: &ColorfulTheme, database: &Option<DatabaseConfig>, meta_framework: &Option<String>) -> Option<String> {
    let mut options = vec!["None"];
    if let Some(db) = database {
        if db.provider.as_deref() == Some("Supabase") { options.push("Supabase Auth"); }
        if db.provider.as_deref() == Some("Firebase") { options.push("Firebase Auth"); }
    }
    if meta_framework.as_deref() == Some("Next.js") { options.push("Auth.js/NextAuth"); }
    options.extend(vec!["Clerk", "Auth0", "Kinde", "Stytch", "Lucia", "Passport.js"]);

    let s = Select::with_theme(theme).with_prompt("Auth provider?").items(&options).default(0).interact().unwrap();
    let choice = options[s].to_string();
    if choice == "None" { None } else { Some(choice) }
}

fn ask_tooling(theme: &ColorfulTheme, frontend_language: &str, backend_language: &str) -> ToolingConfig {
    let js_involved = frontend_language == "TypeScript" || frontend_language == "JavaScript" || backend_language == "TypeScript";

    let mut lint_options: Vec<&str> = vec![];
    if js_involved { lint_options.extend(vec!["ESLint + Prettier", "Biome", "Oxlint"]); }
    match backend_language {
        "Python" => lint_options.extend(vec!["Ruff", "Black + Flake8"]),
        "Go" => lint_options.push("Golangci-lint + Gofmt"),
        "Rust" => lint_options.push("Clippy + Rustfmt"),
        _ => {}
    }
    lint_options.push("None");
    let s = Select::with_theme(theme).with_prompt("Linting/formatting?").items(&lint_options).default(lint_options.len() - 1).interact().unwrap();
    let choice = lint_options[s].to_string();
    let linting = if choice == "None" { None } else { Some(choice) };

    let mut test_options: Vec<&str> = vec![];
    if js_involved { test_options.extend(vec!["Vitest", "Jest", "Playwright", "Cypress"]); }
    match backend_language {
        "Python" => test_options.push("PyTest"),
        "Go" => test_options.push("Go Test"),
        _ => {}
    }
    test_options.push("None");
    let s = Select::with_theme(theme).with_prompt("Testing framework?").items(&test_options).default(test_options.len() - 1).interact().unwrap();
    let choice = test_options[s].to_string();
    let testing = if choice == "None" { None } else { Some(choice) };

    let hook_options: Vec<&str> = if js_involved {
        vec!["Husky + lint-staged", "Lefthook", "None"]
    } else {
        vec!["Lefthook", "simple-git-hooks", "None"]
    };
    let s = Select::with_theme(theme).with_prompt("Git hooks?").items(&hook_options).default(hook_options.len() - 1).interact().unwrap();
    let choice = hook_options[s].to_string();
    let git_hooks = if choice == "None" { None } else { Some(choice) };

    ToolingConfig { linting, testing, git_hooks }
}

fn ask_infra(theme: &ColorfulTheme) -> InfraConfig {
    let container_options = vec!["None", "Docker", "Podman"];
    let s = Select::with_theme(theme).with_prompt("Container tool?").items(&container_options).default(0).interact().unwrap();
    let choice = container_options[s].to_string();
    let container_tool = if choice == "None" { None } else { Some(choice) };

    let hosting_options = vec!["None", "Vercel", "Netlify", "Render", "AWS"];
    let s = Select::with_theme(theme).with_prompt("Hosting target?").items(&hosting_options).default(0).interact().unwrap();
    let choice = hosting_options[s].to_string();
    let hosting = if choice == "None" { None } else { Some(choice) };

    InfraConfig { container_tool, hosting }
}

fn print_summary(config: &ProjectConfig) {
    println!("\n--- Config Summary ---");
    println!("Project name: {}", config.project_name);
    println!("Project type: {}", config.project_type);
    if let Some(fe) = &config.frontend {
        println!("\nFrontend: {} / {:?} / {} / {:?}", fe.framework, fe.meta_framework, fe.styling, fe.component_library);
        println!("  State: {:?}  Data fetching: {:?}  Forms: {:?}  Validator: {:?}", fe.state_management, fe.data_fetching, fe.forms, fe.validator);
    }
    if let Some(be) = &config.backend { println!("\nBackend: {}", be.framework); }
    match &config.database {
        Some(db) => println!("\nDatabase: {} / {:?} / {:?}", db.engine, db.provider, db.orm),
        None => println!("\nDatabase: None"),
    }
    println!("Background jobs: {:?}", config.background_jobs);
    println!("Auth: {:?}", config.auth);
    println!("Tooling: {:?} / {:?} / {:?}", config.tooling.linting, config.tooling.testing, config.tooling.git_hooks);
    println!("Infra: {:?} / {:?}", config.infra.container_tool, config.infra.hosting);
    println!("Install dependencies: {}\n", config.install_dependencies);
}

fn readme_content(config: &ProjectConfig) -> String {
    format!("# {}\n\nGenerated with Chaos.\n", config.project_name)
}

// ---------- Static-site CSS/JS contributors ----------

fn add_html_boilerplate(plan: &mut BuildPlan, frontend: &FrontendConfig, project_name: &str) {
    let css_link = match frontend.styling.as_str() {
        "Tailwind CSS" | "Sass/SCSS" => "<link rel=\"stylesheet\" href=\"dist/output.css\">".to_string(),
        "Bootstrap" => "<link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/bootstrap@5/dist/css/bootstrap.min.css\">".to_string(),
        "Bulma" => "<link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/bulma@1/css/bulma.min.css\">".to_string(),
        "Foundation" => "<link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/foundation-sites@6.9.0/dist/css/foundation.min.css\">".to_string(),
        "Semantic UI" => "<link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/semantic-ui@2.5.0/dist/semantic.min.css\">".to_string(),
        "Materialize" => "<link rel=\"stylesheet\" href=\"https://cdnjs.cloudflare.com/ajax/libs/materialize/1.0.0/css/materialize.min.css\">".to_string(),
        "Pure.css" => "<link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/purecss@3/build/pure-min.css\">".to_string(),
        "UIKit" => "<link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/uikit@3/dist/css/uikit.min.css\">".to_string(),
        "Pico.css" => "<link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css\">".to_string(),
        _ => "<link rel=\"stylesheet\" href=\"styles/style.css\">".to_string(),
    };

    let extra_script = if frontend.styling == "UIKit" {
        "<script src=\"https://cdn.jsdelivr.net/npm/uikit@3/dist/js/uikit.min.js\"></script>\n    <script src=\"https://cdn.jsdelivr.net/npm/uikit@3/dist/js/uikit-icons.min.js\"></script>"
    } else if frontend.styling == "Materialize" {
        "<script src=\"https://cdnjs.cloudflare.com/ajax/libs/materialize/1.0.0/js/materialize.min.js\"></script>"
    } else if frontend.styling == "Foundation" {
        "<script src=\"https://cdn.jsdelivr.net/npm/foundation-sites@6.9.0/dist/js/foundation.min.js\"></script>"
    } else if frontend.styling == "Semantic UI" {
        "<script src=\"https://cdn.jsdelivr.net/npm/jquery@3/dist/jquery.min.js\"></script>\n    <script src=\"https://cdn.jsdelivr.net/npm/semantic-ui@2.5.0/dist/semantic.min.js\"></script>"
    } else { "" };

    let script_tag = if frontend.include_js { "<script src=\"script.js\"></script>" } else { "" };

    let html = format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <title>{}</title>\n    {}\n    {}\n</head>\n<body>\n    <h1>Welcome to {}</h1>\n    {}\n</body>\n</html>",
        project_name, css_link, extra_script, project_name, script_tag
    );
    plan.files.push(("index.html".to_string(), html));
}

fn add_tailwind(plan: &mut BuildPlan, project_name: &str) {
    plan.files.push(("package.json".to_string(), format!("{{\n  \"name\": \"{}\",\n  \"scripts\": {{ \"build\": \"tailwindcss -i src/input.css -o dist/output.css\" }}\n}}", project_name)));
    plan.files.push(("tailwind.config.js".to_string(), "module.exports = {\n  content: [\"./index.html\"],\n  theme: { extend: {} },\n  plugins: [],\n};".to_string()));
    plan.files.push(("src/input.css".to_string(), "@tailwind base;\n@tailwind components;\n@tailwind utilities;".to_string()));
    plan.gitignore_entries.push("node_modules/".to_string());
    plan.gitignore_entries.push("dist/".to_string());
    plan.npm_dev_dependencies.push("tailwindcss".to_string());
}

fn add_sass(plan: &mut BuildPlan, project_name: &str) {
    plan.files.push(("package.json".to_string(), format!("{{\n  \"name\": \"{}\",\n  \"scripts\": {{ \"build\": \"sass src/input.scss dist/output.css\" }}\n}}", project_name)));
    plan.files.push(("src/input.scss".to_string(), "// Your Sass/SCSS here\nbody {\n  font-family: sans-serif;\n}".to_string()));
    plan.gitignore_entries.push("node_modules/".to_string());
    plan.gitignore_entries.push("dist/".to_string());
    plan.npm_dev_dependencies.push("sass".to_string());
}

fn add_plain_css(plan: &mut BuildPlan) {
    plan.files.push(("styles/style.css".to_string(), "/* Your styles here */".to_string()));
}

fn add_javascript(plan: &mut BuildPlan) {
    plan.files.push(("script.js".to_string(), "// Your JavaScript here".to_string()));
}

fn generate_frontend_plan(frontend: &FrontendConfig, project_name: &str) -> BuildPlan {
    let mut plan = BuildPlan::new();
    add_html_boilerplate(&mut plan, frontend, project_name);
    match frontend.styling.as_str() {
        "Tailwind CSS" => add_tailwind(&mut plan, project_name),
        "Sass/SCSS" => add_sass(&mut plan, project_name),
        // Bootstrap and all other CDN-based frameworks need no build step or local files.
        "Bootstrap" | "Bulma" | "Foundation" | "Semantic UI" | "Materialize" | "Pure.css" | "UIKit" | "Pico.css" => {}
        _ => add_plain_css(&mut plan),
    }
    if frontend.include_js { add_javascript(&mut plan); }
    plan
}

fn generate_static_webpage(config: &ProjectConfig) {
    let frontend = config.frontend.as_ref().unwrap();
    let mut plan = generate_frontend_plan(frontend, &config.project_name);
    plan.files.push(("README.md".to_string(), readme_content(config)));
    execute_plan(&config.project_name, &plan);
    if config.install_dependencies && !plan.npm_dev_dependencies.is_empty() {
        run_npm_install(&config.project_name, &[], &plan.npm_dev_dependencies);
    }
}

// ---------- JS/TS addon registry (state mgmt, forms, validators, data fetching,
// component libraries, linters, git hooks, testing, auth) ----------
// Each entry: (dependencies, dev_dependencies, files as (path, content))
// File content deliberately avoids TS-only syntax (no `interface`, no type
// annotations) so the same snippet is valid whether it lands in a .ts or .js
// file — extension is chosen by the caller based on frontend_language.
// These write a self-contained starter file rather than editing the
// scaffolded entry point, since the exact entry file varies by scaffolder
// version and framework — wiring it in is left to the user.

fn js_snippet(choice: &str, e: &str) -> Option<(Vec<&'static str>, Vec<&'static str>, Vec<(String, String)>)> {
    match choice {
        "Zustand" => Some((vec!["zustand"], vec![], vec![(format!("src/store.{}", e),
            "import { create } from 'zustand';\n\nexport const useAppStore = create((set) => ({\n  count: 0,\n  increment: () => set((s) => ({ count: s.count + 1 })),\n}));\n".to_string())])),
        "Redux Toolkit" => Some((vec!["@reduxjs/toolkit", "react-redux"], vec![], vec![(format!("src/store.{}", e),
            "import { configureStore, createSlice } from '@reduxjs/toolkit';\n\nconst counterSlice = createSlice({\n  name: 'counter',\n  initialState: { value: 0 },\n  reducers: {\n    increment: (state) => { state.value += 1; },\n  },\n});\n\nexport const { increment } = counterSlice.actions;\nexport const store = configureStore({ reducer: { counter: counterSlice.reducer } });\n".to_string())])),
        "Recoil" => Some((vec!["recoil"], vec![], vec![(format!("src/state.{}", e),
            "import { atom } from 'recoil';\n\nexport const countState = atom({\n  key: 'countState',\n  default: 0,\n});\n".to_string())])),
        "Jotai" => Some((vec!["jotai"], vec![], vec![(format!("src/atoms.{}", e),
            "import { atom } from 'jotai';\n\nexport const countAtom = atom(0);\n".to_string())])),
        "MobX" => Some((vec!["mobx", "mobx-react-lite"], vec![], vec![(format!("src/store.{}", e),
            "import { makeAutoObservable } from 'mobx';\n\nclass CounterStore {\n  count = 0;\n  constructor() { makeAutoObservable(this); }\n  increment() { this.count += 1; }\n}\n\nexport const counterStore = new CounterStore();\n".to_string())])),
        "Pinia" => Some((vec!["pinia"], vec![], vec![(format!("src/store.{}", e),
            "import { defineStore } from 'pinia';\n\nexport const useCounterStore = defineStore('counter', {\n  state: () => ({ count: 0 }),\n  actions: {\n    increment() { this.count += 1; },\n  },\n});\n".to_string())])),
        "Vuex" => Some((vec!["vuex"], vec![], vec![(format!("src/store.{}", e),
            "import { createStore } from 'vuex';\n\nexport const store = createStore({\n  state: { count: 0 },\n  mutations: {\n    increment(state) { state.count += 1; },\n  },\n});\n".to_string())])),
        "Svelte Stores" => Some((vec![], vec![], vec![(format!("src/lib/store.{}", e),
            "import { writable } from 'svelte/store';\n\nexport const count = writable(0);\n".to_string())])),
        "Svelte Runes" => Some((vec![], vec![], vec![("src/lib/store.svelte.ts".to_string(),
            "export const counter = $state({ value: 0 });\n".to_string())])),
        "Nano Stores" => Some((vec!["nanostores"], vec![], vec![(format!("src/store.{}", e),
            "import { atom } from 'nanostores';\n\nexport const count = atom(0);\n".to_string())])),

        "React Hook Form" => Some((vec!["react-hook-form"], vec![], vec![(format!("src/hooks/useContactForm.{}", if e == "ts" { "tsx" } else { "jsx" }),
            "import { useForm } from 'react-hook-form';\n\nexport function useContactForm() {\n  return useForm({ defaultValues: { name: '', email: '' } });\n}\n".to_string())])),
        "Formik" => Some((vec!["formik"], vec![], vec![(format!("src/forms/contactForm.{}", e),
            "import { useFormik } from 'formik';\n\nexport function useContactForm() {\n  return useFormik({\n    initialValues: { name: '', email: '' },\n    onSubmit: (values) => console.log(values),\n  });\n}\n".to_string())])),
        "FormKit" => Some((vec!["@formkit/vue"], vec![], vec![("src/forms/contactForm.js".to_string(),
            "// Register FormKit in your main entry file: app.use(plugin, defaultConfig)\n// then use <FormKit type=\"form\"> in a component.\n".to_string())])),

        "Zod" => Some((vec!["zod"], vec![], vec![(format!("src/schemas/contact.{}", e),
            "import { z } from 'zod';\n\nexport const contactSchema = z.object({\n  name: z.string().min(1),\n  email: z.string().email(),\n});\n".to_string())])),
        "Valibot" => Some((vec!["valibot"], vec![], vec![(format!("src/schemas/contact.{}", e),
            "import * as v from 'valibot';\n\nexport const contactSchema = v.object({\n  name: v.string(),\n  email: v.pipe(v.string(), v.email()),\n});\n".to_string())])),
        "Yup" => Some((vec!["yup"], vec![], vec![(format!("src/schemas/contact.{}", e),
            "import * as yup from 'yup';\n\nexport const contactSchema = yup.object({\n  name: yup.string().required(),\n  email: yup.string().email().required(),\n});\n".to_string())])),
        "TypeBox" => Some((vec!["@sinclair/typebox"], vec![], vec![(format!("src/schemas/contact.{}", e),
            "import { Type } from '@sinclair/typebox';\n\nexport const ContactSchema = Type.Object({\n  name: Type.String(),\n  email: Type.String({ format: 'email' }),\n});\n".to_string())])),
        "ArkType" => Some((vec!["arktype"], vec![], vec![(format!("src/schemas/contact.{}", e),
            "import { type } from 'arktype';\n\nexport const ContactSchema = type({\n  name: 'string',\n  email: 'string.email',\n});\n".to_string())])),

        "TanStack Query" => Some((vec!["@tanstack/react-query"], vec![], vec![(format!("src/queryClient.{}", e),
            "import { QueryClient } from '@tanstack/react-query';\n\nexport const queryClient = new QueryClient();\n// Wrap your app in <QueryClientProvider client={queryClient}>\n".to_string())])),
        "SWR" => Some((vec!["swr"], vec![], vec![(format!("src/hooks/useApi.{}", e),
            "import useSWR from 'swr';\n\nconst fetcher = (url) => fetch(url).then((r) => r.json());\n\nexport function useApi(url) {\n  return useSWR(url, fetcher);\n}\n".to_string())])),
        "RTK Query" => Some((vec!["@reduxjs/toolkit"], vec![], vec![(format!("src/services/api.{}", e),
            "import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react';\n\nexport const api = createApi({\n  reducerPath: 'api',\n  baseQuery: fetchBaseQuery({ baseUrl: '/api' }),\n  endpoints: (builder) => ({}),\n});\n".to_string())])),

        "Radix UI" => Some((vec!["@radix-ui/react-dialog"], vec![], vec![])),
        "Headless UI" => Some((vec!["@headlessui/react"], vec![], vec![])),
        "Chakra UI" => Some((vec!["@chakra-ui/react", "@emotion/react", "@emotion/styled", "framer-motion"], vec![], vec![(format!("src/chakra.{}", if e == "ts" { "tsx" } else { "jsx" }),
            "// Wrap your app root in <ChakraProvider> from @chakra-ui/react\n".to_string())])),
        "PrimeVue" => Some((vec!["primevue"], vec![], vec![])),
        "DaisyUI" => Some((vec![], vec!["daisyui"], vec![])),

        "Passport.js" => Some((vec!["passport", "passport-local"], vec![], vec![(format!("src/auth/passport.{}", e),
            "import passport from 'passport';\nimport { Strategy as LocalStrategy } from 'passport-local';\n\npassport.use(new LocalStrategy((username, password, done) => {\n  // TODO: look up the user and verify the password\n  return done(null, false);\n}));\n\nexport default passport;\n".to_string())])),
        "Clerk" => Some((vec!["@clerk/clerk-react"], vec![], vec![(".env.example".to_string(),
            "VITE_CLERK_PUBLISHABLE_KEY=\n".to_string())])),
        "Kinde" => Some((vec!["@kinde-oss/kinde-auth-react"], vec![], vec![(".env.example".to_string(),
            "VITE_KINDE_CLIENT_ID=\nVITE_KINDE_DOMAIN=\nVITE_KINDE_REDIRECT_URL=\n".to_string())])),
        "Stytch" => Some((vec!["@stytch/react"], vec![], vec![(".env.example".to_string(),
            "VITE_STYTCH_PUBLIC_TOKEN=\n".to_string())])),
        "Lucia" => Some((vec!["lucia"], vec![], vec![(format!("src/auth/lucia.{}", e),
            "// Configure Lucia with your adapter for the chosen database.\n// See https://lucia-auth.com for the adapter matching your ORM.\n".to_string())])),
        "Auth0" => Some((vec!["@auth0/auth0-react"], vec![], vec![(".env.example".to_string(),
            "VITE_AUTH0_DOMAIN=\nVITE_AUTH0_CLIENT_ID=\n".to_string())])),
        "Auth.js/NextAuth" => Some((vec!["next-auth"], vec![], vec![("src/app/api/auth/[...nextauth]/route.js".to_string(),
            "import NextAuth from 'next-auth';\n\nconst handler = NextAuth({ providers: [] });\nexport { handler as GET, handler as POST };\n".to_string())])),
        "Supabase Auth" => Some((vec!["@supabase/supabase-js"], vec![], vec![(".env.example".to_string(),
            "VITE_SUPABASE_URL=\nVITE_SUPABASE_ANON_KEY=\n".to_string())])),

        _ => None,
    }
}

/// Applies every JS/TS addon selected in the config to a scaffolded frontend
/// project. Only called after the base scaffold command succeeds. Installs
/// real deps and drops in a starter file per addon; never claims success if
/// npm install fails.
fn apply_frontend_addons(frontend_folder_abs: &Path, config: &ProjectConfig) {
    let e = ext(config);
    let fe = config.frontend.as_ref().unwrap();
    let mut deps: Vec<String> = vec![];
    let mut dev_deps: Vec<String> = vec![];
    let mut files: Vec<(String, String)> = vec![];

    let mut choices: Vec<&str> = vec![];
    if let Some(v) = &fe.state_management { choices.push(v); }
    if let Some(v) = &fe.data_fetching { choices.push(v); }
    if let Some(v) = &fe.forms { choices.push(v); }
    if let Some(v) = &fe.validator { choices.push(v); }
    if let Some(v) = &fe.component_library { choices.push(v); }
    if let Some(v) = &config.auth { choices.push(v); }

    for choice in choices {
        if let Some((d, dd, f)) = js_snippet(choice, e) {
            deps.extend(d.into_iter().map(String::from));
            dev_deps.extend(dd.into_iter().map(String::from));
            files.extend(f);
        }
    }

    // Linting
    match config.tooling.linting.as_deref() {
        Some("ESLint + Prettier") => {
            dev_deps.push("eslint".to_string());
            dev_deps.push("prettier".to_string());
            files.push((".prettierrc".to_string(), "{\n  \"semi\": true,\n  \"singleQuote\": true\n}\n".to_string()));
            files.push((".eslintrc.json".to_string(), "{\n  \"extends\": [\"eslint:recommended\"]\n}\n".to_string()));
        }
        Some("Biome") => {
            dev_deps.push("@biomejs/biome".to_string());
            files.push(("biome.json".to_string(), "{\n  \"$schema\": \"https://biomejs.dev/schemas/1.8.0/schema.json\",\n  \"formatter\": { \"enabled\": true },\n  \"linter\": { \"enabled\": true }\n}\n".to_string()));
        }
        Some("Oxlint") => {
            dev_deps.push("oxlint".to_string());
            files.push((".oxlintrc.json".to_string(), "{\n  \"rules\": {}\n}\n".to_string()));
        }
        _ => {}
    }

    // Testing
    match config.tooling.testing.as_deref() {
        Some("Vitest") => { dev_deps.push("vitest".to_string()); }
        Some("Jest") => { dev_deps.push("jest".to_string()); }
        Some("Playwright") => { dev_deps.push("@playwright/test".to_string()); }
        Some("Cypress") => { dev_deps.push("cypress".to_string()); }
        _ => {}
    }

    // Git hooks
    match config.tooling.git_hooks.as_deref() {
        Some("Husky + lint-staged") => {
            dev_deps.push("husky".to_string());
            dev_deps.push("lint-staged".to_string());
            files.push((".husky/pre-commit".to_string(), "#!/bin/sh\nnpx lint-staged\n".to_string()));
        }
        Some("Lefthook") => {
            dev_deps.push("lefthook".to_string());
            files.push(("lefthook.yml".to_string(), "pre-commit:\n  commands:\n    lint:\n      run: npx eslint .\n".to_string()));
        }
        Some("simple-git-hooks") => {
            dev_deps.push("simple-git-hooks".to_string());
            files.push(("simple-git-hooks.json".to_string(), "{\n  \"pre-commit\": \"npx eslint .\"\n}\n".to_string()));
        }
        _ => {}
    }

    for (path, content) in &files {
        let full_path = frontend_folder_abs.join(path);
        if let Some(parent) = full_path.parent() { let _ = fs::create_dir_all(parent); }
        fs::write(&full_path, content).ok();
        println!("Created: {}", full_path.display());
    }

    // Shadcn/ui uses its own real CLI (Pattern A), not just a package install.
    if fe.component_library.as_deref() == Some("Shadcn/ui") {
        if require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") {
            let status = Command::new("npx").arg("--yes").arg("shadcn@latest").arg("init").arg("-d")
                .current_dir(frontend_folder_abs).status();
            match status {
                Ok(s) if s.success() => println!("Initialized shadcn/ui"),
                _ => println!("shadcn/ui init failed — you can run 'npx shadcn@latest init' manually"),
            }
        }
    }

    if !deps.is_empty() || !dev_deps.is_empty() {
        run_npm_install(frontend_folder_abs.to_str().unwrap(), &deps, &dev_deps);
    }
}

fn generate_container_files(config: &ProjectConfig) -> Vec<(String, String)> {
    let Some(_) = &config.infra.container_tool else { return vec![]; };
    let dockerfile = "# Generated by Chaos — generic starter, adjust for your actual stack.\nFROM node:20-alpine\nWORKDIR /app\nCOPY . .\nRUN npm install\nCMD [\"npm\", \"start\"]\n".to_string();
    let compose = "# Generated by Chaos — generic starter, adjust for your actual stack.\nservices:\n  app:\n    build: .\n    ports:\n      - \"3000:3000\"\n".to_string();
    vec![("Dockerfile".to_string(), dockerfile), ("docker-compose.yml".to_string(), compose)]
}

fn generate_hosting_files(config: &ProjectConfig) -> Vec<(String, String)> {
    match config.infra.hosting.as_deref() {
        Some("Render") => vec![("render.yaml".to_string(),
            "services:\n  - type: web\n    name: app\n    env: node\n    buildCommand: npm install && npm run build\n    startCommand: npm start\n".to_string())],
        Some("AWS") => vec![("AWS_DEPLOY.md".to_string(),
            "# AWS deployment notes\n\nAWS covers many services (Amplify, Elastic Beanstalk, ECS, App Runner).\nChaos doesn't provision AWS resources for you — pick a target and consult\nits own getting-started docs; connect your AWS credentials via `aws configure`.\n".to_string())],
        _ => vec![],
    }
}

fn generate_database_files(config: &ProjectConfig) -> Vec<(String, String)> {
    let Some(db) = &config.database else { return vec![]; };
    let conn = match db.engine.as_str() {
        "PostgreSQL" => "postgresql://user:password@localhost:5432/dbname",
        "MySQL" => "mysql://user:password@localhost:3306/dbname",
        "MariaDB" => "mysql://user:password@localhost:3306/dbname",
        "MS SQL Server" => "sqlserver://localhost:1433;database=dbname;user=sa;password=password",
        "SQLite" => "file:./dev.db",
        "MongoDB" => "mongodb://localhost:27017/dbname",
        "Redis" => "redis://localhost:6379",
        _ => "",
    };
    let provider_note = match db.provider.as_deref() {
        Some("Supabase") => "\n# Supabase: replace with the connection string from your project's Settings > Database.",
        Some("PlanetScale") => "\n# PlanetScale: replace with the connection string from your database's Connect panel.",
        Some("Neon") => "\n# Neon: replace with the connection string from your project's Dashboard.",
        Some("MongoDB Atlas") => "\n# MongoDB Atlas: replace with the connection string from Database > Connect.",
        Some("AWS RDS") => "\n# AWS RDS: replace host/user/password with your RDS instance's endpoint and credentials.",
        _ => "",
    };
    if conn.is_empty() { return vec![]; }
    vec![(".env.example".to_string(), format!("DATABASE_URL={}{}\n", conn, provider_note))]
}

fn generate_background_job_files(config: &ProjectConfig) -> Option<(String, String)> {
    match config.background_jobs.as_deref() {
        Some("BullMQ") => Some(("src/worker.js".to_string(),
            "import { Worker, Queue } from 'bullmq';\n\nconst connection = { host: 'localhost', port: 6379 };\nexport const jobQueue = new Queue('jobs', { connection });\n\nnew Worker('jobs', async (job) => {\n  console.log('processing', job.name, job.data);\n}, { connection });\n".to_string())),
        Some("Celery") => Some(("worker.py".to_string(),
            "from celery import Celery\n\napp = Celery('worker', broker='redis://localhost:6379/0')\n\n@app.task\ndef example_task(x, y):\n    return x + y\n".to_string())),
        Some("Asynq") => Some(("worker/main.go".to_string(),
            "package main\n\nimport \"github.com/hibiken/asynq\"\n\nfunc main() {\n\tsrv := asynq.NewServer(\n\t\tasynq.RedisClientOpt{Addr: \"localhost:6379\"},\n\t\tasynq.Config{Concurrency: 10},\n\t)\n\tmux := asynq.NewServeMux()\n\t_ = srv.Run(mux)\n}\n".to_string())),
        Some("Redis (raw)") => Some((".env.example".to_string(), "REDIS_URL=redis://localhost:6379\n".to_string())),
        _ => None,
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
    fs::write(format!("{}/.gitkeep", docs_folder), "").ok();

    let frontend_cfg = config.frontend.as_ref().unwrap();
    let frontend_folder = format!("{}/frontend", config.project_name);
    let mut root_gitignore_entries: Vec<String> = Vec::new();

    if frontend_cfg.framework == "None" {
        let frontend_plan = generate_frontend_plan(frontend_cfg, &config.project_name);
        execute_plan(&frontend_folder, &frontend_plan);
        if config.install_dependencies && !frontend_plan.npm_dev_dependencies.is_empty() {
            run_npm_install(&frontend_folder, &frontend_plan.npm_dependencies, &frontend_plan.npm_dev_dependencies);
        }
        root_gitignore_entries.extend(frontend_plan.gitignore_entries.iter().map(|e| format!("frontend/{}", e)));
    } else if config.install_dependencies {
        let frontend_entries = generate_frontend(config);
        root_gitignore_entries.extend(frontend_entries);
    } else {
        fs::create_dir_all(&frontend_folder).ok();
        println!("\n Frontend wasn't scaffolded — dependency installation is off.");
    }

    if config.install_dependencies {
        let backend_entries = generate_backend(config);
        root_gitignore_entries.extend(backend_entries);
    } else {
        fs::create_dir_all(format!("{}/backend", config.project_name)).ok();
        println!("\n Backend wasn't scaffolded — dependency installation is off.");
    }

    for (path, content) in generate_container_files(config) {
        let full_path = format!("{}/{}", config.project_name, path);
        fs::write(&full_path, content).ok();
        println!("Created: {}", full_path);
    }
    for (path, content) in generate_hosting_files(config) {
        let full_path = format!("{}/{}", config.project_name, path);
        fs::write(&full_path, content).ok();
        println!("Created: {}", full_path);
    }
    for (path, content) in generate_database_files(config) {
        let full_path = format!("{}/{}", config.project_name, path);
        fs::write(&full_path, content).ok();
        println!("Created: {}", full_path);
    }
    if let Some((path, content)) = generate_background_job_files(config) {
        let full_path = format!("{}/backend/{}", config.project_name, path);
        if let Some(parent) = Path::new(&full_path).parent() { fs::create_dir_all(parent).ok(); }
        fs::write(&full_path, content).ok();
        println!("Created: {}", full_path);
    }

    fs::write(format!("{}/README.md", config.project_name), readme_content(config)).ok();
    println!("Created: {}/README.md", config.project_name);

    if !root_gitignore_entries.is_empty() {
        let content = root_gitignore_entries.join("\n");
        fs::write(format!("{}/.gitignore", config.project_name), content).ok();
        println!("Created: {}/.gitignore", config.project_name);
    }

    let mut still_not_generated: Vec<&str> = vec![];
    if let Some(db) = &config.database {
        if !matches!(db.orm.as_deref(), Some("Prisma") | Some("Drizzle") | Some("SQLAlchemy") | Some("Mongoose") | Some("TypeORM") | Some("Sequelize") | Some("Kysely") | Some("SQLModel") | Some("Tortoise ORM")) && db.orm.is_some() {
            still_not_generated.push("ORM (this specific one isn't wired up yet)");
        }
    }
    if !still_not_generated.is_empty() {
        println!("\nNote: {} — captured in config, not generated.", still_not_generated.join(", "));
    }
}

// ---------- Frontend dispatcher ----------

fn generate_frontend(config: &ProjectConfig) -> Vec<String> {
    let frontend_folder = format!("{}/frontend", config.project_name);
    fs::create_dir_all(&frontend_folder).expect("Failed to create frontend folder");
    let frontend = config.frontend.as_ref().unwrap();
    let frontend_folder_abs = fs::canonicalize(&frontend_folder).expect("Failed to resolve frontend folder path");
    let ts = config.frontend_language.as_deref() == Some("TypeScript");

    println!("\n Setting up {} frontend...", frontend.meta_framework.as_deref().unwrap_or(&frontend.framework));

    let succeeded;
    let mut gitignore: Vec<String>;

    match frontend.meta_framework.as_deref() {
        Some("Next.js") => { succeeded = generate_nextjs_frontend(&frontend_folder_abs, ts); gitignore = vec!["frontend/node_modules/".to_string(), "frontend/.next/".to_string()]; }
        Some("Nuxt") => { succeeded = generate_nuxt_frontend(&frontend_folder_abs); gitignore = vec!["frontend/node_modules/".to_string(), "frontend/.nuxt/".to_string(), "frontend/.output/".to_string()]; }
        Some("SvelteKit") => { succeeded = generate_sveltekit_frontend(&frontend_folder_abs); gitignore = vec!["frontend/node_modules/".to_string(), "frontend/.svelte-kit/".to_string(), "frontend/build/".to_string()]; }
        Some(other) => { println!("Meta-framework {} isn't built yet — captured in config only.", other); return vec![]; }
        None => {
            match frontend.framework.as_str() {
                "React" => { succeeded = generate_vite_frontend(&frontend_folder_abs, if ts {"react-ts"} else {"react"}); gitignore = vec!["frontend/node_modules/".to_string(), "frontend/dist/".to_string()]; }
                "Vue.js" => { succeeded = generate_vite_frontend(&frontend_folder_abs, if ts {"vue-ts"} else {"vue"}); gitignore = vec!["frontend/node_modules/".to_string(), "frontend/dist/".to_string()]; }
                "Svelte" => { succeeded = generate_vite_frontend(&frontend_folder_abs, if ts {"svelte-ts"} else {"svelte"}); gitignore = vec!["frontend/node_modules/".to_string(), "frontend/dist/".to_string()]; }
                "Preact" => { succeeded = generate_vite_frontend(&frontend_folder_abs, if ts {"preact-ts"} else {"preact"}); gitignore = vec!["frontend/node_modules/".to_string(), "frontend/dist/".to_string()]; }
                "SolidJS" => { succeeded = generate_vite_frontend(&frontend_folder_abs, if ts {"solid-ts"} else {"solid"}); gitignore = vec!["frontend/node_modules/".to_string(), "frontend/dist/".to_string()]; }
                "Angular" => { succeeded = generate_angular_frontend(&frontend_folder_abs, &config.project_name); gitignore = vec!["frontend/node_modules/".to_string(), "frontend/dist/".to_string(), "frontend/.angular/".to_string()]; }
                other => { println!("Frontend framework {} isn't built yet — captured in config only.", other); return vec![]; }
            }
        }
    }

    if succeeded {
        apply_frontend_addons(&frontend_folder_abs, config);
    } else {
        println!("Base scaffold failed — skipping addon installation to avoid a broken partial state.");
    }
    gitignore
}

fn generate_vite_frontend(frontend_folder_abs: &Path, template: &str) -> bool {
    if !require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") { return false; }
    let status = Command::new("npx").arg("--yes").arg("create-vite@latest").arg(".").arg("--template").arg(template)
        .arg("--no-interactive").arg("--no-immediate").current_dir(frontend_folder_abs).status().expect("Failed to run create-vite");
    if !status.success() { println!("create-vite failed"); return false; }
    println!("Generated Vite + {} project", template);
    let status = Command::new("npm").arg("install").current_dir(frontend_folder_abs).status().expect("Failed to run npm install");
    if status.success() { println!("Installed frontend dependencies"); } else { println!("npm install failed"); }
    status.success()
}

fn generate_angular_frontend(frontend_folder_abs: &Path, project_name: &str) -> bool {
    if !require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") { return false; }
    let status = Command::new("npx").arg("--yes").arg("@angular/cli@latest").arg("new").arg(project_name)
        .arg("--directory").arg(".").arg("--skip-git").arg("--defaults").current_dir(frontend_folder_abs).status().expect("Failed to run ng new");
    if status.success() { println!("Generated Angular project"); } else { println!("ng new failed"); }
    status.success()
}

fn generate_nextjs_frontend(frontend_folder_abs: &Path, typescript: bool) -> bool {
    if !require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") { return false; }
    let mut cmd = Command::new("npx");
    cmd.arg("--yes").arg("create-next-app@latest").arg(".").arg("--eslint").arg("--tailwind").arg("--app")
        .arg("--no-src-dir").arg("--import-alias").arg("@/*").arg("--use-npm").arg("--yes");
    if typescript { cmd.arg("--ts"); } else { cmd.arg("--js"); }
    let status = cmd.current_dir(frontend_folder_abs).status().expect("Failed to run create-next-app");
    if status.success() { println!("Generated Next.js project"); } else { println!("create-next-app failed"); }
    status.success()
}

fn generate_nuxt_frontend(frontend_folder_abs: &Path) -> bool {
    if !require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") { return false; }
    let status = Command::new("npx").arg("--yes").arg("nuxi@latest").arg("init").arg(".").arg("--force")
        .arg("--packageManager").arg("npm").current_dir(frontend_folder_abs).status().expect("Failed to run nuxi init");
    if !status.success() { println!("nuxi init failed"); return false; }
    println!("Generated Nuxt project");
    let status = Command::new("npm").arg("install").current_dir(frontend_folder_abs).status().expect("Failed to run npm install");
    if status.success() { println!("Installed frontend dependencies"); } else { println!("npm install failed"); }
    status.success()
}

fn generate_sveltekit_frontend(frontend_folder_abs: &Path) -> bool {
    if !require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") { return false; }
    let status = Command::new("npx").arg("--yes").arg("sv").arg("create").arg(".").arg("--template").arg("minimal")
        .arg("--types").arg("ts").arg("--no-add-ons").current_dir(frontend_folder_abs).status().expect("Failed to run sv create");
    if status.success() { println!("Generated SvelteKit project"); } else { println!("sv create failed"); }
    status.success()
}

// ---------- Backend dispatcher ----------

fn generate_backend(config: &ProjectConfig) -> Vec<String> {
    let backend_folder = format!("{}/backend", config.project_name);
    fs::create_dir_all(&backend_folder).expect("Failed to create backend folder");
    let backend_folder_abs = fs::canonicalize(&backend_folder).expect("Failed to resolve backend folder path");
    let language = config.backend_language.as_deref();
    let framework = config.backend.as_ref().map(|b| b.framework.as_str());

    println!("\n Setting up {} backend ({})...", language.unwrap_or("?"), framework.unwrap_or("?"));

    let (succeeded, mut gitignore) = match (language, framework) {
        (Some("Python"), Some("Django")) => (generate_django_backend(&backend_folder_abs), vec!["backend/venv/".to_string(), "backend/__pycache__/".to_string()]),
        (Some("Python"), Some("Flask")) => (generate_flask_backend(&backend_folder_abs, config), vec!["backend/venv/".to_string(), "backend/__pycache__/".to_string()]),
        (Some("TypeScript"), Some("Express")) => (generate_express_backend(&backend_folder_abs), vec!["backend/node_modules/".to_string()]),
        (Some("TypeScript"), Some("Fastify")) => (generate_fastify_backend(&backend_folder_abs), vec!["backend/node_modules/".to_string()]),
        (Some("TypeScript"), Some("NestJS")) => (generate_nestjs_backend(&backend_folder_abs), vec!["backend/node_modules/".to_string(), "backend/dist/".to_string()]),
        (Some("TypeScript"), Some("Hono")) => (generate_hono_backend(&backend_folder_abs), vec!["backend/node_modules/".to_string()]),
        (Some("TypeScript"), Some("Elysia")) => (generate_elysia_backend(&backend_folder_abs), vec!["backend/node_modules/".to_string()]),
        (Some("Ruby"), Some("Rails")) => (generate_rails_backend(&backend_folder_abs), vec!["backend/log/".to_string(), "backend/tmp/".to_string()]),
        (Some("PHP"), Some("Laravel")) => (generate_laravel_backend(&backend_folder_abs), vec!["backend/vendor/".to_string(), "backend/.env".to_string()]),
        (Some("Go"), Some("Gin")) => (generate_go_backend(&backend_folder_abs, &config.project_name, "github.com/gin-gonic/gin", gin_main()), vec![]),
        (Some("Go"), Some("Fiber")) => (generate_go_backend(&backend_folder_abs, &config.project_name, "github.com/gofiber/fiber/v2", fiber_main()), vec![]),
        (Some("Go"), Some("Chi")) => (generate_go_backend(&backend_folder_abs, &config.project_name, "github.com/go-chi/chi/v5", chi_main()), vec![]),
        (Some("Go"), Some("Echo")) => (generate_go_backend(&backend_folder_abs, &config.project_name, "github.com/labstack/echo/v4", echo_main()), vec![]),
        _ => {
            println!("Backend combination ({:?}, {:?}) isn't built yet — captured in config only.", language, framework);
            (false, vec![])
        }
    };

    if succeeded {
        apply_backend_addons(&backend_folder_abs, config, language.unwrap_or(""));
    }
    gitignore
}

/// Python/Go post-scaffold addons: ORM, linter, testing, mirroring apply_frontend_addons
/// for the non-JS side.
fn apply_backend_addons(backend_folder_abs: &Path, config: &ProjectConfig, language: &str) {
    if language == "Python" {
        let pip_path = backend_folder_abs.join("venv/bin/pip");
        let mut pkgs: Vec<&str> = vec![];
        if let Some(db) = &config.database {
            match db.orm.as_deref() {
                Some("SQLAlchemy") => pkgs.push("sqlalchemy"),
                Some("SQLModel") => pkgs.push("sqlmodel"),
                Some("Tortoise ORM") => pkgs.push("tortoise-orm"),
                _ => {}
            }
        }
        match config.tooling.linting.as_deref() {
            Some("Ruff") => pkgs.push("ruff"),
            Some("Black + Flake8") => { pkgs.push("black"); pkgs.push("flake8"); }
            _ => {}
        }
        if config.tooling.testing.as_deref() == Some("PyTest") { pkgs.push("pytest"); }
        if config.background_jobs.as_deref() == Some("Celery") { pkgs.push("celery"); pkgs.push("redis"); }

        for pkg in &pkgs {
            let status = Command::new(&pip_path).arg("install").arg(pkg).current_dir(backend_folder_abs).status();
            match status {
                Ok(s) if s.success() => println!("Installed {}", pkg),
                _ => println!("Failed to install {} — you can run 'pip install {}' manually", pkg, pkg),
            }
        }
    } else if language == "Go" {
        match config.tooling.linting.as_deref() {
            Some("Golangci-lint + Gofmt") => {
                if require_tool("golangci-lint", "Install from https://golangci-lint.run/usage/install/") {
                    fs::write(backend_folder_abs.join(".golangci.yml"), "run:\n  timeout: 3m\n").ok();
                }
            }
            _ => {}
        }
        if config.background_jobs.as_deref() == Some("Asynq") {
            let _ = Command::new("go").arg("get").arg("github.com/hibiken/asynq").current_dir(backend_folder_abs).status();
        }
    }
}

fn gin_main() -> &'static str {
    "// Backend generated with Chaos.\n// Uses Gin (https://github.com/gin-gonic/gin), an open source Go web framework.\npackage main\n\nimport \"github.com/gin-gonic/gin\"\n\nfunc main() {\n\tr := gin.Default()\n\tr.GET(\"/\", func(c *gin.Context) {\n\t\tc.JSON(200, gin.H{\"message\": \"Hello from Gin!\"})\n\t})\n\tr.Run()\n}"
}
fn fiber_main() -> &'static str {
    "// Backend generated with Chaos.\n// Uses Fiber (https://github.com/gofiber/fiber), an open source Go web framework.\npackage main\n\nimport \"github.com/gofiber/fiber/v2\"\n\nfunc main() {\n\tapp := fiber.New()\n\tapp.Get(\"/\", func(c *fiber.Ctx) error {\n\t\treturn c.JSON(fiber.Map{\"message\": \"Hello from Fiber!\"})\n\t})\n\tapp.Listen(\":3000\")\n}"
}
fn chi_main() -> &'static str {
    "// Backend generated with Chaos.\n// Uses Chi (https://github.com/go-chi/chi), an open source Go router.\npackage main\n\nimport (\n\t\"net/http\"\n\n\t\"github.com/go-chi/chi/v5\"\n)\n\nfunc main() {\n\tr := chi.NewRouter()\n\tr.Get(\"/\", func(w http.ResponseWriter, req *http.Request) {\n\t\tw.Write([]byte(\"Hello from Chi!\"))\n\t})\n\thttp.ListenAndServe(\":3000\", r)\n}"
}
fn echo_main() -> &'static str {
    "// Backend generated with Chaos.\n// Uses Echo (https://github.com/labstack/echo), an open source Go web framework.\npackage main\n\nimport (\n\t\"net/http\"\n\n\t\"github.com/labstack/echo/v4\"\n)\n\nfunc main() {\n\te := echo.New()\n\te.GET(\"/\", func(c echo.Context) error {\n\t\treturn c.String(http.StatusOK, \"Hello from Echo!\")\n\t})\n\te.Logger.Fatal(e.Start(\":3000\"))\n}"
}

fn generate_django_backend(backend_folder_abs: &Path) -> bool {
    if !require_tool("python3", "Install Python from https://python.org") { return false; }
    let status = Command::new("python3").arg("-m").arg("venv").arg("venv").current_dir(backend_folder_abs).status().expect("venv failed");
    if !status.success() { println!("Failed to create virtual environment"); return false; }
    println!("Created virtual environment");
    let pip_path = backend_folder_abs.join("venv/bin/pip");
    let status = Command::new(&pip_path).arg("install").arg("django").current_dir(backend_folder_abs).status().expect("pip failed");
    if !status.success() { println!("Failed to install Django"); return false; }
    println!("Installed Django");
    let django_admin_path = backend_folder_abs.join("venv/bin/django-admin");
    let status = Command::new(&django_admin_path).arg("startproject").arg("backend").arg(".").current_dir(backend_folder_abs).status().expect("startproject failed");
    if status.success() { println!("Generated Django project"); } else { println!("django-admin startproject failed"); }
    status.success()
}

fn generate_flask_backend(backend_folder_abs: &Path, config: &ProjectConfig) -> bool {
    if !require_tool("python3", "Install Python from https://python.org") { return false; }
    let status = Command::new("python3").arg("-m").arg("venv").arg("venv").current_dir(backend_folder_abs).status().expect("venv failed");
    if !status.success() { println!("Failed to create virtual environment"); return false; }
    println!("Created virtual environment");
    let pip_path = backend_folder_abs.join("venv/bin/pip");
    let status = Command::new(&pip_path).arg("install").arg("flask").current_dir(backend_folder_abs).status().expect("pip failed");
    if !status.success() { println!("Failed to install Flask"); return false; }
    println!("Installed Flask");

    let app_py = "from flask import Flask\n\napp = Flask(__name__)\n\n@app.route(\"/\")\ndef home():\n    return \"Hello from Flask!\"\n\nif __name__ == \"__main__\":\n    app.run(debug=True)";
    fs::write(backend_folder_abs.join("app.py"), app_py).ok();
    let mut requirements = vec!["flask".to_string()];
    if let Some(db) = &config.database {
        match db.orm.as_deref() {
            Some("SQLAlchemy") => requirements.push("sqlalchemy".to_string()),
            Some("SQLModel") => requirements.push("sqlmodel".to_string()),
            Some("Tortoise ORM") => requirements.push("tortoise-orm".to_string()),
            _ => {}
        }
    }
    fs::write(backend_folder_abs.join("requirements.txt"), requirements.join("\n") + "\n").ok();
    println!("Generated Flask project");
    true
}

fn generate_express_backend(backend_folder_abs: &Path) -> bool {
    if !require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") { return false; }
    let status = Command::new("npx").arg("--yes").arg("express-generator").arg("--no-view").current_dir(backend_folder_abs).status().expect("express-generator failed");
    if !status.success() { println!("express-generator failed"); return false; }
    println!("Generated Express project");
    let status = Command::new("npm").arg("install").current_dir(backend_folder_abs).status().expect("npm install failed");
    if status.success() { println!("Installed backend dependencies"); } else { println!("npm install failed"); }
    status.success()
}

fn generate_fastify_backend(backend_folder_abs: &Path) -> bool {
    if !require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") { return false; }
    let status = Command::new("npx").arg("--yes").arg("fastify-cli").arg("generate").arg(".").current_dir(backend_folder_abs).status().expect("fastify-cli failed");
    if status.success() { println!("Generated Fastify project"); } else { println!("fastify-cli generate failed"); }
    status.success()
}

fn generate_nestjs_backend(backend_folder_abs: &Path) -> bool {
    if !require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") { return false; }
    let status = Command::new("npx").arg("--yes").arg("@nestjs/cli").arg("new").arg(".").arg("--package-manager").arg("npm").arg("--skip-git")
        .current_dir(backend_folder_abs).status().expect("nest new failed");
    if status.success() { println!("Generated NestJS project"); } else { println!("nest new failed"); }
    status.success()
}

/// Hono has a real official scaffolder (create-hono). Pattern A.
fn generate_hono_backend(backend_folder_abs: &Path) -> bool {
    if !require_tool("npx", "Install Node.js (which includes npx) from https://nodejs.org") { return false; }
    let status = Command::new("npx").arg("--yes").arg("create-hono@latest").arg(".").arg("--template").arg("nodejs")
        .arg("--pm").arg("npm").arg("--install").current_dir(backend_folder_abs).status().expect("create-hono failed");
    if status.success() { println!("Generated Hono project"); } else { println!("create-hono failed"); }
    status.success()
}

/// Elysia's official scaffolder targets Bun. Falls back to a clear message
/// if Bun isn't installed rather than silently failing or faking success.
fn generate_elysia_backend(backend_folder_abs: &Path) -> bool {
    if !require_tool("bun", "Elysia's official scaffolder requires Bun — install from https://bun.sh") { return false; }
    let status = Command::new("bun").arg("create").arg("elysia").arg(".").current_dir(backend_folder_abs).status().expect("bun create elysia failed");
    if status.success() { println!("Generated Elysia project"); } else { println!("bun create elysia failed"); }
    status.success()
}

fn generate_go_backend(backend_folder_abs: &Path, project_name: &str, module: &str, main_content: &str) -> bool {
    if !require_tool("go", "Install Go from https://go.dev/dl/") { return false; }
    let status = Command::new("go").arg("mod").arg("init").arg(project_name).current_dir(backend_folder_abs).status().expect("go mod init failed");
    if !status.success() { println!("go mod init failed"); return false; }
    println!("Initialized Go module");
    let status = Command::new("go").arg("get").arg(module).current_dir(backend_folder_abs).status().expect("go get failed");
    if !status.success() { println!("Failed to fetch {}", module); return false; }
    println!("Installed {}", module);
    fs::write(backend_folder_abs.join("main.go"), main_content).ok();
    println!("Generated project using {}", module);
    true
}

fn generate_rails_backend(backend_folder_abs: &Path) -> bool {
    if !require_tool("ruby", "Install Ruby from https://www.ruby-lang.org/en/downloads/") { return false; }
    if !require_tool("rails", "Install Rails by running: gem install rails") { return false; }
    let status = Command::new("rails").arg("new").arg(".").arg("--skip-git").current_dir(backend_folder_abs).status().expect("rails new failed");
    if status.success() { println!("Generated Rails project"); } else { println!("rails new failed"); }
    status.success()
}

fn generate_laravel_backend(backend_folder_abs: &Path) -> bool {
    if !require_tool("php", "Install PHP from https://www.php.net/downloads") { return false; }
    if !require_tool("composer", "Install Composer from https://getcomposer.org/download/") { return false; }
    let status = Command::new("composer").arg("create-project").arg("laravel/laravel").arg(".").current_dir(backend_folder_abs).status().expect("composer create-project failed");
    if status.success() { println!("Generated Laravel project"); } else { println!("composer create-project failed"); }
    status.success()
}

// ---------- Shared execution ----------

fn execute_plan(folder: &str, plan: &BuildPlan) {
    fs::create_dir_all(folder).expect("Failed to create project folder");
    for (relative_path, content) in &plan.files {
        let full_path = format!("{}/{}", folder, relative_path);
        if let Some(parent) = Path::new(&full_path).parent() { fs::create_dir_all(parent).ok(); }
        fs::write(&full_path, content).expect("Failed to write file");
        println!("Created: {}", full_path);
    }
    if !plan.gitignore_entries.is_empty() {
        let gitignore_content = plan.gitignore_entries.join("\n");
        let gitignore_path = format!("{}/.gitignore", folder);
        fs::write(&gitignore_path, gitignore_content).ok();
        println!("Created: {}", gitignore_path);
    }
}

fn run_npm_install(folder: &str, deps: &[String], dev_deps: &[String]) {
    if deps.is_empty() && dev_deps.is_empty() { return; }
    if !require_tool("npm", "Install Node.js (which includes npm) from https://nodejs.org") { return; }
    println!("\n Installing dependencies...");

    if !deps.is_empty() {
        let mut cmd = Command::new("npm");
        cmd.arg("install");
        for pkg in deps { cmd.arg(pkg); }
        cmd.current_dir(folder);
        match cmd.status() {
            Ok(s) if s.success() => println!("Dependencies installed successfully"),
            _ => println!("npm install failed — you can run it manually inside the project folder"),
        }
    }
    if !dev_deps.is_empty() {
        let mut cmd = Command::new("npm");
        cmd.arg("install").arg("-D");
        for pkg in dev_deps { cmd.arg(pkg); }
        cmd.current_dir(folder);
        match cmd.status() {
            Ok(s) if s.success() => println!("Dev dependencies installed successfully"),
            _ => println!("npm install -D failed — you can run it manually inside the project folder"),
        }
    }
}