mod defs;
mod search;
mod media;
mod target;
mod handler;

use std::env;
use defs::ARGS;
use search::Search;

fn main() {
    let mut srch = Search::from(env::args());
    //eprintln!("{}",srch);
    srch.search();
}
