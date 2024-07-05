// use outro_03::SaturatingU16;
use std::ops::{Add, Deref};
use std::cmp::Eq;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SaturatingU16 {
    v: u16,
}

impl From<u8> for SaturatingU16 {
    fn from(v: u8) -> Self {
        SaturatingU16 { v: v.into() }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(v: &u8) -> Self {
        SaturatingU16 { v: (*v).into() }
    }
}

impl From<u16> for SaturatingU16 {
    fn from(v: u16) -> Self {
        SaturatingU16 { v: v.into() }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(v: &u16) -> Self {
        SaturatingU16 { v: (*v).into() }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = u16;
    fn add(self, rhs: u16) -> u16 {
        self.v + rhs
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: SaturatingU16) -> SaturatingU16 {
        SaturatingU16 { v: self.v.saturating_add(rhs.v) }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &SaturatingU16) -> SaturatingU16 {
        SaturatingU16 { v: self.v.saturating_add((*rhs).v) }
    }
}

impl Deref for SaturatingU16 {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

// impl Add<&SaturatingU16> for SaturatingU16 {
//     type Output = &SaturatingU16;
//     fn add(self, rhs: &SaturatingU16) -> SaturatingU16 {
//         SaturatingU16 { v: self.v + rhs.v }
//     }
// }

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, rhs: &u16) -> bool {
        self.v == *rhs
    }
}

impl PartialEq<SaturatingU16> for u16 {
    fn eq(&self, rhs: &SaturatingU16) -> bool {
        *self == rhs.v
    }
}


#[test]
fn test_saturating_u16() {
    let a: SaturatingU16 = (&10u8).into();
    let b: SaturatingU16 = 5u8.into();
    let c: SaturatingU16 = u16::MAX.into();
    let d: SaturatingU16 = (&1u16).into();
    let e = &c;

    assert_eq!(a + b, SaturatingU16::from(15u16));
    assert_eq!(a + c, SaturatingU16::from(u16::MAX));
    assert_eq!(a + d, SaturatingU16::from(11u16));
    assert_eq!(a + a, 20u16);
    assert_eq!(a + 5u16, 15u16);
    assert_eq!(a + e, SaturatingU16::from(u16::MAX));
}
