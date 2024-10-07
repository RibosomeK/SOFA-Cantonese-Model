use core::fmt;
use log::error;
use regex::Regex;
use std::{
    env::current_dir,
    fs::{read_to_string, File},
    io::Write,
    path::Path,
    process::exit,
    sync::LazyLock,
};

static NUMERIC_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(\w* *\w+) *= *([\d.]+)"#).unwrap());
static QUOTE_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(\w* *\w+) *= *\"([^\"]*)\""#).unwrap());

#[derive(Clone)]
pub struct Interval {
    pub min_time: f64,
    pub max_time: f64,
    pub text: String,
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "interval: \n   xmin = {}\n   xmax = {}\n   text = \"{}\"",
            self.min_time, self.max_time, self.text
        )
    }
}
#[derive(Clone)]
pub struct IntervalTier {
    pub min_time: f64,
    pub max_time: f64,
    pub name: String,
    pub intervals: Vec<Interval>,
}

impl IntervalTier {
    pub fn size(&self) -> usize {
        self.intervals.len()
    }
}
#[derive(Clone)]
pub struct TextGrid {
    pub min_time: f64,
    pub max_time: f64,
    pub items: Vec<IntervalTier>,
}

impl TextGrid {
    pub fn read(file: &str) -> TextGrid {
        let file = match read_to_string(Path::new(file)) {
            Ok(f) => f,
            Err(_) => {
                let cwd = current_dir().unwrap();
                println!("Can not open {file}, please check again!");
                println!("Current working directory: {}", cwd.display());
                exit(-1);
            }
        };
        let mut lines = file.lines();
        let type_line = lines.next().unwrap();
        let obj_line = lines.next().unwrap();
        if !valid_file_type(type_line) {
            println!("[WARN]:\nUnknown file type: {}", type_line);
        }
        if !valid_obj_class(obj_line) {
            println!("[WARN]:\nUnknown object class: {}", obj_line);
        }

        let _ = lines.next();

        let mut tg = TextGrid {
            min_time: parse_num(lines.next().unwrap()),
            max_time: parse_num(lines.next().unwrap()),
            items: Vec::new(),
        };

        let _ = lines.next();
        let item_count = parse_num(lines.next().unwrap()) as i32;
        let _ = lines.next();

        for _ in 0..item_count {
            let _ = lines.next();
            if parse_text(lines.next().unwrap()) == "IntervalTier" {
                let name = parse_text(lines.next().unwrap()).to_string();
                let min = parse_num(lines.next().unwrap());
                let max = parse_num(lines.next().unwrap());
                let ivl_count = parse_num(lines.next().unwrap()) as i32;
                let mut ivl_tier = IntervalTier {
                    min_time: min,
                    max_time: max,
                    name: name,
                    intervals: Vec::new(),
                };
                for _ in 0..ivl_count {
                    let _ = lines.next();
                    let min = parse_num(lines.next().unwrap());
                    let max = parse_num(lines.next().unwrap());
                    let text = parse_text(lines.next().unwrap()).to_string();
                    ivl_tier.intervals.push(Interval {
                        min_time: min,
                        max_time: max,
                        text: text,
                    });
                }
                tg.items.push(ivl_tier);
            } else {
                println!("unsupported tier");
                exit(-1)
            }
        }
        tg
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn save(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        file.write_all(self.to_string().as_bytes()).unwrap();
    }

    pub fn to_string(&self) -> String {
        let mut string = format!(
            "File type = \"ooTextFile\"\n\
            Object class = \"TextGrid\"\n\
            \n\
            xmin = {}\n\
            xmax = {}\n",
            self.min_time, self.max_time,
        );

        let size = self.size();
        if size == 0 {
            string.push_str("tiers? <absent>\n");
            return string;
        } else {
            string.push_str("tiers? <exists>\n");
            string.push_str(&format!("size = {}\n", size));
            string.push_str("item []:\n");
        }

        let indent = "    ";
        let indent2 = "        ";
        let indent3 = "            ";
        let indent4 = "                ";
        for (idx, item) in self.items.iter().enumerate() {
            string.push_str(&format!(
                "{indent}item [{}]\n\
                    {indent2}class = \"IntervalTier\"\n\
                    {indent2}name = \"{}\"\n\
                    {indent2}xmin = {}\n\
                    {indent2}xmax = {}\n\
                    {indent2}intervals: size = {}\n",
                idx + 1,
                &item.name,
                item.min_time,
                item.max_time,
                item.size()
            ));
            for (jdx, ivl) in item.intervals.iter().enumerate() {
                string.push_str(&format!(
                    "{indent3}intervals [{}]\n\
                        {indent4}xmin = {}\n\
                        {indent4}xmax = {}\n\
                        {indent4}text = \"{}\"\n",
                    jdx + 1,
                    ivl.min_time,
                    ivl.max_time,
                    ivl.text
                ));
            }
        }
        string
    }
}

fn valid_file_type(line: &str) -> bool {
    match Regex::new(r#"^File *type *= *\"ooTextFile\""#)
        .unwrap()
        .captures(line)
    {
        Some(_) => true,
        None => false,
    }
}

fn valid_obj_class(line: &str) -> bool {
    match Regex::new(r#"^Object *class *= *\"TextGrid\""#)
        .unwrap()
        .captures(line)
    {
        Some(_) => true,
        None => false,
    }
}

fn parse_num(line: &str) -> f64 {
    match match NUMERIC_PATTERN.captures(line) {
        Some(c) => c,
        None => {
            error!("no match: {}", line.trim());
            exit(-1)
        }
    }
    .get(2)
    {
        Some(m) => m,
        None => {
            error!("unknown line: {}", line.trim());
            exit(-1)
        }
    }
    .as_str()
    .parse::<f64>()
    .unwrap()
}

fn parse_text(line: &str) -> &str {
    match match QUOTE_PATTERN.captures(line) {
        Some(c) => c,
        None => {
            println!("no match: {}", line);
            exit(-1)
        }
    }
    .get(2)
    {
        Some(m) => m.as_str(),
        None => {
            println!("unknown line: {}", line);
            exit(-1)
        }
    }
}
