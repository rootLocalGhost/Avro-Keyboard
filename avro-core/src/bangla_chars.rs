pub const B_VOCALIC_L: char = '\u{098C}';
pub const B_VOCALIC_LL: char = '\u{09E1}';
pub const B_VOCALIC_RR: char = '\u{09E0}';

pub const B_VOCALIC_RR_KAR: char = '\u{09C4}';
pub const B_VOCALIC_L_KAR: char = '\u{09E2}';
pub const B_VOCALIC_LL_KAR: char = '\u{09E3}';

pub const B_NUKTA: char = '\u{09BC}';
pub const B_AVAGRAHA: char = '\u{09BD}';
pub const B_LENGTH_MARK: char = '\u{09D7}';

pub const B_RUPEE_MARK: char = '\u{09F2}';
pub const B_CURRENCY_NUMERATOR_1: char = '\u{09F4}';
pub const B_CURRENCY_NUMERATOR_2: char = '\u{09F5}';
pub const B_CURRENCY_NUMERATOR_3: char = '\u{09F6}';
pub const B_CURRENCY_NUMERATOR_4: char = '\u{09F7}';
pub const B_CURRENCY_NUMERATOR_1_LESS_THAN_DENOMINATOR: char = '\u{09F8}';
pub const B_CURRENCY_DENOMINATOR_16: char = '\u{09F9}';
pub const B_CURRENCY_ESSHAR: char = '\u{09FA}';

pub const B_0: char = '\u{09E6}';
pub const B_1: char = '\u{09E7}';
pub const B_2: char = '\u{09E8}';
pub const B_3: char = '\u{09E9}';
pub const B_4: char = '\u{09EA}';
pub const B_5: char = '\u{09EB}';
pub const B_6: char = '\u{09EC}';
pub const B_7: char = '\u{09ED}';
pub const B_8: char = '\u{09EE}';
pub const B_9: char = '\u{09EF}';

pub const B_A: char = '\u{0985}';
pub const B_AA: char = '\u{0986}';
pub const B_AAKAR: char = '\u{09BE}';
pub const B_I: char = '\u{0987}';
pub const B_II: char = '\u{0988}';
pub const B_IIKAR: char = '\u{09C0}';
pub const B_IKAR: char = '\u{09BF}';
pub const B_U: char = '\u{0989}';
pub const B_UKAR: char = '\u{09C1}';
pub const B_UU: char = '\u{098A}';
pub const B_UUKAR: char = '\u{09C2}';
pub const B_RRI: char = '\u{098B}';
pub const B_RRIKAR: char = '\u{09C3}';
pub const B_E: char = '\u{098F}';
pub const B_EKAR: char = '\u{09C7}';
pub const B_O: char = '\u{0993}';
pub const B_OI: char = '\u{0990}';
pub const B_OIKAR: char = '\u{09C8}';
pub const B_OKAR: char = '\u{09CB}';
pub const B_OU: char = '\u{0994}';
pub const B_OUKAR: char = '\u{09CC}';

pub const B_ANUSHAR: char = '\u{0982}';
pub const B_B: char = '\u{09AC}';
pub const B_BH: char = '\u{09AD}';
pub const B_BISHARGA: char = '\u{0983}';
pub const B_C: char = '\u{099A}';
pub const B_CH: char = '\u{099B}';
pub const B_CHANDRA: char = '\u{0981}';
pub const B_D: char = '\u{09A6}';
pub const B_DD: char = '\u{09A1}';
pub const B_DDH: char = '\u{09A2}';
pub const B_DH: char = '\u{09A7}';
pub const B_G: char = '\u{0997}';
pub const B_GH: char = '\u{0998}';
pub const B_H: char = '\u{09B9}';
pub const B_J: char = '\u{099C}';
pub const B_JH: char = '\u{099D}';
pub const B_K: char = '\u{0995}';
pub const B_KH: char = '\u{0996}';
pub const B_L: char = '\u{09B2}';
pub const B_M: char = '\u{09AE}';
pub const B_N: char = '\u{09A8}';
pub const B_NGA: char = '\u{0999}';
pub const B_NN: char = '\u{09A3}';
pub const B_NYA: char = '\u{099E}';
pub const B_P: char = '\u{09AA}';
pub const B_PH: char = '\u{09AB}';
pub const B_R: char = '\u{09B0}';
pub const B_RR: char = '\u{09DC}';
pub const B_RRH: char = '\u{09DD}';
pub const B_S: char = '\u{09B8}';
pub const B_SH: char = '\u{09B6}';
pub const B_SS: char = '\u{09B7}';
pub const B_T: char = '\u{09A4}';
pub const B_TH: char = '\u{09A5}';
pub const B_TT: char = '\u{099F}';
pub const B_TTH: char = '\u{09A0}';
pub const B_Y: char = '\u{09DF}';
pub const B_Z: char = '\u{09AF}';
pub const ASSAM_RA: char = '\u{09F0}';
pub const ASSAM_VA: char = '\u{09F1}';
pub const B_KHANDATTA: char = '\u{09CE}';

pub const B_DARI: char = '\u{0964}';
pub const B_HASANTA: char = '\u{09CD}';
pub const B_TAKA: char = '\u{09F3}';
pub const ZWJ: char = '\u{200D}';
pub const ZWNJ: char = '\u{200C}';

pub fn is_vowel(c: char) -> bool {
    matches!(
        c,
        B_A | B_AA
            | B_AAKAR
            | B_I
            | B_II
            | B_IIKAR
            | B_IKAR
            | B_U
            | B_UKAR
            | B_UU
            | B_UUKAR
            | B_RRI
            | B_RRIKAR
            | B_E
            | B_EKAR
            | B_OI
            | B_OIKAR
            | B_O
            | B_OKAR
            | B_OU
            | B_OUKAR
            | B_VOCALIC_L
            | B_VOCALIC_LL
            | B_VOCALIC_RR
            | B_VOCALIC_RR_KAR
            | B_VOCALIC_L_KAR
            | B_VOCALIC_LL_KAR
    )
}

pub fn is_pure_consonent(c: char) -> bool {
    matches!(
        c,
        B_B | B_BH
            | B_C
            | B_CH
            | B_D
            | B_DD
            | B_DDH
            | B_DH
            | B_G
            | B_GH
            | B_H
            | B_J
            | B_JH
            | B_K
            | B_KH
            | B_L
            | B_M
            | B_N
            | B_NGA
            | B_NN
            | B_NYA
            | B_P
            | B_PH
            | B_R
            | B_RR
            | B_RRH
            | B_S
            | B_SH
            | B_SS
            | B_T
            | B_TH
            | B_TT
            | B_TTH
            | B_Z
            | B_Y
            | B_KHANDATTA
            | ASSAM_RA
            | ASSAM_VA
    )
}

pub fn is_kar(c: char) -> bool {
    matches!(
        c,
        B_AAKAR
            | B_IIKAR
            | B_IKAR
            | B_UKAR
            | B_UUKAR
            | B_RRIKAR
            | B_EKAR
            | B_OIKAR
            | B_OUKAR
            | B_VOCALIC_RR_KAR
            | B_VOCALIC_L_KAR
            | B_VOCALIC_LL_KAR
    )
}
