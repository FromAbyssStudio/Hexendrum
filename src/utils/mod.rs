#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::time::Duration;

/// Format duration as MM:SS
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

/// Format duration in seconds as MM:SS
pub fn format_duration_seconds(seconds: u64) -> String {
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, remaining_seconds)
}

/// Format file size in human readable format
pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    match bytes {
        0..KB => format!("{} B", bytes),
        KB..MB => format!("{:.1} KB", bytes as f64 / KB as f64),
        MB..GB => format!("{:.1} MB", bytes as f64 / MB as f64),
        _ => format!("{:.1} GB", bytes as f64 / GB as f64),
    }
}

/// Get file extension from path
pub fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

/// Check if a file is an audio file
pub fn is_audio_file(path: &Path) -> bool {
    if let Some(ext) = get_file_extension(path) {
        matches!(
            ext.as_str(),
            "mp3" | "flac" | "ogg" | "wav" | "m4a" | "aac" | "opus"
        )
    } else {
        false
    }
}

/// Get relative path from base directory
pub fn get_relative_path(path: &Path, base: &Path) -> Option<PathBuf> {
    path.strip_prefix(base).ok().map(|p| p.to_path_buf())
}

/// Ensure directory exists, create if it doesn't
pub fn ensure_directory(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Get file name without extension
pub fn get_file_name_without_extension(path: &Path) -> Option<String> {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
}

/// Sanitize filename for safe storage
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() || "._-".contains(c) {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Parse time string in format MM:SS or HH:MM:SS
pub fn parse_time_string(time_str: &str) -> Option<Duration> {
    let parts: Vec<&str> = time_str.split(':').collect();

    match parts.len() {
        2 => {
            // MM:SS format
            let minutes: u64 = parts[0].parse().ok()?;
            let seconds: u64 = parts[1].parse().ok()?;
            Some(Duration::from_secs(minutes * 60 + seconds))
        }
        3 => {
            // HH:MM:SS format
            let hours: u64 = parts[0].parse().ok()?;
            let minutes: u64 = parts[1].parse().ok()?;
            let seconds: u64 = parts[2].parse().ok()?;
            Some(Duration::from_secs(hours * 3600 + minutes * 60 + seconds))
        }
        _ => None,
    }
}

/// Get human readable time ago string
pub fn time_ago(timestamp: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(timestamp);

    if duration.num_seconds() < 60 {
        "Just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{} minutes ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{} hours ago", duration.num_hours())
    } else if duration.num_days() < 7 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_weeks() < 4 {
        format!("{} weeks ago", duration.num_weeks())
    } else if duration.num_days() < 365 {
        format!("{} months ago", duration.num_days() / 30)
    } else {
        format!("{} years ago", duration.num_days() / 365)
    }
}

/// Truncate string to specified length with ellipsis
pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length - 3])
    }
}

/// Capitalize first letter of string
pub fn capitalize_first(s: &str) -> String {
    if s.is_empty() {
        s.to_string()
    } else {
        let mut chars: Vec<char> = s.chars().collect();
        chars[0] = chars[0].to_uppercase().next().unwrap();
        chars.into_iter().collect()
    }
}

/// Convert string to title case
pub fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| capitalize_first(word))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(65)), "01:05");
        assert_eq!(format_duration(Duration::from_secs(3661)), "61:01");
    }

    #[test]
    fn test_format_duration_seconds() {
        assert_eq!(format_duration_seconds(65), "01:05");
        assert_eq!(format_duration_seconds(3661), "61:01");
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
        assert_eq!(format_file_size(1073741824), "1.0 GB");
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("file name.txt"), "file name.txt");
        assert_eq!(sanitize_filename("file/name.txt"), "file_name.txt");
        assert_eq!(sanitize_filename("file*name.txt"), "file_name.txt");
    }

    #[test]
    fn test_parse_time_string() {
        assert_eq!(parse_time_string("1:30"), Some(Duration::from_secs(90)));
        assert_eq!(
            parse_time_string("1:30:45"),
            Some(Duration::from_secs(5445))
        );
        assert_eq!(parse_time_string("invalid"), None);
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("Hello World", 8), "Hello...");
        assert_eq!(truncate_string("Short", 10), "Short");
    }

    #[test]
    fn test_capitalize_first() {
        assert_eq!(capitalize_first("hello"), "Hello");
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_to_title_case() {
        assert_eq!(to_title_case("hello world"), "Hello World");
        assert_eq!(to_title_case(""), "");
    }
}
