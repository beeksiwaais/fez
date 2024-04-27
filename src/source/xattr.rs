// extract the resource fork inside the macos attr
// in com.apple.ResourceFork 

use std::path::PathBuf;
use xattr::get; 

pub fn is_present(path: &PathBuf) -> bool {
    // retrieve attributes for a given file
    let attrs = get(path, "com.apple.ResourceFork").unwrap();
    println!("{:?}", attrs);
}

//pub fn extract()