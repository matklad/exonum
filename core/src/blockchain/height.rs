use std::fmt;

/// Blockchain's height (number of blocks).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Height(pub u64);

impl Height {
    /// Returns zero value of the height.
    ///
    /// # Examples
    ///
    /// ```
    /// use exonum::helpers::Height;
    ///
    /// let height = Height::zero();
    /// assert_eq!(0, height.0);
    /// ```
    pub fn zero() -> Self {
        Height(0)
    }

    /// Returns next value of the height.
    ///
    /// # Examples
    ///
    /// ```
    /// use exonum::helpers::Height;
    ///
    /// let height = Height(10);
    /// let next_height = height.next();
    /// assert_eq!(11, next_height.0);
    /// ```
    pub fn next(&self) -> Self {
        Height(self.0 + 1)
    }

    /// Returns previous value of the height.
    ///
    /// # Panics
    ///
    /// Panics if `self.0` is equal to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use exonum::helpers::Height;
    ///
    /// let height = Height(10);
    /// let previous_height = height.previous();
    /// assert_eq!(9, previous_height.0);
    /// ```
    pub fn previous(&self) -> Self {
        assert_ne!(0, self.0);
        Height(self.0 - 1)
    }

    /// Increments the height value.
    ///
    /// # Examples
    ///
    /// ```
    /// use exonum::helpers::Height;
    ///
    /// let mut height = Height::zero();
    /// height.increment();
    /// assert_eq!(1, height.0);
    /// ```
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    /// Decrements the height value.
    ///
    /// # Panics
    ///
    /// Panics if `self.0` is equal to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use exonum::helpers::Height;
    ///
    /// let mut height = Height(20);
    /// height.decrement();
    /// assert_eq!(19, height.0);
    /// ```
    pub fn decrement(&mut self) {
        assert_ne!(0, self.0);
        self.0 -= 1;
    }
}

impl fmt::Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Height> for u64 {
    fn from(val: Height) -> Self {
        val.0
    }
}

