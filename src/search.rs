///! This module contains `Search` which is a handler for all supported web media queries supported.
use std::{env::Args, fmt::Display, time::Duration, io::Read};
use ureq::AgentBuilder;
use select::document::Document;
use fastrand;
use crate::{
    ARGS,
    defs::{ YOUTUBE_SEARCH_URI, USER_AGENT_POOL, TIMEOUT, YOUTUBE_MOCK_FILE, RAW_SEPERATOR, RAW_SEPERATOR_ALT },
    media::Media,
    target::WebTargets,
};


#[derive(Default)]
pub struct Search {
    query: Vec<String>,
    results: Option<Vec<Media<String>>>,
    target: WebTargets,
    onlyfirst: bool,
    raw: bool,
    notitle: bool,
    verbosity: u8,
}

impl From<Args> for Search {
    fn from(args: Args) -> Self {
        // Decl
        let which = |arg: &str| -> Option<usize> {
            let mut which: (bool,usize) = (false, 1000);
                for (idx,rg) in ARGS.iter().enumerate() { if rg.contains(&arg) {
                        which = (true,idx);
                        break;
                    }
            } 
            if which.0 { Some(which.1) } else { None }
        };
        let mut _srch = Search::default();
        let mut qwait: (bool,Vec<String>) = (false, Vec::new());

        // Main
        for arg in args.skip(1) {
            if let Some(idx) = which(&arg) {
                match ARGS[idx] {
                    // arg = "-yt" | "--youtube"
                    x if x == ARGS[0] => {
                        _srch.target = WebTargets::YouTube;
                        qwait.0 = true;
                    },
                    // arg = "-v" | "--verbose"
                    x if x == ARGS[1] => _srch.verbosity = 1,
                    // arg = "-vv" | "--very-verbose"
                    x if x == ARGS[2] => _srch.verbosity = 2,
                    // arg = "-f" | "--first"
                    x if x == ARGS[3] => _srch.onlyfirst = true,
                    // arg = "-r" | "--raw"
                    x if x == ARGS[4] => _srch.raw = true,
                    // arg = "-not" | "--no-title"
                    x if x == ARGS[5] => _srch.notitle = true,
                    // arg = *
                    _ => {},
                }
            } else {
                        if qwait.0 { qwait.1.push(arg); }
                        else { qwait.0 = true; qwait.1.push(arg); }
            }
        }
        // After all args have been looped through put entire query into search
        _srch.query = qwait.1;
        // Return
        _srch
    }
}

impl Display for Search {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "┊╎{}╎ {} ┊  : {} ┊", self.target, self.query.join(" "), self.verbosity)
    }
}

impl Search {
    /// ## Search::search()
    /// Start a web search and display the results
    pub fn search(&mut self) {
        // self.mockload();
        self.webfetch();
        match &self.results {
            Some(res) => {
                if self.onlyfirst {
                    if self.raw {
                        if self.notitle { println!("{}", res[0].url()); }
                        else { println!("{}{RAW_SEPERATOR}{}", res[0].url(), res[0].title()); }
                    } else {
                        println!("{}",res[0].url());
                    }
                } else {
                    self.list_results();
                }
            },
            None => eprintln!("No results were parsed"),
        }
    }

    /// ## mockload() -::- `testing`
    /// A mock results feeder.
    fn mockload(&mut self) {
        let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        match std::fs::File::open(format!("{}/{}",root, YOUTUBE_MOCK_FILE )) {
            Ok(file) => {
                let doc = Document::from_read(file).expect("Failed to read from mock file");
                self.results = self.target.parsedoc::<String>(doc,self.onlyfirst);
            },
            Err(reason) => eprintln!("Failed to open: {}",reason), 
        }
    }
    /// ## webfetch() -::- `middleman`
    /// Fetch and structure the fetched data
    fn webfetch(&mut self) {
        match self.target {
            WebTargets::YouTube => {
                let enc_uri = format!("{}{}", YOUTUBE_SEARCH_URI, self.query.join("+"));
                match Search::request(enc_uri, &self.query[0]) {
                    Ok(resp) => self.results_from(resp),
                    Err(reason) => eprintln!("Request failed: {}",reason),
                }
            },
            _ => {}
        }
    }

    /// ## request() -::- `fetch`
    /// Make requests to the specified uri, with an additional hint required for spoofing the target.
    fn request(uri: String, hint: &str) -> Result<Vec<u8>, ureq::Error> {
        let mut _buf : Vec<u8> = Vec::new();
        let _agnt = AgentBuilder::new()
            .timeout(Duration::from_secs(TIMEOUT))
            .user_agent(USER_AGENT_POOL[fastrand::usize(..USER_AGENT_POOL.len())])
            .build()
            .get(&uri)
            .set("Accept","text/html,application/xhtml+xml,application/xml;q= 0.9,*/*;q=0.8")
            .set("Accept-Language","en-us")
            .set("Cache-Control", "no-cache")
            .set("Connection", "keep-alive")
            .set("Referer", &format!("https://www.google.com/search?q={}",hint))
            .set("Sec-CH-UA", r#""Chromium";v="93", " Not A;Brand";v="99""#)
            .set("Sec-CH-UA-Mobile", "?0")
            .set("Sec-Fetch-Mode", "navigate")
            .set("Sec-Fetch-Dest", "document")
            .set("Sec-Fetch-Site", "same-origin")
            .set("Sec-Fetch-User", "?1")
            .set("Upgrade-Insecure-Requests", "1")
            .call()?
            .into_reader()
            .read_to_end(&mut _buf);

        match _agnt {
            Ok(_) => Ok(_buf),
            Err(_) => {
                eprintln!("Failed to read to buffer");
                Ok(_buf)
            },
        }
    }

    /// ## results_from() -::- `helper`
    /// Convert a stream of bytes(response) into results.
    fn results_from(&mut self, response: Vec<u8>) {
        let document = Document::from_read(response.as_slice()).expect("Failed to parse html document");
        self.results = self.target.parsedoc::<String>(document, self.onlyfirst);
    }

    /// ## Search::list_results() -::- `display`
    /// List results prettily.
    pub fn list_results(&self) {
        match &self.results {
            Some(results) => {
                for entry in results {
                    if self.raw {
                        if self.notitle { println!("{}", entry.url()); }
                        else { print!("{}{RAW_SEPERATOR}{}{RAW_SEPERATOR_ALT}", entry.title(), entry.url()); }
                    } else {
                        if self.notitle { println!(" {}", entry.url()); }
                        else { println!("{entry}"); }
                    }
                }
            },
            None => eprintln!("No results"),
        }
    }
}
