<div width="500px">
  <div align="center">
    <img src="images/Logo.png" width="300px" height="240px" align="center"/>  
    <p><b>Ruzzer is a multipurpose multi-threaded website fuzzer made with Rust</b></p>
    <p><b>Fuzz for webpages, directories, specific http codes, strings in webpages, get params and more!</b></p>
  </div>

  <br/>

  <div align="center">
    <img src="images/Divider.png" width="600px" height="5px"/>
  </div>

  <br/>

  <div>
    <h3>🌟 Features</h3>
    <p>🔹 Target a specific position within an URI using an asterisk (*)</p>
    <p>🔹 Only output responses with or without specific HTTP codes</p>
    <p>🔹 Only output responses from webpages with or without target strings in the content</p>
    <p>🔹 Fast, Multi-Threaded Fuzzes</p>
    <p>🔹 Output results to file</p>   
  </div>
  
  
  <br/>

  <div align="center">
    <img src="images/Divider.png" width="600px" height="5px"/>
  </div>

  <br/>
  
  <div>
    <h3>🔍 Fuzz Arguments</h3>
<pre>
<b>-h     --help</b>            Show help
<b>-u     --url</b>             Url with an asterisk (*) marking the fuzz position
<b>-w     --wordlist</b>        Line seprated wordlist to fuzz target
<b>-ac    --acceptcodes</b>     HTTP codes to accept and forward to output
<b>-ic    --ignorecodes</b>     HTTP codes to ignore and not forward to output
<b>-as    --acceptstring</b>    Search content for string and forward Url if found
<b>-is    --ignorestring</b>    Search content for string and ignore Url if found
<b>-to    --timeout</b>         Timeout in seconds to wait for a request  [Default: 3, Range:1-180]
<b>-t     --threads</b>         Threads to use [Default: 5, Range:1-100]
<b>-o     --output</b>          Output results to a file
<b>-e     --extensions</b>      File Extensions (Requires fuzz position marker (*) at the end of the URL)
</pre>
  </div>

  <br/>

  <div align="center">
    <img src="images/Divider.png" width="600px" height="5px"/>
  </div>

  <br/>
  
  <div>
    <h3>👉 Fuzz for Strings</h3>
    <p><code><b>ruzzer --url=</b>"http://<span></span>example.com/<b>*.ext</b>" <b>--wordlist=</b>"wordlist.txt" <b>--acceptstring=</b>"Admin"</code></p>
    <p><code><b>ruzzer --url=</b>"http://<span></span>example.com/api/user/<b>*</b>" <b>--wordlist=</b>"wordlist.txt" <b>--ignorestring=</b>"Endpoint not found"</code></p>
    <div align="center">
      <img src="images/Admin-Cropped.png" width="400px" height="300px"/>
      <img src="images/Endpoint-Cropped.png" width="400px" height="300px"/>  
    </div>
  </div>
  
  <br/>

  <div align="center">
    <img src="images/Divider.png" width="600px" height="5px"/>
  </div>

  <br/>
  
  <div>
    <h3>👉 Fuzz for Directories & Files</h3>
    <p><code><b>ruzzer --url=</b>"http://<span></span>example.com/<b>*</b>" <b>--wordlist=</b>"wordlist.txt" <b>--acceptcodes=</b>"200,403"</code></p>
    <p><code><b>ruzzer --url=</b>"http://<span></span>example.com/<b>*</b>.ext" <b>--wordlist=</b>"wordlist.txt" <b>--ignorecodes=</b>"404"</code></p>
    <div align="center">
      <img src="images/Dir-Search-Cropped.png" width="400px" height="300px"/>
      <img src="images/File-Search-Cropped.png" width="400px" height="300px"/>  
    </div>
  </div>
  
  <br/>

  <div align="center">
    <img src="images/Divider.png" width="600px" height="5px"/>
  </div>

  <br/>
  
  <div>
    <h3>👉 Fuzz Get Params & Values for Strings or HTTP Codes</h3>
    <p><code><b>ruzzer --url=</b>"http://<span></span>example.com/?param=<b>*</b>" <b>--wordlist=</b>"wordlist.txt" <b>--acceptstring=</b>"SQL ERROR"</code></p>
    <p><code><b>ruzzer --url=</b>"http://<span></span>example.com/?<b>*</b>=value" <b>--wordlist=</b>"wordlist.txt" <b>--ignorecodes=</b>"200"</code></p>
    <div align="center">
      <img src="images/MySQL-Error-Cropped.png" width="400px" height="300px"/>
      <img src="images/Get-Param-Cropped.png" width="400px" height="300px"/>  
    </div>
  </div>
  
  <br/>

  <div align="center">
    <img src="images/Divider.png" width="600px" height="5px"/>
  </div>

  <br/>
  
  <div>
    <h3>⬇ Installation</h3>
    <div>
      <p><b>Download Pre-Built Latest Release</b></p>
      <p>🔹 wget https://github.com/akaBase/Ruzzer/raw/master/ruzzer</p>
      <p>🔹 chmod +x ruzzer</p>
      <p>🔹 Use Ruzzer!</p>
    </div>
    <h3>Or</h3>
    <div>
      <p><b>Build from Source</b></p>
      <p>🔹 Install Rust if it isn't already (Required to build to Ruzzer): https://www.rust-lang.org/tools/install</p>
      <p>🔹 git clone https://github.com/akaBase/Ruzzer.git</p>
      <p>🔹 cd Ruzzer/ruzzer-project</p>
      <p>🔹 cargo build --release</p>
      <p>🔹 build location: target/release/ruzzer</p>
      <p>🔹 Use Ruzzer!</p>
    </div>
  </div>
  
  
  
  <br/>

  <div align="center">
    <img src="images/Divider.png" width="600px" height="5px"/>
  </div>

  <br/>
  
  <div>
    <h3>👌 Disclaimer</h3>
    <p><b>Ruzzer is provided as is and by using it you agree to take responsibility for your actions while using it.</b></p>

  </div>
  
</div>
