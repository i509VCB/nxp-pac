#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "LPTMR."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lptmr {
    ptr: *mut u8,
}
unsafe impl Send for Lptmr {}
unsafe impl Sync for Lptmr {}
impl Lptmr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Status."]
    #[inline(always)]
    pub const fn csr(self) -> crate::pac::common::Reg<Csr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Prescaler and Glitch Filter."]
    #[inline(always)]
    pub const fn psr(self) -> crate::pac::common::Reg<Psr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Compare."]
    #[inline(always)]
    pub const fn cmr(self) -> crate::pac::common::Reg<Cmr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Counter."]
    #[inline(always)]
    pub const fn cnr(self) -> crate::pac::common::Reg<Cnr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "Compare."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmr(pub u32);
impl Cmr {
    #[doc = "Compare Value."]
    #[must_use]
    #[inline(always)]
    pub const fn compare(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Compare Value."]
    #[inline(always)]
    pub const fn set_compare(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmr {
    #[inline(always)]
    fn default() -> Cmr {
        Cmr(0)
    }
}
impl core::fmt::Debug for Cmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmr")
            .field("compare", &self.compare())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmr {{ compare: {=u32:?} }}", self.compare())
    }
}
#[doc = "Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnr(pub u32);
impl Cnr {
    #[doc = "Counter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn counter(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Counter Value."]
    #[inline(always)]
    pub const fn set_counter(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cnr {
    #[inline(always)]
    fn default() -> Cnr {
        Cnr(0)
    }
}
impl core::fmt::Debug for Cnr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cnr")
            .field("counter", &self.counter())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cnr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cnr {{ counter: {=u32:?} }}", self.counter())
    }
}
#[doc = "Control Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "Timer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ten(&self) -> Ten {
        let val = (self.0 >> 0usize) & 0x01;
        Ten::from_bits(val as u8)
    }
    #[doc = "Timer Enable."]
    #[inline(always)]
    pub const fn set_ten(&mut self, val: Ten) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Timer Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tms(&self) -> Tms {
        let val = (self.0 >> 1usize) & 0x01;
        Tms::from_bits(val as u8)
    }
    #[doc = "Timer Mode Select."]
    #[inline(always)]
    pub const fn set_tms(&mut self, val: Tms) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Timer Free-Running Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn tfc(&self) -> Tfc {
        let val = (self.0 >> 2usize) & 0x01;
        Tfc::from_bits(val as u8)
    }
    #[doc = "Timer Free-Running Counter."]
    #[inline(always)]
    pub const fn set_tfc(&mut self, val: Tfc) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Timer Pin Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tpp(&self) -> Tpp {
        let val = (self.0 >> 3usize) & 0x01;
        Tpp::from_bits(val as u8)
    }
    #[doc = "Timer Pin Polarity."]
    #[inline(always)]
    pub const fn set_tpp(&mut self, val: Tpp) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Timer Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tps(&self) -> Tps {
        let val = (self.0 >> 4usize) & 0x03;
        Tps::from_bits(val as u8)
    }
    #[doc = "Timer Pin Select."]
    #[inline(always)]
    pub const fn set_tps(&mut self, val: Tps) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Timer Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> Tie {
        let val = (self.0 >> 6usize) & 0x01;
        Tie::from_bits(val as u8)
    }
    #[doc = "Timer Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: Tie) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Timer Compare Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tcf(&self) -> Tcf {
        let val = (self.0 >> 7usize) & 0x01;
        Tcf::from_bits(val as u8)
    }
    #[doc = "Timer Compare Flag."]
    #[inline(always)]
    pub const fn set_tcf(&mut self, val: Tcf) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer DMA Request Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tdre(&self) -> Tdre {
        let val = (self.0 >> 8usize) & 0x01;
        Tdre::from_bits(val as u8)
    }
    #[doc = "Timer DMA Request Enable."]
    #[inline(always)]
    pub const fn set_tdre(&mut self, val: Tdre) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        Csr(0)
    }
}
impl core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csr")
            .field("ten", &self.ten())
            .field("tms", &self.tms())
            .field("tfc", &self.tfc())
            .field("tpp", &self.tpp())
            .field("tps", &self.tps())
            .field("tie", &self.tie())
            .field("tcf", &self.tcf())
            .field("tdre", &self.tdre())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csr {{ ten: {:?}, tms: {:?}, tfc: {:?}, tpp: {:?}, tps: {:?}, tie: {:?}, tcf: {:?}, tdre: {:?} }}",
            self.ten(),
            self.tms(),
            self.tfc(),
            self.tpp(),
            self.tps(),
            self.tie(),
            self.tcf(),
            self.tdre()
        )
    }
}
#[doc = "Prescaler and Glitch Filter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psr(pub u32);
impl Psr {
    #[doc = "Prescaler and Glitch Filter Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pcs(&self) -> Pcs {
        let val = (self.0 >> 0usize) & 0x03;
        Pcs::from_bits(val as u8)
    }
    #[doc = "Prescaler and Glitch Filter Clock Select."]
    #[inline(always)]
    pub const fn set_pcs(&mut self, val: Pcs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Prescaler and Glitch Filter Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyp(&self) -> Pbyp {
        let val = (self.0 >> 2usize) & 0x01;
        Pbyp::from_bits(val as u8)
    }
    #[doc = "Prescaler and Glitch Filter Bypass."]
    #[inline(always)]
    pub const fn set_pbyp(&mut self, val: Pbyp) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Prescaler and Glitch Filter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn prescale(&self) -> Prescale {
        let val = (self.0 >> 3usize) & 0x0f;
        Prescale::from_bits(val as u8)
    }
    #[doc = "Prescaler and Glitch Filter Value."]
    #[inline(always)]
    pub const fn set_prescale(&mut self, val: Prescale) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val.to_bits() as u32) & 0x0f) << 3usize);
    }
}
impl Default for Psr {
    #[inline(always)]
    fn default() -> Psr {
        Psr(0)
    }
}
impl core::fmt::Debug for Psr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psr")
            .field("pcs", &self.pcs())
            .field("pbyp", &self.pbyp())
            .field("prescale", &self.prescale())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Psr {{ pcs: {:?}, pbyp: {:?}, prescale: {:?} }}",
            self.pcs(),
            self.pbyp(),
            self.prescale()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pbyp {
    #[doc = "Prescaler and glitch filter enable."]
    Pbyp0 = 0x0,
    #[doc = "Prescaler and glitch filter bypass."]
    Pbyp1 = 0x01,
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
    #[doc = "Clock 0."]
    Pcs00 = 0x0,
    #[doc = "Clock 1."]
    Pcs01 = 0x01,
    #[doc = "Clock 2."]
    Pcs10 = 0x02,
    #[doc = "Clock 3."]
    Pcs11 = 0x03,
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
    #[doc = "Prescaler divides the prescaler clock by 2; glitch filter does not support this configuration."]
    Prescale0000 = 0x0,
    #[doc = "Prescaler divides the prescaler clock by 4; glitch filter recognizes change on input pin after two rising clock edges."]
    Prescale0001 = 0x01,
    #[doc = "Prescaler divides the prescaler clock by 8; glitch filter recognizes change on input pin after four rising clock edges."]
    Prescale0010 = 0x02,
    #[doc = "Prescaler divides the prescaler clock by 16; glitch filter recognizes change on input pin after eight rising clock edges."]
    Prescale0011 = 0x03,
    #[doc = "Prescaler divides the prescaler clock by 32; glitch filter recognizes change on input pin after 16 rising clock edges."]
    Prescale0100 = 0x04,
    #[doc = "Prescaler divides the prescaler clock by 64; glitch filter recognizes change on input pin after 32 rising clock edges."]
    Prescale0101 = 0x05,
    #[doc = "Prescaler divides the prescaler clock by 128; glitch filter recognizes change on input pin after 64 rising clock edges."]
    Prescale0110 = 0x06,
    #[doc = "Prescaler divides the prescaler clock by 256; glitch filter recognizes change on input pin after 128 rising clock edges."]
    Prescale0111 = 0x07,
    #[doc = "Prescaler divides the prescaler clock by 512; glitch filter recognizes change on input pin after 256 rising clock edges."]
    Prescale1000 = 0x08,
    #[doc = "Prescaler divides the prescaler clock by 1024; glitch filter recognizes change on input pin after 512 rising clock edges."]
    Prescale1001 = 0x09,
    #[doc = "Prescaler divides the prescaler clock by 2048; glitch filter recognizes change on input pin after 1024 rising clock edges."]
    Prescale1010 = 0x0a,
    #[doc = "Prescaler divides the prescaler clock by 4096; glitch filter recognizes change on input pin after 2048 rising clock edges."]
    Prescale1011 = 0x0b,
    #[doc = "Prescaler divides the prescaler clock by 8192; glitch filter recognizes change on input pin after 4096 rising clock edges."]
    Prescale1100 = 0x0c,
    #[doc = "Prescaler divides the prescaler clock by 16,384; glitch filter recognizes change on input pin after 8192 rising clock edges."]
    Prescale1101 = 0x0d,
    #[doc = "Prescaler divides the prescaler clock by 32,768; glitch filter recognizes change on input pin after 16,384 rising clock edges."]
    Prescale1110 = 0x0e,
    #[doc = "Prescaler divides the prescaler clock by 65,536; glitch filter recognizes change on input pin after 32,768 rising clock edges."]
    Prescale1111 = 0x0f,
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
    #[doc = "CNR != (CMR + 1)."]
    Tcf0 = 0x0,
    #[doc = "CNR = (CMR + 1)."]
    Tcf1 = 0x01,
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
    #[doc = "Disable."]
    Trde0 = 0x0,
    #[doc = "Enable."]
    Trde1 = 0x01,
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
    #[doc = "Disable."]
    Ten0 = 0x0,
    #[doc = "Enable."]
    Ten1 = 0x01,
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
    #[doc = "Reset when TCF asserts."]
    Tfc0 = 0x0,
    #[doc = "Reset on overflow."]
    Tfc1 = 0x01,
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
    #[doc = "Disable."]
    Tie0 = 0x0,
    #[doc = "Enable."]
    Tie1 = 0x01,
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
    #[doc = "Time Counter."]
    Tms0 = 0x0,
    #[doc = "Pulse Counter."]
    Tms1 = 0x01,
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
    #[doc = "Active-high."]
    Tpp0 = 0x0,
    #[doc = "Active-low."]
    Tpp1 = 0x01,
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
    #[doc = "Input 0."]
    Tps00 = 0x0,
    #[doc = "Input 1."]
    Tps01 = 0x01,
    #[doc = "Input 2."]
    Tps10 = 0x02,
    #[doc = "Input 3."]
    Tps11 = 0x03,
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
