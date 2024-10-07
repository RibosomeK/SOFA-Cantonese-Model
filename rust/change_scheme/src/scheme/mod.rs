use std::collections::HashMap;
use std::fs::read_to_string;
use std::process::exit;

use log::error;

#[derive(Debug)]
pub struct Pair {
    pub new: String,
    pub old: Vec<String>,
}

impl Pair {
    fn parse(ori: &Vec<&str>, new: &Vec<&str>) -> Vec<Pair> {
        let mut pairs: Vec<Pair> = Vec::new();
        let mut start = 0;
        let mut buf = String::new();
        for i in 0..ori.len() {
            let ph = new[i];
            if !ph.is_empty() {
                if !buf.is_empty() {
                    pairs.push(Pair {
                        new: buf.clone(),
                        old: ori[start..i].iter().map(|s| s.to_string()).collect(),
                    });
                }
                buf.clear();
                buf.push_str(ph);
                start = i;
            }
        }
        if buf.is_empty() {
            error!("Invalid data:\n{:?}\n{:?}", ori, new);
            exit(-1);
        }
        pairs.push(Pair {
            new: buf,
            old: ori[start..].iter().map(|s| s.to_string()).collect(),
        });
        pairs
    }
}

pub type Scheme = HashMap<String, Vec<Pair>>;

pub fn read_scheme(path: &str) -> Scheme {
    let mut scheme: Scheme = HashMap::new();
    let tmp = read_to_string(path).unwrap();
    let mut lines = tmp.lines();
    let mut ori_line = lines.next();
    while ori_line.is_some() {
        let mut ori = ori_line
            .unwrap()
            .split(',')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        let mut new = lines.next().unwrap().split(',').collect::<Vec<&str>>();

        let word = ori[0];
        ori.remove(0);
        new.remove(0);
        let pair = Pair::parse(&ori, &new);
        scheme.insert(word.to_string(), pair);
        ori_line = lines.next();
    }
    scheme
}
