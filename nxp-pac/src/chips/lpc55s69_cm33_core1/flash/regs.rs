#[doc = "command register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "command register."]
    #[must_use]
    #[inline(always)]
    pub const fn cmd(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "command register."]
    #[inline(always)]
    pub const fn set_cmd(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmd {
    #[inline(always)]
    fn default() -> Cmd {
        Cmd(0)
    }
}
impl core::fmt::Debug for Cmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmd").field("cmd", &self.cmd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmd {{ cmd: {=u32:?} }}", self.cmd())
    }
}
#[doc = "data register, word 0-7; Memory data, or command parameter, or command result."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dataw(pub u32);
impl Dataw {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn dataw(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_dataw(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dataw {
    #[inline(always)]
    fn default() -> Dataw {
        Dataw(0)
    }
}
impl core::fmt::Debug for Dataw {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dataw")
            .field("dataw", &self.dataw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dataw {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dataw {{ dataw: {=u32:?} }}", self.dataw())
    }
}
#[doc = "event register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Event(pub u32);
impl Event {
    #[doc = "When bit is set, the controller and flash are reset."]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When bit is set, the controller and flash are reset."]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When bit is set, the controller wakes up from whatever low power or powerdown mode was active."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When bit is set, the controller wakes up from whatever low power or powerdown mode was active."]
    #[inline(always)]
    pub const fn set_wakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When bit is set, a running program/erase command is aborted."]
    #[must_use]
    #[inline(always)]
    pub const fn abort(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When bit is set, a running program/erase command is aborted."]
    #[inline(always)]
    pub const fn set_abort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Event {
    #[inline(always)]
    fn default() -> Event {
        Event(0)
    }
}
impl core::fmt::Debug for Event {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Event")
            .field("rst", &self.rst())
            .field("wakeup", &self.wakeup())
            .field("abort", &self.abort())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Event {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Event {{ rst: {=bool:?}, wakeup: {=bool:?}, abort: {=bool:?} }}",
            self.rst(),
            self.wakeup(),
            self.abort()
        )
    }
}
#[doc = "Clear interrupt enable bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntClrEnable(pub u32);
impl IntClrEnable {
    #[doc = "When a CLR_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn fail(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When a CLR_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is cleared."]
    #[inline(always)]
    pub const fn set_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When a CLR_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When a CLR_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is cleared."]
    #[inline(always)]
    pub const fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When a CLR_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When a CLR_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is cleared."]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When a CLR_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn ecc_err(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When a CLR_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is cleared."]
    #[inline(always)]
    pub const fn set_ecc_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IntClrEnable {
    #[inline(always)]
    fn default() -> IntClrEnable {
        IntClrEnable(0)
    }
}
impl core::fmt::Debug for IntClrEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntClrEnable")
            .field("fail", &self.fail())
            .field("err", &self.err())
            .field("done", &self.done())
            .field("ecc_err", &self.ecc_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntClrEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntClrEnable {{ fail: {=bool:?}, err: {=bool:?}, done: {=bool:?}, ecc_err: {=bool:?} }}",
            self.fail(),
            self.err(),
            self.done(),
            self.ecc_err()
        )
    }
}
#[doc = "Clear interrupt status bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntClrStatus(pub u32);
impl IntClrStatus {
    #[doc = "When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn fail(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[inline(always)]
    pub const fn set_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[inline(always)]
    pub const fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn ecc_err(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[inline(always)]
    pub const fn set_ecc_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IntClrStatus {
    #[inline(always)]
    fn default() -> IntClrStatus {
        IntClrStatus(0)
    }
}
impl core::fmt::Debug for IntClrStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntClrStatus")
            .field("fail", &self.fail())
            .field("err", &self.err())
            .field("done", &self.done())
            .field("ecc_err", &self.ecc_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntClrStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntClrStatus {{ fail: {=bool:?}, err: {=bool:?}, done: {=bool:?}, ecc_err: {=bool:?} }}",
            self.fail(),
            self.err(),
            self.done(),
            self.ecc_err()
        )
    }
}
#[doc = "Interrupt enable bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntEnable(pub u32);
impl IntEnable {
    #[doc = "If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[must_use]
    #[inline(always)]
    pub const fn fail(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub const fn set_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub const fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[must_use]
    #[inline(always)]
    pub const fn ecc_err(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub const fn set_ecc_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IntEnable {
    #[inline(always)]
    fn default() -> IntEnable {
        IntEnable(0)
    }
}
impl core::fmt::Debug for IntEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntEnable")
            .field("fail", &self.fail())
            .field("err", &self.err())
            .field("done", &self.done())
            .field("ecc_err", &self.ecc_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntEnable {{ fail: {=bool:?}, err: {=bool:?}, done: {=bool:?}, ecc_err: {=bool:?} }}",
            self.fail(),
            self.err(),
            self.done(),
            self.ecc_err()
        )
    }
}
#[doc = "Set interrupt enable bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntSetEnable(pub u32);
impl IntSetEnable {
    #[doc = "When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn fail(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[inline(always)]
    pub const fn set_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[inline(always)]
    pub const fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn ecc_err(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[inline(always)]
    pub const fn set_ecc_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IntSetEnable {
    #[inline(always)]
    fn default() -> IntSetEnable {
        IntSetEnable(0)
    }
}
impl core::fmt::Debug for IntSetEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntSetEnable")
            .field("fail", &self.fail())
            .field("err", &self.err())
            .field("done", &self.done())
            .field("ecc_err", &self.ecc_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntSetEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntSetEnable {{ fail: {=bool:?}, err: {=bool:?}, done: {=bool:?}, ecc_err: {=bool:?} }}",
            self.fail(),
            self.err(),
            self.done(),
            self.ecc_err()
        )
    }
}
#[doc = "Set interrupt status bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntSetStatus(pub u32);
impl IntSetStatus {
    #[doc = "When a SET_STATUS bit is written to 1, the corresponding INT_STATUS bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn fail(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When a SET_STATUS bit is written to 1, the corresponding INT_STATUS bit is set."]
    #[inline(always)]
    pub const fn set_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When a SET_STATUS bit is written to 1, the corresponding INT_STATUS bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When a SET_STATUS bit is written to 1, the corresponding INT_STATUS bit is set."]
    #[inline(always)]
    pub const fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When a SET_STATUS bit is written to 1, the corresponding INT_STATUS bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When a SET_STATUS bit is written to 1, the corresponding INT_STATUS bit is set."]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When a SET_STATUS bit is written to 1, the corresponding INT_STATUS bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn ecc_err(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When a SET_STATUS bit is written to 1, the corresponding INT_STATUS bit is set."]
    #[inline(always)]
    pub const fn set_ecc_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IntSetStatus {
    #[inline(always)]
    fn default() -> IntSetStatus {
        IntSetStatus(0)
    }
}
impl core::fmt::Debug for IntSetStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntSetStatus")
            .field("fail", &self.fail())
            .field("err", &self.err())
            .field("done", &self.done())
            .field("ecc_err", &self.ecc_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntSetStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntSetStatus {{ fail: {=bool:?}, err: {=bool:?}, done: {=bool:?}, ecc_err: {=bool:?} }}",
            self.fail(),
            self.err(),
            self.done(),
            self.ecc_err()
        )
    }
}
#[doc = "Interrupt status bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatus(pub u32);
impl IntStatus {
    #[doc = "This status bit is set if execution of a (legal) command failed."]
    #[must_use]
    #[inline(always)]
    pub const fn fail(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set if execution of a (legal) command failed."]
    #[inline(always)]
    pub const fn set_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This status bit is set if execution of an illegal command is detected."]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set if execution of an illegal command is detected."]
    #[inline(always)]
    pub const fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This status bit is set at the end of command execution."]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set at the end of command execution."]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This status bit is set if, during a memory read operation (either a user-requested read, or a speculative read, or reads performed by a controller command), a correctable or uncorrectable error is detected by ECC decoding logic."]
    #[must_use]
    #[inline(always)]
    pub const fn ecc_err(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set if, during a memory read operation (either a user-requested read, or a speculative read, or reads performed by a controller command), a correctable or uncorrectable error is detected by ECC decoding logic."]
    #[inline(always)]
    pub const fn set_ecc_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IntStatus {
    #[inline(always)]
    fn default() -> IntStatus {
        IntStatus(0)
    }
}
impl core::fmt::Debug for IntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntStatus")
            .field("fail", &self.fail())
            .field("err", &self.err())
            .field("done", &self.done())
            .field("ecc_err", &self.ecc_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntStatus {{ fail: {=bool:?}, err: {=bool:?}, done: {=bool:?}, ecc_err: {=bool:?} }}",
            self.fail(),
            self.err(),
            self.done(),
            self.ecc_err()
        )
    }
}
#[doc = "Controller+Memory module identification"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModuleId(pub u32);
impl ModuleId {
    #[doc = "Aperture i."]
    #[must_use]
    #[inline(always)]
    pub const fn aperture(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Aperture i."]
    #[inline(always)]
    pub const fn set_aperture(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Minor revision i."]
    #[must_use]
    #[inline(always)]
    pub const fn minor_rev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision i."]
    #[inline(always)]
    pub const fn set_minor_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Major revision i."]
    #[must_use]
    #[inline(always)]
    pub const fn major_rev(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision i."]
    #[inline(always)]
    pub const fn set_major_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Identifier."]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Identifier."]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for ModuleId {
    #[inline(always)]
    fn default() -> ModuleId {
        ModuleId(0)
    }
}
impl core::fmt::Debug for ModuleId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ModuleId")
            .field("aperture", &self.aperture())
            .field("minor_rev", &self.minor_rev())
            .field("major_rev", &self.major_rev())
            .field("id", &self.id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ModuleId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ModuleId {{ aperture: {=u8:?}, minor_rev: {=u8:?}, major_rev: {=u8:?}, id: {=u16:?} }}",
            self.aperture(),
            self.minor_rev(),
            self.major_rev(),
            self.id()
        )
    }
}
#[doc = "start (or only) address for next flash command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Starta(pub u32);
impl Starta {
    #[doc = "Address / Start address for commands that take an address (range) as a parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn starta(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Address / Start address for commands that take an address (range) as a parameter."]
    #[inline(always)]
    pub const fn set_starta(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for Starta {
    #[inline(always)]
    fn default() -> Starta {
        Starta(0)
    }
}
impl core::fmt::Debug for Starta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Starta")
            .field("starta", &self.starta())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Starta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Starta {{ starta: {=u32:?} }}", self.starta())
    }
}
#[doc = "end address for next flash command, if command operates on address ranges"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stopa(pub u32);
impl Stopa {
    #[doc = "Stop address for commands that take an address range as a parameter (the word specified by STOPA is included in the address range)."]
    #[must_use]
    #[inline(always)]
    pub const fn stopa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Stop address for commands that take an address range as a parameter (the word specified by STOPA is included in the address range)."]
    #[inline(always)]
    pub const fn set_stopa(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for Stopa {
    #[inline(always)]
    fn default() -> Stopa {
        Stopa(0)
    }
}
impl core::fmt::Debug for Stopa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stopa")
            .field("stopa", &self.stopa())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stopa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Stopa {{ stopa: {=u32:?} }}", self.stopa())
    }
}
