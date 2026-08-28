#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "CRC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crc {
    ptr: *mut u8,
}
unsafe impl Send for Crc {}
unsafe impl Sync for Crc {}
impl Crc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn data16(self) -> crate::pac::common::Reg<u16, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn data32(self) -> crate::pac::common::Reg<u32, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn data8(self) -> crate::pac::common::Reg<u8, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn gpoly16(self) -> crate::pac::common::Reg<u16, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn gpoly32(self) -> crate::pac::common::Reg<u32, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn gpoly8(self) -> crate::pac::common::Reg<u8, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::pac::common::Reg<Ctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "TCRC."]
    #[must_use]
    #[inline(always)]
    pub const fn tcrc(&self) -> Tcrc {
        let val = (self.0 >> 24usize) & 0x01;
        Tcrc::from_bits(val as u8)
    }
    #[doc = "TCRC."]
    #[inline(always)]
    pub const fn set_tcrc(&mut self, val: Tcrc) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Write as Seed."]
    #[must_use]
    #[inline(always)]
    pub const fn was(&self) -> Was {
        let val = (self.0 >> 25usize) & 0x01;
        Was::from_bits(val as u8)
    }
    #[doc = "Write as Seed."]
    #[inline(always)]
    pub const fn set_was(&mut self, val: Was) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Complement Read of CRC Data Register."]
    #[must_use]
    #[inline(always)]
    pub const fn fxor(&self) -> Fxor {
        let val = (self.0 >> 26usize) & 0x01;
        Fxor::from_bits(val as u8)
    }
    #[doc = "Complement Read of CRC Data Register."]
    #[inline(always)]
    pub const fn set_fxor(&mut self, val: Fxor) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Transpose Type for Read."]
    #[must_use]
    #[inline(always)]
    pub const fn totr(&self) -> Totr {
        let val = (self.0 >> 28usize) & 0x03;
        Totr::from_bits(val as u8)
    }
    #[doc = "Transpose Type for Read."]
    #[inline(always)]
    pub const fn set_totr(&mut self, val: Totr) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Transpose Type for Write."]
    #[must_use]
    #[inline(always)]
    pub const fn tot(&self) -> Tot {
        let val = (self.0 >> 30usize) & 0x03;
        Tot::from_bits(val as u8)
    }
    #[doc = "Transpose Type for Write."]
    #[inline(always)]
    pub const fn set_tot(&mut self, val: Tot) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("tcrc", &self.tcrc())
            .field("was", &self.was())
            .field("fxor", &self.fxor())
            .field("totr", &self.totr())
            .field("tot", &self.tot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ tcrc: {:?}, was: {:?}, fxor: {:?}, totr: {:?}, tot: {:?} }}",
            self.tcrc(),
            self.was(),
            self.fxor(),
            self.totr(),
            self.tot()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fxor {
    #[doc = "Disables XOR on reading data."]
    Noxor = 0x0,
    #[doc = "Inverts or complements the read value of the CRC Data."]
    Invert = 0x01,
}
impl Fxor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fxor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fxor {
    #[inline(always)]
    fn from(val: u8) -> Fxor {
        Fxor::from_bits(val)
    }
}
impl From<Fxor> for u8 {
    #[inline(always)]
    fn from(val: Fxor) -> u8 {
        Fxor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcrc {
    #[doc = "16 bits."]
    B16 = 0x0,
    #[doc = "32 bits."]
    B32 = 0x01,
}
impl Tcrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcrc {
    #[inline(always)]
    fn from(val: u8) -> Tcrc {
        Tcrc::from_bits(val)
    }
}
impl From<Tcrc> for u8 {
    #[inline(always)]
    fn from(val: Tcrc) -> u8 {
        Tcrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tot {
    #[doc = "No transposition."]
    Notrnps = 0x0,
    #[doc = "Bits in bytes are transposed, but bytes are not transposed."]
    BtsTrnps = 0x01,
    #[doc = "Both bits in bytes and bytes are transposed."]
    BytsBtsTrnps = 0x02,
    #[doc = "Only bytes are transposed, no bits in a byte are transposed."]
    BytsTrnps = 0x03,
}
impl Tot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tot {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tot {
    #[inline(always)]
    fn from(val: u8) -> Tot {
        Tot::from_bits(val)
    }
}
impl From<Tot> for u8 {
    #[inline(always)]
    fn from(val: Tot) -> u8 {
        Tot::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Totr {
    #[doc = "No transposition."]
    Notrnps = 0x0,
    #[doc = "Bits in bytes are transposed, but bytes are not transposed."]
    BtsTrnps = 0x01,
    #[doc = "Both bits in bytes and bytes are transposed."]
    BytsBtsTrnps = 0x02,
    #[doc = "Only bytes are transposed, no bits in a byte are transposed."]
    BytsTrnps = 0x03,
}
impl Totr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Totr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Totr {
    #[inline(always)]
    fn from(val: u8) -> Totr {
        Totr::from_bits(val)
    }
}
impl From<Totr> for u8 {
    #[inline(always)]
    fn from(val: Totr) -> u8 {
        Totr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Was {
    #[doc = "Data values."]
    Data = 0x0,
    #[doc = "Seed values."]
    Seed = 0x01,
}
impl Was {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Was {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Was {
    #[inline(always)]
    fn from(val: u8) -> Was {
        Was::from_bits(val)
    }
}
impl From<Was> for u8 {
    #[inline(always)]
    fn from(val: Was) -> u8 {
        Was::to_bits(val)
    }
}
