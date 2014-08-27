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

        assert_eq!(achar("zog"), Ok("og"));
        assert!(achar("wat").is_err());
        assert!(achar("").is_err());
    }

    #[test]
    fn test_str() {
        auk!(
            grammar astr {
                start = "abc"
            }
        )

        assert_eq!(astr("abcde"), Ok("de"));
        assert!(astr("abde").is_err());
        assert!(astr("").is_err());
    }

    #[test]
    fn test_dot() {
        auk!(
            grammar dot {
                start = .
            }
        )

        assert_eq!(dot("abcde"), Ok("bcde"));
        assert!(dot("").is_err());
    }

    #[test]
    fn test_class() {
        auk!(
            grammar vowels {
                start = ["aeiou"]
            }
        )

        assert_eq!(vowels("acaptain"), Ok("captain"));
        assert_eq!(vowels("ecaptain"), Ok("captain"));
        assert_eq!(vowels("icaptain"), Ok("captain"));
        assert_eq!(vowels("ocaptain"), Ok("captain"));
        assert_eq!(vowels("ucaptain"), Ok("captain"));
        assert!(vowels("captain").is_err());
        assert!(vowels("").is_err());
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

        assert_eq!(zopt("zabc"), Ok("abc"));
        assert_eq!(zopt("abc"), Ok("abc"));
        assert_eq!(zopt(""), Ok(""));

        assert_eq!(dotopt("..."), Ok(".."));
        assert_eq!(dotopt("abc"), Ok("bc"));
        assert_eq!(dotopt(""), Ok(""));

        assert_eq!(abcopt("abc"), Ok(""));
        assert_eq!(abcopt("abcabcabcdef"), Ok("abcabcdef"));
        assert_eq!(abcopt("abba"), Ok("abba"));
        assert_eq!(abcopt(""), Ok(""));

        assert_eq!(vowelopt("a"), Ok(""));
        assert_eq!(vowelopt("e"), Ok(""));
        assert_eq!(vowelopt("i"), Ok(""));
        assert_eq!(vowelopt("o"), Ok(""));
        assert_eq!(vowelopt("u"), Ok(""));
        assert_eq!(vowelopt("cat"), Ok("cat"));
        assert_eq!(vowelopt(""), Ok(""));
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

        assert_eq!(zstar("zabc"), Ok("abc"));
        assert_eq!(zstar("zzzzzzzabc"), Ok("abc"));
        assert_eq!(zstar("abc"), Ok("abc"));
        assert_eq!(zstar(""), Ok(""));

        assert_eq!(dotstar("the cat in the hat. sat! on the? mat"), Ok(""));
        assert_eq!(dotstar(""), Ok(""));

        assert_eq!(abcstar("abc"), Ok(""));
        assert_eq!(abcstar("abcabcabcdef"), Ok("def"));
        assert_eq!(abcstar("abba"), Ok("abba"));
        assert_eq!(abcstar(""), Ok(""));

        assert_eq!(vowelstar("e"), Ok(""));
        assert_eq!(vowelstar("oiiaeuooaToaoaoiii"), Ok("Toaoaoiii"));
        assert_eq!(vowelstar("cat"), Ok("cat"));
        assert_eq!(vowelstar(""), Ok(""));
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

        assert_eq!(zplus("zabc"), Ok("abc"));
        assert_eq!(zplus("zzzzzzzabc"), Ok("abc"));
        assert!(zplus("abc").is_err());
        assert!(zplus("").is_err());

        assert_eq!(dotplus("the cat in the hat. sat! on the? mat"), Ok(""));
        assert!(dotplus("").is_err());

        assert_eq!(abcplus("abc"), Ok(""));
        assert_eq!(abcplus("abcabcabcdef"), Ok("def"));
        assert!(abcplus("abba").is_err());
        assert!(abcplus("").is_err());

        assert_eq!(vowelplus("e"), Ok(""));
        assert_eq!(vowelplus("oiiaeuooaToaoaoiii"), Ok("Toaoaoiii"));
        assert!(vowelplus("cat").is_err());
        assert!(vowelplus("").is_err());
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

        assert!(zneg("zog").is_err());
        assert_eq!(zneg("wat"), Ok("wat"));
        assert_eq!(zneg(""), Ok(""));

        assert!(abcneg("abcde").is_err());
        assert_eq!(abcneg("abde"), Ok("abde"));
        assert_eq!(abcneg(""), Ok(""));

        assert!(dotneg("zuh").is_err());
        assert_eq!(dotneg(""), Ok(""));

        assert!(vowelsneg("oof").is_err());
        assert_eq!(vowelsneg("baby"), Ok("baby"));
        assert_eq!(vowelsneg(""), Ok(""));

        assert!(e_star_neg("ehello").is_err());
        assert!(e_star_neg("eeeehello").is_err());
        assert!(e_star_neg("hello").is_err());
        assert!(e_star_neg("").is_err());

        assert!(e_plus_neg("ehello").is_err());
        assert!(e_plus_neg("eeeehello").is_err());
        assert_eq!(e_plus_neg("hello"), Ok("hello"));
        assert_eq!(e_plus_neg(""), Ok(""));
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

        assert_eq!(zpos("zog"), Ok("zog"));
        assert!(zpos("wat").is_err());
        assert!(zpos("").is_err());

        assert_eq!(abcpos("abcde"), Ok("abcde"));
        assert!(abcpos("abde").is_err());
        assert!(abcpos("").is_err());

        assert_eq!(dotpos("zuh"), Ok("zuh"));
        assert!(dotpos("").is_err());

        assert_eq!(vowelspos("oof"), Ok("oof"));
        assert!(vowelspos("baby").is_err());
        assert!(vowelspos("").is_err());

        assert_eq!(e_star_pos("ehello"), Ok("ehello"));
        assert_eq!(e_star_pos("eeeehello"), Ok("eeeehello"));
        assert_eq!(e_star_pos("hello"), Ok("hello"));
        assert_eq!(e_star_pos(""), Ok(""));

        assert_eq!(e_plus_pos("ehello"), Ok("ehello"));
        assert_eq!(e_plus_pos("eeeehello"), Ok("eeeehello"));
        assert!(e_plus_pos("hello").is_err());
        assert!(e_plus_pos("").is_err());
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

        assert!(seq1("abc").is_err());
        assert!(seq1("abcde").is_err());
        assert!(seq1("").is_err());
        assert_eq!(seq1("abcdefgh"), Ok("gh"));

        assert!(seq2("x").is_err());
        assert!(seq2("").is_err());
        assert_eq!(seq2("xyz"), Ok("z"));

        assert!(alt3("bbc").is_err());
        assert!(alt3("").is_err());
        assert_eq!(alt3("bbczbbc"), Ok("bbc"));
        assert_eq!(alt3("bbcazbbc"), Ok("bbc"));
        assert_eq!(alt3("bbcezbbc"), Ok("bbc"));
        assert_eq!(alt3("bbcizbbc"), Ok("bbc"));
        assert_eq!(alt3("bbcozbbc"), Ok("bbc"));
        assert_eq!(alt3("bbcuzbbc"), Ok("bbc"));
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

        assert_eq!(alt1("abc"), Ok(""));
        assert_eq!(alt1("abcdef"), Ok("def"));
        assert_eq!(alt1("def"), Ok(""));
        assert_eq!(alt1("defgh"), Ok("gh"));
        assert!(alt1("").is_err());

        assert_eq!(alt2("x"), Ok(""));
        assert_eq!(alt2("xy"), Ok("y"));
        assert_eq!(alt2("y"), Ok(""));
        assert_eq!(alt2("yza"), Ok("za"));
        assert!(alt2("").is_err());

        assert_eq!(alt3("bbc"), Ok(""));
        assert_eq!(alt3("bbczbbc"), Ok("zbbc"));
        assert_eq!(alt3("azbbc"), Ok("zbbc"));
        assert_eq!(alt3("ezbbc"), Ok("zbbc"));
        assert_eq!(alt3("izbbc"), Ok("zbbc"));
        assert_eq!(alt3("ozbbc"), Ok("zbbc"));
        assert_eq!(alt3("uzbbc"), Ok("zbbc"));
        assert_eq!(alt3("zog"), Ok("og"));
        assert!(alt3("").is_err());
    }
}
