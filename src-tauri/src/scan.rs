use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::io::Error;
use std::path::{Path, PathBuf};

// pub enum Filter {
//   ExcludeName(PathBuf),
//   ExcludeContainsItem(PathBuf),
// }

#[derive(Debug)]
pub enum ScanError {
  NotADirectory,
  IOError(Error),
}

/// Scan a given folder, return a vector of paths `Vec<PathBuf>`
pub fn scan_dir(path: impl AsRef<Path>) -> Result<Vec<PathBuf>, ScanError> {
  let path = path.as_ref();

  // make sure path is a directory
  let metadata = path.metadata().map_err(ScanError::IOError)?;
  if !metadata.is_dir() { return Err(ScanError::NotADirectory); }

  let mut items = vec![];
  let mut unscanned_dirs = vec![];

  // scan the path for initial list of folders
  let dir_iter = fs::read_dir(&path).map_err(ScanError::IOError)?;
  classify_dir_items(dir_iter.flatten(), &mut items, &mut unscanned_dirs);

  // scan remaining folders
  while !unscanned_dirs.is_empty() {
    if let Ok(dir_iter) = fs::read_dir(unscanned_dirs.pop().unwrap()) {
      classify_dir_items(dir_iter.flatten(), &mut items, &mut unscanned_dirs);
    }
  }

  Ok(items)
}

/// Classify incoming DirEntries as either items or folders to be further scanned.
fn classify_dir_items<T>(dir_iter: T, items: &mut Vec<PathBuf>, unscanned_dirs: &mut Vec<PathBuf>)
where
  T: Iterator<Item = DirEntry>
{
  for entry in dir_iter {
    if let Ok(metadata) = entry.metadata() {
      if metadata.is_dir() {
        unscanned_dirs.push(entry.path());
      } else {
        items.push(entry.path());
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::time::Instant;

  #[test]
  fn benchmark() -> () {
    let path = PathBuf::from("D:/");
    let start = Instant::now();
    let r = scan_dir(path);
    let duration = start.elapsed();

    println!("Time elapsed: {:?}", duration);

    match r {
      Ok(items) => {
        println!("Items: {}", items.len());
        // 151293
      }
      Err(e) => { dbg!(e); }
    }
  }
}
