use std::fs::File;
use std::io::{self, BufRead};

pub fn get_words(wordlist: String) ->  Result<Vec<String>, String>
{
    let mut words: Vec<String> = vec![];
    let mut error_msg = String::new();

    let file = File::open(wordlist);

    match file
    {
        Ok(file) =>
        {
            let lines = io::BufReader::new(file).lines();

            for line in lines
            {
                match line
                {
                    Ok(line) => 
                    {
                        let l = line.trim();

                        if !l.is_empty()
                        {
                            words.push(l.to_string());
                        }
                    },
                    Err(e) => { error_msg = e.to_string(); }
                }
            }
        },
        Err(e) => { error_msg = e.to_string(); }
    }

    if error_msg.is_empty()
    {
        Ok(words)
    }
    else
    {
        Err(error_msg)
    }
}