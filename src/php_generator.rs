// src/php_generator.rs

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
pub struct PhpFileGenerator {
    file: File,
}

impl PhpFileGenerator {
    pub fn new(file_name: &str, dest: &str) -> Option<PhpFileGenerator> {
        let path = format!("{}/{}.php", dest, file_name);

        let mut file = OpenOptions::new().create_new(true).write(true).open(path);

        return match file {
            Ok(file) => {
                writeln!(&file, "{}", "<?php \n\n").expect("Write Fail");
                Some(PhpFileGenerator { file })
            }
            Err(e) => None,
        };
    }

    pub fn add_function(&mut self, function_name: &str, function_body: &str) {
        let content = format!(
            "function {}() {{\n    {}\n}}\n\n",
            function_name, function_body
        );

        self.write_to_file(&content);
    }

    fn write_to_file(&mut self, content: &str) {
        writeln!(self.file, "{}", content).expect("Write Fail");
    }
}