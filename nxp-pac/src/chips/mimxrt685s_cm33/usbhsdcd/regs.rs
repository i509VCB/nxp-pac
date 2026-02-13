#[doc = "Clock register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clock(pub u32);
impl Clock {
    #[doc = "Unit of Measurement Encoding for Clock Speed"]
    #[must_use]
    #[inline(always)]
    pub const fn clock_unit(&self) -> super::vals::ClockUnit {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ClockUnit::from_bits(val as u8)
    }
    #[doc = "Unit of Measurement Encoding for Clock Speed"]
    #[inline(always)]
    pub const fn set_clock_unit(&mut self, val: super::vals::ClockUnit) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Numerical Value of Clock Speed in Binary"]
    #[must_use]
    #[inline(always)]
    pub const fn clock_speed(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x03ff;
        val as u16
    }
    #[doc = "Numerical Value of Clock Speed in Binary"]
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
#[doc = "Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control(pub u32);
impl Control {
    #[doc = "Interrupt Acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn iack(&self) -> super::vals::Iack {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Iack::from_bits(val as u8)
    }
    #[doc = "Interrupt Acknowledge"]
    #[inline(always)]
    pub const fn set_iack(&mut self, val: super::vals::Iack) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn if_(&self) -> super::vals::If {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::If::from_bits(val as u8)
    }
    #[doc = "Interrupt Flag"]
    #[inline(always)]
    pub const fn set_if_(&mut self, val: super::vals::If) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ie(&self) -> super::vals::Ie {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ie::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ie(&mut self, val: super::vals::Ie) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "BC12"]
    #[must_use]
    #[inline(always)]
    pub const fn bc12(&self) -> super::vals::Bc12 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Bc12::from_bits(val as u8)
    }
    #[doc = "BC12"]
    #[inline(always)]
    pub const fn set_bc12(&mut self, val: super::vals::Bc12) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Start Change Detection Sequence"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Start Change Detection Sequence"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sr(&self) -> super::vals::Sr {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Sr::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: super::vals::Sr) {
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
#[doc = "Signal Override Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SignalOverride(pub u32);
impl SignalOverride {
    #[doc = "Phase Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Ps {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ps::from_bits(val as u8)
    }
    #[doc = "Phase Selection"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Ps) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
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
#[doc = "Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Charger Detection Sequence Results"]
    #[must_use]
    #[inline(always)]
    pub const fn seq_res(&self) -> super::vals::SeqRes {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SeqRes::from_bits(val as u8)
    }
    #[doc = "Charger Detection Sequence Results"]
    #[inline(always)]
    pub const fn set_seq_res(&mut self, val: super::vals::SeqRes) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Charger Detection Sequence Status"]
    #[must_use]
    #[inline(always)]
    pub const fn seq_stat(&self) -> super::vals::SeqStat {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::SeqStat::from_bits(val as u8)
    }
    #[doc = "Charger Detection Sequence Status"]
    #[inline(always)]
    pub const fn set_seq_stat(&mut self, val: super::vals::SeqStat) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Error Flag"]
    #[inline(always)]
    pub const fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Timeout Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn to(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Flag"]
    #[inline(always)]
    pub const fn set_to(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Active Status Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> super::vals::Active {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Active::from_bits(val as u8)
    }
    #[doc = "Active Status Indicator"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: super::vals::Active) {
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
#[doc = "TIMER0 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0(pub u32);
impl Timer0 {
    #[doc = "Unit Connection Timer Elapse (in ms)"]
    #[must_use]
    #[inline(always)]
    pub const fn tunitcon(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Unit Connection Timer Elapse (in ms)"]
    #[inline(always)]
    pub const fn set_tunitcon(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Sequence Initiation Time"]
    #[must_use]
    #[inline(always)]
    pub const fn tseq_init(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Sequence Initiation Time"]
    #[inline(always)]
    pub const fn set_tseq_init(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
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
            "Timer0 {{ tunitcon: {=u16:?}, tseq_init: {=u16:?} }}",
            self.tunitcon(),
            self.tseq_init()
        )
    }
}
#[doc = "TIMER1 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1(pub u32);
impl Timer1 {
    #[doc = "Time Period Comparator Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn tvdpsrc_on(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Time Period Comparator Enabled"]
    #[inline(always)]
    pub const fn set_tvdpsrc_on(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Time Period to Debounce D+ Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn tdcd_dbnc(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Time Period to Debounce D+ Signal"]
    #[inline(always)]
    pub const fn set_tdcd_dbnc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
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
            "Timer1 {{ tvdpsrc_on: {=u16:?}, tdcd_dbnc: {=u16:?} }}",
            self.tvdpsrc_on(),
            self.tdcd_dbnc()
        )
    }
}
#[doc = "TIMER2_BC11 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2Bc11(pub u32);
impl Timer2Bc11 {
    #[doc = "Time Before Check of D- Line"]
    #[must_use]
    #[inline(always)]
    pub const fn check_dm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Time Before Check of D- Line"]
    #[inline(always)]
    pub const fn set_check_dm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Time Period Before Enabling D+ Pullup"]
    #[must_use]
    #[inline(always)]
    pub const fn tvdpsrc_con(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Time Period Before Enabling D+ Pullup"]
    #[inline(always)]
    pub const fn set_tvdpsrc_con(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
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
            "Timer2Bc11 {{ check_dm: {=u8:?}, tvdpsrc_con: {=u16:?} }}",
            self.check_dm(),
            self.tvdpsrc_con()
        )
    }
}
#[doc = "TIMER2_BC12 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2Bc12(pub u32);
impl Timer2Bc12 {
    #[doc = "TVDMSRC_ON"]
    #[must_use]
    #[inline(always)]
    pub const fn tvdmsrc_on(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "TVDMSRC_ON"]
    #[inline(always)]
    pub const fn set_tvdmsrc_on(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "TWAIT_AFTER_PRD"]
    #[must_use]
    #[inline(always)]
    pub const fn twait_after_prd(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "TWAIT_AFTER_PRD"]
    #[inline(always)]
    pub const fn set_twait_after_prd(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
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
            "Timer2Bc12 {{ tvdmsrc_on: {=u16:?}, twait_after_prd: {=u16:?} }}",
            self.tvdmsrc_on(),
            self.twait_after_prd()
        )
    }
}
