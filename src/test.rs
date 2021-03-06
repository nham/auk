#![feature(phase)]

#[phase(plugin)] extern crate auk;
extern crate auk;

fn main() {}

#[cfg(test)]
mod test {
    #[test]
    fn test_char() {
        auk!(
            grammar achar {
                start = 'z'
            }
        )

        assert_eq!(achar::parse("zog"), Ok("og"));
        assert!(achar::parse("wat").is_err());
        assert!(achar::parse("").is_err());
    }

    #[test]
    fn test_str() {
        auk!(
            grammar astr {
                start = "abc"
            }
        )

        assert_eq!(astr::parse("abcde"), Ok("de"));
        assert!(astr::parse("abde").is_err());
        assert!(astr::parse("").is_err());
    }

    #[test]
    fn test_dot() {
        auk!(
            grammar dot {
                start = .
            }
        )

        assert_eq!(dot::parse("abcde"), Ok("bcde"));
        assert!(dot::parse("").is_err());
    }

    #[test]
    fn test_class() {
        auk!(
            grammar vowels {
                start = ["aeiou"]
            }
        )

        assert_eq!(vowels::parse("acaptain"), Ok("captain"));
        assert_eq!(vowels::parse("ecaptain"), Ok("captain"));
        assert_eq!(vowels::parse("icaptain"), Ok("captain"));
        assert_eq!(vowels::parse("ocaptain"), Ok("captain"));
        assert_eq!(vowels::parse("ucaptain"), Ok("captain"));
        assert!(vowels::parse("captain").is_err());
        assert!(vowels::parse("").is_err());
    }

    #[test]
    fn test_opt() {
        auk!(
            grammar zopt {
                start = 'z'?
            }
        )

        auk!(
            grammar dotopt {
                start = .?
            }
        )

        auk!(
            grammar abcopt {
                start = "abc"?
            }
        )

        auk!(
            grammar vowelopt {
                start = ["aeiou"]?
            }
        )

        auk!(
            grammar parenopt {
                start = ('z' / "abc" / ["aeiou"])?
            }
        )

        assert_eq!(zopt::parse("zabc"), Ok("abc"));
        assert_eq!(zopt::parse("abc"), Ok("abc"));
        assert_eq!(zopt::parse(""), Ok(""));

        assert_eq!(dotopt::parse("..."), Ok(".."));
        assert_eq!(dotopt::parse("abc"), Ok("bc"));
        assert_eq!(dotopt::parse(""), Ok(""));

        assert_eq!(abcopt::parse("abc"), Ok(""));
        assert_eq!(abcopt::parse("abcabcabcdef"), Ok("abcabcdef"));
        assert_eq!(abcopt::parse("abba"), Ok("abba"));
        assert_eq!(abcopt::parse(""), Ok(""));

        assert_eq!(vowelopt::parse("a"), Ok(""));
        assert_eq!(vowelopt::parse("e"), Ok(""));
        assert_eq!(vowelopt::parse("i"), Ok(""));
        assert_eq!(vowelopt::parse("o"), Ok(""));
        assert_eq!(vowelopt::parse("u"), Ok(""));
        assert_eq!(vowelopt::parse("cat"), Ok("cat"));
        assert_eq!(vowelopt::parse(""), Ok(""));

        assert_eq!(parenopt::parse("zabc"), Ok("abc"));
        assert_eq!(parenopt::parse("abcabcabcdef"), Ok("abcabcdef"));
        assert_eq!(parenopt::parse("abbc"), Ok("bbc"));
        assert_eq!(parenopt::parse("ebbc"), Ok("bbc"));
        assert_eq!(parenopt::parse("ibbc"), Ok("bbc"));
        assert_eq!(parenopt::parse("obbc"), Ok("bbc"));
        assert_eq!(parenopt::parse("ubbc"), Ok("bbc"));
        assert_eq!(parenopt::parse("hello"), Ok("hello"));
    }

    #[test]
    fn test_star() {
        auk!(
            grammar zstar {
                start = 'z'*
            }
        )

        auk!(
            grammar dotstar {
                start = .*
            }
        )

        auk!(
            grammar abcstar {
                start = "abc"*
            }
        )

        auk!(
            grammar vowelstar {
                start = ["aeiou"]*
            }
        )

        auk!(
            grammar parenstar {
                start = ('z' / "abc" / ["aeiou"])*
            }
        )

        assert_eq!(zstar::parse("zabc"), Ok("abc"));
        assert_eq!(zstar::parse("zzzzzzzabc"), Ok("abc"));
        assert_eq!(zstar::parse("abc"), Ok("abc"));
        assert_eq!(zstar::parse(""), Ok(""));

        assert_eq!(dotstar::parse("the cat in the hat. sat! on the? mat"), Ok(""));
        assert_eq!(dotstar::parse(""), Ok(""));

        assert_eq!(abcstar::parse("abc"), Ok(""));
        assert_eq!(abcstar::parse("abcabcabcdef"), Ok("def"));
        assert_eq!(abcstar::parse("abba"), Ok("abba"));
        assert_eq!(abcstar::parse(""), Ok(""));

        assert_eq!(vowelstar::parse("e"), Ok(""));
        assert_eq!(vowelstar::parse("oiiaeuooaToaoaoiii"), Ok("Toaoaoiii"));
        assert_eq!(vowelstar::parse("cat"), Ok("cat"));
        assert_eq!(vowelstar::parse(""), Ok(""));

        assert_eq!(parenstar::parse("zzzzxy"), Ok("xy"));
        assert_eq!(parenstar::parse("abcabcabcdef"), Ok("def"));
        assert_eq!(parenstar::parse("oiiaeuooaToaoaoiii"), Ok("Toaoaoiii"));
        assert_eq!(parenstar::parse("zabczaeoabcuzabc"), Ok(""));
        assert_eq!(parenstar::parse(""), Ok(""));
        assert_eq!(parenstar::parse("hello"), Ok("hello"));
    }

    #[test]
    fn test_plus() {
        auk!(
            grammar zplus {
                start = 'z'+
            }
        )

        auk!(
            grammar dotplus {
                start = .+
            }
        )

        auk!(
            grammar abcplus {
                start = "abc"+
            }
        )

        auk!(
            grammar vowelplus {
                start = ["aeiou"]+
            }
        )

        auk!(
            grammar parenplus {
                start = ('z' / "abc" / ["aeiou"])+
            }
        )

        assert_eq!(zplus::parse("zabc"), Ok("abc"));
        assert_eq!(zplus::parse("zzzzzzzabc"), Ok("abc"));
        assert!(zplus::parse("abc").is_err());
        assert!(zplus::parse("").is_err());

        assert_eq!(dotplus::parse("the cat in the hat. sat! on the? mat"), Ok(""));
        assert!(dotplus::parse("").is_err());

        assert_eq!(abcplus::parse("abc"), Ok(""));
        assert_eq!(abcplus::parse("abcabcabcdef"), Ok("def"));
        assert!(abcplus::parse("abba").is_err());
        assert!(abcplus::parse("").is_err());

        assert_eq!(vowelplus::parse("e"), Ok(""));
        assert_eq!(vowelplus::parse("oiiaeuooaToaoaoiii"), Ok("Toaoaoiii"));
        assert!(vowelplus::parse("cat").is_err());
        assert!(vowelplus::parse("").is_err());

        assert_eq!(parenplus::parse("zzzzxy"), Ok("xy"));
        assert_eq!(parenplus::parse("abcabcabcdef"), Ok("def"));
        assert_eq!(parenplus::parse("oiiaeuooaToaoaoiii"), Ok("Toaoaoiii"));
        assert_eq!(parenplus::parse("zabczaeoabcuzabc"), Ok(""));
        assert!(parenplus::parse("").is_err());
        assert!(parenplus::parse("hello").is_err());
    }

    #[test]
    fn test_neglookahead() {
        auk!(
            grammar zneg {
                start = !'z'
            }
        )

        auk!(
            grammar abcneg {
                start = !"abc"
            }
        )

        auk!(
            grammar dotneg {
                start = !.
            }
        )

        auk!(
            grammar vowelsneg {
                start = !["aeiou"]
            }
        )

        auk!(
            grammar e_star_neg {
                start = !'e'*
            }
        )

        auk!(
            grammar e_plus_neg {
                start = !'e'+
            }
        )

        assert!(zneg::parse("zog").is_err());
        assert_eq!(zneg::parse("wat"), Ok("wat"));
        assert_eq!(zneg::parse(""), Ok(""));

        assert!(abcneg::parse("abcde").is_err());
        assert_eq!(abcneg::parse("abde"), Ok("abde"));
        assert_eq!(abcneg::parse(""), Ok(""));

        assert!(dotneg::parse("zuh").is_err());
        assert_eq!(dotneg::parse(""), Ok(""));

        assert!(vowelsneg::parse("oof").is_err());
        assert_eq!(vowelsneg::parse("baby"), Ok("baby"));
        assert_eq!(vowelsneg::parse(""), Ok(""));

        assert!(e_star_neg::parse("ehello").is_err());
        assert!(e_star_neg::parse("eeeehello").is_err());
        assert!(e_star_neg::parse("hello").is_err());
        assert!(e_star_neg::parse("").is_err());

        assert!(e_plus_neg::parse("ehello").is_err());
        assert!(e_plus_neg::parse("eeeehello").is_err());
        assert_eq!(e_plus_neg::parse("hello"), Ok("hello"));
        assert_eq!(e_plus_neg::parse(""), Ok(""));
    }

    #[test]
    fn test_poslookahead() {
        auk!(
            grammar zpos {
                start = &'z'
            }
        )

        auk!(
            grammar abcpos {
                start = &"abc"
            }
        )

        auk!(
            grammar dotpos {
                start = &.
            }
        )

        auk!(
            grammar vowelspos {
                start = &["aeiou"]
            }
        )

        auk!(
            grammar e_star_pos {
                start = &'e'*
            }
        )

        auk!(
            grammar e_plus_pos {
                start = &'e'+
            }
        )

        assert_eq!(zpos::parse("zog"), Ok("zog"));
        assert!(zpos::parse("wat").is_err());
        assert!(zpos::parse("").is_err());

        assert_eq!(abcpos::parse("abcde"), Ok("abcde"));
        assert!(abcpos::parse("abde").is_err());
        assert!(abcpos::parse("").is_err());

        assert_eq!(dotpos::parse("zuh"), Ok("zuh"));
        assert!(dotpos::parse("").is_err());

        assert_eq!(vowelspos::parse("oof"), Ok("oof"));
        assert!(vowelspos::parse("baby").is_err());
        assert!(vowelspos::parse("").is_err());

        assert_eq!(e_star_pos::parse("ehello"), Ok("ehello"));
        assert_eq!(e_star_pos::parse("eeeehello"), Ok("eeeehello"));
        assert_eq!(e_star_pos::parse("hello"), Ok("hello"));
        assert_eq!(e_star_pos::parse(""), Ok(""));

        assert_eq!(e_plus_pos::parse("ehello"), Ok("ehello"));
        assert_eq!(e_plus_pos::parse("eeeehello"), Ok("eeeehello"));
        assert!(e_plus_pos::parse("hello").is_err());
        assert!(e_plus_pos::parse("").is_err());
    }

    #[test]
    fn test_seq() {
        auk!(
            grammar seq1 {
                start = "abc" "def"
            }
        )

        auk!(
            grammar seq2 {
                start = 'x' 'y'
            }
        )

        auk!(
            grammar alt3 {
                start = "bbc" ["aeiou"]? 'z'
            }
        )

        assert!(seq1::parse("abc").is_err());
        assert!(seq1::parse("abcde").is_err());
        assert!(seq1::parse("").is_err());
        assert_eq!(seq1::parse("abcdefgh"), Ok("gh"));

        assert!(seq2::parse("x").is_err());
        assert!(seq2::parse("").is_err());
        assert_eq!(seq2::parse("xyz"), Ok("z"));

        assert!(alt3::parse("bbc").is_err());
        assert!(alt3::parse("").is_err());
        assert_eq!(alt3::parse("bbczbbc"), Ok("bbc"));
        assert_eq!(alt3::parse("bbcazbbc"), Ok("bbc"));
        assert_eq!(alt3::parse("bbcezbbc"), Ok("bbc"));
        assert_eq!(alt3::parse("bbcizbbc"), Ok("bbc"));
        assert_eq!(alt3::parse("bbcozbbc"), Ok("bbc"));
        assert_eq!(alt3::parse("bbcuzbbc"), Ok("bbc"));
    }

    #[test]
    fn test_alt() {
        auk!(
            grammar alt1 {
                start = "abc" / "def"
            }
        )

        auk!(
            grammar alt2 {
                start = 'x' / 'y'
            }
        )

        auk!(
            grammar alt3 {
                start = "bbc" / ["aeiou"] / 'z'
            }
        )

        assert_eq!(alt1::parse("abc"), Ok(""));
        assert_eq!(alt1::parse("abcdef"), Ok("def"));
        assert_eq!(alt1::parse("def"), Ok(""));
        assert_eq!(alt1::parse("defgh"), Ok("gh"));
        assert!(alt1::parse("").is_err());

        assert_eq!(alt2::parse("x"), Ok(""));
        assert_eq!(alt2::parse("xy"), Ok("y"));
        assert_eq!(alt2::parse("y"), Ok(""));
        assert_eq!(alt2::parse("yza"), Ok("za"));
        assert!(alt2::parse("").is_err());

        assert_eq!(alt3::parse("bbc"), Ok(""));
        assert_eq!(alt3::parse("bbczbbc"), Ok("zbbc"));
        assert_eq!(alt3::parse("azbbc"), Ok("zbbc"));
        assert_eq!(alt3::parse("ezbbc"), Ok("zbbc"));
        assert_eq!(alt3::parse("izbbc"), Ok("zbbc"));
        assert_eq!(alt3::parse("ozbbc"), Ok("zbbc"));
        assert_eq!(alt3::parse("uzbbc"), Ok("zbbc"));
        assert_eq!(alt3::parse("zog"), Ok("og"));
        assert!(alt3::parse("").is_err());
    }

    #[test]
    fn test_parens() {
        auk!(
            grammar zparens {
                start = ((('z')))
            }
        )

        assert_eq!(zparens::parse("zog"), Ok("og"));
        assert!(zparens::parse("wat").is_err());
        assert!(zparens::parse("").is_err());
    }

    #[test]
    fn test_greedy_choice() {
        auk!(
            grammar vowels_abc {
                start = ["aeiou"] / "abc"
            }
        )

        assert_eq!(vowels_abc::parse("oof"), Ok("of"));
        assert_eq!(vowels_abc::parse("abc"), Ok("bc"));
    }

    #[test]
    fn test_multiple_rules() {
        auk!(
            grammar vowels_abc {
                arule = another / ["aeiou"]
                another = "abc"
            }
        )

        auk!(
            grammar exp_recognizer {
                exp = num sp '+' sp num
                    / num sp '-' sp num
                    / num sp '*' sp num
                    / num sp '/' sp num
                    / sp num sp
                dig = ["0123456789"]
                num = dig+
                sp = [" \r\n\t"]*
            }
        )

        assert_eq!(vowels_abc::parse("oof"), Ok("of"));
        assert_eq!(vowels_abc::parse("abc"), Ok(""));

        assert!(exp_recognizer::parse("").is_err());
        assert_eq!(exp_recognizer::parse("1"), Ok(""));
        assert_eq!(exp_recognizer::parse("123"), Ok(""));
        assert_eq!(exp_recognizer::parse("123 + 45"), Ok(""));
        assert_eq!(exp_recognizer::parse("123 - 45"), Ok(""));
        assert_eq!(exp_recognizer::parse("123 * 45"), Ok(""));
        assert_eq!(exp_recognizer::parse("123 / 45"), Ok(""));
    }
}
