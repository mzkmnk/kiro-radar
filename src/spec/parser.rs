use std::{fs, path::Path};

use color_eyre::eyre::{Ok, Result};

pub fn parse_tasks_file(path: &Path) -> Result<(usize, usize)> {
    if !path.exists() {
        return Ok((0, 0));
    }

    let content = fs::read_to_string(path)?;

    if content.is_empty() {
        return Ok((0, 0));
    }

    let mut total = 0;
    let mut completed = 0;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("- [x]") {
            completed += 1;
            total += 1;
        } else if trimmed.starts_with("- [ ]") {
            total += 1;
        }
    }

    Ok((total, completed))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::parse_tasks_file;

    #[test]
    fn test_parse_tasks_file_not_exist() {
        let temp_dir = TempDir::new().unwrap();
        let specs_dir = temp_dir.path().join(".kiro").join("specs");
        let user_auth_dir = specs_dir.join("user-auth");
        fs::create_dir_all(&user_auth_dir).unwrap();

        let (total, completed) = parse_tasks_file(&user_auth_dir.join("tasks.md")).unwrap();

        assert_eq!(total, 0);
        assert_eq!(completed, 0);
    }

    #[test]
    fn test_parse_tasks_file_empty() {
        let temp_dir = TempDir::new().unwrap();
        let specs_dir = temp_dir.path().join(".kiro").join("specs");
        let user_auth_dir = specs_dir.join("user-auth");
        fs::create_dir_all(&user_auth_dir).unwrap();
        fs::write(user_auth_dir.join("tasks.md"), "").unwrap();

        let (total, completed) = parse_tasks_file(&user_auth_dir.join("tasks.md")).unwrap();

        assert_eq!(total, 0);
        assert_eq!(completed, 0);
    }

    #[test]
    fn test_parse_tasks_file() {
        let temp_dir = TempDir::new().unwrap();
        let specs_dir = temp_dir.path().join(".kiro").join("specs");
        let user_auth_dir = specs_dir.join("user-auth");
        fs::create_dir_all(&user_auth_dir).unwrap();
        fs::write(
            user_auth_dir.join("tasks.md"),
            "- [ ] Task 1\n- [ ] Task 2\n  - [ ] Task 2.1\n  - [x] Task 2.2\n- [ ] Task 3",
        )
        .unwrap();

        let (total, completed) = parse_tasks_file(&user_auth_dir.join("tasks.md")).unwrap();

        assert_eq!(total, 5);
        assert_eq!(completed, 1);
    }
}
