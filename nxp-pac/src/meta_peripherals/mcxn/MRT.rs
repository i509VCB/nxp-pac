#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Array of registers: INTVAL, TIMER, CTRL, STAT."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Channel {
    ptr: *mut u8,
}
unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}
impl Channel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Time Interval Value."]
    #[inline(always)]
    pub const fn intval(self) -> crate::pac::common::Reg<Intval, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Timer."]
    #[inline(always)]
    pub const fn timer(self) -> crate::pac::common::Reg<Timer, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::pac::common::Reg<Ctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn stat(self) -> crate::pac::common::Reg<Stat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "Multi-Rate Timer (MRT)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrt {
    ptr: *mut u8,
}
unsafe impl Send for Mrt {}
unsafe impl Sync for Mrt {}
impl Mrt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Array of registers: INTVAL, TIMER, CTRL, STAT."]
    #[inline(always)]
    pub const fn channel(self, n: usize) -> Channel {
        assert!(n < 4usize);
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0x0usize + n * 16usize) as _) }
    }
    #[doc = "Module Configuration."]
    #[inline(always)]
    pub const fn modcfg(self) -> crate::pac::common::Reg<Modcfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Idle Channel."]
    #[inline(always)]
    pub const fn idle_ch(self) -> crate::pac::common::Reg<IdleCh, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Global Interrupt Flag."]
    #[inline(always)]
    pub const fn irq_flag(self) -> crate::pac::common::Reg<IrqFlag, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
}
#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Interrupt request."]
    #[must_use]
    #[inline(always)]
    pub const fn inten(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt request."]
    #[inline(always)]
    pub const fn set_inten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MRT Operating mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> Mode {
        let val = (self.0 >> 1usize) & 0x03;
        Mode::from_bits(val as u8)
    }
    #[doc = "MRT Operating mode."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: Mode) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
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
            .field("inten", &self.inten())
            .field("mode", &self.mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ inten: {=bool:?}, mode: {:?} }}",
            self.inten(),
            self.mode()
        )
    }
}
#[doc = "Idle Channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IdleCh(pub u32);
impl IdleCh {
    #[doc = "Idle Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn chan(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Idle Channel."]
    #[inline(always)]
    pub const fn set_chan(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for IdleCh {
    #[inline(always)]
    fn default() -> IdleCh {
        IdleCh(0)
    }
}
impl core::fmt::Debug for IdleCh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IdleCh")
            .field("chan", &self.chan())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IdleCh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IdleCh {{ chan: {=u8:?} }}", self.chan())
    }
}
#[doc = "Time Interval Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intval(pub u32);
impl Intval {
    #[doc = "Time Interval Load Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ivalue(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Time Interval Load Value."]
    #[inline(always)]
    pub const fn set_ivalue(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Force Load Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn load(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Force Load Enable."]
    #[inline(always)]
    pub const fn set_load(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Intval {
    #[inline(always)]
    fn default() -> Intval {
        Intval(0)
    }
}
impl core::fmt::Debug for Intval {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intval")
            .field("ivalue", &self.ivalue())
            .field("load", &self.load())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intval {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intval {{ ivalue: {=u32:?}, load: {=bool:?} }}",
            self.ivalue(),
            self.load()
        )
    }
}
#[doc = "Global Interrupt Flag."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrqFlag(pub u32);
impl IrqFlag {
    #[doc = "Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn gflag0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag."]
    #[inline(always)]
    pub const fn set_gflag0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn gflag1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag."]
    #[inline(always)]
    pub const fn set_gflag1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn gflag2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag."]
    #[inline(always)]
    pub const fn set_gflag2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn gflag3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag."]
    #[inline(always)]
    pub const fn set_gflag3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IrqFlag {
    #[inline(always)]
    fn default() -> IrqFlag {
        IrqFlag(0)
    }
}
impl core::fmt::Debug for IrqFlag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IrqFlag")
            .field("gflag0", &self.gflag0())
            .field("gflag1", &self.gflag1())
            .field("gflag2", &self.gflag2())
            .field("gflag3", &self.gflag3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IrqFlag {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IrqFlag {{ gflag0: {=bool:?}, gflag1: {=bool:?}, gflag2: {=bool:?}, gflag3: {=bool:?} }}",
            self.gflag0(),
            self.gflag1(),
            self.gflag2(),
            self.gflag3()
        )
    }
}
#[doc = "Module Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modcfg(pub u32);
impl Modcfg {
    #[doc = "Number of Channels."]
    #[must_use]
    #[inline(always)]
    pub const fn noc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Channels."]
    #[inline(always)]
    pub const fn set_noc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Number of Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn nob(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of Bits."]
    #[inline(always)]
    pub const fn set_nob(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
    #[doc = "MULTITASK."]
    #[must_use]
    #[inline(always)]
    pub const fn multitask(&self) -> Multitask {
        let val = (self.0 >> 31usize) & 0x01;
        Multitask::from_bits(val as u8)
    }
    #[doc = "MULTITASK."]
    #[inline(always)]
    pub const fn set_multitask(&mut self, val: Multitask) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Modcfg {
    #[inline(always)]
    fn default() -> Modcfg {
        Modcfg(0)
    }
}
impl core::fmt::Debug for Modcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Modcfg")
            .field("noc", &self.noc())
            .field("nob", &self.nob())
            .field("multitask", &self.multitask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Modcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Modcfg {{ noc: {=u8:?}, nob: {=u8:?}, multitask: {:?} }}",
            self.noc(),
            self.nob(),
            self.multitask()
        )
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn intflag(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag."]
    #[inline(always)]
    pub const fn set_intflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Timer n State."]
    #[must_use]
    #[inline(always)]
    pub const fn run(&self) -> Run {
        let val = (self.0 >> 1usize) & 0x01;
        Run::from_bits(val as u8)
    }
    #[doc = "Timer n State."]
    #[inline(always)]
    pub const fn set_run(&mut self, val: Run) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel-In-Use flag."]
    #[must_use]
    #[inline(always)]
    pub const fn inuse(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel-In-Use flag."]
    #[inline(always)]
    pub const fn set_inuse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
impl core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat")
            .field("intflag", &self.intflag())
            .field("run", &self.run())
            .field("inuse", &self.inuse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stat {{ intflag: {=bool:?}, run: {:?}, inuse: {=bool:?} }}",
            self.intflag(),
            self.run(),
            self.inuse()
        )
    }
}
#[doc = "Timer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer(pub u32);
impl Timer {
    #[doc = "Current Timer Value."]
    #[must_use]
    #[inline(always)]
    pub const fn value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Current Timer Value."]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Timer {
    #[inline(always)]
    fn default() -> Timer {
        Timer(0)
    }
}
impl core::fmt::Debug for Timer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer")
            .field("value", &self.value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer {{ value: {=u32:?} }}", self.value())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "Repeat Interrupt mode."]
    RepeatInterruptMode = 0x0,
    #[doc = "One-Shot Interrupt mode."]
    OneShotInterruptMode = 0x01,
    #[doc = "One-Shot Stall mode."]
    OneShotStallMode = 0x02,
    _RESERVED_3 = 0x03,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Multitask {
    #[doc = "Hardware status mode."]
    HardwareStatusMode = 0x0,
    #[doc = "Multitask mode."]
    MultiTaskMode = 0x01,
}
impl Multitask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Multitask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Multitask {
    #[inline(always)]
    fn from(val: u8) -> Multitask {
        Multitask::from_bits(val)
    }
}
impl From<Multitask> for u8 {
    #[inline(always)]
    fn from(val: Multitask) -> u8 {
        Multitask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Run {
    #[doc = "Idle state."]
    IdleState = 0x0,
    #[doc = "Running."]
    Running = 0x01,
}
impl Run {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Run {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Run {
    #[inline(always)]
    fn from(val: u8) -> Run {
        Run::from_bits(val)
    }
}
impl From<Run> for u8 {
    #[inline(always)]
    fn from(val: Run) -> u8 {
        Run::to_bits(val)
    }
}
