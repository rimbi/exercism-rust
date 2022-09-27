use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Error};

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug)]
pub struct Flags {
    flags: Vec<String>,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            flags: flags.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn has_flag(&self, f: &str) -> bool {
        self.flags.contains(&f.to_string())
    }

    pub fn is_empty(&self) -> bool {
        self.flags.is_empty()
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut res = vec![];
    for fname in files {
        let f = File::open(fname).context(format!("File {fname} does not exist"))?;
        for (n, line) in BufReader::new(f).lines().enumerate() {
            let line = line.context("Can't read the line")?;
            let (l, p) = if flags.has_flag("-i") {
                (line.to_lowercase(), pattern.to_lowercase())
            } else {
                (line.clone(), pattern.to_string())
            };
            let mut found = if flags.has_flag("-x") {
                l == p
            } else {
                l.contains(&p)
            };
            if flags.has_flag("-v") {
                found = !found;
            }
            if found {
                let mut s = vec![];
                if flags.has_flag("-l") || files.len() > 1 {
                    s.push(fname.to_string());
                }
                if flags.has_flag("-n") && !flags.has_flag("-l") {
                    s.push(format!("{}", n + 1));
                }
                if !flags.has_flag("-l") {
                    s.push(format!("{}", line));
                }
                let s = s.join(":");
                if !res.contains(&s) {
                    res.push(s);
                }
            }
        }
    }
    println!("res = {:?}", res);
    Ok(res)
}
