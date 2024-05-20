use plumb::{error::Error, types::SemanticVersion};

pub fn version() -> Result<(), Error> {
    println!("plumb {}", get_version()?);
    Ok(())
}

fn get_version() -> Result<SemanticVersion, Error> {
    SemanticVersion::from_env()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(
            get_version().unwrap(),
            SemanticVersion::try_from("0.1.0").unwrap()
        );
    }
}
