use clap::Subcommand;
use plumb::{core::config::manager::ConfigManager, error::Res, types::Project};

#[derive(Debug, Subcommand)]
pub enum ProjectCommand {
    /// List all projects.
    List,
}

pub fn run(config: ConfigManager, command: ProjectCommand) -> Res<()> {
    match command {
        ProjectCommand::List => list(config),
    }
}

pub fn list(config: ConfigManager) -> Res<()> {
    for project in get_projects(config)? {
        println!(
            "{} @ {}",
            project.name(),
            project.location().path().display()
        );
    }
    Ok(())
}

fn get_projects(config: ConfigManager) -> Res<Vec<Project>> {
    Ok(config.config()?.projects().to_vec())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    use plumb::types::Location;

    #[test]
    fn test_get_projects() {
        let config =
            ConfigManager::try_load(Some(&PathBuf::from("tests/fixtures/project_list.yml")))
                .unwrap();
        let projects = get_projects(config).unwrap();
        assert_eq!(
            projects,
            vec![
                Project::new(
                    "project1",
                    Location::new("~/projects/project1".into(), None)
                ),
                Project::new(
                    "project2",
                    Location::new("~/other_projects/project2".into(), None)
                ),
                Project::new(
                    "project3",
                    Location::new("~/projects/project3".into(), None)
                ),
                Project::new(
                    "project4",
                    Location::new("~/projects/project4".into(), None)
                )
            ]
        );
    }
}
