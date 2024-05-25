use std::{
    env::temp_dir,
    fs::{read_to_string, remove_file},
    path::{Path, PathBuf},
};

pub struct TestDir(PathBuf);

impl Default for TestDir {
    fn default() -> Self {
        Self::new()
    }
}

impl TestDir {
    pub fn new() -> Self {
        let path = temp_dir().join("plumb");
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn file(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

pub struct ExpectedFile {
    pub expected: PathBuf,
    pub actual: PathBuf,
}

impl ExpectedFile {
    pub fn new(path: PathBuf) -> Self {
        let actual = TestDir::new().file(path.file_name().unwrap().to_str().unwrap());

        Self {
            expected: path,
            actual: actual.clone(),
        }
    }

    pub fn temp_file(&self) -> PathBuf {
        self.actual.clone()
    }

    pub fn assert(&self) {
        let expected = read_to_string(&self.expected).unwrap();
        let actual = read_to_string(&self.actual).unwrap();
        remove_file(&self.actual).unwrap();
        assert_eq!(expected, actual);
    }
}

pub fn assert_files_equal(expected: &Path, actual: &Path) {
    let expected = read_to_string(expected).unwrap();
    let actual = read_to_string(actual).unwrap();
    assert_eq!(expected, actual);
}
