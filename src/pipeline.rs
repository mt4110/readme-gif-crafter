#![allow(dead_code)]
use crate::cli::Cli;
use crate::config::Config;
use anyhow::Result;

pub struct Pipeline {
    config: Config,
    cli: Cli,
}

impl Pipeline {
    pub fn new(config: Config, cli: Cli) -> Self {
        Self { config, cli }
    }

    pub fn build_command(&self) -> Result<Vec<String>> {
        let input = self.cli.input.as_ref().expect("Input file required");

        // Determine settings (CLI overrides Config)
        // For now, let's look for a preset if specified, or rely on CLI defaults/Config defaults
        // This logic needs to be robust, but for "Go" request, let's implement the core flow.

        let preset_name = self
            .cli
            .preset
            .as_deref()
            .or(self.config.default_preset.as_deref());

        let preset = preset_name.and_then(|name| self.config.presets.get(name));

        // Resolve parameters
        let fps = self.cli.fps.or(preset.and_then(|p| p.fps)).unwrap_or(15);
        let width = self
            .cli
            .width
            .or(preset.and_then(|p| p.width))
            .unwrap_or(800);

        // Construct filter chain
        // 1. Crop (if needed) - CLI priority
        // 2. FPS
        // 3. Scale
        // 4. Palettegen/use

        let mut filters = Vec::new();

        if let Some(crop) = &self.cli.crop {
            filters.push(format!("crop={}", crop));
        }

        filters.push(format!("fps={}", fps));
        filters.push(format!("scale={}:-1:flags=lanczos", width));

        // Split for palette check
        filters.push("split[s0][s1]".to_string());
        filters.push("[s0]palettegen[p]".to_string());

        let mut args = vec![
            "-y".to_string(), // Overwrite
            "-i".to_string(),
            input.clone(),
        ];

        // Duration/Trim
        if let Some(start) = self.cli.start {
            args.push("-ss".to_string());
            args.push(start.to_string());
        }

        if let Some(duration) = self.cli.duration {
            args.push("-t".to_string());
            args.push(duration.to_string());
        } else if let Some(end) = self.cli.end {
            args.push("-to".to_string());
            args.push(end.to_string());
        }

        // Linear part: crop -> fps -> scale
        let mut linear_filters = Vec::new();
        if let Some(crop) = &self.cli.crop {
            linear_filters.push(format!("crop={}", crop));
        }
        linear_filters.push(format!("fps={}", fps));
        linear_filters.push(format!("scale={}:-1:flags=lanczos", width));

        let linear_str = linear_filters.join(",");
        let full_filter = format!(
            "{},split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
            linear_str
        );

        args.push("-filter_complex".to_string());
        args.push(full_filter);

        let output = self
            .cli
            .output
            .clone()
            .unwrap_or_else(|| "output.gif".to_string());
        args.push(output);

        Ok(args)
    }

    pub fn get_output_filename(&self) -> String {
        self.cli
            .output
            .clone()
            .unwrap_or_else(|| "output.gif".to_string())
    }
}
