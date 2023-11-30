use nom::{
    bytes::complete::take_while1,
    character::complete::multispace0,
    multi::many0,
    sequence::{delimited, tuple},
    *,
};

/// Parses any non whitespace character from the input
pub fn non_space(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_whitespace())(input)
}

/// Parses the command parameters from the input
pub fn cmd_param_parser(input: &str) -> IResult<&str, &str> {
    delimited(multispace0, non_space, multispace0)(input)
}

/// Parses the command arguments from the input
pub fn cmd_arg_parser(input: &str) -> IResult<&str, Vec<&str>> {
    many0(cmd_param_parser)(input)
}

/// Parses the command name from the input
pub fn cmd_name_parser(input: &str) -> IResult<&str, &str> {
    cmd_param_parser(input)
}

/// Parses the command from the input
pub fn cmd_parser(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    tuple((cmd_name_parser, cmd_arg_parser))(input)
}
