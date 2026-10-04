use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::Path;
pub const LUA_ROOT: &str = "../../src-tauri/lua-scripts";

pub fn load_script_with_includes(filename: &str, version: &str) -> io::Result<String> {
    let root_dir = Path::new(LUA_ROOT);
    let file_path = root_dir.join(version).join(filename);
    let content = fs::read_to_string(&file_path).map_err(|e| {
        Error::new(
            ErrorKind::NotFound,
            format!("Failed to read {:?}: {}", file_path, e),
        )
    })?;

    let mut processed_lines = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        // Check for the BullMQ include syntax
        if trimmed.starts_with("--- @include") {
            // Extract the filename inside quotes: --- @include "includes/deduplicateJob"
            if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed[start + 1..].find('"') {
                    let include_path = &trimmed[start + 1..start + 1 + end];

                    let mut real_include_path = String::from(include_path);

                    // Only add "includes/" if it's not already there
                    if !real_include_path.starts_with("includes/") {
                        real_include_path.insert_str(0, "includes/");
                    }

                    // Add .lua if it's not there
                    if !real_include_path.ends_with(".lua") {
                        real_include_path.push_str(".lua");
                    }

                    let included_content = load_script_with_includes(&real_include_path, version)?;
                    processed_lines.push(included_content);
                    continue;
                }
            }
        }

        // If it's not an include line, just push the original line
        processed_lines.push(line.to_string());
    }

    Ok(processed_lines.join("\n"))
}

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("bundled_scripts.rs");

    // 2. Define which main scripts you want to bundle
    let scripts_to_bundle = vec![
        // Custom Scripts
        ("GET_JOB_SCRIPT", "custom/getJobInQueue.lua", "v5"),
        ("GET_JOB_DATA_SCRIPT", "custom/getJobData.lua", "v5"),
        ("SEARCH_JOB_SCRIPT", "custom/searchJobInQueue.lua", "v5"),
        (
            "GET_JOB_COUNT_SCRIPT",
            "custom/getJobCountInQueue.lua",
            "v5",
        ),
        ("GET_ALL_QUEUES", "custom/getAllQueues.lua", "v5"),
        ("GET_JOB_LOGS", "custom/getJobLog.lua", "v5"),
        // Add Scripts
        ("ADD_JOB_SCRIPT", "addStandardJob-9.lua", "v5"),
        ("ADD_DELAYED_JOB_SCRIPT", "addDelayedJob-6.lua", "v5"),
        (
            "ADD_PRIORITIZED_JOB_SCRIPT",
            "addPrioritizedJob-9.lua",
            "v5",
        ),
        // Update Scripts
        ("UPDATE_JOB_DATA_SCRIPT", "updateData-1.lua", "v5"),
        // Remove Scripts
        ("REMOVE_JOB_SCRIPT", "removeJob-2.lua", "v5"),
        // Retry Scripts
        ("RETRY_JOB_SCRIPT", "retryJob-11.lua", "v5"),
        ("REPROCESS_JOB_SCRIPT", "reprocessJob-8.lua", "v5"),
        // Promote Scripts
        ("PROMOTE_JOB_SCRIPT", "promote-9.lua", "v5"),
        // Queue Scripts
        ("PAUSE_QUEUE_SCRIPT", "pause-7.lua", "v5"),
        ("CLEAN_QUEUE_SCRIPT", "cleanJobsInSet-3.lua", "v5"),
    ];

    let mut generated_code = String::new();
    let mut version_groups: std::collections::HashMap<&str, Vec<(&str, &str)>> =
        std::collections::HashMap::new();
    for (name, path, version) in scripts_to_bundle {
        version_groups
            .entry(version)
            .or_default()
            .push((name, path));
    }

    for (version, scripts) in version_groups {
        generated_code.push_str(&format!("pub mod {} {{\n", version));
        for (const_name, relative_path) in scripts {
            let processed_content = load_script_with_includes(relative_path, version)
                .expect(&format!("Failed to bundle script: {}", relative_path));

            generated_code.push_str(&format!(
                "    pub const {}: &str = r###\"{}\"###;\n",
                const_name, processed_content
            ));
        }
        generated_code.push_str("}\n\n");
    }

    fs::write(&dest_path, generated_code).expect("Failed to write bundled scripts");
}
