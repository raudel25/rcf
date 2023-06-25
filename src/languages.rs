use std::fs;

pub enum Languages {
    C,
    Cpp,
    Python,
}

pub fn compiler_extension<'a>(language: Languages) -> (&'a str, &'a str) {
    match language {
        Languages::C => ("gcc -o main", ".c"),
        Languages::Cpp => ("gcc -o main", ".cpp"),
        Languages::Python => ("python3", ".py"),
    }
}

pub fn check_source_file(extension: &str) -> Result<String, String> {
    let files = search(extension);

    if files.len() != 1 {
        return Err(String::from(
            "The source file do not have the correct format",
        ));
    }

    Ok(files[0].clone())
}

pub fn search(extension: &str) -> Vec<String> {
    let files: Vec<_> = fs::read_dir(".")
        .unwrap()
        .filter_map(|entry_res| {
            let entry = entry_res.unwrap();
            let file_name_buf = entry.file_name();
            let file_name = file_name_buf.to_str().unwrap();
            if !file_name.starts_with(".") && file_name.ends_with(extension) {
                Some(entry.path())
            } else {
                None
            }
        })
        .collect();

    let files: Vec<_> = files
        .into_iter()
        .map(|p| String::from(p.to_str().unwrap()))
        .collect();

    files
        .into_iter()
        .map(|s| String::from(&s[2..s.len()]))
        .collect()
}
