#[doc = "ADD Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Add(pub u32);
impl Add {
    #[doc = "ADD Write Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ad(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ADD Write Value"]
    #[inline(always)]
    pub const fn set_ad(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Add {
    #[inline(always)]
    fn default() -> Add {
        Add(0)
    }
}
impl core::fmt::Debug for Add {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Add").field("ad", &self.ad()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Add {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Add {{ ad: {=u32:?} }}", self.ad())
    }
}
#[doc = "ADD1 Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Add1(pub u32);
impl Add1 {
    #[doc = "ADD 1"]
    #[must_use]
    #[inline(always)]
    pub const fn ad1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ADD 1"]
    #[inline(always)]
    pub const fn set_ad1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Add1 {
    #[inline(always)]
    fn default() -> Add1 {
        Add1(0)
    }
}
impl core::fmt::Debug for Add1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Add1").field("ad1", &self.ad1()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Add1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Add1 {{ ad1: {=u32:?} }}", self.ad1())
    }
}
#[doc = "ADD16 Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Add16(pub u32);
impl Add16 {
    #[doc = "ADD 16"]
    #[must_use]
    #[inline(always)]
    pub const fn ad16(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ADD 16"]
    #[inline(always)]
    pub const fn set_ad16(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Add16 {
    #[inline(always)]
    fn default() -> Add16 {
        Add16(0)
    }
}
impl core::fmt::Debug for Add16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Add16").field("ad16", &self.ad16()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Add16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Add16 {{ ad16: {=u32:?} }}", self.ad16())
    }
}
#[doc = "ADD256 Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Add256(pub u32);
impl Add256 {
    #[doc = "ADD 256"]
    #[must_use]
    #[inline(always)]
    pub const fn ad256(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ADD 256"]
    #[inline(always)]
    pub const fn set_ad256(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Add256 {
    #[inline(always)]
    fn default() -> Add256 {
        Add256(0)
    }
}
impl core::fmt::Debug for Add256 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Add256")
            .field("ad256", &self.ad256())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Add256 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Add256 {{ ad256: {=u32:?} }}", self.ad256())
    }
}
#[doc = "ASSERT16 Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Assert16(pub u32);
impl Assert16 {
    #[doc = "ASSERT16 Command"]
    #[must_use]
    #[inline(always)]
    pub const fn ast16(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ASSERT16 Command"]
    #[inline(always)]
    pub const fn set_ast16(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Assert16 {
    #[inline(always)]
    fn default() -> Assert16 {
        Assert16(0)
    }
}
impl core::fmt::Debug for Assert16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Assert16")
            .field("ast16", &self.ast16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Assert16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Assert16 {{ ast16: {=u32:?} }}", self.ast16())
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control(pub u32);
impl Control {
    #[doc = "Lock control"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ctrl(&self) -> super::vals::LockCtrl {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::LockCtrl::from_bits(val as u8)
    }
    #[doc = "Lock control"]
    #[inline(always)]
    pub const fn set_lock_ctrl(&mut self, val: super::vals::LockCtrl) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "TIMEOUT fault control"]
    #[must_use]
    #[inline(always)]
    pub const fn timeout_ctrl(&self) -> super::vals::Ctrl {
        let val = (self.0 >> 2usize) & 0x07;
        super::vals::Ctrl::from_bits(val as u8)
    }
    #[doc = "TIMEOUT fault control"]
    #[inline(always)]
    pub const fn set_timeout_ctrl(&mut self, val: super::vals::Ctrl) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
    }
    #[doc = "MISCOMPARE fault control"]
    #[must_use]
    #[inline(always)]
    pub const fn miscompare_ctrl(&self) -> super::vals::Ctrl {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::Ctrl::from_bits(val as u8)
    }
    #[doc = "MISCOMPARE fault control"]
    #[inline(always)]
    pub const fn set_miscompare_ctrl(&mut self, val: super::vals::Ctrl) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
    #[doc = "SEQUENCE fault control"]
    #[must_use]
    #[inline(always)]
    pub const fn sequence_ctrl(&self) -> super::vals::Ctrl {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Ctrl::from_bits(val as u8)
    }
    #[doc = "SEQUENCE fault control"]
    #[inline(always)]
    pub const fn set_sequence_ctrl(&mut self, val: super::vals::Ctrl) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "STATE fault control"]
    #[must_use]
    #[inline(always)]
    pub const fn state_ctrl(&self) -> super::vals::Ctrl {
        let val = (self.0 >> 14usize) & 0x07;
        super::vals::Ctrl::from_bits(val as u8)
    }
    #[doc = "STATE fault control"]
    #[inline(always)]
    pub const fn set_state_ctrl(&mut self, val: super::vals::Ctrl) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val.to_bits() as u32) & 0x07) << 14usize);
    }
    #[doc = "ADDRESS fault control"]
    #[must_use]
    #[inline(always)]
    pub const fn address_ctrl(&self) -> super::vals::Ctrl {
        let val = (self.0 >> 17usize) & 0x07;
        super::vals::Ctrl::from_bits(val as u8)
    }
    #[doc = "ADDRESS fault control"]
    #[inline(always)]
    pub const fn set_address_ctrl(&mut self, val: super::vals::Ctrl) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
    }
    #[doc = "IRQ pause control"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_pause(&self) -> super::vals::IrqPause {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::IrqPause::from_bits(val as u8)
    }
    #[doc = "IRQ pause control"]
    #[inline(always)]
    pub const fn set_irq_pause(&mut self, val: super::vals::IrqPause) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "DEBUG_HALT control"]
    #[must_use]
    #[inline(always)]
    pub const fn debug_halt_ctrl(&self) -> super::vals::DebugHaltCtrl {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::DebugHaltCtrl::from_bits(val as u8)
    }
    #[doc = "DEBUG_HALT control"]
    #[inline(always)]
    pub const fn set_debug_halt_ctrl(&mut self, val: super::vals::DebugHaltCtrl) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Control {
    #[inline(always)]
    fn default() -> Control {
        Control(0)
    }
}
impl core::fmt::Debug for Control {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Control")
            .field("lock_ctrl", &self.lock_ctrl())
            .field("timeout_ctrl", &self.timeout_ctrl())
            .field("miscompare_ctrl", &self.miscompare_ctrl())
            .field("sequence_ctrl", &self.sequence_ctrl())
            .field("state_ctrl", &self.state_ctrl())
            .field("address_ctrl", &self.address_ctrl())
            .field("irq_pause", &self.irq_pause())
            .field("debug_halt_ctrl", &self.debug_halt_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Control {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Control {{ lock_ctrl: {:?}, timeout_ctrl: {:?}, miscompare_ctrl: {:?}, sequence_ctrl: {:?}, state_ctrl: {:?}, address_ctrl: {:?}, irq_pause: {:?}, debug_halt_ctrl: {:?} }}",
            self.lock_ctrl(),
            self.timeout_ctrl(),
            self.miscompare_ctrl(),
            self.sequence_ctrl(),
            self.state_ctrl(),
            self.address_ctrl(),
            self.irq_pause(),
            self.debug_halt_ctrl()
        )
    }
}
#[doc = "Flags Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flags(pub u32);
impl Flags {
    #[doc = "TIMEOUT fault flag"]
    #[must_use]
    #[inline(always)]
    pub const fn to_flag(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIMEOUT fault flag"]
    #[inline(always)]
    pub const fn set_to_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MISCOMPARE fault flag"]
    #[must_use]
    #[inline(always)]
    pub const fn miscom_flag(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "MISCOMPARE fault flag"]
    #[inline(always)]
    pub const fn set_miscom_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SEQUENCE fault flag"]
    #[must_use]
    #[inline(always)]
    pub const fn seq_flag(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SEQUENCE fault flag"]
    #[inline(always)]
    pub const fn set_seq_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CONTROL fault flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt_flag(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CONTROL fault flag"]
    #[inline(always)]
    pub const fn set_cnt_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "STATE fault flag"]
    #[must_use]
    #[inline(always)]
    pub const fn state_flag(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "STATE fault flag"]
    #[inline(always)]
    pub const fn set_state_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ADDRESS fault flag"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_flag(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "ADDRESS fault flag"]
    #[inline(always)]
    pub const fn set_addr_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Power-on reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn por_flag(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Power-on reset flag"]
    #[inline(always)]
    pub const fn set_por_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Flags {
    #[inline(always)]
    fn default() -> Flags {
        Flags(0)
    }
}
impl core::fmt::Debug for Flags {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flags")
            .field("to_flag", &self.to_flag())
            .field("miscom_flag", &self.miscom_flag())
            .field("seq_flag", &self.seq_flag())
            .field("cnt_flag", &self.cnt_flag())
            .field("state_flag", &self.state_flag())
            .field("addr_flag", &self.addr_flag())
            .field("por_flag", &self.por_flag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flags {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flags {{ to_flag: {=bool:?}, miscom_flag: {=bool:?}, seq_flag: {=bool:?}, cnt_flag: {=bool:?}, state_flag: {=bool:?}, addr_flag: {=bool:?}, por_flag: {=bool:?} }}",
            self.to_flag(),
            self.miscom_flag(),
            self.seq_flag(),
            self.cnt_flag(),
            self.state_flag(),
            self.addr_flag(),
            self.por_flag()
        )
    }
}
#[doc = "Instruction Timer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InstructionTimer(pub u32);
impl InstructionTimer {
    #[doc = "Current value of the Instruction Timer"]
    #[must_use]
    #[inline(always)]
    pub const fn instim(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Current value of the Instruction Timer"]
    #[inline(always)]
    pub const fn set_instim(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for InstructionTimer {
    #[inline(always)]
    fn default() -> InstructionTimer {
        InstructionTimer(0)
    }
}
impl core::fmt::Debug for InstructionTimer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("InstructionTimer")
            .field("instim", &self.instim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for InstructionTimer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "InstructionTimer {{ instim: {=u32:?} }}", self.instim())
    }
}
#[doc = "Persistent Data Storage Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Persistent(pub u32);
impl Persistent {
    #[doc = "Persistent Storage"]
    #[must_use]
    #[inline(always)]
    pub const fn persis(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Persistent Storage"]
    #[inline(always)]
    pub const fn set_persis(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Persistent {
    #[inline(always)]
    fn default() -> Persistent {
        Persistent(0)
    }
}
impl core::fmt::Debug for Persistent {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Persistent")
            .field("persis", &self.persis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Persistent {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Persistent {{ persis: {=u32:?} }}", self.persis())
    }
}
#[doc = "Instruction Timer Reload Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reload(pub u32);
impl Reload {
    #[doc = "Instruction Timer reload value"]
    #[must_use]
    #[inline(always)]
    pub const fn rload(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Instruction Timer reload value"]
    #[inline(always)]
    pub const fn set_rload(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Reload {
    #[inline(always)]
    fn default() -> Reload {
        Reload(0)
    }
}
impl core::fmt::Debug for Reload {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reload")
            .field("rload", &self.rload())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reload {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Reload {{ rload: {=u32:?} }}", self.rload())
    }
}
#[doc = "RESTART Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Restart(pub u32);
impl Restart {
    #[doc = "Restart command"]
    #[must_use]
    #[inline(always)]
    pub const fn rstrt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Restart command"]
    #[inline(always)]
    pub const fn set_rstrt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Restart {
    #[inline(always)]
    fn default() -> Restart {
        Restart(0)
    }
}
impl core::fmt::Debug for Restart {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Restart")
            .field("rstrt", &self.rstrt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Restart {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Restart {{ rstrt: {=u32:?} }}", self.rstrt())
    }
}
#[doc = "START Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Start(pub u32);
impl Start {
    #[doc = "Start command"]
    #[must_use]
    #[inline(always)]
    pub const fn strt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Start command"]
    #[inline(always)]
    pub const fn set_strt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Start {
    #[inline(always)]
    fn default() -> Start {
        Start(0)
    }
}
impl core::fmt::Debug for Start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Start").field("strt", &self.strt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Start {{ strt: {=u32:?} }}", self.strt())
    }
}
#[doc = "Status 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Number of TIMEOUT faults (FLAGS\\[TIMEOUT_FLAG\\]) since the last POR"]
    #[must_use]
    #[inline(always)]
    pub const fn numtof(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of TIMEOUT faults (FLAGS\\[TIMEOUT_FLAG\\]) since the last POR"]
    #[inline(always)]
    pub const fn set_numtof(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Number of MISCOMPARE faults (FLAGS\\[MISCOMPARE_FLAG\\]) since the last POR"]
    #[must_use]
    #[inline(always)]
    pub const fn nummiscompf(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Number of MISCOMPARE faults (FLAGS\\[MISCOMPARE_FLAG\\]) since the last POR"]
    #[inline(always)]
    pub const fn set_nummiscompf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Number of SEQUENCE faults (FLAGS\\[SEQUENCE_FLAG\\]) since the last POR"]
    #[must_use]
    #[inline(always)]
    pub const fn numilseqf(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Number of SEQUENCE faults (FLAGS\\[SEQUENCE_FLAG\\]) since the last POR"]
    #[inline(always)]
    pub const fn set_numilseqf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Current State"]
    #[must_use]
    #[inline(always)]
    pub const fn curst(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Current State"]
    #[inline(always)]
    pub const fn set_curst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
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
            .field("numtof", &self.numtof())
            .field("nummiscompf", &self.nummiscompf())
            .field("numilseqf", &self.numilseqf())
            .field("curst", &self.curst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ numtof: {=u8:?}, nummiscompf: {=u8:?}, numilseqf: {=u8:?}, curst: {=u8:?} }}",
            self.numtof(),
            self.nummiscompf(),
            self.numilseqf(),
            self.curst()
        )
    }
}
#[doc = "Status 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status2(pub u32);
impl Status2 {
    #[doc = "Number of CONTROL faults (FLAGS\\[CONTROL_FLAG\\]) since the last POR"]
    #[must_use]
    #[inline(always)]
    pub const fn numcntf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of CONTROL faults (FLAGS\\[CONTROL_FLAG\\]) since the last POR"]
    #[inline(always)]
    pub const fn set_numcntf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Number of STATE faults (FLAGS\\[STATE_FLAG\\]) since the last POR"]
    #[must_use]
    #[inline(always)]
    pub const fn numillstf(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Number of STATE faults (FLAGS\\[STATE_FLAG\\]) since the last POR"]
    #[inline(always)]
    pub const fn set_numillstf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Number of ADDRESS faults (FLAGS\\[ADDR_FLAG\\]) since the last POR"]
    #[must_use]
    #[inline(always)]
    pub const fn numilla(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Number of ADDRESS faults (FLAGS\\[ADDR_FLAG\\]) since the last POR"]
    #[inline(always)]
    pub const fn set_numilla(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Status2 {
    #[inline(always)]
    fn default() -> Status2 {
        Status2(0)
    }
}
impl core::fmt::Debug for Status2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status2")
            .field("numcntf", &self.numcntf())
            .field("numillstf", &self.numillstf())
            .field("numilla", &self.numilla())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status2 {{ numcntf: {=u8:?}, numillstf: {=u8:?}, numilla: {=u8:?} }}",
            self.numcntf(),
            self.numillstf(),
            self.numilla()
        )
    }
}
#[doc = "STOP Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stop(pub u32);
impl Stop {
    #[doc = "Stop command"]
    #[must_use]
    #[inline(always)]
    pub const fn stp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Stop command"]
    #[inline(always)]
    pub const fn set_stp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Stop {
    #[inline(always)]
    fn default() -> Stop {
        Stop(0)
    }
}
impl core::fmt::Debug for Stop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stop").field("stp", &self.stp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Stop {{ stp: {=u32:?} }}", self.stp())
    }
}
#[doc = "SUB Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sub(pub u32);
impl Sub {
    #[doc = "Subtract Write Value"]
    #[must_use]
    #[inline(always)]
    pub const fn sb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Subtract Write Value"]
    #[inline(always)]
    pub const fn set_sb(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sub {
    #[inline(always)]
    fn default() -> Sub {
        Sub(0)
    }
}
impl core::fmt::Debug for Sub {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sub").field("sb", &self.sb()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sub {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sub {{ sb: {=u32:?} }}", self.sb())
    }
}
#[doc = "SUB1 Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sub1(pub u32);
impl Sub1 {
    #[doc = "Subtract 1"]
    #[must_use]
    #[inline(always)]
    pub const fn sb1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Subtract 1"]
    #[inline(always)]
    pub const fn set_sb1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sub1 {
    #[inline(always)]
    fn default() -> Sub1 {
        Sub1(0)
    }
}
impl core::fmt::Debug for Sub1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sub1").field("sb1", &self.sb1()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sub1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sub1 {{ sb1: {=u32:?} }}", self.sb1())
    }
}
#[doc = "SUB16 Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sub16(pub u32);
impl Sub16 {
    #[doc = "Subtract 16"]
    #[must_use]
    #[inline(always)]
    pub const fn sb16(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Subtract 16"]
    #[inline(always)]
    pub const fn set_sb16(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sub16 {
    #[inline(always)]
    fn default() -> Sub16 {
        Sub16(0)
    }
}
impl core::fmt::Debug for Sub16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sub16").field("sb16", &self.sb16()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sub16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sub16 {{ sb16: {=u32:?} }}", self.sb16())
    }
}
#[doc = "SUB256 Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sub256(pub u32);
impl Sub256 {
    #[doc = "Subtract 256"]
    #[must_use]
    #[inline(always)]
    pub const fn sb256(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Subtract 256"]
    #[inline(always)]
    pub const fn set_sb256(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sub256 {
    #[inline(always)]
    fn default() -> Sub256 {
        Sub256(0)
    }
}
impl core::fmt::Debug for Sub256 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sub256")
            .field("sb256", &self.sb256())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sub256 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sub256 {{ sb256: {=u32:?} }}", self.sb256())
    }
}
