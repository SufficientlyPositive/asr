/* TOML Spec
https://github.com/toml-lang/toml/blob/1.0.0/toml.abnf
https://toml.io/en/v1.0.0#objectives
*/

use std::vec;

// ------------------------
//       Interface
// ------------------------

/**
 * A not particularly efficient TOML parser for building an object tree. Written with almost no knowledge of parsing ... :bleh:
 * Not particularly efficient because it loops through the entire file multiple times
 * Basically doesn't support anything for now :)
 */
pub fn parse_as_toml<'a>(file_contents: &str) -> Result<TOMLStump<'a>, TOMLError> {
    let root = TOMLStump { file_contents: preprocess(file_contents), branches: Vec::new() };
    let file_contents = &root.file_contents;

    let lines = file_contents.lines();
    let acting_root = &root;

    for line in lines {
        todo!("Parse lines of string. Probably pass the root node here and let it build out.")
    }

    Ok(root)
}

pub enum TOMLError {
    ParseError(&'static str),
    TreeStructError(&'static str)
}

pub enum TOMLNode<'a> {
    Branch(&'a str, Vec<TOMLNode<'a>>),
    Attribute(&'a str, TOMLType<'a>)
}

pub struct TOMLStump<'a> {
    file_contents: String,
    branches: Vec<TOMLNode<'a>>
}

pub enum TOMLType<'a> {
    String(&'a str),
    Integer(i32),
    Float(f32),
    Boolean(bool),
    OffsetDT,
    LocalDT,
    LocalDate,
    LocalTime,
    Array,
    InlineTable
}

// ------------------------
//       Implementation
// ------------------------

#[inline(always)]
fn remove_whitespace(s: &mut String) {
    s.retain(|c| !is_whitespace(c));
}

#[inline(always)]
fn is_whitespace(c: char) -> bool {
    c == '\u{09}' || // space
    c == '\u{20}'    // tab
} 

// remove whitespace, maybe do some other things, who knows...
fn preprocess(file_contents: &str) -> String {
    let mut heap_fc = String::from(file_contents);
    remove_whitespace(&mut heap_fc);
    heap_fc
}
