use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::*;
use std::error::Error;
use std::fmt;
use std::process::Command;

#[derive(Debug)]
struct SearchResult {
    sentence: String,
    row: u32
}
#[derive(Debug)]
struct LocalFile {
    file: File,
    unlink: bool
}

#[derive(Debug)]
struct MetaResult<'a> {
    meta: HashMap<&'a str, &'a str>,
    err: Error 
}
struct BodyResult<'a> {
    body: &'a str,
    err: Error
}
fn convert_pdf_text(path: &str) -> &'static str {
    //let mut meta_result = MetaResult {meta: HashMap::new()};
    //let mut body_result = BodyResult {};
    let output = if cfg!(target_os = "linux") {
    Command::new("pdfinfo")
            .args(&[path])
            .output()
            .expect("failed to execute process");
    } else {

    };
    println!("OUTPUT IS {:?}", output);
    return "just for test";
}
impl LocalFile {
    fn new() {
        

    }
}


fn str_with_whitespace<'a>(v: &'a [(usize,&str)]) -> String {

   let mut constructed = String::from("");
   for (i, item) in v.into_iter() {
        println!(" iItem {}",item);
        constructed.push_str(*item);
        constructed.push_str(" ");

   }
   return constructed
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[2];
    let term = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut map = HashMap::new();
    let mut search_results: Vec<SearchResult> = vec![];
    let search_arr: Vec<_> = contents.split_whitespace().enumerate().collect();
    for (i, word) in &search_arr {
        if word == term {
            let before_match = &search_arr[*&(i-10) as usize..*i as usize];
            let after_match = &search_arr[*i as usize.. *&(i+10) as usize];
            let before_to_str = str_with_whitespace(before_match);
            let after_to_str = str_with_whitespace(after_match);
            let sentence = format!("{} {}",&before_to_str, &after_to_str);
            search_results.push(SearchResult {sentence:sentence, row:*i as u32});

        }
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    let mut count_vec: Vec<_> = map.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
   // println!("{:?}", count_vec);
    println!("{:?}", search_results);
}
