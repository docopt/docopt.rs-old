use cmp::Eq;

use std::map::Map;
use send_map::linear::LinearMap;
use std::json;

use pcre::search;
use pcre::MatchExtensions;
use pcre::consts::PCRE_EXTENDED;

use std::json::ToJson;


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


pub struct Option {
    mut short: ~str,
    mut long: ~str,
    mut argcount: int,
    mut value: ~str
}


/// Parse token and return option object
pub fn Option(short: ~str, long: ~str, argcount: int, value: ~str) -> Option {
    Option {
        short: short,
        long: long,
        argcount: argcount,
        value: value
    }
}


pub fn get_option() -> Option {
    Option {
        short: ~"",
        long: ~"",
        argcount: 0,
        value: ~""
    }
}


impl Option {

    /// Parse token and return option object
    fn parse(token: &str) {
        let mut options = token;
        let options = str::replace(options, &",", &"");
        let splitted = str::split_str(options, ~" ");

        for splitted.each() |part| {
            if str::starts_with(*part, ~"--") {
                self.long = *part;
            }
            else if str::starts_with(*part, ~"-") {
                self.short = *part;
            }
            else {
                self.argcount = 1;
            }
        }

        // TODO: parse default value '\[default: (.*)\]'

    }
    
}

impl Option: Eq {
    #[inline(always)]
    pure fn eq(other: &Option) -> bool {
        ((self.short == other.short) && (self.long == other.long) && (self.argcount == other.argcount) && (self.value == other.value))
    }
    #[inline(always)]
    pure fn ne(other: &Option) -> bool { !self.eq(other) }
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


#[cfg(test)]
mod tests {

    fn check_option(token: ~str, option_args: (~str, ~str, int, ~str)) {
        let option = get_option();
        option.parse(token);
        let (short, long, argcount, value) = option_args;
        assert option == Option(short, long, argcount, value);
        io::println(fmt!("%?", option));
    }

    #[test]
    fn test_option() {
        check_option(~"-h", (~"-h", ~"", 0, ~""));
        check_option(~"--help", (~"", ~"--help", 0, ~""));
        check_option(~"-h --help", (~"-h", ~"--help", 0, ~""));
        check_option(~"-h, --help", (~"-h", ~"--help", 0, ~""));

        check_option(~"-h TOPIC", (~"-h", ~"", 1, ~""));
    }

    #[test]
    fn test_docopt_ext_ok() {
        let result = docopt_ext(~"Usage: my_program", ~[]);
        assert result.is_ok();
    }

    // #[test]
    // fn test_docopt_ext_err() {
    //     let result = docopt::docopt_ext(~"Usage: my_program", ~[~"-h"]);
    //     assert result.is_err();
    // }

}
