use crate::utils::docker_commands;

pub fn execute_push(directory: &str, image: &str) -> Result<(), String> {
    docker_commands::build_docker_image(directory, image)?;
    Ok(())
}

// run from core_dockpack containing test_image directory with dockerfile
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_execute_push() {
        let image_name = "test_image:latest";
        let directory = "./test_image";

        fs::create_dir_all(directory).expect("Failed to create test directory");
        let dockerfile_path = format!("{}/Dockerfile", directory);
        let dockerfile_content = "FROM scratch\nCOPY . .\n";
        fs::write(&dockerfile_path, dockerfile_content).expect("Failed to write Dockerfile");

        let result = execute_push(directory, image_name);
        assert!(result.is_ok());

        fs::remove_dir_all(directory).expect("Failed to remove test directory");
    }
}