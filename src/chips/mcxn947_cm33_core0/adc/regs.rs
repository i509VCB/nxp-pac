#[doc = "Calibration General A-Side Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalGar(pub u32);
impl CalGar {
    #[doc = "Calibration General A Side Register Element"]
    #[must_use]
    #[inline(always)]
    pub const fn cal_gar_val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Calibration General A Side Register Element"]
    #[inline(always)]
    pub const fn set_cal_gar_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for CalGar {
    #[inline(always)]
    fn default() -> CalGar {
        CalGar(0)
    }
}
impl core::fmt::Debug for CalGar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CalGar")
            .field("cal_gar_val", &self.cal_gar_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CalGar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CalGar {{ cal_gar_val: {=u16:?} }}", self.cal_gar_val())
    }
}
#[doc = "Calibration General B-Side Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalGbr(pub u32);
impl CalGbr {
    #[doc = "Calibration General B Side Register Element"]
    #[must_use]
    #[inline(always)]
    pub const fn cal_gbr_val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Calibration General B Side Register Element"]
    #[inline(always)]
    pub const fn set_cal_gbr_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for CalGbr {
    #[inline(always)]
    fn default() -> CalGbr {
        CalGbr(0)
    }
}
impl core::fmt::Debug for CalGbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CalGbr")
            .field("cal_gbr_val", &self.cal_gbr_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CalGbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CalGbr {{ cal_gbr_val: {=u16:?} }}", self.cal_gbr_val())
    }
}
#[doc = "Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "ADC Trigger Priority Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tprictrl(&self) -> super::vals::Tprictrl {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Tprictrl::from_bits(val as u8)
    }
    #[doc = "ADC Trigger Priority Control"]
    #[inline(always)]
    pub const fn set_tprictrl(&mut self, val: super::vals::Tprictrl) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Power Configuration Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwrsel(&self) -> super::vals::Pwrsel {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pwrsel::from_bits(val as u8)
    }
    #[doc = "Power Configuration Select"]
    #[inline(always)]
    pub const fn set_pwrsel(&mut self, val: super::vals::Pwrsel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Voltage Reference Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn refsel(&self) -> super::vals::Refsel {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Refsel::from_bits(val as u8)
    }
    #[doc = "Voltage Reference Selection"]
    #[inline(always)]
    pub const fn set_refsel(&mut self, val: super::vals::Refsel) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Trigger Resume Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tres(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Resume Enable"]
    #[inline(always)]
    pub const fn set_tres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Trigger Command Resume"]
    #[must_use]
    #[inline(always)]
    pub const fn tcmdres(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Command Resume"]
    #[inline(always)]
    pub const fn set_tcmdres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "High-Priority Trigger Exception Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn hpt_exdi(&self) -> super::vals::HptExdi {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::HptExdi::from_bits(val as u8)
    }
    #[doc = "High-Priority Trigger Exception Disable"]
    #[inline(always)]
    pub const fn set_hpt_exdi(&mut self, val: super::vals::HptExdi) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Power-up Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn pudly(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Power-up Delay"]
    #[inline(always)]
    pub const fn set_pudly(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "ADC Analog Pre-Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pwren(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "ADC Analog Pre-Enable"]
    #[inline(always)]
    pub const fn set_pwren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
impl core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg")
            .field("tprictrl", &self.tprictrl())
            .field("pwrsel", &self.pwrsel())
            .field("refsel", &self.refsel())
            .field("tres", &self.tres())
            .field("tcmdres", &self.tcmdres())
            .field("hpt_exdi", &self.hpt_exdi())
            .field("pudly", &self.pudly())
            .field("pwren", &self.pwren())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg {{ tprictrl: {:?}, pwrsel: {:?}, refsel: {:?}, tres: {=bool:?}, tcmdres: {=bool:?}, hpt_exdi: {:?}, pudly: {=u8:?}, pwren: {=bool:?} }}",
            self.tprictrl(),
            self.pwrsel(),
            self.refsel(),
            self.tres(),
            self.tcmdres(),
            self.hpt_exdi(),
            self.pudly(),
            self.pwren()
        )
    }
}
#[doc = "Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh1(pub u32);
impl Cmdh1 {
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh1Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh1Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: super::vals::Cmdh1Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[must_use]
    #[inline(always)]
    pub const fn wait_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[inline(always)]
    pub const fn set_wait_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn lwi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn set_lwi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh1Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh1Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: super::vals::Cmdh1Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh1Avgs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Cmdh1Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::Cmdh1Avgs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh1Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh1Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Cmdh1Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh1Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh1Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn set_next(&mut self, val: super::vals::Cmdh1Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh1 {
    #[inline(always)]
    fn default() -> Cmdh1 {
        Cmdh1(0)
    }
}
impl core::fmt::Debug for Cmdh1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdh1")
            .field("cmpen", &self.cmpen())
            .field("wait_trig", &self.wait_trig())
            .field("lwi", &self.lwi())
            .field("sts", &self.sts())
            .field("avgs", &self.avgs())
            .field("loop_", &self.loop_())
            .field("next", &self.next())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdh1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdh1 {{ cmpen: {:?}, wait_trig: {=bool:?}, lwi: {=bool:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
            self.cmpen(),
            self.wait_trig(),
            self.lwi(),
            self.sts(),
            self.avgs(),
            self.loop_(),
            self.next()
        )
    }
}
#[doc = "Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh10(pub u32);
impl Cmdh10 {
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh10Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh10Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: super::vals::Cmdh10Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[must_use]
    #[inline(always)]
    pub const fn wait_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[inline(always)]
    pub const fn set_wait_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn lwi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn set_lwi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh10Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh10Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: super::vals::Cmdh10Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh10Avgs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Cmdh10Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::Cmdh10Avgs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh10Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh10Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Cmdh10Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh10Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh10Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn set_next(&mut self, val: super::vals::Cmdh10Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh10 {
    #[inline(always)]
    fn default() -> Cmdh10 {
        Cmdh10(0)
    }
}
impl core::fmt::Debug for Cmdh10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdh10")
            .field("cmpen", &self.cmpen())
            .field("wait_trig", &self.wait_trig())
            .field("lwi", &self.lwi())
            .field("sts", &self.sts())
            .field("avgs", &self.avgs())
            .field("loop_", &self.loop_())
            .field("next", &self.next())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdh10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdh10 {{ cmpen: {:?}, wait_trig: {=bool:?}, lwi: {=bool:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
            self.cmpen(),
            self.wait_trig(),
            self.lwi(),
            self.sts(),
            self.avgs(),
            self.loop_(),
            self.next()
        )
    }
}
#[doc = "Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh11(pub u32);
impl Cmdh11 {
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh11Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh11Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: super::vals::Cmdh11Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[must_use]
    #[inline(always)]
    pub const fn wait_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[inline(always)]
    pub const fn set_wait_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn lwi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn set_lwi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh11Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh11Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: super::vals::Cmdh11Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh11Avgs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Cmdh11Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::Cmdh11Avgs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh11Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh11Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Cmdh11Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh11Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh11Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn set_next(&mut self, val: super::vals::Cmdh11Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh11 {
    #[inline(always)]
    fn default() -> Cmdh11 {
        Cmdh11(0)
    }
}
impl core::fmt::Debug for Cmdh11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdh11")
            .field("cmpen", &self.cmpen())
            .field("wait_trig", &self.wait_trig())
            .field("lwi", &self.lwi())
            .field("sts", &self.sts())
            .field("avgs", &self.avgs())
            .field("loop_", &self.loop_())
            .field("next", &self.next())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdh11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdh11 {{ cmpen: {:?}, wait_trig: {=bool:?}, lwi: {=bool:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
            self.cmpen(),
            self.wait_trig(),
            self.lwi(),
            self.sts(),
            self.avgs(),
            self.loop_(),
            self.next()
        )
    }
}
#[doc = "Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh12(pub u32);
impl Cmdh12 {
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh12Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh12Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: super::vals::Cmdh12Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[must_use]
    #[inline(always)]
    pub const fn wait_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[inline(always)]
    pub const fn set_wait_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn lwi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn set_lwi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh12Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh12Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: super::vals::Cmdh12Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh12Avgs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Cmdh12Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::Cmdh12Avgs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh12Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh12Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Cmdh12Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh12Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh12Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn set_next(&mut self, val: super::vals::Cmdh12Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh12 {
    #[inline(always)]
    fn default() -> Cmdh12 {
        Cmdh12(0)
    }
}
impl core::fmt::Debug for Cmdh12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdh12")
            .field("cmpen", &self.cmpen())
            .field("wait_trig", &self.wait_trig())
            .field("lwi", &self.lwi())
            .field("sts", &self.sts())
            .field("avgs", &self.avgs())
            .field("loop_", &self.loop_())
            .field("next", &self.next())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdh12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdh12 {{ cmpen: {:?}, wait_trig: {=bool:?}, lwi: {=bool:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
            self.cmpen(),
            self.wait_trig(),
            self.lwi(),
            self.sts(),
            self.avgs(),
            self.loop_(),
            self.next()
        )
    }
}
#[doc = "Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh13(pub u32);
impl Cmdh13 {
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh13Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh13Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: super::vals::Cmdh13Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[must_use]
    #[inline(always)]
    pub const fn wait_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[inline(always)]
    pub const fn set_wait_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn lwi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn set_lwi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh13Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh13Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: super::vals::Cmdh13Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh13Avgs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Cmdh13Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::Cmdh13Avgs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh13Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh13Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Cmdh13Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh13Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh13Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn set_next(&mut self, val: super::vals::Cmdh13Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh13 {
    #[inline(always)]
    fn default() -> Cmdh13 {
        Cmdh13(0)
    }
}
impl core::fmt::Debug for Cmdh13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdh13")
            .field("cmpen", &self.cmpen())
            .field("wait_trig", &self.wait_trig())
            .field("lwi", &self.lwi())
            .field("sts", &self.sts())
            .field("avgs", &self.avgs())
            .field("loop_", &self.loop_())
            .field("next", &self.next())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdh13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdh13 {{ cmpen: {:?}, wait_trig: {=bool:?}, lwi: {=bool:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
            self.cmpen(),
            self.wait_trig(),
            self.lwi(),
            self.sts(),
            self.avgs(),
            self.loop_(),
            self.next()
        )
    }
}
#[doc = "Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh14(pub u32);
impl Cmdh14 {
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh14Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh14Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: super::vals::Cmdh14Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[must_use]
    #[inline(always)]
    pub const fn wait_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[inline(always)]
    pub const fn set_wait_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn lwi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn set_lwi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh14Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh14Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: super::vals::Cmdh14Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh14Avgs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Cmdh14Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::Cmdh14Avgs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh14Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh14Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Cmdh14Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh14Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh14Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn set_next(&mut self, val: super::vals::Cmdh14Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh14 {
    #[inline(always)]
    fn default() -> Cmdh14 {
        Cmdh14(0)
    }
}
impl core::fmt::Debug for Cmdh14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdh14")
            .field("cmpen", &self.cmpen())
            .field("wait_trig", &self.wait_trig())
            .field("lwi", &self.lwi())
            .field("sts", &self.sts())
            .field("avgs", &self.avgs())
            .field("loop_", &self.loop_())
            .field("next", &self.next())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdh14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdh14 {{ cmpen: {:?}, wait_trig: {=bool:?}, lwi: {=bool:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
            self.cmpen(),
            self.wait_trig(),
            self.lwi(),
            self.sts(),
            self.avgs(),
            self.loop_(),
            self.next()
        )
    }
}
#[doc = "Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh15(pub u32);
impl Cmdh15 {
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh15Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh15Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: super::vals::Cmdh15Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[must_use]
    #[inline(always)]
    pub const fn wait_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[inline(always)]
    pub const fn set_wait_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn lwi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn set_lwi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh15Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh15Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: super::vals::Cmdh15Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh15Avgs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Cmdh15Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::Cmdh15Avgs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh15Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh15Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Cmdh15Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh15Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh15Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn set_next(&mut self, val: super::vals::Cmdh15Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh15 {
    #[inline(always)]
    fn default() -> Cmdh15 {
        Cmdh15(0)
    }
}
impl core::fmt::Debug for Cmdh15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdh15")
            .field("cmpen", &self.cmpen())
            .field("wait_trig", &self.wait_trig())
            .field("lwi", &self.lwi())
            .field("sts", &self.sts())
            .field("avgs", &self.avgs())
            .field("loop_", &self.loop_())
            .field("next", &self.next())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdh15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdh15 {{ cmpen: {:?}, wait_trig: {=bool:?}, lwi: {=bool:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
            self.cmpen(),
            self.wait_trig(),
            self.lwi(),
            self.sts(),
            self.avgs(),
            self.loop_(),
            self.next()
        )
    }
}
#[doc = "Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh2(pub u32);
impl Cmdh2 {
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh2Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh2Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: super::vals::Cmdh2Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[must_use]
    #[inline(always)]
    pub const fn wait_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[inline(always)]
    pub const fn set_wait_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn lwi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn set_lwi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh2Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh2Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: super::vals::Cmdh2Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh2Avgs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Cmdh2Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::Cmdh2Avgs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh2Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh2Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Cmdh2Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh2Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh2Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn set_next(&mut self, val: super::vals::Cmdh2Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh2 {
    #[inline(always)]
    fn default() -> Cmdh2 {
        Cmdh2(0)
    }
}
impl core::fmt::Debug for Cmdh2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdh2")
            .field("cmpen", &self.cmpen())
            .field("wait_trig", &self.wait_trig())
            .field("lwi", &self.lwi())
            .field("sts", &self.sts())
            .field("avgs", &self.avgs())
            .field("loop_", &self.loop_())
            .field("next", &self.next())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdh2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdh2 {{ cmpen: {:?}, wait_trig: {=bool:?}, lwi: {=bool:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
            self.cmpen(),
            self.wait_trig(),
            self.lwi(),
            self.sts(),
            self.avgs(),
            self.loop_(),
            self.next()
        )
    }
}
#[doc = "Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh3(pub u32);
impl Cmdh3 {
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh3Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh3Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: super::vals::Cmdh3Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[must_use]
    #[inline(always)]
    pub const fn wait_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[inline(always)]
    pub const fn set_wait_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn lwi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn set_lwi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh3Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh3Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: super::vals::Cmdh3Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh3Avgs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Cmdh3Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::Cmdh3Avgs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh3Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh3Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Cmdh3Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh3Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh3Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn set_next(&mut self, val: super::vals::Cmdh3Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh3 {
    #[inline(always)]
    fn default() -> Cmdh3 {
        Cmdh3(0)
    }
}
impl core::fmt::Debug for Cmdh3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdh3")
            .field("cmpen", &self.cmpen())
            .field("wait_trig", &self.wait_trig())
            .field("lwi", &self.lwi())
            .field("sts", &self.sts())
            .field("avgs", &self.avgs())
            .field("loop_", &self.loop_())
            .field("next", &self.next())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdh3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdh3 {{ cmpen: {:?}, wait_trig: {=bool:?}, lwi: {=bool:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
            self.cmpen(),
            self.wait_trig(),
            self.lwi(),
            self.sts(),
            self.avgs(),
            self.loop_(),
            self.next()
        )
    }
}
#[doc = "Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh4(pub u32);
impl Cmdh4 {
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh4Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh4Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: super::vals::Cmdh4Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[must_use]
    #[inline(always)]
    pub const fn wait_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[inline(always)]
    pub const fn set_wait_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn lwi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn set_lwi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh4Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh4Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: super::vals::Cmdh4Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh4Avgs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Cmdh4Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::Cmdh4Avgs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh4Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh4Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Cmdh4Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh4Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh4Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn set_next(&mut self, val: super::vals::Cmdh4Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh4 {
    #[inline(always)]
    fn default() -> Cmdh4 {
        Cmdh4(0)
    }
}
impl core::fmt::Debug for Cmdh4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdh4")
            .field("cmpen", &self.cmpen())
            .field("wait_trig", &self.wait_trig())
            .field("lwi", &self.lwi())
            .field("sts", &self.sts())
            .field("avgs", &self.avgs())
            .field("loop_", &self.loop_())
            .field("next", &self.next())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdh4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdh4 {{ cmpen: {:?}, wait_trig: {=bool:?}, lwi: {=bool:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
            self.cmpen(),
            self.wait_trig(),
            self.lwi(),
            self.sts(),
            self.avgs(),
            self.loop_(),
            self.next()
        )
    }
}
#[doc = "Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh5(pub u32);
impl Cmdh5 {
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh5Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh5Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: super::vals::Cmdh5Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[must_use]
    #[inline(always)]
    pub const fn wait_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[inline(always)]
    pub const fn set_wait_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn lwi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn set_lwi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh5Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh5Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: super::vals::Cmdh5Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh5Avgs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Cmdh5Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::Cmdh5Avgs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh5Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh5Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Cmdh5Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh5Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh5Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn set_next(&mut self, val: super::vals::Cmdh5Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh5 {
    #[inline(always)]
    fn default() -> Cmdh5 {
        Cmdh5(0)
    }
}
impl core::fmt::Debug for Cmdh5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdh5")
            .field("cmpen", &self.cmpen())
            .field("wait_trig", &self.wait_trig())
            .field("lwi", &self.lwi())
            .field("sts", &self.sts())
            .field("avgs", &self.avgs())
            .field("loop_", &self.loop_())
            .field("next", &self.next())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdh5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdh5 {{ cmpen: {:?}, wait_trig: {=bool:?}, lwi: {=bool:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
            self.cmpen(),
            self.wait_trig(),
            self.lwi(),
            self.sts(),
            self.avgs(),
            self.loop_(),
            self.next()
        )
    }
}
#[doc = "Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh6(pub u32);
impl Cmdh6 {
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh6Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh6Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: super::vals::Cmdh6Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[must_use]
    #[inline(always)]
    pub const fn wait_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[inline(always)]
    pub const fn set_wait_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn lwi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn set_lwi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh6Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh6Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: super::vals::Cmdh6Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh6Avgs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Cmdh6Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::Cmdh6Avgs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh6Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh6Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Cmdh6Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh6Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh6Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn set_next(&mut self, val: super::vals::Cmdh6Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh6 {
    #[inline(always)]
    fn default() -> Cmdh6 {
        Cmdh6(0)
    }
}
impl core::fmt::Debug for Cmdh6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdh6")
            .field("cmpen", &self.cmpen())
            .field("wait_trig", &self.wait_trig())
            .field("lwi", &self.lwi())
            .field("sts", &self.sts())
            .field("avgs", &self.avgs())
            .field("loop_", &self.loop_())
            .field("next", &self.next())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdh6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdh6 {{ cmpen: {:?}, wait_trig: {=bool:?}, lwi: {=bool:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
            self.cmpen(),
            self.wait_trig(),
            self.lwi(),
            self.sts(),
            self.avgs(),
            self.loop_(),
            self.next()
        )
    }
}
#[doc = "Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh7(pub u32);
impl Cmdh7 {
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh7Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh7Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: super::vals::Cmdh7Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[must_use]
    #[inline(always)]
    pub const fn wait_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[inline(always)]
    pub const fn set_wait_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn lwi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn set_lwi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh7Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh7Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: super::vals::Cmdh7Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh7Avgs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Cmdh7Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::Cmdh7Avgs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh7Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh7Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Cmdh7Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh7Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh7Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn set_next(&mut self, val: super::vals::Cmdh7Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh7 {
    #[inline(always)]
    fn default() -> Cmdh7 {
        Cmdh7(0)
    }
}
impl core::fmt::Debug for Cmdh7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdh7")
            .field("cmpen", &self.cmpen())
            .field("wait_trig", &self.wait_trig())
            .field("lwi", &self.lwi())
            .field("sts", &self.sts())
            .field("avgs", &self.avgs())
            .field("loop_", &self.loop_())
            .field("next", &self.next())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdh7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdh7 {{ cmpen: {:?}, wait_trig: {=bool:?}, lwi: {=bool:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
            self.cmpen(),
            self.wait_trig(),
            self.lwi(),
            self.sts(),
            self.avgs(),
            self.loop_(),
            self.next()
        )
    }
}
#[doc = "Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh8(pub u32);
impl Cmdh8 {
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh8Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh8Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: super::vals::Cmdh8Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[must_use]
    #[inline(always)]
    pub const fn wait_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[inline(always)]
    pub const fn set_wait_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn lwi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn set_lwi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh8Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh8Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: super::vals::Cmdh8Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh8Avgs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Cmdh8Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::Cmdh8Avgs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh8Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh8Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Cmdh8Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh8Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh8Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn set_next(&mut self, val: super::vals::Cmdh8Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh8 {
    #[inline(always)]
    fn default() -> Cmdh8 {
        Cmdh8(0)
    }
}
impl core::fmt::Debug for Cmdh8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdh8")
            .field("cmpen", &self.cmpen())
            .field("wait_trig", &self.wait_trig())
            .field("lwi", &self.lwi())
            .field("sts", &self.sts())
            .field("avgs", &self.avgs())
            .field("loop_", &self.loop_())
            .field("next", &self.next())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdh8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdh8 {{ cmpen: {:?}, wait_trig: {=bool:?}, lwi: {=bool:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
            self.cmpen(),
            self.wait_trig(),
            self.lwi(),
            self.sts(),
            self.avgs(),
            self.loop_(),
            self.next()
        )
    }
}
#[doc = "Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh9(pub u32);
impl Cmdh9 {
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh9Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh9Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: super::vals::Cmdh9Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[must_use]
    #[inline(always)]
    pub const fn wait_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    #[inline(always)]
    pub const fn set_wait_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn lwi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn set_lwi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh9Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh9Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: super::vals::Cmdh9Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh9Avgs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Cmdh9Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::Cmdh9Avgs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh9Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh9Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Cmdh9Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh9Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh9Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn set_next(&mut self, val: super::vals::Cmdh9Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh9 {
    #[inline(always)]
    fn default() -> Cmdh9 {
        Cmdh9(0)
    }
}
impl core::fmt::Debug for Cmdh9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdh9")
            .field("cmpen", &self.cmpen())
            .field("wait_trig", &self.wait_trig())
            .field("lwi", &self.lwi())
            .field("sts", &self.sts())
            .field("avgs", &self.avgs())
            .field("loop_", &self.loop_())
            .field("next", &self.next())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdh9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdh9 {{ cmpen: {:?}, wait_trig: {=bool:?}, lwi: {=bool:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
            self.cmpen(),
            self.wait_trig(),
            self.lwi(),
            self.sts(),
            self.avgs(),
            self.loop_(),
            self.next()
        )
    }
}
#[doc = "Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl1(pub u32);
impl Cmdl1 {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl1Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl1Adch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::Cmdl1Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ctype(&self) -> super::vals::Cmdl1Ctype {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Cmdl1Ctype::from_bits(val as u8)
    }
    #[doc = "Conversion Type"]
    #[inline(always)]
    pub const fn set_ctype(&mut self, val: super::vals::Cmdl1Ctype) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select Resolution of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Cmdl1Mode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdl1Mode::from_bits(val as u8)
    }
    #[doc = "Select Resolution of Conversions"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Cmdl1Mode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn altb_adch(&self) -> super::vals::Cmdl1AltbAdch {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Cmdl1AltbAdch::from_bits(val as u8)
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[inline(always)]
    pub const fn set_altb_adch(&mut self, val: super::vals::Cmdl1AltbAdch) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn altben(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[inline(always)]
    pub const fn set_altben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Cmdl1 {
    #[inline(always)]
    fn default() -> Cmdl1 {
        Cmdl1(0)
    }
}
impl core::fmt::Debug for Cmdl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdl1")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .field("altb_adch", &self.altb_adch())
            .field("altben", &self.altben())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl1 {{ adch: {:?}, ctype: {:?}, mode: {:?}, altb_adch: {:?}, altben: {=bool:?} }}",
            self.adch(),
            self.ctype(),
            self.mode(),
            self.altb_adch(),
            self.altben()
        )
    }
}
#[doc = "Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl10(pub u32);
impl Cmdl10 {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl10Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl10Adch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::Cmdl10Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ctype(&self) -> super::vals::Cmdl10Ctype {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Cmdl10Ctype::from_bits(val as u8)
    }
    #[doc = "Conversion Type"]
    #[inline(always)]
    pub const fn set_ctype(&mut self, val: super::vals::Cmdl10Ctype) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select Resolution of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Cmdl10Mode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdl10Mode::from_bits(val as u8)
    }
    #[doc = "Select Resolution of Conversions"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Cmdl10Mode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn altb_adch(&self) -> super::vals::Cmdl10AltbAdch {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Cmdl10AltbAdch::from_bits(val as u8)
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[inline(always)]
    pub const fn set_altb_adch(&mut self, val: super::vals::Cmdl10AltbAdch) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn altben(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[inline(always)]
    pub const fn set_altben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Cmdl10 {
    #[inline(always)]
    fn default() -> Cmdl10 {
        Cmdl10(0)
    }
}
impl core::fmt::Debug for Cmdl10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdl10")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .field("altb_adch", &self.altb_adch())
            .field("altben", &self.altben())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl10 {{ adch: {:?}, ctype: {:?}, mode: {:?}, altb_adch: {:?}, altben: {=bool:?} }}",
            self.adch(),
            self.ctype(),
            self.mode(),
            self.altb_adch(),
            self.altben()
        )
    }
}
#[doc = "Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl11(pub u32);
impl Cmdl11 {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl11Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl11Adch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::Cmdl11Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ctype(&self) -> super::vals::Cmdl11Ctype {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Cmdl11Ctype::from_bits(val as u8)
    }
    #[doc = "Conversion Type"]
    #[inline(always)]
    pub const fn set_ctype(&mut self, val: super::vals::Cmdl11Ctype) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select Resolution of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Cmdl11Mode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdl11Mode::from_bits(val as u8)
    }
    #[doc = "Select Resolution of Conversions"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Cmdl11Mode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn altb_adch(&self) -> super::vals::Cmdl11AltbAdch {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Cmdl11AltbAdch::from_bits(val as u8)
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[inline(always)]
    pub const fn set_altb_adch(&mut self, val: super::vals::Cmdl11AltbAdch) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn altben(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[inline(always)]
    pub const fn set_altben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Cmdl11 {
    #[inline(always)]
    fn default() -> Cmdl11 {
        Cmdl11(0)
    }
}
impl core::fmt::Debug for Cmdl11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdl11")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .field("altb_adch", &self.altb_adch())
            .field("altben", &self.altben())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl11 {{ adch: {:?}, ctype: {:?}, mode: {:?}, altb_adch: {:?}, altben: {=bool:?} }}",
            self.adch(),
            self.ctype(),
            self.mode(),
            self.altb_adch(),
            self.altben()
        )
    }
}
#[doc = "Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl12(pub u32);
impl Cmdl12 {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl12Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl12Adch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::Cmdl12Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ctype(&self) -> super::vals::Cmdl12Ctype {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Cmdl12Ctype::from_bits(val as u8)
    }
    #[doc = "Conversion Type"]
    #[inline(always)]
    pub const fn set_ctype(&mut self, val: super::vals::Cmdl12Ctype) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select Resolution of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Cmdl12Mode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdl12Mode::from_bits(val as u8)
    }
    #[doc = "Select Resolution of Conversions"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Cmdl12Mode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn altb_adch(&self) -> super::vals::Cmdl12AltbAdch {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Cmdl12AltbAdch::from_bits(val as u8)
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[inline(always)]
    pub const fn set_altb_adch(&mut self, val: super::vals::Cmdl12AltbAdch) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn altben(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[inline(always)]
    pub const fn set_altben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Cmdl12 {
    #[inline(always)]
    fn default() -> Cmdl12 {
        Cmdl12(0)
    }
}
impl core::fmt::Debug for Cmdl12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdl12")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .field("altb_adch", &self.altb_adch())
            .field("altben", &self.altben())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl12 {{ adch: {:?}, ctype: {:?}, mode: {:?}, altb_adch: {:?}, altben: {=bool:?} }}",
            self.adch(),
            self.ctype(),
            self.mode(),
            self.altb_adch(),
            self.altben()
        )
    }
}
#[doc = "Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl13(pub u32);
impl Cmdl13 {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl13Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl13Adch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::Cmdl13Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ctype(&self) -> super::vals::Cmdl13Ctype {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Cmdl13Ctype::from_bits(val as u8)
    }
    #[doc = "Conversion Type"]
    #[inline(always)]
    pub const fn set_ctype(&mut self, val: super::vals::Cmdl13Ctype) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select Resolution of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Cmdl13Mode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdl13Mode::from_bits(val as u8)
    }
    #[doc = "Select Resolution of Conversions"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Cmdl13Mode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn altb_adch(&self) -> super::vals::Cmdl13AltbAdch {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Cmdl13AltbAdch::from_bits(val as u8)
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[inline(always)]
    pub const fn set_altb_adch(&mut self, val: super::vals::Cmdl13AltbAdch) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn altben(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[inline(always)]
    pub const fn set_altben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Cmdl13 {
    #[inline(always)]
    fn default() -> Cmdl13 {
        Cmdl13(0)
    }
}
impl core::fmt::Debug for Cmdl13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdl13")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .field("altb_adch", &self.altb_adch())
            .field("altben", &self.altben())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl13 {{ adch: {:?}, ctype: {:?}, mode: {:?}, altb_adch: {:?}, altben: {=bool:?} }}",
            self.adch(),
            self.ctype(),
            self.mode(),
            self.altb_adch(),
            self.altben()
        )
    }
}
#[doc = "Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl14(pub u32);
impl Cmdl14 {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl14Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl14Adch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::Cmdl14Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ctype(&self) -> super::vals::Cmdl14Ctype {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Cmdl14Ctype::from_bits(val as u8)
    }
    #[doc = "Conversion Type"]
    #[inline(always)]
    pub const fn set_ctype(&mut self, val: super::vals::Cmdl14Ctype) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select Resolution of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Cmdl14Mode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdl14Mode::from_bits(val as u8)
    }
    #[doc = "Select Resolution of Conversions"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Cmdl14Mode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn altb_adch(&self) -> super::vals::Cmdl14AltbAdch {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Cmdl14AltbAdch::from_bits(val as u8)
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[inline(always)]
    pub const fn set_altb_adch(&mut self, val: super::vals::Cmdl14AltbAdch) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn altben(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[inline(always)]
    pub const fn set_altben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Cmdl14 {
    #[inline(always)]
    fn default() -> Cmdl14 {
        Cmdl14(0)
    }
}
impl core::fmt::Debug for Cmdl14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdl14")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .field("altb_adch", &self.altb_adch())
            .field("altben", &self.altben())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl14 {{ adch: {:?}, ctype: {:?}, mode: {:?}, altb_adch: {:?}, altben: {=bool:?} }}",
            self.adch(),
            self.ctype(),
            self.mode(),
            self.altb_adch(),
            self.altben()
        )
    }
}
#[doc = "Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl15(pub u32);
impl Cmdl15 {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl15Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl15Adch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::Cmdl15Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ctype(&self) -> super::vals::Cmdl15Ctype {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Cmdl15Ctype::from_bits(val as u8)
    }
    #[doc = "Conversion Type"]
    #[inline(always)]
    pub const fn set_ctype(&mut self, val: super::vals::Cmdl15Ctype) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select Resolution of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Cmdl15Mode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdl15Mode::from_bits(val as u8)
    }
    #[doc = "Select Resolution of Conversions"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Cmdl15Mode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn altb_adch(&self) -> super::vals::Cmdl15AltbAdch {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Cmdl15AltbAdch::from_bits(val as u8)
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[inline(always)]
    pub const fn set_altb_adch(&mut self, val: super::vals::Cmdl15AltbAdch) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn altben(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[inline(always)]
    pub const fn set_altben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Cmdl15 {
    #[inline(always)]
    fn default() -> Cmdl15 {
        Cmdl15(0)
    }
}
impl core::fmt::Debug for Cmdl15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdl15")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .field("altb_adch", &self.altb_adch())
            .field("altben", &self.altben())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl15 {{ adch: {:?}, ctype: {:?}, mode: {:?}, altb_adch: {:?}, altben: {=bool:?} }}",
            self.adch(),
            self.ctype(),
            self.mode(),
            self.altb_adch(),
            self.altben()
        )
    }
}
#[doc = "Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl2(pub u32);
impl Cmdl2 {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl2Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl2Adch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::Cmdl2Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ctype(&self) -> super::vals::Cmdl2Ctype {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Cmdl2Ctype::from_bits(val as u8)
    }
    #[doc = "Conversion Type"]
    #[inline(always)]
    pub const fn set_ctype(&mut self, val: super::vals::Cmdl2Ctype) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select Resolution of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Cmdl2Mode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdl2Mode::from_bits(val as u8)
    }
    #[doc = "Select Resolution of Conversions"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Cmdl2Mode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn altb_adch(&self) -> super::vals::Cmdl2AltbAdch {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Cmdl2AltbAdch::from_bits(val as u8)
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[inline(always)]
    pub const fn set_altb_adch(&mut self, val: super::vals::Cmdl2AltbAdch) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn altben(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[inline(always)]
    pub const fn set_altben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Cmdl2 {
    #[inline(always)]
    fn default() -> Cmdl2 {
        Cmdl2(0)
    }
}
impl core::fmt::Debug for Cmdl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdl2")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .field("altb_adch", &self.altb_adch())
            .field("altben", &self.altben())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl2 {{ adch: {:?}, ctype: {:?}, mode: {:?}, altb_adch: {:?}, altben: {=bool:?} }}",
            self.adch(),
            self.ctype(),
            self.mode(),
            self.altb_adch(),
            self.altben()
        )
    }
}
#[doc = "Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl3(pub u32);
impl Cmdl3 {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl3Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl3Adch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::Cmdl3Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ctype(&self) -> super::vals::Cmdl3Ctype {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Cmdl3Ctype::from_bits(val as u8)
    }
    #[doc = "Conversion Type"]
    #[inline(always)]
    pub const fn set_ctype(&mut self, val: super::vals::Cmdl3Ctype) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select Resolution of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Cmdl3Mode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdl3Mode::from_bits(val as u8)
    }
    #[doc = "Select Resolution of Conversions"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Cmdl3Mode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn altb_adch(&self) -> super::vals::Cmdl3AltbAdch {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Cmdl3AltbAdch::from_bits(val as u8)
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[inline(always)]
    pub const fn set_altb_adch(&mut self, val: super::vals::Cmdl3AltbAdch) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn altben(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[inline(always)]
    pub const fn set_altben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Cmdl3 {
    #[inline(always)]
    fn default() -> Cmdl3 {
        Cmdl3(0)
    }
}
impl core::fmt::Debug for Cmdl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdl3")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .field("altb_adch", &self.altb_adch())
            .field("altben", &self.altben())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl3 {{ adch: {:?}, ctype: {:?}, mode: {:?}, altb_adch: {:?}, altben: {=bool:?} }}",
            self.adch(),
            self.ctype(),
            self.mode(),
            self.altb_adch(),
            self.altben()
        )
    }
}
#[doc = "Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl4(pub u32);
impl Cmdl4 {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl4Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl4Adch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::Cmdl4Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ctype(&self) -> super::vals::Cmdl4Ctype {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Cmdl4Ctype::from_bits(val as u8)
    }
    #[doc = "Conversion Type"]
    #[inline(always)]
    pub const fn set_ctype(&mut self, val: super::vals::Cmdl4Ctype) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select Resolution of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Cmdl4Mode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdl4Mode::from_bits(val as u8)
    }
    #[doc = "Select Resolution of Conversions"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Cmdl4Mode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn altb_adch(&self) -> super::vals::Cmdl4AltbAdch {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Cmdl4AltbAdch::from_bits(val as u8)
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[inline(always)]
    pub const fn set_altb_adch(&mut self, val: super::vals::Cmdl4AltbAdch) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn altben(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[inline(always)]
    pub const fn set_altben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Cmdl4 {
    #[inline(always)]
    fn default() -> Cmdl4 {
        Cmdl4(0)
    }
}
impl core::fmt::Debug for Cmdl4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdl4")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .field("altb_adch", &self.altb_adch())
            .field("altben", &self.altben())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl4 {{ adch: {:?}, ctype: {:?}, mode: {:?}, altb_adch: {:?}, altben: {=bool:?} }}",
            self.adch(),
            self.ctype(),
            self.mode(),
            self.altb_adch(),
            self.altben()
        )
    }
}
#[doc = "Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl5(pub u32);
impl Cmdl5 {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl5Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl5Adch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::Cmdl5Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ctype(&self) -> super::vals::Cmdl5Ctype {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Cmdl5Ctype::from_bits(val as u8)
    }
    #[doc = "Conversion Type"]
    #[inline(always)]
    pub const fn set_ctype(&mut self, val: super::vals::Cmdl5Ctype) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select Resolution of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Cmdl5Mode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdl5Mode::from_bits(val as u8)
    }
    #[doc = "Select Resolution of Conversions"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Cmdl5Mode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn altb_adch(&self) -> super::vals::Cmdl5AltbAdch {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Cmdl5AltbAdch::from_bits(val as u8)
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[inline(always)]
    pub const fn set_altb_adch(&mut self, val: super::vals::Cmdl5AltbAdch) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn altben(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[inline(always)]
    pub const fn set_altben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Cmdl5 {
    #[inline(always)]
    fn default() -> Cmdl5 {
        Cmdl5(0)
    }
}
impl core::fmt::Debug for Cmdl5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdl5")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .field("altb_adch", &self.altb_adch())
            .field("altben", &self.altben())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl5 {{ adch: {:?}, ctype: {:?}, mode: {:?}, altb_adch: {:?}, altben: {=bool:?} }}",
            self.adch(),
            self.ctype(),
            self.mode(),
            self.altb_adch(),
            self.altben()
        )
    }
}
#[doc = "Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl6(pub u32);
impl Cmdl6 {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl6Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl6Adch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::Cmdl6Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ctype(&self) -> super::vals::Cmdl6Ctype {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Cmdl6Ctype::from_bits(val as u8)
    }
    #[doc = "Conversion Type"]
    #[inline(always)]
    pub const fn set_ctype(&mut self, val: super::vals::Cmdl6Ctype) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select Resolution of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Cmdl6Mode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdl6Mode::from_bits(val as u8)
    }
    #[doc = "Select Resolution of Conversions"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Cmdl6Mode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn altb_adch(&self) -> super::vals::Cmdl6AltbAdch {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Cmdl6AltbAdch::from_bits(val as u8)
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[inline(always)]
    pub const fn set_altb_adch(&mut self, val: super::vals::Cmdl6AltbAdch) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn altben(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[inline(always)]
    pub const fn set_altben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Cmdl6 {
    #[inline(always)]
    fn default() -> Cmdl6 {
        Cmdl6(0)
    }
}
impl core::fmt::Debug for Cmdl6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdl6")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .field("altb_adch", &self.altb_adch())
            .field("altben", &self.altben())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl6 {{ adch: {:?}, ctype: {:?}, mode: {:?}, altb_adch: {:?}, altben: {=bool:?} }}",
            self.adch(),
            self.ctype(),
            self.mode(),
            self.altb_adch(),
            self.altben()
        )
    }
}
#[doc = "Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl7(pub u32);
impl Cmdl7 {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl7Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl7Adch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::Cmdl7Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ctype(&self) -> super::vals::Cmdl7Ctype {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Cmdl7Ctype::from_bits(val as u8)
    }
    #[doc = "Conversion Type"]
    #[inline(always)]
    pub const fn set_ctype(&mut self, val: super::vals::Cmdl7Ctype) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select Resolution of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Cmdl7Mode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdl7Mode::from_bits(val as u8)
    }
    #[doc = "Select Resolution of Conversions"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Cmdl7Mode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn altb_adch(&self) -> super::vals::Cmdl7AltbAdch {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Cmdl7AltbAdch::from_bits(val as u8)
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[inline(always)]
    pub const fn set_altb_adch(&mut self, val: super::vals::Cmdl7AltbAdch) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn altben(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[inline(always)]
    pub const fn set_altben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Cmdl7 {
    #[inline(always)]
    fn default() -> Cmdl7 {
        Cmdl7(0)
    }
}
impl core::fmt::Debug for Cmdl7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdl7")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .field("altb_adch", &self.altb_adch())
            .field("altben", &self.altben())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl7 {{ adch: {:?}, ctype: {:?}, mode: {:?}, altb_adch: {:?}, altben: {=bool:?} }}",
            self.adch(),
            self.ctype(),
            self.mode(),
            self.altb_adch(),
            self.altben()
        )
    }
}
#[doc = "Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl8(pub u32);
impl Cmdl8 {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl8Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl8Adch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::Cmdl8Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ctype(&self) -> super::vals::Cmdl8Ctype {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Cmdl8Ctype::from_bits(val as u8)
    }
    #[doc = "Conversion Type"]
    #[inline(always)]
    pub const fn set_ctype(&mut self, val: super::vals::Cmdl8Ctype) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select Resolution of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Cmdl8Mode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdl8Mode::from_bits(val as u8)
    }
    #[doc = "Select Resolution of Conversions"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Cmdl8Mode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn altb_adch(&self) -> super::vals::Cmdl8AltbAdch {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Cmdl8AltbAdch::from_bits(val as u8)
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[inline(always)]
    pub const fn set_altb_adch(&mut self, val: super::vals::Cmdl8AltbAdch) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn altben(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[inline(always)]
    pub const fn set_altben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Cmdl8 {
    #[inline(always)]
    fn default() -> Cmdl8 {
        Cmdl8(0)
    }
}
impl core::fmt::Debug for Cmdl8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdl8")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .field("altb_adch", &self.altb_adch())
            .field("altben", &self.altben())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl8 {{ adch: {:?}, ctype: {:?}, mode: {:?}, altb_adch: {:?}, altben: {=bool:?} }}",
            self.adch(),
            self.ctype(),
            self.mode(),
            self.altb_adch(),
            self.altben()
        )
    }
}
#[doc = "Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl9(pub u32);
impl Cmdl9 {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl9Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl9Adch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::Cmdl9Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ctype(&self) -> super::vals::Cmdl9Ctype {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Cmdl9Ctype::from_bits(val as u8)
    }
    #[doc = "Conversion Type"]
    #[inline(always)]
    pub const fn set_ctype(&mut self, val: super::vals::Cmdl9Ctype) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select Resolution of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Cmdl9Mode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdl9Mode::from_bits(val as u8)
    }
    #[doc = "Select Resolution of Conversions"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Cmdl9Mode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn altb_adch(&self) -> super::vals::Cmdl9AltbAdch {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Cmdl9AltbAdch::from_bits(val as u8)
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    #[inline(always)]
    pub const fn set_altb_adch(&mut self, val: super::vals::Cmdl9AltbAdch) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn altben(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Channel B Select Enable"]
    #[inline(always)]
    pub const fn set_altben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Cmdl9 {
    #[inline(always)]
    fn default() -> Cmdl9 {
        Cmdl9(0)
    }
}
impl core::fmt::Debug for Cmdl9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdl9")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .field("altb_adch", &self.altb_adch())
            .field("altben", &self.altben())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl9 {{ adch: {:?}, ctype: {:?}, mode: {:?}, altb_adch: {:?}, altben: {=bool:?} }}",
            self.adch(),
            self.ctype(),
            self.mode(),
            self.altb_adch(),
            self.altben()
        )
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "ADC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn adcen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADC Enable"]
    #[inline(always)]
    pub const fn set_adcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Doze Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dozen(&self) -> super::vals::Dozen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dozen::from_bits(val as u8)
    }
    #[doc = "Doze Enable"]
    #[inline(always)]
    pub const fn set_dozen(&mut self, val: super::vals::Dozen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Auto-Calibration Request"]
    #[must_use]
    #[inline(always)]
    pub const fn cal_req(&self) -> super::vals::CalReq {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::CalReq::from_bits(val as u8)
    }
    #[doc = "Auto-Calibration Request"]
    #[inline(always)]
    pub const fn set_cal_req(&mut self, val: super::vals::CalReq) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Offset Calibration Request"]
    #[must_use]
    #[inline(always)]
    pub const fn calofs(&self) -> super::vals::Calofs {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Calofs::from_bits(val as u8)
    }
    #[doc = "Offset Calibration Request"]
    #[inline(always)]
    pub const fn set_calofs(&mut self, val: super::vals::Calofs) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Reset FIFO 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rstfifo0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Reset FIFO 0"]
    #[inline(always)]
    pub const fn set_rstfifo0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Reset FIFO 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rstfifo1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Reset FIFO 1"]
    #[inline(always)]
    pub const fn set_rstfifo1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Auto-Calibration Averages"]
    #[must_use]
    #[inline(always)]
    pub const fn cal_avgs(&self) -> super::vals::CalAvgs {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CalAvgs::from_bits(val as u8)
    }
    #[doc = "Auto-Calibration Averages"]
    #[inline(always)]
    pub const fn set_cal_avgs(&mut self, val: super::vals::CalAvgs) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
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
            .field("adcen", &self.adcen())
            .field("rst", &self.rst())
            .field("dozen", &self.dozen())
            .field("cal_req", &self.cal_req())
            .field("calofs", &self.calofs())
            .field("rstfifo0", &self.rstfifo0())
            .field("rstfifo1", &self.rstfifo1())
            .field("cal_avgs", &self.cal_avgs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ adcen: {=bool:?}, rst: {=bool:?}, dozen: {:?}, cal_req: {:?}, calofs: {:?}, rstfifo0: {=bool:?}, rstfifo1: {=bool:?}, cal_avgs: {:?} }}",
            self.adcen(),
            self.rst(),
            self.dozen(),
            self.cal_req(),
            self.calofs(),
            self.rstfifo0(),
            self.rstfifo1(),
            self.cal_avgs()
        )
    }
}
#[doc = "Compare Value Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cv(pub u32);
impl Cv {
    #[doc = "Compare Value Low"]
    #[must_use]
    #[inline(always)]
    pub const fn cvl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Compare Value Low"]
    #[inline(always)]
    pub const fn set_cvl(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Compare Value High"]
    #[must_use]
    #[inline(always)]
    pub const fn cvh(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Compare Value High"]
    #[inline(always)]
    pub const fn set_cvh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cv {
    #[inline(always)]
    fn default() -> Cv {
        Cv(0)
    }
}
impl core::fmt::Debug for Cv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cv")
            .field("cvl", &self.cvl())
            .field("cvh", &self.cvh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cv {{ cvl: {=u16:?}, cvh: {=u16:?} }}",
            self.cvl(),
            self.cvh()
        )
    }
}
#[doc = "DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct De(pub u32);
impl De {
    #[doc = "FIFO 0 Watermark DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fwmde0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 0 Watermark DMA Enable"]
    #[inline(always)]
    pub const fn set_fwmde0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO1 Watermark DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fwmde1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO1 Watermark DMA Enable"]
    #[inline(always)]
    pub const fn set_fwmde1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for De {
    #[inline(always)]
    fn default() -> De {
        De(0)
    }
}
impl core::fmt::Debug for De {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("De")
            .field("fwmde0", &self.fwmde0())
            .field("fwmde1", &self.fwmde1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for De {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "De {{ fwmde0: {=bool:?}, fwmde1: {=bool:?} }}",
            self.fwmde0(),
            self.fwmde1()
        )
    }
}
#[doc = "FIFO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl(pub u32);
impl Fctrl {
    #[doc = "Result FIFO Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn fcount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Result FIFO Counter"]
    #[inline(always)]
    pub const fn set_fcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Watermark Level Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn fwmark(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Watermark Level Selection"]
    #[inline(always)]
    pub const fn set_fwmark(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Fctrl {
    #[inline(always)]
    fn default() -> Fctrl {
        Fctrl(0)
    }
}
impl core::fmt::Debug for Fctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fctrl")
            .field("fcount", &self.fcount())
            .field("fwmark", &self.fwmark())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fctrl {{ fcount: {=u8:?}, fwmark: {=u8:?} }}",
            self.fcount(),
            self.fwmark()
        )
    }
}
#[doc = "Gain Calibration Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gcc(pub u32);
impl Gcc {
    #[doc = "Gain Calibration Value"]
    #[must_use]
    #[inline(always)]
    pub const fn gain_cal(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Gain Calibration Value"]
    #[inline(always)]
    pub const fn set_gain_cal(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Gain Calibration Value Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn rdy(&self) -> super::vals::GccRdy {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::GccRdy::from_bits(val as u8)
    }
    #[doc = "Gain Calibration Value Valid"]
    #[inline(always)]
    pub const fn set_rdy(&mut self, val: super::vals::GccRdy) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Gcc {
    #[inline(always)]
    fn default() -> Gcc {
        Gcc(0)
    }
}
impl core::fmt::Debug for Gcc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gcc")
            .field("gain_cal", &self.gain_cal())
            .field("rdy", &self.rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gcc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gcc {{ gain_cal: {=u16:?}, rdy: {:?} }}",
            self.gain_cal(),
            self.rdy()
        )
    }
}
#[doc = "Gain Calculation Result"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gcr(pub u32);
impl Gcr {
    #[doc = "Gain Calculation Result"]
    #[must_use]
    #[inline(always)]
    pub const fn gcalr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Gain Calculation Result"]
    #[inline(always)]
    pub const fn set_gcalr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Gain Calculation Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn rdy(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Gain Calculation Ready"]
    #[inline(always)]
    pub const fn set_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Gcr {
    #[inline(always)]
    fn default() -> Gcr {
        Gcr(0)
    }
}
impl core::fmt::Debug for Gcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gcr")
            .field("gcalr", &self.gcalr())
            .field("rdy", &self.rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gcr {{ gcalr: {=u16:?}, rdy: {=bool:?} }}",
            self.gcalr(),
            self.rdy()
        )
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ie(pub u32);
impl Ie {
    #[doc = "FIFO 0 Watermark Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fwmie0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 0 Watermark Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fwmie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Result FIFO 0 Overflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fofie0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Result FIFO 0 Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fofie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO1 Watermark Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fwmie1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO1 Watermark Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fwmie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Result FIFO1 Overflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fofie1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Result FIFO1 Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fofie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Trigger Exception Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn texc_ie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Exception Interrupt Enable"]
    #[inline(always)]
    pub const fn set_texc_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Trigger Completion Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcomp_ie(&self) -> super::vals::TcompIe {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::TcompIe::from_bits(val as u8)
    }
    #[doc = "Trigger Completion Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcomp_ie(&mut self, val: super::vals::TcompIe) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
}
impl Default for Ie {
    #[inline(always)]
    fn default() -> Ie {
        Ie(0)
    }
}
impl core::fmt::Debug for Ie {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ie")
            .field("fwmie0", &self.fwmie0())
            .field("fofie0", &self.fofie0())
            .field("fwmie1", &self.fwmie1())
            .field("fofie1", &self.fofie1())
            .field("texc_ie", &self.texc_ie())
            .field("tcomp_ie", &self.tcomp_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ie {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ie {{ fwmie0: {=bool:?}, fofie0: {=bool:?}, fwmie1: {=bool:?}, fofie1: {=bool:?}, texc_ie: {=bool:?}, tcomp_ie: {:?} }}",
            self.fwmie0(),
            self.fofie0(),
            self.fwmie1(),
            self.fofie1(),
            self.texc_ie(),
            self.tcomp_ie()
        )
    }
}
#[doc = "Offset Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ofstrim(pub u32);
impl Ofstrim {
    #[doc = "Trim for Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn ofstrim_a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim for Offset"]
    #[inline(always)]
    pub const fn set_ofstrim_a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Trim for Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn ofstrim_b(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim for Offset"]
    #[inline(always)]
    pub const fn set_ofstrim_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Ofstrim {
    #[inline(always)]
    fn default() -> Ofstrim {
        Ofstrim(0)
    }
}
impl core::fmt::Debug for Ofstrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ofstrim")
            .field("ofstrim_a", &self.ofstrim_a())
            .field("ofstrim_b", &self.ofstrim_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ofstrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ofstrim {{ ofstrim_a: {=u8:?}, ofstrim_b: {=u8:?} }}",
            self.ofstrim_a(),
            self.ofstrim_b()
        )
    }
}
#[doc = "Parameter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Trigger Number"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_num(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trigger Number"]
    #[inline(always)]
    pub const fn set_trig_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Result FIFO Depth"]
    #[must_use]
    #[inline(always)]
    pub const fn fifosize(&self) -> super::vals::Fifosize {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Fifosize::from_bits(val as u8)
    }
    #[doc = "Result FIFO Depth"]
    #[inline(always)]
    pub const fn set_fifosize(&mut self, val: super::vals::Fifosize) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Compare Value Number"]
    #[must_use]
    #[inline(always)]
    pub const fn cv_num(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Compare Value Number"]
    #[inline(always)]
    pub const fn set_cv_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Command Buffer Number"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_num(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Command Buffer Number"]
    #[inline(always)]
    pub const fn set_cmd_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
impl core::fmt::Debug for Param {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Param")
            .field("trig_num", &self.trig_num())
            .field("fifosize", &self.fifosize())
            .field("cv_num", &self.cv_num())
            .field("cmd_num", &self.cmd_num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ trig_num: {=u8:?}, fifosize: {:?}, cv_num: {=u8:?}, cmd_num: {=u8:?} }}",
            self.trig_num(),
            self.fifosize(),
            self.cv_num(),
            self.cmd_num()
        )
    }
}
#[doc = "Pause Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pause(pub u32);
impl Pause {
    #[doc = "Pause Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn pausedly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Pause Delay"]
    #[inline(always)]
    pub const fn set_pausedly(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Pause Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pauseen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Pause Enable"]
    #[inline(always)]
    pub const fn set_pauseen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pause {
    #[inline(always)]
    fn default() -> Pause {
        Pause(0)
    }
}
impl core::fmt::Debug for Pause {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pause")
            .field("pausedly", &self.pausedly())
            .field("pauseen", &self.pauseen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pause {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pause {{ pausedly: {=u16:?}, pauseen: {=bool:?} }}",
            self.pausedly(),
            self.pauseen()
        )
    }
}
#[doc = "Data Result FIFO Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resfifo(pub u32);
impl Resfifo {
    #[doc = "Data Result"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Result"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Trigger Source"]
    #[must_use]
    #[inline(always)]
    pub const fn tsrc(&self) -> super::vals::Tsrc {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Tsrc::from_bits(val as u8)
    }
    #[doc = "Trigger Source"]
    #[inline(always)]
    pub const fn set_tsrc(&mut self, val: super::vals::Tsrc) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Loop Count Value"]
    #[must_use]
    #[inline(always)]
    pub const fn loopcnt(&self) -> super::vals::Loopcnt {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Loopcnt::from_bits(val as u8)
    }
    #[doc = "Loop Count Value"]
    #[inline(always)]
    pub const fn set_loopcnt(&mut self, val: super::vals::Loopcnt) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Command Buffer Source"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdsrc(&self) -> super::vals::Cmdsrc {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdsrc::from_bits(val as u8)
    }
    #[doc = "Command Buffer Source"]
    #[inline(always)]
    pub const fn set_cmdsrc(&mut self, val: super::vals::Cmdsrc) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "FIFO Entry is Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Entry is Valid"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Resfifo {
    #[inline(always)]
    fn default() -> Resfifo {
        Resfifo(0)
    }
}
impl core::fmt::Debug for Resfifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Resfifo")
            .field("d", &self.d())
            .field("tsrc", &self.tsrc())
            .field("loopcnt", &self.loopcnt())
            .field("cmdsrc", &self.cmdsrc())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Resfifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Resfifo {{ d: {=u16:?}, tsrc: {:?}, loopcnt: {:?}, cmdsrc: {:?}, valid: {=bool:?} }}",
            self.d(),
            self.tsrc(),
            self.loopcnt(),
            self.cmdsrc(),
            self.valid()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Result FIFO 0 Ready Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rdy0(&self) -> super::vals::Rdy0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rdy0::from_bits(val as u8)
    }
    #[doc = "Result FIFO 0 Ready Flag"]
    #[inline(always)]
    pub const fn set_rdy0(&mut self, val: super::vals::Rdy0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Result FIFO 0 Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fof0(&self) -> super::vals::Fof0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Fof0::from_bits(val as u8)
    }
    #[doc = "Result FIFO 0 Overflow Flag"]
    #[inline(always)]
    pub const fn set_fof0(&mut self, val: super::vals::Fof0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Result FIFO1 Ready Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rdy1(&self) -> super::vals::Rdy1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Rdy1::from_bits(val as u8)
    }
    #[doc = "Result FIFO1 Ready Flag"]
    #[inline(always)]
    pub const fn set_rdy1(&mut self, val: super::vals::Rdy1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Result FIFO1 Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fof1(&self) -> super::vals::Fof1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Fof1::from_bits(val as u8)
    }
    #[doc = "Result FIFO1 Overflow Flag"]
    #[inline(always)]
    pub const fn set_fof1(&mut self, val: super::vals::Fof1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt Flag For High-Priority Trigger Exception"]
    #[must_use]
    #[inline(always)]
    pub const fn texc_int(&self) -> super::vals::TexcInt {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::TexcInt::from_bits(val as u8)
    }
    #[doc = "Interrupt Flag For High-Priority Trigger Exception"]
    #[inline(always)]
    pub const fn set_texc_int(&mut self, val: super::vals::TexcInt) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Flag For Trigger Completion"]
    #[must_use]
    #[inline(always)]
    pub const fn tcomp_int(&self) -> super::vals::TcompInt {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::TcompInt::from_bits(val as u8)
    }
    #[doc = "Interrupt Flag For Trigger Completion"]
    #[inline(always)]
    pub const fn set_tcomp_int(&mut self, val: super::vals::TcompInt) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Calibration Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn cal_rdy(&self) -> super::vals::CalRdy {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::CalRdy::from_bits(val as u8)
    }
    #[doc = "Calibration Ready"]
    #[inline(always)]
    pub const fn set_cal_rdy(&mut self, val: super::vals::CalRdy) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "ADC Active"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_active(&self) -> super::vals::AdcActive {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::AdcActive::from_bits(val as u8)
    }
    #[doc = "ADC Active"]
    #[inline(always)]
    pub const fn set_adc_active(&mut self, val: super::vals::AdcActive) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Trigger Active"]
    #[must_use]
    #[inline(always)]
    pub const fn trgact(&self) -> super::vals::Trgact {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Trgact::from_bits(val as u8)
    }
    #[doc = "Trigger Active"]
    #[inline(always)]
    pub const fn set_trgact(&mut self, val: super::vals::Trgact) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Command Active"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdact(&self) -> super::vals::Cmdact {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdact::from_bits(val as u8)
    }
    #[doc = "Command Active"]
    #[inline(always)]
    pub const fn set_cmdact(&mut self, val: super::vals::Cmdact) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
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
            .field("rdy0", &self.rdy0())
            .field("fof0", &self.fof0())
            .field("rdy1", &self.rdy1())
            .field("fof1", &self.fof1())
            .field("texc_int", &self.texc_int())
            .field("tcomp_int", &self.tcomp_int())
            .field("cal_rdy", &self.cal_rdy())
            .field("adc_active", &self.adc_active())
            .field("trgact", &self.trgact())
            .field("cmdact", &self.cmdact())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stat {{ rdy0: {:?}, fof0: {:?}, rdy1: {:?}, fof1: {:?}, texc_int: {:?}, tcomp_int: {:?}, cal_rdy: {:?}, adc_active: {:?}, trgact: {:?}, cmdact: {:?} }}",
            self.rdy0(),
            self.fof0(),
            self.rdy1(),
            self.fof1(),
            self.texc_int(),
            self.tcomp_int(),
            self.cal_rdy(),
            self.adc_active(),
            self.trgact(),
            self.cmdact()
        )
    }
}
#[doc = "Software Trigger Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swtrig(pub u32);
impl Swtrig {
    #[doc = "Software Trigger 0"]
    #[must_use]
    #[inline(always)]
    pub const fn swt0(&self) -> super::vals::Swt0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Swt0::from_bits(val as u8)
    }
    #[doc = "Software Trigger 0"]
    #[inline(always)]
    pub const fn set_swt0(&mut self, val: super::vals::Swt0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Trigger 1"]
    #[must_use]
    #[inline(always)]
    pub const fn swt1(&self) -> super::vals::Swt1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Swt1::from_bits(val as u8)
    }
    #[doc = "Software Trigger 1"]
    #[inline(always)]
    pub const fn set_swt1(&mut self, val: super::vals::Swt1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Software Trigger 2"]
    #[must_use]
    #[inline(always)]
    pub const fn swt2(&self) -> super::vals::Swt2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Swt2::from_bits(val as u8)
    }
    #[doc = "Software Trigger 2"]
    #[inline(always)]
    pub const fn set_swt2(&mut self, val: super::vals::Swt2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Software Trigger 3"]
    #[must_use]
    #[inline(always)]
    pub const fn swt3(&self) -> super::vals::Swt3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Swt3::from_bits(val as u8)
    }
    #[doc = "Software Trigger 3"]
    #[inline(always)]
    pub const fn set_swt3(&mut self, val: super::vals::Swt3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Swtrig {
    #[inline(always)]
    fn default() -> Swtrig {
        Swtrig(0)
    }
}
impl core::fmt::Debug for Swtrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swtrig")
            .field("swt0", &self.swt0())
            .field("swt1", &self.swt1())
            .field("swt2", &self.swt2())
            .field("swt3", &self.swt3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swtrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swtrig {{ swt0: {:?}, swt1: {:?}, swt2: {:?}, swt3: {:?} }}",
            self.swt0(),
            self.swt1(),
            self.swt2(),
            self.swt3()
        )
    }
}
#[doc = "Trigger Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tctrl(pub u32);
impl Tctrl {
    #[doc = "Trigger Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hten(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Enable"]
    #[inline(always)]
    pub const fn set_hten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SAR Result Destination for Channel A"]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_sel_a(&self) -> super::vals::FifoSelA {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::FifoSelA::from_bits(val as u8)
    }
    #[doc = "SAR Result Destination for Channel A"]
    #[inline(always)]
    pub const fn set_fifo_sel_a(&mut self, val: super::vals::FifoSelA) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SAR Result Destination for Channel B"]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_sel_b(&self) -> super::vals::FifoSelB {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::FifoSelB::from_bits(val as u8)
    }
    #[doc = "SAR Result Destination for Channel B"]
    #[inline(always)]
    pub const fn set_fifo_sel_b(&mut self, val: super::vals::FifoSelB) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Trigger Priority Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn tpri(&self) -> super::vals::Tpri {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Tpri::from_bits(val as u8)
    }
    #[doc = "Trigger Priority Setting"]
    #[inline(always)]
    pub const fn set_tpri(&mut self, val: super::vals::Tpri) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Trigger Resync"]
    #[must_use]
    #[inline(always)]
    pub const fn rsync(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Resync"]
    #[inline(always)]
    pub const fn set_rsync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Trigger Delay Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tdly(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Trigger Delay Select"]
    #[inline(always)]
    pub const fn set_tdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Trigger Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tcmd(&self) -> super::vals::Tcmd {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Tcmd::from_bits(val as u8)
    }
    #[doc = "Trigger Command Select"]
    #[inline(always)]
    pub const fn set_tcmd(&mut self, val: super::vals::Tcmd) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Tctrl {
    #[inline(always)]
    fn default() -> Tctrl {
        Tctrl(0)
    }
}
impl core::fmt::Debug for Tctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tctrl")
            .field("hten", &self.hten())
            .field("fifo_sel_a", &self.fifo_sel_a())
            .field("fifo_sel_b", &self.fifo_sel_b())
            .field("tpri", &self.tpri())
            .field("rsync", &self.rsync())
            .field("tdly", &self.tdly())
            .field("tcmd", &self.tcmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tctrl {{ hten: {=bool:?}, fifo_sel_a: {:?}, fifo_sel_b: {:?}, tpri: {:?}, rsync: {=bool:?}, tdly: {=u8:?}, tcmd: {:?} }}",
            self.hten(),
            self.fifo_sel_a(),
            self.fifo_sel_b(),
            self.tpri(),
            self.rsync(),
            self.tdly(),
            self.tcmd()
        )
    }
}
#[doc = "Trigger Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tstat(pub u32);
impl Tstat {
    #[doc = "Trigger Exception Number"]
    #[must_use]
    #[inline(always)]
    pub const fn texc_num(&self) -> super::vals::TexcNum {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::TexcNum::from_bits(val as u8)
    }
    #[doc = "Trigger Exception Number"]
    #[inline(always)]
    pub const fn set_texc_num(&mut self, val: super::vals::TexcNum) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Trigger Completion Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcomp_flag(&self) -> super::vals::TcompFlag {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::TcompFlag::from_bits(val as u8)
    }
    #[doc = "Trigger Completion Flag"]
    #[inline(always)]
    pub const fn set_tcomp_flag(&mut self, val: super::vals::TcompFlag) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
}
impl Default for Tstat {
    #[inline(always)]
    fn default() -> Tstat {
        Tstat(0)
    }
}
impl core::fmt::Debug for Tstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tstat")
            .field("texc_num", &self.texc_num())
            .field("tcomp_flag", &self.tcomp_flag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tstat {{ texc_num: {:?}, tcomp_flag: {:?} }}",
            self.texc_num(),
            self.tcomp_flag()
        )
    }
}
#[doc = "Version ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Resolution"]
    #[must_use]
    #[inline(always)]
    pub const fn res(&self) -> super::vals::Res {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Res::from_bits(val as u8)
    }
    #[doc = "Resolution"]
    #[inline(always)]
    pub const fn set_res(&mut self, val: super::vals::Res) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Differential Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn diffen(&self) -> super::vals::Diffen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Diffen::from_bits(val as u8)
    }
    #[doc = "Differential Supported"]
    #[inline(always)]
    pub const fn set_diffen(&mut self, val: super::vals::Diffen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Multiple Vref Implemented"]
    #[must_use]
    #[inline(always)]
    pub const fn mvi(&self) -> super::vals::Mvi {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mvi::from_bits(val as u8)
    }
    #[doc = "Multiple Vref Implemented"]
    #[inline(always)]
    pub const fn set_mvi(&mut self, val: super::vals::Mvi) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel Scale Width"]
    #[must_use]
    #[inline(always)]
    pub const fn csw(&self) -> super::vals::Csw {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Csw::from_bits(val as u8)
    }
    #[doc = "Channel Scale Width"]
    #[inline(always)]
    pub const fn set_csw(&mut self, val: super::vals::Csw) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Voltage Reference 1 Range Control Bit Implemented"]
    #[must_use]
    #[inline(always)]
    pub const fn vr1rngi(&self) -> super::vals::Vr1rngi {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Vr1rngi::from_bits(val as u8)
    }
    #[doc = "Voltage Reference 1 Range Control Bit Implemented"]
    #[inline(always)]
    pub const fn set_vr1rngi(&mut self, val: super::vals::Vr1rngi) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Internal ADC Clock Implemented"]
    #[must_use]
    #[inline(always)]
    pub const fn iadcki(&self) -> super::vals::Iadcki {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Iadcki::from_bits(val as u8)
    }
    #[doc = "Internal ADC Clock Implemented"]
    #[inline(always)]
    pub const fn set_iadcki(&mut self, val: super::vals::Iadcki) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Calibration Function Implemented"]
    #[must_use]
    #[inline(always)]
    pub const fn calofsi(&self) -> super::vals::Calofsi {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Calofsi::from_bits(val as u8)
    }
    #[doc = "Calibration Function Implemented"]
    #[inline(always)]
    pub const fn set_calofsi(&mut self, val: super::vals::Calofsi) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Number of Single-Ended Outputs Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_sec(&self) -> super::vals::NumSec {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::NumSec::from_bits(val as u8)
    }
    #[doc = "Number of Single-Ended Outputs Supported"]
    #[inline(always)]
    pub const fn set_num_sec(&mut self, val: super::vals::NumSec) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Number of FIFOs"]
    #[must_use]
    #[inline(always)]
    pub const fn num_fifo(&self) -> super::vals::NumFifo {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::NumFifo::from_bits(val as u8)
    }
    #[doc = "Number of FIFOs"]
    #[inline(always)]
    pub const fn set_num_fifo(&mut self, val: super::vals::NumFifo) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Minor Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(0)
    }
}
impl core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Verid")
            .field("res", &self.res())
            .field("diffen", &self.diffen())
            .field("mvi", &self.mvi())
            .field("csw", &self.csw())
            .field("vr1rngi", &self.vr1rngi())
            .field("iadcki", &self.iadcki())
            .field("calofsi", &self.calofsi())
            .field("num_sec", &self.num_sec())
            .field("num_fifo", &self.num_fifo())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Verid {{ res: {:?}, diffen: {:?}, mvi: {:?}, csw: {:?}, vr1rngi: {:?}, iadcki: {:?}, calofsi: {:?}, num_sec: {:?}, num_fifo: {:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.res(),
            self.diffen(),
            self.mvi(),
            self.csw(),
            self.vr1rngi(),
            self.iadcki(),
            self.calofsi(),
            self.num_sec(),
            self.num_fifo(),
            self.minor(),
            self.major()
        )
    }
}
