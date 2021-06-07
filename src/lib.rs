use std::rc::Rc;
use std::fmt::{Debug, Formatter, Display};
use std::str::FromStr;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;
use std::ops::{Index, Range};
use std::slice::SliceIndex;

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

impl Default for Text {
    fn default() -> Self {
        let empty = IString(String::new().into());
        Self(TextData::Entire(empty))
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

impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Text {}

impl FromStr for Text {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Text::new(s))
    }
}

impl Hash for Text {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl<'a> From<&'a Text> for String {
    fn from(text: &'a Text) -> Self {
        String::from(text.as_str())
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

impl<'a> AsRef<str> for &'a Text {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> Borrow<str> for &'a Text {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<'a, Idx: SliceIndex<str>> Index<Idx> for &'a Text {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.as_str()[index]
    }
}

impl Text {
    pub fn new<'a, I: Into<&'a str>>(s: I) -> Self {
        let inner = IString(Rc::from(s.into()));
        Self(TextData::Entire(inner))
    }

    pub fn as_str(&self) -> &str {
        self.into()
    }
}