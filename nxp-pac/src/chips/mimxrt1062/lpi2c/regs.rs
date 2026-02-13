#[doc = "Master Clock Configuration 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mccr0(pub u32);
impl Mccr0 {
    #[doc = "Clock Low Period"]
    #[must_use]
    #[inline(always)]
    pub const fn clklo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock Low Period"]
    #[inline(always)]
    pub const fn set_clklo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Clock High Period"]
    #[must_use]
    #[inline(always)]
    pub const fn clkhi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock High Period"]
    #[inline(always)]
    pub const fn set_clkhi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Setup Hold Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn sethold(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Setup Hold Delay"]
    #[inline(always)]
    pub const fn set_sethold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Data Valid Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn datavd(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Data Valid Delay"]
    #[inline(always)]
    pub const fn set_datavd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Mccr0 {
    #[inline(always)]
    fn default() -> Mccr0 {
        Mccr0(0)
    }
}
impl core::fmt::Debug for Mccr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mccr0")
            .field("clklo", &self.clklo())
            .field("clkhi", &self.clkhi())
            .field("sethold", &self.sethold())
            .field("datavd", &self.datavd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mccr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mccr0 {{ clklo: {=u8:?}, clkhi: {=u8:?}, sethold: {=u8:?}, datavd: {=u8:?} }}",
            self.clklo(),
            self.clkhi(),
            self.sethold(),
            self.datavd()
        )
    }
}
#[doc = "Master Clock Configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mccr1(pub u32);
impl Mccr1 {
    #[doc = "Clock Low Period"]
    #[must_use]
    #[inline(always)]
    pub const fn clklo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock Low Period"]
    #[inline(always)]
    pub const fn set_clklo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Clock High Period"]
    #[must_use]
    #[inline(always)]
    pub const fn clkhi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock High Period"]
    #[inline(always)]
    pub const fn set_clkhi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Setup Hold Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn sethold(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Setup Hold Delay"]
    #[inline(always)]
    pub const fn set_sethold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Data Valid Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn datavd(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Data Valid Delay"]
    #[inline(always)]
    pub const fn set_datavd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Mccr1 {
    #[inline(always)]
    fn default() -> Mccr1 {
        Mccr1(0)
    }
}
impl core::fmt::Debug for Mccr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mccr1")
            .field("clklo", &self.clklo())
            .field("clkhi", &self.clkhi())
            .field("sethold", &self.sethold())
            .field("datavd", &self.datavd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mccr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mccr1 {{ clklo: {=u8:?}, clkhi: {=u8:?}, sethold: {=u8:?}, datavd: {=u8:?} }}",
            self.clklo(),
            self.clkhi(),
            self.sethold(),
            self.datavd()
        )
    }
}
#[doc = "Master Configuration 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcfgr0(pub u32);
impl Mcfgr0 {
    #[doc = "Host Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Host Request Enable"]
    #[inline(always)]
    pub const fn set_hren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Request Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn hrpol(&self) -> super::vals::Hrpol {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Hrpol::from_bits(val as u8)
    }
    #[doc = "Host Request Polarity"]
    #[inline(always)]
    pub const fn set_hrpol(&mut self, val: super::vals::Hrpol) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Host Request Select"]
    #[must_use]
    #[inline(always)]
    pub const fn hrsel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Host Request Select"]
    #[inline(always)]
    pub const fn set_hrsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Circular FIFO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cirfifo(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Circular FIFO Enable"]
    #[inline(always)]
    pub const fn set_cirfifo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive Data Match Only"]
    #[must_use]
    #[inline(always)]
    pub const fn rdmo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Match Only"]
    #[inline(always)]
    pub const fn set_rdmo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Mcfgr0 {
    #[inline(always)]
    fn default() -> Mcfgr0 {
        Mcfgr0(0)
    }
}
impl core::fmt::Debug for Mcfgr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcfgr0")
            .field("hren", &self.hren())
            .field("hrpol", &self.hrpol())
            .field("hrsel", &self.hrsel())
            .field("cirfifo", &self.cirfifo())
            .field("rdmo", &self.rdmo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcfgr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcfgr0 {{ hren: {=bool:?}, hrpol: {:?}, hrsel: {=bool:?}, cirfifo: {=bool:?}, rdmo: {=bool:?} }}",
            self.hren(),
            self.hrpol(),
            self.hrsel(),
            self.cirfifo(),
            self.rdmo()
        )
    }
}
#[doc = "Master Configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcfgr1(pub u32);
impl Mcfgr1 {
    #[doc = "Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn prescale(&self) -> super::vals::Prescale {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Prescale::from_bits(val as u8)
    }
    #[doc = "Prescaler"]
    #[inline(always)]
    pub const fn set_prescale(&mut self, val: super::vals::Prescale) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Automatic STOP Generation"]
    #[must_use]
    #[inline(always)]
    pub const fn autostop(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic STOP Generation"]
    #[inline(always)]
    pub const fn set_autostop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "IGNACK"]
    #[must_use]
    #[inline(always)]
    pub const fn ignack(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "IGNACK"]
    #[inline(always)]
    pub const fn set_ignack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Timeout Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn timecfg(&self) -> super::vals::Timecfg {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Timecfg::from_bits(val as u8)
    }
    #[doc = "Timeout Configuration"]
    #[inline(always)]
    pub const fn set_timecfg(&mut self, val: super::vals::Timecfg) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Match Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn matcfg(&self) -> super::vals::Matcfg {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Matcfg::from_bits(val as u8)
    }
    #[doc = "Match Configuration"]
    #[inline(always)]
    pub const fn set_matcfg(&mut self, val: super::vals::Matcfg) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Pin Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pincfg(&self) -> super::vals::Pincfg {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Pincfg::from_bits(val as u8)
    }
    #[doc = "Pin Configuration"]
    #[inline(always)]
    pub const fn set_pincfg(&mut self, val: super::vals::Pincfg) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Mcfgr1 {
    #[inline(always)]
    fn default() -> Mcfgr1 {
        Mcfgr1(0)
    }
}
impl core::fmt::Debug for Mcfgr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcfgr1")
            .field("prescale", &self.prescale())
            .field("autostop", &self.autostop())
            .field("ignack", &self.ignack())
            .field("timecfg", &self.timecfg())
            .field("matcfg", &self.matcfg())
            .field("pincfg", &self.pincfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcfgr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcfgr1 {{ prescale: {:?}, autostop: {=bool:?}, ignack: {=bool:?}, timecfg: {:?}, matcfg: {:?}, pincfg: {:?} }}",
            self.prescale(),
            self.autostop(),
            self.ignack(),
            self.timecfg(),
            self.matcfg(),
            self.pincfg()
        )
    }
}
#[doc = "Master Configuration 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcfgr2(pub u32);
impl Mcfgr2 {
    #[doc = "Bus Idle Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn busidle(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Bus Idle Timeout"]
    #[inline(always)]
    pub const fn set_busidle(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Glitch Filter SCL"]
    #[must_use]
    #[inline(always)]
    pub const fn filtscl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Glitch Filter SCL"]
    #[inline(always)]
    pub const fn set_filtscl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Glitch Filter SDA"]
    #[must_use]
    #[inline(always)]
    pub const fn filtsda(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Glitch Filter SDA"]
    #[inline(always)]
    pub const fn set_filtsda(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Mcfgr2 {
    #[inline(always)]
    fn default() -> Mcfgr2 {
        Mcfgr2(0)
    }
}
impl core::fmt::Debug for Mcfgr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcfgr2")
            .field("busidle", &self.busidle())
            .field("filtscl", &self.filtscl())
            .field("filtsda", &self.filtsda())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcfgr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcfgr2 {{ busidle: {=u16:?}, filtscl: {=u8:?}, filtsda: {=u8:?} }}",
            self.busidle(),
            self.filtscl(),
            self.filtsda()
        )
    }
}
#[doc = "Master Configuration 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcfgr3(pub u32);
impl Mcfgr3 {
    #[doc = "Pin Low Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn pinlow(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "Pin Low Timeout"]
    #[inline(always)]
    pub const fn set_pinlow(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
}
impl Default for Mcfgr3 {
    #[inline(always)]
    fn default() -> Mcfgr3 {
        Mcfgr3(0)
    }
}
impl core::fmt::Debug for Mcfgr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcfgr3")
            .field("pinlow", &self.pinlow())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcfgr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mcfgr3 {{ pinlow: {=u16:?} }}", self.pinlow())
    }
}
#[doc = "Master Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc = "Master Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn men(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Master Enable"]
    #[inline(always)]
    pub const fn set_men(&mut self, val: bool) {
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
    #[doc = "Doze mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dozen(&self) -> super::vals::Dozen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dozen::from_bits(val as u8)
    }
    #[doc = "Doze mode enable"]
    #[inline(always)]
    pub const fn set_dozen(&mut self, val: super::vals::Dozen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reset Transmit FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn rtf(&self) -> super::vals::McrRtf {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::McrRtf::from_bits(val as u8)
    }
    #[doc = "Reset Transmit FIFO"]
    #[inline(always)]
    pub const fn set_rtf(&mut self, val: super::vals::McrRtf) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Reset Receive FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn rrf(&self) -> super::vals::McrRrf {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::McrRrf::from_bits(val as u8)
    }
    #[doc = "Reset Receive FIFO"]
    #[inline(always)]
    pub const fn set_rrf(&mut self, val: super::vals::McrRrf) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        Mcr(0)
    }
}
impl core::fmt::Debug for Mcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr")
            .field("men", &self.men())
            .field("rst", &self.rst())
            .field("dozen", &self.dozen())
            .field("dbgen", &self.dbgen())
            .field("rtf", &self.rtf())
            .field("rrf", &self.rrf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr {{ men: {=bool:?}, rst: {=bool:?}, dozen: {:?}, dbgen: {=bool:?}, rtf: {:?}, rrf: {:?} }}",
            self.men(),
            self.rst(),
            self.dozen(),
            self.dbgen(),
            self.rtf(),
            self.rrf()
        )
    }
}
#[doc = "Master DMA Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mder(pub u32);
impl Mder {
    #[doc = "Transmit Data DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data DMA Enable"]
    #[inline(always)]
    pub const fn set_tdde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rdde(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data DMA Enable"]
    #[inline(always)]
    pub const fn set_rdde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Mder {
    #[inline(always)]
    fn default() -> Mder {
        Mder(0)
    }
}
impl core::fmt::Debug for Mder {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mder")
            .field("tdde", &self.tdde())
            .field("rdde", &self.rdde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mder {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mder {{ tdde: {=bool:?}, rdde: {=bool:?} }}",
            self.tdde(),
            self.rdde()
        )
    }
}
#[doc = "Master Data Match"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdmr(pub u32);
impl Mdmr {
    #[doc = "Match 0 Value"]
    #[must_use]
    #[inline(always)]
    pub const fn match0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Match 0 Value"]
    #[inline(always)]
    pub const fn set_match0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Match 1 Value"]
    #[must_use]
    #[inline(always)]
    pub const fn match1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Match 1 Value"]
    #[inline(always)]
    pub const fn set_match1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Mdmr {
    #[inline(always)]
    fn default() -> Mdmr {
        Mdmr(0)
    }
}
impl core::fmt::Debug for Mdmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mdmr")
            .field("match0", &self.match0())
            .field("match1", &self.match1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mdmr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mdmr {{ match0: {=u8:?}, match1: {=u8:?} }}",
            self.match0(),
            self.match1()
        )
    }
}
#[doc = "Master FIFO Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mfcr(pub u32);
impl Mfcr {
    #[doc = "Transmit FIFO Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn txwater(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Transmit FIFO Watermark"]
    #[inline(always)]
    pub const fn set_txwater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Receive FIFO Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn rxwater(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Receive FIFO Watermark"]
    #[inline(always)]
    pub const fn set_rxwater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for Mfcr {
    #[inline(always)]
    fn default() -> Mfcr {
        Mfcr(0)
    }
}
impl core::fmt::Debug for Mfcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mfcr")
            .field("txwater", &self.txwater())
            .field("rxwater", &self.rxwater())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mfcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mfcr {{ txwater: {=u8:?}, rxwater: {=u8:?} }}",
            self.txwater(),
            self.rxwater()
        )
    }
}
#[doc = "Master FIFO Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mfsr(pub u32);
impl Mfsr {
    #[doc = "Transmit FIFO Count"]
    #[must_use]
    #[inline(always)]
    pub const fn txcount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit FIFO Count"]
    #[inline(always)]
    pub const fn set_txcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Receive FIFO Count"]
    #[must_use]
    #[inline(always)]
    pub const fn rxcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Receive FIFO Count"]
    #[inline(always)]
    pub const fn set_rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for Mfsr {
    #[inline(always)]
    fn default() -> Mfsr {
        Mfsr(0)
    }
}
impl core::fmt::Debug for Mfsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mfsr")
            .field("txcount", &self.txcount())
            .field("rxcount", &self.rxcount())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mfsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mfsr {{ txcount: {=u8:?}, rxcount: {=u8:?} }}",
            self.txcount(),
            self.rxcount()
        )
    }
}
#[doc = "Master Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mier(pub u32);
impl Mier {
    #[doc = "Transmit Data Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rdie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "End Packet Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn epie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "End Packet Interrupt Enable"]
    #[inline(always)]
    pub const fn set_epie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "STOP Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sdie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "STOP Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "NACK Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ndie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "NACK Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ndie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Arbitration Lost Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub const fn set_alie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FIFO Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn feie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_feie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pin Low Timeout Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pltie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pin Low Timeout Interrupt Enable"]
    #[inline(always)]
    pub const fn set_pltie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Data Match Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Data Match Interrupt Enable"]
    #[inline(always)]
    pub const fn set_dmie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Mier {
    #[inline(always)]
    fn default() -> Mier {
        Mier(0)
    }
}
impl core::fmt::Debug for Mier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mier")
            .field("tdie", &self.tdie())
            .field("rdie", &self.rdie())
            .field("epie", &self.epie())
            .field("sdie", &self.sdie())
            .field("ndie", &self.ndie())
            .field("alie", &self.alie())
            .field("feie", &self.feie())
            .field("pltie", &self.pltie())
            .field("dmie", &self.dmie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mier {{ tdie: {=bool:?}, rdie: {=bool:?}, epie: {=bool:?}, sdie: {=bool:?}, ndie: {=bool:?}, alie: {=bool:?}, feie: {=bool:?}, pltie: {=bool:?}, dmie: {=bool:?} }}",
            self.tdie(),
            self.rdie(),
            self.epie(),
            self.sdie(),
            self.ndie(),
            self.alie(),
            self.feie(),
            self.pltie(),
            self.dmie()
        )
    }
}
#[doc = "Master Receive Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrdr(pub u32);
impl Mrdr {
    #[doc = "Receive Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn rxempty(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Empty"]
    #[inline(always)]
    pub const fn set_rxempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Mrdr {
    #[inline(always)]
    fn default() -> Mrdr {
        Mrdr(0)
    }
}
impl core::fmt::Debug for Mrdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mrdr")
            .field("data", &self.data())
            .field("rxempty", &self.rxempty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mrdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mrdr {{ data: {=u8:?}, rxempty: {=bool:?} }}",
            self.data(),
            self.rxempty()
        )
    }
}
#[doc = "Master Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msr(pub u32);
impl Msr {
    #[doc = "Transmit Data Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tdf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Flag"]
    #[inline(always)]
    pub const fn set_tdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rdf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Flag"]
    #[inline(always)]
    pub const fn set_rdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "End Packet Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn epf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "End Packet Flag"]
    #[inline(always)]
    pub const fn set_epf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "STOP Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sdf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "STOP Detect Flag"]
    #[inline(always)]
    pub const fn set_sdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "NACK Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ndf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "NACK Detect Flag"]
    #[inline(always)]
    pub const fn set_ndf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Arbitration Lost Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn alf(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration Lost Flag"]
    #[inline(always)]
    pub const fn set_alf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FIFO Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fef(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Flag"]
    #[inline(always)]
    pub const fn set_fef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pin Low Timeout Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn pltf(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pin Low Timeout Flag"]
    #[inline(always)]
    pub const fn set_pltf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Data Match Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn dmf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Data Match Flag"]
    #[inline(always)]
    pub const fn set_dmf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Master Busy Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn mbf(&self) -> super::vals::Mbf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Mbf::from_bits(val as u8)
    }
    #[doc = "Master Busy Flag"]
    #[inline(always)]
    pub const fn set_mbf(&mut self, val: super::vals::Mbf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus Busy Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bbf(&self) -> super::vals::MsrBbf {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::MsrBbf::from_bits(val as u8)
    }
    #[doc = "Bus Busy Flag"]
    #[inline(always)]
    pub const fn set_bbf(&mut self, val: super::vals::MsrBbf) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Msr {
    #[inline(always)]
    fn default() -> Msr {
        Msr(0)
    }
}
impl core::fmt::Debug for Msr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Msr")
            .field("tdf", &self.tdf())
            .field("rdf", &self.rdf())
            .field("epf", &self.epf())
            .field("sdf", &self.sdf())
            .field("ndf", &self.ndf())
            .field("alf", &self.alf())
            .field("fef", &self.fef())
            .field("pltf", &self.pltf())
            .field("dmf", &self.dmf())
            .field("mbf", &self.mbf())
            .field("bbf", &self.bbf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Msr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Msr {{ tdf: {=bool:?}, rdf: {=bool:?}, epf: {=bool:?}, sdf: {=bool:?}, ndf: {=bool:?}, alf: {=bool:?}, fef: {=bool:?}, pltf: {=bool:?}, dmf: {=bool:?}, mbf: {:?}, bbf: {:?} }}",
            self.tdf(),
            self.rdf(),
            self.epf(),
            self.sdf(),
            self.ndf(),
            self.alf(),
            self.fef(),
            self.pltf(),
            self.dmf(),
            self.mbf(),
            self.bbf()
        )
    }
}
#[doc = "Master Transmit Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtdr(pub u32);
impl Mtdr {
    #[doc = "Transmit Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Command Data"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd(&self) -> super::vals::Cmd {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmd::from_bits(val as u8)
    }
    #[doc = "Command Data"]
    #[inline(always)]
    pub const fn set_cmd(&mut self, val: super::vals::Cmd) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
}
impl Default for Mtdr {
    #[inline(always)]
    fn default() -> Mtdr {
        Mtdr(0)
    }
}
impl core::fmt::Debug for Mtdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mtdr")
            .field("data", &self.data())
            .field("cmd", &self.cmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mtdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mtdr {{ data: {=u8:?}, cmd: {:?} }}",
            self.data(),
            self.cmd()
        )
    }
}
#[doc = "Parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Master Transmit FIFO Size"]
    #[must_use]
    #[inline(always)]
    pub const fn mtxfifo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Master Transmit FIFO Size"]
    #[inline(always)]
    pub const fn set_mtxfifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Master Receive FIFO Size"]
    #[must_use]
    #[inline(always)]
    pub const fn mrxfifo(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Master Receive FIFO Size"]
    #[inline(always)]
    pub const fn set_mrxfifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
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
            .field("mtxfifo", &self.mtxfifo())
            .field("mrxfifo", &self.mrxfifo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ mtxfifo: {=u8:?}, mrxfifo: {=u8:?} }}",
            self.mtxfifo(),
            self.mrxfifo()
        )
    }
}
#[doc = "Slave Address Match"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Samr(pub u32);
impl Samr {
    #[doc = "Address 0 Value"]
    #[must_use]
    #[inline(always)]
    pub const fn addr0(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[doc = "Address 0 Value"]
    #[inline(always)]
    pub const fn set_addr0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
    #[doc = "Address 1 Value"]
    #[must_use]
    #[inline(always)]
    pub const fn addr1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x03ff;
        val as u16
    }
    #[doc = "Address 1 Value"]
    #[inline(always)]
    pub const fn set_addr1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 17usize)) | (((val as u32) & 0x03ff) << 17usize);
    }
}
impl Default for Samr {
    #[inline(always)]
    fn default() -> Samr {
        Samr(0)
    }
}
impl core::fmt::Debug for Samr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Samr")
            .field("addr0", &self.addr0())
            .field("addr1", &self.addr1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Samr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Samr {{ addr0: {=u16:?}, addr1: {=u16:?} }}",
            self.addr0(),
            self.addr1()
        )
    }
}
#[doc = "Slave Address Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sasr(pub u32);
impl Sasr {
    #[doc = "Received Address"]
    #[must_use]
    #[inline(always)]
    pub const fn raddr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Received Address"]
    #[inline(always)]
    pub const fn set_raddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Address Not Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn anv(&self) -> super::vals::Anv {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Anv::from_bits(val as u8)
    }
    #[doc = "Address Not Valid"]
    #[inline(always)]
    pub const fn set_anv(&mut self, val: super::vals::Anv) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
}
impl Default for Sasr {
    #[inline(always)]
    fn default() -> Sasr {
        Sasr(0)
    }
}
impl core::fmt::Debug for Sasr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sasr")
            .field("raddr", &self.raddr())
            .field("anv", &self.anv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sasr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sasr {{ raddr: {=u16:?}, anv: {:?} }}",
            self.raddr(),
            self.anv()
        )
    }
}
#[doc = "Slave Configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scfgr1(pub u32);
impl Scfgr1 {
    #[doc = "Address SCL Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn adrstall(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Address SCL Stall"]
    #[inline(always)]
    pub const fn set_adrstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX SCL Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn rxstall(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX SCL Stall"]
    #[inline(always)]
    pub const fn set_rxstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TX Data SCL Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn txdstall(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TX Data SCL Stall"]
    #[inline(always)]
    pub const fn set_txdstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ACK SCL Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn ackstall(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ACK SCL Stall"]
    #[inline(always)]
    pub const fn set_ackstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "General Call Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gcen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "General Call Enable"]
    #[inline(always)]
    pub const fn set_gcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SMBus Alert Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn saen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SMBus Alert Enable"]
    #[inline(always)]
    pub const fn set_saen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmit Flag Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn txcfg(&self) -> super::vals::Txcfg {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Txcfg::from_bits(val as u8)
    }
    #[doc = "Transmit Flag Configuration"]
    #[inline(always)]
    pub const fn set_txcfg(&mut self, val: super::vals::Txcfg) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Receive Data Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn rxcfg(&self) -> super::vals::Rxcfg {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Rxcfg::from_bits(val as u8)
    }
    #[doc = "Receive Data Configuration"]
    #[inline(always)]
    pub const fn set_rxcfg(&mut self, val: super::vals::Rxcfg) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Ignore NACK"]
    #[must_use]
    #[inline(always)]
    pub const fn ignack(&self) -> super::vals::Scfgr1Ignack {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Scfgr1Ignack::from_bits(val as u8)
    }
    #[doc = "Ignore NACK"]
    #[inline(always)]
    pub const fn set_ignack(&mut self, val: super::vals::Scfgr1Ignack) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "High Speed Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hsmen(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "High Speed Mode Enable"]
    #[inline(always)]
    pub const fn set_hsmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Address Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn addrcfg(&self) -> super::vals::Addrcfg {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Addrcfg::from_bits(val as u8)
    }
    #[doc = "Address Configuration"]
    #[inline(always)]
    pub const fn set_addrcfg(&mut self, val: super::vals::Addrcfg) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
}
impl Default for Scfgr1 {
    #[inline(always)]
    fn default() -> Scfgr1 {
        Scfgr1(0)
    }
}
impl core::fmt::Debug for Scfgr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scfgr1")
            .field("adrstall", &self.adrstall())
            .field("rxstall", &self.rxstall())
            .field("txdstall", &self.txdstall())
            .field("ackstall", &self.ackstall())
            .field("gcen", &self.gcen())
            .field("saen", &self.saen())
            .field("txcfg", &self.txcfg())
            .field("rxcfg", &self.rxcfg())
            .field("ignack", &self.ignack())
            .field("hsmen", &self.hsmen())
            .field("addrcfg", &self.addrcfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scfgr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scfgr1 {{ adrstall: {=bool:?}, rxstall: {=bool:?}, txdstall: {=bool:?}, ackstall: {=bool:?}, gcen: {=bool:?}, saen: {=bool:?}, txcfg: {:?}, rxcfg: {:?}, ignack: {:?}, hsmen: {=bool:?}, addrcfg: {:?} }}",
            self.adrstall(),
            self.rxstall(),
            self.txdstall(),
            self.ackstall(),
            self.gcen(),
            self.saen(),
            self.txcfg(),
            self.rxcfg(),
            self.ignack(),
            self.hsmen(),
            self.addrcfg()
        )
    }
}
#[doc = "Slave Configuration 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scfgr2(pub u32);
impl Scfgr2 {
    #[doc = "Clock Hold Time"]
    #[must_use]
    #[inline(always)]
    pub const fn clkhold(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock Hold Time"]
    #[inline(always)]
    pub const fn set_clkhold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Data Valid Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn datavd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Data Valid Delay"]
    #[inline(always)]
    pub const fn set_datavd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Glitch Filter SCL"]
    #[must_use]
    #[inline(always)]
    pub const fn filtscl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Glitch Filter SCL"]
    #[inline(always)]
    pub const fn set_filtscl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Glitch Filter SDA"]
    #[must_use]
    #[inline(always)]
    pub const fn filtsda(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Glitch Filter SDA"]
    #[inline(always)]
    pub const fn set_filtsda(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Scfgr2 {
    #[inline(always)]
    fn default() -> Scfgr2 {
        Scfgr2(0)
    }
}
impl core::fmt::Debug for Scfgr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scfgr2")
            .field("clkhold", &self.clkhold())
            .field("datavd", &self.datavd())
            .field("filtscl", &self.filtscl())
            .field("filtsda", &self.filtsda())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scfgr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scfgr2 {{ clkhold: {=u8:?}, datavd: {=u8:?}, filtscl: {=u8:?}, filtsda: {=u8:?} }}",
            self.clkhold(),
            self.datavd(),
            self.filtscl(),
            self.filtsda()
        )
    }
}
#[doc = "Slave Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc = "Slave Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Enable"]
    #[inline(always)]
    pub const fn set_sen(&mut self, val: bool) {
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
    #[doc = "Filter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn filten(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Filter Enable"]
    #[inline(always)]
    pub const fn set_filten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Filter Doze Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn filtdz(&self) -> super::vals::Filtdz {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Filtdz::from_bits(val as u8)
    }
    #[doc = "Filter Doze Enable"]
    #[inline(always)]
    pub const fn set_filtdz(&mut self, val: super::vals::Filtdz) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Reset Transmit FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn rtf(&self) -> super::vals::ScrRtf {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::ScrRtf::from_bits(val as u8)
    }
    #[doc = "Reset Transmit FIFO"]
    #[inline(always)]
    pub const fn set_rtf(&mut self, val: super::vals::ScrRtf) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Reset Receive FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn rrf(&self) -> super::vals::ScrRrf {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::ScrRrf::from_bits(val as u8)
    }
    #[doc = "Reset Receive FIFO"]
    #[inline(always)]
    pub const fn set_rrf(&mut self, val: super::vals::ScrRrf) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        Scr(0)
    }
}
impl core::fmt::Debug for Scr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr")
            .field("sen", &self.sen())
            .field("rst", &self.rst())
            .field("filten", &self.filten())
            .field("filtdz", &self.filtdz())
            .field("rtf", &self.rtf())
            .field("rrf", &self.rrf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr {{ sen: {=bool:?}, rst: {=bool:?}, filten: {=bool:?}, filtdz: {:?}, rtf: {:?}, rrf: {:?} }}",
            self.sen(),
            self.rst(),
            self.filten(),
            self.filtdz(),
            self.rtf(),
            self.rrf()
        )
    }
}
#[doc = "Slave DMA Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sder(pub u32);
impl Sder {
    #[doc = "Transmit Data DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data DMA Enable"]
    #[inline(always)]
    pub const fn set_tdde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rdde(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data DMA Enable"]
    #[inline(always)]
    pub const fn set_rdde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Address Valid DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn avde(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Address Valid DMA Enable"]
    #[inline(always)]
    pub const fn set_avde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Sder {
    #[inline(always)]
    fn default() -> Sder {
        Sder(0)
    }
}
impl core::fmt::Debug for Sder {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sder")
            .field("tdde", &self.tdde())
            .field("rdde", &self.rdde())
            .field("avde", &self.avde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sder {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sder {{ tdde: {=bool:?}, rdde: {=bool:?}, avde: {=bool:?} }}",
            self.tdde(),
            self.rdde(),
            self.avde()
        )
    }
}
#[doc = "Slave Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sier(pub u32);
impl Sier {
    #[doc = "Transmit Data Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rdie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Address Valid Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn avie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Address Valid Interrupt Enable"]
    #[inline(always)]
    pub const fn set_avie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit ACK Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn taie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit ACK Interrupt Enable"]
    #[inline(always)]
    pub const fn set_taie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Repeated Start Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rsie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Repeated Start Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "STOP Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sdie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "STOP Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn beie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_beie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "FIFO Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn feie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_feie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Address Match 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn am0ie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Address Match 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_am0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Address Match 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn am1ie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Address Match 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_am1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "General Call Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gcie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "General Call Interrupt Enable"]
    #[inline(always)]
    pub const fn set_gcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SMBus Alert Response Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sarie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SMBus Alert Response Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sarie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Sier {
    #[inline(always)]
    fn default() -> Sier {
        Sier(0)
    }
}
impl core::fmt::Debug for Sier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sier")
            .field("tdie", &self.tdie())
            .field("rdie", &self.rdie())
            .field("avie", &self.avie())
            .field("taie", &self.taie())
            .field("rsie", &self.rsie())
            .field("sdie", &self.sdie())
            .field("beie", &self.beie())
            .field("feie", &self.feie())
            .field("am0ie", &self.am0ie())
            .field("am1ie", &self.am1ie())
            .field("gcie", &self.gcie())
            .field("sarie", &self.sarie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sier {{ tdie: {=bool:?}, rdie: {=bool:?}, avie: {=bool:?}, taie: {=bool:?}, rsie: {=bool:?}, sdie: {=bool:?}, beie: {=bool:?}, feie: {=bool:?}, am0ie: {=bool:?}, am1ie: {=bool:?}, gcie: {=bool:?}, sarie: {=bool:?} }}",
            self.tdie(),
            self.rdie(),
            self.avie(),
            self.taie(),
            self.rsie(),
            self.sdie(),
            self.beie(),
            self.feie(),
            self.am0ie(),
            self.am1ie(),
            self.gcie(),
            self.sarie()
        )
    }
}
#[doc = "Slave Receive Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srdr(pub u32);
impl Srdr {
    #[doc = "Receive Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn rxempty(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Empty"]
    #[inline(always)]
    pub const fn set_rxempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Start Of Frame"]
    #[must_use]
    #[inline(always)]
    pub const fn sof(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Start Of Frame"]
    #[inline(always)]
    pub const fn set_sof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Srdr {
    #[inline(always)]
    fn default() -> Srdr {
        Srdr(0)
    }
}
impl core::fmt::Debug for Srdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srdr")
            .field("data", &self.data())
            .field("rxempty", &self.rxempty())
            .field("sof", &self.sof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Srdr {{ data: {=u8:?}, rxempty: {=bool:?}, sof: {=bool:?} }}",
            self.data(),
            self.rxempty(),
            self.sof()
        )
    }
}
#[doc = "Slave Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssr(pub u32);
impl Ssr {
    #[doc = "Transmit Data Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tdf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Flag"]
    #[inline(always)]
    pub const fn set_tdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rdf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Flag"]
    #[inline(always)]
    pub const fn set_rdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Address Valid Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn avf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Address Valid Flag"]
    #[inline(always)]
    pub const fn set_avf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit ACK Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn taf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit ACK Flag"]
    #[inline(always)]
    pub const fn set_taf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Repeated Start Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rsf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Repeated Start Flag"]
    #[inline(always)]
    pub const fn set_rsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "STOP Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sdf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "STOP Detect Flag"]
    #[inline(always)]
    pub const fn set_sdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bef(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Error Flag"]
    #[inline(always)]
    pub const fn set_bef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "FIFO Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fef(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Flag"]
    #[inline(always)]
    pub const fn set_fef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Address Match 0 Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn am0f(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Address Match 0 Flag"]
    #[inline(always)]
    pub const fn set_am0f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Address Match 1 Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn am1f(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Address Match 1 Flag"]
    #[inline(always)]
    pub const fn set_am1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "General Call Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn gcf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "General Call Flag"]
    #[inline(always)]
    pub const fn set_gcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SMBus Alert Response Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sarf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SMBus Alert Response Flag"]
    #[inline(always)]
    pub const fn set_sarf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Slave Busy Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sbf(&self) -> super::vals::Sbf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Sbf::from_bits(val as u8)
    }
    #[doc = "Slave Busy Flag"]
    #[inline(always)]
    pub const fn set_sbf(&mut self, val: super::vals::Sbf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus Busy Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bbf(&self) -> super::vals::SsrBbf {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::SsrBbf::from_bits(val as u8)
    }
    #[doc = "Bus Busy Flag"]
    #[inline(always)]
    pub const fn set_bbf(&mut self, val: super::vals::SsrBbf) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Ssr {
    #[inline(always)]
    fn default() -> Ssr {
        Ssr(0)
    }
}
impl core::fmt::Debug for Ssr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssr")
            .field("tdf", &self.tdf())
            .field("rdf", &self.rdf())
            .field("avf", &self.avf())
            .field("taf", &self.taf())
            .field("rsf", &self.rsf())
            .field("sdf", &self.sdf())
            .field("bef", &self.bef())
            .field("fef", &self.fef())
            .field("am0f", &self.am0f())
            .field("am1f", &self.am1f())
            .field("gcf", &self.gcf())
            .field("sarf", &self.sarf())
            .field("sbf", &self.sbf())
            .field("bbf", &self.bbf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssr {{ tdf: {=bool:?}, rdf: {=bool:?}, avf: {=bool:?}, taf: {=bool:?}, rsf: {=bool:?}, sdf: {=bool:?}, bef: {=bool:?}, fef: {=bool:?}, am0f: {=bool:?}, am1f: {=bool:?}, gcf: {=bool:?}, sarf: {=bool:?}, sbf: {:?}, bbf: {:?} }}",
            self.tdf(),
            self.rdf(),
            self.avf(),
            self.taf(),
            self.rsf(),
            self.sdf(),
            self.bef(),
            self.fef(),
            self.am0f(),
            self.am1f(),
            self.gcf(),
            self.sarf(),
            self.sbf(),
            self.bbf()
        )
    }
}
#[doc = "Slave Transmit ACK"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Star(pub u32);
impl Star {
    #[doc = "Transmit NACK"]
    #[must_use]
    #[inline(always)]
    pub const fn txnack(&self) -> super::vals::Txnack {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Txnack::from_bits(val as u8)
    }
    #[doc = "Transmit NACK"]
    #[inline(always)]
    pub const fn set_txnack(&mut self, val: super::vals::Txnack) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Star {
    #[inline(always)]
    fn default() -> Star {
        Star(0)
    }
}
impl core::fmt::Debug for Star {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Star")
            .field("txnack", &self.txnack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Star {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Star {{ txnack: {:?} }}", self.txnack())
    }
}
#[doc = "Slave Transmit Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stdr(pub u32);
impl Stdr {
    #[doc = "Transmit Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Stdr {
    #[inline(always)]
    fn default() -> Stdr {
        Stdr(0)
    }
}
impl core::fmt::Debug for Stdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stdr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Stdr {{ data: {=u8:?} }}", self.data())
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: super::vals::Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
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
            .field("feature", &self.feature())
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
            "Verid {{ feature: {:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
