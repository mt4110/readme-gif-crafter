use std::path::Path;

pub fn generate_snippet(output_path: &str, file_size_mb: Option<f64>) -> String {
    let path = Path::new(output_path);
    let filename = path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or(output_path);

    // Use full path for link, ensuring forward slashes
    let rel_path = format!("./{}", output_path).replace('\\', "/");

    let size_info = match file_size_mb {
        Some(sz) => format!(" ({:.1} MB)", sz),
        None => "".to_string(),
    };

    format!(
        "Generated: {}{}\nMarkdown:\n\n![Demo]({})",
        filename, size_info, rel_path
    )
}

pub fn update_readme(
    readme_path: &str,
    markdown_snippet: &str,
    marker_name: &str,
) -> anyhow::Result<()> {
    // 1. Read existing README
    let content = std::fs::read_to_string(readme_path)
        .map_err(|e| anyhow::anyhow!("Failed to read README at {}: {}", readme_path, e))?;

    // 2. Define markers
    let start_marker = format!("<!-- {}:start -->", marker_name);
    let end_marker = format!("<!-- {}:end -->", marker_name);

    // 3. Find marker positions
    let start_pos = content.find(start_marker.as_str());
    let end_pos = content.find(end_marker.as_str());

    if let (Some(start), Some(end)) = (start_pos, end_pos) {
        if start > end {
            return Err(anyhow::anyhow!("Markers found but invalid: start > end"));
        }

        // 4. Construct new content
        let prefix = &content[..start + start_marker.len()];
        let suffix = &content[end..];

        // We assume markdown_snippet contains the "Generated: ..." header which we might not want in the README.
        // Usually for README we only want the image link.
        // Let's extract just the image link part from the snippet for the README injection.
        // Or if the snippet IS the content we want. The current generate_snippet returns a human readable log format.
        // Let's adjust helper to return clean markdown or parse it here.
        // For simplicity, let's just create the clean markdown here.
        let image_link = markdown_snippet
            .lines()
            .find(|l| l.starts_with("!["))
            .unwrap_or(markdown_snippet);

        let new_content = format!("{}\n\n{}\n\n{}", prefix, image_link, suffix);

        // 5. Write back
        std::fs::write(readme_path, new_content)
            .map_err(|e| anyhow::anyhow!("Failed to write README at {}: {}", readme_path, e))?;

        Ok(())
    } else {
        eprintln!(
            "Warning: Markers '{}' and '{}' not found in {}. Skipping README update.",
            start_marker, end_marker, readme_path
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_update_readme_logic() {
        let path = "test_readme_marker.md";
        let content = "Head\n<!-- rgc:start -->\nOld\n<!-- rgc:end -->\nTail";
        fs::write(path, content).unwrap();

        let snippet = "Generated: demo.gif\nMarkdown:\n\n![Demo](./new.gif)";
        update_readme(path, snippet, "rgc").unwrap();

        let new_content = fs::read_to_string(path).unwrap();
        assert!(new_content.contains("![Demo](./new.gif)"));
        assert!(new_content.contains("Head"));
        assert!(new_content.contains("Tail"));
        assert!(!new_content.contains("Old"));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_update_readme_missing_markers() {
        let path = "test_readme_no_marker.md";
        let content = "No markers here";
        fs::write(path, content).unwrap();

        let snippet = "Markdown";
        // Should not error
        update_readme(path, snippet, "rgc").unwrap();

        let new_content = fs::read_to_string(path).unwrap();
        assert_eq!(new_content, content); // Should be unchanged

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_generate_snippet_path_normalization() {
        // Simulate windows path behavior or input
        let output_path = "assets\\demo.gif";

        let snippet = generate_snippet(output_path, None);

        // It should contain ./assets/demo.gif (full path preserved)
        // Note: The logic adds ./ to the start, so ./assets/demo.gif
        // We accept both full match or substring match that confirms logic.
        assert!(snippet.contains("](assets/demo.gif)") || snippet.contains("(./assets/demo.gif)"));
    }
}
