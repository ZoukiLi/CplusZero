//! smoke test.
//! using src files in ../resources/src_code

extern crate core;

/// tests mod
#[cfg(test)]
mod tests {
    use std::fs::{File, read_dir};
    use std::io;
    use std::io::{Read, Write};
    use std::path::{Path, PathBuf};

    use pest::iterators::Pair;
    use pest::Parser;

    use cpz_parser::{CpluszeroParser, Rule};

    /// test source file dir
    const TEST_SRC_DIR: &str = "resources/src_code";
    /// test dest file dir
    #[allow(dead_code)]
    const TEST_DEST_DIR: &str = "resources/dest_code";

    /// get all ".cpz" files in TEST_SRC_DIR
    fn get_test_files() -> io::Result<impl Iterator<Item=PathBuf>> {
        let dir = read_dir(TEST_SRC_DIR)?;
        Ok(dir.filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .filter(|path| path.extension().map_or(false, |ext| ext == "cpz")))
    }

    /// get dest file path according to src file path
    fn get_dest_file_path(src_path: &Path) -> io::Result<PathBuf> {
        let mut dest_path = PathBuf::from(TEST_DEST_DIR);
        let src_file_name = src_path.file_stem()
            .and_then(|name| name.to_str())
            .ok_or(io::Error::new(io::ErrorKind::Other, "Get file name failed"))?;
        let file_name = format!("{}.txt", src_file_name);
        dest_path.push(file_name);
        Ok(dest_path)
    }

    /// smoke test parser
    #[test]
    fn parser_works() {
        // get all test files
        let test_files = get_test_files().expect("Get test files failed");
        // parse each file and check if it's ok
        test_files.for_each(|path| {
            println!("Parsing file: {:?}", path);
            let file = File::open(&path).expect("Open file failed");
            let mut reader = io::BufReader::new(file);
            let mut content = String::new();
            reader.read_to_string(&mut content).expect("Read file failed");
            // only test if parsing is ok
            let _ = CpluszeroParser::parse(Rule::Program, &content).expect("Parse failed");
        })
    }

    /// display pair of parser
    fn display_pairs(pair: Pair<Rule>, indent: usize) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        // indent
        let indent_str = " ".repeat(indent);
        let (row_st, col_st) = pair.as_span().start_pos().line_col();
        let (row_ed, col_ed) = pair.as_span().end_pos().line_col();
        // display rule and span
        writeln!(buffer, "{indent_str}{:?}: [{row_st}:{col_st} - {row_ed}:{col_ed})",
                 pair.as_rule())?;
        // for each pair in pair inner
        for inner_pair in pair.into_inner() {
            // display inner pair
            let inner_str = display_pairs(inner_pair, indent + 2)?;
            buffer.extend(inner_str);
        }

        Ok(buffer)
    }

    /// print result of parser to dest file
    #[test]
    fn print_pairs() {
        // get all test files
        let test_files = get_test_files().expect("Get test files failed");
        // parse each file and check if it's ok
        test_files.for_each(|path| {
            println!("Parsing file: {:?}", path);
            let file = File::open(&path).expect("Open file failed");
            let mut reader = io::BufReader::new(file);
            let mut content = String::new();
            reader.read_to_string(&mut content).expect("Read file failed");

            let dest_path = get_dest_file_path(&path).expect("Get dest file path failed");
            println!("Dest file: {:?}", dest_path);
            // if dest dir not exists, create it
            let dest_dir = dest_path.parent().expect("Get dest dir failed");
            if !dest_dir.exists() {
                println!("Create dest dir: {:?}", dest_dir);
                std::fs::create_dir_all(dest_dir).expect("Create dest dir failed");
            }
            // create dest file or clear it
            let mut dest_file = File::create(&dest_path).expect("Create dest file failed");
            let mut dest_writer = io::BufWriter::new(&mut dest_file);


            let parse_result = CpluszeroParser::parse(Rule::Program, &content);
            match parse_result {
                Ok(pairs) => {
                    // write pairs to dest file
                    let content = pairs.flat_map(|pair| display_pairs(pair, 0))
                        .flatten().collect::<Vec<_>>();

                    dest_writer.write_all(&content).expect("Write to dest file failed");
                }
                Err(e) => {
                    // write error to dest file
                    let mut content = Vec::new();
                    writeln!(content, "Parse failed:").expect("Write to dest file failed");
                    writeln!(content, "{}", e).expect("Write to dest file failed");
                    dest_writer.write_all(&content).expect("Write to dest file failed");
                }
            }
        })
    }
}