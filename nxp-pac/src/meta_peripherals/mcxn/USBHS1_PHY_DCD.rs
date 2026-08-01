#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "USBDCD."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbhs1PhyDcd {
    ptr: *mut u8,
}
unsafe impl Send for Usbhs1PhyDcd {}
unsafe impl Sync for Usbhs1PhyDcd {}
impl Usbhs1PhyDcd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn control(self) -> crate::pac::common::Reg<Control, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Clock."]
    #[inline(always)]
    pub const fn clock(self) -> crate::pac::common::Reg<Clock, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn status(self) -> crate::pac::common::Reg<Status, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Signal Override."]
    #[inline(always)]
    pub const fn signal_override(
        self,
    ) -> crate::pac::common::Reg<SignalOverride, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "TIMER0."]
    #[inline(always)]
    pub const fn timer0(self) -> crate::pac::common::Reg<Timer0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "TIMER1."]
    #[inline(always)]
    pub const fn timer1(self) -> crate::pac::common::Reg<Timer1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "TIMER2_BC11."]
    #[inline(always)]
    pub const fn timer2_bc11(self) -> crate::pac::common::Reg<Timer2Bc11, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "TIMER2_BC12."]
    #[inline(always)]
    pub const fn timer2_bc12(self) -> crate::pac::common::Reg<Timer2Bc12, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
#[doc = "Clock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clock(pub u32);
impl Clock {
    #[doc = "Unit of Measurement Encoding for Clock Speed."]
    #[must_use]
    #[inline(always)]
    pub const fn clock_unit(&self) -> ClockUnit {
        let val = (self.0 >> 0usize) & 0x01;
        ClockUnit::from_bits(val as u8)
    }
    #[doc = "Unit of Measurement Encoding for Clock Speed."]
    #[inline(always)]
    pub const fn set_clock_unit(&mut self, val: ClockUnit) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Numerical Value of Clock Speed in Binary."]
    #[must_use]
    #[inline(always)]
    pub const fn clock_speed(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x03ff;
        val as u16
    }
    #[doc = "Numerical Value of Clock Speed in Binary."]
    #[inline(always)]
    pub const fn set_clock_speed(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 2usize)) | (((val as u32) & 0x03ff) << 2usize);
    }
}
impl Default for Clock {
    #[inline(always)]
    fn default() -> Clock {
        Clock(0)
    }
}
impl core::fmt::Debug for Clock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clock")
            .field("clock_unit", &self.clock_unit())
            .field("clock_speed", &self.clock_speed())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clock {{ clock_unit: {:?}, clock_speed: {=u16:?} }}",
            self.clock_unit(),
            self.clock_speed()
        )
    }
}
#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control(pub u32);
impl Control {
    #[doc = "Interrupt Acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn iack(&self) -> Iack {
        let val = (self.0 >> 0usize) & 0x01;
        Iack::from_bits(val as u8)
    }
    #[doc = "Interrupt Acknowledge."]
    #[inline(always)]
    pub const fn set_iack(&mut self, val: Iack) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn if_(&self) -> If {
        let val = (self.0 >> 8usize) & 0x01;
        If::from_bits(val as u8)
    }
    #[doc = "Interrupt Flag."]
    #[inline(always)]
    pub const fn set_if_(&mut self, val: If) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ie(&self) -> Ie {
        let val = (self.0 >> 16usize) & 0x01;
        Ie::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ie(&mut self, val: Ie) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Battery Charging Revision 1.2 Compatibility."]
    #[must_use]
    #[inline(always)]
    pub const fn bc12(&self) -> Bc12 {
        let val = (self.0 >> 17usize) & 0x01;
        Bc12::from_bits(val as u8)
    }
    #[doc = "Battery Charging Revision 1.2 Compatibility."]
    #[inline(always)]
    pub const fn set_bc12(&mut self, val: Bc12) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Start Change Detection Sequence."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Start Change Detection Sequence."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn sr(&self) -> Sr {
        let val = (self.0 >> 25usize) & 0x01;
        Sr::from_bits(val as u8)
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: Sr) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
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
            .field("iack", &self.iack())
            .field("if_", &self.if_())
            .field("ie", &self.ie())
            .field("bc12", &self.bc12())
            .field("start", &self.start())
            .field("sr", &self.sr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Control {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Control {{ iack: {:?}, if_: {:?}, ie: {:?}, bc12: {:?}, start: {=bool:?}, sr: {:?} }}",
            self.iack(),
            self.if_(),
            self.ie(),
            self.bc12(),
            self.start(),
            self.sr()
        )
    }
}
#[doc = "Signal Override."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SignalOverride(pub u32);
impl SignalOverride {
    #[doc = "Phase Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Ps {
        let val = (self.0 >> 0usize) & 0x07;
        Ps::from_bits(val as u8)
    }
    #[doc = "Phase Selection."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Ps) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for SignalOverride {
    #[inline(always)]
    fn default() -> SignalOverride {
        SignalOverride(0)
    }
}
impl core::fmt::Debug for SignalOverride {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SignalOverride")
            .field("ps", &self.ps())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SignalOverride {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SignalOverride {{ ps: {:?} }}", self.ps())
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Charger Detection Sequence Results."]
    #[must_use]
    #[inline(always)]
    pub const fn seq_res(&self) -> SeqRes {
        let val = (self.0 >> 16usize) & 0x03;
        SeqRes::from_bits(val as u8)
    }
    #[doc = "Charger Detection Sequence Results."]
    #[inline(always)]
    pub const fn set_seq_res(&mut self, val: SeqRes) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Charger Detection Sequence Status."]
    #[must_use]
    #[inline(always)]
    pub const fn seq_stat(&self) -> SeqStat {
        let val = (self.0 >> 18usize) & 0x03;
        SeqStat::from_bits(val as u8)
    }
    #[doc = "Charger Detection Sequence Status."]
    #[inline(always)]
    pub const fn set_seq_stat(&mut self, val: SeqStat) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Error Flag."]
    #[inline(always)]
    pub const fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Timeout Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn to(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Flag."]
    #[inline(always)]
    pub const fn set_to(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Active Status Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> Active {
        let val = (self.0 >> 22usize) & 0x01;
        Active::from_bits(val as u8)
    }
    #[doc = "Active Status Indicator."]
    #[inline(always)]
    pub const fn set_active(&mut self, val: Active) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
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
            .field("seq_res", &self.seq_res())
            .field("seq_stat", &self.seq_stat())
            .field("err", &self.err())
            .field("to", &self.to())
            .field("active", &self.active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ seq_res: {:?}, seq_stat: {:?}, err: {=bool:?}, to: {=bool:?}, active: {:?} }}",
            self.seq_res(),
            self.seq_stat(),
            self.err(),
            self.to(),
            self.active()
        )
    }
}
#[doc = "TIMER0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0(pub u32);
impl Timer0 {
    #[doc = "Unit Connection Timer Elapse (in ms)."]
    #[must_use]
    #[inline(always)]
    pub const fn tunitcon(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Unit Connection Timer Elapse (in ms)."]
    #[inline(always)]
    pub const fn set_tunitcon(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Sequence Initiation Time."]
    #[must_use]
    #[inline(always)]
    pub const fn tseq_init(&self) -> TseqInit {
        let val = (self.0 >> 16usize) & 0x03ff;
        TseqInit::from_bits(val as u16)
    }
    #[doc = "Sequence Initiation Time."]
    #[inline(always)]
    pub const fn set_tseq_init(&mut self, val: TseqInit) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val.to_bits() as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Timer0 {
    #[inline(always)]
    fn default() -> Timer0 {
        Timer0(0)
    }
}
impl core::fmt::Debug for Timer0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer0")
            .field("tunitcon", &self.tunitcon())
            .field("tseq_init", &self.tseq_init())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timer0 {{ tunitcon: {=u16:?}, tseq_init: {:?} }}",
            self.tunitcon(),
            self.tseq_init()
        )
    }
}
#[doc = "TIMER1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1(pub u32);
impl Timer1 {
    #[doc = "Time Period Comparator Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn tvdpsrc_on(&self) -> TvdpsrcOn {
        let val = (self.0 >> 0usize) & 0x03ff;
        TvdpsrcOn::from_bits(val as u16)
    }
    #[doc = "Time Period Comparator Enabled."]
    #[inline(always)]
    pub const fn set_tvdpsrc_on(&mut self, val: TvdpsrcOn) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val.to_bits() as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Time Period to Debounce D+ Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn tdcd_dbnc(&self) -> TdcdDbnc {
        let val = (self.0 >> 16usize) & 0x03ff;
        TdcdDbnc::from_bits(val as u16)
    }
    #[doc = "Time Period to Debounce D+ Signal."]
    #[inline(always)]
    pub const fn set_tdcd_dbnc(&mut self, val: TdcdDbnc) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val.to_bits() as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Timer1 {
    #[inline(always)]
    fn default() -> Timer1 {
        Timer1(0)
    }
}
impl core::fmt::Debug for Timer1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer1")
            .field("tvdpsrc_on", &self.tvdpsrc_on())
            .field("tdcd_dbnc", &self.tdcd_dbnc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timer1 {{ tvdpsrc_on: {:?}, tdcd_dbnc: {:?} }}",
            self.tvdpsrc_on(),
            self.tdcd_dbnc()
        )
    }
}
#[doc = "TIMER2_BC11."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2Bc11(pub u32);
impl Timer2Bc11 {
    #[doc = "Time Before Check of D- Line."]
    #[must_use]
    #[inline(always)]
    pub const fn check_dm(&self) -> CheckDm {
        let val = (self.0 >> 0usize) & 0x0f;
        CheckDm::from_bits(val as u8)
    }
    #[doc = "Time Before Check of D- Line."]
    #[inline(always)]
    pub const fn set_check_dm(&mut self, val: CheckDm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Time Period Before Enabling D+ Pullup."]
    #[must_use]
    #[inline(always)]
    pub const fn tvdpsrc_con(&self) -> TvdpsrcCon {
        let val = (self.0 >> 16usize) & 0x03ff;
        TvdpsrcCon::from_bits(val as u16)
    }
    #[doc = "Time Period Before Enabling D+ Pullup."]
    #[inline(always)]
    pub const fn set_tvdpsrc_con(&mut self, val: TvdpsrcCon) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val.to_bits() as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Timer2Bc11 {
    #[inline(always)]
    fn default() -> Timer2Bc11 {
        Timer2Bc11(0)
    }
}
impl core::fmt::Debug for Timer2Bc11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer2Bc11")
            .field("check_dm", &self.check_dm())
            .field("tvdpsrc_con", &self.tvdpsrc_con())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer2Bc11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timer2Bc11 {{ check_dm: {:?}, tvdpsrc_con: {:?} }}",
            self.check_dm(),
            self.tvdpsrc_con()
        )
    }
}
#[doc = "TIMER2_BC12."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2Bc12(pub u32);
impl Timer2Bc12 {
    #[doc = "TVDMSRC_ON."]
    #[must_use]
    #[inline(always)]
    pub const fn tvdmsrc_on(&self) -> TvdmsrcOn {
        let val = (self.0 >> 0usize) & 0x03ff;
        TvdmsrcOn::from_bits(val as u16)
    }
    #[doc = "TVDMSRC_ON."]
    #[inline(always)]
    pub const fn set_tvdmsrc_on(&mut self, val: TvdmsrcOn) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val.to_bits() as u32) & 0x03ff) << 0usize);
    }
    #[doc = "TWAIT_AFTER_PRD."]
    #[must_use]
    #[inline(always)]
    pub const fn twait_after_prd(&self) -> TwaitAfterPrd {
        let val = (self.0 >> 16usize) & 0x03ff;
        TwaitAfterPrd::from_bits(val as u16)
    }
    #[doc = "TWAIT_AFTER_PRD."]
    #[inline(always)]
    pub const fn set_twait_after_prd(&mut self, val: TwaitAfterPrd) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val.to_bits() as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Timer2Bc12 {
    #[inline(always)]
    fn default() -> Timer2Bc12 {
        Timer2Bc12(0)
    }
}
impl core::fmt::Debug for Timer2Bc12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer2Bc12")
            .field("tvdmsrc_on", &self.tvdmsrc_on())
            .field("twait_after_prd", &self.twait_after_prd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer2Bc12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timer2Bc12 {{ tvdmsrc_on: {:?}, twait_after_prd: {:?} }}",
            self.tvdmsrc_on(),
            self.twait_after_prd()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active {
    #[doc = "The sequence is not running."]
    SeqNotRunning = 0x0,
    #[doc = "The sequence is running."]
    SeqRunning = 0x01,
}
impl Active {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active {
    #[inline(always)]
    fn from(val: u8) -> Active {
        Active::from_bits(val)
    }
}
impl From<Active> for u8 {
    #[inline(always)]
    fn from(val: Active) -> u8 {
        Active::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bc12 {
    #[doc = "Compatible with BC1.1."]
    Bc11 = 0x0,
    #[doc = "Compatible with BC1.2 (default)."]
    Bc12 = 0x01,
}
impl Bc12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bc12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bc12 {
    #[inline(always)]
    fn from(val: u8) -> Bc12 {
        Bc12::from_bits(val)
    }
}
impl From<Bc12> for u8 {
    #[inline(always)]
    fn from(val: Bc12) -> u8 {
        Bc12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CheckDm {
    _RESERVED_0 = 0x0,
    #[doc = "1 ms - 15 ms."]
    Ms1 = 0x01,
    #[doc = "1 ms - 15 ms."]
    Ms2 = 0x02,
    #[doc = "1 ms - 15 ms."]
    Ms3 = 0x03,
    #[doc = "1 ms - 15 ms."]
    Ms4 = 0x04,
    #[doc = "1 ms - 15 ms."]
    Ms5 = 0x05,
    #[doc = "1 ms - 15 ms."]
    Ms6 = 0x06,
    #[doc = "1 ms - 15 ms."]
    Ms7 = 0x07,
    #[doc = "1 ms - 15 ms."]
    Ms8 = 0x08,
    #[doc = "1 ms - 15 ms."]
    Ms9 = 0x09,
    #[doc = "1 ms - 15 ms."]
    Ms10 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl CheckDm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CheckDm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CheckDm {
    #[inline(always)]
    fn from(val: u8) -> CheckDm {
        CheckDm::from_bits(val)
    }
}
impl From<CheckDm> for u8 {
    #[inline(always)]
    fn from(val: CheckDm) -> u8 {
        CheckDm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockUnit {
    #[doc = "kHz Speed (between 4 kHz and 1023 kHz)."]
    KhzClk = 0x0,
    #[doc = "MHz Speed (between 1 MHz and 1023 MHz)."]
    MhzClk = 0x01,
}
impl ClockUnit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClockUnit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClockUnit {
    #[inline(always)]
    fn from(val: u8) -> ClockUnit {
        ClockUnit::from_bits(val)
    }
}
impl From<ClockUnit> for u8 {
    #[inline(always)]
    fn from(val: ClockUnit) -> u8 {
        ClockUnit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iack {
    #[doc = "Do not clear the interrupt."]
    IntNoclear = 0x0,
    #[doc = "Clear the IF field (interrupt flag)."]
    IntClear = 0x01,
}
impl Iack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iack {
    #[inline(always)]
    fn from(val: u8) -> Iack {
        Iack::from_bits(val)
    }
}
impl From<Iack> for u8 {
    #[inline(always)]
    fn from(val: Iack) -> u8 {
        Iack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ie {
    #[doc = "Disable interrupts to the system."]
    DisInt = 0x0,
    #[doc = "Enable interrupts to the system."]
    EnInt = 0x01,
}
impl Ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ie {
    #[inline(always)]
    fn from(val: u8) -> Ie {
        Ie::from_bits(val)
    }
}
impl From<Ie> for u8 {
    #[inline(always)]
    fn from(val: Ie) -> u8 {
        Ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum If {
    #[doc = "No interrupt is pending."]
    IntPend = 0x0,
    #[doc = "An interrupt is pending."]
    IntNopend = 0x01,
}
impl If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for If {
    #[inline(always)]
    fn from(val: u8) -> If {
        If::from_bits(val)
    }
}
impl From<If> for u8 {
    #[inline(always)]
    fn from(val: If) -> u8 {
        If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ps {
    #[doc = "No overrides. Field must remain at this value during normal USB data communication to prevent unexpected conditions on USB_DP and USB_DM pins. (Default)."]
    NoOverride = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Enables VDP_SRC voltage source for the USB_DP pin and IDM_SINK current source for the USB_DM pin."]
    PriDetOverride = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Enables VDM_SRC voltage source only."]
    CdpAdvert = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ps {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ps {
    #[inline(always)]
    fn from(val: u8) -> Ps {
        Ps::from_bits(val)
    }
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(val: Ps) -> u8 {
        Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SeqRes {
    #[doc = "No results to report."]
    NoResult = 0x0,
    #[doc = "Attached to an SDP. Must comply with USB 2.0 by drawing only 2.5 mA (max) until connected."]
    ConnSdp = 0x01,
    #[doc = "Attached to a charging port. The exact meaning depends on the STATUS\\[SEQ_STAT\\] field (value 0: Attached to either a CDP or a DCP. The charger type detection has not completed. value 1: Attached to a CDP. The charger type detection has completed.)."]
    ConnCp = 0x02,
    #[doc = "Attached to a DCP."]
    ConnDcp = 0x03,
}
impl SeqRes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SeqRes {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SeqRes {
    #[inline(always)]
    fn from(val: u8) -> SeqRes {
        SeqRes::from_bits(val)
    }
}
impl From<SeqRes> for u8 {
    #[inline(always)]
    fn from(val: SeqRes) -> u8 {
        SeqRes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SeqStat {
    #[doc = "The module is either not enabled, or the module is enabled but the data pins have not yet been detected."]
    NoDataPinConn = 0x0,
    #[doc = "Data pin contact detection is complete."]
    DataPinConn = 0x01,
    #[doc = "Charging port detection is complete."]
    CpDetDone = 0x02,
    #[doc = "Charger type detection is complete."]
    CtDetDone = 0x03,
}
impl SeqStat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SeqStat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SeqStat {
    #[inline(always)]
    fn from(val: u8) -> SeqStat {
        SeqStat::from_bits(val)
    }
}
impl From<SeqStat> for u8 {
    #[inline(always)]
    fn from(val: SeqStat) -> u8 {
        SeqStat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sr {
    #[doc = "Do not perform a software reset."]
    NoReset = 0x0,
    #[doc = "Perform a software reset."]
    SwReset = 0x01,
}
impl Sr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sr {
    #[inline(always)]
    fn from(val: u8) -> Sr {
        Sr::from_bits(val)
    }
}
impl From<Sr> for u8 {
    #[inline(always)]
    fn from(val: Sr) -> u8 {
        Sr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TdcdDbnc(u16);
impl TdcdDbnc {
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1: Self = Self(0x01);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms2: Self = Self(0x02);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms3: Self = Self(0x03);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms4: Self = Self(0x04);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms5: Self = Self(0x05);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms6: Self = Self(0x06);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms7: Self = Self(0x07);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms8: Self = Self(0x08);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms9: Self = Self(0x09);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms10: Self = Self(0x0a);
}
impl TdcdDbnc {
    pub const fn from_bits(val: u16) -> TdcdDbnc {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TdcdDbnc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Ms1"),
            0x02 => f.write_str("Ms2"),
            0x03 => f.write_str("Ms3"),
            0x04 => f.write_str("Ms4"),
            0x05 => f.write_str("Ms5"),
            0x06 => f.write_str("Ms6"),
            0x07 => f.write_str("Ms7"),
            0x08 => f.write_str("Ms8"),
            0x09 => f.write_str("Ms9"),
            0x0a => f.write_str("Ms10"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TdcdDbnc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Ms1"),
            0x02 => defmt::write!(f, "Ms2"),
            0x03 => defmt::write!(f, "Ms3"),
            0x04 => defmt::write!(f, "Ms4"),
            0x05 => defmt::write!(f, "Ms5"),
            0x06 => defmt::write!(f, "Ms6"),
            0x07 => defmt::write!(f, "Ms7"),
            0x08 => defmt::write!(f, "Ms8"),
            0x09 => defmt::write!(f, "Ms9"),
            0x0a => defmt::write!(f, "Ms10"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TdcdDbnc {
    #[inline(always)]
    fn from(val: u16) -> TdcdDbnc {
        TdcdDbnc::from_bits(val)
    }
}
impl From<TdcdDbnc> for u16 {
    #[inline(always)]
    fn from(val: TdcdDbnc) -> u16 {
        TdcdDbnc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TseqInit(u16);
impl TseqInit {
    #[doc = "0 ms - 1023 ms."]
    pub const Ms0: Self = Self(0x0);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms1: Self = Self(0x01);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms2: Self = Self(0x02);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms3: Self = Self(0x03);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms4: Self = Self(0x04);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms5: Self = Self(0x05);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms6: Self = Self(0x06);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms7: Self = Self(0x07);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms8: Self = Self(0x08);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms9: Self = Self(0x09);
}
impl TseqInit {
    pub const fn from_bits(val: u16) -> TseqInit {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TseqInit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Ms0"),
            0x01 => f.write_str("Ms1"),
            0x02 => f.write_str("Ms2"),
            0x03 => f.write_str("Ms3"),
            0x04 => f.write_str("Ms4"),
            0x05 => f.write_str("Ms5"),
            0x06 => f.write_str("Ms6"),
            0x07 => f.write_str("Ms7"),
            0x08 => f.write_str("Ms8"),
            0x09 => f.write_str("Ms9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TseqInit {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Ms0"),
            0x01 => defmt::write!(f, "Ms1"),
            0x02 => defmt::write!(f, "Ms2"),
            0x03 => defmt::write!(f, "Ms3"),
            0x04 => defmt::write!(f, "Ms4"),
            0x05 => defmt::write!(f, "Ms5"),
            0x06 => defmt::write!(f, "Ms6"),
            0x07 => defmt::write!(f, "Ms7"),
            0x08 => defmt::write!(f, "Ms8"),
            0x09 => defmt::write!(f, "Ms9"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TseqInit {
    #[inline(always)]
    fn from(val: u16) -> TseqInit {
        TseqInit::from_bits(val)
    }
}
impl From<TseqInit> for u16 {
    #[inline(always)]
    fn from(val: TseqInit) -> u16 {
        TseqInit::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TvdmsrcOn(u16);
impl TvdmsrcOn {
    #[doc = "0 ms - 40 ms."]
    pub const Ms0: Self = Self(0x0);
    #[doc = "0 ms - 40 ms."]
    pub const Ms1: Self = Self(0x01);
    #[doc = "0 ms - 40 ms."]
    pub const Ms2: Self = Self(0x02);
    #[doc = "0 ms - 40 ms."]
    pub const Ms3: Self = Self(0x03);
    #[doc = "0 ms - 40 ms."]
    pub const Ms4: Self = Self(0x04);
    #[doc = "0 ms - 40 ms."]
    pub const Ms5: Self = Self(0x05);
    #[doc = "0 ms - 40 ms."]
    pub const Ms6: Self = Self(0x06);
    #[doc = "0 ms - 40 ms."]
    pub const Ms7: Self = Self(0x07);
    #[doc = "0 ms - 40 ms."]
    pub const Ms8: Self = Self(0x08);
    #[doc = "0 ms - 40 ms."]
    pub const Ms9: Self = Self(0x09);
}
impl TvdmsrcOn {
    pub const fn from_bits(val: u16) -> TvdmsrcOn {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TvdmsrcOn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Ms0"),
            0x01 => f.write_str("Ms1"),
            0x02 => f.write_str("Ms2"),
            0x03 => f.write_str("Ms3"),
            0x04 => f.write_str("Ms4"),
            0x05 => f.write_str("Ms5"),
            0x06 => f.write_str("Ms6"),
            0x07 => f.write_str("Ms7"),
            0x08 => f.write_str("Ms8"),
            0x09 => f.write_str("Ms9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TvdmsrcOn {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Ms0"),
            0x01 => defmt::write!(f, "Ms1"),
            0x02 => defmt::write!(f, "Ms2"),
            0x03 => defmt::write!(f, "Ms3"),
            0x04 => defmt::write!(f, "Ms4"),
            0x05 => defmt::write!(f, "Ms5"),
            0x06 => defmt::write!(f, "Ms6"),
            0x07 => defmt::write!(f, "Ms7"),
            0x08 => defmt::write!(f, "Ms8"),
            0x09 => defmt::write!(f, "Ms9"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TvdmsrcOn {
    #[inline(always)]
    fn from(val: u16) -> TvdmsrcOn {
        TvdmsrcOn::from_bits(val)
    }
}
impl From<TvdmsrcOn> for u16 {
    #[inline(always)]
    fn from(val: TvdmsrcOn) -> u16 {
        TvdmsrcOn::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TvdpsrcCon(u16);
impl TvdpsrcCon {
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1: Self = Self(0x01);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms2: Self = Self(0x02);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms3: Self = Self(0x03);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms4: Self = Self(0x04);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms5: Self = Self(0x05);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms6: Self = Self(0x06);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms7: Self = Self(0x07);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms8: Self = Self(0x08);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms9: Self = Self(0x09);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms10: Self = Self(0x0a);
}
impl TvdpsrcCon {
    pub const fn from_bits(val: u16) -> TvdpsrcCon {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TvdpsrcCon {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Ms1"),
            0x02 => f.write_str("Ms2"),
            0x03 => f.write_str("Ms3"),
            0x04 => f.write_str("Ms4"),
            0x05 => f.write_str("Ms5"),
            0x06 => f.write_str("Ms6"),
            0x07 => f.write_str("Ms7"),
            0x08 => f.write_str("Ms8"),
            0x09 => f.write_str("Ms9"),
            0x0a => f.write_str("Ms10"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TvdpsrcCon {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Ms1"),
            0x02 => defmt::write!(f, "Ms2"),
            0x03 => defmt::write!(f, "Ms3"),
            0x04 => defmt::write!(f, "Ms4"),
            0x05 => defmt::write!(f, "Ms5"),
            0x06 => defmt::write!(f, "Ms6"),
            0x07 => defmt::write!(f, "Ms7"),
            0x08 => defmt::write!(f, "Ms8"),
            0x09 => defmt::write!(f, "Ms9"),
            0x0a => defmt::write!(f, "Ms10"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TvdpsrcCon {
    #[inline(always)]
    fn from(val: u16) -> TvdpsrcCon {
        TvdpsrcCon::from_bits(val)
    }
}
impl From<TvdpsrcCon> for u16 {
    #[inline(always)]
    fn from(val: TvdpsrcCon) -> u16 {
        TvdpsrcCon::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TvdpsrcOn(u16);
impl TvdpsrcOn {
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1: Self = Self(0x01);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms2: Self = Self(0x02);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms3: Self = Self(0x03);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms4: Self = Self(0x04);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms5: Self = Self(0x05);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms6: Self = Self(0x06);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms7: Self = Self(0x07);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms8: Self = Self(0x08);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms9: Self = Self(0x09);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms10: Self = Self(0x0a);
}
impl TvdpsrcOn {
    pub const fn from_bits(val: u16) -> TvdpsrcOn {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TvdpsrcOn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Ms1"),
            0x02 => f.write_str("Ms2"),
            0x03 => f.write_str("Ms3"),
            0x04 => f.write_str("Ms4"),
            0x05 => f.write_str("Ms5"),
            0x06 => f.write_str("Ms6"),
            0x07 => f.write_str("Ms7"),
            0x08 => f.write_str("Ms8"),
            0x09 => f.write_str("Ms9"),
            0x0a => f.write_str("Ms10"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TvdpsrcOn {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Ms1"),
            0x02 => defmt::write!(f, "Ms2"),
            0x03 => defmt::write!(f, "Ms3"),
            0x04 => defmt::write!(f, "Ms4"),
            0x05 => defmt::write!(f, "Ms5"),
            0x06 => defmt::write!(f, "Ms6"),
            0x07 => defmt::write!(f, "Ms7"),
            0x08 => defmt::write!(f, "Ms8"),
            0x09 => defmt::write!(f, "Ms9"),
            0x0a => defmt::write!(f, "Ms10"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TvdpsrcOn {
    #[inline(always)]
    fn from(val: u16) -> TvdpsrcOn {
        TvdpsrcOn::from_bits(val)
    }
}
impl From<TvdpsrcOn> for u16 {
    #[inline(always)]
    fn from(val: TvdpsrcOn) -> u16 {
        TvdpsrcOn::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TwaitAfterPrd(u16);
impl TwaitAfterPrd {
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1: Self = Self(0x01);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms2: Self = Self(0x02);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms3: Self = Self(0x03);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms4: Self = Self(0x04);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms5: Self = Self(0x05);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms6: Self = Self(0x06);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms7: Self = Self(0x07);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms8: Self = Self(0x08);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms9: Self = Self(0x09);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms10: Self = Self(0x0a);
}
impl TwaitAfterPrd {
    pub const fn from_bits(val: u16) -> TwaitAfterPrd {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TwaitAfterPrd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Ms1"),
            0x02 => f.write_str("Ms2"),
            0x03 => f.write_str("Ms3"),
            0x04 => f.write_str("Ms4"),
            0x05 => f.write_str("Ms5"),
            0x06 => f.write_str("Ms6"),
            0x07 => f.write_str("Ms7"),
            0x08 => f.write_str("Ms8"),
            0x09 => f.write_str("Ms9"),
            0x0a => f.write_str("Ms10"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TwaitAfterPrd {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Ms1"),
            0x02 => defmt::write!(f, "Ms2"),
            0x03 => defmt::write!(f, "Ms3"),
            0x04 => defmt::write!(f, "Ms4"),
            0x05 => defmt::write!(f, "Ms5"),
            0x06 => defmt::write!(f, "Ms6"),
            0x07 => defmt::write!(f, "Ms7"),
            0x08 => defmt::write!(f, "Ms8"),
            0x09 => defmt::write!(f, "Ms9"),
            0x0a => defmt::write!(f, "Ms10"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TwaitAfterPrd {
    #[inline(always)]
    fn from(val: u16) -> TwaitAfterPrd {
        TwaitAfterPrd::from_bits(val)
    }
}
impl From<TwaitAfterPrd> for u16 {
    #[inline(always)]
    fn from(val: TwaitAfterPrd) -> u16 {
        TwaitAfterPrd::to_bits(val)
    }
}
