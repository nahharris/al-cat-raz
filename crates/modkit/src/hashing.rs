use std::{fs, path::Path};

use anyhow::Result;
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

pub fn hash_dir_sha256(root: &Path) -> Result<String> {
    let mut hasher = Sha256::new();

    let mut files: Vec<_> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    files.sort_by_key(|e| e.path().to_path_buf());

    for entry in files {
        let rel = entry.path().strip_prefix(root).unwrap_or(entry.path());
        hasher.update(rel.to_string_lossy().as_bytes());
        hasher.update([0u8]);

        let bytes = fs::read(entry.path())?;
        hasher.update(bytes.len().to_le_bytes());
        hasher.update(bytes);
    }

    Ok(format!("{:x}", hasher.finalize()))
}
