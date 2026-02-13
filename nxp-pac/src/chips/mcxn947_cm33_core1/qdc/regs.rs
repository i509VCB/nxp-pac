#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u16);
impl Ctrl {
    #[doc = "Compare Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Compare Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpirq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Interrupt Request"]
    #[inline(always)]
    pub const fn set_cmpirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Watchdog Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wde(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Enable"]
    #[inline(always)]
    pub const fn set_wde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Watchdog Timeout Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn die(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Timeout Interrupt Enable"]
    #[inline(always)]
    pub const fn set_die(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Watchdog Timeout Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dirq(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Timeout Interrupt Request"]
    #[inline(always)]
    pub const fn set_dirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Select Positive and Negative Edge of INDEX Pulse"]
    #[must_use]
    #[inline(always)]
    pub const fn xne(&self) -> super::vals::Xne {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Xne::from_bits(val as u8)
    }
    #[doc = "Select Positive and Negative Edge of INDEX Pulse"]
    #[inline(always)]
    pub const fn set_xne(&mut self, val: super::vals::Xne) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "INDEX Triggered Initialization of Position Counters UPOS and LPOS"]
    #[must_use]
    #[inline(always)]
    pub const fn xip(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "INDEX Triggered Initialization of Position Counters UPOS and LPOS"]
    #[inline(always)]
    pub const fn set_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "INDEX Pulse Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn xie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "INDEX Pulse Interrupt Enable"]
    #[inline(always)]
    pub const fn set_xie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "INDEX Pulse Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn xirq(&self) -> super::vals::Xirq {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Xirq::from_bits(val as u8)
    }
    #[doc = "INDEX Pulse Interrupt Request"]
    #[inline(always)]
    pub const fn set_xirq(&mut self, val: super::vals::Xirq) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Enable Signal Phase Count Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ph1(&self) -> super::vals::Ph1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Ph1::from_bits(val as u8)
    }
    #[doc = "Enable Signal Phase Count Mode"]
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
    #[doc = "Use Negative Edge of HOME Input"]
    #[must_use]
    #[inline(always)]
    pub const fn hne(&self) -> super::vals::Hne {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Hne::from_bits(val as u8)
    }
    #[doc = "Use Negative Edge of HOME Input"]
    #[inline(always)]
    pub const fn set_hne(&mut self, val: super::vals::Hne) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Enable HOME to Initialize Position Counters UPOS and LPOS"]
    #[must_use]
    #[inline(always)]
    pub const fn hip(&self) -> super::vals::Hip {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Hip::from_bits(val as u8)
    }
    #[doc = "Enable HOME to Initialize Position Counters UPOS and LPOS"]
    #[inline(always)]
    pub const fn set_hip(&mut self, val: super::vals::Hip) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "HOME Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "HOME Interrupt Enable"]
    #[inline(always)]
    pub const fn set_hie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "HOME Signal Transition Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn hirq(&self) -> super::vals::Hirq {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Hirq::from_bits(val as u8)
    }
    #[doc = "HOME Signal Transition Interrupt Request"]
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
            .field("cmpie", &self.cmpie())
            .field("cmpirq", &self.cmpirq())
            .field("wde", &self.wde())
            .field("die", &self.die())
            .field("dirq", &self.dirq())
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
            "Ctrl {{ cmpie: {=bool:?}, cmpirq: {=bool:?}, wde: {=bool:?}, die: {=bool:?}, dirq: {=bool:?}, xne: {:?}, xip: {=bool:?}, xie: {=bool:?}, xirq: {:?}, ph1: {:?}, rev: {:?}, swip: {:?}, hne: {:?}, hip: {:?}, hie: {=bool:?}, hirq: {:?} }}",
            self.cmpie(),
            self.cmpirq(),
            self.wde(),
            self.die(),
            self.dirq(),
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
#[doc = "Control 2"]
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
    pub const fn updpos(&self) -> super::vals::Updpos {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Updpos::from_bits(val as u8)
    }
    #[doc = "Update Position Registers"]
    #[inline(always)]
    pub const fn set_updpos(&mut self, val: super::vals::Updpos) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable Modulo Counting"]
    #[must_use]
    #[inline(always)]
    pub const fn mod_(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Modulo Counting"]
    #[inline(always)]
    pub const fn set_mod_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Count Direction Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::Dir {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dir::from_bits(val as u8)
    }
    #[doc = "Count Direction Flag"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: super::vals::Dir) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Roll-under Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ruie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Roll-under Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ruie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Roll-under Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn ruirq(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Roll-under Interrupt Request"]
    #[inline(always)]
    pub const fn set_ruirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Roll-over Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn roie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Roll-over Interrupt Enable"]
    #[inline(always)]
    pub const fn set_roie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Roll-over Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn roirq(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Roll-over Interrupt Request"]
    #[inline(always)]
    pub const fn set_roirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
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
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sabie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sabie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn sabirq(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Request"]
    #[inline(always)]
    pub const fn set_sabirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Initialize Position Registers"]
    #[must_use]
    #[inline(always)]
    pub const fn initpos(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Initialize Position Registers"]
    #[inline(always)]
    pub const fn set_initpos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
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
            .field("mod_", &self.mod_())
            .field("dir", &self.dir())
            .field("ruie", &self.ruie())
            .field("ruirq", &self.ruirq())
            .field("roie", &self.roie())
            .field("roirq", &self.roirq())
            .field("revmod", &self.revmod())
            .field("outctl", &self.outctl())
            .field("sabie", &self.sabie())
            .field("sabirq", &self.sabirq())
            .field("initpos", &self.initpos())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl2 {{ updhld: {=bool:?}, updpos: {:?}, mod_: {=bool:?}, dir: {:?}, ruie: {=bool:?}, ruirq: {=bool:?}, roie: {=bool:?}, roirq: {=bool:?}, revmod: {:?}, outctl: {:?}, sabie: {=bool:?}, sabirq: {=bool:?}, initpos: {=bool:?} }}",
            self.updhld(),
            self.updpos(),
            self.mod_(),
            self.dir(),
            self.ruie(),
            self.ruirq(),
            self.roie(),
            self.roirq(),
            self.revmod(),
            self.outctl(),
            self.sabie(),
            self.sabirq(),
            self.initpos()
        )
    }
}
#[doc = "Control 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl3(pub u16);
impl Ctrl3 {
    #[doc = "Period Measurement Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pmen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Period Measurement Function Enable"]
    #[inline(always)]
    pub const fn set_pmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Prescaler"]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
}
impl Default for Ctrl3 {
    #[inline(always)]
    fn default() -> Ctrl3 {
        Ctrl3(0)
    }
}
impl core::fmt::Debug for Ctrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl3")
            .field("pmen", &self.pmen())
            .field("prsc", &self.prsc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl3 {{ pmen: {=bool:?}, prsc: {=u8:?} }}",
            self.pmen(),
            self.prsc()
        )
    }
}
#[doc = "Input Filter"]
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
    #[doc = "Prescaler Divide IPBus Clock to FILT Clock"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_prsc(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Prescaler Divide IPBus Clock to FILT Clock"]
    #[inline(always)]
    pub const fn set_filt_prsc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
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
            .field("filt_prsc", &self.filt_prsc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Filt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Filt {{ filt_per: {=u8:?}, filt_cnt: {=u8:?}, filt_prsc: {=u8:?} }}",
            self.filt_per(),
            self.filt_cnt(),
            self.filt_prsc()
        )
    }
}
#[doc = "Input Monitor"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr(pub u16);
impl Imr {
    #[doc = "HOME"]
    #[must_use]
    #[inline(always)]
    pub const fn home(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HOME"]
    #[inline(always)]
    pub const fn set_home(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "INDEX"]
    #[must_use]
    #[inline(always)]
    pub const fn index(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "INDEX"]
    #[inline(always)]
    pub const fn set_index(&mut self, val: bool) {
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
    #[doc = "FHOM"]
    #[must_use]
    #[inline(always)]
    pub const fn fhom(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "FHOM"]
    #[inline(always)]
    pub const fn set_fhom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "FIND"]
    #[must_use]
    #[inline(always)]
    pub const fn find(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "FIND"]
    #[inline(always)]
    pub const fn set_find(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "FPHB"]
    #[must_use]
    #[inline(always)]
    pub const fn fphb(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "FPHB"]
    #[inline(always)]
    pub const fn set_fphb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "FPHA"]
    #[must_use]
    #[inline(always)]
    pub const fn fpha(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FPHA"]
    #[inline(always)]
    pub const fn set_fpha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
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
            .field("home", &self.home())
            .field("index", &self.index())
            .field("phb", &self.phb())
            .field("pha", &self.pha())
            .field("fhom", &self.fhom())
            .field("find", &self.find())
            .field("fphb", &self.fphb())
            .field("fpha", &self.fpha())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Imr {{ home: {=bool:?}, index: {=bool:?}, phb: {=bool:?}, pha: {=bool:?}, fhom: {=bool:?}, find: {=bool:?}, fphb: {=bool:?}, fpha: {=bool:?} }}",
            self.home(),
            self.index(),
            self.phb(),
            self.pha(),
            self.fhom(),
            self.find(),
            self.fphb(),
            self.fpha()
        )
    }
}
#[doc = "Last Edge Time"]
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
#[doc = "Last Edge Time Hold"]
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
#[doc = "Lower Position Compare"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcomp(pub u16);
impl Lcomp {
    #[doc = "COMP"]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "COMP"]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Lcomp {
    #[inline(always)]
    fn default() -> Lcomp {
        Lcomp(0)
    }
}
impl core::fmt::Debug for Lcomp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lcomp").field("comp", &self.comp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lcomp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lcomp {{ comp: {=u16:?} }}", self.comp())
    }
}
#[doc = "Lower Initialization"]
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
#[doc = "Lower Modulus"]
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
#[doc = "Lower Position Counter"]
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
#[doc = "Lower Position Hold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lposh(pub u16);
impl Lposh {
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
impl Default for Lposh {
    #[inline(always)]
    fn default() -> Lposh {
        Lposh(0)
    }
}
impl core::fmt::Debug for Lposh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lposh").field("posh", &self.posh()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lposh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lposh {{ posh: {=u16:?} }}", self.posh())
    }
}
#[doc = "Position Difference Counter"]
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
#[doc = "Position Difference Hold"]
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
#[doc = "Position Difference Period Counter"]
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
#[doc = "Position Difference Period Buffer"]
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
#[doc = "Position Difference Period Hold"]
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
#[doc = "Revolution Counter"]
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
#[doc = "Revolution Hold"]
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
#[doc = "Test"]
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
    pub const fn tce(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Test Counter Enable"]
    #[inline(always)]
    pub const fn set_tce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Test Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ten(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Test Mode Enable"]
    #[inline(always)]
    pub const fn set_ten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
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
            "Tst {{ test_count: {=u8:?}, test_period: {=u8:?}, qdn: {:?}, tce: {=bool:?}, ten: {=bool:?} }}",
            self.test_count(),
            self.test_period(),
            self.qdn(),
            self.tce(),
            self.ten()
        )
    }
}
#[doc = "Upper Position Compare"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ucomp(pub u16);
impl Ucomp {
    #[doc = "COMP"]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "COMP"]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Ucomp {
    #[inline(always)]
    fn default() -> Ucomp {
        Ucomp(0)
    }
}
impl core::fmt::Debug for Ucomp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ucomp").field("comp", &self.comp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ucomp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ucomp {{ comp: {=u16:?} }}", self.comp())
    }
}
#[doc = "Upper Initialization"]
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
#[doc = "Upper Modulus"]
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
#[doc = "Upper Position Counter"]
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
#[doc = "Upper Position Hold"]
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
#[doc = "Watchdog Timeout"]
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
