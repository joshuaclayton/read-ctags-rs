use super::ctag_parser;
use super::language::Language;
use super::token_kind::TokenKind;
use nom::IResult;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, PartialEq)]
pub struct CtagItem<'a> {
    pub name: &'a str,
    pub file_path: &'a str,
    pub language: Option<Language>,
    pub tags: HashMap<String, String>,
    pub kind: TokenKind,
}

impl<'a> CtagItem<'a> {
    pub fn parse(input: &str) -> IResult<&str, Vec<CtagItem>> {
        ctag_parser::parse(input)
    }
}
