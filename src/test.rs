#![feature(phase)]

#[phase(plugin)] extern crate auk;
extern crate auk;

fn main() {
    auk!(
        grammar foo {
            start = 'z'
        }
    )

    auk!(
        grammar foo {
            zuh = !'z'
        }
    )

    auk!(
        grammar foo {
            buh = &'z'
        }
    )

    auk!(
        grammar bar {
            quux = "abc"
        }
    )

    auk!(
        grammar bar {
            quux2 = &"abc"
        }
    )

    auk!(
        grammar bar {
            quux3 = !"abc"
        }
    )

    auk!(
        grammar bar {
            dot = .
        }
    )

    auk!(
        grammar bar {
            posdot = &.
        }
    )

    auk!(
        grammar bar {
            negdot = !.
        }
    )

    auk!(
        grammar bar {
            vowels = ["aeiou"]
        }
    )

    println!("{}", parse_start("zog"));
    println!("{}", parse_start("wat"));
    println!("{}", parse_start(""));
    println!("-----------");

    println!("{}", parse_zuh("zog"));
    println!("{}", parse_zuh("wat"));
    println!("{}", parse_zuh(""));
    println!("-----------");

    println!("{}", parse_buh("zog"));
    println!("{}", parse_buh("wat"));
    println!("{}", parse_buh(""));
    println!("-----------");

    println!("{}", parse_quux("abcde"));
    println!("{}", parse_quux("abde"));
    println!("{}", parse_quux(""));
    println!("-----------");

    println!("{}", parse_quux2("abcde"));
    println!("{}", parse_quux2("abde"));
    println!("{}", parse_quux2(""));
    println!("-----------");

    println!("{}", parse_quux3("abcde"));
    println!("{}", parse_quux3("abde"));
    println!("{}", parse_quux3(""));
    println!("-----------");

    println!("{}", parse_dot("abcde"));
    println!("{}", parse_dot(""));
    println!("-----------");

    println!("{}", parse_posdot("abcde"));
    println!("{}", parse_posdot(""));
    println!("-----------");

    println!("{}", parse_negdot("abcde"));
    println!("{}", parse_negdot(""));

    println!("{}", parse_vowels("abc"));
    println!("{}", parse_vowels("bbc"));
    println!("{}", parse_vowels(""));
}
