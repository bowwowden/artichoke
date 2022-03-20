use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};

use super::BinaryString;

impl Extend<u8> for BinaryString {
    #[inline]
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        self.inner.extend(iter.into_iter());
    }
}

impl<'a> Extend<&'a u8> for BinaryString {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a u8>>(&mut self, iter: I) {
        self.inner.extend(iter.into_iter().copied());
    }
}

impl<'a> Extend<&'a mut u8> for BinaryString {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a mut u8>>(&mut self, iter: I) {
        self.inner.extend(iter.into_iter().map(|byte| *byte));
    }
}

impl From<Vec<u8>> for BinaryString {
    #[inline]
    fn from(content: Vec<u8>) -> Self {
        Self::new(content)
    }
}

impl<'a> From<&'a [u8]> for BinaryString {
    #[inline]
    fn from(content: &'a [u8]) -> Self {
        Self::new(content.to_vec())
    }
}

impl<'a> From<&'a mut [u8]> for BinaryString {
    #[inline]
    fn from(content: &'a mut [u8]) -> Self {
        Self::new(content.to_vec())
    }
}

impl<'a> From<Cow<'a, [u8]>> for BinaryString {
    #[inline]
    fn from(content: Cow<'a, [u8]>) -> Self {
        Self::new(content.into_owned())
    }
}

impl From<String> for BinaryString {
    #[inline]
    fn from(s: String) -> Self {
        Self::new(s.into_bytes())
    }
}

impl From<&str> for BinaryString {
    #[inline]
    fn from(s: &str) -> Self {
        Self::new(s.as_bytes().to_vec())
    }
}

impl From<BinaryString> for Vec<u8> {
    #[inline]
    fn from(s: BinaryString) -> Self {
        s.into_vec()
    }
}

impl AsRef<[u8]> for BinaryString {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.inner.as_slice()
    }
}

impl AsMut<[u8]> for BinaryString {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.inner.as_mut_slice()
    }
}

impl AsRef<Vec<u8>> for BinaryString {
    #[inline]
    fn as_ref(&self) -> &Vec<u8> {
        &self.inner
    }
}

impl AsMut<Vec<u8>> for BinaryString {
    #[inline]
    fn as_mut(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }
}

impl Deref for BinaryString {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl DerefMut for BinaryString {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}
