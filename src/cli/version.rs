use plumb::{error::Res, types::SemanticVersion};

pub fn version() -> Res<()> {
    println!("plumb {}", get_version()?);
    Ok(())
}

fn get_version() -> Res<SemanticVersion> {
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
