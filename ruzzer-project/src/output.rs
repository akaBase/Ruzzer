use ansi_term::{Style, Color};
use super::{FuzzParams, FuzzType};
use chrono::{Timelike, Utc};

pub fn help()
{
    println!("{}", Style::new().fg(Color::Blue).bold().paint("Ruzzer v1.0.0"));

    println!("{}", Style::new().bold().paint("\nArguments"));
    println!("{}", Style::new().paint("-h     --help            Show help"));
    println!("{}", Style::new().paint("-u     --url             Url with an asterisk (*) marking the fuzz position"));
    println!("{}", Style::new().paint("-w     --wordlist        Line seprated wordlist to fuzz target"));
    println!("{}", Style::new().paint("-ac    --acceptcodes     HTTP codes to accept and forward to output"));
    println!("{}", Style::new().paint("-ic    --ignorecodes     HTTP codes to ignore and not forward to output"));
    println!("{}", Style::new().paint("-as    --acceptstring    Search content for string and forward Url if found"));
    println!("{}", Style::new().paint("-is    --ignorestring    Search content for string and ignore Url if found"));
    println!("{}", Style::new().paint("-to    --timeout         Timeout in seconds to wait for a request  [Default: 3, Range:1-180]"));
    println!("{}", Style::new().paint("-t     --threads         Threads to use [Default: 5, Range:1-100]"));
    println!("{}", Style::new().paint("-o     --output          Output results to a file"));

    println!("{}", Style::new().bold().paint("\nExample"));
    println!("{}", Style::new().paint("ruzzer --url=\"http://example.com/*\" --wordlist=\"wordlist.txt\" --acceptcodes=\"200,210,403\" --output=\"results.txt\" --threads=5"));

    println!("{}", Style::new().bold().paint("\nDisclaimer"));
    println!("{}", Style::new().paint("Ruzzer is provided as is and by using it you agree to take responsibility for your actions while using it."));
    std::process::exit(1);
}

pub fn error(error_message: String, show_help: bool)
{
    println!("{}", Style::new().fg(Color::Red).bold().paint(error_message));
    if show_help
    {
        println!("\r");
        help();
    }
    else
    {
        std::process::exit(0);
    }
}

pub fn errors(error_messages: Vec<String>, show_help: bool)
{
    for error_message in error_messages
    {
        println!("{}\r", Style::new().fg(Color::Red).bold().paint(error_message));
    }
    
    if show_help
    {
        println!("\r");
        help();
    }
    else
    {
        std::process::exit(0);
    }
}

pub fn warning(message: String)
{
    println!("{}", Style::new().fg(Color::Yellow).paint(message));
}

pub fn ok_result(status_code: i32, url: &str)
{
    println!("[{}] {}", Style::new().fg(Color::Green).bold().paint(status_code.to_string()), Style::new().bold().paint(url));
}

pub fn scan_init(fuzz_params: &FuzzParams)
{
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();
    let time = format!("{:02}:{:02}:{:02} {}\n", hour, now.minute(), now.second(), if is_pm { "PM" } else { "AM" });

    println!("{} {}", Style::new().fg(Color::Blue).bold().paint("Rust Fuzzer v1.0"), time);
    
    if fuzz_params.fuzz_type == FuzzType::AcceptCodes
    {
        println!("{} {}", 
            Style::new().fg(Color::Blue).bold().paint("Fuzz Type: "), 
            Style::new().fg(Color::Green).bold().paint(format!("{:?} {:?}\n", fuzz_params.fuzz_type, fuzz_params.http_codes))
        );
    }
    if fuzz_params.fuzz_type == FuzzType::IgnoreCodes
    {
        println!("{} {}", 
            Style::new().fg(Color::Blue).bold().paint("Fuzz Type: "), 
            Style::new().fg(Color::Red).bold().paint(format!("{:?} {:?}\n", fuzz_params.fuzz_type, fuzz_params.http_codes))
        );
    }
    else if fuzz_params.fuzz_type == FuzzType::AcceptString
    {
        println!("{} {}", 
            Style::new().fg(Color::Blue).bold().paint("Fuzz Type: "), 
            Style::new().fg(Color::Green).bold().paint(format!("{:?} \"{}\"\n", fuzz_params.fuzz_type, fuzz_params.search_string))
        );
    }   
    else if fuzz_params.fuzz_type == FuzzType::IgnoreString
    {
        println!("{} {}", 
            Style::new().fg(Color::Blue).bold().paint("Fuzz Type: "), 
            Style::new().fg(Color::Red).bold().paint(format!("{:?} \"{}\"\n", fuzz_params.fuzz_type, fuzz_params.search_string))
        );
    }   
}