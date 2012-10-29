extern mod std;
extern mod docopt;

use send_map::linear::LinearMap;
use io::{ReaderUtil, WriterUtil};


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

    let result: Result<~LinearMap<~str, ~str>, ()> = do task::try {
        let options = docopt::docopt(copy doc);
        move options
    };

    io::println(fmt!("%?", result));

    /* TODO: need to serialize options to JSON and write
       to sdout */

}