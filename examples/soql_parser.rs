use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take},
    character::complete::{alpha1, alphanumeric1, one_of, multispace0},
    combinator::{ opt, flat_map }, 
    error::{context, ErrorKind, VerboseError},
    multi::{count, many0, many1, many_m_n},
    sequence::{preceded, separated_pair, terminated, tuple},
    AsChar, Err as NomErr, IResult, InputTakeAtPosition
};

/// https://github.com/Geal/nom/blob/main/doc/choosing_a_combinator.md

pub fn main() {}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq)]
pub struct SOQL {
    begin: OpenCloseSymbol,
    selectclause: SelectClause,
    fromclause: FromClause,
    whereclause: WhereClause
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpenCloseSymbol {
    OPEN,
    CLOSE
}

impl From<&str> for OpenCloseSymbol {
    fn from(i: &str) -> Self {
        match i {
            "[" => OpenCloseSymbol::OPEN,
            "]" => OpenCloseSymbol::CLOSE,
            _ => unimplemented!("not supported")
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SelectClause {
    SELECT
}

impl From<&str> for SelectClause {
    fn from(i: &str) -> Self {
        SelectClause::SELECT
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FromClause {
    FROM
}

#[derive(Debug, PartialEq, Eq)]
pub enum WhereClause {
    WHERE
}


pub fn openclosesymbol(input: &str) -> Res<&str, OpenCloseSymbol> {
    context("openclosesymbol", 
        alt((tag_no_case("["), tag_no_case("]"))),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn select(input: &str) -> Res<&str, SelectClause> {
    let (next_input, _) = multispace0(input)?;
    let (next_input, res) = tag_no_case("select")(next_input).map(|(next_input, res) | (next_input, res.into()))?;
    let (next_input, _) = multispace0(next_input)?;

    Ok((next_input, res))
}


#[cfg(test)]
mod tests {
    use super::*;
    use nom:: {
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    #[test]
    fn test_openclosesymbol() {
        claim::assert_ok_eq!(openclosesymbol("[SELECT"), ("SELECT", OpenCloseSymbol::OPEN));
        claim::assert_ok_eq!(openclosesymbol("]"), ("", OpenCloseSymbol::CLOSE));
        claim::assert_err!(openclosesymbol("SELECT"));
    }


    #[test]
    fn test_select() {
        claim::assert_ok_eq!(select("  \nSELECT  "), ("", SelectClause::SELECT));
        claim::assert_ok_eq!(select("sElecT"), ("", SelectClause::SELECT));
        claim::assert_err!(select("[SELECT"));
        claim::assert_ok_eq!(select("SELECT a"), ("a", SelectClause::SELECT));
    }
}