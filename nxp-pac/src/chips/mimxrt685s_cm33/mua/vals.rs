#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrFn {
    #[doc = "Clears the Fn bit in the SR register."]
    FN_0 = 0x0,
    #[doc = "Sets the Fn bit in the SR register."]
    FN_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl CrFn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrFn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrFn {
    #[inline(always)]
    fn from(val: u8) -> CrFn {
        CrFn::from_bits(val)
    }
}
impl From<CrFn> for u8 {
    #[inline(always)]
    fn from(val: CrFn) -> u8 {
        CrFn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ep {
    #[doc = "The MUA side event is not pending (default)."]
    EP_0 = 0x0,
    #[doc = "The MUA side event is pending."]
    EP_1 = 0x01,
}
impl Ep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ep {
    #[inline(always)]
    fn from(val: u8) -> Ep {
        Ep::from_bits(val)
    }
}
impl From<Ep> for u8 {
    #[inline(always)]
    fn from(val: Ep) -> u8 {
        Ep::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard features implemented"]
    pub const FEATURE_0: Self = Self(0x0);
    #[doc = "RAIP and RAIE register bits implemented on MUA side"]
    pub const FEATURE_1: Self = Self(0x01);
    #[doc = "MUA and MUB implemented with the same function. some bits in CR register are moved to CCR register."]
    pub const FEATURE_2: Self = Self(0x02);
    #[doc = "some sync logic are deleted for synchronized MUA and MUB. RAIP and RDIP monitor Core reset instead of MU reset. Add HRIP and MURIP and their interrupt enable bits on both sides. Delete RS bit. Add COO mode in PM state."]
    pub const FEATURE_4: Self = Self(0x04);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("FEATURE_0"),
            0x01 => f.write_str("FEATURE_1"),
            0x02 => f.write_str("FEATURE_2"),
            0x04 => f.write_str("FEATURE_4"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "FEATURE_0"),
            0x01 => defmt::write!(f, "FEATURE_1"),
            0x02 => defmt::write!(f, "FEATURE_2"),
            0x04 => defmt::write!(f, "FEATURE_4"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fup {
    #[doc = "No flags updated, initiated by the MUA, in progress (default)"]
    FUP_0 = 0x0,
    #[doc = "MUA initiated flags update, processing"]
    FUP_1 = 0x01,
}
impl Fup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fup {
    #[inline(always)]
    fn from(val: u8) -> Fup {
        Fup::from_bits(val)
    }
}
impl From<Fup> for u8 {
    #[inline(always)]
    fn from(val: Fup) -> u8 {
        Fup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gien {
    #[doc = "Disables MUA General Interrupt n. (default)"]
    GIEN_0 = 0x0,
    #[doc = "Enables MUA General Interrupt n."]
    GIEN_1 = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Gien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gien {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gien {
    #[inline(always)]
    fn from(val: u8) -> Gien {
        Gien::from_bits(val)
    }
}
impl From<Gien> for u8 {
    #[inline(always)]
    fn from(val: Gien) -> u8 {
        Gien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gipn {
    #[doc = "MUA general purpose interrupt n is not pending. (default)"]
    GIPN_0 = 0x0,
    #[doc = "MUA general purpose interrupt n is pending."]
    GIPN_1 = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Gipn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gipn {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gipn {
    #[inline(always)]
    fn from(val: u8) -> Gipn {
        Gipn::from_bits(val)
    }
}
impl From<Gipn> for u8 {
    #[inline(always)]
    fn from(val: Gipn) -> u8 {
        Gipn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Girn {
    #[doc = "MUA General Interrupt n is not requested to the MUB (default)."]
    GIRN_0 = 0x0,
    #[doc = "MUA General Interrupt n is requested to the MUB."]
    GIRN_1 = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Girn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Girn {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Girn {
    #[inline(always)]
    fn from(val: u8) -> Girn {
        Girn::from_bits(val)
    }
}
impl From<Girn> for u8 {
    #[inline(always)]
    fn from(val: Girn) -> u8 {
        Girn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mur {
    #[doc = "N/A. Self clearing bit (default)."]
    MUR_0 = 0x0,
    #[doc = "Asserts the MU reset."]
    MUR_1 = 0x01,
}
impl Mur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mur {
    #[inline(always)]
    fn from(val: u8) -> Mur {
        Mur::from_bits(val)
    }
}
impl From<Mur> for u8 {
    #[inline(always)]
    fn from(val: Mur) -> u8 {
        Mur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pm {
    #[doc = "The MUB processor is in Run Mode."]
    RUN = 0x0,
    #[doc = "The MUB processor is in WAIT Mode."]
    WAIT = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Pm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pm {
    #[inline(always)]
    fn from(val: u8) -> Pm {
        Pm::from_bits(val)
    }
}
impl From<Pm> for u8 {
    #[inline(always)]
    fn from(val: Pm) -> u8 {
        Pm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Raie {
    #[doc = "Disables Processor A General Purpose Interrupt 3 request due to Processor B reset assertion."]
    RAIE_0 = 0x0,
    #[doc = "Enables Processor A General Purpose Interrupt 3 request due to Processor B reset assertion."]
    RAIE_1 = 0x01,
}
impl Raie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Raie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Raie {
    #[inline(always)]
    fn from(val: u8) -> Raie {
        Raie::from_bits(val)
    }
}
impl From<Raie> for u8 {
    #[inline(always)]
    fn from(val: Raie) -> u8 {
        Raie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Raip {
    #[doc = "Processor B-side did not enter reset"]
    RAIP_0 = 0x0,
    #[doc = "Processor B-side entered reset"]
    RAIP_1 = 0x01,
}
impl Raip {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Raip {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Raip {
    #[inline(always)]
    fn from(val: u8) -> Raip {
        Raip::from_bits(val)
    }
}
impl From<Raip> for u8 {
    #[inline(always)]
    fn from(val: Raip) -> u8 {
        Raip::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdie {
    #[doc = "Disables Processor A General Purpose Interrupt 3 request due to Processor B reset de-assertion."]
    RDIE_0 = 0x0,
    #[doc = "Enables Processor A General Purpose Interrupt 3 request due to Processor B reset de-assertion."]
    RDIE_1 = 0x01,
}
impl Rdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdie {
    #[inline(always)]
    fn from(val: u8) -> Rdie {
        Rdie::from_bits(val)
    }
}
impl From<Rdie> for u8 {
    #[inline(always)]
    fn from(val: Rdie) -> u8 {
        Rdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdip {
    #[doc = "Processor B-side did not exit reset"]
    RDIP_0 = 0x0,
    #[doc = "Processor B-side exited from reset"]
    RDIP_1 = 0x01,
}
impl Rdip {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdip {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdip {
    #[inline(always)]
    fn from(val: u8) -> Rdip {
        Rdip::from_bits(val)
    }
}
impl From<Rdip> for u8 {
    #[inline(always)]
    fn from(val: Rdip) -> u8 {
        Rdip::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfn {
    #[doc = "MUA RRn register is not full (default)."]
    RFN_0 = 0x0,
    #[doc = "MUA RRn register has received data from MUB TRn register and is ready to be read by the MUA."]
    RFN_1 = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Rfn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfn {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfn {
    #[inline(always)]
    fn from(val: u8) -> Rfn {
        Rfn::from_bits(val)
    }
}
impl From<Rfn> for u8 {
    #[inline(always)]
    fn from(val: Rfn) -> u8 {
        Rfn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rien {
    #[doc = "Disables MUA Receive Interrupt n. (default)"]
    RIEN_0 = 0x0,
    #[doc = "Enables MUA Receive Interrupt n."]
    RIEN_1 = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Rien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rien {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rien {
    #[inline(always)]
    fn from(val: u8) -> Rien {
        Rien::from_bits(val)
    }
}
impl From<Rien> for u8 {
    #[inline(always)]
    fn from(val: Rien) -> u8 {
        Rien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rs {
    #[doc = "The MUB side of the MU is not in reset."]
    RS_0 = 0x0,
    #[doc = "The MUB side of the MU is in reset."]
    RS_1 = 0x01,
}
impl Rs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rs {
    #[inline(always)]
    fn from(val: u8) -> Rs {
        Rs::from_bits(val)
    }
}
impl From<Rs> for u8 {
    #[inline(always)]
    fn from(val: Rs) -> u8 {
        Rs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrFn {
    #[doc = "Fn bit in the CR register is written 0 (default)."]
    FN_0 = 0x0,
    #[doc = "Fn bit in the CR register is written 1."]
    FN_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SrFn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrFn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrFn {
    #[inline(always)]
    fn from(val: u8) -> SrFn {
        SrFn::from_bits(val)
    }
}
impl From<SrFn> for u8 {
    #[inline(always)]
    fn from(val: SrFn) -> u8 {
        SrFn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ten {
    #[doc = "MUA TRn register is not empty."]
    TEN_0 = 0x0,
    #[doc = "MUA TRn register is empty (default)."]
    TEN_1 = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Ten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ten {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ten {
    #[inline(always)]
    fn from(val: u8) -> Ten {
        Ten::from_bits(val)
    }
}
impl From<Ten> for u8 {
    #[inline(always)]
    fn from(val: Ten) -> u8 {
        Ten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tien {
    #[doc = "Disables MUA Transmit Interrupt n. (default)"]
    TIEN_0 = 0x0,
    #[doc = "Enables MUA Transmit Interrupt n."]
    TIEN_1 = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Tien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tien {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tien {
    #[inline(always)]
    fn from(val: u8) -> Tien {
        Tien::from_bits(val)
    }
}
impl From<Tien> for u8 {
    #[inline(always)]
    fn from(val: Tien) -> u8 {
        Tien::to_bits(val)
    }
}
