#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    fn setup_git_repo() -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path().to_str().unwrap().to_string();
        
        // Initialize git repo
        git2::Repository::init(&repo_path).unwrap();
        
        // Create a test file
        let file_path = Path::new(&repo_path).join("test.txt");
        fs::write(&file_path, "initial content\n").unwrap();
        
        (temp_dir, file_path.to_str().unwrap().to_string())
    }

    #[tokio::test]
    async fn test_file_edit_with_git() {
        let (temp_dir, file_path) = setup_git_repo();
        
        let request = FileEditRequest {
            file_path: file_path.clone(),
            start_line: 0,
            end_line: 0,
            new_content: "modified content".to_string(),
            commit_message: "test commit".to_string(),
        };

        let result = file_edit(request).await.unwrap();
        
        // Verify file was modified
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "modified content");
        
        // Verify git commit happened
        let repo = git2::Repository::open(temp_dir.path()).unwrap();
        let head_commit = repo.head().unwrap().peel_to_commit().unwrap();
        assert_eq!(head_commit.message().unwrap(), "test commit");
        
        // Clean up
        drop(temp_dir);
    }

    #[tokio::test]
    async fn test_file_edit_without_git() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "initial content\n").unwrap();
        
        let request = FileEditRequest {
            file_path: file_path.to_str().unwrap().to_string(),
            start_line: 0,
            end_line: 0,
            new_content: "modified content".to_string(),
            commit_message: "test commit".to_string(),
        };

        let result = file_edit(request).await.unwrap();
        
        // Verify file was modified
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "modified content");
        
        // Clean up
        drop(temp_dir);
    }
}
