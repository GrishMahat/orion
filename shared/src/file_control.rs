use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub is_file: bool,
    pub is_directory: bool,
    pub modified: std::time::SystemTime,
}

pub struct FileControl;

impl FileControl {
    pub fn new() -> Self {
        FileControl
    }

    pub fn list_directory(&self, path: &Path) -> Result<Vec<FileInfo>> {
        let mut files = Vec::new();
        
        for entry in fs::read_dir(path)
            .with_context(|| format!("Failed to read directory: {:?}", path))? {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;
            
            files.push(FileInfo {
                path: path.clone(),
                size: metadata.len(),
                is_file: metadata.is_file(),
                is_directory: metadata.is_dir(),
                modified: metadata.modified()?,
            });
        }
        
        Ok(files)
    }

    pub fn create_directory(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {:?}", path))
    }

    pub fn delete_path(&self, path: &Path) -> Result<()> {
        if path.is_file() {
            fs::remove_file(path)
                .with_context(|| format!("Failed to delete file: {:?}", path))?;
        } else if path.is_dir() {
            fs::remove_dir_all(path)
                .with_context(|| format!("Failed to delete directory: {:?}", path))?;
        }
        Ok(())
    }

    pub fn move_path(&self, from: &Path, to: &Path) -> Result<()> {
        fs::rename(from, to)
            .with_context(|| format!("Failed to move from {:?} to {:?}", from, to))
    }

    pub fn copy_path(&self, from: &Path, to: &Path) -> Result<()> {
        if from.is_file() {
            fs::copy(from, to)
                .with_context(|| format!("Failed to copy file from {:?} to {:?}", from, to))?;
        } else if from.is_dir() {
            self.copy_directory(from, to)?;
        }
        Ok(())
    }

    fn copy_directory(&self, from: &Path, to: &Path) -> Result<()> {
        fs::create_dir_all(to)?;
        
        for entry in fs::read_dir(from)? {
            let entry = entry?;
            let path = entry.path();
            let new_path = to.join(path.file_name().unwrap());
            
            if path.is_file() {
                fs::copy(&path, &new_path)?;
            } else if path.is_dir() {
                self.copy_directory(&path, &new_path)?;
            }
        }
        
        Ok(())
    }
} 