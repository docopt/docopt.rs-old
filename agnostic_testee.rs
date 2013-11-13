extern mod std;
extern mod docopt;

use std::rt::io;
use std::str;

/// Reads whole stream and outputs it as string
fn read_whole_stream_to_str(input_stream: &mut io::Reader) -> ~str {
    let mut buf = ~[];
    do std::rt::io::io_error::cond.trap(|_| ()).inside {
        loop {
            match input_stream.read_byte() {
                Some(c) => buf.push(c as char),
                None    => break,
            }
        }
    }
    str::from_chars(buf)
}


fn main () {

    let mut input_stream = io::stdin();

    let doc: ~str = read_whole_stream_to_str(&mut input_stream as &mut io::Reader);

    let docopt_result = docopt::docopt(doc.clone());

    match docopt_result {
        Ok(options) => io::println(format!("{}", options.to_str())),
        Err(error_message) => io::println(error_message)

    }

    /* Testing various things */
    io::println(docopt::printable_usage(doc.clone()));
}
