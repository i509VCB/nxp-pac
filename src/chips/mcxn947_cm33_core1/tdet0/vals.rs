#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Atcs0 {
    #[doc = "1 Hz prescaler clock"]
    FREQ_1_HZ = 0x0,
    #[doc = "64 Hz prescaler clock"]
    FREQ_64_HZ = 0x01,
}
impl Atcs0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Atcs0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Atcs0 {
    #[inline(always)]
    fn from(val: u8) -> Atcs0 {
        Atcs0::from_bits(val)
    }
}
impl From<Atcs0> for u8 {
    #[inline(always)]
    fn from(val: Atcs0) -> u8 {
        Atcs0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Atcs1 {
    #[doc = "1 Hz prescaler clock"]
    FREQ_1_HZ = 0x0,
    #[doc = "64 Hz prescaler clock"]
    FREQ_64_HZ = 0x01,
}
impl Atcs1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Atcs1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Atcs1 {
    #[inline(always)]
    fn from(val: u8) -> Atcs1 {
        Atcs1::from_bits(val)
    }
}
impl From<Atcs1> for u8 {
    #[inline(always)]
    fn from(val: Atcs1) -> u8 {
        Atcs1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Atl0 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Atl0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Atl0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Atl0 {
    #[inline(always)]
    fn from(val: u8) -> Atl0 {
        Atl0::from_bits(val)
    }
}
impl From<Atl0> for u8 {
    #[inline(always)]
    fn from(val: Atl0) -> u8 {
        Atl0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Atl1 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Atl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Atl1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Atl1 {
    #[inline(always)]
    fn from(val: u8) -> Atl1 {
        Atl1::from_bits(val)
    }
}
impl From<Atl1> for u8 {
    #[inline(always)]
    fn from(val: Atl1) -> u8 {
        Atl1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crl {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Crl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crl {
    #[inline(always)]
    fn from(val: u8) -> Crl {
        Crl::from_bits(val)
    }
}
impl From<Crl> for u8 {
    #[inline(always)]
    fn from(val: Crl) -> u8 {
        Crl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Den {
    #[doc = "Disables TDET clock and prescaler"]
    DISABLE = 0x0,
    #[doc = "Enables TDET clock and prescaler"]
    ENABLE = 0x01,
}
impl Den {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Den {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Den {
    #[inline(always)]
    fn from(val: u8) -> Den {
        Den::from_bits(val)
    }
}
impl From<Den> for u8 {
    #[inline(always)]
    fn from(val: Den) -> u8 {
        Den::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Distam {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Automatically disables the prescaler after tamper detection"]
    AUTO_DIS = 0x01,
}
impl Distam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Distam {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Distam {
    #[inline(always)]
    fn from(val: u8) -> Distam {
        Distam::from_bits(val)
    }
}
impl From<Distam> for u8 {
    #[inline(always)]
    fn from(val: Distam) -> u8 {
        Distam::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtf {
    #[doc = "TDET tampering not detected"]
    NO_DET = 0x0,
    #[doc = "TDET tampering detected"]
    DET = 0x01,
}
impl Dtf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtf {
    #[inline(always)]
    fn from(val: u8) -> Dtf {
        Dtf::from_bits(val)
    }
}
impl From<Dtf> for u8 {
    #[inline(always)]
    fn from(val: Dtf) -> u8 {
        Dtf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtie {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Dtie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtie {
    #[inline(always)]
    fn from(val: u8) -> Dtie {
        Dtie::from_bits(val)
    }
}
impl From<Dtie> for u8 {
    #[inline(always)]
    fn from(val: Dtie) -> u8 {
        Dtie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfe {
    #[doc = "Bypasses"]
    BYPASS = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Gfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfe {
    #[inline(always)]
    fn from(val: u8) -> Gfe {
        Gfe::from_bits(val)
    }
}
impl From<Gfe> for u8 {
    #[inline(always)]
    fn from(val: Gfe) -> u8 {
        Gfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl0 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Gfl0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl0 {
    #[inline(always)]
    fn from(val: u8) -> Gfl0 {
        Gfl0::from_bits(val)
    }
}
impl From<Gfl0> for u8 {
    #[inline(always)]
    fn from(val: Gfl0) -> u8 {
        Gfl0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl1 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Gfl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl1 {
    #[inline(always)]
    fn from(val: u8) -> Gfl1 {
        Gfl1::from_bits(val)
    }
}
impl From<Gfl1> for u8 {
    #[inline(always)]
    fn from(val: Gfl1) -> u8 {
        Gfl1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl2 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Gfl2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl2 {
    #[inline(always)]
    fn from(val: u8) -> Gfl2 {
        Gfl2::from_bits(val)
    }
}
impl From<Gfl2> for u8 {
    #[inline(always)]
    fn from(val: Gfl2) -> u8 {
        Gfl2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl3 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Gfl3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl3 {
    #[inline(always)]
    fn from(val: u8) -> Gfl3 {
        Gfl3::from_bits(val)
    }
}
impl From<Gfl3> for u8 {
    #[inline(always)]
    fn from(val: Gfl3) -> u8 {
        Gfl3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl4 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Gfl4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl4 {
    #[inline(always)]
    fn from(val: u8) -> Gfl4 {
        Gfl4::from_bits(val)
    }
}
impl From<Gfl4> for u8 {
    #[inline(always)]
    fn from(val: Gfl4) -> u8 {
        Gfl4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl5 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Gfl5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl5 {
    #[inline(always)]
    fn from(val: u8) -> Gfl5 {
        Gfl5::from_bits(val)
    }
}
impl From<Gfl5> for u8 {
    #[inline(always)]
    fn from(val: Gfl5) -> u8 {
        Gfl5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl6 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Gfl6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl6 {
    #[inline(always)]
    fn from(val: u8) -> Gfl6 {
        Gfl6::from_bits(val)
    }
}
impl From<Gfl6> for u8 {
    #[inline(always)]
    fn from(val: Gfl6) -> u8 {
        Gfl6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl7 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Gfl7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl7 {
    #[inline(always)]
    fn from(val: u8) -> Gfl7 {
        Gfl7::from_bits(val)
    }
}
impl From<Gfl7> for u8 {
    #[inline(always)]
    fn from(val: Gfl7) -> u8 {
        Gfl7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfp {
    #[doc = "512 Hz prescaler clock"]
    FREQ_512_HZ = 0x0,
    #[doc = "32.768 kHz clock"]
    FREQ_32_KHZ = 0x01,
}
impl Gfp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfp {
    #[inline(always)]
    fn from(val: u8) -> Gfp {
        Gfp::from_bits(val)
    }
}
impl From<Gfp> for u8 {
    #[inline(always)]
    fn from(val: Gfp) -> u8 {
        Gfp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iel {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Iel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iel {
    #[inline(always)]
    fn from(val: u8) -> Iel {
        Iel::from_bits(val)
    }
}
impl From<Iel> for u8 {
    #[inline(always)]
    fn from(val: Iel) -> u8 {
        Iel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lrl {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Lrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lrl {
    #[inline(always)]
    fn from(val: u8) -> Lrl {
        Lrl::from_bits(val)
    }
}
impl From<Lrl> for u8 {
    #[inline(always)]
    fn from(val: Lrl) -> u8 {
        Lrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdl {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Pdl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdl {
    #[inline(always)]
    fn from(val: u8) -> Pdl {
        Pdl::from_bits(val)
    }
}
impl From<Pdl> for u8 {
    #[inline(always)]
    fn from(val: Pdl) -> u8 {
        Pdl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ppl {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Ppl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ppl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ppl {
    #[inline(always)]
    fn from(val: u8) -> Ppl {
        Ppl::from_bits(val)
    }
}
impl From<Ppl> for u8 {
    #[inline(always)]
    fn from(val: Ppl) -> u8 {
        Ppl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srl {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Srl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srl {
    #[inline(always)]
    fn from(val: u8) -> Srl {
        Srl::from_bits(val)
    }
}
impl From<Srl> for u8 {
    #[inline(always)]
    fn from(val: Srl) -> u8 {
        Srl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swr {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Perform a software reset"]
    SW_RESET = 0x01,
}
impl Swr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swr {
    #[inline(always)]
    fn from(val: u8) -> Swr {
        Swr::from_bits(val)
    }
}
impl From<Swr> for u8 {
    #[inline(always)]
    fn from(val: Swr) -> u8 {
        Swr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Taf {
    #[doc = "Digital Tamper Flag (SR\\[DTF\\]) is clear or chip reset has not occurred after Digital Tamper Flag (SR\\[DTF\\]) was set."]
    NOT_OCCUR = 0x0,
    #[doc = "Chip reset has occurred after Digital Tamper Flag (SR\\[DTF\\]) was set."]
    OCCUR = 0x01,
}
impl Taf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Taf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Taf {
    #[inline(always)]
    fn from(val: u8) -> Taf {
        Taf::from_bits(val)
    }
}
impl From<Taf> for u8 {
    #[inline(always)]
    fn from(val: Taf) -> u8 {
        Taf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tel {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Tel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tel {
    #[inline(always)]
    fn from(val: u8) -> Tel {
        Tel::from_bits(val)
    }
}
impl From<Tel> for u8 {
    #[inline(always)]
    fn from(val: Tel) -> u8 {
        Tel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfsr {
    #[doc = "Do not force chip reset"]
    NO_RESET = 0x0,
    #[doc = "Force chip reset"]
    RESET = 0x01,
}
impl Tfsr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfsr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfsr {
    #[inline(always)]
    fn from(val: u8) -> Tfsr {
        Tfsr::from_bits(val)
    }
}
impl From<Tfsr> for u8 {
    #[inline(always)]
    fn from(val: Tfsr) -> u8 {
        Tfsr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tie0 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie0 {
    #[inline(always)]
    fn from(val: u8) -> Tie0 {
        Tie0::from_bits(val)
    }
}
impl From<Tie0> for u8 {
    #[inline(always)]
    fn from(val: Tie0) -> u8 {
        Tie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tie1 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie1 {
    #[inline(always)]
    fn from(val: u8) -> Tie1 {
        Tie1::from_bits(val)
    }
}
impl From<Tie1> for u8 {
    #[inline(always)]
    fn from(val: Tie1) -> u8 {
        Tie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tie2 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie2 {
    #[inline(always)]
    fn from(val: u8) -> Tie2 {
        Tie2::from_bits(val)
    }
}
impl From<Tie2> for u8 {
    #[inline(always)]
    fn from(val: Tie2) -> u8 {
        Tie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tie3 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie3 {
    #[inline(always)]
    fn from(val: u8) -> Tie3 {
        Tie3::from_bits(val)
    }
}
impl From<Tie3> for u8 {
    #[inline(always)]
    fn from(val: Tie3) -> u8 {
        Tie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tie4 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie4 {
    #[inline(always)]
    fn from(val: u8) -> Tie4 {
        Tie4::from_bits(val)
    }
}
impl From<Tie4> for u8 {
    #[inline(always)]
    fn from(val: Tie4) -> u8 {
        Tie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tie5 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie5 {
    #[inline(always)]
    fn from(val: u8) -> Tie5 {
        Tie5::from_bits(val)
    }
}
impl From<Tie5> for u8 {
    #[inline(always)]
    fn from(val: Tie5) -> u8 {
        Tie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tie6 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie6 {
    #[inline(always)]
    fn from(val: u8) -> Tie6 {
        Tie6::from_bits(val)
    }
}
impl From<Tie6> for u8 {
    #[inline(always)]
    fn from(val: Tie6) -> u8 {
        Tie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tie7 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie7 {
    #[inline(always)]
    fn from(val: u8) -> Tie7 {
        Tie7::from_bits(val)
    }
}
impl From<Tie7> for u8 {
    #[inline(always)]
    fn from(val: Tie7) -> u8 {
        Tie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tie8 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tie8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie8 {
    #[inline(always)]
    fn from(val: u8) -> Tie8 {
        Tie8::from_bits(val)
    }
}
impl From<Tie8> for u8 {
    #[inline(always)]
    fn from(val: Tie8) -> u8 {
        Tie8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tie9 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tie9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie9 {
    #[inline(always)]
    fn from(val: u8) -> Tie9 {
        Tie9::from_bits(val)
    }
}
impl From<Tie9> for u8 {
    #[inline(always)]
    fn from(val: Tie9) -> u8 {
        Tie9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tif0 {
    #[doc = "On-chip tamper not detected"]
    NO_DET = 0x0,
    #[doc = "On-chip tamper detected"]
    DET = 0x01,
}
impl Tif0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tif0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tif0 {
    #[inline(always)]
    fn from(val: u8) -> Tif0 {
        Tif0::from_bits(val)
    }
}
impl From<Tif0> for u8 {
    #[inline(always)]
    fn from(val: Tif0) -> u8 {
        Tif0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tif1 {
    #[doc = "On-chip tamper not detected"]
    NO_DET = 0x0,
    #[doc = "On-chip tamper detected"]
    DET = 0x01,
}
impl Tif1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tif1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tif1 {
    #[inline(always)]
    fn from(val: u8) -> Tif1 {
        Tif1::from_bits(val)
    }
}
impl From<Tif1> for u8 {
    #[inline(always)]
    fn from(val: Tif1) -> u8 {
        Tif1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tif2 {
    #[doc = "On-chip tamper not detected"]
    NO_DET = 0x0,
    #[doc = "On-chip tamper detected"]
    DET = 0x01,
}
impl Tif2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tif2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tif2 {
    #[inline(always)]
    fn from(val: u8) -> Tif2 {
        Tif2::from_bits(val)
    }
}
impl From<Tif2> for u8 {
    #[inline(always)]
    fn from(val: Tif2) -> u8 {
        Tif2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tif3 {
    #[doc = "On-chip tamper not detected"]
    NO_DET = 0x0,
    #[doc = "On-chip tamper detected"]
    DET = 0x01,
}
impl Tif3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tif3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tif3 {
    #[inline(always)]
    fn from(val: u8) -> Tif3 {
        Tif3::from_bits(val)
    }
}
impl From<Tif3> for u8 {
    #[inline(always)]
    fn from(val: Tif3) -> u8 {
        Tif3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tif4 {
    #[doc = "On-chip tamper not detected"]
    NO_DET = 0x0,
    #[doc = "On-chip tamper detected"]
    DET = 0x01,
}
impl Tif4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tif4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tif4 {
    #[inline(always)]
    fn from(val: u8) -> Tif4 {
        Tif4::from_bits(val)
    }
}
impl From<Tif4> for u8 {
    #[inline(always)]
    fn from(val: Tif4) -> u8 {
        Tif4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tif5 {
    #[doc = "On-chip tamper not detected"]
    NO_DET = 0x0,
    #[doc = "On-chip tamper detected"]
    DET = 0x01,
}
impl Tif5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tif5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tif5 {
    #[inline(always)]
    fn from(val: u8) -> Tif5 {
        Tif5::from_bits(val)
    }
}
impl From<Tif5> for u8 {
    #[inline(always)]
    fn from(val: Tif5) -> u8 {
        Tif5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tif6 {
    #[doc = "On-chip tamper not detected"]
    NO_DET = 0x0,
    #[doc = "On-chip tamper detected"]
    DET = 0x01,
}
impl Tif6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tif6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tif6 {
    #[inline(always)]
    fn from(val: u8) -> Tif6 {
        Tif6::from_bits(val)
    }
}
impl From<Tif6> for u8 {
    #[inline(always)]
    fn from(val: Tif6) -> u8 {
        Tif6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tif7 {
    #[doc = "On-chip tamper not detected"]
    NO_DET = 0x0,
    #[doc = "On-chip tamper detected"]
    DET = 0x01,
}
impl Tif7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tif7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tif7 {
    #[inline(always)]
    fn from(val: u8) -> Tif7 {
        Tif7::from_bits(val)
    }
}
impl From<Tif7> for u8 {
    #[inline(always)]
    fn from(val: Tif7) -> u8 {
        Tif7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tif8 {
    #[doc = "On-chip tamper not detected"]
    NO_DET = 0x0,
    #[doc = "On-chip tamper detected"]
    DET = 0x01,
}
impl Tif8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tif8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tif8 {
    #[inline(always)]
    fn from(val: u8) -> Tif8 {
        Tif8::from_bits(val)
    }
}
impl From<Tif8> for u8 {
    #[inline(always)]
    fn from(val: Tif8) -> u8 {
        Tif8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tif9 {
    #[doc = "On-chip tamper not detected"]
    NO_DET = 0x0,
    #[doc = "On-chip tamper detected"]
    DET = 0x01,
}
impl Tif9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tif9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tif9 {
    #[inline(always)]
    fn from(val: u8) -> Tif9 {
        Tif9::from_bits(val)
    }
}
impl From<Tif9> for u8 {
    #[inline(always)]
    fn from(val: Tif9) -> u8 {
        Tif9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tiie0 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tiie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tiie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tiie0 {
    #[inline(always)]
    fn from(val: u8) -> Tiie0 {
        Tiie0::from_bits(val)
    }
}
impl From<Tiie0> for u8 {
    #[inline(always)]
    fn from(val: Tiie0) -> u8 {
        Tiie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tiie1 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tiie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tiie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tiie1 {
    #[inline(always)]
    fn from(val: u8) -> Tiie1 {
        Tiie1::from_bits(val)
    }
}
impl From<Tiie1> for u8 {
    #[inline(always)]
    fn from(val: Tiie1) -> u8 {
        Tiie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tiie2 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tiie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tiie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tiie2 {
    #[inline(always)]
    fn from(val: u8) -> Tiie2 {
        Tiie2::from_bits(val)
    }
}
impl From<Tiie2> for u8 {
    #[inline(always)]
    fn from(val: Tiie2) -> u8 {
        Tiie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tiie3 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tiie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tiie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tiie3 {
    #[inline(always)]
    fn from(val: u8) -> Tiie3 {
        Tiie3::from_bits(val)
    }
}
impl From<Tiie3> for u8 {
    #[inline(always)]
    fn from(val: Tiie3) -> u8 {
        Tiie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tiie4 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tiie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tiie4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tiie4 {
    #[inline(always)]
    fn from(val: u8) -> Tiie4 {
        Tiie4::from_bits(val)
    }
}
impl From<Tiie4> for u8 {
    #[inline(always)]
    fn from(val: Tiie4) -> u8 {
        Tiie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tiie5 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tiie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tiie5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tiie5 {
    #[inline(always)]
    fn from(val: u8) -> Tiie5 {
        Tiie5::from_bits(val)
    }
}
impl From<Tiie5> for u8 {
    #[inline(always)]
    fn from(val: Tiie5) -> u8 {
        Tiie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tiie6 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tiie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tiie6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tiie6 {
    #[inline(always)]
    fn from(val: u8) -> Tiie6 {
        Tiie6::from_bits(val)
    }
}
impl From<Tiie6> for u8 {
    #[inline(always)]
    fn from(val: Tiie6) -> u8 {
        Tiie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tiie7 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tiie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tiie7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tiie7 {
    #[inline(always)]
    fn from(val: u8) -> Tiie7 {
        Tiie7::from_bits(val)
    }
}
impl From<Tiie7> for u8 {
    #[inline(always)]
    fn from(val: Tiie7) -> u8 {
        Tiie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tiie8 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tiie8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tiie8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tiie8 {
    #[inline(always)]
    fn from(val: u8) -> Tiie8 {
        Tiie8::from_bits(val)
    }
}
impl From<Tiie8> for u8 {
    #[inline(always)]
    fn from(val: Tiie8) -> u8 {
        Tiie8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tiie9 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tiie9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tiie9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tiie9 {
    #[inline(always)]
    fn from(val: u8) -> Tiie9 {
        Tiie9::from_bits(val)
    }
}
impl From<Tiie9> for u8 {
    #[inline(always)]
    fn from(val: Tiie9) -> u8 {
        Tiie9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpd0 {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output and drives the inverse of the expected value (tamper pin is asserted)"]
    OUTPUT = 0x01,
}
impl Tpd0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpd0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpd0 {
    #[inline(always)]
    fn from(val: u8) -> Tpd0 {
        Tpd0::from_bits(val)
    }
}
impl From<Tpd0> for u8 {
    #[inline(always)]
    fn from(val: Tpd0) -> u8 {
        Tpd0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpd1 {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output and drives the inverse of the expected value (tamper pin is asserted)"]
    OUTPUT = 0x01,
}
impl Tpd1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpd1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpd1 {
    #[inline(always)]
    fn from(val: u8) -> Tpd1 {
        Tpd1::from_bits(val)
    }
}
impl From<Tpd1> for u8 {
    #[inline(always)]
    fn from(val: Tpd1) -> u8 {
        Tpd1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpd2 {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output and drives the inverse of the expected value (tamper pin is asserted)"]
    OUTPUT = 0x01,
}
impl Tpd2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpd2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpd2 {
    #[inline(always)]
    fn from(val: u8) -> Tpd2 {
        Tpd2::from_bits(val)
    }
}
impl From<Tpd2> for u8 {
    #[inline(always)]
    fn from(val: Tpd2) -> u8 {
        Tpd2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpd3 {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output and drives the inverse of the expected value (tamper pin is asserted)"]
    OUTPUT = 0x01,
}
impl Tpd3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpd3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpd3 {
    #[inline(always)]
    fn from(val: u8) -> Tpd3 {
        Tpd3::from_bits(val)
    }
}
impl From<Tpd3> for u8 {
    #[inline(always)]
    fn from(val: Tpd3) -> u8 {
        Tpd3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpd4 {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output and drives the inverse of the expected value (tamper pin is asserted)"]
    OUTPUT = 0x01,
}
impl Tpd4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpd4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpd4 {
    #[inline(always)]
    fn from(val: u8) -> Tpd4 {
        Tpd4::from_bits(val)
    }
}
impl From<Tpd4> for u8 {
    #[inline(always)]
    fn from(val: Tpd4) -> u8 {
        Tpd4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpd5 {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output and drives the inverse of the expected value (tamper pin is asserted)"]
    OUTPUT = 0x01,
}
impl Tpd5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpd5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpd5 {
    #[inline(always)]
    fn from(val: u8) -> Tpd5 {
        Tpd5::from_bits(val)
    }
}
impl From<Tpd5> for u8 {
    #[inline(always)]
    fn from(val: Tpd5) -> u8 {
        Tpd5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpd6 {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output and drives the inverse of the expected value (tamper pin is asserted)"]
    OUTPUT = 0x01,
}
impl Tpd6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpd6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpd6 {
    #[inline(always)]
    fn from(val: u8) -> Tpd6 {
        Tpd6::from_bits(val)
    }
}
impl From<Tpd6> for u8 {
    #[inline(always)]
    fn from(val: Tpd6) -> u8 {
        Tpd6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpd7 {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output and drives the inverse of the expected value (tamper pin is asserted)"]
    OUTPUT = 0x01,
}
impl Tpd7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpd7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpd7 {
    #[inline(always)]
    fn from(val: u8) -> Tpd7 {
        Tpd7::from_bits(val)
    }
}
impl From<Tpd7> for u8 {
    #[inline(always)]
    fn from(val: Tpd7) -> u8 {
        Tpd7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpe {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tpe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpe {
    #[inline(always)]
    fn from(val: u8) -> Tpe {
        Tpe::from_bits(val)
    }
}
impl From<Tpe> for u8 {
    #[inline(always)]
    fn from(val: Tpe) -> u8 {
        Tpe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpe0 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tpe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpe0 {
    #[inline(always)]
    fn from(val: u8) -> Tpe0 {
        Tpe0::from_bits(val)
    }
}
impl From<Tpe0> for u8 {
    #[inline(always)]
    fn from(val: Tpe0) -> u8 {
        Tpe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpe1 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tpe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpe1 {
    #[inline(always)]
    fn from(val: u8) -> Tpe1 {
        Tpe1::from_bits(val)
    }
}
impl From<Tpe1> for u8 {
    #[inline(always)]
    fn from(val: Tpe1) -> u8 {
        Tpe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpe2 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tpe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpe2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpe2 {
    #[inline(always)]
    fn from(val: u8) -> Tpe2 {
        Tpe2::from_bits(val)
    }
}
impl From<Tpe2> for u8 {
    #[inline(always)]
    fn from(val: Tpe2) -> u8 {
        Tpe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpe3 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tpe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpe3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpe3 {
    #[inline(always)]
    fn from(val: u8) -> Tpe3 {
        Tpe3::from_bits(val)
    }
}
impl From<Tpe3> for u8 {
    #[inline(always)]
    fn from(val: Tpe3) -> u8 {
        Tpe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpe4 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tpe4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpe4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpe4 {
    #[inline(always)]
    fn from(val: u8) -> Tpe4 {
        Tpe4::from_bits(val)
    }
}
impl From<Tpe4> for u8 {
    #[inline(always)]
    fn from(val: Tpe4) -> u8 {
        Tpe4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpe5 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tpe5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpe5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpe5 {
    #[inline(always)]
    fn from(val: u8) -> Tpe5 {
        Tpe5::from_bits(val)
    }
}
impl From<Tpe5> for u8 {
    #[inline(always)]
    fn from(val: Tpe5) -> u8 {
        Tpe5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpe6 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tpe6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpe6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpe6 {
    #[inline(always)]
    fn from(val: u8) -> Tpe6 {
        Tpe6::from_bits(val)
    }
}
impl From<Tpe6> for u8 {
    #[inline(always)]
    fn from(val: Tpe6) -> u8 {
        Tpe6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpe7 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tpe7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpe7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpe7 {
    #[inline(always)]
    fn from(val: u8) -> Tpe7 {
        Tpe7::from_bits(val)
    }
}
impl From<Tpe7> for u8 {
    #[inline(always)]
    fn from(val: Tpe7) -> u8 {
        Tpe7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpex {
    #[doc = "Zero/passive tamper"]
    ZERO = 0x0,
    #[doc = "Active Tamper 0 output"]
    VAL_TAMP0 = 0x01,
    #[doc = "Active Tamper 1 output"]
    VAL_TAMP1 = 0x02,
    #[doc = "Active Tamper 0 output XORed with Active Tamper 1 output"]
    TAMP0_XOR_TAMP1 = 0x03,
}
impl Tpex {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpex {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpex {
    #[inline(always)]
    fn from(val: u8) -> Tpex {
        Tpex::from_bits(val)
    }
}
impl From<Tpex> for u8 {
    #[inline(always)]
    fn from(val: Tpex) -> u8 {
        Tpex::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpf0 {
    #[doc = "Pin tamper not detected"]
    NO_DET = 0x0,
    #[doc = "Pin tamper detected"]
    DET = 0x01,
}
impl Tpf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpf0 {
    #[inline(always)]
    fn from(val: u8) -> Tpf0 {
        Tpf0::from_bits(val)
    }
}
impl From<Tpf0> for u8 {
    #[inline(always)]
    fn from(val: Tpf0) -> u8 {
        Tpf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpf1 {
    #[doc = "Pin tamper not detected"]
    NO_DET = 0x0,
    #[doc = "Pin tamper detected"]
    DET = 0x01,
}
impl Tpf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpf1 {
    #[inline(always)]
    fn from(val: u8) -> Tpf1 {
        Tpf1::from_bits(val)
    }
}
impl From<Tpf1> for u8 {
    #[inline(always)]
    fn from(val: Tpf1) -> u8 {
        Tpf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpf2 {
    #[doc = "Pin tamper not detected"]
    NO_DET = 0x0,
    #[doc = "Pin tamper detected"]
    DET = 0x01,
}
impl Tpf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpf2 {
    #[inline(always)]
    fn from(val: u8) -> Tpf2 {
        Tpf2::from_bits(val)
    }
}
impl From<Tpf2> for u8 {
    #[inline(always)]
    fn from(val: Tpf2) -> u8 {
        Tpf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpf3 {
    #[doc = "Pin tamper not detected"]
    NO_DET = 0x0,
    #[doc = "Pin tamper detected"]
    DET = 0x01,
}
impl Tpf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpf3 {
    #[inline(always)]
    fn from(val: u8) -> Tpf3 {
        Tpf3::from_bits(val)
    }
}
impl From<Tpf3> for u8 {
    #[inline(always)]
    fn from(val: Tpf3) -> u8 {
        Tpf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpf4 {
    #[doc = "Pin tamper not detected"]
    NO_DET = 0x0,
    #[doc = "Pin tamper detected"]
    DET = 0x01,
}
impl Tpf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpf4 {
    #[inline(always)]
    fn from(val: u8) -> Tpf4 {
        Tpf4::from_bits(val)
    }
}
impl From<Tpf4> for u8 {
    #[inline(always)]
    fn from(val: Tpf4) -> u8 {
        Tpf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpf5 {
    #[doc = "Pin tamper not detected"]
    NO_DET = 0x0,
    #[doc = "Pin tamper detected"]
    DET = 0x01,
}
impl Tpf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpf5 {
    #[inline(always)]
    fn from(val: u8) -> Tpf5 {
        Tpf5::from_bits(val)
    }
}
impl From<Tpf5> for u8 {
    #[inline(always)]
    fn from(val: Tpf5) -> u8 {
        Tpf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpf6 {
    #[doc = "Pin tamper not detected"]
    NO_DET = 0x0,
    #[doc = "Pin tamper detected"]
    DET = 0x01,
}
impl Tpf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpf6 {
    #[inline(always)]
    fn from(val: u8) -> Tpf6 {
        Tpf6::from_bits(val)
    }
}
impl From<Tpf6> for u8 {
    #[inline(always)]
    fn from(val: Tpf6) -> u8 {
        Tpf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpf7 {
    #[doc = "Pin tamper not detected"]
    NO_DET = 0x0,
    #[doc = "Pin tamper detected"]
    DET = 0x01,
}
impl Tpf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpf7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpf7 {
    #[inline(always)]
    fn from(val: u8) -> Tpf7 {
        Tpf7::from_bits(val)
    }
}
impl From<Tpf7> for u8 {
    #[inline(always)]
    fn from(val: Tpf7) -> u8 {
        Tpf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpid0 {
    #[doc = "Zero"]
    ZERO = 0x0,
    #[doc = "One"]
    ONE = 0x01,
}
impl Tpid0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpid0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpid0 {
    #[inline(always)]
    fn from(val: u8) -> Tpid0 {
        Tpid0::from_bits(val)
    }
}
impl From<Tpid0> for u8 {
    #[inline(always)]
    fn from(val: Tpid0) -> u8 {
        Tpid0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpid1 {
    #[doc = "Zero"]
    ZERO = 0x0,
    #[doc = "One"]
    ONE = 0x01,
}
impl Tpid1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpid1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpid1 {
    #[inline(always)]
    fn from(val: u8) -> Tpid1 {
        Tpid1::from_bits(val)
    }
}
impl From<Tpid1> for u8 {
    #[inline(always)]
    fn from(val: Tpid1) -> u8 {
        Tpid1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpid2 {
    #[doc = "Zero"]
    ZERO = 0x0,
    #[doc = "One"]
    ONE = 0x01,
}
impl Tpid2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpid2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpid2 {
    #[inline(always)]
    fn from(val: u8) -> Tpid2 {
        Tpid2::from_bits(val)
    }
}
impl From<Tpid2> for u8 {
    #[inline(always)]
    fn from(val: Tpid2) -> u8 {
        Tpid2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpid3 {
    #[doc = "Zero"]
    ZERO = 0x0,
    #[doc = "One"]
    ONE = 0x01,
}
impl Tpid3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpid3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpid3 {
    #[inline(always)]
    fn from(val: u8) -> Tpid3 {
        Tpid3::from_bits(val)
    }
}
impl From<Tpid3> for u8 {
    #[inline(always)]
    fn from(val: Tpid3) -> u8 {
        Tpid3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpid4 {
    #[doc = "Zero"]
    ZERO = 0x0,
    #[doc = "One"]
    ONE = 0x01,
}
impl Tpid4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpid4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpid4 {
    #[inline(always)]
    fn from(val: u8) -> Tpid4 {
        Tpid4::from_bits(val)
    }
}
impl From<Tpid4> for u8 {
    #[inline(always)]
    fn from(val: Tpid4) -> u8 {
        Tpid4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpid5 {
    #[doc = "Zero"]
    ZERO = 0x0,
    #[doc = "One"]
    ONE = 0x01,
}
impl Tpid5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpid5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpid5 {
    #[inline(always)]
    fn from(val: u8) -> Tpid5 {
        Tpid5::from_bits(val)
    }
}
impl From<Tpid5> for u8 {
    #[inline(always)]
    fn from(val: Tpid5) -> u8 {
        Tpid5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpid6 {
    #[doc = "Zero"]
    ZERO = 0x0,
    #[doc = "One"]
    ONE = 0x01,
}
impl Tpid6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpid6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpid6 {
    #[inline(always)]
    fn from(val: u8) -> Tpid6 {
        Tpid6::from_bits(val)
    }
}
impl From<Tpid6> for u8 {
    #[inline(always)]
    fn from(val: Tpid6) -> u8 {
        Tpid6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpid7 {
    #[doc = "Zero"]
    ZERO = 0x0,
    #[doc = "One"]
    ONE = 0x01,
}
impl Tpid7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpid7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpid7 {
    #[inline(always)]
    fn from(val: u8) -> Tpid7 {
        Tpid7::from_bits(val)
    }
}
impl From<Tpid7> for u8 {
    #[inline(always)]
    fn from(val: Tpid7) -> u8 {
        Tpid7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpie0 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tpie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpie0 {
    #[inline(always)]
    fn from(val: u8) -> Tpie0 {
        Tpie0::from_bits(val)
    }
}
impl From<Tpie0> for u8 {
    #[inline(always)]
    fn from(val: Tpie0) -> u8 {
        Tpie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpie1 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tpie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpie1 {
    #[inline(always)]
    fn from(val: u8) -> Tpie1 {
        Tpie1::from_bits(val)
    }
}
impl From<Tpie1> for u8 {
    #[inline(always)]
    fn from(val: Tpie1) -> u8 {
        Tpie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpie2 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tpie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpie2 {
    #[inline(always)]
    fn from(val: u8) -> Tpie2 {
        Tpie2::from_bits(val)
    }
}
impl From<Tpie2> for u8 {
    #[inline(always)]
    fn from(val: Tpie2) -> u8 {
        Tpie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpie3 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tpie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpie3 {
    #[inline(always)]
    fn from(val: u8) -> Tpie3 {
        Tpie3::from_bits(val)
    }
}
impl From<Tpie3> for u8 {
    #[inline(always)]
    fn from(val: Tpie3) -> u8 {
        Tpie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpie4 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tpie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpie4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpie4 {
    #[inline(always)]
    fn from(val: u8) -> Tpie4 {
        Tpie4::from_bits(val)
    }
}
impl From<Tpie4> for u8 {
    #[inline(always)]
    fn from(val: Tpie4) -> u8 {
        Tpie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpie5 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tpie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpie5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpie5 {
    #[inline(always)]
    fn from(val: u8) -> Tpie5 {
        Tpie5::from_bits(val)
    }
}
impl From<Tpie5> for u8 {
    #[inline(always)]
    fn from(val: Tpie5) -> u8 {
        Tpie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpie6 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tpie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpie6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpie6 {
    #[inline(always)]
    fn from(val: u8) -> Tpie6 {
        Tpie6::from_bits(val)
    }
}
impl From<Tpie6> for u8 {
    #[inline(always)]
    fn from(val: Tpie6) -> u8 {
        Tpie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpie7 {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Tpie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpie7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpie7 {
    #[inline(always)]
    fn from(val: u8) -> Tpie7 {
        Tpie7::from_bits(val)
    }
}
impl From<Tpie7> for u8 {
    #[inline(always)]
    fn from(val: Tpie7) -> u8 {
        Tpie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpod0 {
    #[doc = "Zero"]
    ZERO = 0x0,
    #[doc = "One"]
    ONE = 0x01,
}
impl Tpod0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpod0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpod0 {
    #[inline(always)]
    fn from(val: u8) -> Tpod0 {
        Tpod0::from_bits(val)
    }
}
impl From<Tpod0> for u8 {
    #[inline(always)]
    fn from(val: Tpod0) -> u8 {
        Tpod0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpod1 {
    #[doc = "Zero"]
    ZERO = 0x0,
    #[doc = "One"]
    ONE = 0x01,
}
impl Tpod1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpod1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpod1 {
    #[inline(always)]
    fn from(val: u8) -> Tpod1 {
        Tpod1::from_bits(val)
    }
}
impl From<Tpod1> for u8 {
    #[inline(always)]
    fn from(val: Tpod1) -> u8 {
        Tpod1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpod2 {
    #[doc = "Zero"]
    ZERO = 0x0,
    #[doc = "One"]
    ONE = 0x01,
}
impl Tpod2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpod2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpod2 {
    #[inline(always)]
    fn from(val: u8) -> Tpod2 {
        Tpod2::from_bits(val)
    }
}
impl From<Tpod2> for u8 {
    #[inline(always)]
    fn from(val: Tpod2) -> u8 {
        Tpod2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpod3 {
    #[doc = "Zero"]
    ZERO = 0x0,
    #[doc = "One"]
    ONE = 0x01,
}
impl Tpod3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpod3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpod3 {
    #[inline(always)]
    fn from(val: u8) -> Tpod3 {
        Tpod3::from_bits(val)
    }
}
impl From<Tpod3> for u8 {
    #[inline(always)]
    fn from(val: Tpod3) -> u8 {
        Tpod3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpod4 {
    #[doc = "Zero"]
    ZERO = 0x0,
    #[doc = "One"]
    ONE = 0x01,
}
impl Tpod4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpod4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpod4 {
    #[inline(always)]
    fn from(val: u8) -> Tpod4 {
        Tpod4::from_bits(val)
    }
}
impl From<Tpod4> for u8 {
    #[inline(always)]
    fn from(val: Tpod4) -> u8 {
        Tpod4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpod5 {
    #[doc = "Zero"]
    ZERO = 0x0,
    #[doc = "One"]
    ONE = 0x01,
}
impl Tpod5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpod5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpod5 {
    #[inline(always)]
    fn from(val: u8) -> Tpod5 {
        Tpod5::from_bits(val)
    }
}
impl From<Tpod5> for u8 {
    #[inline(always)]
    fn from(val: Tpod5) -> u8 {
        Tpod5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpod6 {
    #[doc = "Zero"]
    ZERO = 0x0,
    #[doc = "One"]
    ONE = 0x01,
}
impl Tpod6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpod6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpod6 {
    #[inline(always)]
    fn from(val: u8) -> Tpod6 {
        Tpod6::from_bits(val)
    }
}
impl From<Tpod6> for u8 {
    #[inline(always)]
    fn from(val: Tpod6) -> u8 {
        Tpod6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpod7 {
    #[doc = "Zero"]
    ZERO = 0x0,
    #[doc = "One"]
    ONE = 0x01,
}
impl Tpod7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpod7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpod7 {
    #[inline(always)]
    fn from(val: u8) -> Tpod7 {
        Tpod7::from_bits(val)
    }
}
impl From<Tpod7> for u8 {
    #[inline(always)]
    fn from(val: Tpod7) -> u8 {
        Tpod7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpp0 {
    #[doc = "Not inverted"]
    NO_INVERT = 0x0,
    #[doc = "Inverted"]
    INVERT = 0x01,
}
impl Tpp0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpp0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpp0 {
    #[inline(always)]
    fn from(val: u8) -> Tpp0 {
        Tpp0::from_bits(val)
    }
}
impl From<Tpp0> for u8 {
    #[inline(always)]
    fn from(val: Tpp0) -> u8 {
        Tpp0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpp1 {
    #[doc = "Not inverted"]
    NO_INVERT = 0x0,
    #[doc = "Inverted"]
    INVERT = 0x01,
}
impl Tpp1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpp1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpp1 {
    #[inline(always)]
    fn from(val: u8) -> Tpp1 {
        Tpp1::from_bits(val)
    }
}
impl From<Tpp1> for u8 {
    #[inline(always)]
    fn from(val: Tpp1) -> u8 {
        Tpp1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpp2 {
    #[doc = "Not inverted"]
    NO_INVERT = 0x0,
    #[doc = "Inverted"]
    INVERT = 0x01,
}
impl Tpp2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpp2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpp2 {
    #[inline(always)]
    fn from(val: u8) -> Tpp2 {
        Tpp2::from_bits(val)
    }
}
impl From<Tpp2> for u8 {
    #[inline(always)]
    fn from(val: Tpp2) -> u8 {
        Tpp2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpp3 {
    #[doc = "Not inverted"]
    NO_INVERT = 0x0,
    #[doc = "Inverted"]
    INVERT = 0x01,
}
impl Tpp3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpp3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpp3 {
    #[inline(always)]
    fn from(val: u8) -> Tpp3 {
        Tpp3::from_bits(val)
    }
}
impl From<Tpp3> for u8 {
    #[inline(always)]
    fn from(val: Tpp3) -> u8 {
        Tpp3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpp4 {
    #[doc = "Not inverted"]
    NO_INVERT = 0x0,
    #[doc = "Inverted"]
    INVERT = 0x01,
}
impl Tpp4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpp4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpp4 {
    #[inline(always)]
    fn from(val: u8) -> Tpp4 {
        Tpp4::from_bits(val)
    }
}
impl From<Tpp4> for u8 {
    #[inline(always)]
    fn from(val: Tpp4) -> u8 {
        Tpp4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpp5 {
    #[doc = "Not inverted"]
    NO_INVERT = 0x0,
    #[doc = "Inverted"]
    INVERT = 0x01,
}
impl Tpp5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpp5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpp5 {
    #[inline(always)]
    fn from(val: u8) -> Tpp5 {
        Tpp5::from_bits(val)
    }
}
impl From<Tpp5> for u8 {
    #[inline(always)]
    fn from(val: Tpp5) -> u8 {
        Tpp5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpp6 {
    #[doc = "Not inverted"]
    NO_INVERT = 0x0,
    #[doc = "Inverted"]
    INVERT = 0x01,
}
impl Tpp6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpp6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpp6 {
    #[inline(always)]
    fn from(val: u8) -> Tpp6 {
        Tpp6::from_bits(val)
    }
}
impl From<Tpp6> for u8 {
    #[inline(always)]
    fn from(val: Tpp6) -> u8 {
        Tpp6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpp7 {
    #[doc = "Not inverted"]
    NO_INVERT = 0x0,
    #[doc = "Inverted"]
    INVERT = 0x01,
}
impl Tpp7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpp7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpp7 {
    #[inline(always)]
    fn from(val: u8) -> Tpp7 {
        Tpp7::from_bits(val)
    }
}
impl From<Tpp7> for u8 {
    #[inline(always)]
    fn from(val: Tpp7) -> u8 {
        Tpp7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tps {
    #[doc = "Asserts"]
    ASSERT = 0x0,
    #[doc = "Negates"]
    NEGATE = 0x01,
}
impl Tps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tps {
        unsafe { core::mem::transmute(val & 0x01) }
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpsf {
    #[doc = "Every 8 cycles"]
    CYCLES_8 = 0x0,
    #[doc = "Every 32 cycles"]
    CYCLES_32 = 0x01,
    #[doc = "Every 128 cycles"]
    CYCLES_128 = 0x02,
    #[doc = "Every 512 cycles"]
    CYCLES_512 = 0x03,
}
impl Tpsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpsf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpsf {
    #[inline(always)]
    fn from(val: u8) -> Tpsf {
        Tpsf::from_bits(val)
    }
}
impl From<Tpsf> for u8 {
    #[inline(always)]
    fn from(val: Tpsf) -> u8 {
        Tpsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpsw {
    #[doc = "Continuous monitoring, pin sampling disabled"]
    DISABLE = 0x0,
    #[doc = "2 cycles for pull enable and 1 cycle for input buffer enable"]
    CYCLES_2 = 0x01,
    #[doc = "4 cycles for pull enable and 2 cycles for input buffer enable"]
    CYCLES_4 = 0x02,
    #[doc = "8 cycles for pull enable and 4 cycles for input buffer enable"]
    CYCLES_8 = 0x03,
}
impl Tpsw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpsw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpsw {
    #[inline(always)]
    fn from(val: u8) -> Tpsw {
        Tpsw::from_bits(val)
    }
}
impl From<Tpsw> for u8 {
    #[inline(always)]
    fn from(val: Tpsw) -> u8 {
        Tpsw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsl {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Tsl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsl {
    #[inline(always)]
    fn from(val: u8) -> Tsl {
        Tsl::from_bits(val)
    }
}
impl From<Tsl> for u8 {
    #[inline(always)]
    fn from(val: Tsl) -> u8 {
        Tsl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Um {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Allows the clearing of interrupts"]
    CLEAR_INTS = 0x01,
}
impl Um {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Um {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Um {
    #[inline(always)]
    fn from(val: u8) -> Um {
        Um::from_bits(val)
    }
}
impl From<Um> for u8 {
    #[inline(always)]
    fn from(val: Um) -> u8 {
        Um::to_bits(val)
    }
}
