//! **quetta** (from the Quenya word for "word") is a library providing simple
//! immutable strings in Rust.
//! Essentially, it is a wrapper around `Rc<str>`, but with support for slicing and compatibility features
//! with `&str`.
//!
//! The primary type provided by **quetta** is [`Text`].
//! [`Text`] can be either a full string or a slice from another [`Text`], but this is of no concern to the user.
//! [`Text`] is immutable and can be cloned very cheaply.
//!
//! # Example
//! ```
//! use quetta::Text;
//!
//! let t = Text::new("a.b.c");
//! let s1 = t.slice(0, 2);
//! assert_eq!("a.", s1.as_str());
//! ```
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Index;
use std::rc::Rc;
use std::slice::SliceIndex;
use std::str::FromStr;

#[derive(Clone)]
struct IString(Rc<str>);

#[derive(Clone)]
enum TextData {
    Entire(IString),
    Slice {
        string: IString,
        start: usize,
        len: usize,
    },
}

/// The primary type of **quetta**, representing an immutable sequence of characters.
/// Internally, this can be either a full string or a slice into another [`Text`].
/// Can be cloned cheaply.
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

impl PartialOrd for Text {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl Ord for Text {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
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
                &s[*start..*start + *len]
            }
        }
    }
}

impl<'a> From<&'a str> for Text {
    fn from(t: &'a str) -> Self {
        Text::new(t)
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

impl<'a, Idx: SliceIndex<str>> Index<Idx> for Text {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.as_str()[index]
    }
}

impl Text {
    /// Creates a new [`Text`] by copying the provided slice.
    pub fn new<'a, I: Into<&'a str>>(s: I) -> Self {
        let inner = IString(Rc::from(s.into()));
        Self(TextData::Entire(inner))
    }

    /// Gets the [`Text`] as a slice.
    pub fn as_str(&self) -> &str {
        self.into()
    }

    /// Creates another [`Text`] with a provided start code point and length.
    /// Will panic if the substring exceeds the [`Text`]'s bounds.
    ///
    /// # Example
    /// ```
    /// use quetta::Text;
    ///
    /// let text = Text::new("qwerty");
    /// let sub = text.substring(0, 2);
    /// assert_eq!("qw", sub.as_str());
    /// ```
    pub fn substring(&self, start: usize, len: usize) -> Text {
        if start + len > self.len() {
            panic!("Slice index out of bounds: Length of string is {}, but slice start was {} and slice length was {}", self.len(), start, len)
        }
        match &self.0 {
            TextData::Entire(s) => Self(TextData::Slice {
                string: s.clone(),
                start,
                len,
            }),
            TextData::Slice {
                string,
                start: s2,
                len: _,
            } => Self(TextData::Slice {
                string: string.clone(),
                start: s2 + start,
                len,
            }),
        }
    }

    /// Creates another [`Text`] with a provided start code point and end code point, similar to the slice operator.
    /// Will panic if the slice exceeds the [`Text`]'s bounds.
    ///
    /// # Example
    /// ```
    /// use quetta::Text;
    ///
    /// let text = Text::new("qwerty");
    /// let sub = text.slice(1, 3);
    /// assert_eq!("we", sub.as_str());
    /// ```
    pub fn slice(&self, start: usize, end: usize) -> Text {
        self.substring(start, end - start)
    }

    /// Gets the length of the [`Text`].
    ///
    /// # Example
    /// ```
    /// use quetta::Text;
    ///
    /// let text = Text::new("I was born in a water moon");
    /// assert_eq!(26, text.len());
    /// ```
    pub fn len(&self) -> usize {
        self.as_str().len()
    }

    /// Is this [`Text`] empty?
    ///
    /// # Example
    /// ```
    /// use quetta::Text;
    ///
    /// let text = Text::default();
    /// assert!(text.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }

    /// Attempt to create a [`Text`] from a slice sliced from this [`Text`].
    /// Will return `None` if the `slice` is not contained in `self`.
    ///
    /// # Example
    /// ```
    /// use quetta::Text;
    ///
    /// let text = Text::new("Test");
    /// let s = &text.as_str()[0..2];
    /// let st = text.lift_slice(s);
    /// assert!(st.is_some());
    /// assert_eq!("Te", st.unwrap().as_str());
    /// ```
    pub fn lift_slice(&self, slice: &str) -> Option<Text> {
        get_offset(self.as_str(), slice).map(|offset| self.substring(offset, slice.len()))
    }

    /// Lifts a function `&str -> &str` so it will be executed on the `&str` self.
    /// Will return none if the `&str` returned by the function is not contained in `self`.
    ///
    /// # Example
    /// ```
    /// use quetta::Text;
    ///
    /// let text = Text::new("  a  ");
    /// let trimmed = text.try_lift(|t| t.trim())?;
    /// assert_eq!("a", trimmed.as_str());
    /// ```
    pub fn try_lift<F: Fn(&str) -> &str>(&self, f: F) -> Option<Text> {
        let s = self.as_str();
        let res = f(s);
        self.lift_slice(res)
    }

    /// Lifts a function `&str -> &str` so it will be executed on the `&str` self.
    /// If the returned `&str` is not contained in `self`, a new [`Text`] will be created from it.
    ///
    /// # Example
    /// ```
    /// use quetta::Text;
    ///
    /// let text = Text::new("  a  ");
    /// let trimmed = text.try_lift(|t| t.trim())?;
    /// assert_eq!("a", trimmed.as_str());
    /// ```
    pub fn lift<F: Fn(&str) -> &str>(&self, f: F) -> Text {
        let s = self.as_str();
        let res = f(s);
        self.lift_slice(res).unwrap_or_else(|| Text::new(res))
    }

    /// Lifts a function `&str -> Iterator<Item=&str>` so it will be executed on `self` and returns an `Iterator<Item=[`Text`]>`.
    /// If one of the `&str` in the iterator is not contained in `self`, the iterator will end.
    ///
    /// # Example
    /// ```
    /// use quetta::Text;
    /// let t = Text::new("A:B:C:D");
    /// let lifted: Vec<Text> = t.try_lift_many(|s| s.split(":")).collect();
    /// assert_eq!(4, lifted.len());
    /// assert_eq!("A", lifted[0].as_str());
    /// assert_eq!("B", lifted[1].as_str());
    /// assert_eq!("C", lifted[2].as_str());
    /// assert_eq!("D", lifted[3].as_str());
    /// ```
    pub fn try_lift_many<'a, I: Iterator<Item = &'a str> + 'a, F: Fn(&'a str) -> I>(
        &'a self,
        f: F,
    ) -> impl Iterator<Item = Text> + 'a {
        let s = self.as_str();
        let res = f(s);
        res.scan((), move |(), s| self.lift_slice(s)).fuse()
    }

    /// Lifts a function `&str -> Iterator<Item=&str>` so it will be executed on `self` and returns an `Iterator<Item=[`Text`]>`.
    /// If the iterator yields a `&str` that is not contained within `self`, a new [`Text`] will be created from it.
    ///
    /// # Example
    /// ```
    /// use quetta::Text;
    /// let t = Text::new("A:B:C:D");
    /// let lifted: Vec<Text> = t.lift_many(|s| s.split(":")).collect();
    /// assert_eq!(4, lifted.len());
    /// assert_eq!("A", lifted[0].as_str());
    /// assert_eq!("B", lifted[1].as_str());
    /// assert_eq!("C", lifted[2].as_str());
    /// assert_eq!("D", lifted[3].as_str());
    /// ```
    pub fn lift_many<'a, I: Iterator<Item = &'a str> + 'a, F: Fn(&'a str) -> I>(
        &'a self,
        f: F,
    ) -> impl Iterator<Item = Text> + 'a {
        let s = self.as_str();
        let res = f(s);
        res.map(move |s| self.lift_slice(s).unwrap_or_else(|| Text::new(s)))
    }
}

fn get_offset(original: &str, slice: &str) -> Option<usize> {
    let orig_pos = original.as_ptr() as usize;
    let orig_end = orig_pos + original.len();
    let slice_pos = slice.as_ptr() as usize;
    let slice_end = slice_pos + slice.len();
    if slice_pos < orig_pos || slice_end > orig_end {
        None
    } else {
        Some(slice_pos - orig_pos)
    }
}

#[cfg(test)]
mod tests {
    use crate::Text;

    #[test]
    pub fn test_slice1() {
        let t = Text::new("a.b.c");
        let s1 = t.slice(0, 2);
        assert_eq!("a.", s1.as_str());
        assert_eq!(&t[..2], s1.as_str());
        assert_eq!(&s1, &t.substring(0, 2));
        assert_eq!(&s1, &Text::new("a."));
        assert_eq!(2, s1.len());
        let s2 = t.slice(4, 4);
        assert_eq!(&t[4..4], s2.as_str());
        assert_eq!(0, s2.len())
    }

    #[test]
    #[should_panic]
    pub fn test_invalid_slices1() {
        let t = Text::new("ASDFG");
        t.substring(4, 5);
    }

    #[test]
    #[should_panic]
    pub fn test_invalid_slices2() {
        let t = Text::new("ASDFG");
        t.slice(6, 8);
    }

    #[test]
    pub fn test_lift() {
        let t = Text::new(" TEST  ");
        let trimmed = t.try_lift(|t| t.trim()).expect("Lifting failed");
        assert_eq!("TEST", trimmed.as_str());
    }

    #[test]
    pub fn test_lift_many() {
        let t = Text::new("A:B:C:D");
        let lifted: Vec<Text> = t.lift_many(|s| s.split(":")).collect();
        assert_eq!(4, lifted.len());
        assert_eq!("A", lifted[0].as_str());
        assert_eq!("B", lifted[1].as_str());
        assert_eq!("C", lifted[2].as_str());
        assert_eq!("D", lifted[3].as_str());
    }
}
