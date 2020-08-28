use lazy_static::lazy_static;
use std::sync::Mutex;
use chrono::{Timelike, Utc};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::{output, FuzzParams, FuzzType};

lazy_static! {
    static ref RESULTS: Mutex<Vec<FuzzResult>> = Mutex::new(vec![]);
}

#[derive(Debug, Clone)]
pub struct FuzzResult
{
    pub status_code: i32,
    pub url: String
}

pub fn add(status_code: i32, url: &str)
{
    RESULTS.lock().unwrap().push(
        FuzzResult{status_code:status_code, url: url.to_owned()}
    );
}

pub fn get_sorted_results() -> Vec<FuzzResult>
{
    let mut results: Vec<FuzzResult> = vec![];
    for i in RESULTS.lock().unwrap().iter()
    {
        results.push(i.clone());
    }
    results.sort_by(|a,b| a.status_code.cmp(&b.status_code));

    results
}

pub fn write_results_to_file(fuzz_params: FuzzParams)
{
    let results: Vec<FuzzResult> = get_sorted_results();

    let path = Path::new(fuzz_params.output.as_str());

    let file = match File::create(&path) {
        Err(_) => {
            output::error("Error writing results to file".to_owned(), false);
            panic!()// dead code as error cancels process
        },
        Ok(file) => { file },
    };

    let now = Utc::now();
    let (is_pm, hour) = now.hour12();
    let time = format!("{:02}:{:02}:{:02} {}\n", hour, now.minute(), now.second(), if is_pm { "PM" } else { "AM" });
    

    if fuzz_params.fuzz_type == FuzzType::IgnoreCodes || fuzz_params.fuzz_type == FuzzType::AcceptCodes
    {
        try_write_to_file(&file, format!("Results for {:?} {:?} {}", fuzz_params.fuzz_type, fuzz_params.http_codes, time));
    }
    else if fuzz_params.fuzz_type == FuzzType::IgnoreString || fuzz_params.fuzz_type == FuzzType::AcceptString
    {
        try_write_to_file(&file, format!("Results for {:?} \"{}\" {}", fuzz_params.fuzz_type, fuzz_params.search_string, time));
    }    

    let mut current_code = -999;

    for r in results
    {
        if r.status_code != current_code
        {
            current_code = r.status_code;

            let line: String = format!("\n\n[{}] Responses\n", r.status_code);
            try_write_to_file(&file, line);
        }

        let line: String = format!("[{}] {}\n", r.status_code, r.url);
        try_write_to_file(&file, line);
    }

    output::results_file_location(fuzz_params.output);
}

fn try_write_to_file(mut file: &File, line: String)
{
    match file.write(line.as_str().as_bytes()) {
        Err(_) => output::error("Error writing results to file".to_owned(), false),
        Ok(_) => {}
    }   
}