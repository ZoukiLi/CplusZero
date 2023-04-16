//! smoke test.
//! using src files in ../resources/src_code

/// tests mod
#[cfg(test)]
mod tests {
    /// test source file dir
    const TEST_SRC_DIR: &str = "../resources/src_code";
    /// test dest file dir
    const TEST_DEST_DIR: &str = "../resources/dest_code";

    /// get all ".cpz" files in TEST_SRC_DIR
    fn get_test_files() -> Vec<String> {
        let mut test_files = Vec::new();
        let paths = std::fs::read_dir(TEST_SRC_DIR).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            let path_str = path.to_str().unwrap();
            if path_str.ends_with(".cpz") {
                test_files.push(path_str.to_string());
            }
        }
        test_files
    }

    /// test parser
    #[test]
    fn parser_works() {}
}