#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AOn {
    #[doc = "DMA Channel Always ON function is disabled"]
    A_ON_0 = 0x0,
    #[doc = "DMA Channel Always ON function is enabled"]
    A_ON_1 = 0x01,
}
impl AOn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AOn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AOn {
    #[inline(always)]
    fn from(val: u8) -> AOn {
        AOn::from_bits(val)
    }
}
impl From<AOn> for u8 {
    #[inline(always)]
    fn from(val: AOn) -> u8 {
        AOn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enbl {
    #[doc = "DMA Mux channel is disabled"]
    ENBL_0 = 0x0,
    #[doc = "DMA Mux channel is enabled"]
    ENBL_1 = 0x01,
}
impl Enbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enbl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enbl {
    #[inline(always)]
    fn from(val: u8) -> Enbl {
        Enbl::from_bits(val)
    }
}
impl From<Enbl> for u8 {
    #[inline(always)]
    fn from(val: Enbl) -> u8 {
        Enbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig {
    #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    TRIG_0 = 0x0,
    #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMA_CH_MUX is in Periodic Trigger mode."]
    TRIG_1 = 0x01,
}
impl Trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig {
    #[inline(always)]
    fn from(val: u8) -> Trig {
        Trig::from_bits(val)
    }
}
impl From<Trig> for u8 {
    #[inline(always)]
    fn from(val: Trig) -> u8 {
        Trig::to_bits(val)
    }
}
