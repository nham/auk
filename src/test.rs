#![feature(phase)]

#[phase(plugin)] extern crate auk;
extern crate auk;

fn main() {
    auk!(
        grammar foo {
            start = 'd'
        }
    )

    println!("{}", parse_dot("dog"));
    println!("{}", parse_dot("cat"));
    println!("{}", parse_dot(""));
}
