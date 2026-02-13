#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc15I2cSclFsel {
    #[doc = "Function 0."]
    FUNCTION_0 = 0x0,
    #[doc = "Function 1."]
    FUNCTION_1 = 0x01,
    #[doc = "Function 2."]
    FUNCTION_2 = 0x02,
    #[doc = "Function 3."]
    FUNCTION_3 = 0x03,
    #[doc = "Function 4."]
    FUNCTION_4 = 0x04,
    #[doc = "Function 5."]
    FUNCTION_5 = 0x05,
    #[doc = "Function 6."]
    FUNCTION_6 = 0x06,
    #[doc = "Function 7."]
    FUNCTION_7 = 0x07,
    #[doc = "Function 8."]
    FUNCTION_8 = 0x08,
    #[doc = "Function 9."]
    FUNCTION_9 = 0x09,
    #[doc = "Function 10."]
    FUNCTION_10 = 0x0a,
    #[doc = "Function 11."]
    FUNCTION_11 = 0x0b,
    #[doc = "Function 12."]
    FUNCTION_12 = 0x0c,
    #[doc = "Function 13."]
    FUNCTION_13 = 0x0d,
    #[doc = "Function 14."]
    FUNCTION_14 = 0x0e,
    #[doc = "Function 15."]
    FUNCTION_15 = 0x0f,
}
impl Fc15I2cSclFsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc15I2cSclFsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc15I2cSclFsel {
    #[inline(always)]
    fn from(val: u8) -> Fc15I2cSclFsel {
        Fc15I2cSclFsel::from_bits(val)
    }
}
impl From<Fc15I2cSclFsel> for u8 {
    #[inline(always)]
    fn from(val: Fc15I2cSclFsel) -> u8 {
        Fc15I2cSclFsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc15I2cSclFulldrive {
    #[doc = "Normal Drive."]
    NORMAL_DRIVE = 0x0,
    #[doc = "Full Drive."]
    FULL_DRIVE = 0x01,
}
impl Fc15I2cSclFulldrive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc15I2cSclFulldrive {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc15I2cSclFulldrive {
    #[inline(always)]
    fn from(val: u8) -> Fc15I2cSclFulldrive {
        Fc15I2cSclFulldrive::from_bits(val)
    }
}
impl From<Fc15I2cSclFulldrive> for u8 {
    #[inline(always)]
    fn from(val: Fc15I2cSclFulldrive) -> u8 {
        Fc15I2cSclFulldrive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc15I2cSclPupdsel {
    #[doc = "Pull-down."]
    PULL_DOWN = 0x0,
    #[doc = "Pull-up."]
    PULL_UP = 0x01,
}
impl Fc15I2cSclPupdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc15I2cSclPupdsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc15I2cSclPupdsel {
    #[inline(always)]
    fn from(val: u8) -> Fc15I2cSclPupdsel {
        Fc15I2cSclPupdsel::from_bits(val)
    }
}
impl From<Fc15I2cSclPupdsel> for u8 {
    #[inline(always)]
    fn from(val: Fc15I2cSclPupdsel) -> u8 {
        Fc15I2cSclPupdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc15I2cSclSlewrate {
    #[doc = "Slew Rate is Normal."]
    NORMAL = 0x0,
    #[doc = "Slew Rate Slow."]
    SLOW = 0x01,
}
impl Fc15I2cSclSlewrate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc15I2cSclSlewrate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc15I2cSclSlewrate {
    #[inline(always)]
    fn from(val: u8) -> Fc15I2cSclSlewrate {
        Fc15I2cSclSlewrate::from_bits(val)
    }
}
impl From<Fc15I2cSclSlewrate> for u8 {
    #[inline(always)]
    fn from(val: Fc15I2cSclSlewrate) -> u8 {
        Fc15I2cSclSlewrate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc15I2cSdaFsel {
    #[doc = "Function 0."]
    FUNCTION_0 = 0x0,
    #[doc = "Function 1."]
    FUNCTION_1 = 0x01,
    #[doc = "Function 2."]
    FUNCTION_2 = 0x02,
    #[doc = "Function 3."]
    FUNCTION_3 = 0x03,
    #[doc = "Function 4."]
    FUNCTION_4 = 0x04,
    #[doc = "Function 5."]
    FUNCTION_5 = 0x05,
    #[doc = "Function 6."]
    FUNCTION_6 = 0x06,
    #[doc = "Function 7."]
    FUNCTION_7 = 0x07,
    #[doc = "Function 8."]
    FUNCTION_8 = 0x08,
    #[doc = "Function 9."]
    FUNCTION_9 = 0x09,
    #[doc = "Function 10."]
    FUNCTION_10 = 0x0a,
    #[doc = "Function 11."]
    FUNCTION_11 = 0x0b,
    #[doc = "Function 12."]
    FUNCTION_12 = 0x0c,
    #[doc = "Function 13."]
    FUNCTION_13 = 0x0d,
    #[doc = "Function 14."]
    FUNCTION_14 = 0x0e,
    #[doc = "Function 15."]
    FUNCTION_15 = 0x0f,
}
impl Fc15I2cSdaFsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc15I2cSdaFsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc15I2cSdaFsel {
    #[inline(always)]
    fn from(val: u8) -> Fc15I2cSdaFsel {
        Fc15I2cSdaFsel::from_bits(val)
    }
}
impl From<Fc15I2cSdaFsel> for u8 {
    #[inline(always)]
    fn from(val: Fc15I2cSdaFsel) -> u8 {
        Fc15I2cSdaFsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc15I2cSdaFulldrive {
    #[doc = "Normal Drive."]
    NORMAL_DRIVE = 0x0,
    #[doc = "Full Drive."]
    FULL_DRIVE = 0x01,
}
impl Fc15I2cSdaFulldrive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc15I2cSdaFulldrive {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc15I2cSdaFulldrive {
    #[inline(always)]
    fn from(val: u8) -> Fc15I2cSdaFulldrive {
        Fc15I2cSdaFulldrive::from_bits(val)
    }
}
impl From<Fc15I2cSdaFulldrive> for u8 {
    #[inline(always)]
    fn from(val: Fc15I2cSdaFulldrive) -> u8 {
        Fc15I2cSdaFulldrive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc15I2cSdaPupdsel {
    #[doc = "Pull-down."]
    PULL_DOWN = 0x0,
    #[doc = "Pull-up."]
    PULL_UP = 0x01,
}
impl Fc15I2cSdaPupdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc15I2cSdaPupdsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc15I2cSdaPupdsel {
    #[inline(always)]
    fn from(val: u8) -> Fc15I2cSdaPupdsel {
        Fc15I2cSdaPupdsel::from_bits(val)
    }
}
impl From<Fc15I2cSdaPupdsel> for u8 {
    #[inline(always)]
    fn from(val: Fc15I2cSdaPupdsel) -> u8 {
        Fc15I2cSdaPupdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc15I2cSdaSlewrate {
    #[doc = "Slew Rate is Normal."]
    NORMAL = 0x0,
    #[doc = "Slew Rate Slow."]
    SLOW = 0x01,
}
impl Fc15I2cSdaSlewrate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc15I2cSdaSlewrate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc15I2cSdaSlewrate {
    #[inline(always)]
    fn from(val: u8) -> Fc15I2cSdaSlewrate {
        Fc15I2cSdaSlewrate::from_bits(val)
    }
}
impl From<Fc15I2cSdaSlewrate> for u8 {
    #[inline(always)]
    fn from(val: Fc15I2cSdaSlewrate) -> u8 {
        Fc15I2cSdaSlewrate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fsel {
    #[doc = "Function 0."]
    FUNCTION_0 = 0x0,
    #[doc = "Function 1."]
    FUNCTION_1 = 0x01,
    #[doc = "Function 2."]
    FUNCTION_2 = 0x02,
    #[doc = "Function 3."]
    FUNCTION_3 = 0x03,
    #[doc = "Function 4."]
    FUNCTION_4 = 0x04,
    #[doc = "Function 5."]
    FUNCTION_5 = 0x05,
    #[doc = "Function 6."]
    FUNCTION_6 = 0x06,
    #[doc = "Function 7."]
    FUNCTION_7 = 0x07,
    #[doc = "Function 8."]
    FUNCTION_8 = 0x08,
    #[doc = "Function 9."]
    FUNCTION_9 = 0x09,
    #[doc = "Function 10."]
    FUNCTION_10 = 0x0a,
    #[doc = "Function 11."]
    FUNCTION_11 = 0x0b,
    #[doc = "Function 12."]
    FUNCTION_12 = 0x0c,
    #[doc = "Function 13."]
    FUNCTION_13 = 0x0d,
    #[doc = "Function 14."]
    FUNCTION_14 = 0x0e,
    #[doc = "Function 15."]
    FUNCTION_15 = 0x0f,
}
impl Fsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fsel {
    #[inline(always)]
    fn from(val: u8) -> Fsel {
        Fsel::from_bits(val)
    }
}
impl From<Fsel> for u8 {
    #[inline(always)]
    fn from(val: Fsel) -> u8 {
        Fsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fulldrive {
    #[doc = "Normal Drive."]
    NORMAL_DRIVE = 0x0,
    #[doc = "Full Drive."]
    FULL_DRIVE = 0x01,
}
impl Fulldrive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fulldrive {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fulldrive {
    #[inline(always)]
    fn from(val: u8) -> Fulldrive {
        Fulldrive::from_bits(val)
    }
}
impl From<Fulldrive> for u8 {
    #[inline(always)]
    fn from(val: Fulldrive) -> u8 {
        Fulldrive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pupdsel {
    #[doc = "Pull-down."]
    PULL_DOWN = 0x0,
    #[doc = "Pull-up."]
    PULL_UP = 0x01,
}
impl Pupdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pupdsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pupdsel {
    #[inline(always)]
    fn from(val: u8) -> Pupdsel {
        Pupdsel::from_bits(val)
    }
}
impl From<Pupdsel> for u8 {
    #[inline(always)]
    fn from(val: Pupdsel) -> u8 {
        Pupdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slewrate {
    #[doc = "Slew Rate is Normal."]
    NORMAL = 0x0,
    #[doc = "Slew Rate Slow."]
    SLOW = 0x01,
}
impl Slewrate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slewrate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slewrate {
    #[inline(always)]
    fn from(val: u8) -> Slewrate {
        Slewrate::from_bits(val)
    }
}
impl From<Slewrate> for u8 {
    #[inline(always)]
    fn from(val: Slewrate) -> u8 {
        Slewrate::to_bits(val)
    }
}
