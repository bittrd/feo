fn main() {    
    format!("Hello, world! {}", add());
}

fn add() -> i64 {
    2+2
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(),4)
    }
    #[test]
    fn test_main() {
        main()
    }
}