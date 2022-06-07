use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
#[cfg(test)]
mod test;

const OUTPUT_DIR: &str = "dist";
#[derive(Parser)]
#[clap(version = "1.0", author = "usagrada <usagrada.kind@gmail.com>")]
struct Opts {
    input: Vec<String>,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("Using input file: {:?}", opts.input);

    for file in &opts.input {
        file_read(file);
    }
}

fn exit() {
    std::process::exit(0x0100);
}

fn dir_type_check(input: &str) -> bool {
    let disable_dir_type = ["dist/"];
    if disable_dir_type
        .iter()
        .fold(false, |flag, f| flag || input.contains(f))
    {
        true
    } else {
        false
    }
}

fn file_type_check(input: &str) -> bool {
    let enable_file_type = [".md", ".mdx", ".txt"];
    if enable_file_type
        .iter()
        .fold(false, |flag, f| flag || input.ends_with(f))
    {
        true
    } else {
        false
    }
}

fn file_read(input: &str) {
    if !file_type_check(input) {
        println!("This CLI doesn't allow file type of {} !!", input);
        exit();
    }
    if dir_type_check(input) {
        println!("This CLI doesn't allow dist directory's file");
        return;
    }

    let mut file = File::open(input).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File Read Error");
    let res = parse_file(&mut contents, input);
    let mut dir: Vec<_> = input.split("/").collect();
    dir.pop();
    let output_dir = [OUTPUT_DIR, &dir.join("/")].join("/");
    let output_file = [OUTPUT_DIR, input].join("/");
    println!("output dir: {}", output_dir);
    std::fs::create_dir_all(output_dir).expect("Directory Create Error");
    let mut resfile = File::create(output_file).expect("create file error");
    resfile.write_all(res.as_bytes()).expect("write error");
}

fn parse_file(input: &str, filename: &str) -> String {
    let mut dir: Vec<_> = filename.split("/").collect();
    dir.pop();

    // file名の解決に (相対パス + )ファイル名を用いる
    // alphabet or 数字 or 記号(-_) を許す(相対パスのために . / を許している)
    let re =
        Regex::new(r"(?m)^\s*#import\((?P<filename>[[:alnum:]_/\-\.]+\.[[:alpha:]]+)\)").unwrap();
    let result = re.replace_all(input, |caps: &regex::Captures| {
        println!(
            "replace import file: '{}' in '{}'",
            &caps["filename"], filename
        );
        let mut file = dir.clone();
        file.push(&caps["filename"]);
        let mut read_file = File::open(file.join("/")).expect("file not found");
        let mut contents = String::new();
        read_file
            .read_to_string(&mut contents)
            .expect("File Read Error");
        contents
    });

    result.into_owned()
}
