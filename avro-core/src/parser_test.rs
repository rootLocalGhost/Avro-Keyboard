#[cfg(test)]
mod tests {
    use crate::parser::EnglishToBangla;

    #[test]
    fn test_basic_transliteration() {
        let mut parser = EnglishToBangla::new();
        parser.auto_correct = false;

        assert_eq!(parser.convert("amra").unwrap(), "আম্রা");
        assert_eq!(parser.convert("bangla").unwrap(), "বাংলা");
        assert_eq!(parser.convert("kotha").unwrap(), "কথা");
        assert_eq!(parser.convert("shikkha").unwrap(), "শিক্ষা");
    }

    #[test]
    fn test_vowels_and_kars() {
        let mut parser = EnglishToBangla::new();
        parser.auto_correct = false;

        assert_eq!(parser.convert("a").unwrap(), "আ");
        assert_eq!(parser.convert("i").unwrap(), "ই");
        assert_eq!(parser.convert("u").unwrap(), "উ");
        assert_eq!(parser.convert("e").unwrap(), "এ");
        assert_eq!(parser.convert("o").unwrap(), "অ");
        assert_eq!(parser.convert("O").unwrap(), "ও");

        assert_eq!(parser.convert("ki").unwrap(), "কি");
        assert_eq!(parser.convert("ku").unwrap(), "কু");
        assert_eq!(parser.convert("ke").unwrap(), "কে");
        assert_eq!(parser.convert("ko").unwrap(), "ক");
        assert_eq!(parser.convert("kO").unwrap(), "কো");
    }

    #[test]
    fn test_conjuncts() {
        let mut parser = EnglishToBangla::new();
        parser.auto_correct = false;

        assert_eq!(parser.convert("kkh").unwrap(), "ক্ষ");
        assert_eq!(parser.convert("gg").unwrap(), "জ্ঞ");
        assert_eq!(parser.convert("ng").unwrap(), "ং");
    }
}
