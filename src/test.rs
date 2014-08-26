#![feature(phase)]

#[phase(plugin)] extern crate auk;
extern crate auk;

fn main() {
    auk!(
        grammar achar {
            start = 'z'
        }
    )

    auk!(
        grammar charneg {
            start = !'z'
        }
    )

    auk!(
        grammar charpos {
            start = &'z'
        }
    )

    auk!(
        grammar astr {
            start = "abc"
        }
    )

    auk!(
        grammar strpos {
            start = &"abc"
        }
    )

    auk!(
        grammar strneg {
            start = !"abc"
        }
    )

    auk!(
        grammar dot {
            start = .
        }
    )

    auk!(
        grammar dotpos {
            start = &.
        }
    )

    auk!(
        grammar dotneg {
            start = !.
        }
    )

    auk!(
        grammar vowels {
            start = ["aeiou"]
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
        grammar abcplus {
            start = "abc"+
        }
    )

    auk!(
        grammar abcq {
            start = "abc"?
        }
    )

    auk!(
        grammar vowelq {
            start = ["aeiou"]?
        }
    )

    auk!(
        grammar abc_cat_def {
            start = "abc" "def"
        }
    )

    auk!(
        grammar bigcat {
            start = "bbc" ["aeiou"]? 'z'
        }
    )

    auk!(
        grammar bigalt {
            start = "bbc" / ["aeiou"] / 'z'
        }
    )

    println!("{}", achar("zog"));
    println!("{}", achar("wat"));
    println!("{}", achar(""));
    println!("-----------");

    println!("{}", charneg("zog"));
    println!("{}", charneg("wat"));
    println!("{}", charneg(""));
    println!("-----------");

    println!("{}", charpos("zog"));
    println!("{}", charpos("wat"));
    println!("{}", charpos(""));
    println!("-----------");

    println!("{}", astr("abcde"));
    println!("{}", astr("abde"));
    println!("{}", astr(""));
    println!("-----------");

    println!("{}", strpos("abcde"));
    println!("{}", strpos("abde"));
    println!("{}", strpos(""));
    println!("-----------");

    println!("{}", strneg("abcde"));
    println!("{}", strneg("abde"));
    println!("{}", strneg(""));
    println!("-----------");

    println!("{}", dot("abcde"));
    println!("{}", dot(""));
    println!("-----------");

    println!("{}", dotpos("abcde"));
    println!("{}", dotpos(""));
    println!("-----------");

    println!("{}", dotneg("abcde"));
    println!("{}", dotneg(""));
    println!("-----------");

    println!("{}", vowels("abc"));
    println!("{}", vowels("bbc"));
    println!("{}", vowels(""));
    println!("-----------");

    println!("{}", abcstar("abc"));
    println!("{}", abcstar("bbc"));
    println!("{}", abcstar("abcabcabcde"));
    println!("{}", abcstar(""));
    println!("-----------");

    println!("{}", vowelstar("aaaoooitieooouuuu"));
    println!("{}", vowelstar("cat"));
    println!("{}", vowelstar("adog"));
    println!("{}", vowelstar(""));
    println!("-----------");

    println!("{}", abcplus("abc"));
    println!("{}", abcplus("bbc"));
    println!("{}", abcplus("abcabcabcde"));
    println!("{}", abcplus(""));
    println!("-----------");


    println!("{}", abcq("abc"));
    println!("{}", abcq("bbc"));
    println!("{}", abcq("abcabcabcde"));
    println!("{}", abcq(""));
    println!("-----------");

    println!("{}", vowelq("aaaoooitieooouuuu"));
    println!("{}", vowelq("cat"));
    println!("{}", vowelq("adog"));
    println!("{}", vowelq(""));
    println!("-----------");

    println!("{}", abc_cat_def("abc"));
    println!("{}", abc_cat_def("bbc"));
    println!("{}", abc_cat_def("def"));
    println!("{}", abc_cat_def("abcdefgh"));
    println!("{}", abc_cat_def(""));
    println!("-----------");

    println!("{}", bigcat("abc"));
    println!("{}", bigcat("bbc"));
    println!("{}", bigcat("bbczbbc"));
    println!("{}", bigcat("bbcazbbc"));
    println!("{}", bigcat("bbcezbbc"));
    println!("{}", bigcat("bbcizbbc"));
    println!("{}", bigcat("bbcozbbc"));
    println!("{}", bigcat("bbcuzbbc"));
    println!("{}", bigcat(""));
    println!("-----------");

    println!("{}", bigalt("abc"));
    println!("{}", bigalt("bbc"));
    println!("{}", bigalt("bbczbbc"));
    println!("{}", bigalt("azbbc"));
    println!("{}", bigalt("ezbbc"));
    println!("{}", bigalt("izbbc"));
    println!("{}", bigalt("ozbbc"));
    println!("{}", bigalt("uzbbc"));
    println!("{}", bigalt("ze cat"));
    println!("{}", bigalt(""));
}
