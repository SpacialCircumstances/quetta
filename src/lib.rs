use std::rc::Rc;

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

impl Text {
    pub fn new<'a, I: Into<&'a str>>(s: I) -> Self {
        let inner = IString(Rc::from(s.into()));
        Self(TextData::Entire(inner))
    }
}