use anyhow::Result;
use clap::Parser;

mod cli;
mod config;
mod ffmpeg;
mod markdown;
mod pipeline;

use cli::Cli;
use config::load_config;
use markdown::generate_snippet;
use pipeline::Pipeline;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // 1. Config loading
    let config = load_config()?;

    // 2. Markdown only mode
    if cli.markdown_only {
        let input = cli
            .input
            .as_ref()
            .expect("Input file required for markdown-only");
        let snippet = generate_snippet(input, None);
        println!("{}", snippet);

        if let Some(readme_path) = cli.update_readme {
            println!("Updating README at: {}", readme_path);
            if let Err(e) = markdown::update_readme(&readme_path, &snippet, &cli.marker) {
                eprintln!("Error updating README: {}", e);
            } else {
                println!("README updated successfully.");
            }
        }
        return Ok(());
    }

    // 3. Pipeline setup
    let pipeline = Pipeline::new(config.clone(), cli.clone());
    let args = pipeline.build_command()?;

    // 4. Execution & Optimization Loop
    let output_file = pipeline.get_output_filename();

    // Determine max size (CLI > Config)
    // We need to resolve this same way pipeline does, or expose it from pipeline?
    // Let's resolve simple here:
    let preset_name = cli.preset.as_deref().or(config.default_preset.as_deref());
    let preset = preset_name.and_then(|name| config.presets.get(name));
    let target_max_size = cli.max_size.or(preset.and_then(|p| p.max_size_mb));

    if cli.dry_run {
        println!("ffmpeg {}", args.join(" "));
        // output_file mock for snippet generation
    } else {
        // First attempt
        ffmpeg::run_conversion(&args)?;

        // Loop if size is too big
        // We only try reducing width for now (simple strategy)
        let mut current_cli = cli.clone();
        let mut retries = 0;
        const MAX_RETRIES: u32 = 2;

        while let Some(max_mb) = target_max_size {
            if retries >= MAX_RETRIES {
                println!("Max retries reached. Output may exceed target size.");
                break;
            }

            let metadata = std::fs::metadata(&output_file);
            if let Ok(meta) = metadata {
                let size_mb = meta.len() as f64 / 1024.0 / 1024.0;
                if size_mb <= max_mb {
                    println!("Target size met: {:.2} MB <= {:.2} MB", size_mb, max_mb);
                    break;
                }

                println!(
                    "File too large ({:.2} MB > {:.2} MB). Optimizing...",
                    size_mb, max_mb
                );
                retries += 1;

                // Reduce width by 20%
                let current_width = current_cli.width.unwrap_or(800); // Should match pipeline default
                let new_width = (current_width as f64 * 0.8) as u32;
                current_cli.width = Some(new_width);

                println!("Retrying with width: {}", new_width);
                let new_pipeline = Pipeline::new(config.clone(), current_cli.clone());
                let new_args = new_pipeline.build_command()?;

                ffmpeg::run_conversion(&new_args)?;
            } else {
                break; // File not found?
            }
        }
    }

    // 5. Output result
    if !cli.dry_run {
        // Get actual file size for snippet
        let size_mb = if std::path::Path::new(&output_file).exists() {
            std::fs::metadata(&output_file)
                .ok()
                .map(|m| m.len() as f64 / 1024.0 / 1024.0)
        } else {
            None
        };

        let snippet = generate_snippet(&output_file, size_mb);
        println!("{}", snippet);

        if let Some(readme_path) = cli.update_readme {
            println!("Updating README at: {}", readme_path);
            if let Err(e) = markdown::update_readme(&readme_path, &snippet, &cli.marker) {
                eprintln!("Error updating README: {}", e);
            } else {
                println!("README updated successfully.");
            }
        }
    }

    Ok(())
}
