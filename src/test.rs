#![feature(phase)]

#[phase(plugin)] extern crate auk;
extern crate auk;

fn main() {
    let g: auk::PEGGrammar<char, String> = auk!(grammar foo {});

    println!("{}", g.parse(&auk::Empty, &['a', 'b', 'c']));
}
