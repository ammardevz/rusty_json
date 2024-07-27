use std::convert::Infallible;
use indexmap::IndexMap;
use nom::{branch::alt, bytes::complete::tag, character::complete::{char, multispace0}, combinator::map, Finish, IResult, multi::separated_list0, number::complete::double, sequence::{delimited, separated_pair}};
use nom::combinator::value;
use nom::error::Error as NomError;
use thiserror::Error;
use crate::base::casting::CastError;

use crate::base::JsonValue;

pub struct JsonParser;

/// Error type representing various conversion errors during JSON parsing.
#[derive(Debug, Error)]
pub enum ConversationError {
    /// Parsing error with a specific message.
    #[error("Parsing error: {0}")]
    ParsingError(String),

    /// Generic conversation error with a specific message.
    #[error("Conversation error: {0}")]
    GenericError(String),

    /// Conversion error from CastError.
    #[error(transparent)]
    Cast(#[from] CastError),

    /// Conversion error from Infallible.
    #[error(transparent)]
    Infallible(#[from] Infallible),
}

impl From<NomError<&str>> for ConversationError {
    fn from(error: NomError<&str>) -> Self {
        ConversationError::ParsingError(error.to_string())
    }
}


impl JsonParser {
    /// Parses a JSON string into a `JsonValue`.
    ///
    /// # Arguments
    ///
    /// * `json_str` - A JSON string to parse.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `JsonValue` if parsing is successful, otherwise a `ConversationError`.
    pub fn parse(json_str: &str) -> Result<JsonValue, ConversationError> {
        // Attempt to parse the JSON string into a `JsonValue`
        let (remaining, result) = parse_json_value(json_str).finish().map_err(|e| ConversationError::ParsingError(e.to_string()))?;

        // Check if there are any remaining unparsed characters
        if !remaining.trim().is_empty() {
            // If there are remaining characters, return an error indicating incomplete parsing
            return Err(ConversationError::ParsingError(format!(
                "Incomplete parsing: {} characters were left unparsed. Remaining input: '{}'",
                remaining.len(),
                &remaining[..std::cmp::min(20, remaining.len())]
            )));
        }

        // Return the successfully parsed `JsonValue`
        Ok(result)
    }
}



type JsonObject = IndexMap<String, JsonValue>;
type JsonArray = Vec<JsonValue>;

fn ws<'a, F: 'a, O, E>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        F: FnMut(&'a str) -> IResult<&'a str, O, E>,
        E: nom::error::ParseError<&'a str>,
{
    delimited(multispace0, inner, multispace0)
}

fn parse_str(input: &str) -> IResult<&str, &str> {
    delimited(
        char('"'),
        nom::bytes::complete::take_while(|c| c != '"'),
        char('"')
    )(input)
}

fn parse_bool(input: &str) -> IResult<&str, bool> {
    alt((
        value(true, tag("true")),
        value(false, tag("false")),
    ))(input)
}

fn parse_null(input: &str) -> IResult<&str, ()> {
    value((), tag("null"))(input)
}

fn parse_array(input: &str) -> IResult<&str, JsonArray> {
    delimited(
        ws(char('[')),
        separated_list0(
            ws(char(',')),
            parse_json_value
        ),
        ws(char(']'))
    )(input)
}

fn parse_object(input: &str) -> IResult<&str, JsonObject> {
    map(
        delimited(
            ws(char('{')),
            separated_list0(
                ws(char(',')),
                separated_pair(
                    parse_str,
                    ws(char(':')),
                    parse_json_value
                )
            ),
            ws(char('}'))
        ),
        |pairs| pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
    )(input)
}

fn parse_json_value(input: &str) -> IResult<&str, JsonValue> {
    ws(alt((
        map(parse_str, |s| JsonValue::String(s.to_string())),
        map(double, JsonValue::Number),
        map(parse_bool, JsonValue::Boolean),
        map(parse_null, |_| JsonValue::Null),
        map(parse_array, |v| JsonValue::from(v)),
        map(parse_object, |v| JsonValue::from(v)),
    )))(input)
}
