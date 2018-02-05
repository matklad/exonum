use std::fmt;

/// Validators identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ValidatorId(pub u16);

impl ValidatorId {
    /// Returns zero value of the validator id.
    ///
    /// # Examples
    ///
    /// ```
    /// use exonum::helpers::ValidatorId;
    ///
    /// let id = ValidatorId::zero();
    /// assert_eq!(0, id.0);
    /// ```
    pub fn zero() -> Self {
        ValidatorId(0)
    }
}

impl fmt::Display for ValidatorId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ValidatorId> for u16 {
    fn from(val: ValidatorId) -> Self {
        val.0
    }
}

impl From<ValidatorId> for usize {
    fn from(val: ValidatorId) -> Self {
        val.0 as usize
    }
}
