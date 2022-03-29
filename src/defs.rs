//==============//
//    Flags     //
//==============//
pub const ARGS: [ [&'static str; 2]; 6 ] =
    [ 
    YOUTUBE,
    VERBOSE,
    VERY_VERBOSE,
    FIRST,
    RAW,
    NO_TITLE
];

const YOUTUBE: [ &'static str; 2] = [ "-yt", "--youtube" ];
const VERBOSE: [ &'static str; 2] = [ "-v", "--verbose" ];
const VERY_VERBOSE: [ &'static str; 2] = [ "-vv", "--very-verbose" ];
const FIRST: [ &'static str; 2] = [ "-f", "--first" ];
const RAW: [ &'static str; 2] = [ "-r", "--raw" ];
const NO_TITLE: [ &'static str; 2] = [ "-not", "--no-title" ];

//==============//
//     ANSI     //
//==============//
pub const REVERSE: &'static str = "\x1b[7m";
pub const BOLD: &'static str = "\x1b[1m";
pub const RESET: &'static str = "\x1b[0m";

pub const RAW_SEPERATOR: &'static str = "┊";
pub const RAW_SEPERATOR_ALT: &'static str = "\n";
//====================//
//  Request-Options   //
//====================//
pub const TIMEOUT: u64 = 5;
//=================//
//   User-Agent    //
//=================//
pub const USER_AGENT_POOL : [&'static str; 7] = [
                                                  "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.3 Safari/605.1.15",
                                                  "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36",
                                                  "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36",
                                                  "Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36",
                                                  "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36",
                                                  "Mozilla/5.0 (Windows NT 10.0; Trident/7.0; rv:11.0) like Gecko",
                                                  "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36 Edg/92.0.902.78",
                                                ];
//==============//
//   YouTube    //
//==============//
pub const YOUTUBE_SEARCH_URI: &'static str = "https://www.youtube.com/results?search_query=";
pub const YOUTUBE_BASE_URI: &'static str = "https://www.youtube.com";
pub const YOUTUBE_FRAME: &'static str = "ytd-video-renderer";
pub const YOUTUBE_MOCK_FILE: &'static str = "prey/youtube.html";
// pub const YOUTUBE_MOCK_FILE: &'static str = "prey/ytxmonad.html";
pub const YT_STRIP_PREFIX: &'static str = "var ytInitialData";
pub const YT_STRIP_LEN: usize = YT_STRIP_PREFIX.len();
