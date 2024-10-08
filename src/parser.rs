use clap::error::ErrorKind;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_until};
use nom::character::complete::{
    alphanumeric0, alphanumeric1, line_ending, multispace0, multispace1, space0, space1,
};
use nom::character::is_newline;
use nom::combinator::{opt, value};
use nom::error::{append_error, ParseError};
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, tuple};
use nom::{IResult, Parser};
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str, &'a str>;

#[derive(Debug)]
struct Function {
    name: String,
    parameters: Vec<Parameter>,
}

#[derive(Debug)]
struct Parameter {
    color: Color,
    name: String,
}

#[derive(Debug)]
struct FuncBody {}

#[derive(Debug, PartialEq)]
enum Color {
    Green,
    Red,
}

#[derive(Debug, Clone)]
struct Identifer(String);

#[derive(Debug)]
enum Statement {
    Comment,
    Empty,
    Expresion(StatementExpresion),
    Condition,
}

#[derive(Debug)]
enum StatementExpresion {
    Declaration {
        color: Color,
        name: Identifer,
        assigment: Option<Expresion>,
    },
    Assigment,
    Expresion,
}

#[derive(Debug)]
struct Assigment {
    color: Color,
    name: Identifer,
    expression: Expresion,
}

#[derive(Debug)]
enum Expresion {
    UnaryOp {
        target: Box<Expresion>,
        op: UnaryOp,
    },
    BinaryOp {
        left: Box<Expresion>,
        right: Box<Expresion>,
        op: BinaryOp,
    },
    Identifer(Identifer),
    FuncCall {
        name: Identifer,
        parameters: Vec<Expresion>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Inc,
    Dec,
    Add,
    Minus,
    Not,
    Complement,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
    Or,
    Xor,
    And,
    BitOr,
    BitXor,
    BitAnd,
    Equal,
    NonEqual,
    LT,
    GT,
    LTE,
    GTE,
    LShift,
    RShift,
    Add,
    Sub,
    Mult,
    Div,
    Mod,
}

pub struct Ast {}

pub fn parse<'a>(input: Span<'a>) -> anyhow::Result<Ast> {
    let a = parse_func(input);
    Ok(Ast {})
}

fn color_type<'a>(input: Span<'a>) -> IResult<Span<'a>, Color> {
    let (input, color) = alt((tag("red"), tag("green")))(input)?;
    let color = match color.into_fragment() {
        "red" => Color::Red,
        "green" => Color::Green,
        _ => unreachable!("Do not know this color"),
    };
    Ok((input, color))
}

fn parameter<'a>(input: Span<'a>) -> IResult<Span<'a>, Parameter> {
    let (input, (_, color, _, name, _)) =
        tuple((space0, color_type, space1, alphanumeric1, space0))(input)?;
    Ok((
        input,
        Parameter {
            color,
            name: name.into_fragment().to_string(),
        },
    ))
}

fn open_body<'a>(input: Span<'a>) -> IResult<Span<'a>, ()> {
    let (input, _) = ws(tag("{"))(input)?;
    Ok((input, ()))
}

fn close_body<'a>(input: Span<'a>) -> IResult<Span<'a>, ()> {
    let (input, _) = ws(tag("}"))(input)?;
    Ok((input, ()))
}

fn body<'a>(input: Span<'a>) -> IResult<Span<'a>, FuncBody> {
    let (input, (_, expressions, _)) = tuple((open_body, opt(func_inner_body), close_body))(input)?;
    Ok((input, FuncBody {}))
}

fn func_inner_body<'a>(input: Span<'a>) -> IResult<Span<'a>, Vec<Statement>> {
    println!("func_inner_body {}", input);
    let (input, statements) = many0(statement)(input)?;
    Ok((input, statements))
}

fn statement<'a>(input: Span<'a>) -> IResult<Span<'a>, Statement> {
    println!("statement {}", input);
    let (input, statement) = alt((empty_lines, condition, comment, expression))(input)?;
    Ok((input, statement))
}

fn empty_lines<'a>(input: Span<'a>) -> IResult<Span<'a>, Statement> {
    println!("empty_lines '{}'", input);
    let (input, _) = multispace1(input)?;
    Ok((input, Statement::Empty))
}
fn comment<'a>(input: Span<'a>) -> IResult<Span<'a>, Statement> {
    println!("comment '{}'", input);
    let inner = take_till(|x| is_newline(x as u8));
    let (input, _) = tuple((tag("//"), inner))(input)?;
    Ok((input, Statement::Comment))
}

fn expression<'a>(input: Span<'a>) -> IResult<Span<'a>, Statement> {
    println!("expression '{}'", input);
    let (input, statment_expression) =
        alt((parse_assigment, parse_declaration, parse_expression))(input)?;
    Ok((input, Statement::Expresion(statment_expression)))
}

fn parse_assigment<'a>(input: Span<'a>) -> IResult<Span<'a>, StatementExpresion> {
    todo!("sda");
}

fn parse_declaration<'a>(input: Span<'a>) -> IResult<Span<'a>, StatementExpresion> {
    todo!("sda");
}

fn parse_expression<'a>(input: Span<'a>) -> IResult<Span<'a>, StatementExpresion> {
    todo!("sda");
}

//todo: add declaration

fn condition<'a>(input: Span<'a>) -> IResult<Span<'a>, Statement> {
    let (input, _) = tuple((
        tag("if"),
        expression,
        open_body,
        func_inner_body,
        close_body,
    ))(input)?;
    Ok((input, Statement::Condition))
}

fn parameters<'a>(input: Span<'a>) -> IResult<Span<'a>, Vec<Parameter>> {
    let (input, (_, params, _)) =
        tuple((tag("("), separated_list0(tag(","), parameter), tag(")")))(input)?;
    Ok((input, params))
}

fn parse_func<'a>(input: Span<'a>) -> IResult<Span<'a>, Function> {
    let (input, (_, name, parameters, body)) =
        tuple((tag("fn"), ws(alphanumeric0), ws(parameters), ws(body)))(input)?;
    Ok((
        input,
        Function {
            name: name.fragment().to_string(),
            parameters: Vec::new(),
        },
    ))
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F: 'a, O, E: ParseError<Span<'a>>>(
    inner: F,
) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O, E>
where
    F: Fn(Span<'a>) -> IResult<Span<'a>, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let input = Span::new_extra("", "file");
        let ast = parse(input).unwrap();
    }

    #[test]
    fn empty_fn() {
        let input = Span::new_extra("fn empty() {}", "file");
        let (input, func) = parse_func(input).unwrap();
        assert_eq!(func.name, "empty");
        assert!(input.is_empty());
    }

    #[test]
    fn empty_fn_parameter() {
        let input = Span::new_extra("fn empty(red in1) { }", "file");
        let (input, func) = parse_func(input).unwrap();
        assert_eq!(func.name, "empty");
        assert!(input.is_empty());

        let input = Span::new_extra("fn empty(red in1) {}", "file");
        let (input, func) = parse_func(input).unwrap();
        assert_eq!(func.name, "empty");
        assert!(input.is_empty());
    }

    #[test]
    fn parameters_test() {
        let input = Span::new_extra("()", "file");
        let (input, params) = parameters(input).unwrap();
        assert!(params.is_empty());
        assert!(input.is_empty());

        let input = Span::new_extra("(red in1)", "file");
        let (input, params) = parameters(input).unwrap();
        assert_eq!(params.len(), 1);
        assert!(input.is_empty());
        assert_eq!(params[0].color, Color::Red);
        assert_eq!(params[0].name.as_str(), "in1");

        let input = Span::new_extra("(red in1, green in2)", "file");
        let (input, params) = parameters(input).unwrap();
        assert_eq!(params.len(), 2);
        assert!(input.is_empty());
        assert_eq!(params[0].color, Color::Red);
        assert_eq!(params[0].name.as_str(), "in1");
        assert_eq!(params[1].color, Color::Green);
        assert_eq!(params[1].name.as_str(), "in2");
    }

    #[test]
    fn color_type_test() {
        let input = Span::new_extra("green", "file");
        let (input, color) = color_type(input).unwrap();
        assert!(matches!(color, Color::Green));
        assert!(input.is_empty());
        let input = Span::new_extra("red", "file");
        let (input, color) = color_type(input).unwrap();
        assert!(matches!(color, Color::Red));
        assert!(input.is_empty());
        let input = Span::new_extra("   runkown", "file");
        let ret = color_type(input).unwrap_err();
    }

    #[test]
    fn test_func_body() {
        let input = Span::new_extra("{}", "file");
        let (input, stms) = body(input).unwrap();
        // assert_eq!(stms), 0);
        assert!(input.is_empty());
    }

    #[test]
    fn test_func_inner_body() {
        let input = Span::new_extra("", "file");
        func_inner_body(input).unwrap_err();
    }

    #[test]
    fn test_many0() {
        fn parser<'a>(s: Span<'a>) -> IResult<Span<'a>, Vec<Span>> {
            many0(tag("abc"))(s)
        }
        let input = Span::new_extra("abcabc", "file");

        let (input, stms) = parser(input).unwrap();

        let input = Span::new_extra("", "file");

        let (input, stms) = parser(input).unwrap();
    }
}
