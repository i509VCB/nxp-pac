#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pbyp {
    #[doc = "Prescaler and glitch filter enable"]
    PBYP0 = 0x0,
    #[doc = "Prescaler and glitch filter bypass"]
    PBYP1 = 0x01,
}
impl Pbyp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pbyp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pbyp {
    #[inline(always)]
    fn from(val: u8) -> Pbyp {
        Pbyp::from_bits(val)
    }
}
impl From<Pbyp> for u8 {
    #[inline(always)]
    fn from(val: Pbyp) -> u8 {
        Pbyp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcs {
    #[doc = "Clock 0"]
    PCS00 = 0x0,
    #[doc = "Clock 1"]
    PCS01 = 0x01,
    #[doc = "Clock 2"]
    PCS10 = 0x02,
    #[doc = "Clock 3"]
    PCS11 = 0x03,
}
impl Pcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcs {
    #[inline(always)]
    fn from(val: u8) -> Pcs {
        Pcs::from_bits(val)
    }
}
impl From<Pcs> for u8 {
    #[inline(always)]
    fn from(val: Pcs) -> u8 {
        Pcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prescale {
    #[doc = "Prescaler divides the prescaler clock by 2; glitch filter does not support this configuration"]
    PRESCALE0000 = 0x0,
    #[doc = "Prescaler divides the prescaler clock by 4; glitch filter recognizes change on input pin after two rising clock edges"]
    PRESCALE0001 = 0x01,
    #[doc = "Prescaler divides the prescaler clock by 8; glitch filter recognizes change on input pin after four rising clock edges"]
    PRESCALE0010 = 0x02,
    #[doc = "Prescaler divides the prescaler clock by 16; glitch filter recognizes change on input pin after eight rising clock edges"]
    PRESCALE0011 = 0x03,
    #[doc = "Prescaler divides the prescaler clock by 32; glitch filter recognizes change on input pin after 16 rising clock edges"]
    PRESCALE0100 = 0x04,
    #[doc = "Prescaler divides the prescaler clock by 64; glitch filter recognizes change on input pin after 32 rising clock edges"]
    PRESCALE0101 = 0x05,
    #[doc = "Prescaler divides the prescaler clock by 128; glitch filter recognizes change on input pin after 64 rising clock edges"]
    PRESCALE0110 = 0x06,
    #[doc = "Prescaler divides the prescaler clock by 256; glitch filter recognizes change on input pin after 128 rising clock edges"]
    PRESCALE0111 = 0x07,
    #[doc = "Prescaler divides the prescaler clock by 512; glitch filter recognizes change on input pin after 256 rising clock edges"]
    PRESCALE1000 = 0x08,
    #[doc = "Prescaler divides the prescaler clock by 1024; glitch filter recognizes change on input pin after 512 rising clock edges"]
    PRESCALE1001 = 0x09,
    #[doc = "Prescaler divides the prescaler clock by 2048; glitch filter recognizes change on input pin after 1024 rising clock edges"]
    PRESCALE1010 = 0x0a,
    #[doc = "Prescaler divides the prescaler clock by 4096; glitch filter recognizes change on input pin after 2048 rising clock edges"]
    PRESCALE1011 = 0x0b,
    #[doc = "Prescaler divides the prescaler clock by 8192; glitch filter recognizes change on input pin after 4096 rising clock edges"]
    PRESCALE1100 = 0x0c,
    #[doc = "Prescaler divides the prescaler clock by 16,384; glitch filter recognizes change on input pin after 8192 rising clock edges"]
    PRESCALE1101 = 0x0d,
    #[doc = "Prescaler divides the prescaler clock by 32,768; glitch filter recognizes change on input pin after 16,384 rising clock edges"]
    PRESCALE1110 = 0x0e,
    #[doc = "Prescaler divides the prescaler clock by 65,536; glitch filter recognizes change on input pin after 32,768 rising clock edges"]
    PRESCALE1111 = 0x0f,
}
impl Prescale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prescale {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prescale {
    #[inline(always)]
    fn from(val: u8) -> Prescale {
        Prescale::from_bits(val)
    }
}
impl From<Prescale> for u8 {
    #[inline(always)]
    fn from(val: Prescale) -> u8 {
        Prescale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcf {
    #[doc = "CNR != (CMR + 1)"]
    TCF0 = 0x0,
    #[doc = "CNR = (CMR + 1)"]
    TCF1 = 0x01,
}
impl Tcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcf {
    #[inline(always)]
    fn from(val: u8) -> Tcf {
        Tcf::from_bits(val)
    }
}
impl From<Tcf> for u8 {
    #[inline(always)]
    fn from(val: Tcf) -> u8 {
        Tcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdre {
    #[doc = "Disable"]
    TRDE0 = 0x0,
    #[doc = "Enable"]
    TRDE1 = 0x01,
}
impl Tdre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdre {
    #[inline(always)]
    fn from(val: u8) -> Tdre {
        Tdre::from_bits(val)
    }
}
impl From<Tdre> for u8 {
    #[inline(always)]
    fn from(val: Tdre) -> u8 {
        Tdre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ten {
    #[doc = "Disable"]
    TEN0 = 0x0,
    #[doc = "Enable"]
    TEN1 = 0x01,
}
impl Ten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ten {
        unsafe { core::mem::transmute(val & 0x01) }
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
pub enum Tfc {
    #[doc = "Reset when TCF asserts"]
    TFC0 = 0x0,
    #[doc = "Reset on overflow"]
    TFC1 = 0x01,
}
impl Tfc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfc {
    #[inline(always)]
    fn from(val: u8) -> Tfc {
        Tfc::from_bits(val)
    }
}
impl From<Tfc> for u8 {
    #[inline(always)]
    fn from(val: Tfc) -> u8 {
        Tfc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tie {
    #[doc = "Disable"]
    TIE0 = 0x0,
    #[doc = "Enable"]
    TIE1 = 0x01,
}
impl Tie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie {
    #[inline(always)]
    fn from(val: u8) -> Tie {
        Tie::from_bits(val)
    }
}
impl From<Tie> for u8 {
    #[inline(always)]
    fn from(val: Tie) -> u8 {
        Tie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tms {
    #[doc = "Time Counter"]
    TMS0 = 0x0,
    #[doc = "Pulse Counter"]
    TMS1 = 0x01,
}
impl Tms {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tms {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tms {
    #[inline(always)]
    fn from(val: u8) -> Tms {
        Tms::from_bits(val)
    }
}
impl From<Tms> for u8 {
    #[inline(always)]
    fn from(val: Tms) -> u8 {
        Tms::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpp {
    #[doc = "Active-high"]
    TPP0 = 0x0,
    #[doc = "Active-low"]
    TPP1 = 0x01,
}
impl Tpp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpp {
    #[inline(always)]
    fn from(val: u8) -> Tpp {
        Tpp::from_bits(val)
    }
}
impl From<Tpp> for u8 {
    #[inline(always)]
    fn from(val: Tpp) -> u8 {
        Tpp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tps {
    #[doc = "Input 0"]
    TPS00 = 0x0,
    #[doc = "Input 1"]
    TPS01 = 0x01,
    #[doc = "Input 2"]
    TPS10 = 0x02,
    #[doc = "Input 3"]
    TPS11 = 0x03,
}
impl Tps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tps {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tps {
    #[inline(always)]
    fn from(val: u8) -> Tps {
        Tps::from_bits(val)
    }
}
impl From<Tps> for u8 {
    #[inline(always)]
    fn from(val: Tps) -> u8 {
        Tps::to_bits(val)
    }
}
