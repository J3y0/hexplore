use std::{fs, path::Path};

use ratatui::text::{Line, Text};
use sha2::{self, Digest};

#[derive(Default, Debug)]
pub struct FileInfo {
    pub name: String,
    pub sha256: String,
    pub content: Vec<u8>,
    pub size: usize,
    #[cfg(target_os = "linux")]
    pub filetype: String,
}

impl FileInfo {
    pub fn new(filename: &str) -> anyhow::Result<Self> {
        let strict_filename = Path::new(filename).file_name().unwrap().to_os_string();
        let content = fs::read(filename)?;
        let sha256 = calc_sha256(&content);
        #[cfg(target_os = "linux")]
        let filetype = get_filetype(filename)?;

        Ok(FileInfo {
            name: strict_filename.into_string().unwrap_or_default(),
            size: content.len(),
            content,
            sha256,
            #[cfg(target_os = "linux")]
            filetype,
        })
    }

    pub fn to_text(&self) -> Text<'_> {
        Text::from(vec![
            Line::from(vec!["name:   ".into(), self.name.as_str().into()]),
            Line::from(vec![
                "size:   ".into(),
                self.size.to_string().into(),
                " bytes".into(),
            ]),
            #[cfg(target_os = "linux")]
            Line::from(vec!["type:   ".into(), self.filetype.as_str().into()]),
            Line::from(vec!["sha256: ".into(), self.sha256.as_str().into()]),
        ])
    }
}

fn calc_sha256(content: &[u8]) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(content);
    let hash = hasher.finalize();

    format!("{hash:x}")
}

#[cfg(target_os = "linux")]
fn get_filetype(filename: &str) -> anyhow::Result<String> {
    use std::process::Command;

    // Run 'file' command
    let file_output = Command::new("file").arg(filename).output()?;

    let file_output = String::from_utf8_lossy(&file_output.stdout);
    let (_, filetype) = file_output.split_once(": ").unwrap_or(("", ""));
    let filetype = filetype.trim();

    Ok(filetype.to_string())
}
