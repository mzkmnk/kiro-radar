use std::{fs, path::PathBuf};

use crate::spec::finder::SpecSet;

/// Spec ファイルの内容を保持する構造体
#[derive(Debug)]
pub struct SpecContent {
    pub requirements: Option<String>,
    pub design: Option<String>,
    pub tasks: Option<String>,
}

/// Spec セットの全ファイルを読み込む
///
/// # 引数
/// * `spec_set` - 読み込む Spec セット
///
/// # 戻り値
/// * `SpecContent` - 読み込んだファイルの内容
pub fn read_spec_content(spec_set: &SpecSet) -> SpecContent {
    SpecContent {
        requirements: read_spec_file(&spec_set.requirements),
        design: read_spec_file(&spec_set.design),
        tasks: read_spec_file(&spec_set.tasks),
    }
}

/// 単一の Spec ファイルを読み込む
///
/// # 引数
/// * `path` - ファイルパス（Option）
///
/// # 戻り値
/// * `Option<String>` - ファイルの内容（読み込み成功時）、None（ファイル不在またはエラー時）
fn read_spec_file(path: &Option<PathBuf>) -> Option<String> {
    match path {
        None => None,
        Some(p) => fs::read_to_string(p).ok(),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_read_spec_file_none() {
        let result = read_spec_file(&None);
        assert_eq!(result, None);
    }

    #[test]
    fn test_read_spec_file_not_exist() {
        let path = Some(PathBuf::from("/nonexistent/file.md"));
        let result = read_spec_file(&path);
        assert_eq!(result, None);
    }

    #[test]
    fn test_read_spec_file_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.md");
        fs::write(&file_path, "# Test Content").unwrap();

        let result = read_spec_file(&Some(file_path));
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "# Test Content");
    }

    #[test]
    fn test_read_spec_content_all_files() {
        let temp_dir = TempDir::new().unwrap();
        let spec_dir = temp_dir.path().join("test-spec");
        fs::create_dir_all(&spec_dir).unwrap();

        let req_path = spec_dir.join("requirements.md");
        let design_path = spec_dir.join("design.md");
        let tasks_path = spec_dir.join("tasks.md");

        fs::write(&req_path, "# Requirements").unwrap();
        fs::write(&design_path, "# Design").unwrap();
        fs::write(&tasks_path, "# Tasks").unwrap();

        let spec_set = SpecSet {
            name: "test-spec".to_string(),
            requirements: Some(req_path),
            design: Some(design_path),
            tasks: Some(tasks_path),
            total_tasks: Some(0),
            completed_tasks: Some(0),
        };

        let content = read_spec_content(&spec_set);

        assert!(content.requirements.is_some());
        assert_eq!(content.requirements.unwrap(), "# Requirements");
        assert!(content.design.is_some());
        assert_eq!(content.design.unwrap(), "# Design");
        assert!(content.tasks.is_some());
        assert_eq!(content.tasks.unwrap(), "# Tasks");
    }

    #[test]
    fn test_read_spec_content_missing_files() {
        let spec_set = SpecSet {
            name: "test-spec".to_string(),
            requirements: None,
            design: None,
            tasks: None,
            total_tasks: Some(0),
            completed_tasks: Some(0),
        };

        let content = read_spec_content(&spec_set);

        assert!(content.requirements.is_none());
        assert!(content.design.is_none());
        assert!(content.tasks.is_none());
    }

    #[test]
    fn test_read_spec_content_partial_files() {
        let temp_dir = TempDir::new().unwrap();
        let spec_dir = temp_dir.path().join("test-spec");
        fs::create_dir_all(&spec_dir).unwrap();

        let req_path = spec_dir.join("requirements.md");
        fs::write(&req_path, "# Requirements").unwrap();

        let spec_set = SpecSet {
            name: "test-spec".to_string(),
            requirements: Some(req_path),
            design: None,
            tasks: Some(PathBuf::from("/nonexistent/tasks.md")),
            total_tasks: Some(0),
            completed_tasks: Some(0),
        };

        let content = read_spec_content(&spec_set);

        assert!(content.requirements.is_some());
        assert_eq!(content.requirements.unwrap(), "# Requirements");
        assert!(content.design.is_none());
        assert!(content.tasks.is_none());
    }
}
