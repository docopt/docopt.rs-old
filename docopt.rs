use std::map::Map;
use send_map::linear::LinearMap;


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
