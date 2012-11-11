extern mod std;
extern mod docopt;
use std;
use docopt;


fn main () {

}


#[cfg(test)]
mod tests {

    #[test]
    fn test_docopt_ext_ok() {
        let result = docopt::docopt_ext(~"Usage: my_program", ~[]);
        io::println(fmt!("%?", result));
    }

    #[test]
    fn test_docopt_ext_err() {
        let result = docopt::docopt_ext(~"Usage: my_program", ~[~"-h"]);
        assert result.is_err();
    }

}