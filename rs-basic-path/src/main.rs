use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // parse a path
    let path = Path::new("foo/bar/baz.txt");
    println!("Folder containing the file: {:?}", path.parent().unwrap());
    println!("Name of the file: {:?}", path.file_stem().unwrap());
    println!("Extension of the file: {:?}", path.extension().unwrap());

    // create a path
    let mut path = PathBuf::from("./");
    path.push("foo");
    path.push("bar");
    path.push("baz");
    path.set_extension("txt");
    println!("Path: {:?}", path);

    // create a path
    let path = ["./", "foo", "bar", "baz.txt"]
        .iter()
        .collect::<PathBuf>();
    println!("Path: {:?}", path);

    // validate folder
    let path = Path::new("/root");
    println!("Is the path a directory? {:?}", path.is_dir());
    println!("Is the path a file? {:?}", path.is_file());

    let data = path.metadata().unwrap();
    println!("The metadata of path is:\n{:?}", data);

    // parse directory
    let path = Path::new("/home/vscode");
    for file in path.read_dir().expect("read_dir call failed") {
        let file = file.unwrap();
        println!("Name: {:?}", file.file_name());
        println!("Path: {:?}", file.path());
        println!("Is the path a directory? {:?}", file.path().is_dir());
        println!("Is the path a file? {:?}", file.path().is_file());
    }

    // current path
    let curr_path = env::current_dir().expect("current_dir call failed");
    println!("Current path: {:?}", curr_path);

    // create dirs
    println!("Create a directory: {:?}", fs::create_dir("/home/vscode/foo"));
    println!("Create all directory: {:?}", fs::create_dir_all("/home/vscode/bar/baz"));
    // delete dirs
    println!("Remove a directory: {:?}", fs::remove_dir("/home/vscode/foo"));
    println!("Remove all directory: {:?}", fs::remove_dir_all("/home/vscode/bar"));
}
