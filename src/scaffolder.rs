use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn scaffold(path: &PathBuf, force: bool) -> io::Result<()> {
    // check if it's safe to scaffold the new site by counting the children
    if !force && path.exists() {
        let iter = try!(path.read_dir());

        if iter.count() > 0 {
            let error = "The path exists and is not empty. To overwrite files in the path, run \
                         this command again with the \"--force\" flag.";

            return Err(io::Error::new(io::ErrorKind::Other, error));
        }
    }

    // create the directories and files
    try!(create_dirs(&path));
    try!(create_files(&path));

    Ok(())
}

fn create_dirs(path: &PathBuf) -> io::Result<()> {
    let dirs = [path.to_path_buf(),
                path.join("src"),
                path.join("src/_drafts"),
                path.join("src/_layouts"),
                path.join("src/_partials"),
                path.join("src/_posts")];

    for d in dirs.into_iter() {
        if !d.exists() {
            try!(fs::create_dir(&d))
        }
    }

    Ok(())
}

fn create_files(path: &PathBuf) -> io::Result<()> {
    let files = [(path.join(".gitignore"), include_str!("templates/gitignore")),
                 (path.join("gizmo.toml"),
                  include_str!("templates/gizmo.toml"))];

    for f in files.into_iter() {
        let mut handle = try!(File::create(&f.0));

        try!(handle.write_all(&f.1.as_bytes()))
    }

    Ok(())
}
