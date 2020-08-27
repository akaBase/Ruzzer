# Ruzzer
#### Ruzzer a multi-threaded website fuzzer built with Rust.


***
#### Arguments
<pre>
-u     --url             Url with an asterisk (*) marking the fuzz position
-w     --wordlist        Line seprated wordlist to fuzz target
-ac    --acceptcodes     HTTP codes to accept and forward to output
-ic    --ignorecodes     HTTP codes to ignore and not forward to output
-as    --acceptstring    Search content for string and forward Url if found
-is    --ignorestring    Search content for string and ignore Url if found
-to    --timeout         Timeout in seconds to wait for a request  [Default: 3, Range:1-180]
-t     --threads         Threads to use [Default: 5, Range:1-100]
-o     --output          Output results to a file
</pre>

***
#### Example
<pre>
ruzzer --url="http://example.com/*" --wordlist="wordlist.txt" --acceptcodes="200,403" --output="results.txt"
</pre>

***
#### Download Built ruzzer 
Download Link: https://github.com/akaBase/Ruzzer/raw/master/ruzzer
***
#### Build ruzzer from source
Install Rust if it isn't (Required to build but not run built ruzzer): https://www.rust-lang.org/tools/install


* git clone https://github.com/akaBase/Ruzzer.git
* cd Ruzzer/ruzzer-project
* cargo build --release

Release ruzzer location: target/release/ruzzer
***
#### Disclaimer
Ruzzer is provided as is and by using it you agree to take responsibility for your actions while using it.
***
