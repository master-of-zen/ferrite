use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileOperationError {
    #[error("Failed to move file to trash: {0}")]
    TrashError(#[from] trash::Error),
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, FileOperationError>;

/// File operations for managing image files
pub struct FileOperations;

impl FileOperations {
    /// Delete a file by moving it to trash
    ///
    /// # Arguments
    /// * `path` - Path to the file to delete
    ///
    /// # Returns
    /// * `Ok(())` if the file was successfully moved to trash
    /// * `Err(FileOperationError)` if the operation failed
    pub fn delete_file<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(FileOperationError::FileNotFound(path.to_path_buf()));
        }

        trash::delete(path)?;
        tracing::info!("Successfully moved file to trash: {}", path.display());
        Ok(())
    }

    /// Permanently delete a file (bypassing trash)
    ///
    /// # Arguments
    /// * `path` - Path to the file to delete
    ///
    /// # Returns
    /// * `Ok(())` if the file was successfully deleted
    /// * `Err(FileOperationError)` if the operation failed
    ///
    /// # Warning
    /// This permanently deletes the file and cannot be undone
    pub fn delete_file_permanent<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(FileOperationError::FileNotFound(path.to_path_buf()));
        }

        std::fs::remove_file(path)?;
        tracing::warn!("Permanently deleted file: {}", path.display());
        Ok(())
    }
}
