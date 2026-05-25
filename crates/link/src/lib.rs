use std::fs;
use std::path::Path;

pub fn link(input: &Path, output: &Path) {
    if let Err(e) = junction::create(input, output) {
        panic!("[mcbe] Link failed to create: {e}")
    }
}

pub fn unlink(input: &Path) {
    if let Err(e) = junction::delete(input) {
        panic!("[mcbe] Link failed to delete: {e}")
    }
    if let Err(e) = fs::remove_dir(input) {
        panic!("[mcbe] Folder failed to delete: {e}")
    }
}
