#![feature(phase)]

#[phase(plugin)] extern crate auk;
extern crate auk;

fn main() {
    auk!(
        grammar foo {
            start = 'd'
        }
    )

    println!("{}", parse_char("dog"));
    println!("{}", parse_char("cat"));
    println!("{}", parse_char(""));
}
