#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u16);
impl Ctrl {
    #[doc = "Load Okay"]
    #[must_use]
    #[inline(always)]
    pub const fn ldok(&self) -> super::vals::Ldok {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ldok::from_bits(val as u8)
    }
    #[doc = "Load Okay"]
    #[inline(always)]
    pub const fn set_ldok(&mut self, val: super::vals::Ldok) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> super::vals::Dmaen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dmaen::from_bits(val as u8)
    }
    #[doc = "DMA Enable"]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: super::vals::Dmaen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Watchdog Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wde(&self) -> super::vals::Wde {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Wde::from_bits(val as u8)
    }
    #[doc = "Watchdog Enable"]
    #[inline(always)]
    pub const fn set_wde(&mut self, val: super::vals::Wde) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Watchdog Timeout Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wdie(&self) -> super::vals::Wdie {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Wdie::from_bits(val as u8)
    }
    #[doc = "Watchdog Timeout Interrupt Enable"]
    #[inline(always)]
    pub const fn set_wdie(&mut self, val: super::vals::Wdie) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Watchdog Timeout Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn wdirq(&self) -> super::vals::Wdirq {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Wdirq::from_bits(val as u8)
    }
    #[doc = "Watchdog Timeout Interrupt Request"]
    #[inline(always)]
    pub const fn set_wdirq(&mut self, val: super::vals::Wdirq) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Select Positive/Negative Edge of INDEX/PRESET Pulse"]
    #[must_use]
    #[inline(always)]
    pub const fn xne(&self) -> super::vals::Xne {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Xne::from_bits(val as u8)
    }
    #[doc = "Select Positive/Negative Edge of INDEX/PRESET Pulse"]
    #[inline(always)]
    pub const fn set_xne(&mut self, val: super::vals::Xne) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "INDEX Triggered Initialization of Position Counters UPOS and LPOS"]
    #[must_use]
    #[inline(always)]
    pub const fn xip(&self) -> super::vals::Xip {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Xip::from_bits(val as u8)
    }
    #[doc = "INDEX Triggered Initialization of Position Counters UPOS and LPOS"]
    #[inline(always)]
    pub const fn set_xip(&mut self, val: super::vals::Xip) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "INDEX/PRESET Pulse Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn xie(&self) -> super::vals::Xie {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Xie::from_bits(val as u8)
    }
    #[doc = "INDEX/PRESET Pulse Interrupt Enable"]
    #[inline(always)]
    pub const fn set_xie(&mut self, val: super::vals::Xie) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "INDEX/PRESET Pulse Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn xirq(&self) -> super::vals::Xirq {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Xirq::from_bits(val as u8)
    }
    #[doc = "INDEX/PRESET Pulse Interrupt Request"]
    #[inline(always)]
    pub const fn set_xirq(&mut self, val: super::vals::Xirq) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Enable Single Phase Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ph1(&self) -> super::vals::Ph1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Ph1::from_bits(val as u8)
    }
    #[doc = "Enable Single Phase Mode"]
    #[inline(always)]
    pub const fn set_ph1(&mut self, val: super::vals::Ph1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Enable Reverse Direction Counting"]
    #[must_use]
    #[inline(always)]
    pub const fn rev(&self) -> super::vals::Rev {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Rev::from_bits(val as u8)
    }
    #[doc = "Enable Reverse Direction Counting"]
    #[inline(always)]
    pub const fn set_rev(&mut self, val: super::vals::Rev) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Software-Triggered Initialization of Position Counters UPOS and LPOS"]
    #[must_use]
    #[inline(always)]
    pub const fn swip(&self) -> super::vals::Swip {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Swip::from_bits(val as u8)
    }
    #[doc = "Software-Triggered Initialization of Position Counters UPOS and LPOS"]
    #[inline(always)]
    pub const fn set_swip(&mut self, val: super::vals::Swip) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Use Negative Edge of HOME/ENABLE Input"]
    #[must_use]
    #[inline(always)]
    pub const fn hne(&self) -> super::vals::Hne {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Hne::from_bits(val as u8)
    }
    #[doc = "Use Negative Edge of HOME/ENABLE Input"]
    #[inline(always)]
    pub const fn set_hne(&mut self, val: super::vals::Hne) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Enable HOME to Initialize Position Counter UPOS/LPOS"]
    #[must_use]
    #[inline(always)]
    pub const fn hip(&self) -> super::vals::Hip {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Hip::from_bits(val as u8)
    }
    #[doc = "Enable HOME to Initialize Position Counter UPOS/LPOS"]
    #[inline(always)]
    pub const fn set_hip(&mut self, val: super::vals::Hip) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "HOME/ENABLE Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hie(&self) -> super::vals::Hie {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Hie::from_bits(val as u8)
    }
    #[doc = "HOME/ENABLE Interrupt Enable"]
    #[inline(always)]
    pub const fn set_hie(&mut self, val: super::vals::Hie) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "HOME/ENABLE Signal Transition Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn hirq(&self) -> super::vals::Hirq {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Hirq::from_bits(val as u8)
    }
    #[doc = "HOME/ENABLE Signal Transition Interrupt Request"]
    #[inline(always)]
    pub const fn set_hirq(&mut self, val: super::vals::Hirq) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            .field("ldok", &self.ldok())
            .field("dmaen", &self.dmaen())
            .field("wde", &self.wde())
            .field("wdie", &self.wdie())
            .field("wdirq", &self.wdirq())
            .field("xne", &self.xne())
            .field("xip", &self.xip())
            .field("xie", &self.xie())
            .field("xirq", &self.xirq())
            .field("ph1", &self.ph1())
            .field("rev", &self.rev())
            .field("swip", &self.swip())
            .field("hne", &self.hne())
            .field("hip", &self.hip())
            .field("hie", &self.hie())
            .field("hirq", &self.hirq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ ldok: {:?}, dmaen: {:?}, wde: {:?}, wdie: {:?}, wdirq: {:?}, xne: {:?}, xip: {:?}, xie: {:?}, xirq: {:?}, ph1: {:?}, rev: {:?}, swip: {:?}, hne: {:?}, hip: {:?}, hie: {:?}, hirq: {:?} }}",
            self.ldok(),
            self.dmaen(),
            self.wde(),
            self.wdie(),
            self.wdirq(),
            self.xne(),
            self.xip(),
            self.xie(),
            self.xirq(),
            self.ph1(),
            self.rev(),
            self.swip(),
            self.hne(),
            self.hip(),
            self.hie(),
            self.hirq()
        )
    }
}
#[doc = "Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2(pub u16);
impl Ctrl2 {
    #[doc = "Update Hold Registers"]
    #[must_use]
    #[inline(always)]
    pub const fn updhld(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update Hold Registers"]
    #[inline(always)]
    pub const fn set_updhld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Update Position Registers"]
    #[must_use]
    #[inline(always)]
    pub const fn updpos(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Update Position Registers"]
    #[inline(always)]
    pub const fn set_updpos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Operation Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn opmode(&self) -> super::vals::Opmode {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Opmode::from_bits(val as u8)
    }
    #[doc = "Operation Mode Select"]
    #[inline(always)]
    pub const fn set_opmode(&mut self, val: super::vals::Opmode) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Buffered Register Load (Update) Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> super::vals::Ldmod {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ldmod::from_bits(val as u8)
    }
    #[doc = "Buffered Register Load (Update) Mode Select"]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: super::vals::Ldmod) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Revolution Counter Modulus Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn revmod(&self) -> super::vals::Revmod {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Revmod::from_bits(val as u8)
    }
    #[doc = "Revolution Counter Modulus Enable"]
    #[inline(always)]
    pub const fn set_revmod(&mut self, val: super::vals::Revmod) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Output Control"]
    #[must_use]
    #[inline(always)]
    pub const fn outctl(&self) -> super::vals::Outctl {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Outctl::from_bits(val as u8)
    }
    #[doc = "Output Control"]
    #[inline(always)]
    pub const fn set_outctl(&mut self, val: super::vals::Outctl) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Period measurement function enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pmen(&self) -> super::vals::Pmen {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pmen::from_bits(val as u8)
    }
    #[doc = "Period measurement function enable"]
    #[inline(always)]
    pub const fn set_pmen(&mut self, val: super::vals::Pmen) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Enables/disables the position counter to be initialized by Index Event Edge Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn emip(&self) -> super::vals::Emip {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Emip::from_bits(val as u8)
    }
    #[doc = "Enables/disables the position counter to be initialized by Index Event Edge Mark"]
    #[inline(always)]
    pub const fn set_emip(&mut self, val: super::vals::Emip) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Initial Position Register"]
    #[must_use]
    #[inline(always)]
    pub const fn initpos(&self) -> super::vals::Initpos {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Initpos::from_bits(val as u8)
    }
    #[doc = "Initial Position Register"]
    #[inline(always)]
    pub const fn set_initpos(&mut self, val: super::vals::Initpos) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Count Once"]
    #[must_use]
    #[inline(always)]
    pub const fn once(&self) -> super::vals::Once {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Once::from_bits(val as u8)
    }
    #[doc = "Count Once"]
    #[inline(always)]
    pub const fn set_once(&mut self, val: super::vals::Once) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Counting Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cmode(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Counting Mode"]
    #[inline(always)]
    pub const fn set_cmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
    }
}
impl Default for Ctrl2 {
    #[inline(always)]
    fn default() -> Ctrl2 {
        Ctrl2(0)
    }
}
impl core::fmt::Debug for Ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2")
            .field("updhld", &self.updhld())
            .field("updpos", &self.updpos())
            .field("opmode", &self.opmode())
            .field("ldmod", &self.ldmod())
            .field("revmod", &self.revmod())
            .field("outctl", &self.outctl())
            .field("pmen", &self.pmen())
            .field("emip", &self.emip())
            .field("initpos", &self.initpos())
            .field("once", &self.once())
            .field("cmode", &self.cmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl2 {{ updhld: {=bool:?}, updpos: {=bool:?}, opmode: {:?}, ldmod: {:?}, revmod: {:?}, outctl: {:?}, pmen: {:?}, emip: {:?}, initpos: {:?}, once: {:?}, cmode: {=u8:?} }}",
            self.updhld(),
            self.updpos(),
            self.opmode(),
            self.ldmod(),
            self.revmod(),
            self.outctl(),
            self.pmen(),
            self.emip(),
            self.initpos(),
            self.once(),
            self.cmode()
        )
    }
}
#[doc = "Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Filt(pub u16);
impl Filt {
    #[doc = "Input Filter Sample Period"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Filter Sample Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Filter Sample Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Filter Sample Count"]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
    #[doc = "Filter Clock Source selection"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cs(&self) -> super::vals::FiltCs {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::FiltCs::from_bits(val as u8)
    }
    #[doc = "Filter Clock Source selection"]
    #[inline(always)]
    pub const fn set_filt_cs(&mut self, val: super::vals::FiltCs) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Prescaler"]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
    }
}
impl Default for Filt {
    #[inline(always)]
    fn default() -> Filt {
        Filt(0)
    }
}
impl core::fmt::Debug for Filt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Filt")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .field("filt_cs", &self.filt_cs())
            .field("prsc", &self.prsc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Filt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Filt {{ filt_per: {=u8:?}, filt_cnt: {=u8:?}, filt_cs: {:?}, prsc: {=u8:?} }}",
            self.filt_per(),
            self.filt_cnt(),
            self.filt_cs(),
            self.prsc()
        )
    }
}
#[doc = "Input Monitor Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr(pub u16);
impl Imr {
    #[doc = "HOME_ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn home_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HOME_ENABLE"]
    #[inline(always)]
    pub const fn set_home_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "INDEX_PRESET"]
    #[must_use]
    #[inline(always)]
    pub const fn index_preset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "INDEX_PRESET"]
    #[inline(always)]
    pub const fn set_index_preset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "PHB"]
    #[must_use]
    #[inline(always)]
    pub const fn phb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PHB"]
    #[inline(always)]
    pub const fn set_phb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "PHA"]
    #[must_use]
    #[inline(always)]
    pub const fn pha(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PHA"]
    #[inline(always)]
    pub const fn set_pha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "filter operation on HOME/ENABLE input"]
    #[must_use]
    #[inline(always)]
    pub const fn fhom_ena(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "filter operation on HOME/ENABLE input"]
    #[inline(always)]
    pub const fn set_fhom_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "filter operation on INDEX/PRESET input"]
    #[must_use]
    #[inline(always)]
    pub const fn find_pre(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "filter operation on INDEX/PRESET input"]
    #[inline(always)]
    pub const fn set_find_pre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "filter operation on PHASEB input"]
    #[must_use]
    #[inline(always)]
    pub const fn fphb(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "filter operation on PHASEB input"]
    #[inline(always)]
    pub const fn set_fphb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "filter operation on PHASEA input"]
    #[must_use]
    #[inline(always)]
    pub const fn fpha(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "filter operation on PHASEA input"]
    #[inline(always)]
    pub const fn set_fpha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Position Compare 0 Flag Output"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf0(&self) -> super::vals::Cmpf0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cmpf0::from_bits(val as u8)
    }
    #[doc = "Position Compare 0 Flag Output"]
    #[inline(always)]
    pub const fn set_cmpf0(&mut self, val: super::vals::Cmpf0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Position Compare1 Flag Output"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1f(&self) -> super::vals::Cmp1f {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Cmp1f::from_bits(val as u8)
    }
    #[doc = "Position Compare1 Flag Output"]
    #[inline(always)]
    pub const fn set_cmp1f(&mut self, val: super::vals::Cmp1f) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Position Compare2 Flag Output"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2f(&self) -> super::vals::Cmp2f {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Cmp2f::from_bits(val as u8)
    }
    #[doc = "Position Compare2 Flag Output"]
    #[inline(always)]
    pub const fn set_cmp2f(&mut self, val: super::vals::Cmp2f) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Position Compare3 Flag Output"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp3f(&self) -> super::vals::Cmp3f {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Cmp3f::from_bits(val as u8)
    }
    #[doc = "Position Compare3 Flag Output"]
    #[inline(always)]
    pub const fn set_cmp3f(&mut self, val: super::vals::Cmp3f) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Count Direction Flag Hold"]
    #[must_use]
    #[inline(always)]
    pub const fn dirh(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Count Direction Flag Hold"]
    #[inline(always)]
    pub const fn set_dirh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Count Direction Flag Output"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::Dir {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dir::from_bits(val as u8)
    }
    #[doc = "Count Direction Flag Output"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: super::vals::Dir) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Imr {
    #[inline(always)]
    fn default() -> Imr {
        Imr(0)
    }
}
impl core::fmt::Debug for Imr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imr")
            .field("home_enable", &self.home_enable())
            .field("index_preset", &self.index_preset())
            .field("phb", &self.phb())
            .field("pha", &self.pha())
            .field("fhom_ena", &self.fhom_ena())
            .field("find_pre", &self.find_pre())
            .field("fphb", &self.fphb())
            .field("fpha", &self.fpha())
            .field("cmpf0", &self.cmpf0())
            .field("cmp1f", &self.cmp1f())
            .field("cmp2f", &self.cmp2f())
            .field("cmp3f", &self.cmp3f())
            .field("dirh", &self.dirh())
            .field("dir", &self.dir())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Imr {{ home_enable: {=bool:?}, index_preset: {=bool:?}, phb: {=bool:?}, pha: {=bool:?}, fhom_ena: {=bool:?}, find_pre: {=bool:?}, fphb: {=bool:?}, fpha: {=bool:?}, cmpf0: {:?}, cmp1f: {:?}, cmp2f: {:?}, cmp3f: {:?}, dirh: {=bool:?}, dir: {:?} }}",
            self.home_enable(),
            self.index_preset(),
            self.phb(),
            self.pha(),
            self.fhom_ena(),
            self.find_pre(),
            self.fphb(),
            self.fpha(),
            self.cmpf0(),
            self.cmp1f(),
            self.cmp2f(),
            self.cmp3f(),
            self.dirh(),
            self.dir()
        )
    }
}
#[doc = "Interrupt Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intctrl(pub u16);
impl Intctrl {
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sabie(&self) -> super::vals::Sabie {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sabie::from_bits(val as u8)
    }
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sabie(&mut self, val: super::vals::Sabie) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn sabirq(&self) -> super::vals::Sabirq {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sabirq::from_bits(val as u8)
    }
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Request"]
    #[inline(always)]
    pub const fn set_sabirq(&mut self, val: super::vals::Sabirq) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Count direction change interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dirie(&self) -> super::vals::Dirie {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dirie::from_bits(val as u8)
    }
    #[doc = "Count direction change interrupt enable"]
    #[inline(always)]
    pub const fn set_dirie(&mut self, val: super::vals::Dirie) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Count direction change interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn dirirq(&self) -> super::vals::Dirirq {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dirirq::from_bits(val as u8)
    }
    #[doc = "Count direction change interrupt"]
    #[inline(always)]
    pub const fn set_dirirq(&mut self, val: super::vals::Dirirq) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Roll-under Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ruie(&self) -> super::vals::Ruie {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ruie::from_bits(val as u8)
    }
    #[doc = "Roll-under Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ruie(&mut self, val: super::vals::Ruie) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Roll-under Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn ruirq(&self) -> super::vals::Ruirq {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ruirq::from_bits(val as u8)
    }
    #[doc = "Roll-under Interrupt Request"]
    #[inline(always)]
    pub const fn set_ruirq(&mut self, val: super::vals::Ruirq) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Roll-over Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn roie(&self) -> super::vals::Roie {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Roie::from_bits(val as u8)
    }
    #[doc = "Roll-over Interrupt Enable"]
    #[inline(always)]
    pub const fn set_roie(&mut self, val: super::vals::Roie) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Roll-over Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn roirq(&self) -> super::vals::Roirq {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Roirq::from_bits(val as u8)
    }
    #[doc = "Roll-over Interrupt Request"]
    #[inline(always)]
    pub const fn set_roirq(&mut self, val: super::vals::Roirq) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Compare 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0ie(&self) -> super::vals::Cmp0ie {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cmp0ie::from_bits(val as u8)
    }
    #[doc = "Compare 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cmp0ie(&mut self, val: super::vals::Cmp0ie) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Compare 0 Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0irq(&self) -> super::vals::Cmp0irq {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Cmp0irq::from_bits(val as u8)
    }
    #[doc = "Compare 0 Interrupt Request"]
    #[inline(always)]
    pub const fn set_cmp0irq(&mut self, val: super::vals::Cmp0irq) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Compare1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1ie(&self) -> super::vals::Cmp1ie {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Cmp1ie::from_bits(val as u8)
    }
    #[doc = "Compare1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cmp1ie(&mut self, val: super::vals::Cmp1ie) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Compare1 Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1irq(&self) -> super::vals::Cmp1irq {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Cmp1irq::from_bits(val as u8)
    }
    #[doc = "Compare1 Interrupt Request"]
    #[inline(always)]
    pub const fn set_cmp1irq(&mut self, val: super::vals::Cmp1irq) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Compare2 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2ie(&self) -> super::vals::Cmp2ie {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Cmp2ie::from_bits(val as u8)
    }
    #[doc = "Compare2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cmp2ie(&mut self, val: super::vals::Cmp2ie) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Compare2 Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2irq(&self) -> super::vals::Cmp2irq {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cmp2irq::from_bits(val as u8)
    }
    #[doc = "Compare2 Interrupt Request"]
    #[inline(always)]
    pub const fn set_cmp2irq(&mut self, val: super::vals::Cmp2irq) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Compare3 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp3ie(&self) -> super::vals::Cmp3ie {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Cmp3ie::from_bits(val as u8)
    }
    #[doc = "Compare3 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cmp3ie(&mut self, val: super::vals::Cmp3ie) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Compare3 Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp3irq(&self) -> super::vals::Cmp3irq {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Cmp3irq::from_bits(val as u8)
    }
    #[doc = "Compare3 Interrupt Request"]
    #[inline(always)]
    pub const fn set_cmp3irq(&mut self, val: super::vals::Cmp3irq) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Intctrl {
    #[inline(always)]
    fn default() -> Intctrl {
        Intctrl(0)
    }
}
impl core::fmt::Debug for Intctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intctrl")
            .field("sabie", &self.sabie())
            .field("sabirq", &self.sabirq())
            .field("dirie", &self.dirie())
            .field("dirirq", &self.dirirq())
            .field("ruie", &self.ruie())
            .field("ruirq", &self.ruirq())
            .field("roie", &self.roie())
            .field("roirq", &self.roirq())
            .field("cmp0ie", &self.cmp0ie())
            .field("cmp0irq", &self.cmp0irq())
            .field("cmp1ie", &self.cmp1ie())
            .field("cmp1irq", &self.cmp1irq())
            .field("cmp2ie", &self.cmp2ie())
            .field("cmp2irq", &self.cmp2irq())
            .field("cmp3ie", &self.cmp3ie())
            .field("cmp3irq", &self.cmp3irq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intctrl {{ sabie: {:?}, sabirq: {:?}, dirie: {:?}, dirirq: {:?}, ruie: {:?}, ruirq: {:?}, roie: {:?}, roirq: {:?}, cmp0ie: {:?}, cmp0irq: {:?}, cmp1ie: {:?}, cmp1irq: {:?}, cmp2ie: {:?}, cmp2irq: {:?}, cmp3ie: {:?}, cmp3irq: {:?} }}",
            self.sabie(),
            self.sabirq(),
            self.dirie(),
            self.dirirq(),
            self.ruie(),
            self.ruirq(),
            self.roie(),
            self.roirq(),
            self.cmp0ie(),
            self.cmp0irq(),
            self.cmp1ie(),
            self.cmp1irq(),
            self.cmp2ie(),
            self.cmp2irq(),
            self.cmp3ie(),
            self.cmp3irq()
        )
    }
}
#[doc = "Last Edge Time Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lastedge(pub u16);
impl Lastedge {
    #[doc = "Last Edge Time Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn lastedge(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Last Edge Time Counter"]
    #[inline(always)]
    pub const fn set_lastedge(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Lastedge {
    #[inline(always)]
    fn default() -> Lastedge {
        Lastedge(0)
    }
}
impl core::fmt::Debug for Lastedge {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lastedge")
            .field("lastedge", &self.lastedge())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lastedge {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lastedge {{ lastedge: {=u16:?} }}", self.lastedge())
    }
}
#[doc = "Last Edge Time Hold Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lastedgeh(pub u16);
impl Lastedgeh {
    #[doc = "Last Edge Time Hold"]
    #[must_use]
    #[inline(always)]
    pub const fn lastedgeh(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Last Edge Time Hold"]
    #[inline(always)]
    pub const fn set_lastedgeh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Lastedgeh {
    #[inline(always)]
    fn default() -> Lastedgeh {
        Lastedgeh(0)
    }
}
impl core::fmt::Debug for Lastedgeh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lastedgeh")
            .field("lastedgeh", &self.lastedgeh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lastedgeh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lastedgeh {{ lastedgeh: {=u16:?} }}", self.lastedgeh())
    }
}
#[doc = "Lower Position Compare Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcomp0(pub u16);
impl Lcomp0 {
    #[doc = "LCOMP0"]
    #[must_use]
    #[inline(always)]
    pub const fn lcomp0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "LCOMP0"]
    #[inline(always)]
    pub const fn set_lcomp0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Lcomp0 {
    #[inline(always)]
    fn default() -> Lcomp0 {
        Lcomp0(0)
    }
}
impl core::fmt::Debug for Lcomp0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lcomp0")
            .field("lcomp0", &self.lcomp0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lcomp0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lcomp0 {{ lcomp0: {=u16:?} }}", self.lcomp0())
    }
}
#[doc = "Lower Position Compare 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcomp1(pub u16);
impl Lcomp1 {
    #[doc = "LCOMP1"]
    #[must_use]
    #[inline(always)]
    pub const fn lcomp1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "LCOMP1"]
    #[inline(always)]
    pub const fn set_lcomp1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Lcomp1 {
    #[inline(always)]
    fn default() -> Lcomp1 {
        Lcomp1(0)
    }
}
impl core::fmt::Debug for Lcomp1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lcomp1")
            .field("lcomp1", &self.lcomp1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lcomp1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lcomp1 {{ lcomp1: {=u16:?} }}", self.lcomp1())
    }
}
#[doc = "Lower Position Compare 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcomp2(pub u16);
impl Lcomp2 {
    #[doc = "LCOMP2"]
    #[must_use]
    #[inline(always)]
    pub const fn lcomp2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "LCOMP2"]
    #[inline(always)]
    pub const fn set_lcomp2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Lcomp2 {
    #[inline(always)]
    fn default() -> Lcomp2 {
        Lcomp2(0)
    }
}
impl core::fmt::Debug for Lcomp2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lcomp2")
            .field("lcomp2", &self.lcomp2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lcomp2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lcomp2 {{ lcomp2: {=u16:?} }}", self.lcomp2())
    }
}
#[doc = "Lower Position Compare 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcomp3(pub u16);
impl Lcomp3 {
    #[doc = "LCOMP3"]
    #[must_use]
    #[inline(always)]
    pub const fn lcomp3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "LCOMP3"]
    #[inline(always)]
    pub const fn set_lcomp3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Lcomp3 {
    #[inline(always)]
    fn default() -> Lcomp3 {
        Lcomp3(0)
    }
}
impl core::fmt::Debug for Lcomp3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lcomp3")
            .field("lcomp3", &self.lcomp3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lcomp3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lcomp3 {{ lcomp3: {=u16:?} }}", self.lcomp3())
    }
}
#[doc = "Lower Initialization Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Linit(pub u16);
impl Linit {
    #[doc = "INIT"]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "INIT"]
    #[inline(always)]
    pub const fn set_init(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Linit {
    #[inline(always)]
    fn default() -> Linit {
        Linit(0)
    }
}
impl core::fmt::Debug for Linit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Linit").field("init", &self.init()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Linit {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Linit {{ init: {=u16:?} }}", self.init())
    }
}
#[doc = "Lower Modulus Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lmod(pub u16);
impl Lmod {
    #[doc = "MOD"]
    #[must_use]
    #[inline(always)]
    pub const fn mod_(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MOD"]
    #[inline(always)]
    pub const fn set_mod_(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Lmod {
    #[inline(always)]
    fn default() -> Lmod {
        Lmod(0)
    }
}
impl core::fmt::Debug for Lmod {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lmod").field("mod_", &self.mod_()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lmod {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lmod {{ mod_: {=u16:?} }}", self.mod_())
    }
}
#[doc = "Lower Position Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpos(pub u16);
impl Lpos {
    #[doc = "POS"]
    #[must_use]
    #[inline(always)]
    pub const fn pos(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "POS"]
    #[inline(always)]
    pub const fn set_pos(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Lpos {
    #[inline(always)]
    fn default() -> Lpos {
        Lpos(0)
    }
}
impl core::fmt::Debug for Lpos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpos").field("pos", &self.pos()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpos {{ pos: {=u16:?} }}", self.pos())
    }
}
#[doc = "Lower Position Hold Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lposh(pub u16);
impl Lposh {
    #[doc = "POSH"]
    #[must_use]
    #[inline(always)]
    pub const fn lposh(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "POSH"]
    #[inline(always)]
    pub const fn set_lposh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Lposh {
    #[inline(always)]
    fn default() -> Lposh {
        Lposh(0)
    }
}
impl core::fmt::Debug for Lposh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lposh")
            .field("lposh", &self.lposh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lposh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lposh {{ lposh: {=u16:?} }}", self.lposh())
    }
}
#[doc = "Lower Position Holder Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lposh1(pub u16);
impl Lposh1 {
    #[doc = "LPOSH1"]
    #[must_use]
    #[inline(always)]
    pub const fn lposh1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "LPOSH1"]
    #[inline(always)]
    pub const fn set_lposh1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Lposh1 {
    #[inline(always)]
    fn default() -> Lposh1 {
        Lposh1(0)
    }
}
impl core::fmt::Debug for Lposh1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lposh1")
            .field("lposh1", &self.lposh1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lposh1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lposh1 {{ lposh1: {=u16:?} }}", self.lposh1())
    }
}
#[doc = "Lower Position Holder Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lposh2(pub u16);
impl Lposh2 {
    #[doc = "LPOSH2"]
    #[must_use]
    #[inline(always)]
    pub const fn lposh2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "LPOSH2"]
    #[inline(always)]
    pub const fn set_lposh2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Lposh2 {
    #[inline(always)]
    fn default() -> Lposh2 {
        Lposh2(0)
    }
}
impl core::fmt::Debug for Lposh2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lposh2")
            .field("lposh2", &self.lposh2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lposh2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lposh2 {{ lposh2: {=u16:?} }}", self.lposh2())
    }
}
#[doc = "Lower Position Holder Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lposh3(pub u16);
impl Lposh3 {
    #[doc = "LPOSH3"]
    #[must_use]
    #[inline(always)]
    pub const fn lposh3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "LPOSH3"]
    #[inline(always)]
    pub const fn set_lposh3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Lposh3 {
    #[inline(always)]
    fn default() -> Lposh3 {
        Lposh3(0)
    }
}
impl core::fmt::Debug for Lposh3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lposh3")
            .field("lposh3", &self.lposh3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lposh3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lposh3 {{ lposh3: {=u16:?} }}", self.lposh3())
    }
}
#[doc = "Lower VERID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lverid(pub u16);
impl Lverid {
    #[doc = "LVERID"]
    #[must_use]
    #[inline(always)]
    pub const fn lverid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "LVERID"]
    #[inline(always)]
    pub const fn set_lverid(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Lverid {
    #[inline(always)]
    fn default() -> Lverid {
        Lverid(0)
    }
}
impl core::fmt::Debug for Lverid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lverid")
            .field("lverid", &self.lverid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lverid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lverid {{ lverid: {=u16:?} }}", self.lverid())
    }
}
#[doc = "Position Difference Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Posd(pub u16);
impl Posd {
    #[doc = "POSD"]
    #[must_use]
    #[inline(always)]
    pub const fn posd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "POSD"]
    #[inline(always)]
    pub const fn set_posd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Posd {
    #[inline(always)]
    fn default() -> Posd {
        Posd(0)
    }
}
impl core::fmt::Debug for Posd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Posd").field("posd", &self.posd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Posd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Posd {{ posd: {=u16:?} }}", self.posd())
    }
}
#[doc = "Position Difference Hold Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Posdh(pub u16);
impl Posdh {
    #[doc = "POSDH"]
    #[must_use]
    #[inline(always)]
    pub const fn posdh(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "POSDH"]
    #[inline(always)]
    pub const fn set_posdh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Posdh {
    #[inline(always)]
    fn default() -> Posdh {
        Posdh(0)
    }
}
impl core::fmt::Debug for Posdh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Posdh")
            .field("posdh", &self.posdh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Posdh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Posdh {{ posdh: {=u16:?} }}", self.posdh())
    }
}
#[doc = "Position Difference Period Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Posdper(pub u16);
impl Posdper {
    #[doc = "Position difference period"]
    #[must_use]
    #[inline(always)]
    pub const fn posdper(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Position difference period"]
    #[inline(always)]
    pub const fn set_posdper(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Posdper {
    #[inline(always)]
    fn default() -> Posdper {
        Posdper(0)
    }
}
impl core::fmt::Debug for Posdper {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Posdper")
            .field("posdper", &self.posdper())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Posdper {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Posdper {{ posdper: {=u16:?} }}", self.posdper())
    }
}
#[doc = "Position Difference Period Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Posdperbfr(pub u16);
impl Posdperbfr {
    #[doc = "Position difference period buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn posdperbfr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Position difference period buffer"]
    #[inline(always)]
    pub const fn set_posdperbfr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Posdperbfr {
    #[inline(always)]
    fn default() -> Posdperbfr {
        Posdperbfr(0)
    }
}
impl core::fmt::Debug for Posdperbfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Posdperbfr")
            .field("posdperbfr", &self.posdperbfr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Posdperbfr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Posdperbfr {{ posdperbfr: {=u16:?} }}",
            self.posdperbfr()
        )
    }
}
#[doc = "Position Difference Period Hold Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Posdperh(pub u16);
impl Posdperh {
    #[doc = "Position difference period hold"]
    #[must_use]
    #[inline(always)]
    pub const fn posdperh(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Position difference period hold"]
    #[inline(always)]
    pub const fn set_posdperh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Posdperh {
    #[inline(always)]
    fn default() -> Posdperh {
        Posdperh(0)
    }
}
impl core::fmt::Debug for Posdperh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Posdperh")
            .field("posdperh", &self.posdperh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Posdperh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Posdperh {{ posdperh: {=u16:?} }}", self.posdperh())
    }
}
#[doc = "Revolution Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rev(pub u16);
impl Rev {
    #[doc = "REV"]
    #[must_use]
    #[inline(always)]
    pub const fn rev(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "REV"]
    #[inline(always)]
    pub const fn set_rev(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Rev {
    #[inline(always)]
    fn default() -> Rev {
        Rev(0)
    }
}
impl core::fmt::Debug for Rev {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rev").field("rev", &self.rev()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rev {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rev {{ rev: {=u16:?} }}", self.rev())
    }
}
#[doc = "Revolution Hold Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Revh(pub u16);
impl Revh {
    #[doc = "REVH"]
    #[must_use]
    #[inline(always)]
    pub const fn revh(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "REVH"]
    #[inline(always)]
    pub const fn set_revh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Revh {
    #[inline(always)]
    fn default() -> Revh {
        Revh(0)
    }
}
impl core::fmt::Debug for Revh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Revh").field("revh", &self.revh()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Revh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Revh {{ revh: {=u16:?} }}", self.revh())
    }
}
#[doc = "Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tst(pub u16);
impl Tst {
    #[doc = "TEST_COUNT"]
    #[must_use]
    #[inline(always)]
    pub const fn test_count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "TEST_COUNT"]
    #[inline(always)]
    pub const fn set_test_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "TEST_PERIOD"]
    #[must_use]
    #[inline(always)]
    pub const fn test_period(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "TEST_PERIOD"]
    #[inline(always)]
    pub const fn set_test_period(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
    }
    #[doc = "Quadrature Decoder Negative Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn qdn(&self) -> super::vals::Qdn {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Qdn::from_bits(val as u8)
    }
    #[doc = "Quadrature Decoder Negative Signal"]
    #[inline(always)]
    pub const fn set_qdn(&mut self, val: super::vals::Qdn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Test Counter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tce(&self) -> super::vals::Tce {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Tce::from_bits(val as u8)
    }
    #[doc = "Test Counter Enable"]
    #[inline(always)]
    pub const fn set_tce(&mut self, val: super::vals::Tce) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Test Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ten(&self) -> super::vals::Ten {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Ten::from_bits(val as u8)
    }
    #[doc = "Test Mode Enable"]
    #[inline(always)]
    pub const fn set_ten(&mut self, val: super::vals::Ten) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Tst {
    #[inline(always)]
    fn default() -> Tst {
        Tst(0)
    }
}
impl core::fmt::Debug for Tst {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tst")
            .field("test_count", &self.test_count())
            .field("test_period", &self.test_period())
            .field("qdn", &self.qdn())
            .field("tce", &self.tce())
            .field("ten", &self.ten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tst {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tst {{ test_count: {=u8:?}, test_period: {=u8:?}, qdn: {:?}, tce: {:?}, ten: {:?} }}",
            self.test_count(),
            self.test_period(),
            self.qdn(),
            self.tce(),
            self.ten()
        )
    }
}
#[doc = "Upper Position Compare Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ucomp0(pub u16);
impl Ucomp0 {
    #[doc = "UCOMP0"]
    #[must_use]
    #[inline(always)]
    pub const fn ucomp0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "UCOMP0"]
    #[inline(always)]
    pub const fn set_ucomp0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Ucomp0 {
    #[inline(always)]
    fn default() -> Ucomp0 {
        Ucomp0(0)
    }
}
impl core::fmt::Debug for Ucomp0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ucomp0")
            .field("ucomp0", &self.ucomp0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ucomp0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ucomp0 {{ ucomp0: {=u16:?} }}", self.ucomp0())
    }
}
#[doc = "Upper Position Compare 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ucomp1(pub u16);
impl Ucomp1 {
    #[doc = "UCOMP1"]
    #[must_use]
    #[inline(always)]
    pub const fn ucomp1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "UCOMP1"]
    #[inline(always)]
    pub const fn set_ucomp1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Ucomp1 {
    #[inline(always)]
    fn default() -> Ucomp1 {
        Ucomp1(0)
    }
}
impl core::fmt::Debug for Ucomp1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ucomp1")
            .field("ucomp1", &self.ucomp1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ucomp1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ucomp1 {{ ucomp1: {=u16:?} }}", self.ucomp1())
    }
}
#[doc = "Upper Position Compare 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ucomp2(pub u16);
impl Ucomp2 {
    #[doc = "UCOMP2"]
    #[must_use]
    #[inline(always)]
    pub const fn ucomp2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "UCOMP2"]
    #[inline(always)]
    pub const fn set_ucomp2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Ucomp2 {
    #[inline(always)]
    fn default() -> Ucomp2 {
        Ucomp2(0)
    }
}
impl core::fmt::Debug for Ucomp2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ucomp2")
            .field("ucomp2", &self.ucomp2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ucomp2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ucomp2 {{ ucomp2: {=u16:?} }}", self.ucomp2())
    }
}
#[doc = "Upper Position Compare 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ucomp3(pub u16);
impl Ucomp3 {
    #[doc = "UCOMP3"]
    #[must_use]
    #[inline(always)]
    pub const fn ucomp3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "UCOMP3"]
    #[inline(always)]
    pub const fn set_ucomp3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Ucomp3 {
    #[inline(always)]
    fn default() -> Ucomp3 {
        Ucomp3(0)
    }
}
impl core::fmt::Debug for Ucomp3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ucomp3")
            .field("ucomp3", &self.ucomp3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ucomp3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ucomp3 {{ ucomp3: {=u16:?} }}", self.ucomp3())
    }
}
#[doc = "Upper Initialization Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uinit(pub u16);
impl Uinit {
    #[doc = "INIT"]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "INIT"]
    #[inline(always)]
    pub const fn set_init(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Uinit {
    #[inline(always)]
    fn default() -> Uinit {
        Uinit(0)
    }
}
impl core::fmt::Debug for Uinit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uinit").field("init", &self.init()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uinit {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Uinit {{ init: {=u16:?} }}", self.init())
    }
}
#[doc = "Upper Modulus Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Umod(pub u16);
impl Umod {
    #[doc = "MOD"]
    #[must_use]
    #[inline(always)]
    pub const fn mod_(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MOD"]
    #[inline(always)]
    pub const fn set_mod_(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Umod {
    #[inline(always)]
    fn default() -> Umod {
        Umod(0)
    }
}
impl core::fmt::Debug for Umod {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Umod").field("mod_", &self.mod_()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Umod {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Umod {{ mod_: {=u16:?} }}", self.mod_())
    }
}
#[doc = "Upper Position Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Upos(pub u16);
impl Upos {
    #[doc = "POS"]
    #[must_use]
    #[inline(always)]
    pub const fn pos(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "POS"]
    #[inline(always)]
    pub const fn set_pos(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Upos {
    #[inline(always)]
    fn default() -> Upos {
        Upos(0)
    }
}
impl core::fmt::Debug for Upos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Upos").field("pos", &self.pos()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Upos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Upos {{ pos: {=u16:?} }}", self.pos())
    }
}
#[doc = "Upper Position Hold Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uposh(pub u16);
impl Uposh {
    #[doc = "POSH"]
    #[must_use]
    #[inline(always)]
    pub const fn posh(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "POSH"]
    #[inline(always)]
    pub const fn set_posh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Uposh {
    #[inline(always)]
    fn default() -> Uposh {
        Uposh(0)
    }
}
impl core::fmt::Debug for Uposh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uposh").field("posh", &self.posh()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uposh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Uposh {{ posh: {=u16:?} }}", self.posh())
    }
}
#[doc = "Upper Position Holder Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uposh1(pub u16);
impl Uposh1 {
    #[doc = "UPOSH1"]
    #[must_use]
    #[inline(always)]
    pub const fn uposh1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "UPOSH1"]
    #[inline(always)]
    pub const fn set_uposh1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Uposh1 {
    #[inline(always)]
    fn default() -> Uposh1 {
        Uposh1(0)
    }
}
impl core::fmt::Debug for Uposh1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uposh1")
            .field("uposh1", &self.uposh1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uposh1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Uposh1 {{ uposh1: {=u16:?} }}", self.uposh1())
    }
}
#[doc = "Upper Position Holder Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uposh2(pub u16);
impl Uposh2 {
    #[doc = "UPOSH2"]
    #[must_use]
    #[inline(always)]
    pub const fn uposh2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "UPOSH2"]
    #[inline(always)]
    pub const fn set_uposh2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Uposh2 {
    #[inline(always)]
    fn default() -> Uposh2 {
        Uposh2(0)
    }
}
impl core::fmt::Debug for Uposh2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uposh2")
            .field("uposh2", &self.uposh2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uposh2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Uposh2 {{ uposh2: {=u16:?} }}", self.uposh2())
    }
}
#[doc = "Upper Position Holder Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uposh3(pub u16);
impl Uposh3 {
    #[doc = "UPOSH3"]
    #[must_use]
    #[inline(always)]
    pub const fn uposh3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "UPOSH3"]
    #[inline(always)]
    pub const fn set_uposh3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Uposh3 {
    #[inline(always)]
    fn default() -> Uposh3 {
        Uposh3(0)
    }
}
impl core::fmt::Debug for Uposh3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uposh3")
            .field("uposh3", &self.uposh3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uposh3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Uposh3 {{ uposh3: {=u16:?} }}", self.uposh3())
    }
}
#[doc = "Upper VERID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uverid(pub u16);
impl Uverid {
    #[doc = "UVERID"]
    #[must_use]
    #[inline(always)]
    pub const fn uverid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "UVERID"]
    #[inline(always)]
    pub const fn set_uverid(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Uverid {
    #[inline(always)]
    fn default() -> Uverid {
        Uverid(0)
    }
}
impl core::fmt::Debug for Uverid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uverid")
            .field("uverid", &self.uverid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uverid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Uverid {{ uverid: {=u16:?} }}", self.uverid())
    }
}
#[doc = "Watchdog Timeout Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wtr(pub u16);
impl Wtr {
    #[doc = "WDOG"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "WDOG"]
    #[inline(always)]
    pub const fn set_wdog(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Wtr {
    #[inline(always)]
    fn default() -> Wtr {
        Wtr(0)
    }
}
impl core::fmt::Debug for Wtr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wtr").field("wdog", &self.wdog()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wtr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wtr {{ wdog: {=u16:?} }}", self.wdog())
    }
}
