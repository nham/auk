#![feature(phase)]

#[phase(plugin)] extern crate auk;
extern crate auk;

fn main() {
    let g: auk::PEGGrammar<char, String> =
        auk!(
            grammar foo {
                start = 'd'
            });

    println!("{}", g("dog"));
    println!("{}", g("cat"));
}
