auk!(
    grammar simple_arith {
        expr = x:(addexpr / subexpr) -> { x },
        addexpr = x:num '+' y:num -> { x + y },
        subexpr = x:num '-' y:num -> { x - y },
        num = x:('-'? digit+) -> { from_str::<int>(x).unwrap() }
    }
)
