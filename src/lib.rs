use std::rc::Rc;
use std::fmt::{Debug, Formatter, Display};
use std::borrow::Borrow;

#[derive(Clone)]
struct IString(Rc<str>);

#[derive(Clone)]
enum TextData {
    Entire(IString),
    Slice { string: IString, start: usize, len: usize }
}

pub struct Text(TextData);

impl Clone for Text {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Debug for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = self.into();
        write!(f, "{}", s)
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = self.into();
        write!(f, "{}", s)
    }
}

impl<'a> From<&'a Text> for &'a str {
    fn from(t: &'a Text) -> Self {
        match &t.0 {
            TextData::Entire(s) => &*s.0,
            TextData::Slice { string, start, len } => {
                let s = &*string.0;
                &s[*start..*start+*len]
            }
        }
    }
}

impl Text {
    pub fn new<'a, I: Into<&'a str>>(s: I) -> Self {
        let inner = IString(Rc::from(s.into()));
        Self(TextData::Entire(inner))
    }
}