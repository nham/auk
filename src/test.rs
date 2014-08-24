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
}
