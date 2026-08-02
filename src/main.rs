//! Command-line entry point for Chaos.
//!
//! `main.rs` is an orchestration layer only: it parses arguments, and for
//! `chaos initialize` wires together a `ProjectManifest`, a
//! `GenerationContext`, a `Planner`, and a `PlanExecutor` into a
//! `Generator`, then runs it. It contains no planning logic (that's
//! `generator::planner`), no filesystem logic (that's
//! `generator::filesystem`), and no template logic (that's
//! `generator::template`) — it only constructs these pieces and hands
//! control to them.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use chaos::generator::{
    DefaultPlanner, FilesystemExecutor, GenerationContext, GenerationError, Generator,
    TemplateResolver,
};
use chaos::manifest::{
    Docker, Git, ProjectManifest, ProjectMetadata, ProjectState, Testing, ToolingManifest,
};

#[derive(Parser)]
#[command(name = "chaos")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes a new Chaos project.
    Initialize,
    // TODO: `write`, `end`, `run` are recognized but not yet implemented,
    // pending their own subsystems.
    Write,
    End,
    Run,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Initialize => run_initialize(),
        Commands::Write => println!("chaos write: not yet implemented"),
        Commands::End => println!("chaos end: not yet implemented"),
        Commands::Run => println!("chaos run: not yet implemented"),
    }
}

/// A `TemplateResolver` with no real templates to resolve.
///
/// Until concrete templates beyond `ReadmeTemplate` exist (Cargo.toml,
/// package.json, Next.js, Django, and so on — see `generator::template`),
/// nothing in the pipeline should be relying on
/// `GenerationOperation::CopyTemplate` succeeding. This stub exists only
/// so `FilesystemExecutor` has a `TemplateResolver` to be constructed
/// with; it fails honestly for every name rather than fabricating content.
struct PlaceholderTemplateResolver;

impl TemplateResolver for PlaceholderTemplateResolver {
    fn resolve(&self, template_name: &str) -> Result<String, GenerationError> {
        Err(GenerationError::MissingTemplate {
            name: template_name.to_string(),
        })
    }
}

/// Runs `chaos initialize` end to end.
///
/// Builds a minimal `ProjectManifest` using placeholder values, wraps it
/// in a `GenerationContext`, and runs it through a `Generator` composed of
/// `DefaultPlanner` and a `FilesystemExecutor`. This is the first
/// end-to-end wiring of the generation pipeline; the manifest it builds is
/// a stand-in for one produced by the Question Engine.
// TODO: replace the placeholder manifest below with one built from
// `engine::ChaosEngine::build_manifest`, once the engine's `Field` model
// covers the project name, frontend routing/styling/state management, and
// every other field `ManifestBuilder` currently reports as unmodeled (see
// `manifest::builder::ManifestBuildError::UnmodeledField`). Until then, a
// minimal manifest is constructed directly so the generation pipeline
// itself can be exercised.
// TODO: once the Question Engine is wired in, this function should also
// surface `AnswerError` (invalid answers) and `ManifestBuildError`
// (mapping failures) to the user, not just `GenerationError`.
fn run_initialize() {
    let manifest = ProjectManifest {
        metadata: ProjectMetadata::new("chaos-project"),
        frontend: None,
        backend: None,
        tooling: ToolingManifest::new(Git::Disabled, Docker::Disabled, Testing::None),
        state: ProjectState::Validated,
    };

    let output_directory = PathBuf::from(&manifest.metadata.name);
    let context = GenerationContext::with_defaults(manifest, output_directory);

    let planner = DefaultPlanner::new();
    let executor = FilesystemExecutor::new(PlaceholderTemplateResolver);
    let generator = Generator::new(planner, executor);

    match generator.generate(&context) {
        Ok(()) => {
            println!(
                "Project generated at '{}'.",
                context.output_directory().display()
            );
        }
        Err(error) => {
            eprintln!("Generation failed: {}", error);
        }
    }
}
