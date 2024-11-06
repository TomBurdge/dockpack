//! Builds a Dockerfile from a directory

use std::fs:: File;
use std::io:: Write;

pub fn create_dockerfile(directory: &str) -> Result<(), String> {

    let docker_file_content = format! {
        "FROM scratch\nCOPY {} .\n", directory
    }; 

    let mut dockerfile = File::create("Dockerfile").map_err(|e| e.to_string())?;

    dockerfile.write_all(docker_file_content.as_bytes()).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_dockerfile() {
        let directory = ".";

        let result = create_dockerfile(directory);

        assert!(result.is_ok());
    
        let dockerfile_content = std::fs::read_to_string("Dockerfile").expect("Error reading dockerfile");
        assert_eq!(dockerfile_content, format!("FROM scratch\nCOPY . .\n"));

        std::fs::remove_file("Dockerfile").expect("Failed to remove dockerfile")
    }
}
