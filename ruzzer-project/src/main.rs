mod arguments;
mod output;
mod wordlist;
mod requests;
mod results;

use std::path::Path;
use arguments::{get, FuzzArgs, CLIarg};
use http::Uri;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[derive(Debug, PartialEq)]
pub enum FuzzType
{
    NotSet,
    IgnoreCodes,
    AcceptCodes,
    IgnoreString,
    AcceptString
}

#[derive(Debug)]
pub struct FuzzParams
{
    pub url: String,
    pub url_parts: Vec<String>,

    pub words: Vec<String>,

    pub fuzz_type: FuzzType,

    pub http_codes: Vec<i32>,
    pub search_string: String,

    pub threads: usize,

    pub output: String,

    pub timeout:u64,

    pub extensions: Vec<String>
}

fn main() {

    let help: CLIarg = get(FuzzArgs::HELP);

    if help.isset
    {
        output::help();
    }

    let fuzz_params: FuzzParams = build_fuzz_params();

    output::scan_init(&fuzz_params);

    requests::begin(&fuzz_params);

    if !fuzz_params.output.is_empty()
    {
        results::write_results_to_file(fuzz_params);
    }
}


fn build_fuzz_params() -> FuzzParams
{
    let mut param_errors: Vec<String> = vec![];

    // Build empty FuzzParams struct
    let mut fuzz_params = FuzzParams
    {
        url: String::new(),
        url_parts: vec![],

        words: vec![],

        fuzz_type: FuzzType::NotSet,

        http_codes: vec![],
        search_string: String::new(),

        threads: 5,

        output: String::new(),

        timeout: 3,

        extensions: vec![]
    };

    // Get Url
    let url: CLIarg = get(FuzzArgs::URL);
    if url.isset
    {
        let uri: Uri = url.value.parse::<Uri>().unwrap();

        if uri.scheme() == None
        {
            param_errors.push("Url doesn't contain scheme!".to_owned());
        } 
        else if uri.host() == None
        {
            param_errors.push("Invalid Url parameter!".to_owned());
        }
        else
        {
            if url.value.contains('*')
            {
                let url_parts: Vec<&str> = url.value.split('*').collect();
                if url_parts.len() == 2
                {
                    for url_part in url_parts
                    {
                        fuzz_params.url_parts.push(url_part.to_owned());
                    }
                }
                else
                {
                    param_errors.push("Url must contain only one fuzz position asterisk (*)".to_owned())
                }
            }
            else
            {
                param_errors.push("Url must contain an asterisk (*) showing the fuzz position!".to_owned());
            }
        }
    }
    else
    {
        param_errors.push("Url parameter is required! [-u --url]".to_owned());
    }


    // Get Words from Wordlist
    let wordlist: CLIarg = get(FuzzArgs::WORDLIST);
    if wordlist.isset
    {
        match wordlist::get_words(wordlist.value)
        {
            Ok(words) => {
                fuzz_params.words = words;
                if fuzz_params.words.len() == 0
                {
                    param_errors.push("Wordlist is empty!".to_owned());
                }
            },
            Err(e) => param_errors.push(e)
        }
    }
    else
    {
        param_errors.push("Wordlist parameter is required! [-w --wordlist]".to_owned());
    }

    // Ignore Codes
    let ignore_codes: CLIarg = get(FuzzArgs::IGNORECODES);
    if ignore_codes.isset
    {
        fuzz_params.fuzz_type = FuzzType::IgnoreCodes;
        fuzz_params.http_codes = http_codes_from_string(ignore_codes.value);
        
        if fuzz_params.http_codes.len() == 0
        {
            param_errors.push("No ignore codes supplied!".to_owned());
        }
    }

    // Accept Codes
    let accept_codes: CLIarg = get(FuzzArgs::ACCEPTCODES);
    if accept_codes.isset
    {
        if fuzz_params.fuzz_type != FuzzType::NotSet
        {
            param_errors.push("Only one type of Fuzz can be performed at a time".to_owned());
        }

        fuzz_params.fuzz_type = FuzzType::AcceptCodes;
        fuzz_params.http_codes = http_codes_from_string(accept_codes.value);
        
        if fuzz_params.http_codes.len() == 0
        {
            param_errors.push("No accept codes supplied!".to_owned());
        }
    }

    // Ignore String
    let ignore_string: CLIarg = get(FuzzArgs::IGNORESTRING);
    if ignore_string.isset
    {
        fuzz_params.fuzz_type = FuzzType::IgnoreString;
        fuzz_params.search_string = ignore_string.value;
        
        if fuzz_params.search_string.is_empty()
        {
            param_errors.push("Search string cannot be empty!".to_owned());
        }
    }

    // Accept String
    let accept_string: CLIarg = get(FuzzArgs::ACCEPTSTRING);
    if accept_string.isset
    {
        fuzz_params.fuzz_type = FuzzType::AcceptString;
        fuzz_params.search_string = accept_string.value;
        
        if fuzz_params.search_string.is_empty()
        {
            param_errors.push("Search string cannot be empty!".to_owned());
        }
    }

    // Check Fuzz Type has been set
    if fuzz_params.fuzz_type == FuzzType::NotSet
    {
        param_errors.push("No fuzz type set!".to_owned())
    }

    // Threads
    let threads: CLIarg = get(FuzzArgs::THREADS);
    if threads.isset
    {
        let num = threads.value.parse::<usize>();
        match num
        {
            Ok(num) => {
                if num >= 1 && num <= 100
                {
                    fuzz_params.threads = num;
                }
            },
            Err(_) => param_errors.push("Invalid threads value".to_owned())
        }
    }

    // Timeout
    let timeout: CLIarg = get(FuzzArgs::TIMEOUT);
    if timeout.isset
    {
        let num = timeout.value.parse::<u64>();
        match num
        {
            Ok(num) => {
                if num >= 1 && num <= 180
                {
                    fuzz_params.timeout = num;
                }
            },
            Err(_) => param_errors.push("Invalid Timeout value".to_owned())
        }
    }

    // Output
    let fuzz_output: CLIarg = get(FuzzArgs::OUTPUT);
    if fuzz_output.isset
    {
        if !fuzz_output.value.is_empty()
        {
            let check: (i32, String) = output_file_check(fuzz_output.value);

            let error = check.0;
            let output_file = check.1;

            if error == 0
            {
                fuzz_params.output = output_file.to_owned();
            }
            else if error == 1
            {
                param_errors.push("Output file already exists".to_owned());
            }
            else if error == 2
            {
                param_errors.push("Invalid output file path".to_owned());
            }
        }
        else
        {
            let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
            fuzz_params.output = format!("web-fuzzer-results--{}.txt", rand_string).to_owned();
        }
    }


    // Extensions
    let extensions_string: CLIarg = get(FuzzArgs::EXTENSIONS);
    if extensions_string.isset 
    {
        if &fuzz_params.url_parts[1] == ""
        {
            fuzz_params.extensions = file_extensions_from_string(extensions_string.value);
            
            if fuzz_params.extensions.len() == 0
            {
                param_errors.push("Extensions is set but no valid entries".to_owned());
            }
        }
        else
        {
            param_errors.push("Unable to use file extensions if the fuzz position marker is not at the end of the URL".to_owned())
        }
    }


    if param_errors.len() > 0
    {
        output::errors(param_errors, true);
    } 

    fuzz_params
}


fn http_codes_from_string(arg_string:  String) -> Vec<i32>
{
    let mut codes: Vec<i32> = vec![];

    for cs in arg_string.split(',')
    {
        let n = cs.to_string().trim().parse::<i32>();
        match n 
        {
            Ok(n) =>
            {
                if n > 1 && n <= 600
                {
                    codes.push(n);
                }
            }, 
            Err(_) =>  output::warning(format!("Invalid HTTP Code: {}, dropping it", cs))
        }
    }

    codes
}


fn file_extensions_from_string(arg_string:  String) -> Vec<String>
{
    let mut extensions: Vec<String> = vec![];

    for cs in arg_string.split(',')
    {
        let ext = cs.to_owned();
        println!("{}", ext);
        extensions.push(cs.to_owned());
    }

    extensions
}

fn output_file_check(user_defined_path: String) -> (i32, String)
{
    let mut output_file = String::new();
    let mut error: i32 = 0;

    let path = Path::new(&user_defined_path);

    let filename = path.file_name();
    
    match filename
    {
        Some(fname) => {
            let basename: String = user_defined_path[0..(user_defined_path.len() - fname.len())].to_owned();
            if !basename.is_empty()
            {
                if !Path::new(&basename).exists()
                {
                    error = 2;
                }
                else if path.exists()
                {
                    error = 1;
                }
                else
                {
                    output_file = user_defined_path;
                }
            }
            else
            {
                if !Path::new(&user_defined_path).exists()
                {
                    output_file = user_defined_path;
                }
                else
                {
                    error = 1;
                }
            }
        },
        None => {}
    }

    (error, output_file)
}