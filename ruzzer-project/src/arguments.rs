const HELPARG: [&str; 2] = ["-h", "--help"];
const URLARG: [&str; 2] = ["-u=", "--url="];
const WORDLISTARG: [&str; 2] = ["-w=", "--wordlist="];
const IGNORECODESARG: [&str; 2] = ["-ic=", "--ignorecodes="];
const ACCEPTCODESARG: [&str; 2] = ["-ac=", "--acceptcodes="];
const IGNORESTRINGARG: [&str; 2] = ["-is=", "--ignorestring="];
const ACCEPTSTRINGARG: [&str; 2] = ["-as=", "--acceptstring="];
const THREADSARG: [&str; 2] = ["-t=", "--threads"];
const OUTPUTARG: [&str; 2] = ["-o=", "--output"];
const TIMEOUTARG: [&str; 2] = ["-to=", "--timeout="];

#[derive(Debug, PartialEq)]
pub enum FuzzArgs
{
    HELP,
    URL,
    WORDLIST,
    IGNORECODES,
    ACCEPTCODES,
    IGNORESTRING,
    ACCEPTSTRING,
    THREADS,
    OUTPUT,
    TIMEOUT
}



pub struct CLIarg
{
    pub value: String,
    pub isset: bool
}

pub fn get(arg: FuzzArgs) -> CLIarg
{
    match arg
    {
        FuzzArgs::HELP => get_arg_value(HELPARG),
        FuzzArgs::URL => get_arg_value(URLARG),
        FuzzArgs::WORDLIST => get_arg_value(WORDLISTARG),
        FuzzArgs::IGNORECODES => get_arg_value(IGNORECODESARG),
        FuzzArgs::ACCEPTCODES => get_arg_value(ACCEPTCODESARG),
        FuzzArgs::IGNORESTRING => get_arg_value(IGNORESTRINGARG),
        FuzzArgs::ACCEPTSTRING => get_arg_value(ACCEPTSTRINGARG),
        FuzzArgs::THREADS => get_arg_value(THREADSARG),
        FuzzArgs::OUTPUT => get_arg_value(OUTPUTARG),
        FuzzArgs::TIMEOUT => get_arg_value(TIMEOUTARG),
    }
}

fn get_arg_value(param_keys: [&str; 2]) -> CLIarg
{
    let cli_args: Vec<String> = std::env::args().collect();

    let mut arg_value: String = String::new();
    let mut found_arg = false;

    let mut index: usize = 0;

    while !found_arg &&  index < cli_args.len()
    {
        for &param_key in &param_keys
        {
            let key: String = param_key.to_owned();

            if cli_args[index].len() >= key.len() && cli_args[index][0..key.len()] == key
            {
                arg_value = cli_args[index].replace(&key, "");
                found_arg = true;
                break;
            }
        }
        index += 1;
    }

    CLIarg{value:arg_value,isset:found_arg}
}

