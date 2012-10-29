use std::map::Map;
use send_map::linear::LinearMap;


/// parses doc and outputs map with args
pub fn docopt(doc: ~str) -> ~LinearMap<~str, ~str>{
    let mut args = ~LinearMap();
    args.insert(copy doc, copy doc);

    /* TODO: insert data to map here */
    move args
}    

