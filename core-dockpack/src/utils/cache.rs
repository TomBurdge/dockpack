//! Defines all the paths to cache directories and also handles the wiping of cache.
use std::path::PathBuf;

/// Processes the image name into a directory name.
///
/// # Arguments
/// * `image_name` - the name of the image to be processed
///
/// # Returns
/// * the converted image name string that can be used in a directory path.
pub fn process_image_name(image_name: &str) -> String {
    image_name
        .replace(":latest", "")
        .replace("/", "_")
        .replace(":", "_")
}

/// Wipes the cache directory and creates a new cache directory.
///
/// # Returns
/// None
pub fn wipe_and_create_cache(directory: &PathBuf) {
    if directory.exists() {
        std::fs::remove_dir_all(directory).expect("Failed to remove cache directory");
    }
    std::fs::create_dir_all(directory).expect("Failed to create nanoservices cache directory");
    let tar_dir = directory.join("tar");
    std::fs::create_dir_all(tar_dir).expect("Failed to create nanoservices tar cache directory");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process_image_name() {
        let image_name = "surrealdb/surrealcs-client:latest".to_string();
        // let expected = "docker.io_library_alpine".to_string();
        println!("{}", process_image_name(&image_name));
    }
}
