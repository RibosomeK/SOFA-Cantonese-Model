mod scheme;
mod textgrid;

use std::fs::{create_dir_all, read_dir};
use std::path::{Path, PathBuf};
use std::process::exit;

use log::{error, info, warn, LevelFilter};

use clap::{arg, value_parser, ArgAction, Command};
use scheme::Pair;
use textgrid::Interval;

use crate::scheme::{read_scheme, Scheme};
use crate::textgrid::TextGrid;

fn iter_indexes(tg: &TextGrid) -> Vec<(usize, (usize, usize))> {
    let mut indexes: Vec<(usize, (usize, usize))> = Vec::new();
    let mut start = 0;
    for (i, word_ivl) in tg.items[0].intervals.iter().enumerate() {
        for (j, ph_ivl) in tg.items[1].intervals[start..].iter().enumerate() {
            if ph_ivl.max_time == word_ivl.max_time {
                indexes.push((i, (start, j + start)));
                start = start + j + 1;
                break;
            }
        }
    }
    indexes
}

fn change_scheme(scheme: &Scheme, tg: &TextGrid) -> TextGrid {
    let mut new_tg = tg.clone();
    new_tg.items[1].intervals.clear();
    for (word_idx, (start, end)) in iter_indexes(tg) {
        let word = &tg.items[0].intervals[word_idx].text;
        let mut count = 0;
        match scheme.get(word.as_str()) {
            Some(pairs) => {
                let old_ph = tg.items[1].intervals[start..end + 1]
                    .iter()
                    .map(|ivl| ivl.text.as_str())
                    .collect::<Vec<&str>>();
                let sch_new_ph = pairs.iter().map(|p| p.new.as_str()).collect::<Vec<&str>>();
                let sch_old_ph = pairs
                    .iter()
                    .flat_map(|p| &p.old)
                    .map(|s| s.as_str())
                    .collect::<Vec<&str>>();
                if sch_old_ph != old_ph {
                    if sch_new_ph == old_ph {
                        info!("Word \"{word}\" already in given scheme: {:?}", old_ph);
                        info!("Skipped");
                    } else {
                        warn!(
                            "Word: \"{word}: {:?}\" does not match the given scheme: {:?}",
                            old_ph, sch_old_ph
                        );
                        warn!("Skipped");
                    }
                    for i in start..end + 1 {
                        new_tg.items[1]
                            .intervals
                            .push(tg.items[1].intervals[i].clone());
                    }
                    continue;
                }
                for pair in pairs {
                    new_tg.items[1].intervals.push(Interval {
                        min_time: tg.items[1].intervals[start + count].min_time,
                        max_time: tg.items[1].intervals[start + count + pair.old.len() - 1]
                            .max_time,
                        text: pair.new.clone(),
                    });
                    count += pair.old.len();
                }
            }
            None => {
                warn!("Unknown words: \"{word}\"");
                warn!("Skipped");
                for i in start..end + 1 {
                    new_tg.items[1]
                        .intervals
                        .push(tg.items[1].intervals[i].clone());
                }
                continue;
            }
        }
    }
    new_tg
}

const DEFAULT_OUT: &str = "./out";
const DEFAULT_SCHEME: &str = "./configs/cantonese-two-seg.csv";

fn main() {
    let matches = Command::new("Change Scheme")
        .about("A cli app to change dictionary scheme of TextGrid files.")
        .version("0.1")
        .arg(
            arg!([TextGrids] "The directory of textgrid files, non recursive.")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-s --scheme <PATH> "The path of scheme file.")
                .required(false)
                .value_parser(value_parser!(PathBuf))
                .default_value(DEFAULT_SCHEME),
        )
        .arg(
            arg!(-o --out <DIR> "The path of output directory.")
                .required(false)
                .value_parser(value_parser!(PathBuf))
                .default_value(DEFAULT_OUT),
        )
        .arg(arg!(-d --debug "Turn on debug mode").action(ArgAction::SetTrue))
        .get_matches();

    if matches.get_flag("debug") {
        env_logger::Builder::new()
            .filter_level(LevelFilter::Info)
            .init();
    } else {
        env_logger::init();
    }

    let tg_dir = if let Some(tg_dir) = matches.get_one::<PathBuf>("TextGrids") {
        info!("TextGrid directory: {}", tg_dir.display());
        if !Path::exists(&tg_dir) {
            error!("{} does not exists", tg_dir.display());
            exit(-1);
        };
        tg_dir.to_owned()
    } else {
        error!("No TextGrid directory, which is required.");
        exit(-1)
    };

    let scheme_path = matches.get_one::<PathBuf>("scheme").unwrap();
    let output_dir = matches.get_one::<PathBuf>("out").unwrap();

    if !Path::exists(&scheme_path) {
        error!("{} does not exist.", scheme_path.display());
        error!("Please provide a valid scheme file path.");
        exit(-1)
    }
    if !Path::exists(&output_dir) {
        match create_dir_all(&output_dir) {
            Ok(_) => {}
            Err(err) => {
                error!(
                    "{} does not exists and failed to create it due to {err}.",
                    output_dir.display()
                );
                exit(-1);
            }
        }
    }

    info!("Using dictionary scheme from: {}", scheme_path.display());
    info!(
        "New TextGrid files will be save to: {}",
        output_dir.display()
    );

    let mut scheme = read_scheme(match scheme_path.to_str() {
        Some(s) => s,
        None => {
            warn!("Fail to read scheme: {}", scheme_path.display());
            exit(-1);
        }
    });
    scheme.insert(
        "SP".to_string(),
        vec![Pair {
            new: "SP".to_string(),
            old: vec!["SP".to_string()],
        }],
    );
    scheme.insert(
        "AP".to_string(),
        vec![Pair {
            new: "AP".to_string(),
            old: vec!["AP".to_string()],
        }],
    );

    let paths = match read_dir(tg_dir) {
        Ok(paths) => paths,
        Err(err) => {
            error!("Unable to read TextGrid directory due to {}", err);
            exit(-1);
        }
    };
    for path in paths {
        let entry = match path {
            Ok(entry) => entry,
            Err(err) => {
                warn!("Fail to get directory entry due to: {err}");
                continue;
            }
        }
        .path();
        if !entry.is_file() {
            continue;
        }
        let ext = match entry.extension() {
            Some(ext) => ext,
            None => continue,
        };
        let basename = entry.file_name().unwrap();
        match ext.to_str() {
            Some("TextGrid") => {
                let mut save_path = PathBuf::from(&output_dir);
                save_path.push(basename.to_str().unwrap());
                info!("Change file: {}", basename.to_str().unwrap());
                change_scheme(&scheme, &TextGrid::read(entry.to_str().unwrap()))
                    .save(save_path.to_str().unwrap());
            }
            _ => continue,
        };
    }
}
