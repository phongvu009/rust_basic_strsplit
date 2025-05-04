//#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

//create struct
pub struct StrSplit<'a> {
    remainder: &'a str, // create variable in stack, &str reference to a string slice - borrowed
    delimiter: &'a str, // String::from() creates string literal - stored in read-only memory
}

//add function
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        //return Struct itself
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

//implement trait
impl<'a> Iterator for StrSplit<'a> {
    //def a type
    type Item = &'a str;
    //return Some() or None
    fn next(&mut self) -> Option<Self::Item> {
        //find delimiter
        if let Some(idx_next_delim) = self.remainder.find(self.delimiter) {
            //&self.remainder[..next_dlim] borrowed
            let until_delimiter = &self.remainder[..idx_next_delim];
            //create ref - bss of the left portion
            self.remainder = &self.remainder[(idx_next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            None
        } else {
            let rest = self.remainder;
            //assign longer lifetime &'static str instead of &'a str
            self.remainder = "";
            Some(rest)
        }
    }
}

#[test]
fn test_fn() {
    let haystack = "a b c d";
    let letters = StrSplit::new(haystack, " ");
    //make comparision
    assert!(letters.eq(vec!["a", "b", "c", "d"].into_iter()));
}
