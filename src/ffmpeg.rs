#![allow(dead_code)]
use anyhow::{anyhow, Context, Result};
use std::process::Command;

pub struct VideoMetadata {
    pub width: u32,
    pub height: u32,
    pub duration: f64,
    pub fps: f64,
}

pub fn check_availability() -> Result<()> {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map_err(|_| anyhow!("ffmpeg not found"))?;

    Command::new("ffprobe")
        .arg("-version")
        .output()
        .map_err(|_| anyhow!("ffprobe not found"))?;

    Ok(())
}

pub fn probe(path: &str) -> Result<VideoMetadata> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=width,height,duration,r_frame_rate",
            "-of",
            "csv=p=0",
            path,
        ])
        .output()
        .context("Failed to execute ffprobe")?;

    if !output.status.success() {
        return Err(anyhow!(
            "ffprobe failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let output_str = String::from_utf8(output.stdout)?;
    let parts: Vec<&str> = output_str.trim().split(',').collect();

    // Simplistic parsing for skeleton
    if parts.len() < 4 {
        return Err(anyhow!("Unexpected ffprobe output: {}", output_str));
    }

    let width = parts[0].parse().unwrap_or(0);
    let height = parts[1].parse().unwrap_or(0);
    // Duration might be in format tag if not in stream, but simplicity first
    let duration = parts[2].parse().unwrap_or(0.0);

    // FPS often comes as "30/1" or "30000/1001"
    let fps_str = parts[3];
    let fps = if fps_str.contains('/') {
        let fr_parts: Vec<&str> = fps_str.split('/').collect();
        let num: f64 = fr_parts[0].parse().unwrap_or(0.0);
        let den: f64 = fr_parts[1].parse().unwrap_or(1.0);
        if den == 0.0 {
            0.0
        } else {
            num / den
        }
    } else {
        fps_str.parse().unwrap_or(0.0)
    };

    Ok(VideoMetadata {
        width,
        height,
        duration,
        fps,
    })
}

pub fn run_conversion(args: &[String]) -> Result<()> {
    println!("Running: ffmpeg {}", args.join(" "));

    let output = Command::new("ffmpeg")
        .args(args)
        .output()
        .context("Failed to execute ffmpeg")?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow!("ffmpeg failed: {}", stderr))
    }
}
