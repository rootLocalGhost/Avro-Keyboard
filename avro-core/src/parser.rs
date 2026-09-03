use crate::bangla_chars::*;
use thiserror::Error;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Error, Debug)]
pub enum AvroError {
    #[error("Empty input string")]
    EmptyInput,
    #[error("Internal parser error: {0}")]
    InternalError(String),
}

pub struct EnglishToBangla {
    pub auto_correct: bool,
    determine_zwnj_zwj: char,
}

impl Default for EnglishToBangla {
    fn default() -> Self {
        Self::new()
    }
}

impl EnglishToBangla {
    pub fn new() -> Self {
        Self {
            auto_correct: true,
            determine_zwnj_zwj: ZWJ,
        }
    }

    pub fn convert(&self, english_t: &str) -> Result<String, AvroError> {
        if english_t.is_empty() {
            return Ok(String::new());
        }

        // We use unicode-segmentation to properly iterate over graphemes if needed,
        // although the core mapping is mostly 1:1 on chars.
        let graphemes: Vec<&str> = english_t.graphemes(true).collect();
        if graphemes.is_empty() {
            return Ok(String::new());
        }

        let eng_str = self.correct_case(english_t);

        let mut parser = ParserState::new(&eng_str, self.determine_zwnj_zwj);
        Ok(parser.my_convert())
    }

    pub fn correct_case(&self, input_t: &str) -> String {
        use unicode_normalization::UnicodeNormalization;
        let mut s = String::new();
        let normalized = input_t.nfc().collect::<String>();
        for c in normalized.chars() {
            match c {
                'o' | 'O' | 'i' | 'I' | 'u' | 'U' | 'd' | 'D' | 'g' | 'G' | 'j' | 'n' | 'N'
                | 'r' | 'R' | 's' | 'S' | 't' | 'T' | 'y' | 'Y' | 'z' | 'Z' => {
                    s.push(c);
                }
                'J' => {
                    // if EnableJoNukta { s.push(c); } else {
                    s.extend(c.to_lowercase());
                }
                _ => {
                    s.extend(c.to_lowercase());
                }
            }
        }
        s
    }
}

struct ParserState {
    p_english_text: Vec<char>,
    pos: usize,
    ln: usize,
    rs: String,
    determine_zwnj_zwj: char,
}

impl ParserState {
    fn new(text: &str, zwnj_zwj: char) -> Self {
        let chars: Vec<char> = text.chars().collect();
        let ln = chars.len();
        Self {
            p_english_text: chars,
            pos: 0, // In Rust we use 0-indexed
            ln,
            rs: String::new(),
            determine_zwnj_zwj: zwnj_zwj,
        }
    }


    fn add_rs_char(&mut self, t: char, move_pos: usize) {
        self.rs.push(t);
        self.pos += move_pos;
    }

    fn add_rs_chars(&mut self, chars: &[char], move_pos: usize) {
        for &c in chars {
            self.rs.push(c);
        }
        self.pos += move_pos;
    }

    fn prev_t(&self) -> Option<char> {
        if self.pos == 0 {
            None
        } else {
            self.p_english_text.get(self.pos - 1).copied()
        }
    }

    fn prev_t_ex(&self, position: usize) -> Option<char> {
        if self.pos < position {
            None
        } else {
            self.p_english_text.get(self.pos - position).copied()
        }
    }

    fn next_t(&self) -> Option<char> {
        self.p_english_text.get(self.pos + 1).copied()
    }

    fn next_t_ex(&self, i_length: usize, skipstart: usize) -> String {
        let start = self.pos + skipstart + 1;
        let mut res = String::new();
        for i in start..start + i_length {
            if let Some(&c) = self.p_english_text.get(i) {
                res.push(c);
            } else {
                break;
            }
        }
        res
    }

    fn vowel(&self, t: Option<char>) -> bool {
        match t {
            Some(c) => matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'),
            None => false,
        }
    }

    fn consonent(&self, t: Option<char>) -> bool {
        match t {
            Some(c) => matches!(
                c.to_ascii_lowercase(),
                'b' | 'c'
                    | 'd'
                    | 'f'
                    | 'g'
                    | 'h'
                    | 'j'
                    | 'k'
                    | 'l'
                    | 'm'
                    | 'n'
                    | 'p'
                    | 'q'
                    | 'r'
                    | 's'
                    | 't'
                    | 'v'
                    | 'w'
                    | 'x'
                    | 'y'
                    | 'z'
            ),
            None => false,
        }
    }

    fn number(&self, t: Option<char>) -> bool {
        match t {
            Some(c) => c.is_ascii_digit(),
            None => false,
        }
    }

    fn begining(&self) -> bool {
        let t = self.prev_t();
        match t {
            Some(c) => {
                if c.is_ascii_digit() || c.is_ascii_alphabetic() {
                    false
                } else {
                    true
                }
            }
            None => true,
        }
    }

    fn cnv_str(&mut self, compare: &str, if_true: &str) -> bool {
        let comp_chars: Vec<char> = compare.chars().collect();
        let i = comp_chars.len();
        if self.pos + i <= self.ln {
            let tmp = &self.p_english_text[self.pos..self.pos + i];
            if tmp == comp_chars.as_slice() {
                self.rs.push_str(if_true);
                self.pos += i;
                return true;
            }
        }
        false
    }

    fn cnv_chars(&mut self, compare: &str, if_true: &[char]) -> bool {
        let comp_chars: Vec<char> = compare.chars().collect();
        let i = comp_chars.len();
        if self.pos + i <= self.ln {
            let tmp = &self.p_english_text[self.pos..self.pos + i];
            if tmp == comp_chars.as_slice() {
                for &c in if_true {
                    self.rs.push(c);
                }
                self.pos += i;
                return true;
            }
        }
        false
    }

    fn my_convert(&mut self) -> String {
        while self.pos < self.ln {
            let tt = self.p_english_text[self.pos];

            match tt {
                // Number Generation
                '0' => self.add_rs_char(B_0, 1),
                '1' => self.add_rs_char(B_1, 1),
                '2' => self.add_rs_char(B_2, 1),
                '3' => self.add_rs_char(B_3, 1),
                '4' => self.add_rs_char(B_4, 1),
                '5' => self.add_rs_char(B_5, 1),
                '6' => self.add_rs_char(B_6, 1),
                '7' => self.add_rs_char(B_7, 1),
                '8' => self.add_rs_char(B_8, 1),
                '9' => self.add_rs_char(B_9, 1),

                // Vowel Generation
                'o' => self.small_o(),
                'a' | 'A' => {
                    if self.next_t() == Some('Z') {
                        self.add_rs_chars(&[B_A, B_HASANTA, B_Z, B_AAKAR], 2);
                    } else if self.begining() && self.next_t() != Some('`') {
                        self.add_rs_char(B_AA, 1);
                    } else if !self.consonent(self.prev_t())
                        && self.prev_t() != Some('a')
                        && self.next_t() != Some('`')
                    {
                        self.add_rs_chars(&[B_Y, B_AAKAR], 1);
                    } else if self.next_t() == Some('`') {
                        self.add_rs_char(B_AAKAR, 2);
                    } else if self.prev_t() == Some('a') && self.next_t() != Some('`') {
                        self.add_rs_char(B_AA, 1);
                    } else {
                        self.add_rs_char(B_AAKAR, 1);
                    }
                }
                'i' => {
                    if (!self.consonent(self.prev_t()) || self.begining())
                        && self.next_t() != Some('`')
                    {
                        self.add_rs_char(B_I, 1);
                    } else if self.next_t() == Some('`') {
                        self.add_rs_char(B_IKAR, 2);
                    } else {
                        self.add_rs_char(B_IKAR, 1);
                    }
                }
                'I' => {
                    if (!self.consonent(self.prev_t()) || self.begining())
                        && self.next_t() != Some('`')
                    {
                        self.add_rs_char(B_II, 1);
                    } else if self.next_t() == Some('`') {
                        self.add_rs_char(B_IIKAR, 2);
                    } else {
                        self.add_rs_char(B_IIKAR, 1);
                    }
                }
                'u' => {
                    if (!self.consonent(self.prev_t()) || self.begining())
                        && self.next_t() != Some('`')
                    {
                        self.add_rs_char(B_U, 1);
                    } else if self.next_t() == Some('`') {
                        self.add_rs_char(B_UKAR, 2);
                    } else {
                        self.add_rs_char(B_UKAR, 1);
                    }
                }
                'U' => {
                    if (!self.consonent(self.prev_t()) || self.begining())
                        && self.next_t() != Some('`')
                    {
                        self.add_rs_char(B_UU, 1);
                    } else if self.next_t() == Some('`') {
                        self.add_rs_char(B_UUKAR, 2);
                    } else {
                        self.add_rs_char(B_UUKAR, 1);
                    }
                }
                'e' | 'E' => {
                    if (!self.consonent(self.prev_t()) || self.begining())
                        && self.next_t() != Some('`')
                    {
                        if self.next_t() == Some('e') {
                            self.add_rs_char(B_II, 2);
                        } else {
                            self.add_rs_char(B_E, 1);
                        }
                    } else if self.next_t() == Some('`') {
                        self.add_rs_char(B_EKAR, 2);
                    } else {
                        if self.next_t() == Some('e') {
                            self.add_rs_char(B_IIKAR, 2);
                        } else {
                            self.add_rs_char(B_EKAR, 1);
                        }
                    }
                }
                'O' => self.big_o(),

                // Consonant Processing
                'k' => self.k(),
                'G' | 'g' => self.g(),
                'N' | 'n' => self.n(),
                'c' => self.c(),
                'J' | 'j' => self.j(),
                'T' | 't' => self.t(),
                'D' | 'd' => self.d(),
                'p' | 'f' => self.p(),
                'b' | 'v' => self.b(),
                'm' => self.m(),
                'z' => self.add_rs_char(B_Z, 1),
                'Z' => {
                    if self.prev_t() == Some('r') {
                        if self.consonent(self.prev_t_ex(2))
                            && self.prev_t_ex(2) != Some('r')
                            && self.prev_t_ex(2) != Some('y')
                            && self.prev_t_ex(2) != Some('w')
                            && self.prev_t_ex(2) != Some('x')
                        {
                            self.add_rs_chars(&[B_HASANTA, B_Z], 1);
                        } else {
                            self.add_rs_chars(&[self.determine_zwnj_zwj, B_HASANTA, B_Z], 1);
                        }
                    } else {
                        self.add_rs_chars(&[B_HASANTA, B_Z], 1);
                    }
                }
                'R' | 'r' => self.r(),
                'l' => self.l(),
                'S' | 's' => self.s(),
                'h' => self.h(),
                'y' => {
                    if !self.consonent(self.prev_t()) && !self.begining() {
                        self.add_rs_char(B_Y, 1);
                    } else if self.begining() {
                        self.add_rs_chars(&[B_I, B_Y], 1);
                    } else {
                        if self.prev_t() == Some('r') {
                            if self.consonent(self.prev_t_ex(2))
                                && self.prev_t_ex(2) != Some('r')
                                && self.prev_t_ex(2) != Some('y')
                                && self.prev_t_ex(2) != Some('w')
                                && self.prev_t_ex(2) != Some('x')
                            {
                                self.add_rs_chars(&[B_HASANTA, B_Z], 1);
                            } else {
                                self.add_rs_chars(&[self.determine_zwnj_zwj, B_HASANTA, B_Z], 1);
                            }
                        } else {
                            self.add_rs_chars(&[B_HASANTA, B_Z], 1);
                        }
                    }
                }
                'Y' => self.add_rs_char(B_Y, 1),
                'w' => {
                    if self.begining() && self.vowel(self.next_t()) {
                        self.add_rs_chars(&[B_O, B_Y], 1);
                    } else if self.consonent(self.prev_t()) {
                        self.add_rs_chars(&[B_HASANTA, B_B], 1);
                    } else {
                        self.add_rs_char(B_O, 1);
                    }
                }
                'q' => self.add_rs_char(B_K, 1),
                'x' => {
                    if self.begining() {
                        self.add_rs_chars(&[B_E, B_K, B_HASANTA, B_S], 1);
                    } else {
                        self.add_rs_chars(&[B_K, B_HASANTA, B_S], 1);
                    }
                }

                // Symbols
                '.' => self.dot(),
                ':' => {
                    if self.next_t() != Some('`') {
                        self.add_rs_char(B_BISHARGA, 1);
                    } else {
                        self.add_rs_char(':', 2);
                    }
                }
                '^' => {
                    if self.next_t() != Some('`') {
                        self.add_rs_char(B_CHANDRA, 1);
                    } else {
                        self.add_rs_char('^', 2);
                    }
                }
                ',' => {
                    if self.next_t() == Some(',') {
                        self.add_rs_chars(&[B_HASANTA, ZWNJ], 2);
                    } else {
                        self.add_rs_char(',', 1);
                    }
                }
                '$' => self.add_rs_char(B_TAKA, 1),
                '`' => self.pos += 1, // Bypass

                _ => self.add_rs_char(tt, 1),
            }
        }
        self.rs.clone()
    }

    fn b(&mut self) {
        if self.cnv_chars("bdh", &[B_B, B_HASANTA, B_DH]) { return; }
        if self.cnv_chars("bhl", &[B_BH, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("bj", &[B_B, B_HASANTA, B_J]) { return; }
        if self.cnv_chars("bd", &[B_B, B_HASANTA, B_D]) { return; }
        if self.cnv_chars("bb", &[B_B, B_HASANTA, B_B]) { return; }
        if self.cnv_chars("bl", &[B_B, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("bh", &[B_BH]) { return; }
        if self.cnv_chars("vl", &[B_BH, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("b", &[B_B]) { return; }
        if self.cnv_chars("v", &[B_BH]) { return; }
    }

    fn c(&mut self) {
        if self.cnv_chars("cNG", &[B_C, B_HASANTA, B_NYA]) { return; }
        if self.cnv_chars("cch", &[B_C, B_HASANTA, B_CH]) { return; }
        if self.cnv_chars("cc", &[B_C, B_HASANTA, B_C]) { return; }
        if self.cnv_chars("ch", &[B_CH]) { return; }
        if self.cnv_chars("c", &[B_C]) { return; }
    }

    fn d(&mut self) {
        if self.cnv_chars("dhn", &[B_DH, B_HASANTA, B_N]) { return; }
        if self.cnv_chars("dhm", &[B_DH, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("dgh", &[B_D, B_HASANTA, B_GH]) { return; }
        if self.cnv_chars("ddh", &[B_D, B_HASANTA, B_DH]) { return; }
        if self.cnv_chars("dbh", &[B_D, B_HASANTA, B_BH]) { return; }
        if self.cnv_chars("dv", &[B_D, B_HASANTA, B_BH]) { return; }
        if self.cnv_chars("dm", &[B_D, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("DD", &[B_DD, B_HASANTA, B_DD]) { return; }
        if self.cnv_chars("Dh", &[B_DDH]) { return; }
        if self.cnv_chars("dh", &[B_DH]) { return; }
        if self.cnv_chars("dg", &[B_D, B_HASANTA, B_G]) { return; }
        if self.cnv_chars("dd", &[B_D, B_HASANTA, B_D]) { return; }
        if self.cnv_chars("D", &[B_DD]) { return; }
        if self.cnv_chars("d", &[B_D]) { return; }
    }

    fn dot(&mut self) {
        if self.cnv_str("...", "...") { return; }
        if self.cnv_str(".`", ".") { return; }
        if self.cnv_chars("..", &[B_DARI, B_DARI]) { return; }
        if self.number(self.next_t()) {
            if self.cnv_str(".", ".") { return; }
        } else if self.cnv_chars(".", &[B_DARI]) { return; }
    }

    fn g(&mut self) {
        if self.cnv_chars("ghn", &[B_GH, B_HASANTA, B_N]) { return; }
        if self.cnv_chars("Ghn", &[B_GH, B_HASANTA, B_N]) { return; }
        if self.cnv_chars("gdh", &[B_G, B_HASANTA, B_DH]) { return; }
        if self.cnv_chars("Gdh", &[B_G, B_HASANTA, B_DH]) { return; }
        if self.cnv_chars("gN", &[B_G, B_HASANTA, B_NN]) { return; }
        if self.cnv_chars("GN", &[B_G, B_HASANTA, B_NN]) { return; }
        if self.cnv_chars("gn", &[B_G, B_HASANTA, B_N]) { return; }
        if self.cnv_chars("Gn", &[B_G, B_HASANTA, B_N]) { return; }
        if self.cnv_chars("gm", &[B_G, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("Gm", &[B_G, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("gl", &[B_G, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("Gl", &[B_G, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("gg", &[B_J, B_HASANTA, B_NYA]) { return; }
        if self.cnv_chars("GG", &[B_J, B_HASANTA, B_NYA]) { return; }
        if self.cnv_chars("Gg", &[B_J, B_HASANTA, B_NYA]) { return; }
        if self.cnv_chars("gG", &[B_J, B_HASANTA, B_NYA]) { return; }
        if self.cnv_chars("gh", &[B_GH]) { return; }
        if self.cnv_chars("Gh", &[B_GH]) { return; }
        if self.cnv_chars("g", &[B_G]) { return; }
        if self.cnv_chars("G", &[B_G]) { return; }
    }

    fn h(&mut self) {
        if self.cnv_chars("hN", &[B_H, B_HASANTA, B_NN]) { return; }
        if self.cnv_chars("hn", &[B_H, B_HASANTA, B_N]) { return; }
        if self.cnv_chars("hm", &[B_H, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("hl", &[B_H, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("h", &[B_H]) { return; }
    }

    fn j(&mut self) {
        if self.cnv_chars("jjh", &[B_J, B_HASANTA, B_JH]) { return; }
        if self.cnv_chars("jNG", &[B_J, B_HASANTA, B_NYA]) { return; }
        if self.cnv_chars("jh", &[B_JH]) { return; }
        if self.cnv_chars("jj", &[B_J, B_HASANTA, B_J]) { return; }
        if self.cnv_chars("j", &[B_J]) { return; }
        // EnableJoNukta branch is false in Pascal, so just B_J
        if self.cnv_chars("J", &[B_J]) { return; }
    }

    fn k(&mut self) {
        if self.cnv_chars("kkhN", &[B_K, B_HASANTA, B_SS, B_HASANTA, B_NN]) { return; }
        if self.cnv_chars("kShN", &[B_K, B_HASANTA, B_SS, B_HASANTA, B_NN]) { return; }
        if self.cnv_chars("kkhm", &[B_K, B_HASANTA, B_SS, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("kShm", &[B_K, B_HASANTA, B_SS, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("kxN", &[B_K, B_HASANTA, B_SS, B_HASANTA, B_NN]) { return; }
        if self.cnv_chars("kxm", &[B_K, B_HASANTA, B_SS, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("kkh", &[B_K, B_HASANTA, B_SS]) { return; }
        if self.cnv_chars("kSh", &[B_K, B_HASANTA, B_SS]) { return; }
        if self.cnv_chars("ksh", &[B_K, B_SH]) { return; }
        if self.cnv_chars("kx", &[B_K, B_HASANTA, B_SS]) { return; }
        if self.cnv_chars("kk", &[B_K, B_HASANTA, B_K]) { return; }
        if self.cnv_chars("kT", &[B_K, B_HASANTA, B_TT]) { return; }
        if self.cnv_chars("kt", &[B_K, B_HASANTA, B_T]) { return; }
        if self.cnv_chars("km", &[B_K, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("kl", &[B_K, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("ks", &[B_K, B_HASANTA, B_S]) { return; }
        if self.cnv_chars("kh", &[B_KH]) { return; }
        if self.cnv_chars("k", &[B_K]) { return; }
    }

    fn l(&mut self) {
        if self.cnv_chars("lbh", &[B_L, B_HASANTA, B_BH]) { return; }
        if self.cnv_chars("ldh", &[B_L, B_HASANTA, B_DH]) { return; }
        if self.cnv_chars("lkh", &[B_L, B_KH]) { return; }
        if self.cnv_chars("lgh", &[B_L, B_GH]) { return; }
        if self.cnv_chars("lph", &[B_L, B_PH]) { return; }
        if self.cnv_chars("lk", &[B_L, B_HASANTA, B_K]) { return; }
        if self.cnv_chars("lg", &[B_L, B_HASANTA, B_G]) { return; }
        if self.cnv_chars("lT", &[B_L, B_HASANTA, B_TT]) { return; }
        if self.cnv_chars("lD", &[B_L, B_HASANTA, B_DD]) { return; }
        if self.cnv_chars("lp", &[B_L, B_HASANTA, B_P]) { return; }
        if self.cnv_chars("lv", &[B_L, B_HASANTA, B_BH]) { return; }
        if self.cnv_chars("lm", &[B_L, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("ll", &[B_L, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("lb", &[B_L, B_HASANTA, B_B]) { return; }
        if self.cnv_chars("l", &[B_L]) { return; }
    }

    fn m(&mut self) {
        if self.cnv_chars("mth", &[B_M, B_HASANTA, B_TH]) { return; }
        if self.cnv_chars("mph", &[B_M, B_HASANTA, B_PH]) { return; }
        if self.cnv_chars("mbh", &[B_M, B_HASANTA, B_BH]) { return; }
        if self.cnv_chars("mpl", &[B_M, B_P, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("mn", &[B_M, B_HASANTA, B_N]) { return; }
        if self.cnv_chars("mp", &[B_M, B_HASANTA, B_P]) { return; }
        if self.cnv_chars("mv", &[B_M, B_HASANTA, B_BH]) { return; }
        if self.cnv_chars("mm", &[B_M, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("ml", &[B_M, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("mb", &[B_M, B_HASANTA, B_B]) { return; }
        if self.cnv_chars("mf", &[B_M, B_HASANTA, B_PH]) { return; }
        if self.cnv_chars("m", &[B_M]) { return; }
    }

    fn n(&mut self) {
        if self.cnv_chars("NgkSh", &[B_NGA, B_HASANTA, B_K, B_HASANTA, B_SS]) { return; }
        if self.cnv_chars("Ngkkh", &[B_NGA, B_HASANTA, B_K, B_HASANTA, B_SS]) { return; }
        if self.cnv_chars("NGch", &[B_NYA, B_HASANTA, B_CH]) { return; }
        if self.cnv_chars("Nggh", &[B_NGA, B_HASANTA, B_GH]) { return; }
        if self.cnv_chars("Ngkh", &[B_NGA, B_HASANTA, B_KH]) { return; }
        if self.cnv_chars("NGjh", &[B_NYA, B_HASANTA, B_JH]) { return; }
        if self.cnv_chars("ngOU", &[B_NGA, B_HASANTA, B_G, B_OUKAR]) { return; }
        if self.cnv_chars("ngOI", &[B_NGA, B_HASANTA, B_G, B_OIKAR]) { return; }
        if self.cnv_chars("Ngkx", &[B_NGA, B_HASANTA, B_K, B_HASANTA, B_SS]) { return; }
        if self.cnv_chars("NGc", &[B_NYA, B_HASANTA, B_C]) { return; }
        if self.cnv_chars("nch", &[B_NYA, B_HASANTA, B_CH]) { return; }
        if self.cnv_chars("njh", &[B_NYA, B_HASANTA, B_JH]) { return; }
        if self.cnv_chars("ngh", &[B_NGA, B_HASANTA, B_GH]) { return; }
        if self.cnv_chars("Ngk", &[B_NGA, B_HASANTA, B_K]) { return; }
        if self.cnv_chars("Ngx", &[B_NGA, B_HASANTA, B_SS]) { return; }
        if self.cnv_chars("Ngg", &[B_NGA, B_HASANTA, B_G]) { return; }
        if self.cnv_chars("Ngm", &[B_NGA, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("NGj", &[B_NYA, B_HASANTA, B_J]) { return; }
        if self.cnv_chars("ndh", &[B_N, B_HASANTA, B_DH]) { return; }
        if self.cnv_chars("nTh", &[B_N, B_HASANTA, B_TTH]) { return; }
        if self.cnv_chars("NTh", &[B_NN, B_HASANTA, B_TTH]) { return; }
        if self.cnv_chars("nth", &[B_N, B_HASANTA, B_TH]) { return; }
        if self.cnv_chars("nkh", &[B_NGA, B_HASANTA, B_KH]) { return; }
        if self.cnv_chars("ngo", &[B_NGA, B_HASANTA, B_G]) { return; }
        if self.cnv_chars("nga", &[B_NGA, B_HASANTA, B_G, B_AAKAR]) { return; }
        if self.cnv_chars("ngi", &[B_NGA, B_HASANTA, B_G, B_IKAR]) { return; }
        if self.cnv_chars("ngI", &[B_NGA, B_HASANTA, B_G, B_IIKAR]) { return; }
        if self.cnv_chars("ngu", &[B_NGA, B_HASANTA, B_G, B_UKAR]) { return; }
        if self.cnv_chars("ngU", &[B_NGA, B_HASANTA, B_G, B_UUKAR]) { return; }
        if self.cnv_chars("nge", &[B_NGA, B_HASANTA, B_G, B_EKAR]) { return; }
        if self.cnv_chars("ngO", &[B_NGA, B_HASANTA, B_G, B_OKAR]) { return; }
        if self.cnv_chars("NDh", &[B_NN, B_HASANTA, B_DDH]) { return; }
        if self.cnv_chars("nsh", &[B_N, B_SH]) { return; }
        if self.cnv_chars("Ngr", &[B_NGA, B_R]) { return; }
        if self.cnv_chars("NGr", &[B_NYA, B_R]) { return; }
        if self.cnv_chars("ngr", &[B_ANUSHAR, B_R]) { return; }
        if self.cnv_chars("nj", &[B_NYA, B_HASANTA, B_J]) { return; }
        if self.cnv_chars("Ng", &[B_NGA]) { return; }
        if self.cnv_chars("NG", &[B_NYA]) { return; }
        if self.cnv_chars("nk", &[B_NGA, B_HASANTA, B_K]) { return; }
        if self.cnv_chars("ng", &[B_ANUSHAR]) { return; }
        if self.cnv_chars("nn", &[B_N, B_HASANTA, B_N]) { return; }
        if self.cnv_chars("NN", &[B_NN, B_HASANTA, B_NN]) { return; }
        if self.cnv_chars("Nn", &[B_NN, B_HASANTA, B_N]) { return; }
        if self.cnv_chars("nm", &[B_N, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("Nm", &[B_NN, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("nd", &[B_N, B_HASANTA, B_D]) { return; }
        if self.cnv_chars("nT", &[B_N, B_HASANTA, B_TT]) { return; }
        if self.cnv_chars("NT", &[B_NN, B_HASANTA, B_TT]) { return; }
        if self.cnv_chars("nD", &[B_N, B_HASANTA, B_DD]) { return; }
        if self.cnv_chars("ND", &[B_NN, B_HASANTA, B_DD]) { return; }
        if self.cnv_chars("nt", &[B_N, B_HASANTA, B_T]) { return; }
        if self.cnv_chars("ns", &[B_N, B_HASANTA, B_S]) { return; }
        if self.cnv_chars("nc", &[B_NYA, B_HASANTA, B_C]) { return; }
        if self.cnv_chars("n", &[B_N]) { return; }
        if self.cnv_chars("N", &[B_NN]) { return; }
    }

    fn big_o(&mut self) {
        if self.cnv_chars("OI`", &[B_OIKAR]) { return; }
        if self.cnv_chars("OU`", &[B_OUKAR]) { return; }
        if self.cnv_chars("O`", &[B_OKAR]) { return; }

        if !self.consonent(self.prev_t()) || self.begining() {
            if self.cnv_chars("OI", &[B_OI]) { return; }
            if self.cnv_chars("OU", &[B_OU]) { return; }
            if self.cnv_chars("O", &[B_O]) { return; }
        } else {
            if self.cnv_chars("OI", &[B_OIKAR]) { return; }
            if self.cnv_chars("OU", &[B_OUKAR]) { return; }
            if self.cnv_chars("O", &[B_OKAR]) { return; }
        }
    }

    fn p(&mut self) {
        if self.cnv_chars("phl", &[B_PH, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("pT", &[B_P, B_HASANTA, B_TT]) { return; }
        if self.cnv_chars("pt", &[B_P, B_HASANTA, B_T]) { return; }
        if self.cnv_chars("pn", &[B_P, B_HASANTA, B_N]) { return; }
        if self.cnv_chars("pp", &[B_P, B_HASANTA, B_P]) { return; }
        if self.cnv_chars("pl", &[B_P, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("ps", &[B_P, B_HASANTA, B_S]) { return; }
        if self.cnv_chars("ph", &[B_PH]) { return; }
        if self.cnv_chars("fl", &[B_PH, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("f", &[B_PH]) { return; }
        if self.cnv_chars("p", &[B_P]) { return; }
    }

    fn r(&mut self) {
        if self.next_t_ex(1, 2) == "`" {
            if self.cnv_chars("rri", &[B_RRIKAR]) { return; }
        }
        if !self.consonent(self.prev_t()) {
            if self.cnv_chars("rri", &[B_RRI]) { return; }
        } else if self.begining() {
            if self.cnv_chars("rri", &[B_RRI]) { return; }
        } else {
            if self.cnv_chars("rri", &[B_RRIKAR]) { return; }
        }

        let n1 = self.next_t_ex(1, 1);
        let n1_c = n1.chars().next();
        if !self.consonent(self.prev_t()) && !self.vowel(n1_c) && n1 != "r" && !n1.is_empty() {
            if self.cnv_chars("rr", &[B_R, B_HASANTA]) { return; }
        }

        if self.cnv_chars("Rg", &[B_RR, B_HASANTA, B_G]) { return; }
        if self.cnv_chars("Rh", &[B_RRH]) { return; }

        if self.consonent(self.prev_t())
            && self.prev_t() != Some('r')
            && self.prev_t() != Some('y')
            && self.prev_t() != Some('w')
            && self.prev_t() != Some('x')
            && self.prev_t() != Some('Z')
        {
            if self.cnv_chars("r", &[B_HASANTA, B_R]) { return; }
        } else {
            if self.cnv_chars("r", &[B_R]) { return; }
        }

        if self.cnv_chars("R", &[B_RR]) { return; }
    }

    fn s(&mut self) {
        if self.cnv_chars("shch", &[B_SH, B_HASANTA, B_CH]) { return; }
        if self.cnv_chars("ShTh", &[B_SS, B_HASANTA, B_TTH]) { return; }
        if self.cnv_chars("Shph", &[B_SS, B_HASANTA, B_PH]) { return; }
        if self.cnv_chars("Sch", &[B_SH, B_HASANTA, B_CH]) { return; }
        if self.cnv_chars("skl", &[B_S, B_HASANTA, B_K, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("skh", &[B_S, B_HASANTA, B_KH]) { return; }
        if self.cnv_chars("sth", &[B_S, B_HASANTA, B_TH]) { return; }
        if self.cnv_chars("sph", &[B_S, B_HASANTA, B_PH]) { return; }
        if self.cnv_chars("shc", &[B_SH, B_HASANTA, B_C]) { return; }
        if self.cnv_chars("sht", &[B_SH, B_HASANTA, B_T]) { return; }
        if self.cnv_chars("shn", &[B_SH, B_HASANTA, B_N]) { return; }
        if self.cnv_chars("shm", &[B_SH, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("shl", &[B_SH, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("Shk", &[B_SS, B_HASANTA, B_K]) { return; }
        if self.cnv_chars("ShT", &[B_SS, B_HASANTA, B_TT]) { return; }
        if self.cnv_chars("ShN", &[B_SS, B_HASANTA, B_NN]) { return; }
        if self.cnv_chars("Shp", &[B_SS, B_HASANTA, B_P]) { return; }
        if self.cnv_chars("Shf", &[B_SS, B_HASANTA, B_PH]) { return; }
        if self.cnv_chars("Shm", &[B_SS, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("spl", &[B_S, B_HASANTA, B_P, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("sk", &[B_S, B_HASANTA, B_K]) { return; }
        if self.cnv_chars("Sc", &[B_SH, B_HASANTA, B_C]) { return; }
        if self.cnv_chars("sT", &[B_S, B_HASANTA, B_TT]) { return; }
        if self.cnv_chars("st", &[B_S, B_HASANTA, B_T]) { return; }
        if self.cnv_chars("sn", &[B_S, B_HASANTA, B_N]) { return; }
        if self.cnv_chars("sp", &[B_S, B_HASANTA, B_P]) { return; }
        if self.cnv_chars("sf", &[B_S, B_HASANTA, B_PH]) { return; }
        if self.cnv_chars("sm", &[B_S, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("sl", &[B_S, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("sh", &[B_SH]) { return; }
        if self.cnv_chars("Sc", &[B_SH, B_HASANTA, B_C]) { return; }
        if self.cnv_chars("St", &[B_SH, B_HASANTA, B_T]) { return; }
        if self.cnv_chars("Sn", &[B_SH, B_HASANTA, B_N]) { return; }
        if self.cnv_chars("Sm", &[B_SH, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("Sl", &[B_SH, B_HASANTA, B_L]) { return; }
        if self.cnv_chars("Sh", &[B_SS]) { return; }
        if self.cnv_chars("s", &[B_S]) { return; }
        if self.cnv_chars("S", &[B_SH]) { return; }
    }

    fn small_o(&mut self) {
        if (!self.consonent(self.prev_t()) || self.begining()) && self.next_t() != Some('`') {
            if self.cnv_chars("oo", &[B_U]) { return; }
            if self.cnv_chars("oZ", &[B_A, B_HASANTA, B_Z]) { return; }
            if self.vowel(self.prev_t()) && self.prev_t() != Some('o') {
                if self.cnv_chars("o", &[B_O]) { return; }
            } else {
                if self.cnv_chars("o", &[B_A]) { return; }
            }
        }
        if self.cnv_chars("oo", &[B_UKAR]) { return; }
        if self.cnv_str("o`", "") { return; }
        if self.cnv_str("o", "") { return; }
    }

    fn t(&mut self) {
        if self.cnv_chars("tth", &[B_T, B_HASANTA, B_TH]) { return; }
        if self.cnv_chars("t``", &[B_KHANDATTA]) { return; }
        if self.cnv_chars("TT", &[B_TT, B_HASANTA, B_TT]) { return; }
        if self.cnv_chars("Tm", &[B_TT, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("Th", &[B_TTH]) { return; }
        if self.cnv_chars("tn", &[B_T, B_HASANTA, B_N]) { return; }
        if self.cnv_chars("tm", &[B_T, B_HASANTA, B_M]) { return; }
        if self.cnv_chars("th", &[B_TH]) { return; }
        if self.cnv_chars("tt", &[B_T, B_HASANTA, B_T]) { return; }
        if self.cnv_chars("T", &[B_TT]) { return; }
        if self.cnv_chars("t", &[B_T]) { return; }
    }
}

#[cfg(test)]
#[path = "parser_test.rs"]
mod parser_test;
