#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MegaPdnReq {
    #[doc = "No Request"]
    MEGA_PDN_REQ_0 = 0x0,
    #[doc = "Request power down sequence"]
    MEGA_PDN_REQ_1 = 0x01,
}
impl MegaPdnReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MegaPdnReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MegaPdnReq {
    #[inline(always)]
    fn from(val: u8) -> MegaPdnReq {
        MegaPdnReq::from_bits(val)
    }
}
impl From<MegaPdnReq> for u8 {
    #[inline(always)]
    fn from(val: MegaPdnReq) -> u8 {
        MegaPdnReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MegaPupReq {
    #[doc = "No Request"]
    MEGA_PUP_REQ_0 = 0x0,
    #[doc = "Request power up sequence"]
    MEGA_PUP_REQ_1 = 0x01,
}
impl MegaPupReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MegaPupReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MegaPupReq {
    #[inline(always)]
    fn from(val: u8) -> MegaPupReq {
        MegaPupReq::from_bits(val)
    }
}
impl From<MegaPupReq> for u8 {
    #[inline(always)]
    fn from(val: MegaPupReq) -> u8 {
        MegaPupReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdram0Pge {
    #[doc = "FlexRAM PDRAM0 domain will keep power even if the CPU core is powered down."]
    PDRAM0_PGE_0 = 0x0,
    #[doc = "FlexRAM PDRAM0 domain will be powered down when the CPU core is powered down.."]
    PDRAM0_PGE_1 = 0x01,
}
impl Pdram0Pge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdram0Pge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdram0Pge {
    #[inline(always)]
    fn from(val: u8) -> Pdram0Pge {
        Pdram0Pge::from_bits(val)
    }
}
impl From<Pdram0Pge> for u8 {
    #[inline(always)]
    fn from(val: Pdram0Pge) -> u8 {
        Pdram0Pge::to_bits(val)
    }
}
