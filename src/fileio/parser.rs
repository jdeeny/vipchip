use nom::{IResult, eof};


pub fn parse(input: &[u8]) -> Vec<u8> {
    let w = program(input);
    if let IResult::Done(x, result) = w {
         println!("Done: x: {:?} result: {:?}", x, result);
         result
    } else {
        match w {
            IResult::Incomplete(n) => { println!("incomplete: {:?}", n); },
            IResult::Error(e) => { println!("error: {:?}", e); }
            _ => () ,
        };
        panic!("parse error");
    }
}

named!(program<Vec<u8> >,
    chain!(
        opt!(whitespace) ~
        l: many0!(
            chain!(
                v: hexliteral ~
                alt!(whitespace | eof ),
                || v
            )
        ),
        || l
    )
);



named!(hexliteral<u8>,
    chain!(
        hexprefix ~
        v: hexbyte,
        || v
    )
);

named!(hexprefix,
    alt!(
        tag!("0x") |
        tag!("0X")
    )
);

fn hexval(c: char) -> u8{
    c.to_digit(16).unwrap() as u8
}

named!(hexbyte<u8>,
    chain!(
        hi: hexdigit ~
        lo: hexdigit,
        || (hexval(hi) << 4) | hexval(lo)
    )
);


named!(hexdigit<char>,
    one_of!("0123456789abcdefABCDEF")
);

named!(whitespace,
    map!(many1!(whitechar), |_| b"")
);

named!(whitechar<char>,
        one_of!("[],; \r\n\t")
);


#[test]
fn test_whitespace() {
    assert_eq!(whitespace(&b"   [[]]\r\n"[..]), IResult::Done(&b""[..], &b""[..]));
}

#[test]
fn test_hexliteral() {
    assert_eq!(hexliteral(&b"0x00"[..]), IResult::Done(&b""[..], 0x00 as u8));
    assert_eq!(hexliteral(&b"0x77"[..]), IResult::Done(&b""[..], 0x77 as u8));
    assert_eq!(hexliteral(&b"0xD3"[..]), IResult::Done(&b""[..], 0xD3 as u8));
    assert_eq!(hexliteral(&b"0xFF"[..]), IResult::Done(&b""[..], 0xFF as u8));
}
#[test]
fn test_program() {
    let p =  program(&b"0xC3 0x2A"[..]);
    if let IResult::Done( remainder, output ) = p {
        assert_eq!(remainder, &b""[..]);
        assert_eq!(output.as_slice(), &[0xC3, 0x2A][..]);
    } else {
        println!("{:?}", p);
        panic!("couldn't validate")
    }
}
