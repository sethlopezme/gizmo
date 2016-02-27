use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub fn scaffold(path: &str, force: bool) -> io::Result<()> {
    let base_path = Path::new(path).to_path_buf();

    // check if it's safe to scaffold the new site by counting the children
    if !force && base_path.exists() {
        let iter = try!(base_path.read_dir());

        if iter.count() > 0 {
            let error = "The path exists and is not empty. To overwrite files in the path, run \
                         this command again with the \"--force\" flag.";

            return Err(io::Error::new(io::ErrorKind::Other, error));
        }
    }

    // create the directories and files
    try!(create_dirs(&base_path));
    try!(create_files(&base_path));

    Ok(())
}

fn create_dirs(base_path: &PathBuf) -> io::Result<()> {
    let dirs = [base_path.to_path_buf(),
                base_path.join("src"),
                base_path.join("src/_drafts"),
                base_path.join("src/_layouts"),
                base_path.join("src/_partials"),
                base_path.join("src/_posts")];

    for d in dirs.into_iter() {
        if !d.exists() {
            try!(fs::create_dir(&d))
        }
    }

    Ok(())
}

fn create_files(base_path: &PathBuf) -> io::Result<()> {
    let files = [(base_path.join(".gitignore"),
                  include_str!("templates/gitignore")),
                 (base_path.join("gizmo.toml"),
                  include_str!("templates/gizmo.toml"))];

    for f in files.into_iter() {
        let mut handle = try!(File::create(&f.0));

        try!(handle.write_all(&f.1.as_bytes()))
    }

    Ok(())
}
