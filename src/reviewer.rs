use std::{fs, io, path};
use reader::read_file;
use rule::Rule;

fn visit_dirs(
    dir: &path::Path,
    rules: &Vec<Rule>,
    cb: &Fn(&Vec<Rule>, &fs::DirEntry),
) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, rules, cb)?;
            } else {
                cb(rules, &entry);
            }
        }
    }
    Ok(())
}

fn get_path_string(path: &path::PathBuf) -> String {
    path.clone().into_os_string().into_string().unwrap()
}

pub fn review_files(path_string: Option<String>, rules: Vec<Rule>) {
    let path_string_ref = path_string.unwrap();
    let path = path::Path::new(&path_string_ref);

    visit_dirs(
        path,
        &rules,
        &|rules: &Vec<Rule>, entry: &fs::DirEntry| {
            let buf_path = entry.path();
            let path_string = get_path_string(&buf_path);

            if path_string.contains(".DS_Store") {
                return;
            } else if !path_string.contains(".ts") {
                return;
            }

            let result = read_file(&buf_path).unwrap();

            for rule in rules {
                rule.check(&result);
            }
        },
    ).unwrap();
}
