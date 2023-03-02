use crate::{collections::raw_vec::RawVec, traits::Capacity, traits::SingleRangeStorage};

/// Poc of String
///
/// A UTF-8-encoded, growable string.
pub struct RawString<S: SingleRangeStorage> {
    vec: RawVec<u8, S>,
}

impl<S: SingleRangeStorage> RawString<S> {
    /// Creates a new empty `String`
    pub fn new(storage: S) -> Self {
        Self {
            vec: RawVec::new(storage),
        }
    }

    /// Creates a new empty `String` with the specified capacity.
    pub fn with_capacity(storage: S, capacity: usize) -> Self {
        Self {
            vec: RawVec::with_capacity(storage, capacity),
        }
    }

    /// Create a RawString from a String
    pub fn from_string(storage: S, s: String) -> Self {
        let mut raw_string = Self::with_capacity(storage, s.len());
        raw_string.vec.extend_from_slice(s.as_bytes());
        raw_string
    }
    ///
    pub fn push_str(&mut self, string: &str) {
        self.vec.extend_from_slice(string.as_bytes())
    }
}

impl<S: Default + SingleRangeStorage> Default for RawString<S> {
    fn default() -> Self {
        Self {
            vec: Default::default(),
        }
    }
}

impl<S: SingleRangeStorage> core::ops::Deref for RawString<S> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.vec) }
    }
}

// impl Debug
impl<S: SingleRangeStorage> core::fmt::Debug for RawString<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self[..], f)
    }
}

// impl Display
impl<S: SingleRangeStorage> core::fmt::Display for RawString<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self[..], f)
    }
}

#[cfg(test)]
mod test_inline {
    use core::mem;

    use crate::inline::SingleRange;
    use crate::inline::U8Storage;

    use super::*;

    #[test]
    fn smoke_test() {
        type Storage = U8Storage<u8, 31>;
        type String = RawString<Storage>;

        let mut vec = String::default();
        let x: &str = &vec;

        assert_eq!("", x);

        let vec = String::from_string(Storage::default(), std::string::String::from("xxxxx"));

        let x: &str = &vec;

        assert_eq!("xxxxx", x);

        let output = format!("{:?}", vec);
        assert_eq!("\"xxxxx\"", output);

        let str = String::from_string(Storage::default(), std::string::String::from("你好"));
        let output = format!("{:?}", str);
        assert_eq!("\"你好\"", output);
    }

    #[test]
    #[should_panic]
    fn exceed_capacity() {
        type Storage = U8Storage<u8, 5>;
        type String = RawString<Storage>;
        let vec = String::from_string(Storage::default(), std::string::String::from("xxxxxx"));
    }
}

#[cfg(test)]
mod test_allocator {
    use core::mem;
    use std::ops::Deref;

    use crate::allocator::SingleRange;
    use crate::utils::{NonAllocator, SpyAllocator};

    use super::*;

    #[test]
    fn smoke_test() {
        type Storage = SingleRange<SpyAllocator>;
        type String = RawString<Storage>;

        let allocator = SpyAllocator::default();

        let storage = SingleRange::new(allocator.clone());
        let mut string = String::new(storage);

        assert_eq!(0, allocator.allocated());
        assert_eq!(0, allocator.deallocated());

        string.push_str("x");
        string.push_str("你好");

        assert_eq!(2, allocator.allocated());

        string.push_str("啊");

        assert_eq!(3, allocator.allocated());

        string.push_str("啊");

        assert_eq!(3, allocator.allocated());

        assert_eq!("x你好啊啊", string.deref());
        drop(string);

        assert_eq!(3, allocator.allocated());
        assert_eq!(3, allocator.deallocated());
    }
}
