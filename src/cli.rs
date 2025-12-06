use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "rgc", about = "README Gif Crafter", version = "0.1.0")]
pub struct Cli {
    /// Input video file
    #[arg(required_unless_present = "markdown_only")]
    pub input: Option<String>,

    /// Output file name
    #[arg(short, long)]
    pub output: Option<String>,

    /// Output Markdown only (requires input to be a path to existing video/gif if no generation is done?
    /// Actually user said "make markdown from existing file".
    /// If markdown_only is set, input should be the gif path.)
    #[arg(long)]
    pub markdown_only: bool,

    #[arg(long)]
    pub dry_run: bool,

    /// Start time in seconds
    #[arg(long)]
    pub start: Option<f64>,

    /// End time in seconds
    #[arg(long)]
    pub end: Option<f64>,

    /// Duration in seconds (override end)
    #[arg(long)]
    pub duration: Option<f64>,

    /// Crop area (x,y,w,h)
    #[arg(long)]
    pub crop: Option<String>,

    /// Target width (height maintained by aspect ratio)
    #[arg(long)]
    pub width: Option<u32>,

    /// Frame rate
    #[arg(long)]
    pub fps: Option<u32>,

    /// Target max size in MB
    #[arg(long)]
    pub max_size: Option<f64>,

    /// Preset name (e.g., github, mini)
    #[arg(long)]
    pub preset: Option<String>,

    /// Update the README file at the given path with the generated markdown
    #[arg(long)]
    pub update_readme: Option<String>,

    /// Marker name for README injection (default: "rgc")
    /// e.g. "config-demo" => <!-- config-demo:start --> ... <!-- config-demo:end -->
    #[arg(long, default_value = "rgc")]
    pub marker: String,
}
