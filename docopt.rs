#[link(name = "docopt",
       vers = "0.1",
       author = "Kirill Panshin")];
#[crate_type = "lib"];

extern mod std;
extern mod extra;
use std::cmp::Eq;
use std::hashmap::HashMap;
use std::str;
use extra::json;

use extra::json::ToJson;


//Toplevel public function for parsing. Args are taken from os::args()
pub fn docopt(doc: ~str) -> Result<HashMap<~str, json::Json>, ~str> {

    let argv = std::os::args();
    docopt_ext(doc.clone(), argv.clone())
}

/// Toplevel public function for parsing doc. Arguments passed explicitly
pub fn docopt_ext(doc: ~str, argv: ~[~str]) -> Result<HashMap<~str, json::Json>, ~str> {

    let mut options = HashMap::new();

    /* TODO: insert data to map here */
    options.insert(~"Arguments", argv.to_json());

    if doc == ~"trigger_error" {
        Err(format!("Error: {}", doc))
    }
    else {
        Ok(options)
    }
}


pub struct Option {
    short: ~str,
    long: ~str,
    argcount: int,
    value: ~str
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
    fn parse(&mut self, option_description: &str) {
        let splitted = option_description.trim().split_str_iter("  ").to_owned_vec();
        let (options, description) = match splitted.len() {
            1 => (splitted[0].to_owned(), ~""),
            2 => (splitted[0].to_owned(), splitted[1].to_owned()),
            _ => {fail!("Error: double space must appear only once");
                 }// Handle this situation more gracefully
        };

        let options = str::replace(options, ",", " ");
        let options = str::replace(options, "=", " ");
        let splitted = options.split_iter(' ').to_owned_vec();

        for part in splitted.iter() {
            if *part == "" {
                // pass
            } else if part.starts_with("--") {
                self.long = part.to_owned();
            }
            else if part.starts_with("-") {
                self.short = part.to_owned();
            }
            else {
                self.argcount = 1;
            }
        }

        if self.argcount > 0 {
            let splitted_desc = description.split_str_iter("[default: ").to_owned_vec();
            self.value = match splitted_desc.len() {
                1 => {~""},
                2 => {splitted_desc[1].split_iter(']').nth(0).unwrap().to_owned()},
                _ => {fail!("Error: [default: VALUE] must \
                             appear only once");
                     } // May be handle this more gracefully
            };
        }
        // TODO: parse default value '\[default: (.*)\]'

    }

}


impl Eq for Option {
    #[inline(always)]
    fn eq(&self, other: &Option) -> bool {
        ((self.short == other.short) && (self.long == other.long) && (self.argcount == other.argcount) && (self.value == other.value))
    }
    #[inline(always)]
    fn ne(&self, other: &Option) -> bool { !self.eq(other) }
}


/// Print usage
pub fn printable_usage(doc: ~str) -> ~str {

    let splitted = doc.split_str_iter("Usage:").to_owned_vec();
    let (word_usage, usage) =match splitted.len() {
        1 => (splitted[0].to_owned(), ~""),
        2 => (splitted[0].to_owned(), splitted[1].to_owned()),
        _ => {fail!("Error in description: ``Usage:`` \
                     must appear only once");
              // Handle more gracefully
             }
    };

    format!("{}{}", word_usage, usage)
}


#[cfg(test)]
mod tests {

    fn check_option(token: ~str, option_args: (~str, ~str, int, ~str)) {
        let mut option = super::get_option();
        option.parse(token);
        let (short, long, argcount, value) = option_args;
        assert!(option == super::Option(short.clone(), long.clone(),
                                        argcount.clone(), value.clone()),
                "Parsing: {}", token);
    }

    #[test]
    fn test_option() {
        check_option(~"-h", (~"-h", ~"", 0, ~""));
        check_option(~"--help", (~"", ~"--help", 0, ~""));
        check_option(~"-h --help", (~"-h", ~"--help", 0, ~""));
        check_option(~"-h, --help", (~"-h", ~"--help", 0, ~""));

        check_option(~"-h TOPIC", (~"-h", ~"", 1, ~""));
        check_option(~"--help TOPIC", (~"", ~"--help", 1, ~""));
        check_option(~"-h TOPIC --help TOPIC", (~"-h", ~"--help", 1, ~""));
        check_option(~"-h TOPIC, --help TOPIC", (~"-h", ~"--help", 1, ~""));
        check_option(~"-h TOPIC, --help=TOPIC", (~"-h", ~"--help", 1, ~""));

        check_option(~"-h  Description...", (~"-h", ~"", 0, ~""));
        check_option(~"-h --help  Description...", (~"-h", ~"--help", 0, ~""));
        check_option(~"-h TOPIC  Description...", (~"-h", ~"", 1, ~""));

        check_option(~"    -h", (~"-h", ~"", 0, ~""));

        check_option(~"-h TOPIC  Descripton... [default: 2]",
                    (~"-h", ~"", 1, ~"2"));
        check_option(~"-h TOPIC  Descripton... [default: topic-1]",
                    (~"-h", ~"", 1, ~"topic-1"));
        check_option(~"--help=TOPIC  ... [default: 3.14]",
                    (~"", ~"--help", 1, ~"3.14"));
        check_option(~"-h, --help=DIR  ... [default: ./]",
                    (~"-h", ~"--help", 1, ~"./"));
    }

    #[test]
    fn test_docopt_ext_ok() {
        let result = super::docopt_ext(~"Usage: my_program", ~[]);
        assert!(result.is_ok());
    }

    // #[test]
    // fn test_docopt_ext_err() {
    //     let result = super::docopt::docopt_ext(~"Usage: my_program", ~[~"-h"]);
    //     assert result.is_err();
    // }

}
