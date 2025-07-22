#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterCnt {
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA."]
    FILTER_CNT_0 = 0x0,
    #[doc = "One sample must agree. The comparator output is simply sampled."]
    FILTER_CNT_1 = 0x01,
    #[doc = "2 consecutive samples must agree."]
    FILTER_CNT_2 = 0x02,
    #[doc = "3 consecutive samples must agree."]
    FILTER_CNT_3 = 0x03,
    #[doc = "4 consecutive samples must agree."]
    FILTER_CNT_4 = 0x04,
    #[doc = "5 consecutive samples must agree."]
    FILTER_CNT_5 = 0x05,
    #[doc = "6 consecutive samples must agree."]
    FILTER_CNT_6 = 0x06,
    #[doc = "7 consecutive samples must agree."]
    FILTER_CNT_7 = 0x07,
}
impl FilterCnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilterCnt {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilterCnt {
    #[inline(always)]
    fn from(val: u8) -> FilterCnt {
        FilterCnt::from_bits(val)
    }
}
impl From<FilterCnt> for u8 {
    #[inline(always)]
    fn from(val: FilterCnt) -> u8 {
        FilterCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hystctr {
    #[doc = "Level 0"]
    HYSTCTR_0 = 0x0,
    #[doc = "Level 1"]
    HYSTCTR_1 = 0x01,
    #[doc = "Level 2"]
    HYSTCTR_2 = 0x02,
    #[doc = "Level 3"]
    HYSTCTR_3 = 0x03,
}
impl Hystctr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hystctr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hystctr {
    #[inline(always)]
    fn from(val: u8) -> Hystctr {
        Hystctr::from_bits(val)
    }
}
impl From<Hystctr> for u8 {
    #[inline(always)]
    fn from(val: Hystctr) -> u8 {
        Hystctr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Msel {
    #[doc = "IN0"]
    MSEL_0 = 0x0,
    #[doc = "IN1"]
    MSEL_1 = 0x01,
    #[doc = "IN2"]
    MSEL_2 = 0x02,
    #[doc = "IN3"]
    MSEL_3 = 0x03,
    #[doc = "IN4"]
    MSEL_4 = 0x04,
    #[doc = "IN5"]
    MSEL_5 = 0x05,
    #[doc = "IN6"]
    MSEL_6 = 0x06,
    #[doc = "IN7"]
    MSEL_7 = 0x07,
}
impl Msel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Msel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Msel {
    #[inline(always)]
    fn from(val: u8) -> Msel {
        Msel::from_bits(val)
    }
}
impl From<Msel> for u8 {
    #[inline(always)]
    fn from(val: Msel) -> u8 {
        Msel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Psel {
    #[doc = "IN0"]
    PSEL_0 = 0x0,
    #[doc = "IN1"]
    PSEL_1 = 0x01,
    #[doc = "IN2"]
    PSEL_2 = 0x02,
    #[doc = "IN3"]
    PSEL_3 = 0x03,
    #[doc = "IN4"]
    PSEL_4 = 0x04,
    #[doc = "IN5"]
    PSEL_5 = 0x05,
    #[doc = "IN6"]
    PSEL_6 = 0x06,
    #[doc = "IN7"]
    PSEL_7 = 0x07,
}
impl Psel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psel {
    #[inline(always)]
    fn from(val: u8) -> Psel {
        Psel::from_bits(val)
    }
}
impl From<Psel> for u8 {
    #[inline(always)]
    fn from(val: Psel) -> u8 {
        Psel::to_bits(val)
    }
}
