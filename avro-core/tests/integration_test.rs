use avro_core::parser::EnglishToBangla;

#[test]
fn test_integration_transliterations() {
    let mut parser = EnglishToBangla::new();
    parser.auto_correct = false; // matching old test logic

    let cases = vec![
        ("amra", "আম্রা"),
        ("bangla", "বাংলা"),
        ("kotha", "কথা"),
        ("shikkha", "শিক্ষা"),
        // Basic Vowels
        ("a", "আ"),
        ("e", "এ"),
        ("i", "ই"),
        ("O", "ও"), // phonetic 'o' is aw, 'O' is 'o'
        ("u", "উ"),

        // Consonants
        ("k", "ক"),
        ("kh", "খ"),
        ("g", "গ"),
        ("gh", "ঘ"),
        ("c", "চ"),
        ("ch", "ছ"),
        ("j", "জ"),
        ("jh", "ঝ"),
        ("T", "ট"),
        ("Th", "ঠ"),
        ("D", "ড"),
        ("Dh", "ঢ"),
        ("t", "ত"),
        ("th", "থ"),
        ("d", "দ"),
        ("dh", "ধ"),
        ("n", "ন"),
        ("p", "প"),
        ("ph", "ফ"),
        ("b", "ব"),
        ("bh", "ভ"),
        ("m", "ম"),
        ("r", "র"),
        ("l", "ল"),
        ("s", "স"),
        ("sh", "শ"),
        ("h", "হ"),

        // Kar (Vowel signs)
        ("ka", "কা"),
        ("ki", "কি"),
        ("ku", "কু"),
        ("ke", "কে"),
        ("kO", "কো"),

        // Fala
        ("kya", "ক্যা"), // jofala + a

        // Conjuncts
        ("kkO", "ক্কো"),
        ("kTa", "ক্টা"),
        ("nda", "ন্দা"),

        // ZWNJ / ZWJ
        ("k`", "ক"),
    ];

    for (input, expected) in cases {
        let result = parser.convert(input).expect("Conversion failed");
        assert_eq!(result, expected, "Failed for input: {}", input);
    }
}
