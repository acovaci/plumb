pub mod version;

pub fn command_version() -> String {
    let version = version::version();
    format!("plumb {}", version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(command_version().contains("plumb"));
    }
}
