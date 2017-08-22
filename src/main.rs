extern crate regex;
use regex::Regex;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;

fn usage() {
    println!("rsgrep PATTERN FILENAME")
}

fn main() {
    let pattern = match env::args().nth(1) {
        Some(pattern) => pattern,
        None => {
            usage();
            return;
        }
    };

    let filename = match env::args().nth(2) {
        Some(filename) => filename,
        None => {
            usage();
            return;
        }
    };
    let file = match File::open(&filename) {
        // 成功すれば取り出す。
        Ok(file) => file,
        // ファイルが見つからないなどのエラーの場合はそのままプログラム終了
        Err(e) => {
            println!("An error occurred while opening file {}: {}", filename, e);
            return;
        }
    };
    let input = BufReader::new(file);
    for line in input.lines() {
        let line = match line {
            Ok(line) => line,
            // 失敗したらそのまま終了することにする。
            Err(e) => {
                println!("An error occurred while reading a line {}", e);
                return;
            }
        };
        println!("{}", line);
    }
}
