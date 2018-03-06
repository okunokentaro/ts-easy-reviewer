use std::{fs, io, path};
use reader::read_file;
use path::get_path_string;
use rule::{Report, Rule};

fn visit_dirs(
    dir: &path::Path,
    reports: &mut Vec<Report>,
    rules: &Vec<Rule>,
    cb: &Fn(&mut Vec<Report>, &Vec<Rule>, &fs::DirEntry),
) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, reports, rules, cb)?;
            } else {
                cb(reports, rules, &entry);
            }
        }
    }
    Ok(())
}

pub fn review_files(path_string: Option<String>, rules: Vec<Rule>) -> Result<(), String> {
    let path_string_ref = path_string.unwrap();
    let path = path::Path::new(&path_string_ref);

    let mut reports = vec![];
    visit_dirs(
        path,
        &mut reports,
        &rules,
        &|reports: &mut Vec<Report>, rules: &Vec<Rule>, entry: &fs::DirEntry| {
            let buf_path = entry.path();
            let path_string = get_path_string(&buf_path);

            if path_string.contains(".DS_Store") {
                return;
            } else if !path_string.contains(".ts") {
                return;
            }

            let code = read_file(&buf_path).unwrap();

            for rule in rules {
                let report = rule.check(&buf_path, &code);
                reports.push(report);
            }
        },
    ).unwrap();

    let reports = reports
        .iter()
        .filter(|r| r.exists())
        .collect::<Vec<&Report>>();
    if 0 < reports.len() {
        for r in reports {
            for output in &r.rejected {
                println!("{}", output);
            }
        }
        ::std::process::exit(1)
    }

    Ok(())
}
