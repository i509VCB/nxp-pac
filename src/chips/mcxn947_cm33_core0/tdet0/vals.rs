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
