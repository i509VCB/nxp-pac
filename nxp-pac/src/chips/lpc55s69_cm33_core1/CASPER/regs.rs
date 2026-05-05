#[doc = "A register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AREG(pub u32);
impl AREG {
    #[doc = "Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[must_use]
    #[inline(always)]
    pub const fn REG_VALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub const fn set_REG_VALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AREG {
    #[inline(always)]
    fn default() -> AREG {
        AREG(0)
    }
}
impl core::fmt::Debug for AREG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AREG")
            .field("REG_VALUE", &self.REG_VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AREG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AREG {{ REG_VALUE: {=u32:?} }}", self.REG_VALUE())
    }
}
#[doc = "B register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BREG(pub u32);
impl BREG {
    #[doc = "Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[must_use]
    #[inline(always)]
    pub const fn REG_VALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub const fn set_REG_VALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BREG {
    #[inline(always)]
    fn default() -> BREG {
        BREG(0)
    }
}
impl core::fmt::Debug for BREG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BREG")
            .field("REG_VALUE", &self.REG_VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BREG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BREG {{ REG_VALUE: {=u32:?} }}", self.REG_VALUE())
    }
}
#[doc = "C register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CREG(pub u32);
impl CREG {
    #[doc = "Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[must_use]
    #[inline(always)]
    pub const fn REG_VALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub const fn set_REG_VALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CREG {
    #[inline(always)]
    fn default() -> CREG {
        CREG(0)
    }
}
impl core::fmt::Debug for CREG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CREG")
            .field("REG_VALUE", &self.REG_VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CREG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CREG {{ REG_VALUE: {=u32:?} }}", self.REG_VALUE())
    }
}
#[doc = "Contains the offsets of AB and CD in the RAM."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL0(pub u32);
impl CTRL0 {
    #[doc = "Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up."]
    #[must_use]
    #[inline(always)]
    pub const fn ABBPAIR(&self) -> super::vals::ABBPAIR {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ABBPAIR::from_bits(val as u8)
    }
    #[doc = "Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up."]
    #[inline(always)]
    pub const fn set_ABBPAIR(&mut self, val: super::vals::ABBPAIR) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up."]
    #[must_use]
    #[inline(always)]
    pub const fn ABOFF(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x07ff;
        val as u16
    }
    #[doc = "Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up."]
    #[inline(always)]
    pub const fn set_ABOFF(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 2usize)) | (((val as u32) & 0x07ff) << 2usize);
    }
    #[doc = "Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up."]
    #[must_use]
    #[inline(always)]
    pub const fn CDBPAIR(&self) -> super::vals::CDBPAIR {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::CDBPAIR::from_bits(val as u8)
    }
    #[doc = "Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up."]
    #[inline(always)]
    pub const fn set_CDBPAIR(&mut self, val: super::vals::CDBPAIR) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values."]
    #[must_use]
    #[inline(always)]
    pub const fn CDOFF(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values."]
    #[inline(always)]
    pub const fn set_CDOFF(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
}
impl Default for CTRL0 {
    #[inline(always)]
    fn default() -> CTRL0 {
        CTRL0(0)
    }
}
impl core::fmt::Debug for CTRL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL0")
            .field("ABBPAIR", &self.ABBPAIR())
            .field("ABOFF", &self.ABOFF())
            .field("CDBPAIR", &self.CDBPAIR())
            .field("CDOFF", &self.CDOFF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL0 {{ ABBPAIR: {:?}, ABOFF: {=u16:?}, CDBPAIR: {:?}, CDOFF: {=u16:?} }}",
            self.ABBPAIR(),
            self.ABOFF(),
            self.CDBPAIR(),
            self.CDOFF()
        )
    }
}
#[doc = "Contains the opcode mode, iteration count, and result offset (in RAM) and also launches the accelerator. Note: with CP version: CTRL0 and CRTL1 can be written in one go with MCRR."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL1(pub u32);
impl CTRL1 {
    #[doc = "Iteration counter. Is number_cycles - 1. write 0 means Does one cycle - does not iterate."]
    #[must_use]
    #[inline(always)]
    pub const fn ITER(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Iteration counter. Is number_cycles - 1. write 0 means Does one cycle - does not iterate."]
    #[inline(always)]
    pub const fn set_ITER(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Operation mode to perform. write 0 means Accelerator is inactive. write others means accelerator is active."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Operation mode to perform. write 0 means Accelerator is inactive. write others means accelerator is active."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Which bank-pair the offset RESOFF is within. This must be 0 if only 2-up. Ideally this is not the same bank as ABBPAIR (when 4-up supported)."]
    #[must_use]
    #[inline(always)]
    pub const fn RESBPAIR(&self) -> super::vals::RESBPAIR {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::RESBPAIR::from_bits(val as u8)
    }
    #[doc = "Which bank-pair the offset RESOFF is within. This must be 0 if only 2-up. Ideally this is not the same bank as ABBPAIR (when 4-up supported)."]
    #[inline(always)]
    pub const fn set_RESBPAIR(&mut self, val: super::vals::RESBPAIR) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Word or DWord Offset of result. Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB and CD values."]
    #[must_use]
    #[inline(always)]
    pub const fn RESOFF(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "Word or DWord Offset of result. Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB and CD values."]
    #[inline(always)]
    pub const fn set_RESOFF(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
    #[doc = "Skip rules on Carry if needed. This operation will be skipped based on Carry value (from previous operation) if not 0:."]
    #[must_use]
    #[inline(always)]
    pub const fn CSKIP(&self) -> super::vals::CSKIP {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::CSKIP::from_bits(val as u8)
    }
    #[doc = "Skip rules on Carry if needed. This operation will be skipped based on Carry value (from previous operation) if not 0:."]
    #[inline(always)]
    pub const fn set_CSKIP(&mut self, val: super::vals::CSKIP) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for CTRL1 {
    #[inline(always)]
    fn default() -> CTRL1 {
        CTRL1(0)
    }
}
impl core::fmt::Debug for CTRL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("ITER", &self.ITER())
            .field("MODE", &self.MODE())
            .field("RESBPAIR", &self.RESBPAIR())
            .field("RESOFF", &self.RESOFF())
            .field("CSKIP", &self.CSKIP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL1 {{ ITER: {=u8:?}, MODE: {=u8:?}, RESBPAIR: {:?}, RESOFF: {=u16:?}, CSKIP: {:?} }}",
            self.ITER(),
            self.MODE(),
            self.RESBPAIR(),
            self.RESOFF(),
            self.CSKIP()
        )
    }
}
#[doc = "D register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DREG(pub u32);
impl DREG {
    #[doc = "Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[must_use]
    #[inline(always)]
    pub const fn REG_VALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub const fn set_REG_VALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DREG {
    #[inline(always)]
    fn default() -> DREG {
        DREG(0)
    }
}
impl core::fmt::Debug for DREG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DREG")
            .field("REG_VALUE", &self.REG_VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DREG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DREG {{ REG_VALUE: {=u32:?} }}", self.REG_VALUE())
    }
}
#[doc = "Clears interrupts."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTENCLR(pub u32);
impl INTENCLR {
    #[doc = "Written to clear an interrupt set with INTENSET."]
    #[must_use]
    #[inline(always)]
    pub const fn DONE(&self) -> super::vals::INTENCLR_DONE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::INTENCLR_DONE::from_bits(val as u8)
    }
    #[doc = "Written to clear an interrupt set with INTENSET."]
    #[inline(always)]
    pub const fn set_DONE(&mut self, val: super::vals::INTENCLR_DONE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for INTENCLR {
    #[inline(always)]
    fn default() -> INTENCLR {
        INTENCLR(0)
    }
}
impl core::fmt::Debug for INTENCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENCLR")
            .field("DONE", &self.DONE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTENCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INTENCLR {{ DONE: {:?} }}", self.DONE())
    }
}
#[doc = "Sets interrupts."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTENSET(pub u32);
impl INTENSET {
    #[doc = "Set if the accelerator should interrupt when done."]
    #[must_use]
    #[inline(always)]
    pub const fn DONE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set if the accelerator should interrupt when done."]
    #[inline(always)]
    pub const fn set_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for INTENSET {
    #[inline(always)]
    fn default() -> INTENSET {
        INTENSET(0)
    }
}
impl core::fmt::Debug for INTENSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENSET")
            .field("DONE", &self.DONE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTENSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INTENSET {{ DONE: {=bool:?} }}", self.DONE())
    }
}
#[doc = "Interrupt status bits (mask of INTENSET and STATUS)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTSTAT(pub u32);
impl INTSTAT {
    #[doc = "If set, interrupt is caused by accelerator being done."]
    #[must_use]
    #[inline(always)]
    pub const fn DONE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If set, interrupt is caused by accelerator being done."]
    #[inline(always)]
    pub const fn set_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for INTSTAT {
    #[inline(always)]
    fn default() -> INTSTAT {
        INTSTAT(0)
    }
}
impl core::fmt::Debug for INTSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTAT")
            .field("DONE", &self.DONE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INTSTAT {{ DONE: {=bool:?} }}", self.DONE())
    }
}
#[doc = "Contains an optional loader to load into CTRL0/1 in steps to perform a set of operations."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LOADER(pub u32);
impl LOADER {
    #[doc = "Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load."]
    #[must_use]
    #[inline(always)]
    pub const fn COUNT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load."]
    #[inline(always)]
    pub const fn set_COUNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
    #[must_use]
    #[inline(always)]
    pub const fn CTRLBPAIR(&self) -> super::vals::CTRLBPAIR {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::CTRLBPAIR::from_bits(val as u8)
    }
    #[doc = "Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
    #[inline(always)]
    pub const fn set_CTRLBPAIR(&mut self, val: super::vals::CTRLBPAIR) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DWord Offset of CTRL pair to load next."]
    #[must_use]
    #[inline(always)]
    pub const fn CTRLOFF(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "DWord Offset of CTRL pair to load next."]
    #[inline(always)]
    pub const fn set_CTRLOFF(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
}
impl Default for LOADER {
    #[inline(always)]
    fn default() -> LOADER {
        LOADER(0)
    }
}
impl core::fmt::Debug for LOADER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOADER")
            .field("COUNT", &self.COUNT())
            .field("CTRLBPAIR", &self.CTRLBPAIR())
            .field("CTRLOFF", &self.CTRLOFF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LOADER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LOADER {{ COUNT: {=u8:?}, CTRLBPAIR: {:?}, CTRLOFF: {=u16:?} }}",
            self.COUNT(),
            self.CTRLBPAIR(),
            self.CTRLOFF()
        )
    }
}
#[doc = "Security lock register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LOCK(pub u32);
impl LOCK {
    #[doc = "Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock."]
    #[inline(always)]
    pub const fn set_LOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Must be written as 0x73D to change the register."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY(&self) -> super::vals::KEY {
        let val = (self.0 >> 4usize) & 0x1fff;
        super::vals::KEY::from_bits(val as u16)
    }
    #[doc = "Must be written as 0x73D to change the register."]
    #[inline(always)]
    pub const fn set_KEY(&mut self, val: super::vals::KEY) {
        self.0 = (self.0 & !(0x1fff << 4usize)) | (((val.to_bits() as u32) & 0x1fff) << 4usize);
    }
}
impl Default for LOCK {
    #[inline(always)]
    fn default() -> LOCK {
        LOCK(0)
    }
}
impl core::fmt::Debug for LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCK")
            .field("LOCK", &self.LOCK())
            .field("KEY", &self.KEY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LOCK {{ LOCK: {=bool:?}, KEY: {:?} }}",
            self.LOCK(),
            self.KEY()
        )
    }
}
#[doc = "Optional mask register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MASK(pub u32);
impl MASK {
    #[doc = "Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values."]
    #[must_use]
    #[inline(always)]
    pub const fn MASK(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values."]
    #[inline(always)]
    pub const fn set_MASK(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MASK {
    #[inline(always)]
    fn default() -> MASK {
        MASK(0)
    }
}
impl core::fmt::Debug for MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK").field("MASK", &self.MASK()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MASK {{ MASK: {=u32:?} }}", self.MASK())
    }
}
#[doc = "Optional re-mask register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct REMASK(pub u32);
impl REMASK {
    #[doc = "Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values."]
    #[must_use]
    #[inline(always)]
    pub const fn MASK(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values."]
    #[inline(always)]
    pub const fn set_MASK(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for REMASK {
    #[inline(always)]
    fn default() -> REMASK {
        REMASK(0)
    }
}
impl core::fmt::Debug for REMASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMASK")
            .field("MASK", &self.MASK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for REMASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "REMASK {{ MASK: {=u32:?} }}", self.MASK())
    }
}
#[doc = "Result register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RES0(pub u32);
impl RES0 {
    #[doc = "Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[must_use]
    #[inline(always)]
    pub const fn REG_VALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub const fn set_REG_VALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RES0 {
    #[inline(always)]
    fn default() -> RES0 {
        RES0(0)
    }
}
impl core::fmt::Debug for RES0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RES0")
            .field("REG_VALUE", &self.REG_VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RES0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RES0 {{ REG_VALUE: {=u32:?} }}", self.REG_VALUE())
    }
}
#[doc = "Result register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RES1(pub u32);
impl RES1 {
    #[doc = "Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[must_use]
    #[inline(always)]
    pub const fn REG_VALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub const fn set_REG_VALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RES1 {
    #[inline(always)]
    fn default() -> RES1 {
        RES1(0)
    }
}
impl core::fmt::Debug for RES1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RES1")
            .field("REG_VALUE", &self.REG_VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RES1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RES1 {{ REG_VALUE: {=u32:?} }}", self.REG_VALUE())
    }
}
#[doc = "Result register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RES2(pub u32);
impl RES2 {
    #[doc = "Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[must_use]
    #[inline(always)]
    pub const fn REG_VALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub const fn set_REG_VALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RES2 {
    #[inline(always)]
    fn default() -> RES2 {
        RES2(0)
    }
}
impl core::fmt::Debug for RES2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RES2")
            .field("REG_VALUE", &self.REG_VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RES2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RES2 {{ REG_VALUE: {=u32:?} }}", self.REG_VALUE())
    }
}
#[doc = "Result register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RES3(pub u32);
impl RES3 {
    #[doc = "Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[must_use]
    #[inline(always)]
    pub const fn REG_VALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub const fn set_REG_VALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RES3 {
    #[inline(always)]
    fn default() -> RES3 {
        RES3(0)
    }
}
impl core::fmt::Debug for RES3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RES3")
            .field("REG_VALUE", &self.REG_VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RES3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RES3 {{ REG_VALUE: {=u32:?} }}", self.REG_VALUE())
    }
}
#[doc = "Indicates operational status and would contain the carry bit if used."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STATUS(pub u32);
impl STATUS {
    #[doc = "Indicates if the accelerator has finished an operation. Write 1 to clear, or write CTRL1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn DONE(&self) -> super::vals::STATUS_DONE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::STATUS_DONE::from_bits(val as u8)
    }
    #[doc = "Indicates if the accelerator has finished an operation. Write 1 to clear, or write CTRL1 to clear."]
    #[inline(always)]
    pub const fn set_DONE(&mut self, val: super::vals::STATUS_DONE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Last carry value if operation produced a carry bit."]
    #[must_use]
    #[inline(always)]
    pub const fn CARRY(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Last carry value if operation produced a carry bit."]
    #[inline(always)]
    pub const fn set_CARRY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates if the accelerator is busy performing an operation."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSY(&self) -> super::vals::BUSY {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::BUSY::from_bits(val as u8)
    }
    #[doc = "Indicates if the accelerator is busy performing an operation."]
    #[inline(always)]
    pub const fn set_BUSY(&mut self, val: super::vals::BUSY) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for STATUS {
    #[inline(always)]
    fn default() -> STATUS {
        STATUS(0)
    }
}
impl core::fmt::Debug for STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("DONE", &self.DONE())
            .field("CARRY", &self.CARRY())
            .field("BUSY", &self.BUSY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STATUS {{ DONE: {:?}, CARRY: {=bool:?}, BUSY: {:?} }}",
            self.DONE(),
            self.CARRY(),
            self.BUSY()
        )
    }
}
