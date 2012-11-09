extern mod std;
extern mod docopt;

use io::{ReaderUtil, WriterUtil};
use send_map::linear::LinearMap;
use std::json::LinearMap;


/// Reads whole stream and outputs it as string
fn read_whole_stream_to_str(input_stream: io::Reader) -> ~str {
    let mut buf = ~[];
    loop {
        let ch = input_stream.read_byte();
        if input_stream.eof() { break; }
        buf.push(ch as u8);
    }
    str::from_bytes(buf)
}


fn main () {

    let input_stream = io::stdin();

    let doc: ~str = read_whole_stream_to_str(input_stream);

    let docopt_result = docopt::docopt(copy doc);

    match docopt_result {
        Ok(options) => io::println(options.to_json().to_str()),
        Err(error_message) => io::println(error_message)

    }

    /* TODO: need to serialize options to JSON and write
       to sdout */

}