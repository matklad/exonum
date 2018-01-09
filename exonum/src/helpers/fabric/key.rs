use std::marker::PhantomData;
use std::fmt;

/// `Key` provides strongly typed access to data inside `Context`.
/// See `exonum::fabric::keys` for keys used by the exonum itself.
pub struct Key<T> {
    // These fields are public so that `config_key`
    // macro works outside of this crate. It should be
    // replaced with `const fn`, once it is stable.
    #[doc(hidden)]
    pub __name: &'static str,
    #[doc(hidden)]
    pub __phantom: PhantomData<T>,
}

// We need explicit `impl Copy` because derive won't work if `T: !Copy`.
impl<T> Copy for Key<T> {}

#[cfg_attr(feature = "cargo-clippy", allow(expl_impl_clone_on_copy))]
impl<T> Clone for Key<T> {
    fn clone(&self) -> Self {
        Key {
            __name: self.__name,
            __phantom: self.__phantom,
        }
    }
}

impl<T> fmt::Debug for Key<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Key({:?})", self.__name)
    }
}

impl<T> Key<T> {
    /// Name of this key.
    pub fn name(&self) -> &str {
        self.__name
    }
}

/// Constructs a `Key` from a given name.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate exonum;
/// use exonum::helpers::fabric::Key;
///
/// const NAME: Key<String> = key!("name");
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! key {
    ($name:expr) => {{
        $crate::helpers::fabric::Key {
            __name: $name,
            __phantom: ::std::marker::PhantomData
        }
    }}
}

#[test]
fn key_is_really_copy() {
    const K: Key<Vec<String>> = key!("k");
    let x = K;
    let y = x;
    assert_eq!(x.name(), y.name());
}
