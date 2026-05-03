#[doc = "Write address for issuing the ADD command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADD(pub u32);
impl ADD {
    #[doc = "Address of ADD command."]
    #[must_use]
    #[inline(always)]
    pub const fn AD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of ADD command."]
    #[inline(always)]
    pub const fn set_AD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ADD {
    #[inline(always)]
    fn default() -> ADD {
        ADD(0)
    }
}
impl core::fmt::Debug for ADD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADD").field("AD", &self.AD()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADD {{ AD: {=u32:?} }}", self.AD())
    }
}
#[doc = "Write address for issuing the ADD1 command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADD1(pub u32);
impl ADD1 {
    #[doc = "Address of ADD1 command."]
    #[must_use]
    #[inline(always)]
    pub const fn AD1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of ADD1 command."]
    #[inline(always)]
    pub const fn set_AD1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ADD1 {
    #[inline(always)]
    fn default() -> ADD1 {
        ADD1(0)
    }
}
impl core::fmt::Debug for ADD1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADD1").field("AD1", &self.AD1()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADD1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADD1 {{ AD1: {=u32:?} }}", self.AD1())
    }
}
#[doc = "Write address for issuing the ADD16 command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADD16(pub u32);
impl ADD16 {
    #[doc = "Address of ADD16."]
    #[must_use]
    #[inline(always)]
    pub const fn AD16(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of ADD16."]
    #[inline(always)]
    pub const fn set_AD16(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ADD16 {
    #[inline(always)]
    fn default() -> ADD16 {
        ADD16(0)
    }
}
impl core::fmt::Debug for ADD16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADD16").field("AD16", &self.AD16()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADD16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADD16 {{ AD16: {=u32:?} }}", self.AD16())
    }
}
#[doc = "Write address for issuing the ADD16 command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADD256(pub u32);
impl ADD256 {
    #[doc = "Address of ADD256 command."]
    #[must_use]
    #[inline(always)]
    pub const fn AD256(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of ADD256 command."]
    #[inline(always)]
    pub const fn set_AD256(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ADD256 {
    #[inline(always)]
    fn default() -> ADD256 {
        ADD256(0)
    }
}
impl core::fmt::Debug for ADD256 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADD256")
            .field("AD256", &self.AD256())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADD256 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADD256 {{ AD256: {=u32:?} }}", self.AD256())
    }
}
#[doc = "The control fields, which constitute CONTROL, control all controllable attributes of the module, including those of CONTROL itself."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONTROL(pub u32);
impl CONTROL {
    #[doc = "Lock control field."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK_CTRL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Lock control field."]
    #[inline(always)]
    pub const fn set_LOCK_CTRL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "TIMEOUT control."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMEOUT_CTRL(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "TIMEOUT control."]
    #[inline(always)]
    pub const fn set_TIMEOUT_CTRL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "MISCOMPARE control field."]
    #[must_use]
    #[inline(always)]
    pub const fn MISCOMPARE_CTRL(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "MISCOMPARE control field."]
    #[inline(always)]
    pub const fn set_MISCOMPARE_CTRL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "SEQUENCE control field."]
    #[must_use]
    #[inline(always)]
    pub const fn SEQUENCE_CTRL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "SEQUENCE control field."]
    #[inline(always)]
    pub const fn set_SEQUENCE_CTRL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "CONTROL control field."]
    #[must_use]
    #[inline(always)]
    pub const fn CONTROL_CTRL(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[doc = "CONTROL control field."]
    #[inline(always)]
    pub const fn set_CONTROL_CTRL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
    #[doc = "STATE control field."]
    #[must_use]
    #[inline(always)]
    pub const fn STATE_CTRL(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x07;
        val as u8
    }
    #[doc = "STATE control field."]
    #[inline(always)]
    pub const fn set_STATE_CTRL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
    }
    #[doc = "ADDRESS control field."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDRESS_CTRL(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[doc = "ADDRESS control field."]
    #[inline(always)]
    pub const fn set_ADDRESS_CTRL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[doc = "IRQ pause control field."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQ_PAUSE(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "IRQ pause control field."]
    #[inline(always)]
    pub const fn set_IRQ_PAUSE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "DEBUG_HALT control field."]
    #[must_use]
    #[inline(always)]
    pub const fn DEBUG_HALT_CTRL(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "DEBUG_HALT control field."]
    #[inline(always)]
    pub const fn set_DEBUG_HALT_CTRL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for CONTROL {
    #[inline(always)]
    fn default() -> CONTROL {
        CONTROL(0)
    }
}
impl core::fmt::Debug for CONTROL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTROL")
            .field("LOCK_CTRL", &self.LOCK_CTRL())
            .field("TIMEOUT_CTRL", &self.TIMEOUT_CTRL())
            .field("MISCOMPARE_CTRL", &self.MISCOMPARE_CTRL())
            .field("SEQUENCE_CTRL", &self.SEQUENCE_CTRL())
            .field("CONTROL_CTRL", &self.CONTROL_CTRL())
            .field("STATE_CTRL", &self.STATE_CTRL())
            .field("ADDRESS_CTRL", &self.ADDRESS_CTRL())
            .field("IRQ_PAUSE", &self.IRQ_PAUSE())
            .field("DEBUG_HALT_CTRL", &self.DEBUG_HALT_CTRL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONTROL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONTROL {{ LOCK_CTRL: {=u8:?}, TIMEOUT_CTRL: {=u8:?}, MISCOMPARE_CTRL: {=u8:?}, SEQUENCE_CTRL: {=u8:?}, CONTROL_CTRL: {=u8:?}, STATE_CTRL: {=u8:?}, ADDRESS_CTRL: {=u8:?}, IRQ_PAUSE: {=u8:?}, DEBUG_HALT_CTRL: {=u8:?} }}",
            self.LOCK_CTRL(),
            self.TIMEOUT_CTRL(),
            self.MISCOMPARE_CTRL(),
            self.SEQUENCE_CTRL(),
            self.CONTROL_CTRL(),
            self.STATE_CTRL(),
            self.ADDRESS_CTRL(),
            self.IRQ_PAUSE(),
            self.DEBUG_HALT_CTRL()
        )
    }
}
#[doc = "Hardware flags."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLAGS(pub u32);
impl FLAGS {
    #[doc = "Timeout flag."]
    #[must_use]
    #[inline(always)]
    pub const fn TO_FLAG(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout flag."]
    #[inline(always)]
    pub const fn set_TO_FLAG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Miscompare flag."]
    #[must_use]
    #[inline(always)]
    pub const fn MISCOM_FLAG(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Miscompare flag."]
    #[inline(always)]
    pub const fn set_MISCOM_FLAG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Sequence flag."]
    #[must_use]
    #[inline(always)]
    pub const fn SEQ_FLAG(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence flag."]
    #[inline(always)]
    pub const fn set_SEQ_FLAG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control (fault) flag."]
    #[must_use]
    #[inline(always)]
    pub const fn CNT_FLAG(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control (fault) flag."]
    #[inline(always)]
    pub const fn set_CNT_FLAG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "State flag."]
    #[must_use]
    #[inline(always)]
    pub const fn STATE_FLAG(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "State flag."]
    #[inline(always)]
    pub const fn set_STATE_FLAG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Address flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR_FLAG(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Address flag."]
    #[inline(always)]
    pub const fn set_ADDR_FLAG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Power-on reset flag."]
    #[must_use]
    #[inline(always)]
    pub const fn POR_FLAG(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Power-on reset flag."]
    #[inline(always)]
    pub const fn set_POR_FLAG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for FLAGS {
    #[inline(always)]
    fn default() -> FLAGS {
        FLAGS(0)
    }
}
impl core::fmt::Debug for FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLAGS")
            .field("TO_FLAG", &self.TO_FLAG())
            .field("MISCOM_FLAG", &self.MISCOM_FLAG())
            .field("SEQ_FLAG", &self.SEQ_FLAG())
            .field("CNT_FLAG", &self.CNT_FLAG())
            .field("STATE_FLAG", &self.STATE_FLAG())
            .field("ADDR_FLAG", &self.ADDR_FLAG())
            .field("POR_FLAG", &self.POR_FLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLAGS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLAGS {{ TO_FLAG: {=bool:?}, MISCOM_FLAG: {=bool:?}, SEQ_FLAG: {=bool:?}, CNT_FLAG: {=bool:?}, STATE_FLAG: {=bool:?}, ADDR_FLAG: {=bool:?}, POR_FLAG: {=bool:?} }}",
            self.TO_FLAG(),
            self.MISCOM_FLAG(),
            self.SEQ_FLAG(),
            self.CNT_FLAG(),
            self.STATE_FLAG(),
            self.ADDR_FLAG(),
            self.POR_FLAG()
        )
    }
}
#[doc = "The INSTRUCTION TIMER itself."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INSTRUCTION_TIMER(pub u32);
impl INSTRUCTION_TIMER {
    #[doc = "INSTRUCTION TIMER 32-bit value."]
    #[must_use]
    #[inline(always)]
    pub const fn INSTIM(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "INSTRUCTION TIMER 32-bit value."]
    #[inline(always)]
    pub const fn set_INSTIM(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for INSTRUCTION_TIMER {
    #[inline(always)]
    fn default() -> INSTRUCTION_TIMER {
        INSTRUCTION_TIMER(0)
    }
}
impl core::fmt::Debug for INSTRUCTION_TIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INSTRUCTION_TIMER")
            .field("INSTIM", &self.INSTIM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INSTRUCTION_TIMER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INSTRUCTION_TIMER {{ INSTIM: {=u32:?} }}", self.INSTIM())
    }
}
#[doc = "Persistent (Ad. Hoc., quasi-NV) data storage."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PERSISTENT(pub u32);
impl PERSISTENT {
    #[doc = "32 regs free for user SW to enjoy."]
    #[must_use]
    #[inline(always)]
    pub const fn PERSIS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "32 regs free for user SW to enjoy."]
    #[inline(always)]
    pub const fn set_PERSIS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PERSISTENT {
    #[inline(always)]
    fn default() -> PERSISTENT {
        PERSISTENT(0)
    }
}
impl core::fmt::Debug for PERSISTENT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERSISTENT")
            .field("PERSIS", &self.PERSIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PERSISTENT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PERSISTENT {{ PERSIS: {=u32:?} }}", self.PERSIS())
    }
}
#[doc = "Instruction timer reload."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RELOAD(pub u32);
impl RELOAD {
    #[doc = "Inst. Timer reload value."]
    #[must_use]
    #[inline(always)]
    pub const fn RLOAD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Inst. Timer reload value."]
    #[inline(always)]
    pub const fn set_RLOAD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RELOAD {
    #[inline(always)]
    fn default() -> RELOAD {
        RELOAD(0)
    }
}
impl core::fmt::Debug for RELOAD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RELOAD")
            .field("RLOAD", &self.RLOAD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RELOAD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RELOAD {{ RLOAD: {=u32:?} }}", self.RLOAD())
    }
}
#[doc = "Write address for issuing the RESTART command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RESTART(pub u32);
impl RESTART {
    #[doc = "Write address for issuing the RESTART command."]
    #[must_use]
    #[inline(always)]
    pub const fn RSTRT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Write address for issuing the RESTART command."]
    #[inline(always)]
    pub const fn set_RSTRT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RESTART {
    #[inline(always)]
    fn default() -> RESTART {
        RESTART(0)
    }
}
impl core::fmt::Debug for RESTART {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESTART")
            .field("RSTRT", &self.RSTRT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESTART {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RESTART {{ RSTRT: {=u32:?} }}", self.RSTRT())
    }
}
#[doc = "Also known as SEC_CNT."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SECURE_COUNTER(pub u32);
impl SECURE_COUNTER {
    #[doc = "Secure Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn SECCNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Secure Counter."]
    #[inline(always)]
    pub const fn set_SECCNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SECURE_COUNTER {
    #[inline(always)]
    fn default() -> SECURE_COUNTER {
        SECURE_COUNTER(0)
    }
}
impl core::fmt::Debug for SECURE_COUNTER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECURE_COUNTER")
            .field("SECCNT", &self.SECCNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SECURE_COUNTER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SECURE_COUNTER {{ SECCNT: {=u32:?} }}", self.SECCNT())
    }
}
#[doc = "Write address for issuing the START command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct START(pub u32);
impl START {
    #[doc = "Address of start command access."]
    #[must_use]
    #[inline(always)]
    pub const fn STRT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of start command access."]
    #[inline(always)]
    pub const fn set_STRT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for START {
    #[inline(always)]
    fn default() -> START {
        START(0)
    }
}
impl core::fmt::Debug for START {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("START").field("STRT", &self.STRT()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for START {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "START {{ STRT: {=u32:?} }}", self.STRT())
    }
}
#[doc = "Status register (1 of 2)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STATUS(pub u32);
impl STATUS {
    #[doc = "Number of Timeout Faults."]
    #[must_use]
    #[inline(always)]
    pub const fn NUMTOF(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of Timeout Faults."]
    #[inline(always)]
    pub const fn set_NUMTOF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Number of Miscompare Faults."]
    #[must_use]
    #[inline(always)]
    pub const fn NUMMISCOMPF(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Number of Miscompare Faults."]
    #[inline(always)]
    pub const fn set_NUMMISCOMPF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Number of illegal sequence faults."]
    #[must_use]
    #[inline(always)]
    pub const fn NUMILSEQF(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Number of illegal sequence faults."]
    #[inline(always)]
    pub const fn set_NUMILSEQF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Current State."]
    #[must_use]
    #[inline(always)]
    pub const fn CURST(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Current State."]
    #[inline(always)]
    pub const fn set_CURST(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
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
            .field("NUMTOF", &self.NUMTOF())
            .field("NUMMISCOMPF", &self.NUMMISCOMPF())
            .field("NUMILSEQF", &self.NUMILSEQF())
            .field("CURST", &self.CURST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STATUS {{ NUMTOF: {=u8:?}, NUMMISCOMPF: {=u8:?}, NUMILSEQF: {=u8:?}, CURST: {=u8:?} }}",
            self.NUMTOF(),
            self.NUMMISCOMPF(),
            self.NUMILSEQF(),
            self.CURST()
        )
    }
}
#[doc = "STATUS register (2 of 2)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STATUS2(pub u32);
impl STATUS2 {
    #[doc = "Number (of) control faults."]
    #[must_use]
    #[inline(always)]
    pub const fn NUMCNTF(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number (of) control faults."]
    #[inline(always)]
    pub const fn set_NUMCNTF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Number (of) state faults."]
    #[must_use]
    #[inline(always)]
    pub const fn NUMILLSTF(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Number (of) state faults."]
    #[inline(always)]
    pub const fn set_NUMILLSTF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Number of (illegal) address faults."]
    #[must_use]
    #[inline(always)]
    pub const fn NUMILLA(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Number of (illegal) address faults."]
    #[inline(always)]
    pub const fn set_NUMILLA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for STATUS2 {
    #[inline(always)]
    fn default() -> STATUS2 {
        STATUS2(0)
    }
}
impl core::fmt::Debug for STATUS2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS2")
            .field("NUMCNTF", &self.NUMCNTF())
            .field("NUMILLSTF", &self.NUMILLSTF())
            .field("NUMILLA", &self.NUMILLA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STATUS2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STATUS2 {{ NUMCNTF: {=u8:?}, NUMILLSTF: {=u8:?}, NUMILLA: {=u8:?} }}",
            self.NUMCNTF(),
            self.NUMILLSTF(),
            self.NUMILLA()
        )
    }
}
#[doc = "Write address for issuing the STOP command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STOP(pub u32);
impl STOP {
    #[doc = "Address of stop command access."]
    #[must_use]
    #[inline(always)]
    pub const fn STP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of stop command access."]
    #[inline(always)]
    pub const fn set_STP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STOP {
    #[inline(always)]
    fn default() -> STOP {
        STOP(0)
    }
}
impl core::fmt::Debug for STOP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STOP").field("STP", &self.STP()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STOP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STOP {{ STP: {=u32:?} }}", self.STP())
    }
}
#[doc = "Write address for issuing the SUB command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SUB(pub u32);
impl SUB {
    #[doc = "Address of SUB command."]
    #[must_use]
    #[inline(always)]
    pub const fn S0B(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of SUB command."]
    #[inline(always)]
    pub const fn set_S0B(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SUB {
    #[inline(always)]
    fn default() -> SUB {
        SUB(0)
    }
}
impl core::fmt::Debug for SUB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUB").field("S0B", &self.S0B()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SUB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SUB {{ S0B: {=u32:?} }}", self.S0B())
    }
}
#[doc = "Write address for issuing the SUB1 command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SUB1(pub u32);
impl SUB1 {
    #[doc = "Address of SUB1 command."]
    #[must_use]
    #[inline(always)]
    pub const fn S1B(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of SUB1 command."]
    #[inline(always)]
    pub const fn set_S1B(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SUB1 {
    #[inline(always)]
    fn default() -> SUB1 {
        SUB1(0)
    }
}
impl core::fmt::Debug for SUB1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUB1").field("S1B", &self.S1B()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SUB1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SUB1 {{ S1B: {=u32:?} }}", self.S1B())
    }
}
#[doc = "Write address for issuing the SUB16 command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SUB16(pub u32);
impl SUB16 {
    #[doc = "Address of SUB16 command."]
    #[must_use]
    #[inline(always)]
    pub const fn SB16(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of SUB16 command."]
    #[inline(always)]
    pub const fn set_SB16(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SUB16 {
    #[inline(always)]
    fn default() -> SUB16 {
        SUB16(0)
    }
}
impl core::fmt::Debug for SUB16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUB16").field("SB16", &self.SB16()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SUB16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SUB16 {{ SB16: {=u32:?} }}", self.SB16())
    }
}
#[doc = "Write address for issuing the SUB256 command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SUB256(pub u32);
impl SUB256 {
    #[doc = "Address of (you guessed it) SUB256 command."]
    #[must_use]
    #[inline(always)]
    pub const fn SB256(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of (you guessed it) SUB256 command."]
    #[inline(always)]
    pub const fn set_SB256(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SUB256 {
    #[inline(always)]
    fn default() -> SUB256 {
        SUB256(0)
    }
}
impl core::fmt::Debug for SUB256 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUB256")
            .field("SB256", &self.SB256())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SUB256 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SUB256 {{ SB256: {=u32:?} }}", self.SB256())
    }
}
