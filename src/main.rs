use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
#[cfg(test)]
mod test;

const OUTPUT_DIR: &str = "dist";
/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Parser)]
#[clap(version = "1.0", author = "usagrada <usagrada.kind@gmail.com>")]
struct Opts {
  /// Sets a custom config file. Could have been an Option<T> with no default too
  // #[clap(short, long, default_value = "default.conf")]
  // config: String,
  // Some input. Because this isn't an Option<T> it's required to be used
  #[clap(about = "input file name")]
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

  let mut file = File::open(input).expect("file not found");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("File Read Error");
  let res = parse_file(&mut contents, input);
  std::fs::create_dir_all(OUTPUT_DIR).expect("Directory Create Error");
  let mut resfile = File::create([OUTPUT_DIR, input].join("/")).expect("create file error");
  resfile.write_all(res.as_bytes()).expect("write error");
  // file.flush().expect("flush error");
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
