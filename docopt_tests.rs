extern mod std;
extern mod docopt;
use std;
use docopt::*;


fn main () {

}


#[cfg(test)]
mod tests {

    #[test]
    fn test_split() {
        let splitted = pcre_split(~"hello abc world", ~"(abc)");
        assert splitted.is_ok();
        let expected = &const [~"hello ", ~"abc", ~" world"];
        let result = splitted.get();
        assert result[0] == expected[0];
        assert result[1] == expected[1];
        assert result[2] == expected[2];
    }

    #[test]
    fn test_dummy() {
        assert 1 == 1;
    }

}