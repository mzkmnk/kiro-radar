use std::{
    fs,
    path::{Path, PathBuf},
};

use color_eyre::eyre::Result;

use crate::spec::parser::parse_tasks_file;

#[derive(Debug)]
pub struct SpecSet {
    // spec name
    pub name: String,
    // requirements.md path (reserved for future file opening feature)
    #[allow(dead_code)]
    pub requirements: Option<PathBuf>,
    // design.md path (reserved for future file opening feature)
    #[allow(dead_code)]
    pub design: Option<PathBuf>,
    // tasks.md path (reserved for future file opening feature)
    #[allow(dead_code)]
    pub tasks: Option<PathBuf>,
    // total tasks
    pub total_tasks: Option<usize>,
    // completed tasks
    pub completed_tasks: Option<usize>,
}

/// .kiro/specs 配下の全てのSpecを探索
///
/// # 引数
/// * `project_root` - プロジェクトのルートディレクトリ
///
/// # 戻り値
/// * `Ok(Vec<SpecSet>)` - 見つけたSpecのリスト
pub fn find_all_specs(project_root: &Path) -> Result<Vec<SpecSet>> {
    let specs_dir = project_root.join(".kiro").join("specs");

    if !specs_dir.exists() {
        return Ok(Vec::new());
    }

    let mut spec_sets = Vec::new();

    for entry in fs::read_dir(&specs_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir()
            && let Some(name) = path.file_name()
        {
            let name = name.to_string_lossy().to_string();

            let requirements = path.join("requirements.md");
            let design = path.join("design.md");
            let tasks = path.join("tasks.md");
            let (total_tasks, completed_tasks) = parse_tasks_file(&tasks)?;

            spec_sets.push(SpecSet {
                name,
                requirements: if requirements.exists() {
                    Some(requirements)
                } else {
                    None
                },
                tasks: if tasks.exists() { Some(tasks) } else { None },
                design: if design.exists() { Some(design) } else { None },
                total_tasks: Some(total_tasks),
                completed_tasks: Some(completed_tasks),
            });
        }
    }

    spec_sets.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(spec_sets)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use crate::spec::finder::find_all_specs;

    #[test]
    fn test_find_all_specs_empty() {
        let temp_dir = TempDir::new().unwrap();
        let result = find_all_specs(temp_dir.path()).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_find_all_specs_multiple() {
        let temp_dir = TempDir::new().unwrap();
        let specs_dir = temp_dir.path().join(".kiro").join("specs");

        // create user-auth spec
        let user_auth_dir = specs_dir.join("user-auth");
        fs::create_dir_all(&user_auth_dir).unwrap();
        fs::write(user_auth_dir.join("requirements.md"), "#requirements").unwrap();

        // create dashboard spec
        let dashboard = specs_dir.join("dashboard");
        fs::create_dir_all(&dashboard).unwrap();
        fs::write(dashboard.join("requirements.md"), "#requirements").unwrap();

        let result = find_all_specs(temp_dir.path()).unwrap();

        assert_eq!(result.len(), 2);

        // sort check
        assert_eq!(result[0].name, "dashboard");
        assert_eq!(result[1].name, "user-auth");
    }
}
