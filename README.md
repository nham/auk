# auk

A PEG parser generator implemented via Rust syntax extensions (procedural macros).

Parsers are generated at compile time. For example:

```
auk!(
    grammar foo {
        start = ('z' / "abc" / ["aeiou"])?
    }
)

// prints "HELLO", the remaining input
println!("{}", foo::parse("zabcooooozzzHELLO"));
```
