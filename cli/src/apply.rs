use std::{fs::File, io::Read, path::PathBuf, process::exit};

use template_engine::TemplateEngine;

pub fn apply() {
    // Create array of file paths and contents
    let mut content_array: Vec<(PathBuf, PathBuf, String)> = Vec::new();

    let config_struct = crate::get_config_file_content();

    for (source, destination) in config_struct.paths_iter() {
        if destination.is_none() {
            println!("WARNING: Config with path `{:#?}` has no link location. Skipping...", source);
            continue;
        }

        let destination = destination.unwrap();

        if destination.exists() {
            println!("WARNING: Link destination with path `{:#?}` already exists. Skipping...", destination);
            continue;
        }

        if !source.exists() {
            println!("WARNING: Source file with path `{:#?}` does not exist. Skipping...", source);
            continue;
        }
        
        let mut source_content = String::new();

        match File::open(source).expect("ERROR: Failed to open source file, but path exists.").read_to_string(&mut source_content) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("ERROR: Failed to read source file with path `{:#?}`, but path exists.", source);
                exit(1);
            }
        }
        
        if source_content.is_empty() {
            println!("WARNING: Source file at path `{:#?}` is empty. Skipping...", source);
            continue;
        }

        content_array.push((source, destination, source_content));
    }
    // Parse all source files through the template engine
    let mut template_engine = TemplateEngine::default();

    for (_, _, source_content) in content_array {
        // TODO: Change template engine code to parse multiple files while returning their content
        template_engine.evaluate_template_file(source_content);

    }
}
