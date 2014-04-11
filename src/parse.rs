use std::fmt::Show;
use intern;
use std::uint;
use std::str;

//////////////////////////////////////////////////////////////////////////////
// Simple parser combinator interface

/// A parser that, given a grammar def'n of type `G`, produces data of
/// type `T`. The grammar is used to create cyclic productions, see
/// the `Ref` parser and `grammar::Kind` for an example.
pub trait Parse<G,T> {
    fn parse(&self,
             grammar: &G,
             input: &[u8],
             start: uint)
             -> ParseError<(uint, T)>;
}

pub type ParseError<T> = Result<T, uint>;

pub type Parser<G,T> = ~Parse<G,T>:'static;

pub fn obj<G,T,R:'static+Parse<G,T>>(r: R) -> Parser<G,T> {
    ~r as Parser<G,T>
}

//////////////////////////////////////////////////////////////////////////////

pub fn is_digit(c: char) -> bool {
    match c {
        '0' .. '9' => true,
        _ => false
    }
}

pub fn is_type_start(c: char) -> bool {
    match c {
        'A' .. 'Z' => true,
        _ => false
    }
}

pub fn is_ident_start(c: char) -> bool {
    match c {
        'a' .. 'z' | '_' => true,
        _ => false
    }
}

pub fn is_not_ident_start(c: char) -> bool {
    !is_ident_start(c)
}

pub fn is_ident_cont(c: char) -> bool {
    match c {
        '0' .. '9' | 'a' .. 'z' | 'A' .. 'Z' | '_' => true,
        _ => false
    }
}

pub fn is_not_ident_cont(c: char) -> bool {
    !is_ident_cont(c)
}

pub fn is_oper(c: char) -> bool {
    match c {
        '+' | '-' | '>' | '<' | '*' | '/' => true,
        _ => false
    }
}

pub fn is_any(_: char) -> bool {
    true
}

pub fn is_not_oper(c: char) -> bool {
    !is_oper(c)
}

pub fn is_whitespace(c: char) -> bool {
    match c {
        ' ' | '\t' | '\n' | '\r' => true,
        _ => false,
    }
}

fn accumulate(buf: &mut ~str,
              input: &[u8],
              start: uint,
              test: |char| -> bool)
              -> uint {
    let mut i = start;
    while i < input.len() {
        let c = input[i] as char;
        if test(c) {
            buf.push_char(c);
            i += 1;
        } else {
            break;
        }
    }
    return i;
}

fn skip_whitespace(input: &[u8], start: uint) -> uint {
    let mut i = start;
    while i < input.len() && is_whitespace(input[i] as char) {
        i += 1;
    }
    i
}

//////////////////////////////////////////////////////////////////////////////

struct Nothing1;

fn Nothing<G>() -> Parser<G,()> {
    obj(Nothing1)
}

impl<G> Parse<G,()> for Nothing1 {
    fn parse(&self,
             _: &G,
             _: &[u8],
             start: uint)
             -> ParseError<(uint, ())> {
        Ok((start, ()))
    }
}

//////////////////////////////////////////////////////////////////////////////

type RefFn<G,T> = extern "Rust" fn<'a>(g: &'a G) -> &'a Parser<G,T>;

struct Ref<G,T> {
    func: RefFn<G,T>
}

pub fn Ref<G,T>(func: RefFn<G,T>) -> Parser<G,T> {
    obj(Ref {func: func})
}

impl<G,T> Parse<G,T> for Ref<G,T> {
    fn parse(&self,
             grammar: &G,
             input: &[u8],
             start: uint)
             -> ParseError<(uint, T)> {
        (self.func)(grammar).parse(grammar, input, start)
    }
}

//////////////////////////////////////////////////////////////////////////////

struct Token {
    str: &'static str,
    term: Fn<char, bool>
}

pub fn Token<G>(s: &'static str, term: Fn<char, bool>) -> Parser<G,()> {
    obj(Token { str: s, term: term })
}

pub fn Comma<G>() -> Parser<G,()> {
    Token(",", is_any)
}

pub fn Star<G>() -> Parser<G,()> {
    Token("*", is_not_oper)
}

pub fn Eq<G>() -> Parser<G,()> {
    Token("=", is_not_oper)
}

pub fn Arrow<G>() -> Parser<G,()> {
    Token("->", is_not_oper)
}

pub fn FatArrow<G>() -> Parser<G,()> {
    Token("=>", is_not_oper)
}

pub fn ColonColon<G>() -> Parser<G,()> {
    Token("::", is_not_oper)
}

pub fn Semi<G>() -> Parser<G,()> {
    Token(";", is_any)
}

pub fn Lparen<G>() -> Parser<G,()> {
    Token("(", is_any)
}

pub fn Rparen<G>() -> Parser<G,()> {
    Token(")", is_any)
}

pub fn Lbrace<G>() -> Parser<G,()> {
    Token("{", is_any)
}

pub fn Rbrace<G>() -> Parser<G,()> {
    Token("}", is_any)
}

impl<G> Parse<G,()> for Token {
    fn parse(&self,
             _: &G,
             input: &[u8],
             start: uint)
             -> Result<(uint, ()), uint> {
        let start = skip_whitespace(input, start);
        let bytes = self.str.as_bytes();
        let end = start + bytes.len();

        if end > input.len() {
            return Err(start);
        }

        if input.slice(start, end) != bytes {
            return Err(start);
        }

        if end == input.len() || (self.term)(input[end] as char) {
            return Ok((end, ()));
        }

        return Err(start);
    }
}

//////////////////////////////////////////////////////////////////////////////

struct Integer1;

pub fn Integer<G>() -> Parser<G,uint> {
    obj(Integer1)
}

impl<G> Parse<G,uint> for Integer1 {
    fn parse(&self,
             _: &G,
             input: &[u8],
             start: uint)
             -> Result<(uint, uint), uint> {
        let start = skip_whitespace(input, start);
        let mut i = start;
        let mut r: uint = 0;
        while i < input.len() {
            if is_digit(input[i] as char) {
                r = r * 10 + (input[i] as uint) - ('0' as uint);
                i += 1;
            } else {
                break;
            }
        }

        if i == start {
            Err(start)
        } else {
            Ok((i, r))
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

struct TypeName1;

pub fn TypeName<G>() -> Parser<G,intern::Id> {
    obj(TypeName1)
}

impl<G> Parse<G,intern::Id> for TypeName1 {
    fn parse(&self,
             _: &G,
             input: &[u8],
             start: uint)
             -> ParseError<(uint, intern::Id)> {
        let mut buf = ~"";
        let start = skip_whitespace(input, start);

        if start == input.len() {
            return Err(start);
        }

        let c0 = input[start] as char;
        if !is_type_start(c0) {
            return Err(start);
        }

        buf.push_char(c0);
        let end = accumulate(&mut buf, input, start+1, is_ident_cont);
        let id = intern::intern(buf);
        Ok((end, id))
    }
}

//////////////////////////////////////////////////////////////////////////////

struct Ident1;

pub fn Ident<G>() -> Parser<G,intern::Id> {
    obj(Ident1)
}

impl<G> Parse<G,intern::Id> for Ident1 {
    fn parse(&self,
             _: &G,
             input: &[u8],
             start: uint)
             -> ParseError<(uint, intern::Id)> {
        let mut buf = ~"";
        let start = skip_whitespace(input, start);

        if start == input.len() {
            return Err(start);
        }

        let c0 = input[start] as char;
        let end = if is_ident_start(c0) {
            buf.push_char(c0);
            accumulate(&mut buf, input, start+1, is_ident_cont)
        } else if c0 == '(' {
            buf.push_char('(');
            let end = accumulate(&mut buf, input, start+1, is_oper);
            if end < input.len() && input[end] as char == ')' {
                buf.push_char(')');
                end + 1
            } else {
                start
            }
        } else {
            start
        };

        if start == end {
            Err(start)
        } else {
            let id = intern::intern(buf);
            Ok((end, id))
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Repeat: Parse over and over with an optional separator.

pub struct Repeat<G,T> {
    sub: Parser<G,T>,
    min: uint,
    sep: Option<Parser<G,()>>
}

pub fn Repeat<G,T>(sub: Parser<G,T>,
                   min: uint,
                   sep: Option<Parser<G,()>>) -> Parser<G,~[T]> {
    obj(Repeat { sub: sub, min: min, sep: sep })
}

impl<G,T> Parse<G,~[T]> for Repeat<G,T> {
    fn parse(&self,
             grammar: &G,
             input: &[u8],
             start: uint)
             -> ParseError<(uint, ~[T])> {
        let mut pos = start;
        let mut err_pos;
        let mut result = ~[];
        loop {
            match self.sub.parse(grammar, input, pos) {
                Ok((end, v)) => {
                    result.push(v);
                    pos = end;
                }
                Err(end) => {
                    err_pos = end;
                    break;
                }
            }

            match self.sep {
                Some(ref sep) => {
                    match sep.parse(grammar, input, pos) {
                        Ok((end, ())) => {
                            pos = end;
                        }
                        Err(end) => {
                            err_pos = end;
                            break; // no separator, list is complete.
                        }
                    }
                }
                None => { }
            }
        }

        if result.len() >= self.min {
            return Ok((pos, result));
        } else {
            return Err(err_pos);
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Optional: Parse at most once.

pub struct Optional<G,T> {
    sub: Parser<G,T>,
}

pub fn Optional<G,T>(sub: Parser<G,T>) -> Parser<G,Option<T>> {
    obj(Optional { sub: sub })
}

impl<G,T> Parse<G,Option<T>> for Optional<G,T> {
    fn parse(&self,
             grammar: &G,
             input: &[u8],
             start: uint)
             -> ParseError<(uint, Option<T>)> {
        match self.sub.parse(grammar, input, start) {
            Ok((end, v)) => {
                Ok((end, Some(v)))
            }
            Err(_) => {
                Ok((start, None))
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Map: Parse then apply a (bare) fn.

type Fn<T,U> = extern "Rust" fn(T) -> U;

pub struct Map<G,T,U> {
    sub: Parser<G,T>,
    f: Fn<T,U>
}

pub fn Map<G,T,U>(sub: Parser<G,T>, f: Fn<T,U>) -> Parser<G,U> {
    obj(Map { sub: sub, f: f })
}

impl<G,T,U> Parse<G,U> for Map<G,T,U> {
    fn parse(&self,
             grammar: &G,
             input: &[u8],
             start: uint)
             -> ParseError<(uint, U)> {
        match self.sub.parse(grammar, input, start) {
            Ok((end, t)) => {
                let u = (self.f)(t);
                Ok((end, u))
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Choice: Tries many parsers in turn.

pub struct Choice<G,T> {
    choices: ~[Parser<G,T>]
}

pub fn Choice<G,T>(choices: ~[Parser<G,T>]) -> Parser<G,T> {
    obj(Choice { choices: choices })
}

pub fn Choice2<G,T>(left: Parser<G,T>, right: Parser<G,T>) -> Parser<G,T> {
    Choice(~[left, right])
}

impl<G,T> Parse<G,T> for Choice<G,T> {
    fn parse(&self,
             grammar: &G,
             input: &[u8],
             start: uint)
             -> ParseError<(uint, T)> {
        let mut farthest = start;
        for choice in self.choices.iter() {
            match choice.parse(grammar, input, start) {
                Ok((end, v)) => {
                    return Ok((end, v));
                }
                Err(e) => {
                    farthest = if e > farthest {e} else {farthest};
                }
            }
        }
        return Err(farthest);
    }
}

//////////////////////////////////////////////////////////////////////////////
// Tuple: Parses with `first` then `second`, returning tuple of results.

pub struct Tuple<G,T,U> {
    first: Parser<G,T>,
    second: Parser<G,U>
}

pub fn Tuple<G,T,U>(first: Parser<G,T>, second: Parser<G,U>) -> Parser<G,(T,U)> {
    obj(Tuple { first: first, second: second })
}

impl<G,T,U> Parse<G,(T,U)> for Tuple<G,T,U> {
    fn parse(&self,
             grammar: &G,
             input: &[u8],
             start: uint)
             -> ParseError<(uint, (T,U))> {
        match self.first.parse(grammar, input, start) {
            Ok((pos, a)) => {
                match self.second.parse(grammar, input, pos) {
                    Ok((end, b)) => {
                        Ok((end, (a,b)))
                    }
                    Err(e) => {
                        Err(e)
                    }
                }
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// PostPredicate: Parses with `first` then tests whether `second` can parse,
// but result of `second` is dropped (and `second` does not consume text).

pub struct PostPredicate<G,T,U> {
    first: Parser<G,T>,
    second: Parser<G,U>
}

pub fn PostPredicate<G,T,U>(first: Parser<G,T>,
                            second: Parser<G,U>)
                            -> Parser<G,T> {
    obj(PostPredicate { first: first, second: second })
}

impl<G,T,U> Parse<G,T> for PostPredicate<G,T,U> {
    fn parse(&self,
             grammar: &G,
             input: &[u8],
             start: uint)
             -> ParseError<(uint, T)> {
        match self.first.parse(grammar, input, start) {
            Ok((pos, a)) => {
                match self.second.parse(grammar, input, start) {
                    Ok((_, _)) => {
                        Ok((pos, a))
                    }
                    Err(e) => {
                        Err(e)
                    }
                }
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Not: Succeeds if `test` fails. Consumes nothing.

pub struct Not<G,T> {
    test: Parser<G,T>,
}

pub fn Not<G,T>(test: Parser<G,T>) -> Parser<G,()> {
    obj(Not { test: test })
}

impl<G,T> Parse<G,()> for Not<G,T> {
    fn parse(&self,
             grammar: &G,
             input: &[u8],
             start: uint)
             -> ParseError<(uint, ())> {
        match self.test.parse(grammar, input, start) {
            Ok((pos, _)) => {
                Err(pos)
            }
            Err(_) => {
                Ok((start, ()))
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Not: Succeeds if `test` fails. Consumes nothing.

pub struct Debug<G,T> {
    tag: ~str,
    sub: Parser<G,T>,
}

pub fn Debug<G,T>(tag: ~str, sub: Parser<G,T>) -> Parser<G,T> {
    obj(Debug { tag: tag, sub: sub })
}

impl<G,T> Parse<G,T> for Debug<G,T> {
    fn parse(&self,
             grammar: &G,
             input: &[u8],
             start: uint)
             -> ParseError<(uint, T)> {
        match self.sub.parse(grammar, input, start) {
            Ok((pos, v)) => {
                Ok((pos, v))
            }
            Err(pos) => {
                Err(pos)
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Convenient methods for constructing grammars

fn first<T,U>((x, _): (T,U)) -> T { x }
fn second<T,U>((_, x): (T,U)) -> U { x }

pub trait Convenience<G,T> {
    fn rep(self, min: uint) -> Parser<G,~[T]>;
    fn rep_sep(self, min: uint, sep: Parser<G,()>) -> Parser<G,~[T]>;
    fn opt(self) -> Parser<G,Option<T>>;
    fn then<U>(self, u: Parser<G,U>) -> Parser<G,(T,U)>;
    fn thenl<U>(self, u: Parser<G,U>) -> Parser<G,T>;
    fn thenr<U>(self, u: Parser<G,U>) -> Parser<G,U>;
    fn map<U>(self, f: Fn<T,U>) -> Parser<G,U>;
    fn test<U>(self, p: Parser<G,U>) -> Parser<G,T>;
    fn not(self) -> Parser<G,()>;
    fn debug(self, tag: ~str) -> Parser<G,T>;
}

impl<G,T> Convenience<G,T> for Parser<G,T> {
    /// Repeat `self` at least `min` times
    fn rep(self, min: uint) -> Parser<G,~[T]> {
        Repeat(self, min, None)
    }

    /// Repeat `self` at least `min` times with separator `sep`
    fn rep_sep(self, min: uint, sep: Parser<G,()>) -> Parser<G,~[T]> {
        Repeat(self, min, Some(sep))
    }

    /// At most once.
    fn opt(self) -> Parser<G,Option<T>> {
        Optional(self)
    }

    /// Parse `self` then `u`, keeping both sides
    fn then<U>(self, u: Parser<G,U>) -> Parser<G,(T,U)> {
        Tuple(self, u)
    }

    /// Parse `self` then `u`, keeping left
    fn thenl<U>(self, u: Parser<G,U>) -> Parser<G,T> {
        Tuple(self, u).map(first)
    }

    /// Parse `self` then `u`, keeping right
    fn thenr<U>(self, u: Parser<G,U>) -> Parser<G,U> {
        Tuple(self, u).map(second)
    }

    /// Map result
    fn map<U>(self, f: Fn<T,U>) -> Parser<G,U> {
        Map(self, f)
    }

    /// `self` but only if `p` can be parsed
    fn test<U>(self, p: Parser<G,U>) -> Parser<G,T> {
        PostPredicate(self, p)
    }

    /// Parse succeeds if `self` produces an error.
    /// Typically used in combination with test:
    ///   foo.test(bar.not())
    fn not(self) -> Parser<G,()> {
        Not(self)
    }

    /// Print some debug output after parse is attempted
    fn debug(self, tag: ~str) -> Parser<G,T> {
        Debug(tag, self)
    }
}

//////////////////////////////////////////////////////////////////////////////

pub fn parse<G,T>(grammar: &G,
                  text: &[u8],
                  parser: &Parser<G,T>)
                  -> ParseError<T> {
    match parser.parse(grammar, text, 0) {
        Err(e) => Err(e),
        Ok((end, v)) => {
            let end1 = skip_whitespace(text, end);
            if end1 != text.len() {
                Err(end)
            } else {
                Ok(v)
            }
        }
    }
}

pub fn parse_or_fail<G,T>(grammar: &G,
                          text: &[u8],
                          parser: &Parser<G,T>)
                          -> T {
    match parse(grammar, text, parser) {
        Ok(v) => v,
        Err(pos) => {
            fail!(format!("Parse error: \"{}\" (here) \"{}\"",
                          str::from_utf8(text.slice_to(pos)),
                          str::from_utf8(text.slice_from(pos))));
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
pub fn test<G,T:Show>(grammar: G,
                      text: &'static str,
                      parser: &Parser<G,T>,
                      expected: &'static str) {
    let bytes = text.as_bytes();
    intern::install(|| {
        match parse(&grammar, bytes, parser) {
            Err(idx) => {
                fail!(format!("Parse error at index {}", idx))
            }
            Ok(v) => {
                let description = format!("{}", v);
                assert_eq!(description.slice_from(0), expected);
            }
        }
    })
}

#[cfg(test)]
pub fn test_err<G,T:Show>(grammar: G,
                          text: &'static str,
                          parser: &Parser<G,T>,
                          expected: uint) {
    let bytes = text.as_bytes();
    intern::install(|| {
        match parse(&grammar, bytes, parser) {
            Err(index) => {
                assert_eq!(index, expected);
            }
            Ok(v) => {
                let description = format!("{}", v);
                fail!(format!("Parse succeeded with {}", description));
            }
        }
    })
}

#[test]
fn idents() {
    let parser = Ident().rep(1);
    test((), " hello    world", &parser, "[hello, world]");
    test_err((), "", &parser, 0);
    test_err((), "h 1", &parser, 1);
}

#[test]
fn digits() {
    let parser = Integer().rep(1);
    test((), " 12 24   36", &parser, "[12, 24, 36]");
    test_err((), "", &parser, 0);
    test_err((), "1 h", &parser, 1);
}

#[test]
fn idents_or_digits() {
    #[deriving(Show)]
    enum Choice { IsIdent(intern::Id), IsNumber(uint) }

    let parser =
        Choice(~[
            Integer().map(IsNumber),
            Ident().map(IsIdent)]).rep(1);
    test((), " 12 24 hi  36", &parser, "[IsNumber(12), IsNumber(24), IsIdent(hi), IsNumber(36)]");
    test_err((), "--", &parser, 0);
}

#[test]
fn arrows() {
    let parser = Arrow().rep(1);
    test((), "-> ->  ->", &parser, "[(), (), ()]");
}

#[test]
fn stars() {
    let parser = Star().rep(1);
    test((), "* *", &parser, "[(), ()]");
}

#[test]
fn star_arrow() {
    let parser = Star().then(Arrow().thenr(Star()).rep(1));
    test((), "* -> * -> *", &parser, "((), [(), ()])");
}

