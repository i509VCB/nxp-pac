#[doc = "command register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMD(pub u32);
impl CMD {
    #[doc = "command register."]
    #[must_use]
    #[inline(always)]
    pub const fn CMD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "command register."]
    #[inline(always)]
    pub const fn set_CMD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CMD {
    #[inline(always)]
    fn default() -> CMD {
        CMD(0)
    }
}
impl core::fmt::Debug for CMD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD").field("CMD", &self.CMD()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CMD {{ CMD: {=u32:?} }}", self.CMD())
    }
}
#[doc = "data register, word 0-7; Memory data, or command parameter, or command result."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DATAW(pub u32);
impl DATAW {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn DATAW(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_DATAW(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DATAW {
    #[inline(always)]
    fn default() -> DATAW {
        DATAW(0)
    }
}
impl core::fmt::Debug for DATAW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATAW")
            .field("DATAW", &self.DATAW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DATAW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DATAW {{ DATAW: {=u32:?} }}", self.DATAW())
    }
}
#[doc = "event register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVENT(pub u32);
impl EVENT {
    #[doc = "When bit is set, the controller and flash are reset."]
    #[must_use]
    #[inline(always)]
    pub const fn RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When bit is set, the controller and flash are reset."]
    #[inline(always)]
    pub const fn set_RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When bit is set, the controller wakes up from whatever low power or powerdown mode was active."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKEUP(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When bit is set, the controller wakes up from whatever low power or powerdown mode was active."]
    #[inline(always)]
    pub const fn set_WAKEUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When bit is set, a running program/erase command is aborted."]
    #[must_use]
    #[inline(always)]
    pub const fn ABORT(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When bit is set, a running program/erase command is aborted."]
    #[inline(always)]
    pub const fn set_ABORT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for EVENT {
    #[inline(always)]
    fn default() -> EVENT {
        EVENT(0)
    }
}
impl core::fmt::Debug for EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVENT")
            .field("RST", &self.RST())
            .field("WAKEUP", &self.WAKEUP())
            .field("ABORT", &self.ABORT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EVENT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EVENT {{ RST: {=bool:?}, WAKEUP: {=bool:?}, ABORT: {=bool:?} }}",
            self.RST(),
            self.WAKEUP(),
            self.ABORT()
        )
    }
}
#[doc = "Clear interrupt enable bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INT_CLR_ENABLE(pub u32);
impl INT_CLR_ENABLE {
    #[doc = "When a CLR_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn FAIL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When a CLR_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is cleared."]
    #[inline(always)]
    pub const fn set_FAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When a CLR_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn ERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When a CLR_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is cleared."]
    #[inline(always)]
    pub const fn set_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When a CLR_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn DONE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When a CLR_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is cleared."]
    #[inline(always)]
    pub const fn set_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When a CLR_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn ECC_ERR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When a CLR_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is cleared."]
    #[inline(always)]
    pub const fn set_ECC_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for INT_CLR_ENABLE {
    #[inline(always)]
    fn default() -> INT_CLR_ENABLE {
        INT_CLR_ENABLE(0)
    }
}
impl core::fmt::Debug for INT_CLR_ENABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_CLR_ENABLE")
            .field("FAIL", &self.FAIL())
            .field("ERR", &self.ERR())
            .field("DONE", &self.DONE())
            .field("ECC_ERR", &self.ECC_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INT_CLR_ENABLE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INT_CLR_ENABLE {{ FAIL: {=bool:?}, ERR: {=bool:?}, DONE: {=bool:?}, ECC_ERR: {=bool:?} }}",
            self.FAIL(),
            self.ERR(),
            self.DONE(),
            self.ECC_ERR()
        )
    }
}
#[doc = "Clear interrupt status bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INT_CLR_STATUS(pub u32);
impl INT_CLR_STATUS {
    #[doc = "When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn FAIL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[inline(always)]
    pub const fn set_FAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn ERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[inline(always)]
    pub const fn set_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn DONE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[inline(always)]
    pub const fn set_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn ECC_ERR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[inline(always)]
    pub const fn set_ECC_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for INT_CLR_STATUS {
    #[inline(always)]
    fn default() -> INT_CLR_STATUS {
        INT_CLR_STATUS(0)
    }
}
impl core::fmt::Debug for INT_CLR_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_CLR_STATUS")
            .field("FAIL", &self.FAIL())
            .field("ERR", &self.ERR())
            .field("DONE", &self.DONE())
            .field("ECC_ERR", &self.ECC_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INT_CLR_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INT_CLR_STATUS {{ FAIL: {=bool:?}, ERR: {=bool:?}, DONE: {=bool:?}, ECC_ERR: {=bool:?} }}",
            self.FAIL(),
            self.ERR(),
            self.DONE(),
            self.ECC_ERR()
        )
    }
}
#[doc = "Interrupt enable bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INT_ENABLE(pub u32);
impl INT_ENABLE {
    #[doc = "If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[must_use]
    #[inline(always)]
    pub const fn FAIL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub const fn set_FAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[must_use]
    #[inline(always)]
    pub const fn ERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub const fn set_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[must_use]
    #[inline(always)]
    pub const fn DONE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub const fn set_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[must_use]
    #[inline(always)]
    pub const fn ECC_ERR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub const fn set_ECC_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for INT_ENABLE {
    #[inline(always)]
    fn default() -> INT_ENABLE {
        INT_ENABLE(0)
    }
}
impl core::fmt::Debug for INT_ENABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENABLE")
            .field("FAIL", &self.FAIL())
            .field("ERR", &self.ERR())
            .field("DONE", &self.DONE())
            .field("ECC_ERR", &self.ECC_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INT_ENABLE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INT_ENABLE {{ FAIL: {=bool:?}, ERR: {=bool:?}, DONE: {=bool:?}, ECC_ERR: {=bool:?} }}",
            self.FAIL(),
            self.ERR(),
            self.DONE(),
            self.ECC_ERR()
        )
    }
}
#[doc = "Set interrupt enable bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INT_SET_ENABLE(pub u32);
impl INT_SET_ENABLE {
    #[doc = "When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn FAIL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[inline(always)]
    pub const fn set_FAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn ERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[inline(always)]
    pub const fn set_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DONE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[inline(always)]
    pub const fn set_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn ECC_ERR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[inline(always)]
    pub const fn set_ECC_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for INT_SET_ENABLE {
    #[inline(always)]
    fn default() -> INT_SET_ENABLE {
        INT_SET_ENABLE(0)
    }
}
impl core::fmt::Debug for INT_SET_ENABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_SET_ENABLE")
            .field("FAIL", &self.FAIL())
            .field("ERR", &self.ERR())
            .field("DONE", &self.DONE())
            .field("ECC_ERR", &self.ECC_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INT_SET_ENABLE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INT_SET_ENABLE {{ FAIL: {=bool:?}, ERR: {=bool:?}, DONE: {=bool:?}, ECC_ERR: {=bool:?} }}",
            self.FAIL(),
            self.ERR(),
            self.DONE(),
            self.ECC_ERR()
        )
    }
}
#[doc = "Set interrupt status bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INT_SET_STATUS(pub u32);
impl INT_SET_STATUS {
    #[doc = "When a SET_STATUS bit is written to 1, the corresponding INT_STATUS bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn FAIL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When a SET_STATUS bit is written to 1, the corresponding INT_STATUS bit is set."]
    #[inline(always)]
    pub const fn set_FAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When a SET_STATUS bit is written to 1, the corresponding INT_STATUS bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn ERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When a SET_STATUS bit is written to 1, the corresponding INT_STATUS bit is set."]
    #[inline(always)]
    pub const fn set_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When a SET_STATUS bit is written to 1, the corresponding INT_STATUS bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DONE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When a SET_STATUS bit is written to 1, the corresponding INT_STATUS bit is set."]
    #[inline(always)]
    pub const fn set_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When a SET_STATUS bit is written to 1, the corresponding INT_STATUS bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn ECC_ERR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When a SET_STATUS bit is written to 1, the corresponding INT_STATUS bit is set."]
    #[inline(always)]
    pub const fn set_ECC_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for INT_SET_STATUS {
    #[inline(always)]
    fn default() -> INT_SET_STATUS {
        INT_SET_STATUS(0)
    }
}
impl core::fmt::Debug for INT_SET_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_SET_STATUS")
            .field("FAIL", &self.FAIL())
            .field("ERR", &self.ERR())
            .field("DONE", &self.DONE())
            .field("ECC_ERR", &self.ECC_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INT_SET_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INT_SET_STATUS {{ FAIL: {=bool:?}, ERR: {=bool:?}, DONE: {=bool:?}, ECC_ERR: {=bool:?} }}",
            self.FAIL(),
            self.ERR(),
            self.DONE(),
            self.ECC_ERR()
        )
    }
}
#[doc = "Interrupt status bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INT_STATUS(pub u32);
impl INT_STATUS {
    #[doc = "This status bit is set if execution of a (legal) command failed."]
    #[must_use]
    #[inline(always)]
    pub const fn FAIL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set if execution of a (legal) command failed."]
    #[inline(always)]
    pub const fn set_FAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This status bit is set if execution of an illegal command is detected."]
    #[must_use]
    #[inline(always)]
    pub const fn ERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set if execution of an illegal command is detected."]
    #[inline(always)]
    pub const fn set_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This status bit is set at the end of command execution."]
    #[must_use]
    #[inline(always)]
    pub const fn DONE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set at the end of command execution."]
    #[inline(always)]
    pub const fn set_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This status bit is set if, during a memory read operation (either a user-requested read, or a speculative read, or reads performed by a controller command), a correctable or uncorrectable error is detected by ECC decoding logic."]
    #[must_use]
    #[inline(always)]
    pub const fn ECC_ERR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set if, during a memory read operation (either a user-requested read, or a speculative read, or reads performed by a controller command), a correctable or uncorrectable error is detected by ECC decoding logic."]
    #[inline(always)]
    pub const fn set_ECC_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for INT_STATUS {
    #[inline(always)]
    fn default() -> INT_STATUS {
        INT_STATUS(0)
    }
}
impl core::fmt::Debug for INT_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STATUS")
            .field("FAIL", &self.FAIL())
            .field("ERR", &self.ERR())
            .field("DONE", &self.DONE())
            .field("ECC_ERR", &self.ECC_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INT_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INT_STATUS {{ FAIL: {=bool:?}, ERR: {=bool:?}, DONE: {=bool:?}, ECC_ERR: {=bool:?} }}",
            self.FAIL(),
            self.ERR(),
            self.DONE(),
            self.ECC_ERR()
        )
    }
}
#[doc = "Controller+Memory module identification."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MODULE_ID(pub u32);
impl MODULE_ID {
    #[doc = "Aperture i."]
    #[must_use]
    #[inline(always)]
    pub const fn APERTURE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Aperture i."]
    #[inline(always)]
    pub const fn set_APERTURE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Minor revision i."]
    #[must_use]
    #[inline(always)]
    pub const fn MINOR_REV(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision i."]
    #[inline(always)]
    pub const fn set_MINOR_REV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Major revision i."]
    #[must_use]
    #[inline(always)]
    pub const fn MAJOR_REV(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision i."]
    #[inline(always)]
    pub const fn set_MAJOR_REV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Identifier."]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Identifier."]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MODULE_ID {
    #[inline(always)]
    fn default() -> MODULE_ID {
        MODULE_ID(0)
    }
}
impl core::fmt::Debug for MODULE_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODULE_ID")
            .field("APERTURE", &self.APERTURE())
            .field("MINOR_REV", &self.MINOR_REV())
            .field("MAJOR_REV", &self.MAJOR_REV())
            .field("ID", &self.ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MODULE_ID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MODULE_ID {{ APERTURE: {=u8:?}, MINOR_REV: {=u8:?}, MAJOR_REV: {=u8:?}, ID: {=u16:?} }}",
            self.APERTURE(),
            self.MINOR_REV(),
            self.MAJOR_REV(),
            self.ID()
        )
    }
}
#[doc = "start (or only) address for next flash command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STARTA(pub u32);
impl STARTA {
    #[doc = "Address / Start address for commands that take an address (range) as a parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn STARTA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Address / Start address for commands that take an address (range) as a parameter."]
    #[inline(always)]
    pub const fn set_STARTA(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for STARTA {
    #[inline(always)]
    fn default() -> STARTA {
        STARTA(0)
    }
}
impl core::fmt::Debug for STARTA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STARTA")
            .field("STARTA", &self.STARTA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STARTA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STARTA {{ STARTA: {=u32:?} }}", self.STARTA())
    }
}
#[doc = "end address for next flash command, if command operates on address ranges."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STOPA(pub u32);
impl STOPA {
    #[doc = "Stop address for commands that take an address range as a parameter (the word specified by STOPA is included in the address range)."]
    #[must_use]
    #[inline(always)]
    pub const fn STOPA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Stop address for commands that take an address range as a parameter (the word specified by STOPA is included in the address range)."]
    #[inline(always)]
    pub const fn set_STOPA(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for STOPA {
    #[inline(always)]
    fn default() -> STOPA {
        STOPA(0)
    }
}
impl core::fmt::Debug for STOPA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STOPA")
            .field("STOPA", &self.STOPA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STOPA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STOPA {{ STOPA: {=u32:?} }}", self.STOPA())
    }
}
