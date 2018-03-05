use std::{fs, io, path};
use reader::read_file;
use config::Config;

fn visit_dirs(
    dir: &path::Path,
    config: &Config,
    cb: &Fn(&Config, &fs::DirEntry),
) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, config, cb)?;
            } else {
                cb(config, &entry);
            }
        }
    }
    Ok(())
}

fn get_path_string(path: &path::PathBuf) -> String {
    path.clone().into_os_string().into_string().unwrap()
}

pub fn review_files(config_: Config) {
    visit_dirs(
        path::Path::new("./"),
        &config_,
        &|config: &Config, entry: &fs::DirEntry| {
            let buf_path = entry.path();
            let path_string = get_path_string(&buf_path);

            if path_string.contains(".DS_Store") {
                return;
            } else if !path_string.contains(".ts") {
                return;
            }

            let result = read_file(&buf_path).unwrap();

            match config.rules {
                Some(ref rules) => {
                    for rule in rules {
                        println!("{:?}", rule);
                    }
                    Some(())
                }
                None => None
            };
            println!("{:?}", &buf_path);
            println!("{}", result);
        },
    ).unwrap();
}
