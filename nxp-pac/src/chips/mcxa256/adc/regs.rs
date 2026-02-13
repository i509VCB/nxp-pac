#[doc = "Calibration General A-Side Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalGar(pub u32);
impl CalGar {
    #[doc = "Calibration General A Side Register Element"]
    #[must_use]
    #[inline(always)]
    pub const fn cal_gar_val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Calibration General A Side Register Element"]
    #[inline(always)]
    pub const fn set_cal_gar_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pwrsel::from_bits(val as u8)
    }
    #[doc = "Power Configuration Select"]
    #[inline(always)]
    pub const fn set_pwrsel(&mut self, val: super::vals::Pwrsel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
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
    #[doc = "High Priority Trigger Exception Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn hpt_exdi(&self) -> super::vals::HptExdi {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::HptExdi::from_bits(val as u8)
    }
    #[doc = "High Priority Trigger Exception Disable"]
    #[inline(always)]
    pub const fn set_hpt_exdi(&mut self, val: super::vals::HptExdi) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Power Up Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn pudly(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Power Up Delay"]
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
#[doc = "Configuration 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg2(pub u32);
impl Cfg2 {
    #[doc = "Justified Left Enable register"]
    #[must_use]
    #[inline(always)]
    pub const fn jleft(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Justified Left Enable register"]
    #[inline(always)]
    pub const fn set_jleft(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "High Speed Enable register"]
    #[must_use]
    #[inline(always)]
    pub const fn hs(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "High Speed Enable register"]
    #[inline(always)]
    pub const fn set_hs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "High Speed Extra register"]
    #[must_use]
    #[inline(always)]
    pub const fn hsextra(&self) -> super::vals::Hsextra {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Hsextra::from_bits(val as u8)
    }
    #[doc = "High Speed Extra register"]
    #[inline(always)]
    pub const fn set_hsextra(&mut self, val: super::vals::Hsextra) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Tune Mode register"]
    #[must_use]
    #[inline(always)]
    pub const fn tune(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Tune Mode register"]
    #[inline(always)]
    pub const fn set_tune(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for Cfg2 {
    #[inline(always)]
    fn default() -> Cfg2 {
        Cfg2(0)
    }
}
impl core::fmt::Debug for Cfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg2")
            .field("jleft", &self.jleft())
            .field("hs", &self.hs())
            .field("hsextra", &self.hsextra())
            .field("tune", &self.tune())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg2 {{ jleft: {=bool:?}, hs: {=bool:?}, hsextra: {:?}, tune: {=u8:?} }}",
            self.jleft(),
            self.hs(),
            self.hsextra(),
            self.tune()
        )
    }
}
#[doc = "Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh(pub u32);
impl Cmdh {
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: super::vals::Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for Trigger Assertion before Execution."]
    #[must_use]
    #[inline(always)]
    pub const fn wait_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Trigger Assertion before Execution."]
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
    pub const fn sts(&self) -> super::vals::Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: super::vals::Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Avgs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::Avgs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Next {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn set_next(&mut self, val: super::vals::Next) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Cmdh {
    #[inline(always)]
    fn default() -> Cmdh {
        Cmdh(0)
    }
}
impl core::fmt::Debug for Cmdh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdh")
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
impl defmt::Format for Cmdh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdh {{ cmpen: {:?}, wait_trig: {=bool:?}, lwi: {=bool:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
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
pub struct Cmdl(pub u32);
impl Cmdl {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ctype(&self) -> super::vals::Ctype {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Ctype::from_bits(val as u8)
    }
    #[doc = "Conversion Type"]
    #[inline(always)]
    pub const fn set_ctype(&mut self, val: super::vals::Ctype) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select Resolution of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Select Resolution of Conversions"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Cmdl {
    #[inline(always)]
    fn default() -> Cmdl {
        Cmdl(0)
    }
}
impl core::fmt::Debug for Cmdl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdl")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl {{ adch: {=u8:?}, ctype: {:?}, mode: {:?} }}",
            self.adch(),
            self.ctype(),
            self.mode()
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
    pub const fn rst(&self) -> super::vals::Rst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rst::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: super::vals::Rst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
    #[doc = "High Speed Mode Trim Request"]
    #[must_use]
    #[inline(always)]
    pub const fn calhs(&self) -> super::vals::Calhs {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Calhs::from_bits(val as u8)
    }
    #[doc = "High Speed Mode Trim Request"]
    #[inline(always)]
    pub const fn set_calhs(&mut self, val: super::vals::Calhs) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Reset FIFO 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rstfifo0(&self) -> super::vals::Rstfifo0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Rstfifo0::from_bits(val as u8)
    }
    #[doc = "Reset FIFO 0"]
    #[inline(always)]
    pub const fn set_rstfifo0(&mut self, val: super::vals::Rstfifo0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
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
            .field("calhs", &self.calhs())
            .field("rstfifo0", &self.rstfifo0())
            .field("cal_avgs", &self.cal_avgs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ adcen: {=bool:?}, rst: {:?}, dozen: {:?}, cal_req: {:?}, calofs: {:?}, calhs: {:?}, rstfifo0: {:?}, cal_avgs: {:?} }}",
            self.adcen(),
            self.rst(),
            self.dozen(),
            self.cal_req(),
            self.calofs(),
            self.calhs(),
            self.rstfifo0(),
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for De {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "De {{ fwmde0: {=bool:?} }}", self.fwmde0())
    }
}
#[doc = "FIFO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl0(pub u32);
impl Fctrl0 {
    #[doc = "Result FIFO Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn fcount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Result FIFO Counter"]
    #[inline(always)]
    pub const fn set_fcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Watermark Level Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn fwmark(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Watermark Level Selection"]
    #[inline(always)]
    pub const fn set_fwmark(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for Fctrl0 {
    #[inline(always)]
    fn default() -> Fctrl0 {
        Fctrl0(0)
    }
}
impl core::fmt::Debug for Fctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fctrl0")
            .field("fcount", &self.fcount())
            .field("fwmark", &self.fwmark())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fctrl0 {{ fcount: {=u8:?}, fwmark: {=u8:?} }}",
            self.fcount(),
            self.fwmark()
        )
    }
}
#[doc = "Gain Calibration Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gcc0(pub u32);
impl Gcc0 {
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
    pub const fn rdy(&self) -> super::vals::Gcc0Rdy {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Gcc0Rdy::from_bits(val as u8)
    }
    #[doc = "Gain Calibration Value Valid"]
    #[inline(always)]
    pub const fn set_rdy(&mut self, val: super::vals::Gcc0Rdy) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Gcc0 {
    #[inline(always)]
    fn default() -> Gcc0 {
        Gcc0(0)
    }
}
impl core::fmt::Debug for Gcc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gcc0")
            .field("gain_cal", &self.gain_cal())
            .field("rdy", &self.rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gcc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gcc0 {{ gain_cal: {=u16:?}, rdy: {:?} }}",
            self.gain_cal(),
            self.rdy()
        )
    }
}
#[doc = "Gain Calculation Result"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gcr0(pub u32);
impl Gcr0 {
    #[doc = "Gain Calculation Result"]
    #[must_use]
    #[inline(always)]
    pub const fn gcalr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Gain Calculation Result"]
    #[inline(always)]
    pub const fn set_gcalr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
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
impl Default for Gcr0 {
    #[inline(always)]
    fn default() -> Gcr0 {
        Gcr0(0)
    }
}
impl core::fmt::Debug for Gcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gcr0")
            .field("gcalr", &self.gcalr())
            .field("rdy", &self.rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gcr0 {{ gcalr: {=u32:?}, rdy: {=bool:?} }}",
            self.gcalr(),
            self.rdy()
        )
    }
}
#[doc = "High Speed Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hstrim(pub u32);
impl Hstrim {
    #[doc = "Trim for High Speed Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn hstrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim for High Speed Conversions"]
    #[inline(always)]
    pub const fn set_hstrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Hstrim {
    #[inline(always)]
    fn default() -> Hstrim {
        Hstrim(0)
    }
}
impl core::fmt::Debug for Hstrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hstrim")
            .field("hstrim", &self.hstrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hstrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hstrim {{ hstrim: {=u8:?} }}", self.hstrim())
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
            "Ie {{ fwmie0: {=bool:?}, fofie0: {=bool:?}, texc_ie: {=bool:?}, tcomp_ie: {:?} }}",
            self.fwmie0(),
            self.fofie0(),
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
    pub const fn ofstrim(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Trim for Offset"]
    #[inline(always)]
    pub const fn set_ofstrim(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
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
            .field("ofstrim", &self.ofstrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ofstrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ofstrim {{ ofstrim: {=u16:?} }}", self.ofstrim())
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
    #[doc = "PAUSE Option Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pauseen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "PAUSE Option Enable"]
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
pub struct Resfifo0(pub u32);
impl Resfifo0 {
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
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Cmdsrc::from_bits(val as u8)
    }
    #[doc = "Command Buffer Source"]
    #[inline(always)]
    pub const fn set_cmdsrc(&mut self, val: super::vals::Cmdsrc) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
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
impl Default for Resfifo0 {
    #[inline(always)]
    fn default() -> Resfifo0 {
        Resfifo0(0)
    }
}
impl core::fmt::Debug for Resfifo0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Resfifo0")
            .field("d", &self.d())
            .field("tsrc", &self.tsrc())
            .field("loopcnt", &self.loopcnt())
            .field("cmdsrc", &self.cmdsrc())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Resfifo0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Resfifo0 {{ d: {=u16:?}, tsrc: {:?}, loopcnt: {:?}, cmdsrc: {:?}, valid: {=bool:?} }}",
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
    #[doc = "Interrupt Flag For High Priority Trigger Exception"]
    #[must_use]
    #[inline(always)]
    pub const fn texc_int(&self) -> super::vals::TexcInt {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::TexcInt::from_bits(val as u8)
    }
    #[doc = "Interrupt Flag For High Priority Trigger Exception"]
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
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Cmdact::from_bits(val as u8)
    }
    #[doc = "Command Active"]
    #[inline(always)]
    pub const fn set_cmdact(&mut self, val: super::vals::Cmdact) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
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
            "Stat {{ rdy0: {:?}, fof0: {:?}, texc_int: {:?}, tcomp_int: {:?}, cal_rdy: {:?}, adc_active: {:?}, trgact: {:?}, cmdact: {:?} }}",
            self.rdy0(),
            self.fof0(),
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
    #[doc = "Software Trigger 0 Event"]
    #[must_use]
    #[inline(always)]
    pub const fn swt(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Software Trigger 0 Event"]
    #[inline(always)]
    pub const fn set_swt(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("swt[0]", &self.swt(0usize))
            .field("swt[1]", &self.swt(1usize))
            .field("swt[2]", &self.swt(2usize))
            .field("swt[3]", &self.swt(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swtrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swtrig {{ swt[0]: {=bool:?}, swt[1]: {=bool:?}, swt[2]: {=bool:?}, swt[3]: {=bool:?} }}",
            self.swt(0usize),
            self.swt(1usize),
            self.swt(2usize),
            self.swt(3usize)
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
    #[doc = "Trigger Synchronous Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tsync(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Synchronous Select"]
    #[inline(always)]
    pub const fn set_tsync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Trigger Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tcmd(&self) -> super::vals::Tcmd {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Tcmd::from_bits(val as u8)
    }
    #[doc = "Trigger Command Select"]
    #[inline(always)]
    pub const fn set_tcmd(&mut self, val: super::vals::Tcmd) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
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
            .field("tpri", &self.tpri())
            .field("rsync", &self.rsync())
            .field("tdly", &self.tdly())
            .field("tsync", &self.tsync())
            .field("tcmd", &self.tcmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tctrl {{ hten: {=bool:?}, tpri: {:?}, rsync: {=bool:?}, tdly: {=u8:?}, tsync: {=bool:?}, tcmd: {:?} }}",
            self.hten(),
            self.tpri(),
            self.rsync(),
            self.tdly(),
            self.tsync(),
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
    #[doc = "Multi Vref Implemented"]
    #[must_use]
    #[inline(always)]
    pub const fn mvi(&self) -> super::vals::Mvi {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mvi::from_bits(val as u8)
    }
    #[doc = "Multi Vref Implemented"]
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
    #[doc = "Number of Single Ended Outputs Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_sec(&self) -> super::vals::NumSec {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::NumSec::from_bits(val as u8)
    }
    #[doc = "Number of Single Ended Outputs Supported"]
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
