use reqwest::{Client, Response, redirect};
use futures::{stream, StreamExt}; 
use super::output;
use super::{FuzzParams, FuzzType};
use super::results;
use std::time::Duration;

struct FuzzResponse
{
    pub url: String,
    pub status_code: i32,
    pub text: String,
    pub error: bool
}

pub fn begin(fuzz_params: &FuzzParams) {

    let custom = redirect::Policy::custom(|attempt| {
        attempt.stop()
    });

    let client = Client::builder().redirect(custom).timeout(Duration::from_secs(fuzz_params.timeout)).build();
    match client
    {
        Ok(client) => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let future = send_requests(&fuzz_params, client);
            rt.block_on(future);
        }
        Err(_e) => {}
    }
}

async fn send_requests(fuzz_params: &FuzzParams, client: Client)
{
    let urls = generate_urls(&fuzz_params);

    let urls_count: usize = urls.len();
    let mut responses_count: i32 = 0;

    let string_search: bool = fuzz_params.fuzz_type == FuzzType::AcceptString || fuzz_params.fuzz_type == FuzzType::IgnoreString;

    let responses = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {

                let mut fuzz_response = FuzzResponse
                {
                    url: url.to_owned(),
                    status_code:0,
                    text:format!(""),
                    error:false
                };

                if string_search
                {
                    let resp = client.head(url.as_str()).send().await;                    
                    match resp
                    {
                        Ok(resp) => {
    
                            fuzz_response.status_code = get_status_code(&resp);
                        },
                        Err(e) => {
                            output::warning(format!("{}", e));
                            fuzz_response.error = true;
                        }
                    }
                }
                else
                {
                    let resp = client.get(url.as_str()).send().await;

                    match resp
                    {
                        Ok(resp) => {
    
                            fuzz_response.status_code = get_status_code(&resp);
    
                            if string_search
                            {
                                let content_response = get_content(resp).await;
                                fuzz_response.text = content_response.0;
                                fuzz_response.error = content_response.1;
                            }
                        },
                        Err(e) => {
                            output::warning(format!("{}", e));
                            fuzz_response.error = true;
                        }
                    }
                }
                fuzz_response
            }
        })
        .buffer_unordered(fuzz_params.threads);

    responses.for_each(|r| {

        responses_count += 1;
        output::progress_update(responses_count, urls_count);

        async move{
            if !r.error
            {
                let re = r;
                let url: &str = re.url.as_str();

                if fuzz_params.fuzz_type == FuzzType::AcceptString && re.text.contains(&fuzz_params.search_string) 
                {
                    output::ok_result(re.status_code, url);
                    results::add(re.status_code, url);
                }
                else if fuzz_params.fuzz_type == FuzzType::IgnoreString && !re.text.contains(&fuzz_params.search_string) 
                {
                    output::ok_result(re.status_code, url);
                    results::add(re.status_code, url);
                }
                else if fuzz_params.fuzz_type == FuzzType::AcceptCodes && fuzz_params.http_codes.contains(&re.status_code)
                {
                    output::ok_result(re.status_code, url);
                    results::add(re.status_code, url);
                }
                else if fuzz_params.fuzz_type == FuzzType::IgnoreCodes && !fuzz_params.http_codes.contains(&re.status_code)
                {
                    output::ok_result(re.status_code, url);
                    results::add(re.status_code, url);
                }                
            }
            else
            {
                output::info("Consider reducing threads!".to_owned())
            }
        }
    })
    .await;
}


async fn get_content(resp: Response) -> (String, bool)
{
    let txt = resp.text().await;

    match txt
    {
        Ok(txt) => {
            (txt.to_owned(), false)
        },
        Err(e) => {
            output::warning(format!("{}", e));
            (String::new(), true)
        }
    }
}

fn get_status_code(resp: &Response) -> i32
{
    let scs = &resp.status().to_string()[..];
    let n = scs.split_whitespace().next().unwrap().parse::<i32>();

    let mut code = -1;
    match n
    {
        Ok(n) => {
            code = n;
        },
        Err(n) => output::warning(format!("Error Parsing Status code: {:?}", n))
    }
    code
}

fn generate_urls(fuzz_params: &FuzzParams) -> Vec<String>
{
    let mut urls: Vec<String> = vec![];

    for word in &fuzz_params.words
    {
        let url: String = format!("{}{}{}", fuzz_params.url_parts[0], word, fuzz_params.url_parts[1]);

        for ext in &fuzz_params.extensions
        {
            urls.push(format!("{}.{}", url, ext))
        }
        urls.push(url);
    }

    urls
}