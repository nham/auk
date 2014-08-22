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
        grammar bar {
            quux = "abc"
        }
    )

    println!("{}", parse_start("zog"));
    println!("{}", parse_start("wat"));
    println!("{}", parse_start(""));

    println!("{}", parse_quux("abcde"));
    println!("{}", parse_quux("abde"));
    println!("{}", parse_quux(""));
}
