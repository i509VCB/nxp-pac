#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CkctrlCkmode {
    #[doc = "Core clock is on"]
    CKMODE0000 = 0x0,
    #[doc = "Core clock is off"]
    CKMODE0001 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Core, platform, and peripheral clocks are off, and core enters Low-Power mode"]
    CKMODE1111 = 0x0f,
}
impl CkctrlCkmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CkctrlCkmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CkctrlCkmode {
    #[inline(always)]
    fn from(val: u8) -> CkctrlCkmode {
        CkctrlCkmode::from_bits(val)
    }
}
impl From<CkctrlCkmode> for u8 {
    #[inline(always)]
    fn from(val: CkctrlCkmode) -> u8 {
        CkctrlCkmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CkstatCkmode {
    #[doc = "Core clock is on"]
    CKMODE0000 = 0x0,
    #[doc = "Core clock is off"]
    CKMODE0001 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Core, platform, and peripheral clocks are off, and core enters Low-Power mode"]
    CKMODE1111 = 0x0f,
}
impl CkstatCkmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CkstatCkmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CkstatCkmode {
    #[inline(always)]
    fn from(val: u8) -> CkstatCkmode {
        CkstatCkmode::from_bits(val)
    }
}
impl From<CkstatCkmode> for u8 {
    #[inline(always)]
    fn from(val: CkstatCkmode) -> u8 {
        CkstatCkmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmctrlmainLpmode {
    #[doc = "Active/Sleep"]
    LPMODE0000 = 0x0,
    #[doc = "Deep Sleep"]
    LPMODE0001 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Power Down"]
    LPMODE0011 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Deep-Power Down"]
    LPMODE1111 = 0x0f,
}
impl PmctrlmainLpmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmctrlmainLpmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmctrlmainLpmode {
    #[inline(always)]
    fn from(val: u8) -> PmctrlmainLpmode {
        PmctrlmainLpmode::from_bits(val)
    }
}
impl From<PmctrlmainLpmode> for u8 {
    #[inline(always)]
    fn from(val: PmctrlmainLpmode) -> u8 {
        PmctrlmainLpmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmprotLpmode {
    #[doc = "Not allowed"]
    DISABLED = 0x0,
    #[doc = "Allowed"]
    EN = 0x01,
    #[doc = "Allowed"]
    EN1 = 0x02,
    #[doc = "Allowed"]
    EN2 = 0x03,
    #[doc = "Allowed"]
    EN3 = 0x04,
    #[doc = "Allowed"]
    EN4 = 0x05,
    #[doc = "Allowed"]
    EN5 = 0x06,
    #[doc = "Allowed"]
    EN6 = 0x07,
    #[doc = "Allowed"]
    EN7 = 0x08,
    #[doc = "Allowed"]
    EN8 = 0x09,
    #[doc = "Allowed"]
    EN9 = 0x0a,
    #[doc = "Allowed"]
    EN10 = 0x0b,
    #[doc = "Allowed"]
    EN11 = 0x0c,
    #[doc = "Allowed"]
    EN12 = 0x0d,
    #[doc = "Allowed"]
    EN13 = 0x0e,
    #[doc = "Allowed"]
    EN14 = 0x0f,
}
impl PmprotLpmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmprotLpmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmprotLpmode {
    #[inline(always)]
    fn from(val: u8) -> PmprotLpmode {
        PmprotLpmode::from_bits(val)
    }
}
impl From<PmprotLpmode> for u8 {
    #[inline(always)]
    fn from(val: PmprotLpmode) -> u8 {
        PmprotLpmode::to_bits(val)
    }
}
