use std::fs;
use std::path::Path;

pub fn read_lines_text_plain(path: &Path) -> Result<Vec<String>, String>  {
    let read_file = fs::read_to_string(path)
        .map_err(|e| e.to_string())?;
    Ok(
        read_file.lines()
            .map(|lines| lines.to_string())
            .collect::<Vec<String>>()
    )

}