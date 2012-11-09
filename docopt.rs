use std::map::Map;
use send_map::linear::LinearMap;

use pcre::search;
use pcre::MatchExtensions;
use pcre::consts::PCRE_EXTENDED;


pub fn pcre_split(doc: ~str, split_pattern: ~str) -> Result<[~str * 3], pcre::RegexErr> {
    let search_result: pcre::SearchResult = search(
        split_pattern, doc, PCRE_EXTENDED);
    match search_result {
        Ok(result) => Ok([result.prematch(),
                           result.matched(), result.postmatch()]),
        Err(error_message) => Err(error_message)
    }
}


/// Print usage
pub fn printable_usage(doc: ~str) -> ~str {
    let options_result: pcre::SearchResult = search(
        ~"([uU][sS][aA][gG][eE]:[ \t]+)", doc, PCRE_EXTENDED);
    let m = options_result.get();
    let word_usage: ~str = m.matched();
    let usage: ~str = m.postmatch();

    fmt!("%s%s", word_usage, usage)
}


/// Toplevel public function for parsing doc. Returns ``Result``
pub fn docopt(doc: ~str) -> Result<~LinearMap<~str, ~str>, ~str> {
    let mut args = ~LinearMap();

    /* TODO: insert data to map here */

    if doc == ~"trigger_error" {
        Err(fmt!("Error: %s", doc))
    }
    else {
        Ok(move args)
    }
}
