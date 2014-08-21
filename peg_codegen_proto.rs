struct ParseError {
    msg: String,
}

type ParseResult<'a> = Result<&'a str, ParseError>;

fn parse_empty<'a>(input: &'a str) -> ParseResult<'a> {
    Ok(input)
}

fn parse_dot<'a>(input: &'a str) -> ParseResult<'a> {
    if input.len() > 0 {
        let n = input.char_range_at(0).next;
        Ok(input.slice_from(n))
    } else {
        Err(format!("Could not match '.' (end of input)"))
    }
}

fn parse_a<'a>(input: &'a str) -> ParseResult<'a> {
    let x: char = 'a';
    if input.len() > 0 {
        let CharRange{ch, next} = input.char_range_at(0);
        if ch == x {
            Ok(input.slice_from(next))
        } else {
            Err(format!("Could not match '{}': (saw '{}' instead)", x, ch))
        }
    } else {
        Err(format!("Could not match '{}' (end of input)", x))
    }
}

fn parse_abc<'a>(input: &'a str) -> ParseResult<'a> {
    let x: &str = "abc";
    let n = x.len();
    let xbytes = x.as_bytes().len();

    if input.len() >= n {
        if input.starts_with(x) {
            Ok(input.slice_from(xbytes))
        } else {
            Err(format!("Could not match '{}': (saw '{}' instead)", x, input))
        }
    } else {
        Err(format!("Could not match '{}' (end of input)", x))
    }
}

// parse ab|c
fn parse_abc_or_de<'a>(input: &'a str) -> ParseResult<'a> {
    let x: &str = "abc";
    let xlen = x.len();
    let xbytes = x.as_bytes().len();

    let y: &str = "de";
    let ylen = y.len();
    let ybytes = y.as_bytes().len();

    if input.len() >= xlen {
        if input.starts_with(x) {
            return Ok(input.slice_from(xbytes));
        } else {
            Err(format!("Could not match '{}': (saw '{}' instead)", x, input))
        }
    } else if input.len() >= ylen {
        if input.starts_with(y) {
            return Ok(input.slice_from(ybytes));
        } else {
            Err(format!("Could not match '{}': (saw '{}' instead)", y, input))
        }
    } else {
        Err(format!("Could not match either '{}' or '{}' (end of input)", x, y))
    }
}

fn main() {

}
