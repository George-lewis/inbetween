#[cfg(test)]
mod tests {
    use inbetween::between;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test() {
        assert_eq!(between!(0 < 1 < 2), true);
        assert_eq!(between!(0 < 5 < 10), true);
        assert_eq!(between!(0 < 5 > 0), true);

        let c = 10;
        assert_eq!(between!(0 < c > 3), true);
        
        assert_eq!(between!(0 < 0 < 1), false);
        assert_eq!(between!(0 > 1 > 0), false);

        let c = 10;
        assert_eq!(between!(0 > c > 0), false);

        assert_eq!(between!(10 <= c < 11), true);
        assert_eq!(between!(10 <= c < 10), true);
        assert_eq!(between!(12 <= c == 11), true);
    }
}
