use std::path::Path;

pub fn link(input: &Path, output: &Path) {
    if let Err(e) = junction::create(input, output) {
        println!("[mcbe] link failed to create {e}")
    }
}

pub fn unlink(input: &Path) {
    if let Err(e) = junction::delete(input) {
        println!("[mcbe] link failed to delete {e}")
    }
}
