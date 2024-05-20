use plumb::{core::config::PlumbConfig, error::Error, types::Project};

pub fn list(config: PlumbConfig) -> Result<(), Error> {
    for project in get_projects(config) {
        println!(
            "{} @ {}",
            project.name(),
            project.location().path().display()
        );
    }
    Ok(())
}

fn get_projects(config: PlumbConfig) -> Vec<Project> {
    config.projects().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    use plumb::types::Location;

    #[test]
    fn test_get_projects() {
        let config = PlumbConfig::load(None).unwrap();
        let projects = get_projects(config);
        assert_eq!(
            projects,
            vec![
                Project::new("plumb", Location::new("~/projects/plumb".into(), None)),
                Project::new("monad", Location::new("~/projects/monad".into(), None))
            ]
        );
    }
}
