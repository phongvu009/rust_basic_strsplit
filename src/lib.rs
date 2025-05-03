#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

//create struct
pub struct StrSplit {}

//add function
impl StrSplit{
    pub fn new(haystack: &str, delimiter: &str) -> Self{

    }
}

//implement trait
impl Iterator for StrSplit{
    //def a type
    type Item=&str;
    //return Some() or None
    fn next(&mut self) ->Option(Self::Item)
}
