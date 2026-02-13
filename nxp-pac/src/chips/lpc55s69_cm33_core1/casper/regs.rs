#[doc = "A register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Areg(pub u32);
impl Areg {
    #[doc = "Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[must_use]
    #[inline(always)]
    pub const fn reg_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub const fn set_reg_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Areg {
    #[inline(always)]
    fn default() -> Areg {
        Areg(0)
    }
}
impl core::fmt::Debug for Areg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Areg")
            .field("reg_value", &self.reg_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Areg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Areg {{ reg_value: {=u32:?} }}", self.reg_value())
    }
}
#[doc = "B register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Breg(pub u32);
impl Breg {
    #[doc = "Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[must_use]
    #[inline(always)]
    pub const fn reg_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub const fn set_reg_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Breg {
    #[inline(always)]
    fn default() -> Breg {
        Breg(0)
    }
}
impl core::fmt::Debug for Breg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Breg")
            .field("reg_value", &self.reg_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Breg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Breg {{ reg_value: {=u32:?} }}", self.reg_value())
    }
}
#[doc = "C register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Creg(pub u32);
impl Creg {
    #[doc = "Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[must_use]
    #[inline(always)]
    pub const fn reg_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub const fn set_reg_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Creg {
    #[inline(always)]
    fn default() -> Creg {
        Creg(0)
    }
}
impl core::fmt::Debug for Creg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Creg")
            .field("reg_value", &self.reg_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Creg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Creg {{ reg_value: {=u32:?} }}", self.reg_value())
    }
}
#[doc = "Contains the offsets of AB and CD in the RAM."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0(pub u32);
impl Ctrl0 {
    #[doc = "Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
    #[must_use]
    #[inline(always)]
    pub const fn abbpair(&self) -> super::vals::Abbpair {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Abbpair::from_bits(val as u8)
    }
    #[doc = "Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    pub const fn set_abbpair(&mut self, val: super::vals::Abbpair) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
    #[must_use]
    #[inline(always)]
    pub const fn aboff(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x07ff;
        val as u16
    }
    #[doc = "Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
    #[inline(always)]
    pub const fn set_aboff(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 2usize)) | (((val as u32) & 0x07ff) << 2usize);
    }
    #[doc = "Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
    #[must_use]
    #[inline(always)]
    pub const fn cdbpair(&self) -> super::vals::Cdbpair {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Cdbpair::from_bits(val as u8)
    }
    #[doc = "Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    pub const fn set_cdbpair(&mut self, val: super::vals::Cdbpair) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
    #[must_use]
    #[inline(always)]
    pub const fn cdoff(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
    #[inline(always)]
    pub const fn set_cdoff(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
}
impl Default for Ctrl0 {
    #[inline(always)]
    fn default() -> Ctrl0 {
        Ctrl0(0)
    }
}
impl core::fmt::Debug for Ctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0")
            .field("abbpair", &self.abbpair())
            .field("aboff", &self.aboff())
            .field("cdbpair", &self.cdbpair())
            .field("cdoff", &self.cdoff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl0 {{ abbpair: {:?}, aboff: {=u16:?}, cdbpair: {:?}, cdoff: {=u16:?} }}",
            self.abbpair(),
            self.aboff(),
            self.cdbpair(),
            self.cdoff()
        )
    }
}
#[doc = "Contains the opcode mode, iteration count, and result offset (in RAM) and also launches the accelerator. Note: with CP version: CTRL0 and CRTL1 can be written in one go with MCRR."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u32);
impl Ctrl1 {
    #[doc = "Iteration counter. Is number_cycles - 1. write 0 means Does one cycle - does not iterate."]
    #[must_use]
    #[inline(always)]
    pub const fn iter(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Iteration counter. Is number_cycles - 1. write 0 means Does one cycle - does not iterate."]
    #[inline(always)]
    pub const fn set_iter(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Operation mode to perform. write 0 means Accelerator is inactive. write others means accelerator is active."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Operation mode to perform. write 0 means Accelerator is inactive. write others means accelerator is active."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Which bank-pair the offset RESOFF is within. This must be 0 if only 2-up. Ideally this is not the same bank as ABBPAIR (when 4-up supported)"]
    #[must_use]
    #[inline(always)]
    pub const fn resbpair(&self) -> super::vals::Resbpair {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Resbpair::from_bits(val as u8)
    }
    #[doc = "Which bank-pair the offset RESOFF is within. This must be 0 if only 2-up. Ideally this is not the same bank as ABBPAIR (when 4-up supported)"]
    #[inline(always)]
    pub const fn set_resbpair(&mut self, val: super::vals::Resbpair) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Word or DWord Offset of result. Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB and CD values"]
    #[must_use]
    #[inline(always)]
    pub const fn resoff(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "Word or DWord Offset of result. Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB and CD values"]
    #[inline(always)]
    pub const fn set_resoff(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
    #[doc = "Skip rules on Carry if needed. This operation will be skipped based on Carry value (from previous operation) if not 0:"]
    #[must_use]
    #[inline(always)]
    pub const fn cskip(&self) -> super::vals::Cskip {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Cskip::from_bits(val as u8)
    }
    #[doc = "Skip rules on Carry if needed. This operation will be skipped based on Carry value (from previous operation) if not 0:"]
    #[inline(always)]
    pub const fn set_cskip(&mut self, val: super::vals::Cskip) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Ctrl1 {
    #[inline(always)]
    fn default() -> Ctrl1 {
        Ctrl1(0)
    }
}
impl core::fmt::Debug for Ctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1")
            .field("iter", &self.iter())
            .field("mode", &self.mode())
            .field("resbpair", &self.resbpair())
            .field("resoff", &self.resoff())
            .field("cskip", &self.cskip())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl1 {{ iter: {=u8:?}, mode: {=u8:?}, resbpair: {:?}, resoff: {=u16:?}, cskip: {:?} }}",
            self.iter(),
            self.mode(),
            self.resbpair(),
            self.resoff(),
            self.cskip()
        )
    }
}
#[doc = "D register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dreg(pub u32);
impl Dreg {
    #[doc = "Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[must_use]
    #[inline(always)]
    pub const fn reg_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub const fn set_reg_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dreg {
    #[inline(always)]
    fn default() -> Dreg {
        Dreg(0)
    }
}
impl core::fmt::Debug for Dreg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dreg")
            .field("reg_value", &self.reg_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dreg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dreg {{ reg_value: {=u32:?} }}", self.reg_value())
    }
}
#[doc = "Clears interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenclr(pub u32);
impl Intenclr {
    #[doc = "Written to clear an interrupt set with INTENSET."]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> super::vals::IntenclrDone {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::IntenclrDone::from_bits(val as u8)
    }
    #[doc = "Written to clear an interrupt set with INTENSET."]
    #[inline(always)]
    pub const fn set_done(&mut self, val: super::vals::IntenclrDone) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Intenclr {
    #[inline(always)]
    fn default() -> Intenclr {
        Intenclr(0)
    }
}
impl core::fmt::Debug for Intenclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intenclr")
            .field("done", &self.done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intenclr {{ done: {:?} }}", self.done())
    }
}
#[doc = "Sets interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenset(pub u32);
impl Intenset {
    #[doc = "Set if the accelerator should interrupt when done."]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set if the accelerator should interrupt when done."]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Intenset {
    #[inline(always)]
    fn default() -> Intenset {
        Intenset(0)
    }
}
impl core::fmt::Debug for Intenset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intenset")
            .field("done", &self.done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intenset {{ done: {=bool:?} }}", self.done())
    }
}
#[doc = "Interrupt status bits (mask of INTENSET and STATUS)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "If set, interrupt is caused by accelerator being done."]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If set, interrupt is caused by accelerator being done."]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Intstat {
    #[inline(always)]
    fn default() -> Intstat {
        Intstat(0)
    }
}
impl core::fmt::Debug for Intstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intstat")
            .field("done", &self.done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intstat {{ done: {=bool:?} }}", self.done())
    }
}
#[doc = "Contains an optional loader to load into CTRL0/1 in steps to perform a set of operations."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Loader(pub u32);
impl Loader {
    #[doc = "Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
    #[must_use]
    #[inline(always)]
    pub const fn ctrlbpair(&self) -> super::vals::Ctrlbpair {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ctrlbpair::from_bits(val as u8)
    }
    #[doc = "Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
    #[inline(always)]
    pub const fn set_ctrlbpair(&mut self, val: super::vals::Ctrlbpair) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DWord Offset of CTRL pair to load next."]
    #[must_use]
    #[inline(always)]
    pub const fn ctrloff(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "DWord Offset of CTRL pair to load next."]
    #[inline(always)]
    pub const fn set_ctrloff(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
}
impl Default for Loader {
    #[inline(always)]
    fn default() -> Loader {
        Loader(0)
    }
}
impl core::fmt::Debug for Loader {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Loader")
            .field("count", &self.count())
            .field("ctrlbpair", &self.ctrlbpair())
            .field("ctrloff", &self.ctrloff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Loader {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Loader {{ count: {=u8:?}, ctrlbpair: {:?}, ctrloff: {=u16:?} }}",
            self.count(),
            self.ctrlbpair(),
            self.ctrloff()
        )
    }
}
#[doc = "Security lock register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc = "Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Must be written as 0x73D to change the register."]
    #[must_use]
    #[inline(always)]
    pub const fn key(&self) -> super::vals::Key {
        let val = (self.0 >> 4usize) & 0x1fff;
        super::vals::Key::from_bits(val as u16)
    }
    #[doc = "Must be written as 0x73D to change the register."]
    #[inline(always)]
    pub const fn set_key(&mut self, val: super::vals::Key) {
        self.0 = (self.0 & !(0x1fff << 4usize)) | (((val.to_bits() as u32) & 0x1fff) << 4usize);
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
            .field("lock", &self.lock())
            .field("key", &self.key())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lock {{ lock: {=bool:?}, key: {:?} }}",
            self.lock(),
            self.key()
        )
    }
}
#[doc = "Optional mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask(pub u32);
impl Mask {
    #[doc = "Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
    #[must_use]
    #[inline(always)]
    pub const fn mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
    #[inline(always)]
    pub const fn set_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mask {
    #[inline(always)]
    fn default() -> Mask {
        Mask(0)
    }
}
impl core::fmt::Debug for Mask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mask").field("mask", &self.mask()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mask {{ mask: {=u32:?} }}", self.mask())
    }
}
#[doc = "Optional re-mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Remask(pub u32);
impl Remask {
    #[doc = "Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
    #[must_use]
    #[inline(always)]
    pub const fn mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
    #[inline(always)]
    pub const fn set_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Remask {
    #[inline(always)]
    fn default() -> Remask {
        Remask(0)
    }
}
impl core::fmt::Debug for Remask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Remask")
            .field("mask", &self.mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Remask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Remask {{ mask: {=u32:?} }}", self.mask())
    }
}
#[doc = "Result register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Res0(pub u32);
impl Res0 {
    #[doc = "Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[must_use]
    #[inline(always)]
    pub const fn reg_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub const fn set_reg_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Res0 {
    #[inline(always)]
    fn default() -> Res0 {
        Res0(0)
    }
}
impl core::fmt::Debug for Res0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Res0")
            .field("reg_value", &self.reg_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Res0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Res0 {{ reg_value: {=u32:?} }}", self.reg_value())
    }
}
#[doc = "Result register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Res1(pub u32);
impl Res1 {
    #[doc = "Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[must_use]
    #[inline(always)]
    pub const fn reg_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub const fn set_reg_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Res1 {
    #[inline(always)]
    fn default() -> Res1 {
        Res1(0)
    }
}
impl core::fmt::Debug for Res1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Res1")
            .field("reg_value", &self.reg_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Res1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Res1 {{ reg_value: {=u32:?} }}", self.reg_value())
    }
}
#[doc = "Result register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Res2(pub u32);
impl Res2 {
    #[doc = "Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[must_use]
    #[inline(always)]
    pub const fn reg_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub const fn set_reg_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Res2 {
    #[inline(always)]
    fn default() -> Res2 {
        Res2(0)
    }
}
impl core::fmt::Debug for Res2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Res2")
            .field("reg_value", &self.reg_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Res2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Res2 {{ reg_value: {=u32:?} }}", self.reg_value())
    }
}
#[doc = "Result register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Res3(pub u32);
impl Res3 {
    #[doc = "Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[must_use]
    #[inline(always)]
    pub const fn reg_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub const fn set_reg_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Res3 {
    #[inline(always)]
    fn default() -> Res3 {
        Res3(0)
    }
}
impl core::fmt::Debug for Res3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Res3")
            .field("reg_value", &self.reg_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Res3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Res3 {{ reg_value: {=u32:?} }}", self.reg_value())
    }
}
#[doc = "Indicates operational status and would contain the carry bit if used."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Indicates if the accelerator has finished an operation. Write 1 to clear, or write CTRL1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> super::vals::StatusDone {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::StatusDone::from_bits(val as u8)
    }
    #[doc = "Indicates if the accelerator has finished an operation. Write 1 to clear, or write CTRL1 to clear."]
    #[inline(always)]
    pub const fn set_done(&mut self, val: super::vals::StatusDone) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Last carry value if operation produced a carry bit"]
    #[must_use]
    #[inline(always)]
    pub const fn carry(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Last carry value if operation produced a carry bit"]
    #[inline(always)]
    pub const fn set_carry(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates if the accelerator is busy performing an operation"]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> super::vals::Busy {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Busy::from_bits(val as u8)
    }
    #[doc = "Indicates if the accelerator is busy performing an operation"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: super::vals::Busy) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("done", &self.done())
            .field("carry", &self.carry())
            .field("busy", &self.busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ done: {:?}, carry: {=bool:?}, busy: {:?} }}",
            self.done(),
            self.carry(),
            self.busy()
        )
    }
}
