pub mod file {
  use std::{fs::{read_dir, metadata, Metadata, Permissions, FileType}, io, time::SystemTime, result::{Result::Ok, Result}, path::Path, fmt};

  pub struct FileStats {
    len: u64,
    modified: io::Result<SystemTime>,
    created: Result<SystemTime, io::Error>,
    permissions: Permissions,
    is_file: bool,
    is_dir: bool,
    file_type: FileType,
  }

  impl FileStats {
    pub fn new(    
      len: u64,
      modified: io::Result<SystemTime>,
      created: Result<SystemTime, io::Error>,
      permissions: Permissions,
      is_file: bool,
      is_dir: bool,
      file_type: FileType,
    ) -> Self {
      FileStats {
        len,
        modified,
        created,
        permissions,
        is_file,
        is_dir,
        file_type,
      }
    }

    pub fn get_len(&self) -> &u64 {
      &self.len
    }
    pub fn get_modified(&self) -> io::Result<SystemTime> {
      self.modified
    }
    pub fn get_created(&self) -> Result<SystemTime, io::Error> {
      self.created
    }
    pub fn get_permissions(&self) -> &Permissions {
      &self.permissions
    }
    pub fn get_is_file(&self) ->&bool {
      &self.is_file
    }
    pub fn get_is_dir(&self) ->&bool {
      &self.is_dir
    }
    pub fn get_file_type(&self) ->&FileType {
      &self.file_type
    }
  }

  pub fn explore_directory(path: &Path) {
    if let Ok(entries) = read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    explore_directory(&entry_path);
                } else {
                    println!("File: {:?}", entry_path);

                }
            }
        }
    } else {
        eprintln!("Error reading directory: {:?}", path);
    }
  }

  fn move_file(path: &str) {

    match get_file_stats(path) {
      Ok(metadata) => {
        // Use the metadata, for example:
        println!("File size: {} bytes", metadata.len());
        println!("File permissions: {:?}", metadata.permissions());
      }

      Err(err) => {
        eprintln!("{}", err)
      }
    }
  }

  fn get_file_stats(path: &str) -> FileStats {
    match metadata(path) {
      Ok(mt) => {

        let file_stats: FileStats = FileStats::new(mt.len(), mt.modified(), mt.created(), mt.permissions(),  mt.is_file(),  mt.is_dir(),  mt.file_type());
        file_stats
      }
      Err(err) => {
        eprintln!("Error obtaining file metadata for {}: {}", path, err);
        
      }
    }
  }
}