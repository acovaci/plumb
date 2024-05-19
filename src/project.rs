use crate::Config;

pub fn list(config: Config) {
    for project in config.projects() {
        println!("{} @ {:?}", project.name, project.location.unwrap());
    }
}
