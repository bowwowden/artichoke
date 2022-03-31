use core::fmt;
use core::iter::{Cycle, FusedIterator, Take};
use core::slice;

use crate::chars::Chars;

/// Error returned when failing to construct a [`Center`] iterator.
///
/// This error is returned from [`String::center`]. See its documentation for
/// more detail.
///
/// This error corresponds to the [Ruby `ArgumentError` Exception class].
///
/// When the **std** feature of `spinoso-string` is enabled, this struct
/// implements [`std::error::Error`].
///
/// [`String::center`]: crate::String::center
/// [Ruby `ArgumentError` Exception class]: https://ruby-doc.org/core-2.6.3/ArgumentError.html
/// [`std::error::Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CenterError {
    /// Error returned when calling [`String::center`] with an empty padding
    /// byte string.
    ///
    /// [`String::center`]: crate::String::center
    ZeroWidthPadding,
}

impl CenterError {
    pub const EXCEPTION_TYPE: &'static str = "ArgumentError";

    /// Create a new zero width padding `CenterError`.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::CenterError;
    ///
    /// const ERR: CenterError = CenterError::zero_width_padding();
    /// assert_eq!(ERR.message(), "zero width padding");
    /// ```
    #[inline]
    #[must_use]
    pub const fn zero_width_padding() -> Self {
        Self::ZeroWidthPadding
    }

    /// Retrieve the exception message associated with this center error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_string::CenterError;
    /// let err = CenterError::zero_width_padding();
    /// assert_eq!(err.message(), "zero width padding");
    /// ```
    #[inline]
    #[must_use]
    #[allow(clippy::unused_self)]
    pub const fn message(self) -> &'static str {
        "zero width padding"
    }
}

impl fmt::Display for CenterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let CenterError::ZeroWidthPadding = self;
        f.write_str(self.message())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CenterError {}

/// An iterator that yields a byte string centered within a padding byte string.
///
/// This struct is created by the [`center`] method on a Spinoso [`String`]. See
/// its documentation for more.
///
/// # Examples
///
/// ```
/// use spinoso_string::String;
/// # fn example() -> Result<(), spinoso_string::CenterError> {
/// let s = String::from("hello");
///
/// assert_eq!(s.center(4, None)?.collect::<Vec<_>>(), b"hello");
/// assert_eq!(s.center(20, None)?.collect::<Vec<_>>(), b"       hello        ");
/// assert_eq!(s.center(20, Some(&b"123"[..]))?.collect::<Vec<_>>(), b"1231231hello12312312");
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// This iterator is [encoding-aware]. [Conventionally UTF-8] strings are
/// iterated by UTF-8 byte sequences.
///
/// ```
/// use spinoso_string::String;
/// # fn example() -> Result<(), spinoso_string::CenterError> {
/// let s = String::from("💎");
///
/// assert_eq!(s.center(3, None)?.collect::<Vec<_>>(), " 💎 ".as_bytes());
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// [`String`]: crate::String
/// [`center`]: crate::String::center
/// [encoding-aware]: crate::Encoding
/// [Conventionally UTF-8]: crate::Encoding::Utf8
#[derive(Debug, Clone)]
pub struct Center<'a, 'b> {
    pub left: Take<Cycle<slice::Iter<'b, u8>>>,
    pub next: Option<&'a [u8]>,
    pub s: Chars<'a>,
    pub right: Take<Cycle<slice::Iter<'b, u8>>>,
}

impl<'a, 'b> Center<'a, 'b> {
    #[inline]
    #[must_use]
    pub(crate) fn with_chars_width_and_padding(
        s: Chars<'a>,
        padding_width: usize,
        padding: Option<&'b [u8]>,
    ) -> Result<Self, CenterError> {
        // ```
        // [3.0.3] > "abc".center 10, ""
        // (irb):5:in `center': zero width padding (ArgumentError)
        // 	from (irb):5:in `<main>'
        // 	from /usr/local/var/rbenv/versions/3.0.3/lib/ruby/gems/3.0.0/gems/irb-1.3.5/exe/irb:11:in `<top (required)>'
        // 	from /usr/local/var/rbenv/versions/3.0.3/bin/irb:23:in `load'
        // 	from /usr/local/var/rbenv/versions/3.0.3/bin/irb:23:in `<main>'
        // [3.0.3] > "abc".center 3, ""
        // (irb):6:in `center': zero width padding (ArgumentError)
        // 	from (irb):6:in `<main>'
        // 	from /usr/local/var/rbenv/versions/3.0.3/lib/ruby/gems/3.0.0/gems/irb-1.3.5/exe/irb:11:in `<top (required)>'
        // 	from /usr/local/var/rbenv/versions/3.0.3/bin/irb:23:in `load'
        // 	from /usr/local/var/rbenv/versions/3.0.3/bin/irb:23:in `<main>'
        // [3.0.3] > "abc".center 0, ""
        // (irb):7:in `center': zero width padding (ArgumentError)
        // 	from (irb):7:in `<main>'
        // 	from /usr/local/var/rbenv/versions/3.0.3/lib/ruby/gems/3.0.0/gems/irb-1.3.5/exe/irb:11:in `<top (required)>'
        // 	from /usr/local/var/rbenv/versions/3.0.3/bin/irb:23:in `load'
        // 	from /usr/local/var/rbenv/versions/3.0.3/bin/irb:23:in `<main>'
        // [3.0.3] > "abc".center 10, " "
        // => "   abc    "
        // [3.0.3] > "abc".center 3, " "
        // => "abc"
        // [3.0.3] > "abc".center 0, " "
        // => "abc"
        // ```
        let padding = match padding {
            None => b" ",
            Some(p) if p.is_empty() => return Err(CenterError::ZeroWidthPadding),
            Some(p) => p,
        };

        let pre_pad = padding_width / 2;
        let post_pad = padding_width - pre_pad;

        // Left and right padding starts from the beginning of padding.
        //
        // ```
        // [3.0.3] > "abc".center 10, "123456789"
        // => "123abc1234"
        // ```
        let left = padding.iter().cycle().take(pre_pad);
        let right = padding.iter().cycle().take(post_pad);

        Ok(Self {
            left,
            next: None,
            s,
            right,
        })
    }
}

impl<'a, 'b> Iterator for Center<'a, 'b> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&next) = self.left.next() {
            return Some(next);
        }
        if let Some(next) = self.next.take() {
            if let Some((&first, tail)) = next.split_first() {
                self.next = Some(tail);
                return Some(first);
            }
        }
        if let Some(next) = self.s.next() {
            if let Some((&first, tail)) = next.split_first() {
                if !tail.is_empty() {
                    self.next = Some(tail);
                }
                return Some(first);
            }
        }
        self.right.next().copied()
    }
}

impl<'a, 'b> FusedIterator for Center<'a, 'b> {}

impl<'a, 'b> ExactSizeIterator for Center<'a, 'b> {}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use alloc::vec::Vec;

    use super::{Center, CenterError};

    #[test]
    fn empty_padding_gives_error() {
        let s = crate::String::from("");
        let center = Center::with_chars_width_and_padding(s.chars(), 0, Some(&b""[..]));
        assert_eq!(center.unwrap_err(), CenterError::ZeroWidthPadding);
    }
}
