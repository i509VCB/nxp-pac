#[doc = "Value of OTP Bank 1 Word 5 (Analog Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpAna0(pub u32);
impl HwOcotpAna0 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpAna0 {
    #[inline(always)]
    fn default() -> HwOcotpAna0 {
        HwOcotpAna0(0)
    }
}
impl core::fmt::Debug for HwOcotpAna0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpAna0")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpAna0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpAna0 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank 1 Word 6 (Analog Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpAna1(pub u32);
impl HwOcotpAna1 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpAna1 {
    #[inline(always)]
    fn default() -> HwOcotpAna1 {
        HwOcotpAna1(0)
    }
}
impl core::fmt::Debug for HwOcotpAna1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpAna1")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpAna1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpAna1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank 1 Word 7 (Analog Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpAna2(pub u32);
impl HwOcotpAna2 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpAna2 {
    #[inline(always)]
    fn default() -> HwOcotpAna2 {
        HwOcotpAna2(0)
    }
}
impl core::fmt::Debug for HwOcotpAna2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpAna2")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpAna2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpAna2 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpCfg0(pub u32);
impl HwOcotpCfg0 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpCfg0 {
    #[inline(always)]
    fn default() -> HwOcotpCfg0 {
        HwOcotpCfg0(0)
    }
}
impl core::fmt::Debug for HwOcotpCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpCfg0")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpCfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpCfg0 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpCfg1(pub u32);
impl HwOcotpCfg1 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpCfg1 {
    #[inline(always)]
    fn default() -> HwOcotpCfg1 {
        HwOcotpCfg1(0)
    }
}
impl core::fmt::Debug for HwOcotpCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpCfg1")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpCfg1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpCfg2(pub u32);
impl HwOcotpCfg2 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpCfg2 {
    #[inline(always)]
    fn default() -> HwOcotpCfg2 {
        HwOcotpCfg2(0)
    }
}
impl core::fmt::Debug for HwOcotpCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpCfg2")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpCfg2 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpCfg3(pub u32);
impl HwOcotpCfg3 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpCfg3 {
    #[inline(always)]
    fn default() -> HwOcotpCfg3 {
        HwOcotpCfg3(0)
    }
}
impl core::fmt::Debug for HwOcotpCfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpCfg3")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpCfg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpCfg3 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpCfg4(pub u32);
impl HwOcotpCfg4 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpCfg4 {
    #[inline(always)]
    fn default() -> HwOcotpCfg4 {
        HwOcotpCfg4(0)
    }
}
impl core::fmt::Debug for HwOcotpCfg4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpCfg4")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpCfg4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpCfg4 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpCfg5(pub u32);
impl HwOcotpCfg5 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpCfg5 {
    #[inline(always)]
    fn default() -> HwOcotpCfg5 {
        HwOcotpCfg5(0)
    }
}
impl core::fmt::Debug for HwOcotpCfg5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpCfg5")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpCfg5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpCfg5 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpCfg6(pub u32);
impl HwOcotpCfg6 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpCfg6 {
    #[inline(always)]
    fn default() -> HwOcotpCfg6 {
        HwOcotpCfg6(0)
    }
}
impl core::fmt::Debug for HwOcotpCfg6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpCfg6")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpCfg6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpCfg6 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "OTP Controller Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpCtrl(pub u32);
impl HwOcotpCtrl {
    #[doc = "OTP write and read access address register"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "OTP write and read access address register"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "OTP controller status bit"]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OTP controller status bit"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Locked Region Access Error"]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Locked Region Access Error"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Reload Shadow Registers"]
    #[must_use]
    #[inline(always)]
    pub const fn reload_shadows(&self) -> super::vals::ReloadShadows {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::ReloadShadows::from_bits(val as u8)
    }
    #[doc = "Reload Shadow Registers"]
    #[inline(always)]
    pub const fn set_reload_shadows(&mut self, val: super::vals::ReloadShadows) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Write Unlock"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_unlock(&self) -> super::vals::WrUnlock {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::WrUnlock::from_bits(val as u16)
    }
    #[doc = "Write Unlock"]
    #[inline(always)]
    pub const fn set_wr_unlock(&mut self, val: super::vals::WrUnlock) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for HwOcotpCtrl {
    #[inline(always)]
    fn default() -> HwOcotpCtrl {
        HwOcotpCtrl(0)
    }
}
impl core::fmt::Debug for HwOcotpCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpCtrl")
            .field("addr", &self.addr())
            .field("busy", &self.busy())
            .field("error", &self.error())
            .field("reload_shadows", &self.reload_shadows())
            .field("wr_unlock", &self.wr_unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwOcotpCtrl {{ addr: {=u8:?}, busy: {=bool:?}, error: {=bool:?}, reload_shadows: {:?}, wr_unlock: {:?} }}",
            self.addr(),
            self.busy(),
            self.error(),
            self.reload_shadows(),
            self.wr_unlock()
        )
    }
}
#[doc = "OTP Controller Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpCtrlClr(pub u32);
impl HwOcotpCtrlClr {
    #[doc = "OTP write and read access address register"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "OTP write and read access address register"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "OTP controller status bit"]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OTP controller status bit"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Locked Region Access Error"]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Locked Region Access Error"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Reload Shadow Registers"]
    #[must_use]
    #[inline(always)]
    pub const fn reload_shadows(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Shadow Registers"]
    #[inline(always)]
    pub const fn set_reload_shadows(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write Unlock"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_unlock(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Write Unlock"]
    #[inline(always)]
    pub const fn set_wr_unlock(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for HwOcotpCtrlClr {
    #[inline(always)]
    fn default() -> HwOcotpCtrlClr {
        HwOcotpCtrlClr(0)
    }
}
impl core::fmt::Debug for HwOcotpCtrlClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpCtrlClr")
            .field("addr", &self.addr())
            .field("busy", &self.busy())
            .field("error", &self.error())
            .field("reload_shadows", &self.reload_shadows())
            .field("wr_unlock", &self.wr_unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpCtrlClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwOcotpCtrlClr {{ addr: {=u8:?}, busy: {=bool:?}, error: {=bool:?}, reload_shadows: {=bool:?}, wr_unlock: {=u16:?} }}",
            self.addr(),
            self.busy(),
            self.error(),
            self.reload_shadows(),
            self.wr_unlock()
        )
    }
}
#[doc = "OTP Controller Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpCtrlSet(pub u32);
impl HwOcotpCtrlSet {
    #[doc = "OTP write and read access address register"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "OTP write and read access address register"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "OTP controller status bit"]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OTP controller status bit"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Locked Region Access Error"]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Locked Region Access Error"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Reload Shadow Registers"]
    #[must_use]
    #[inline(always)]
    pub const fn reload_shadows(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Shadow Registers"]
    #[inline(always)]
    pub const fn set_reload_shadows(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write Unlock"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_unlock(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Write Unlock"]
    #[inline(always)]
    pub const fn set_wr_unlock(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for HwOcotpCtrlSet {
    #[inline(always)]
    fn default() -> HwOcotpCtrlSet {
        HwOcotpCtrlSet(0)
    }
}
impl core::fmt::Debug for HwOcotpCtrlSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpCtrlSet")
            .field("addr", &self.addr())
            .field("busy", &self.busy())
            .field("error", &self.error())
            .field("reload_shadows", &self.reload_shadows())
            .field("wr_unlock", &self.wr_unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpCtrlSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwOcotpCtrlSet {{ addr: {=u8:?}, busy: {=bool:?}, error: {=bool:?}, reload_shadows: {=bool:?}, wr_unlock: {=u16:?} }}",
            self.addr(),
            self.busy(),
            self.error(),
            self.reload_shadows(),
            self.wr_unlock()
        )
    }
}
#[doc = "OTP Controller Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpCtrlTog(pub u32);
impl HwOcotpCtrlTog {
    #[doc = "OTP write and read access address register"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "OTP write and read access address register"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "OTP controller status bit"]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OTP controller status bit"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Locked Region Access Error"]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Locked Region Access Error"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Reload Shadow Registers"]
    #[must_use]
    #[inline(always)]
    pub const fn reload_shadows(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Shadow Registers"]
    #[inline(always)]
    pub const fn set_reload_shadows(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write Unlock"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_unlock(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Write Unlock"]
    #[inline(always)]
    pub const fn set_wr_unlock(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for HwOcotpCtrlTog {
    #[inline(always)]
    fn default() -> HwOcotpCtrlTog {
        HwOcotpCtrlTog(0)
    }
}
impl core::fmt::Debug for HwOcotpCtrlTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpCtrlTog")
            .field("addr", &self.addr())
            .field("busy", &self.busy())
            .field("error", &self.error())
            .field("reload_shadows", &self.reload_shadows())
            .field("wr_unlock", &self.wr_unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpCtrlTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwOcotpCtrlTog {{ addr: {=u8:?}, busy: {=bool:?}, error: {=bool:?}, reload_shadows: {=bool:?}, wr_unlock: {=u16:?} }}",
            self.addr(),
            self.busy(),
            self.error(),
            self.reload_shadows(),
            self.wr_unlock()
        )
    }
}
#[doc = "OTP Controller Write Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpData(pub u32);
impl HwOcotpData {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpData {
    #[inline(always)]
    fn default() -> HwOcotpData {
        HwOcotpData(0)
    }
}
impl core::fmt::Debug for HwOcotpData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpData")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpData {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpGp1(pub u32);
impl HwOcotpGp1 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpGp1 {
    #[inline(always)]
    fn default() -> HwOcotpGp1 {
        HwOcotpGp1(0)
    }
}
impl core::fmt::Debug for HwOcotpGp1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpGp1")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpGp1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpGp1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpGp2(pub u32);
impl HwOcotpGp2 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpGp2 {
    #[inline(always)]
    fn default() -> HwOcotpGp2 {
        HwOcotpGp2(0)
    }
}
impl core::fmt::Debug for HwOcotpGp2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpGp2")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpGp2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpGp2 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank4 Word4 (OTFAD)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpGp3(pub u32);
impl HwOcotpGp3 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpGp3 {
    #[inline(always)]
    fn default() -> HwOcotpGp3 {
        HwOcotpGp3(0)
    }
}
impl core::fmt::Debug for HwOcotpGp3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpGp3")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpGp3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpGp3 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank0 Word0 (Lock controls)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpLock(pub u32);
impl HwOcotpLock {
    #[doc = "BOOT_CFG Write Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn boot_cfg(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "BOOT_CFG Write Lock Status"]
    #[inline(always)]
    pub const fn set_boot_cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "SJC_RESP Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn sjc_resp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SJC_RESP Lock Status"]
    #[inline(always)]
    pub const fn set_sjc_resp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "OTFAD Write Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn otfad(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "OTFAD Write Lock Status"]
    #[inline(always)]
    pub const fn set_otfad(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "GP1 Write Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn gp1(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "GP1 Write Lock Status"]
    #[inline(always)]
    pub const fn set_gp1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "GP2 Write Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn gp2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "GP2 Write Lock Status"]
    #[inline(always)]
    pub const fn set_gp2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "SW_GP1 Write Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_gp1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SW_GP1 Write Lock Status"]
    #[inline(always)]
    pub const fn set_sw_gp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ANALOG Write Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn analog(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "ANALOG Write Lock Status"]
    #[inline(always)]
    pub const fn set_analog(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "SW_GP2 Write Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_gp2_lock(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "SW_GP2 Write Lock Status"]
    #[inline(always)]
    pub const fn set_sw_gp2_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "MISC_CONF Write Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn misc_conf(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "MISC_CONF Write Lock Status"]
    #[inline(always)]
    pub const fn set_misc_conf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "SW_GP2 Read Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_gp2_rlock(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "SW_GP2 Read Lock Status"]
    #[inline(always)]
    pub const fn set_sw_gp2_rlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "GP3 Write Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn gp3(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "GP3 Write Lock Status"]
    #[inline(always)]
    pub const fn set_gp3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "FIELD RETURN Status"]
    #[must_use]
    #[inline(always)]
    pub const fn field_return(&self) -> super::vals::FieldReturn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FieldReturn::from_bits(val as u8)
    }
    #[doc = "FIELD RETURN Status"]
    #[inline(always)]
    pub const fn set_field_return(&mut self, val: super::vals::FieldReturn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for HwOcotpLock {
    #[inline(always)]
    fn default() -> HwOcotpLock {
        HwOcotpLock(0)
    }
}
impl core::fmt::Debug for HwOcotpLock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpLock")
            .field("boot_cfg", &self.boot_cfg())
            .field("sjc_resp", &self.sjc_resp())
            .field("otfad", &self.otfad())
            .field("gp1", &self.gp1())
            .field("gp2", &self.gp2())
            .field("sw_gp1", &self.sw_gp1())
            .field("analog", &self.analog())
            .field("sw_gp2_lock", &self.sw_gp2_lock())
            .field("misc_conf", &self.misc_conf())
            .field("sw_gp2_rlock", &self.sw_gp2_rlock())
            .field("gp3", &self.gp3())
            .field("field_return", &self.field_return())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpLock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwOcotpLock {{ boot_cfg: {=u8:?}, sjc_resp: {=bool:?}, otfad: {=u8:?}, gp1: {=u8:?}, gp2: {=u8:?}, sw_gp1: {=bool:?}, analog: {=u8:?}, sw_gp2_lock: {=bool:?}, misc_conf: {=bool:?}, sw_gp2_rlock: {=bool:?}, gp3: {=u8:?}, field_return: {:?} }}",
            self.boot_cfg(),
            self.sjc_resp(),
            self.otfad(),
            self.gp1(),
            self.gp2(),
            self.sw_gp1(),
            self.analog(),
            self.sw_gp2_lock(),
            self.misc_conf(),
            self.sw_gp2_rlock(),
            self.gp3(),
            self.field_return()
        )
    }
}
#[doc = "Value of OTP Bank1 Word0 (Memory Related Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpMem0(pub u32);
impl HwOcotpMem0 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpMem0 {
    #[inline(always)]
    fn default() -> HwOcotpMem0 {
        HwOcotpMem0(0)
    }
}
impl core::fmt::Debug for HwOcotpMem0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpMem0")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpMem0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpMem0 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank1 Word1 (Memory Related Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpMem1(pub u32);
impl HwOcotpMem1 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpMem1 {
    #[inline(always)]
    fn default() -> HwOcotpMem1 {
        HwOcotpMem1(0)
    }
}
impl core::fmt::Debug for HwOcotpMem1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpMem1")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpMem1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpMem1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank1 Word2 (Memory Related Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpMem2(pub u32);
impl HwOcotpMem2 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpMem2 {
    #[inline(always)]
    fn default() -> HwOcotpMem2 {
        HwOcotpMem2(0)
    }
}
impl core::fmt::Debug for HwOcotpMem2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpMem2")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpMem2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpMem2 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank1 Word3 (Memory Related Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpMem3(pub u32);
impl HwOcotpMem3 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpMem3 {
    #[inline(always)]
    fn default() -> HwOcotpMem3 {
        HwOcotpMem3(0)
    }
}
impl core::fmt::Debug for HwOcotpMem3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpMem3")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpMem3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpMem3 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank 1 Word 4 (Memory Related Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpMem4(pub u32);
impl HwOcotpMem4 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpMem4 {
    #[inline(always)]
    fn default() -> HwOcotpMem4 {
        HwOcotpMem4(0)
    }
}
impl core::fmt::Debug for HwOcotpMem4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpMem4")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpMem4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpMem4 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank5 Word5 (Misc Conf)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpMiscConf0(pub u32);
impl HwOcotpMiscConf0 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpMiscConf0 {
    #[inline(always)]
    fn default() -> HwOcotpMiscConf0 {
        HwOcotpMiscConf0(0)
    }
}
impl core::fmt::Debug for HwOcotpMiscConf0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpMiscConf0")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpMiscConf0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpMiscConf0 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank5 Word6 (Misc Conf)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpMiscConf1(pub u32);
impl HwOcotpMiscConf1 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpMiscConf1 {
    #[inline(always)]
    fn default() -> HwOcotpMiscConf1 {
        HwOcotpMiscConf1(0)
    }
}
impl core::fmt::Debug for HwOcotpMiscConf1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpMiscConf1")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpMiscConf1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpMiscConf1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank4 Word2 (OTFAD)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpOtfadCfg0(pub u32);
impl HwOcotpOtfadCfg0 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpOtfadCfg0 {
    #[inline(always)]
    fn default() -> HwOcotpOtfadCfg0 {
        HwOcotpOtfadCfg0(0)
    }
}
impl core::fmt::Debug for HwOcotpOtfadCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpOtfadCfg0")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpOtfadCfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpOtfadCfg0 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank4 Word3 (OTFAD)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpOtfadCfg1(pub u32);
impl HwOcotpOtfadCfg1 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpOtfadCfg1 {
    #[inline(always)]
    fn default() -> HwOcotpOtfadCfg1 {
        HwOcotpOtfadCfg1(0)
    }
}
impl core::fmt::Debug for HwOcotpOtfadCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpOtfadCfg1")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpOtfadCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpOtfadCfg1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "OTP Controller Write Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpReadCtrl(pub u32);
impl HwOcotpReadCtrl {
    #[doc = "Read Fuse"]
    #[must_use]
    #[inline(always)]
    pub const fn read_fuse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read Fuse"]
    #[inline(always)]
    pub const fn set_read_fuse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for HwOcotpReadCtrl {
    #[inline(always)]
    fn default() -> HwOcotpReadCtrl {
        HwOcotpReadCtrl(0)
    }
}
impl core::fmt::Debug for HwOcotpReadCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpReadCtrl")
            .field("read_fuse", &self.read_fuse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpReadCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwOcotpReadCtrl {{ read_fuse: {=bool:?} }}",
            self.read_fuse()
        )
    }
}
#[doc = "OTP Controller Read Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpReadFuseData(pub u32);
impl HwOcotpReadFuseData {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpReadFuseData {
    #[inline(always)]
    fn default() -> HwOcotpReadFuseData {
        HwOcotpReadFuseData(0)
    }
}
impl core::fmt::Debug for HwOcotpReadFuseData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpReadFuseData")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpReadFuseData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpReadFuseData {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Software Controllable Signals Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpScs(pub u32);
impl HwOcotpScs {
    #[doc = "HAB JTAG Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hab_jde(&self) -> super::vals::HabJde {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::HabJde::from_bits(val as u8)
    }
    #[doc = "HAB JTAG Debug Enable"]
    #[inline(always)]
    pub const fn set_hab_jde(&mut self, val: super::vals::HabJde) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Spare"]
    #[must_use]
    #[inline(always)]
    pub const fn spare(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Spare"]
    #[inline(always)]
    pub const fn set_spare(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 1usize)) | (((val as u32) & 0x3fff_ffff) << 1usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HwOcotpScs {
    #[inline(always)]
    fn default() -> HwOcotpScs {
        HwOcotpScs(0)
    }
}
impl core::fmt::Debug for HwOcotpScs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpScs")
            .field("hab_jde", &self.hab_jde())
            .field("spare", &self.spare())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpScs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwOcotpScs {{ hab_jde: {:?}, spare: {=u32:?}, lock: {=bool:?} }}",
            self.hab_jde(),
            self.spare(),
            self.lock()
        )
    }
}
#[doc = "Software Controllable Signals Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpScsClr(pub u32);
impl HwOcotpScsClr {
    #[doc = "HAB JTAG Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hab_jde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HAB JTAG Debug Enable"]
    #[inline(always)]
    pub const fn set_hab_jde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Spare"]
    #[must_use]
    #[inline(always)]
    pub const fn spare(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Spare"]
    #[inline(always)]
    pub const fn set_spare(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 1usize)) | (((val as u32) & 0x3fff_ffff) << 1usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HwOcotpScsClr {
    #[inline(always)]
    fn default() -> HwOcotpScsClr {
        HwOcotpScsClr(0)
    }
}
impl core::fmt::Debug for HwOcotpScsClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpScsClr")
            .field("hab_jde", &self.hab_jde())
            .field("spare", &self.spare())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpScsClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwOcotpScsClr {{ hab_jde: {=bool:?}, spare: {=u32:?}, lock: {=bool:?} }}",
            self.hab_jde(),
            self.spare(),
            self.lock()
        )
    }
}
#[doc = "Software Controllable Signals Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpScsSet(pub u32);
impl HwOcotpScsSet {
    #[doc = "HAB JTAG Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hab_jde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HAB JTAG Debug Enable"]
    #[inline(always)]
    pub const fn set_hab_jde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Spare"]
    #[must_use]
    #[inline(always)]
    pub const fn spare(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Spare"]
    #[inline(always)]
    pub const fn set_spare(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 1usize)) | (((val as u32) & 0x3fff_ffff) << 1usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HwOcotpScsSet {
    #[inline(always)]
    fn default() -> HwOcotpScsSet {
        HwOcotpScsSet(0)
    }
}
impl core::fmt::Debug for HwOcotpScsSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpScsSet")
            .field("hab_jde", &self.hab_jde())
            .field("spare", &self.spare())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpScsSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwOcotpScsSet {{ hab_jde: {=bool:?}, spare: {=u32:?}, lock: {=bool:?} }}",
            self.hab_jde(),
            self.spare(),
            self.lock()
        )
    }
}
#[doc = "Software Controllable Signals Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpScsTog(pub u32);
impl HwOcotpScsTog {
    #[doc = "HAB JTAG Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hab_jde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HAB JTAG Debug Enable"]
    #[inline(always)]
    pub const fn set_hab_jde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Spare"]
    #[must_use]
    #[inline(always)]
    pub const fn spare(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Spare"]
    #[inline(always)]
    pub const fn set_spare(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 1usize)) | (((val as u32) & 0x3fff_ffff) << 1usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HwOcotpScsTog {
    #[inline(always)]
    fn default() -> HwOcotpScsTog {
        HwOcotpScsTog(0)
    }
}
impl core::fmt::Debug for HwOcotpScsTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpScsTog")
            .field("hab_jde", &self.hab_jde())
            .field("spare", &self.spare())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpScsTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwOcotpScsTog {{ hab_jde: {=bool:?}, spare: {=u32:?}, lock: {=bool:?} }}",
            self.hab_jde(),
            self.spare(),
            self.lock()
        )
    }
}
#[doc = "Value of OTP Bank4 Word0 (Secure JTAG Response Field)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpSjcResp0(pub u32);
impl HwOcotpSjcResp0 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpSjcResp0 {
    #[inline(always)]
    fn default() -> HwOcotpSjcResp0 {
        HwOcotpSjcResp0(0)
    }
}
impl core::fmt::Debug for HwOcotpSjcResp0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpSjcResp0")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpSjcResp0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpSjcResp0 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank4 Word1 (Secure JTAG Response Field)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpSjcResp1(pub u32);
impl HwOcotpSjcResp1 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpSjcResp1 {
    #[inline(always)]
    fn default() -> HwOcotpSjcResp1 {
        HwOcotpSjcResp1(0)
    }
}
impl core::fmt::Debug for HwOcotpSjcResp1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpSjcResp1")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpSjcResp1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpSjcResp1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Shadow Register for OTP Bank3 Word0 (SRK Hash)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpSrk0(pub u32);
impl HwOcotpSrk0 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpSrk0 {
    #[inline(always)]
    fn default() -> HwOcotpSrk0 {
        HwOcotpSrk0(0)
    }
}
impl core::fmt::Debug for HwOcotpSrk0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpSrk0")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpSrk0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpSrk0 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Shadow Register for OTP Bank3 Word1 (SRK Hash)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpSrk1(pub u32);
impl HwOcotpSrk1 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpSrk1 {
    #[inline(always)]
    fn default() -> HwOcotpSrk1 {
        HwOcotpSrk1(0)
    }
}
impl core::fmt::Debug for HwOcotpSrk1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpSrk1")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpSrk1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpSrk1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Shadow Register for OTP Bank3 Word2 (SRK Hash)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpSrk2(pub u32);
impl HwOcotpSrk2 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpSrk2 {
    #[inline(always)]
    fn default() -> HwOcotpSrk2 {
        HwOcotpSrk2(0)
    }
}
impl core::fmt::Debug for HwOcotpSrk2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpSrk2")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpSrk2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpSrk2 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Shadow Register for OTP Bank3 Word3 (SRK Hash)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpSrk3(pub u32);
impl HwOcotpSrk3 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpSrk3 {
    #[inline(always)]
    fn default() -> HwOcotpSrk3 {
        HwOcotpSrk3(0)
    }
}
impl core::fmt::Debug for HwOcotpSrk3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpSrk3")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpSrk3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpSrk3 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Shadow Register for OTP Bank3 Word4 (SRK Hash)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpSrk4(pub u32);
impl HwOcotpSrk4 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpSrk4 {
    #[inline(always)]
    fn default() -> HwOcotpSrk4 {
        HwOcotpSrk4(0)
    }
}
impl core::fmt::Debug for HwOcotpSrk4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpSrk4")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpSrk4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpSrk4 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Shadow Register for OTP Bank3 Word5 (SRK Hash)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpSrk5(pub u32);
impl HwOcotpSrk5 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpSrk5 {
    #[inline(always)]
    fn default() -> HwOcotpSrk5 {
        HwOcotpSrk5(0)
    }
}
impl core::fmt::Debug for HwOcotpSrk5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpSrk5")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpSrk5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpSrk5 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Shadow Register for OTP Bank3 Word6 (SRK Hash)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpSrk6(pub u32);
impl HwOcotpSrk6 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpSrk6 {
    #[inline(always)]
    fn default() -> HwOcotpSrk6 {
        HwOcotpSrk6(0)
    }
}
impl core::fmt::Debug for HwOcotpSrk6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpSrk6")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpSrk6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpSrk6 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Shadow Register for OTP Bank3 Word7 (SRK Hash)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpSrk7(pub u32);
impl HwOcotpSrk7 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpSrk7 {
    #[inline(always)]
    fn default() -> HwOcotpSrk7 {
        HwOcotpSrk7(0)
    }
}
impl core::fmt::Debug for HwOcotpSrk7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpSrk7")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpSrk7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpSrk7 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank5 Word7 (SRK Revoke)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpSrkRevoke(pub u32);
impl HwOcotpSrkRevoke {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpSrkRevoke {
    #[inline(always)]
    fn default() -> HwOcotpSrkRevoke {
        HwOcotpSrkRevoke(0)
    }
}
impl core::fmt::Debug for HwOcotpSrkRevoke {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpSrkRevoke")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpSrkRevoke {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpSrkRevoke {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank5 Word0 (SW GP1)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpSwGp1(pub u32);
impl HwOcotpSwGp1 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpSwGp1 {
    #[inline(always)]
    fn default() -> HwOcotpSwGp1 {
        HwOcotpSwGp1(0)
    }
}
impl core::fmt::Debug for HwOcotpSwGp1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpSwGp1")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpSwGp1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpSwGp1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank5 Word1 (SW GP2)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpSwGp20(pub u32);
impl HwOcotpSwGp20 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpSwGp20 {
    #[inline(always)]
    fn default() -> HwOcotpSwGp20 {
        HwOcotpSwGp20(0)
    }
}
impl core::fmt::Debug for HwOcotpSwGp20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpSwGp20")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpSwGp20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpSwGp20 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank5 Word2 (SW GP2)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpSwGp21(pub u32);
impl HwOcotpSwGp21 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpSwGp21 {
    #[inline(always)]
    fn default() -> HwOcotpSwGp21 {
        HwOcotpSwGp21(0)
    }
}
impl core::fmt::Debug for HwOcotpSwGp21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpSwGp21")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpSwGp21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpSwGp21 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank5 Word3 (SW GP2)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpSwGp22(pub u32);
impl HwOcotpSwGp22 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpSwGp22 {
    #[inline(always)]
    fn default() -> HwOcotpSwGp22 {
        HwOcotpSwGp22(0)
    }
}
impl core::fmt::Debug for HwOcotpSwGp22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpSwGp22")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpSwGp22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpSwGp22 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank5 Word4 (SW GP2)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpSwGp23(pub u32);
impl HwOcotpSwGp23 {
    #[doc = "BITS"]
    #[must_use]
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BITS"]
    #[inline(always)]
    pub const fn set_bits(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwOcotpSwGp23 {
    #[inline(always)]
    fn default() -> HwOcotpSwGp23 {
        HwOcotpSwGp23(0)
    }
}
impl core::fmt::Debug for HwOcotpSwGp23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpSwGp23")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpSwGp23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwOcotpSwGp23 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Sticky bit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpSwSticky(pub u32);
impl HwOcotpSwSticky {
    #[doc = "SRK Revoke Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn srk_revoke_lock(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SRK Revoke Lock"]
    #[inline(always)]
    pub const fn set_srk_revoke_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Field Return Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn field_return_lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Field Return Lock"]
    #[inline(always)]
    pub const fn set_field_return_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for HwOcotpSwSticky {
    #[inline(always)]
    fn default() -> HwOcotpSwSticky {
        HwOcotpSwSticky(0)
    }
}
impl core::fmt::Debug for HwOcotpSwSticky {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpSwSticky")
            .field("srk_revoke_lock", &self.srk_revoke_lock())
            .field("field_return_lock", &self.field_return_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpSwSticky {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwOcotpSwSticky {{ srk_revoke_lock: {=bool:?}, field_return_lock: {=bool:?} }}",
            self.srk_revoke_lock(),
            self.field_return_lock()
        )
    }
}
#[doc = "OTP Controller Timing Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpTiming(pub u32);
impl HwOcotpTiming {
    #[doc = "Write Strobe Period"]
    #[must_use]
    #[inline(always)]
    pub const fn strobe_prog(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Write Strobe Period"]
    #[inline(always)]
    pub const fn set_strobe_prog(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Relax Count Value"]
    #[must_use]
    #[inline(always)]
    pub const fn relax(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Relax Count Value"]
    #[inline(always)]
    pub const fn set_relax(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Read Strobe Period"]
    #[must_use]
    #[inline(always)]
    pub const fn strobe_read(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Read Strobe Period"]
    #[inline(always)]
    pub const fn set_strobe_read(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Wait Interval"]
    #[must_use]
    #[inline(always)]
    pub const fn wait(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x3f;
        val as u8
    }
    #[doc = "Wait Interval"]
    #[inline(always)]
    pub const fn set_wait(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 22usize)) | (((val as u32) & 0x3f) << 22usize);
    }
}
impl Default for HwOcotpTiming {
    #[inline(always)]
    fn default() -> HwOcotpTiming {
        HwOcotpTiming(0)
    }
}
impl core::fmt::Debug for HwOcotpTiming {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpTiming")
            .field("strobe_prog", &self.strobe_prog())
            .field("relax", &self.relax())
            .field("strobe_read", &self.strobe_read())
            .field("wait", &self.wait())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpTiming {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwOcotpTiming {{ strobe_prog: {=u16:?}, relax: {=u8:?}, strobe_read: {=u8:?}, wait: {=u8:?} }}",
            self.strobe_prog(),
            self.relax(),
            self.strobe_read(),
            self.wait()
        )
    }
}
#[doc = "OTP Controller Timing Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpTiming2(pub u32);
impl HwOcotpTiming2 {
    #[doc = "Relax Prog. count value"]
    #[must_use]
    #[inline(always)]
    pub const fn relax_prog(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Relax Prog. count value"]
    #[inline(always)]
    pub const fn set_relax_prog(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Relax Read count value"]
    #[must_use]
    #[inline(always)]
    pub const fn relax_read(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Relax Read count value"]
    #[inline(always)]
    pub const fn set_relax_read(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Auto read and write time interval"]
    #[must_use]
    #[inline(always)]
    pub const fn relax1(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0xff;
        val as u8
    }
    #[doc = "Auto read and write time interval"]
    #[inline(always)]
    pub const fn set_relax1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 22usize)) | (((val as u32) & 0xff) << 22usize);
    }
}
impl Default for HwOcotpTiming2 {
    #[inline(always)]
    fn default() -> HwOcotpTiming2 {
        HwOcotpTiming2(0)
    }
}
impl core::fmt::Debug for HwOcotpTiming2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpTiming2")
            .field("relax_prog", &self.relax_prog())
            .field("relax_read", &self.relax_read())
            .field("relax1", &self.relax1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpTiming2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwOcotpTiming2 {{ relax_prog: {=u16:?}, relax_read: {=u8:?}, relax1: {=u8:?} }}",
            self.relax_prog(),
            self.relax_read(),
            self.relax1()
        )
    }
}
#[doc = "OTP Controller Version Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwOcotpVersion(pub u32);
impl HwOcotpVersion {
    #[doc = "RTL Version Steping"]
    #[must_use]
    #[inline(always)]
    pub const fn step(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "RTL Version Steping"]
    #[inline(always)]
    pub const fn set_step(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor RTL Version"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor RTL Version"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major RTL Version"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major RTL Version"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for HwOcotpVersion {
    #[inline(always)]
    fn default() -> HwOcotpVersion {
        HwOcotpVersion(0)
    }
}
impl core::fmt::Debug for HwOcotpVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwOcotpVersion")
            .field("step", &self.step())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwOcotpVersion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwOcotpVersion {{ step: {=u16:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.step(),
            self.minor(),
            self.major()
        )
    }
}
