

pub mod codecommit {
    use std::process::{exit, Command};

    pub fn list_repositories(query: &str) {
        let output = Command::new("aws")
            .args(&["codecommit", "list-repositories", "--output", "text", "--query"])
            .arg(query)
            .output()
            .expect("Failed to execute command");
    
        if output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            eprintln!("Error listing repositories: {}", String::from_utf8_lossy(&output.stderr));
            exit(1);
        }
    }

    pub fn clone_repository(name: &str) {
        let output = Command::new("aws")
            .args(&["codecommit", "get-repository", "--output", "text", "--repository-name", name, "--query", "repositoryMetadata.cloneUrlSsh"])
            .output()
            .expect("Failed to execute command");
    
        if output.status.success() {
            let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let output = Command::new("git")
                .args(&["clone", &url])
                .output()
                .expect("Failed to execute command");
            
            if output.status.success() {
                println!("Repository cloned successfully");
            } else {
                eprintln!("Error cloning repository: {}", String::from_utf8_lossy(&output.stderr));
                exit(1);
            }
        } else {
            eprintln!("Error getting repository URL: {}", String::from_utf8_lossy(&output.stderr));
            exit(1);
        }
    }
}