#[doc = "Value of OTP Bank1 Word5 (Analog Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ana0(pub u32);
impl Ana0 {
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
impl Default for Ana0 {
    #[inline(always)]
    fn default() -> Ana0 {
        Ana0(0)
    }
}
impl core::fmt::Debug for Ana0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ana0").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ana0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ana0 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank1 Word6 (Analog Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ana1(pub u32);
impl Ana1 {
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
impl Default for Ana1 {
    #[inline(always)]
    fn default() -> Ana1 {
        Ana1(0)
    }
}
impl core::fmt::Debug for Ana1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ana1").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ana1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ana1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank1 Word7 (Analog Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ana2(pub u32);
impl Ana2 {
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
impl Default for Ana2 {
    #[inline(always)]
    fn default() -> Ana2 {
        Ana2(0)
    }
}
impl core::fmt::Debug for Ana2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ana2").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ana2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ana2 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg0(pub u32);
impl Cfg0 {
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
impl Default for Cfg0 {
    #[inline(always)]
    fn default() -> Cfg0 {
        Cfg0(0)
    }
}
impl core::fmt::Debug for Cfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg0").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cfg0 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg1(pub u32);
impl Cfg1 {
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
impl Default for Cfg1 {
    #[inline(always)]
    fn default() -> Cfg1 {
        Cfg1(0)
    }
}
impl core::fmt::Debug for Cfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg1").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cfg1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg2(pub u32);
impl Cfg2 {
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
impl Default for Cfg2 {
    #[inline(always)]
    fn default() -> Cfg2 {
        Cfg2(0)
    }
}
impl core::fmt::Debug for Cfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg2").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cfg2 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg3(pub u32);
impl Cfg3 {
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
impl Default for Cfg3 {
    #[inline(always)]
    fn default() -> Cfg3 {
        Cfg3(0)
    }
}
impl core::fmt::Debug for Cfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg3").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cfg3 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg4(pub u32);
impl Cfg4 {
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
impl Default for Cfg4 {
    #[inline(always)]
    fn default() -> Cfg4 {
        Cfg4(0)
    }
}
impl core::fmt::Debug for Cfg4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg4").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cfg4 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg5(pub u32);
impl Cfg5 {
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
impl Default for Cfg5 {
    #[inline(always)]
    fn default() -> Cfg5 {
        Cfg5(0)
    }
}
impl core::fmt::Debug for Cfg5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg5").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cfg5 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg6(pub u32);
impl Cfg6 {
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
impl Default for Cfg6 {
    #[inline(always)]
    fn default() -> Cfg6 {
        Cfg6(0)
    }
}
impl core::fmt::Debug for Cfg6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg6").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cfg6 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "OTP Controller Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
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
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("addr", &self.addr())
            .field("busy", &self.busy())
            .field("error", &self.error())
            .field("reload_shadows", &self.reload_shadows())
            .field("wr_unlock", &self.wr_unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ addr: {=u8:?}, busy: {=bool:?}, error: {=bool:?}, reload_shadows: {:?}, wr_unlock: {:?} }}",
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
pub struct CtrlClr(pub u32);
impl CtrlClr {
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
impl Default for CtrlClr {
    #[inline(always)]
    fn default() -> CtrlClr {
        CtrlClr(0)
    }
}
impl core::fmt::Debug for CtrlClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlClr")
            .field("addr", &self.addr())
            .field("busy", &self.busy())
            .field("error", &self.error())
            .field("reload_shadows", &self.reload_shadows())
            .field("wr_unlock", &self.wr_unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CtrlClr {{ addr: {=u8:?}, busy: {=bool:?}, error: {=bool:?}, reload_shadows: {=bool:?}, wr_unlock: {=u16:?} }}",
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
pub struct CtrlSet(pub u32);
impl CtrlSet {
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
impl Default for CtrlSet {
    #[inline(always)]
    fn default() -> CtrlSet {
        CtrlSet(0)
    }
}
impl core::fmt::Debug for CtrlSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlSet")
            .field("addr", &self.addr())
            .field("busy", &self.busy())
            .field("error", &self.error())
            .field("reload_shadows", &self.reload_shadows())
            .field("wr_unlock", &self.wr_unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CtrlSet {{ addr: {=u8:?}, busy: {=bool:?}, error: {=bool:?}, reload_shadows: {=bool:?}, wr_unlock: {=u16:?} }}",
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
pub struct CtrlTog(pub u32);
impl CtrlTog {
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
impl Default for CtrlTog {
    #[inline(always)]
    fn default() -> CtrlTog {
        CtrlTog(0)
    }
}
impl core::fmt::Debug for CtrlTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlTog")
            .field("addr", &self.addr())
            .field("busy", &self.busy())
            .field("error", &self.error())
            .field("reload_shadows", &self.reload_shadows())
            .field("wr_unlock", &self.wr_unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CtrlTog {{ addr: {=u8:?}, busy: {=bool:?}, error: {=bool:?}, reload_shadows: {=bool:?}, wr_unlock: {=u16:?} }}",
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
pub struct Data(pub u32);
impl Data {
    #[doc = "Used to initiate a write to OTP. See the section for operating details."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Used to initiate a write to OTP. See the section for operating details."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Data {
    #[inline(always)]
    fn default() -> Data {
        Data(0)
    }
}
impl core::fmt::Debug for Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Data").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Data {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gp1(pub u32);
impl Gp1 {
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
impl Default for Gp1 {
    #[inline(always)]
    fn default() -> Gp1 {
        Gp1(0)
    }
}
impl core::fmt::Debug for Gp1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gp1").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gp1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gp1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gp2(pub u32);
impl Gp2 {
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
impl Default for Gp2 {
    #[inline(always)]
    fn default() -> Gp2 {
        Gp2(0)
    }
}
impl core::fmt::Debug for Gp2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gp2").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gp2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gp2 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank7 Word0 (GP3)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gp30(pub u32);
impl Gp30 {
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
impl Default for Gp30 {
    #[inline(always)]
    fn default() -> Gp30 {
        Gp30(0)
    }
}
impl core::fmt::Debug for Gp30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gp30").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gp30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gp30 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank7 Word1 (GP3)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gp31(pub u32);
impl Gp31 {
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
impl Default for Gp31 {
    #[inline(always)]
    fn default() -> Gp31 {
        Gp31(0)
    }
}
impl core::fmt::Debug for Gp31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gp31").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gp31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gp31 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank7 Word2 (GP3)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gp32(pub u32);
impl Gp32 {
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
impl Default for Gp32 {
    #[inline(always)]
    fn default() -> Gp32 {
        Gp32(0)
    }
}
impl core::fmt::Debug for Gp32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gp32").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gp32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gp32 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank7 Word3 (GP3)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gp33(pub u32);
impl Gp33 {
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
impl Default for Gp33 {
    #[inline(always)]
    fn default() -> Gp33 {
        Gp33(0)
    }
}
impl core::fmt::Debug for Gp33 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gp33").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gp33 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gp33 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank7 Word4 (GP4)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gp40(pub u32);
impl Gp40 {
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
impl Default for Gp40 {
    #[inline(always)]
    fn default() -> Gp40 {
        Gp40(0)
    }
}
impl core::fmt::Debug for Gp40 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gp40").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gp40 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gp40 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank7 Word5 (GP4)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gp41(pub u32);
impl Gp41 {
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
impl Default for Gp41 {
    #[inline(always)]
    fn default() -> Gp41 {
        Gp41(0)
    }
}
impl core::fmt::Debug for Gp41 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gp41").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gp41 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gp41 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank7 Word6 (GP4)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gp42(pub u32);
impl Gp42 {
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
impl Default for Gp42 {
    #[inline(always)]
    fn default() -> Gp42 {
        Gp42(0)
    }
}
impl core::fmt::Debug for Gp42 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gp42").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gp42 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gp42 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank7 Word7 (GP4)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gp43(pub u32);
impl Gp43 {
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
impl Default for Gp43 {
    #[inline(always)]
    fn default() -> Gp43 {
        Gp43(0)
    }
}
impl core::fmt::Debug for Gp43 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gp43").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gp43 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gp43 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank0 Word0 (Lock controls)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lock(pub u32);
impl Lock {
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
    #[doc = "GP4 Read Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn gp4_rlock(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GP4 Read Lock Status"]
    #[inline(always)]
    pub const fn set_gp4_rlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "MAC_ADDR Write Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_addr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "MAC_ADDR Write Lock Status"]
    #[inline(always)]
    pub const fn set_mac_addr(&mut self, val: u8) {
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
    #[doc = "GP4 Write Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn gp4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "GP4 Write Lock Status"]
    #[inline(always)]
    pub const fn set_gp4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
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
impl Default for Lock {
    #[inline(always)]
    fn default() -> Lock {
        Lock(0)
    }
}
impl core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lock")
            .field("boot_cfg", &self.boot_cfg())
            .field("sjc_resp", &self.sjc_resp())
            .field("gp4_rlock", &self.gp4_rlock())
            .field("mac_addr", &self.mac_addr())
            .field("gp1", &self.gp1())
            .field("gp2", &self.gp2())
            .field("sw_gp1", &self.sw_gp1())
            .field("analog", &self.analog())
            .field("sw_gp2_lock", &self.sw_gp2_lock())
            .field("misc_conf", &self.misc_conf())
            .field("sw_gp2_rlock", &self.sw_gp2_rlock())
            .field("gp4", &self.gp4())
            .field("gp3", &self.gp3())
            .field("field_return", &self.field_return())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lock {{ boot_cfg: {=u8:?}, sjc_resp: {=bool:?}, gp4_rlock: {=bool:?}, mac_addr: {=u8:?}, gp1: {=u8:?}, gp2: {=u8:?}, sw_gp1: {=bool:?}, analog: {=u8:?}, sw_gp2_lock: {=bool:?}, misc_conf: {=bool:?}, sw_gp2_rlock: {=bool:?}, gp4: {=u8:?}, gp3: {=u8:?}, field_return: {:?} }}",
            self.boot_cfg(),
            self.sjc_resp(),
            self.gp4_rlock(),
            self.mac_addr(),
            self.gp1(),
            self.gp2(),
            self.sw_gp1(),
            self.analog(),
            self.sw_gp2_lock(),
            self.misc_conf(),
            self.sw_gp2_rlock(),
            self.gp4(),
            self.gp3(),
            self.field_return()
        )
    }
}
#[doc = "Value of OTP Bank4 Word2 (MAC Address)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mac0(pub u32);
impl Mac0 {
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
impl Default for Mac0 {
    #[inline(always)]
    fn default() -> Mac0 {
        Mac0(0)
    }
}
impl core::fmt::Debug for Mac0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mac0").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mac0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mac0 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank4 Word3 (MAC Address)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mac1(pub u32);
impl Mac1 {
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
impl Default for Mac1 {
    #[inline(always)]
    fn default() -> Mac1 {
        Mac1(0)
    }
}
impl core::fmt::Debug for Mac1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mac1").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mac1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mac1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank4 Word4 (MAC2 Address)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mac2(pub u32);
impl Mac2 {
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
impl Default for Mac2 {
    #[inline(always)]
    fn default() -> Mac2 {
        Mac2(0)
    }
}
impl core::fmt::Debug for Mac2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mac2").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mac2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mac2 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank1 Word0 (Memory Related Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mem0(pub u32);
impl Mem0 {
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
impl Default for Mem0 {
    #[inline(always)]
    fn default() -> Mem0 {
        Mem0(0)
    }
}
impl core::fmt::Debug for Mem0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mem0").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mem0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mem0 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank1 Word1 (Memory Related Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mem1(pub u32);
impl Mem1 {
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
impl Default for Mem1 {
    #[inline(always)]
    fn default() -> Mem1 {
        Mem1(0)
    }
}
impl core::fmt::Debug for Mem1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mem1").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mem1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mem1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank1 Word2 (Memory Related Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mem2(pub u32);
impl Mem2 {
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
impl Default for Mem2 {
    #[inline(always)]
    fn default() -> Mem2 {
        Mem2(0)
    }
}
impl core::fmt::Debug for Mem2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mem2").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mem2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mem2 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank1 Word3 (Memory Related Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mem3(pub u32);
impl Mem3 {
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
impl Default for Mem3 {
    #[inline(always)]
    fn default() -> Mem3 {
        Mem3(0)
    }
}
impl core::fmt::Debug for Mem3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mem3").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mem3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mem3 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank1 Word4 (Memory Related Info.)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mem4(pub u32);
impl Mem4 {
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
impl Default for Mem4 {
    #[inline(always)]
    fn default() -> Mem4 {
        Mem4(0)
    }
}
impl core::fmt::Debug for Mem4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mem4").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mem4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mem4 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank5 Word5 (Misc Conf)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscConf0(pub u32);
impl MiscConf0 {
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
impl Default for MiscConf0 {
    #[inline(always)]
    fn default() -> MiscConf0 {
        MiscConf0(0)
    }
}
impl core::fmt::Debug for MiscConf0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscConf0")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscConf0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MiscConf0 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank5 Word6 (Misc Conf)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscConf1(pub u32);
impl MiscConf1 {
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
impl Default for MiscConf1 {
    #[inline(always)]
    fn default() -> MiscConf1 {
        MiscConf1(0)
    }
}
impl core::fmt::Debug for MiscConf1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscConf1")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscConf1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MiscConf1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "OTP Controller Write Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReadCtrl(pub u32);
impl ReadCtrl {
    #[doc = "READ_FUSE"]
    #[must_use]
    #[inline(always)]
    pub const fn read_fuse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READ_FUSE"]
    #[inline(always)]
    pub const fn set_read_fuse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ReadCtrl {
    #[inline(always)]
    fn default() -> ReadCtrl {
        ReadCtrl(0)
    }
}
impl core::fmt::Debug for ReadCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ReadCtrl")
            .field("read_fuse", &self.read_fuse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ReadCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ReadCtrl {{ read_fuse: {=bool:?} }}", self.read_fuse())
    }
}
#[doc = "OTP Controller Read Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReadFuseData(pub u32);
impl ReadFuseData {
    #[doc = "The data read from OTP. See the section for operating details."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "The data read from OTP. See the section for operating details."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ReadFuseData {
    #[inline(always)]
    fn default() -> ReadFuseData {
        ReadFuseData(0)
    }
}
impl core::fmt::Debug for ReadFuseData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ReadFuseData")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ReadFuseData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ReadFuseData {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Software Controllable Signals Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scs(pub u32);
impl Scs {
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
    #[doc = "Unallocated read/write bits for implementation specific software use."]
    #[must_use]
    #[inline(always)]
    pub const fn spare(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Unallocated read/write bits for implementation specific software use."]
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
impl Default for Scs {
    #[inline(always)]
    fn default() -> Scs {
        Scs(0)
    }
}
impl core::fmt::Debug for Scs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scs")
            .field("hab_jde", &self.hab_jde())
            .field("spare", &self.spare())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scs {{ hab_jde: {:?}, spare: {=u32:?}, lock: {=bool:?} }}",
            self.hab_jde(),
            self.spare(),
            self.lock()
        )
    }
}
#[doc = "Software Controllable Signals Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScsClr(pub u32);
impl ScsClr {
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
    #[doc = "Unallocated read/write bits for implementation specific software use."]
    #[must_use]
    #[inline(always)]
    pub const fn spare(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Unallocated read/write bits for implementation specific software use."]
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
impl Default for ScsClr {
    #[inline(always)]
    fn default() -> ScsClr {
        ScsClr(0)
    }
}
impl core::fmt::Debug for ScsClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ScsClr")
            .field("hab_jde", &self.hab_jde())
            .field("spare", &self.spare())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ScsClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ScsClr {{ hab_jde: {=bool:?}, spare: {=u32:?}, lock: {=bool:?} }}",
            self.hab_jde(),
            self.spare(),
            self.lock()
        )
    }
}
#[doc = "Software Controllable Signals Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScsSet(pub u32);
impl ScsSet {
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
    #[doc = "Unallocated read/write bits for implementation specific software use."]
    #[must_use]
    #[inline(always)]
    pub const fn spare(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Unallocated read/write bits for implementation specific software use."]
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
impl Default for ScsSet {
    #[inline(always)]
    fn default() -> ScsSet {
        ScsSet(0)
    }
}
impl core::fmt::Debug for ScsSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ScsSet")
            .field("hab_jde", &self.hab_jde())
            .field("spare", &self.spare())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ScsSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ScsSet {{ hab_jde: {=bool:?}, spare: {=u32:?}, lock: {=bool:?} }}",
            self.hab_jde(),
            self.spare(),
            self.lock()
        )
    }
}
#[doc = "Software Controllable Signals Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScsTog(pub u32);
impl ScsTog {
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
    #[doc = "Unallocated read/write bits for implementation specific software use."]
    #[must_use]
    #[inline(always)]
    pub const fn spare(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Unallocated read/write bits for implementation specific software use."]
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
impl Default for ScsTog {
    #[inline(always)]
    fn default() -> ScsTog {
        ScsTog(0)
    }
}
impl core::fmt::Debug for ScsTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ScsTog")
            .field("hab_jde", &self.hab_jde())
            .field("spare", &self.spare())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ScsTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ScsTog {{ hab_jde: {=bool:?}, spare: {=u32:?}, lock: {=bool:?} }}",
            self.hab_jde(),
            self.spare(),
            self.lock()
        )
    }
}
#[doc = "Value of OTP Bank4 Word0 (Secure JTAG Response Field)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SjcResp0(pub u32);
impl SjcResp0 {
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
impl Default for SjcResp0 {
    #[inline(always)]
    fn default() -> SjcResp0 {
        SjcResp0(0)
    }
}
impl core::fmt::Debug for SjcResp0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SjcResp0")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SjcResp0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SjcResp0 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank4 Word1 (Secure JTAG Response Field)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SjcResp1(pub u32);
impl SjcResp1 {
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
impl Default for SjcResp1 {
    #[inline(always)]
    fn default() -> SjcResp1 {
        SjcResp1(0)
    }
}
impl core::fmt::Debug for SjcResp1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SjcResp1")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SjcResp1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SjcResp1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Shadow Register for OTP Bank3 Word0 (SRK Hash)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srk0(pub u32);
impl Srk0 {
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
impl Default for Srk0 {
    #[inline(always)]
    fn default() -> Srk0 {
        Srk0(0)
    }
}
impl core::fmt::Debug for Srk0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srk0").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srk0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srk0 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Shadow Register for OTP Bank3 Word1 (SRK Hash)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srk1(pub u32);
impl Srk1 {
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
impl Default for Srk1 {
    #[inline(always)]
    fn default() -> Srk1 {
        Srk1(0)
    }
}
impl core::fmt::Debug for Srk1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srk1").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srk1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srk1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Shadow Register for OTP Bank3 Word2 (SRK Hash)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srk2(pub u32);
impl Srk2 {
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
impl Default for Srk2 {
    #[inline(always)]
    fn default() -> Srk2 {
        Srk2(0)
    }
}
impl core::fmt::Debug for Srk2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srk2").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srk2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srk2 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Shadow Register for OTP Bank3 Word3 (SRK Hash)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srk3(pub u32);
impl Srk3 {
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
impl Default for Srk3 {
    #[inline(always)]
    fn default() -> Srk3 {
        Srk3(0)
    }
}
impl core::fmt::Debug for Srk3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srk3").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srk3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srk3 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Shadow Register for OTP Bank3 Word4 (SRK Hash)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srk4(pub u32);
impl Srk4 {
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
impl Default for Srk4 {
    #[inline(always)]
    fn default() -> Srk4 {
        Srk4(0)
    }
}
impl core::fmt::Debug for Srk4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srk4").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srk4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srk4 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Shadow Register for OTP Bank3 Word5 (SRK Hash)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srk5(pub u32);
impl Srk5 {
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
impl Default for Srk5 {
    #[inline(always)]
    fn default() -> Srk5 {
        Srk5(0)
    }
}
impl core::fmt::Debug for Srk5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srk5").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srk5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srk5 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Shadow Register for OTP Bank3 Word6 (SRK Hash)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srk6(pub u32);
impl Srk6 {
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
impl Default for Srk6 {
    #[inline(always)]
    fn default() -> Srk6 {
        Srk6(0)
    }
}
impl core::fmt::Debug for Srk6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srk6").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srk6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srk6 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Shadow Register for OTP Bank3 Word7 (SRK Hash)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srk7(pub u32);
impl Srk7 {
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
impl Default for Srk7 {
    #[inline(always)]
    fn default() -> Srk7 {
        Srk7(0)
    }
}
impl core::fmt::Debug for Srk7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srk7").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srk7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srk7 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank5 Word7 (SRK Revoke)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrkRevoke(pub u32);
impl SrkRevoke {
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
impl Default for SrkRevoke {
    #[inline(always)]
    fn default() -> SrkRevoke {
        SrkRevoke(0)
    }
}
impl core::fmt::Debug for SrkRevoke {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SrkRevoke")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SrkRevoke {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SrkRevoke {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank5 Word0 (SW GP1)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwGp1(pub u32);
impl SwGp1 {
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
impl Default for SwGp1 {
    #[inline(always)]
    fn default() -> SwGp1 {
        SwGp1(0)
    }
}
impl core::fmt::Debug for SwGp1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwGp1").field("bits", &self.bits()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwGp1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SwGp1 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank5 Word1 (SW GP2)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwGp20(pub u32);
impl SwGp20 {
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
impl Default for SwGp20 {
    #[inline(always)]
    fn default() -> SwGp20 {
        SwGp20(0)
    }
}
impl core::fmt::Debug for SwGp20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwGp20")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwGp20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SwGp20 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank5 Word2 (SW GP2)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwGp21(pub u32);
impl SwGp21 {
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
impl Default for SwGp21 {
    #[inline(always)]
    fn default() -> SwGp21 {
        SwGp21(0)
    }
}
impl core::fmt::Debug for SwGp21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwGp21")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwGp21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SwGp21 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank5 Word3 (SW GP2)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwGp22(pub u32);
impl SwGp22 {
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
impl Default for SwGp22 {
    #[inline(always)]
    fn default() -> SwGp22 {
        SwGp22(0)
    }
}
impl core::fmt::Debug for SwGp22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwGp22")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwGp22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SwGp22 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Value of OTP Bank5 Word4 (SW GP2)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwGp23(pub u32);
impl SwGp23 {
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
impl Default for SwGp23 {
    #[inline(always)]
    fn default() -> SwGp23 {
        SwGp23(0)
    }
}
impl core::fmt::Debug for SwGp23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwGp23")
            .field("bits", &self.bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwGp23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SwGp23 {{ bits: {=u32:?} }}", self.bits())
    }
}
#[doc = "Sticky bit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwSticky(pub u32);
impl SwSticky {
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
impl Default for SwSticky {
    #[inline(always)]
    fn default() -> SwSticky {
        SwSticky(0)
    }
}
impl core::fmt::Debug for SwSticky {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwSticky")
            .field("srk_revoke_lock", &self.srk_revoke_lock())
            .field("field_return_lock", &self.field_return_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwSticky {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SwSticky {{ srk_revoke_lock: {=bool:?}, field_return_lock: {=bool:?} }}",
            self.srk_revoke_lock(),
            self.field_return_lock()
        )
    }
}
#[doc = "OTP Controller Timing Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timing(pub u32);
impl Timing {
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
impl Default for Timing {
    #[inline(always)]
    fn default() -> Timing {
        Timing(0)
    }
}
impl core::fmt::Debug for Timing {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timing")
            .field("strobe_prog", &self.strobe_prog())
            .field("relax", &self.relax())
            .field("strobe_read", &self.strobe_read())
            .field("wait", &self.wait())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timing {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timing {{ strobe_prog: {=u16:?}, relax: {=u8:?}, strobe_read: {=u8:?}, wait: {=u8:?} }}",
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
pub struct Timing2(pub u32);
impl Timing2 {
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
}
impl Default for Timing2 {
    #[inline(always)]
    fn default() -> Timing2 {
        Timing2(0)
    }
}
impl core::fmt::Debug for Timing2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timing2")
            .field("relax_prog", &self.relax_prog())
            .field("relax_read", &self.relax_read())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timing2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timing2 {{ relax_prog: {=u16:?}, relax_read: {=u8:?} }}",
            self.relax_prog(),
            self.relax_read()
        )
    }
}
#[doc = "OTP Controller Version Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Version(pub u32);
impl Version {
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
impl Default for Version {
    #[inline(always)]
    fn default() -> Version {
        Version(0)
    }
}
impl core::fmt::Debug for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Version")
            .field("step", &self.step())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Version {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Version {{ step: {=u16:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.step(),
            self.minor(),
            self.major()
        )
    }
}
