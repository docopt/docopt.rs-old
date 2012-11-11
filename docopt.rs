use std::map::Map;
use send_map::linear::LinearMap;
use std::json;

use pcre::search;
use pcre::MatchExtensions;
use pcre::consts::PCRE_EXTENDED;

use std::json::ToJson;


/// Print usage
pub fn printable_usage(doc: ~str) -> ~str {
    let options_result: pcre::SearchResult = search(
        ~"([uU][sS][aA][gG][eE]:[ \t]+)", doc, PCRE_EXTENDED);
    let m = options_result.get();
    let word_usage: ~str = m.matched();
    let usage: ~str = m.postmatch();

    fmt!("%s%s", word_usage, usage)
}


//Toplevel public function for parsing. Args are taken from os::args()
pub fn docopt(doc: ~str) -> Result<~LinearMap<~str, json::Json>, ~str> {

    let argv = os::args();
    docopt_ext(doc, argv)
}

/// Toplevel public function for parsing doc. Arguments passed explicitly
pub fn docopt_ext(doc: ~str, argv: ~[~str]) -> Result<~LinearMap<~str, json::Json>, ~str> {

    let mut options = ~LinearMap();

    /* TODO: insert data to map here */
    options.insert(~"Arguments", argv.to_json());

    if doc == ~"trigger_error" {
        Err(str::append(~"Error: ", doc))
    }
    else {
        Ok(options)
    }
}
