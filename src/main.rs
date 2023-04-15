fn main() {
    pretty_print("hello"); 
}


pub fn pretty_print(s: &str) {
    print!(r"oh my god {}", s);
}

#[cfg(test)]
mod test {
    use crate::pretty_print;

    #[test]
    fn test_pretty_print() {
        assert_eq!((), pretty_print("hello"));
    }
}