// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
#[derive( Debug, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16
}
impl From<u16> for SaturatingU16 {
    fn from(val: u16) -> Self {
        SaturatingU16 { value: val } 
    }
}

impl From<u8> for SaturatingU16 {
    fn from(val: u8) -> Self {
        SaturatingU16 { value: val.into() } 
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(val: &u16) -> Self {
        SaturatingU16 { value: (*val).into()} 
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(val: &u8) -> Self {
        SaturatingU16 { value: (*val).into() } 
    }
}


impl std::ops::Add for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.value
    }
}

impl std::ops::Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        self + *rhs
    }
}

impl std::ops::Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &u16) -> Self::Output {
        let sum = self.value.saturating_add(*rhs);
        Self {value: sum}
    }
}

impl std::ops::Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: u16) -> Self::Output {
        let sum = self.value.saturating_add(rhs);
        Self {value: sum}
    }
}

impl PartialEq for SaturatingU16 {
    fn eq(&self, other : &SaturatingU16) -> bool {
        self.value == other.value
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other : &u16) -> bool {
        self.value == *other
    }
}