#[doc = "Timer Correction Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atcor(pub u32);
impl Atcor {
    #[doc = "Correction Counter Wrap-Around Value"]
    #[must_use]
    #[inline(always)]
    pub const fn cor(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Correction Counter Wrap-Around Value"]
    #[inline(always)]
    pub const fn set_cor(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
}
impl Default for Atcor {
    #[inline(always)]
    fn default() -> Atcor {
        Atcor(0)
    }
}
impl core::fmt::Debug for Atcor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Atcor").field("cor", &self.cor()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Atcor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Atcor {{ cor: {=u32:?} }}", self.cor())
    }
}
#[doc = "Adjustable Timer Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atcr(pub u32);
impl Atcr {
    #[doc = "Enable Timer"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> super::vals::En {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Enable Timer"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable One-Shot Offset Event"]
    #[must_use]
    #[inline(always)]
    pub const fn offen(&self) -> super::vals::Offen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Offen::from_bits(val as u8)
    }
    #[doc = "Enable One-Shot Offset Event"]
    #[inline(always)]
    pub const fn set_offen(&mut self, val: super::vals::Offen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Reset Timer On Offset Event"]
    #[must_use]
    #[inline(always)]
    pub const fn offrst(&self) -> super::vals::Offrst {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Offrst::from_bits(val as u8)
    }
    #[doc = "Reset Timer On Offset Event"]
    #[inline(always)]
    pub const fn set_offrst(&mut self, val: super::vals::Offrst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Periodical Event"]
    #[must_use]
    #[inline(always)]
    pub const fn peren(&self) -> super::vals::Peren {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Peren::from_bits(val as u8)
    }
    #[doc = "Enable Periodical Event"]
    #[inline(always)]
    pub const fn set_peren(&mut self, val: super::vals::Peren) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables event signal output external pin frc_evt_period assertion on period event"]
    #[must_use]
    #[inline(always)]
    pub const fn pinper(&self) -> super::vals::Pinper {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pinper::from_bits(val as u8)
    }
    #[doc = "Enables event signal output external pin frc_evt_period assertion on period event"]
    #[inline(always)]
    pub const fn set_pinper(&mut self, val: super::vals::Pinper) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Reset Timer"]
    #[must_use]
    #[inline(always)]
    pub const fn restart(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Timer"]
    #[inline(always)]
    pub const fn set_restart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture Timer Value"]
    #[must_use]
    #[inline(always)]
    pub const fn capture(&self) -> super::vals::Capture {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Capture::from_bits(val as u8)
    }
    #[doc = "Capture Timer Value"]
    #[inline(always)]
    pub const fn set_capture(&mut self, val: super::vals::Capture) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable Timer Slave Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn slave(&self) -> super::vals::Slave {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Slave::from_bits(val as u8)
    }
    #[doc = "Enable Timer Slave Mode"]
    #[inline(always)]
    pub const fn set_slave(&mut self, val: super::vals::Slave) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Atcr {
    #[inline(always)]
    fn default() -> Atcr {
        Atcr(0)
    }
}
impl core::fmt::Debug for Atcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Atcr")
            .field("en", &self.en())
            .field("offen", &self.offen())
            .field("offrst", &self.offrst())
            .field("peren", &self.peren())
            .field("pinper", &self.pinper())
            .field("restart", &self.restart())
            .field("capture", &self.capture())
            .field("slave", &self.slave())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Atcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Atcr {{ en: {:?}, offen: {:?}, offrst: {:?}, peren: {:?}, pinper: {:?}, restart: {=bool:?}, capture: {:?}, slave: {:?} }}",
            self.en(),
            self.offen(),
            self.offrst(),
            self.peren(),
            self.pinper(),
            self.restart(),
            self.capture(),
            self.slave()
        )
    }
}
#[doc = "Time-Stamping Clock Period Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atinc(pub u32);
impl Atinc {
    #[doc = "Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds"]
    #[must_use]
    #[inline(always)]
    pub const fn inc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds"]
    #[inline(always)]
    pub const fn set_inc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Correction Increment Value"]
    #[must_use]
    #[inline(always)]
    pub const fn inc_corr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Correction Increment Value"]
    #[inline(always)]
    pub const fn set_inc_corr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
}
impl Default for Atinc {
    #[inline(always)]
    fn default() -> Atinc {
        Atinc(0)
    }
}
impl core::fmt::Debug for Atinc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Atinc")
            .field("inc", &self.inc())
            .field("inc_corr", &self.inc_corr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Atinc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Atinc {{ inc: {=u8:?}, inc_corr: {=u8:?} }}",
            self.inc(),
            self.inc_corr()
        )
    }
}
#[doc = "Timer Offset Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atoff(pub u32);
impl Atoff {
    #[doc = "Offset value for one-shot event generation"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Offset value for one-shot event generation"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Atoff {
    #[inline(always)]
    fn default() -> Atoff {
        Atoff(0)
    }
}
impl core::fmt::Debug for Atoff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Atoff")
            .field("offset", &self.offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Atoff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Atoff {{ offset: {=u32:?} }}", self.offset())
    }
}
#[doc = "Timer Period Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atper(pub u32);
impl Atper {
    #[doc = "Value for generating periodic events"]
    #[must_use]
    #[inline(always)]
    pub const fn period(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Value for generating periodic events"]
    #[inline(always)]
    pub const fn set_period(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Atper {
    #[inline(always)]
    fn default() -> Atper {
        Atper(0)
    }
}
impl core::fmt::Debug for Atper {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Atper")
            .field("period", &self.period())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Atper {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Atper {{ period: {=u32:?} }}", self.period())
    }
}
#[doc = "Timestamp of Last Transmitted Frame"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atstmp(pub u32);
impl Atstmp {
    #[doc = "Timestamp of the last frame transmitted by the core that had TxBD\\[TS\\] set the ff_tx_ts_frm signal asserted from the user application"]
    #[must_use]
    #[inline(always)]
    pub const fn timestamp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timestamp of the last frame transmitted by the core that had TxBD\\[TS\\] set the ff_tx_ts_frm signal asserted from the user application"]
    #[inline(always)]
    pub const fn set_timestamp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Atstmp {
    #[inline(always)]
    fn default() -> Atstmp {
        Atstmp(0)
    }
}
impl core::fmt::Debug for Atstmp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Atstmp")
            .field("timestamp", &self.timestamp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Atstmp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Atstmp {{ timestamp: {=u32:?} }}", self.timestamp())
    }
}
#[doc = "Timer Value Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atvr(pub u32);
impl Atvr {
    #[doc = "A write sets the timer"]
    #[must_use]
    #[inline(always)]
    pub const fn atime(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "A write sets the timer"]
    #[inline(always)]
    pub const fn set_atime(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Atvr {
    #[inline(always)]
    fn default() -> Atvr {
        Atvr(0)
    }
}
impl core::fmt::Debug for Atvr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Atvr")
            .field("atime", &self.atime())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Atvr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Atvr {{ atime: {=u32:?} }}", self.atime())
    }
}
#[doc = "Ethernet Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecr(pub u32);
impl Ecr {
    #[doc = "Ethernet MAC Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet MAC Reset"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Ethernet Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn etheren(&self) -> super::vals::Etheren {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Etheren::from_bits(val as u8)
    }
    #[doc = "Ethernet Enable"]
    #[inline(always)]
    pub const fn set_etheren(&mut self, val: super::vals::Etheren) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Magic Packet Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn magicen(&self) -> super::vals::Magicen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Magicen::from_bits(val as u8)
    }
    #[doc = "Magic Packet Detection Enable"]
    #[inline(always)]
    pub const fn set_magicen(&mut self, val: super::vals::Magicen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Sleep Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sleep(&self) -> super::vals::Sleep {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Sleep::from_bits(val as u8)
    }
    #[doc = "Sleep Mode Enable"]
    #[inline(always)]
    pub const fn set_sleep(&mut self, val: super::vals::Sleep) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "EN1588 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en1588(&self) -> super::vals::En1588 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::En1588::from_bits(val as u8)
    }
    #[doc = "EN1588 Enable"]
    #[inline(always)]
    pub const fn set_en1588(&mut self, val: super::vals::En1588) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> super::vals::Dbgen {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dbgen::from_bits(val as u8)
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: super::vals::Dbgen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Descriptor Byte Swapping Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbswp(&self) -> super::vals::Dbswp {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dbswp::from_bits(val as u8)
    }
    #[doc = "Descriptor Byte Swapping Enable"]
    #[inline(always)]
    pub const fn set_dbswp(&mut self, val: super::vals::Dbswp) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Ecr {
    #[inline(always)]
    fn default() -> Ecr {
        Ecr(0)
    }
}
impl core::fmt::Debug for Ecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ecr")
            .field("reset", &self.reset())
            .field("etheren", &self.etheren())
            .field("magicen", &self.magicen())
            .field("sleep", &self.sleep())
            .field("en1588", &self.en1588())
            .field("dbgen", &self.dbgen())
            .field("dbswp", &self.dbswp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ecr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ecr {{ reset: {=bool:?}, etheren: {:?}, magicen: {:?}, sleep: {:?}, en1588: {:?}, dbgen: {:?}, dbswp: {:?} }}",
            self.reset(),
            self.etheren(),
            self.magicen(),
            self.sleep(),
            self.en1588(),
            self.dbgen(),
            self.dbswp()
        )
    }
}
#[doc = "Interrupt Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eimr(pub u32);
impl Eimr {
    #[doc = "TS_TIMER Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn ts_timer(&self) -> super::vals::TsTimer {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::TsTimer::from_bits(val as u8)
    }
    #[doc = "TS_TIMER Interrupt Mask"]
    #[inline(always)]
    pub const fn set_ts_timer(&mut self, val: super::vals::TsTimer) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "TS_AVAIL Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn ts_avail(&self) -> super::vals::TsAvail {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::TsAvail::from_bits(val as u8)
    }
    #[doc = "TS_AVAIL Interrupt Mask"]
    #[inline(always)]
    pub const fn set_ts_avail(&mut self, val: super::vals::TsAvail) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "WAKEUP Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup(&self) -> super::vals::Wakeup {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Wakeup::from_bits(val as u8)
    }
    #[doc = "WAKEUP Interrupt Mask"]
    #[inline(always)]
    pub const fn set_wakeup(&mut self, val: super::vals::Wakeup) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "PLR Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn plr(&self) -> super::vals::Plr {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Plr::from_bits(val as u8)
    }
    #[doc = "PLR Interrupt Mask"]
    #[inline(always)]
    pub const fn set_plr(&mut self, val: super::vals::Plr) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "UN Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn un(&self) -> super::vals::Un {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Un::from_bits(val as u8)
    }
    #[doc = "UN Interrupt Mask"]
    #[inline(always)]
    pub const fn set_un(&mut self, val: super::vals::Un) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "RL Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rl(&self) -> super::vals::Rl {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Rl::from_bits(val as u8)
    }
    #[doc = "RL Interrupt Mask"]
    #[inline(always)]
    pub const fn set_rl(&mut self, val: super::vals::Rl) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "LC Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn lc(&self) -> super::vals::Lc {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Lc::from_bits(val as u8)
    }
    #[doc = "LC Interrupt Mask"]
    #[inline(always)]
    pub const fn set_lc(&mut self, val: super::vals::Lc) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "EBERR Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn eberr(&self) -> super::vals::Eberr {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Eberr::from_bits(val as u8)
    }
    #[doc = "EBERR Interrupt Mask"]
    #[inline(always)]
    pub const fn set_eberr(&mut self, val: super::vals::Eberr) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "MII Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn mii(&self) -> super::vals::Mii {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mii::from_bits(val as u8)
    }
    #[doc = "MII Interrupt Mask"]
    #[inline(always)]
    pub const fn set_mii(&mut self, val: super::vals::Mii) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "RXB Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rxb(&self) -> super::vals::Rxb {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Rxb::from_bits(val as u8)
    }
    #[doc = "RXB Interrupt Mask"]
    #[inline(always)]
    pub const fn set_rxb(&mut self, val: super::vals::Rxb) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "RXF Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rxf(&self) -> super::vals::Rxf {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Rxf::from_bits(val as u8)
    }
    #[doc = "RXF Interrupt Mask"]
    #[inline(always)]
    pub const fn set_rxf(&mut self, val: super::vals::Rxf) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "TXB Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn txb(&self) -> super::vals::Txb {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Txb::from_bits(val as u8)
    }
    #[doc = "TXB Interrupt Mask"]
    #[inline(always)]
    pub const fn set_txb(&mut self, val: super::vals::Txb) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "TXF Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn txf(&self) -> super::vals::Txf {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Txf::from_bits(val as u8)
    }
    #[doc = "TXF Interrupt Mask"]
    #[inline(always)]
    pub const fn set_txf(&mut self, val: super::vals::Txf) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "GRA Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn gra(&self) -> super::vals::Gra {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Gra::from_bits(val as u8)
    }
    #[doc = "GRA Interrupt Mask"]
    #[inline(always)]
    pub const fn set_gra(&mut self, val: super::vals::Gra) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "BABT Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn babt(&self) -> super::vals::Babt {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Babt::from_bits(val as u8)
    }
    #[doc = "BABT Interrupt Mask"]
    #[inline(always)]
    pub const fn set_babt(&mut self, val: super::vals::Babt) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "BABR Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn babr(&self) -> super::vals::Babr {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Babr::from_bits(val as u8)
    }
    #[doc = "BABR Interrupt Mask"]
    #[inline(always)]
    pub const fn set_babr(&mut self, val: super::vals::Babr) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Eimr {
    #[inline(always)]
    fn default() -> Eimr {
        Eimr(0)
    }
}
impl core::fmt::Debug for Eimr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eimr")
            .field("ts_timer", &self.ts_timer())
            .field("ts_avail", &self.ts_avail())
            .field("wakeup", &self.wakeup())
            .field("plr", &self.plr())
            .field("un", &self.un())
            .field("rl", &self.rl())
            .field("lc", &self.lc())
            .field("eberr", &self.eberr())
            .field("mii", &self.mii())
            .field("rxb", &self.rxb())
            .field("rxf", &self.rxf())
            .field("txb", &self.txb())
            .field("txf", &self.txf())
            .field("gra", &self.gra())
            .field("babt", &self.babt())
            .field("babr", &self.babr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eimr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eimr {{ ts_timer: {:?}, ts_avail: {:?}, wakeup: {:?}, plr: {:?}, un: {:?}, rl: {:?}, lc: {:?}, eberr: {:?}, mii: {:?}, rxb: {:?}, rxf: {:?}, txb: {:?}, txf: {:?}, gra: {:?}, babt: {:?}, babr: {:?} }}",
            self.ts_timer(),
            self.ts_avail(),
            self.wakeup(),
            self.plr(),
            self.un(),
            self.rl(),
            self.lc(),
            self.eberr(),
            self.mii(),
            self.rxb(),
            self.rxf(),
            self.txb(),
            self.txf(),
            self.gra(),
            self.babt(),
            self.babr()
        )
    }
}
#[doc = "Interrupt Event Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eir(pub u32);
impl Eir {
    #[doc = "Timestamp Timer"]
    #[must_use]
    #[inline(always)]
    pub const fn ts_timer(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Timer"]
    #[inline(always)]
    pub const fn set_ts_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Transmit Timestamp Available"]
    #[must_use]
    #[inline(always)]
    pub const fn ts_avail(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Timestamp Available"]
    #[inline(always)]
    pub const fn set_ts_avail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Node Wakeup Request Indication"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Node Wakeup Request Indication"]
    #[inline(always)]
    pub const fn set_wakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Payload Receive Error"]
    #[must_use]
    #[inline(always)]
    pub const fn plr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Payload Receive Error"]
    #[inline(always)]
    pub const fn set_plr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Transmit FIFO Underrun"]
    #[must_use]
    #[inline(always)]
    pub const fn un(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Underrun"]
    #[inline(always)]
    pub const fn set_un(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Collision Retry Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn rl(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Collision Retry Limit"]
    #[inline(always)]
    pub const fn set_rl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Late Collision"]
    #[must_use]
    #[inline(always)]
    pub const fn lc(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Late Collision"]
    #[inline(always)]
    pub const fn set_lc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Ethernet Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn eberr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet Bus Error"]
    #[inline(always)]
    pub const fn set_eberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "MII Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn mii(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "MII Interrupt."]
    #[inline(always)]
    pub const fn set_mii(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Receive Buffer Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn rxb(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Buffer Interrupt"]
    #[inline(always)]
    pub const fn set_rxb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Receive Frame Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn rxf(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Frame Interrupt"]
    #[inline(always)]
    pub const fn set_rxf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Transmit Buffer Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn txb(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Buffer Interrupt"]
    #[inline(always)]
    pub const fn set_txb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Transmit Frame Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn txf(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Frame Interrupt"]
    #[inline(always)]
    pub const fn set_txf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Graceful Stop Complete"]
    #[must_use]
    #[inline(always)]
    pub const fn gra(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Graceful Stop Complete"]
    #[inline(always)]
    pub const fn set_gra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Babbling Transmit Error"]
    #[must_use]
    #[inline(always)]
    pub const fn babt(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Babbling Transmit Error"]
    #[inline(always)]
    pub const fn set_babt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Babbling Receive Error"]
    #[must_use]
    #[inline(always)]
    pub const fn babr(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Babbling Receive Error"]
    #[inline(always)]
    pub const fn set_babr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Eir {
    #[inline(always)]
    fn default() -> Eir {
        Eir(0)
    }
}
impl core::fmt::Debug for Eir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eir")
            .field("ts_timer", &self.ts_timer())
            .field("ts_avail", &self.ts_avail())
            .field("wakeup", &self.wakeup())
            .field("plr", &self.plr())
            .field("un", &self.un())
            .field("rl", &self.rl())
            .field("lc", &self.lc())
            .field("eberr", &self.eberr())
            .field("mii", &self.mii())
            .field("rxb", &self.rxb())
            .field("rxf", &self.rxf())
            .field("txb", &self.txb())
            .field("txf", &self.txf())
            .field("gra", &self.gra())
            .field("babt", &self.babt())
            .field("babr", &self.babr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eir {{ ts_timer: {=bool:?}, ts_avail: {=bool:?}, wakeup: {=bool:?}, plr: {=bool:?}, un: {=bool:?}, rl: {=bool:?}, lc: {=bool:?}, eberr: {=bool:?}, mii: {=bool:?}, rxb: {=bool:?}, rxf: {=bool:?}, txb: {=bool:?}, txf: {=bool:?}, gra: {=bool:?}, babt: {=bool:?}, babr: {=bool:?} }}",
            self.ts_timer(),
            self.ts_avail(),
            self.wakeup(),
            self.plr(),
            self.un(),
            self.rl(),
            self.lc(),
            self.eberr(),
            self.mii(),
            self.rxb(),
            self.rxf(),
            self.txb(),
            self.txf(),
            self.gra(),
            self.babt(),
            self.babr()
        )
    }
}
#[doc = "Frame Truncation Length"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftrl(pub u32);
impl Ftrl {
    #[doc = "Frame Truncation Length"]
    #[must_use]
    #[inline(always)]
    pub const fn trunc_fl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Frame Truncation Length"]
    #[inline(always)]
    pub const fn set_trunc_fl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Ftrl {
    #[inline(always)]
    fn default() -> Ftrl {
        Ftrl(0)
    }
}
impl core::fmt::Debug for Ftrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ftrl")
            .field("trunc_fl", &self.trunc_fl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ftrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ftrl {{ trunc_fl: {=u16:?} }}", self.trunc_fl())
    }
}
#[doc = "Descriptor Group Lower Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Galr(pub u32);
impl Galr {
    #[doc = "Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[must_use]
    #[inline(always)]
    pub const fn gaddr2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline(always)]
    pub const fn set_gaddr2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Galr {
    #[inline(always)]
    fn default() -> Galr {
        Galr(0)
    }
}
impl core::fmt::Debug for Galr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Galr")
            .field("gaddr2", &self.gaddr2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Galr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Galr {{ gaddr2: {=u32:?} }}", self.gaddr2())
    }
}
#[doc = "Descriptor Group Upper Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gaur(pub u32);
impl Gaur {
    #[doc = "Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[must_use]
    #[inline(always)]
    pub const fn gaddr1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline(always)]
    pub const fn set_gaddr1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gaur {
    #[inline(always)]
    fn default() -> Gaur {
        Gaur(0)
    }
}
impl core::fmt::Debug for Gaur {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gaur")
            .field("gaddr1", &self.gaddr1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gaur {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gaur {{ gaddr1: {=u32:?} }}", self.gaddr1())
    }
}
#[doc = "Descriptor Individual Lower Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ialr(pub u32);
impl Ialr {
    #[doc = "Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[must_use]
    #[inline(always)]
    pub const fn iaddr2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[inline(always)]
    pub const fn set_iaddr2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ialr {
    #[inline(always)]
    fn default() -> Ialr {
        Ialr(0)
    }
}
impl core::fmt::Debug for Ialr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ialr")
            .field("iaddr2", &self.iaddr2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ialr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ialr {{ iaddr2: {=u32:?} }}", self.iaddr2())
    }
}
#[doc = "Descriptor Individual Upper Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iaur(pub u32);
impl Iaur {
    #[doc = "Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[must_use]
    #[inline(always)]
    pub const fn iaddr1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[inline(always)]
    pub const fn set_iaddr1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Iaur {
    #[inline(always)]
    fn default() -> Iaur {
        Iaur(0)
    }
}
impl core::fmt::Debug for Iaur {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iaur")
            .field("iaddr1", &self.iaddr1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iaur {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iaur {{ iaddr1: {=u32:?} }}", self.iaddr1())
    }
}
#[doc = "Frames Received with Alignment Error Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeRAlign(pub u32);
impl IeeeRAlign {
    #[doc = "Number of frames received with alignment error"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames received with alignment error"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeRAlign {
    #[inline(always)]
    fn default() -> IeeeRAlign {
        IeeeRAlign(0)
    }
}
impl core::fmt::Debug for IeeeRAlign {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeRAlign")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeRAlign {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeRAlign {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Received with CRC Error Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeRCrc(pub u32);
impl IeeeRCrc {
    #[doc = "Number of frames received with CRC error"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames received with CRC error"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeRCrc {
    #[inline(always)]
    fn default() -> IeeeRCrc {
        IeeeRCrc(0)
    }
}
impl core::fmt::Debug for IeeeRCrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeRCrc")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeRCrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeRCrc {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames not Counted Correctly Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeRDrop(pub u32);
impl IeeeRDrop {
    #[doc = "Frame count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Frame count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeRDrop {
    #[inline(always)]
    fn default() -> IeeeRDrop {
        IeeeRDrop(0)
    }
}
impl core::fmt::Debug for IeeeRDrop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeRDrop")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeRDrop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeRDrop {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Flow Control Pause Frames Received Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeRFdxfc(pub u32);
impl IeeeRFdxfc {
    #[doc = "Number of flow-control pause frames received"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of flow-control pause frames received"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeRFdxfc {
    #[inline(always)]
    fn default() -> IeeeRFdxfc {
        IeeeRFdxfc(0)
    }
}
impl core::fmt::Debug for IeeeRFdxfc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeRFdxfc")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeRFdxfc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeRFdxfc {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Received OK Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeRFrameOk(pub u32);
impl IeeeRFrameOk {
    #[doc = "Number of frames received OK"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames received OK"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeRFrameOk {
    #[inline(always)]
    fn default() -> IeeeRFrameOk {
        IeeeRFrameOk(0)
    }
}
impl core::fmt::Debug for IeeeRFrameOk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeRFrameOk")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeRFrameOk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeRFrameOk {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Receive FIFO Overflow Count Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeRMacerr(pub u32);
impl IeeeRMacerr {
    #[doc = "Receive FIFO overflow count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Receive FIFO overflow count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeRMacerr {
    #[inline(always)]
    fn default() -> IeeeRMacerr {
        IeeeRMacerr(0)
    }
}
impl core::fmt::Debug for IeeeRMacerr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeRMacerr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeRMacerr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeRMacerr {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Octet Count for Frames Received without Error Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeROctetsOk(pub u32);
impl IeeeROctetsOk {
    #[doc = "Number of octets for frames received without error"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Number of octets for frames received without error"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IeeeROctetsOk {
    #[inline(always)]
    fn default() -> IeeeROctetsOk {
        IeeeROctetsOk(0)
    }
}
impl core::fmt::Debug for IeeeROctetsOk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeROctetsOk")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeROctetsOk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeROctetsOk {{ count: {=u32:?} }}", self.count())
    }
}
#[doc = "Frames Transmitted with Single Collision Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeT1col(pub u32);
impl IeeeT1col {
    #[doc = "Number of frames transmitted with one collision"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames transmitted with one collision"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeT1col {
    #[inline(always)]
    fn default() -> IeeeT1col {
        IeeeT1col(0)
    }
}
impl core::fmt::Debug for IeeeT1col {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeT1col")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeT1col {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeT1col {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Transmitted with Carrier Sense Error Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTCserr(pub u32);
impl IeeeTCserr {
    #[doc = "Number of frames transmitted with carrier sense error"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames transmitted with carrier sense error"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTCserr {
    #[inline(always)]
    fn default() -> IeeeTCserr {
        IeeeTCserr(0)
    }
}
impl core::fmt::Debug for IeeeTCserr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTCserr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTCserr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTCserr {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Transmitted after Deferral Delay Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTDef(pub u32);
impl IeeeTDef {
    #[doc = "Number of frames transmitted with deferral delay"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames transmitted with deferral delay"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTDef {
    #[inline(always)]
    fn default() -> IeeeTDef {
        IeeeTDef(0)
    }
}
impl core::fmt::Debug for IeeeTDef {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTDef")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTDef {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTDef {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Transmitted with Excessive Collisions Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTExcol(pub u32);
impl IeeeTExcol {
    #[doc = "Number of frames transmitted with excessive collisions"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames transmitted with excessive collisions"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTExcol {
    #[inline(always)]
    fn default() -> IeeeTExcol {
        IeeeTExcol(0)
    }
}
impl core::fmt::Debug for IeeeTExcol {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTExcol")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTExcol {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTExcol {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Flow Control Pause Frames Transmitted Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTFdxfc(pub u32);
impl IeeeTFdxfc {
    #[doc = "Number of flow-control pause frames transmitted"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of flow-control pause frames transmitted"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTFdxfc {
    #[inline(always)]
    fn default() -> IeeeTFdxfc {
        IeeeTFdxfc(0)
    }
}
impl core::fmt::Debug for IeeeTFdxfc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTFdxfc")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTFdxfc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTFdxfc {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Transmitted OK Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTFrameOk(pub u32);
impl IeeeTFrameOk {
    #[doc = "Number of frames transmitted OK"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames transmitted OK"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTFrameOk {
    #[inline(always)]
    fn default() -> IeeeTFrameOk {
        IeeeTFrameOk(0)
    }
}
impl core::fmt::Debug for IeeeTFrameOk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTFrameOk")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTFrameOk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTFrameOk {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Transmitted with Late Collision Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTLcol(pub u32);
impl IeeeTLcol {
    #[doc = "Number of frames transmitted with late collision"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames transmitted with late collision"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTLcol {
    #[inline(always)]
    fn default() -> IeeeTLcol {
        IeeeTLcol(0)
    }
}
impl core::fmt::Debug for IeeeTLcol {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTLcol")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTLcol {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTLcol {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Transmitted with Tx FIFO Underrun Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTMacerr(pub u32);
impl IeeeTMacerr {
    #[doc = "Number of frames transmitted with transmit FIFO underrun"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames transmitted with transmit FIFO underrun"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTMacerr {
    #[inline(always)]
    fn default() -> IeeeTMacerr {
        IeeeTMacerr(0)
    }
}
impl core::fmt::Debug for IeeeTMacerr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTMacerr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTMacerr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTMacerr {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Transmitted with Multiple Collisions Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTMcol(pub u32);
impl IeeeTMcol {
    #[doc = "Number of frames transmitted with multiple collisions"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames transmitted with multiple collisions"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTMcol {
    #[inline(always)]
    fn default() -> IeeeTMcol {
        IeeeTMcol(0)
    }
}
impl core::fmt::Debug for IeeeTMcol {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTMcol")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTMcol {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTMcol {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Octet Count for Frames Transmitted w/o Error Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTOctetsOk(pub u32);
impl IeeeTOctetsOk {
    #[doc = "Octet count for frames transmitted without error Counts total octets (includes header and FCS fields)."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Octet count for frames transmitted without error Counts total octets (includes header and FCS fields)."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IeeeTOctetsOk {
    #[inline(always)]
    fn default() -> IeeeTOctetsOk {
        IeeeTOctetsOk(0)
    }
}
impl core::fmt::Debug for IeeeTOctetsOk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTOctetsOk")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTOctetsOk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTOctetsOk {{ count: {=u32:?} }}", self.count())
    }
}
#[doc = "Reserved Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTSqe(pub u32);
impl IeeeTSqe {
    #[doc = "This read-only field is reserved and always has the value 0"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This read-only field is reserved and always has the value 0"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTSqe {
    #[inline(always)]
    fn default() -> IeeeTSqe {
        IeeeTSqe(0)
    }
}
impl core::fmt::Debug for IeeeTSqe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTSqe")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTSqe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTSqe {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "MIB Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mibc(pub u32);
impl Mibc {
    #[doc = "MIB Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn mib_clear(&self) -> super::vals::MibClear {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MibClear::from_bits(val as u8)
    }
    #[doc = "MIB Clear"]
    #[inline(always)]
    pub const fn set_mib_clear(&mut self, val: super::vals::MibClear) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "MIB Idle"]
    #[must_use]
    #[inline(always)]
    pub const fn mib_idle(&self) -> super::vals::MibIdle {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MibIdle::from_bits(val as u8)
    }
    #[doc = "MIB Idle"]
    #[inline(always)]
    pub const fn set_mib_idle(&mut self, val: super::vals::MibIdle) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Disable MIB Logic"]
    #[must_use]
    #[inline(always)]
    pub const fn mib_dis(&self) -> super::vals::MibDis {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MibDis::from_bits(val as u8)
    }
    #[doc = "Disable MIB Logic"]
    #[inline(always)]
    pub const fn set_mib_dis(&mut self, val: super::vals::MibDis) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mibc {
    #[inline(always)]
    fn default() -> Mibc {
        Mibc(0)
    }
}
impl core::fmt::Debug for Mibc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mibc")
            .field("mib_clear", &self.mib_clear())
            .field("mib_idle", &self.mib_idle())
            .field("mib_dis", &self.mib_dis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mibc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mibc {{ mib_clear: {:?}, mib_idle: {:?}, mib_dis: {:?} }}",
            self.mib_clear(),
            self.mib_idle(),
            self.mib_dis()
        )
    }
}
#[doc = "MII Management Frame Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmfr(pub u32);
impl Mmfr {
    #[doc = "Management Frame Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Management Frame Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Turn Around"]
    #[must_use]
    #[inline(always)]
    pub const fn ta(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Turn Around"]
    #[inline(always)]
    pub const fn set_ta(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Register Address"]
    #[must_use]
    #[inline(always)]
    pub const fn ra(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x1f;
        val as u8
    }
    #[doc = "Register Address"]
    #[inline(always)]
    pub const fn set_ra(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
    }
    #[doc = "PHY Address"]
    #[must_use]
    #[inline(always)]
    pub const fn pa(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x1f;
        val as u8
    }
    #[doc = "PHY Address"]
    #[inline(always)]
    pub const fn set_pa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 23usize)) | (((val as u32) & 0x1f) << 23usize);
    }
    #[doc = "Operation Code"]
    #[must_use]
    #[inline(always)]
    pub const fn op(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Operation Code"]
    #[inline(always)]
    pub const fn set_op(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Start Of Frame Delimiter"]
    #[must_use]
    #[inline(always)]
    pub const fn st(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Start Of Frame Delimiter"]
    #[inline(always)]
    pub const fn set_st(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Mmfr {
    #[inline(always)]
    fn default() -> Mmfr {
        Mmfr(0)
    }
}
impl core::fmt::Debug for Mmfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mmfr")
            .field("data", &self.data())
            .field("ta", &self.ta())
            .field("ra", &self.ra())
            .field("pa", &self.pa())
            .field("op", &self.op())
            .field("st", &self.st())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mmfr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mmfr {{ data: {=u16:?}, ta: {=u8:?}, ra: {=u8:?}, pa: {=u8:?}, op: {=u8:?}, st: {=u8:?} }}",
            self.data(),
            self.ta(),
            self.ra(),
            self.pa(),
            self.op(),
            self.st()
        )
    }
}
#[doc = "Maximum Receive Buffer Size Register - Ring 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrbr(pub u32);
impl Mrbr {
    #[doc = "Receive buffer size in bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn r_buf_size(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x03ff;
        val as u16
    }
    #[doc = "Receive buffer size in bytes"]
    #[inline(always)]
    pub const fn set_r_buf_size(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 4usize)) | (((val as u32) & 0x03ff) << 4usize);
    }
}
impl Default for Mrbr {
    #[inline(always)]
    fn default() -> Mrbr {
        Mrbr(0)
    }
}
impl core::fmt::Debug for Mrbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mrbr")
            .field("r_buf_size", &self.r_buf_size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mrbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mrbr {{ r_buf_size: {=u16:?} }}", self.r_buf_size())
    }
}
#[doc = "MII Speed Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mscr(pub u32);
impl Mscr {
    #[doc = "MII Speed"]
    #[must_use]
    #[inline(always)]
    pub const fn mii_speed(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "MII Speed"]
    #[inline(always)]
    pub const fn set_mii_speed(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "Disable Preamble"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_pre(&self) -> super::vals::DisPre {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::DisPre::from_bits(val as u8)
    }
    #[doc = "Disable Preamble"]
    #[inline(always)]
    pub const fn set_dis_pre(&mut self, val: super::vals::DisPre) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Hold time On MDIO Output"]
    #[must_use]
    #[inline(always)]
    pub const fn holdtime(&self) -> super::vals::Holdtime {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Holdtime::from_bits(val as u8)
    }
    #[doc = "Hold time On MDIO Output"]
    #[inline(always)]
    pub const fn set_holdtime(&mut self, val: super::vals::Holdtime) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
}
impl Default for Mscr {
    #[inline(always)]
    fn default() -> Mscr {
        Mscr(0)
    }
}
impl core::fmt::Debug for Mscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mscr")
            .field("mii_speed", &self.mii_speed())
            .field("dis_pre", &self.dis_pre())
            .field("holdtime", &self.holdtime())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mscr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mscr {{ mii_speed: {=u8:?}, dis_pre: {:?}, holdtime: {:?} }}",
            self.mii_speed(),
            self.dis_pre(),
            self.holdtime()
        )
    }
}
#[doc = "Opcode/Pause Duration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opd(pub u32);
impl Opd {
    #[doc = "Pause Duration"]
    #[must_use]
    #[inline(always)]
    pub const fn pause_dur(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Pause Duration"]
    #[inline(always)]
    pub const fn set_pause_dur(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Opcode Field In PAUSE Frames"]
    #[must_use]
    #[inline(always)]
    pub const fn opcode(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Opcode Field In PAUSE Frames"]
    #[inline(always)]
    pub const fn set_opcode(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Opd {
    #[inline(always)]
    fn default() -> Opd {
        Opd(0)
    }
}
impl core::fmt::Debug for Opd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opd")
            .field("pause_dur", &self.pause_dur())
            .field("opcode", &self.opcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Opd {{ pause_dur: {=u16:?}, opcode: {=u16:?} }}",
            self.pause_dur(),
            self.opcode()
        )
    }
}
#[doc = "Physical Address Lower Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Palr(pub u32);
impl Palr {
    #[doc = "Pause Address"]
    #[must_use]
    #[inline(always)]
    pub const fn paddr1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Pause Address"]
    #[inline(always)]
    pub const fn set_paddr1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Palr {
    #[inline(always)]
    fn default() -> Palr {
        Palr(0)
    }
}
impl core::fmt::Debug for Palr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Palr")
            .field("paddr1", &self.paddr1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Palr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Palr {{ paddr1: {=u32:?} }}", self.paddr1())
    }
}
#[doc = "Physical Address Upper Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Paur(pub u32);
impl Paur {
    #[doc = "Type Field In PAUSE Frames"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Type Field In PAUSE Frames"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames"]
    #[must_use]
    #[inline(always)]
    pub const fn paddr2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames"]
    #[inline(always)]
    pub const fn set_paddr2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Paur {
    #[inline(always)]
    fn default() -> Paur {
        Paur(0)
    }
}
impl core::fmt::Debug for Paur {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Paur")
            .field("type_", &self.type_())
            .field("paddr2", &self.paddr2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Paur {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Paur {{ type_: {=u16:?}, paddr2: {=u16:?} }}",
            self.type_(),
            self.paddr2()
        )
    }
}
#[doc = "Receive Accelerator Function Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Racc(pub u32);
impl Racc {
    #[doc = "Enable Padding Removal For Short IP Frames"]
    #[must_use]
    #[inline(always)]
    pub const fn padrem(&self) -> super::vals::Padrem {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Padrem::from_bits(val as u8)
    }
    #[doc = "Enable Padding Removal For Short IP Frames"]
    #[inline(always)]
    pub const fn set_padrem(&mut self, val: super::vals::Padrem) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
    #[must_use]
    #[inline(always)]
    pub const fn ipdis(&self) -> super::vals::Ipdis {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ipdis::from_bits(val as u8)
    }
    #[doc = "Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
    #[inline(always)]
    pub const fn set_ipdis(&mut self, val: super::vals::Ipdis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Discard Of Frames With Wrong Protocol Checksum"]
    #[must_use]
    #[inline(always)]
    pub const fn prodis(&self) -> super::vals::Prodis {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Prodis::from_bits(val as u8)
    }
    #[doc = "Enable Discard Of Frames With Wrong Protocol Checksum"]
    #[inline(always)]
    pub const fn set_prodis(&mut self, val: super::vals::Prodis) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Discard Of Frames With MAC Layer Errors"]
    #[must_use]
    #[inline(always)]
    pub const fn linedis(&self) -> super::vals::Linedis {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Linedis::from_bits(val as u8)
    }
    #[doc = "Enable Discard Of Frames With MAC Layer Errors"]
    #[inline(always)]
    pub const fn set_linedis(&mut self, val: super::vals::Linedis) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "RX FIFO Shift-16"]
    #[must_use]
    #[inline(always)]
    pub const fn shift16(&self) -> super::vals::RaccShift16 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::RaccShift16::from_bits(val as u8)
    }
    #[doc = "RX FIFO Shift-16"]
    #[inline(always)]
    pub const fn set_shift16(&mut self, val: super::vals::RaccShift16) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Racc {
    #[inline(always)]
    fn default() -> Racc {
        Racc(0)
    }
}
impl core::fmt::Debug for Racc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Racc")
            .field("padrem", &self.padrem())
            .field("ipdis", &self.ipdis())
            .field("prodis", &self.prodis())
            .field("linedis", &self.linedis())
            .field("shift16", &self.shift16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Racc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Racc {{ padrem: {:?}, ipdis: {:?}, prodis: {:?}, linedis: {:?}, shift16: {:?} }}",
            self.padrem(),
            self.ipdis(),
            self.prodis(),
            self.linedis(),
            self.shift16()
        )
    }
}
#[doc = "Receive FIFO Almost Empty Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Raem(pub u32);
impl Raem {
    #[doc = "Value Of The Receive FIFO Almost Empty Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_almost_empty(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value Of The Receive FIFO Almost Empty Threshold"]
    #[inline(always)]
    pub const fn set_rx_almost_empty(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Raem {
    #[inline(always)]
    fn default() -> Raem {
        Raem(0)
    }
}
impl core::fmt::Debug for Raem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Raem")
            .field("rx_almost_empty", &self.rx_almost_empty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Raem {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Raem {{ rx_almost_empty: {=u8:?} }}",
            self.rx_almost_empty()
        )
    }
}
#[doc = "Receive FIFO Almost Full Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rafl(pub u32);
impl Rafl {
    #[doc = "Value Of The Receive FIFO Almost Full Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_almost_full(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value Of The Receive FIFO Almost Full Threshold"]
    #[inline(always)]
    pub const fn set_rx_almost_full(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rafl {
    #[inline(always)]
    fn default() -> Rafl {
        Rafl(0)
    }
}
impl core::fmt::Debug for Rafl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rafl")
            .field("rx_almost_full", &self.rx_almost_full())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rafl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rafl {{ rx_almost_full: {=u8:?} }}",
            self.rx_almost_full()
        )
    }
}
#[doc = "Receive Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr(pub u32);
impl Rcr {
    #[doc = "Internal Loopback"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Loop {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Loop::from_bits(val as u8)
    }
    #[doc = "Internal Loopback"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Loop) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Disable Receive On Transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn drt(&self) -> super::vals::Drt {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Drt::from_bits(val as u8)
    }
    #[doc = "Disable Receive On Transmit"]
    #[inline(always)]
    pub const fn set_drt(&mut self, val: super::vals::Drt) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Media Independent Interface Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mii_mode(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Media Independent Interface Mode"]
    #[inline(always)]
    pub const fn set_mii_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Promiscuous Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn prom(&self) -> super::vals::Prom {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Prom::from_bits(val as u8)
    }
    #[doc = "Promiscuous Mode"]
    #[inline(always)]
    pub const fn set_prom(&mut self, val: super::vals::Prom) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Broadcast Frame Reject"]
    #[must_use]
    #[inline(always)]
    pub const fn bc_rej(&self) -> super::vals::BcRej {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::BcRej::from_bits(val as u8)
    }
    #[doc = "Broadcast Frame Reject"]
    #[inline(always)]
    pub const fn set_bc_rej(&mut self, val: super::vals::BcRej) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Flow Control Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fce(&self) -> super::vals::Fce {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Fce::from_bits(val as u8)
    }
    #[doc = "Flow Control Enable"]
    #[inline(always)]
    pub const fn set_fce(&mut self, val: super::vals::Fce) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "RMII Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rmii_mode(&self) -> super::vals::RmiiMode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::RmiiMode::from_bits(val as u8)
    }
    #[doc = "RMII Mode Enable"]
    #[inline(always)]
    pub const fn set_rmii_mode(&mut self, val: super::vals::RmiiMode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables 10-Mbit/s mode of the RMII ."]
    #[must_use]
    #[inline(always)]
    pub const fn rmii_10t(&self) -> super::vals::Rmii10t {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Rmii10t::from_bits(val as u8)
    }
    #[doc = "Enables 10-Mbit/s mode of the RMII ."]
    #[inline(always)]
    pub const fn set_rmii_10t(&mut self, val: super::vals::Rmii10t) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable Frame Padding Remove On Receive"]
    #[must_use]
    #[inline(always)]
    pub const fn paden(&self) -> super::vals::Paden {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Paden::from_bits(val as u8)
    }
    #[doc = "Enable Frame Padding Remove On Receive"]
    #[inline(always)]
    pub const fn set_paden(&mut self, val: super::vals::Paden) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Terminate/Forward Pause Frames"]
    #[must_use]
    #[inline(always)]
    pub const fn paufwd(&self) -> super::vals::Paufwd {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Paufwd::from_bits(val as u8)
    }
    #[doc = "Terminate/Forward Pause Frames"]
    #[inline(always)]
    pub const fn set_paufwd(&mut self, val: super::vals::Paufwd) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Terminate/Forward Received CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn crcfwd(&self) -> super::vals::RcrCrcfwd {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::RcrCrcfwd::from_bits(val as u8)
    }
    #[doc = "Terminate/Forward Received CRC"]
    #[inline(always)]
    pub const fn set_crcfwd(&mut self, val: super::vals::RcrCrcfwd) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "MAC Control Frame Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cfen(&self) -> super::vals::Cfen {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Cfen::from_bits(val as u8)
    }
    #[doc = "MAC Control Frame Enable"]
    #[inline(always)]
    pub const fn set_cfen(&mut self, val: super::vals::Cfen) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Maximum Frame Length"]
    #[must_use]
    #[inline(always)]
    pub const fn max_fl(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "Maximum Frame Length"]
    #[inline(always)]
    pub const fn set_max_fl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
    #[doc = "Payload Length Check Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn nlc(&self) -> super::vals::Nlc {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Nlc::from_bits(val as u8)
    }
    #[doc = "Payload Length Check Disable"]
    #[inline(always)]
    pub const fn set_nlc(&mut self, val: super::vals::Nlc) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Graceful Receive Stopped"]
    #[must_use]
    #[inline(always)]
    pub const fn grs(&self) -> super::vals::Grs {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Grs::from_bits(val as u8)
    }
    #[doc = "Graceful Receive Stopped"]
    #[inline(always)]
    pub const fn set_grs(&mut self, val: super::vals::Grs) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Rcr {
    #[inline(always)]
    fn default() -> Rcr {
        Rcr(0)
    }
}
impl core::fmt::Debug for Rcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr")
            .field("loop_", &self.loop_())
            .field("drt", &self.drt())
            .field("mii_mode", &self.mii_mode())
            .field("prom", &self.prom())
            .field("bc_rej", &self.bc_rej())
            .field("fce", &self.fce())
            .field("rmii_mode", &self.rmii_mode())
            .field("rmii_10t", &self.rmii_10t())
            .field("paden", &self.paden())
            .field("paufwd", &self.paufwd())
            .field("crcfwd", &self.crcfwd())
            .field("cfen", &self.cfen())
            .field("max_fl", &self.max_fl())
            .field("nlc", &self.nlc())
            .field("grs", &self.grs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcr {{ loop_: {:?}, drt: {:?}, mii_mode: {=bool:?}, prom: {:?}, bc_rej: {:?}, fce: {:?}, rmii_mode: {:?}, rmii_10t: {:?}, paden: {:?}, paufwd: {:?}, crcfwd: {:?}, cfen: {:?}, max_fl: {=u16:?}, nlc: {:?}, grs: {:?} }}",
            self.loop_(),
            self.drt(),
            self.mii_mode(),
            self.prom(),
            self.bc_rej(),
            self.fce(),
            self.rmii_mode(),
            self.rmii_10t(),
            self.paden(),
            self.paufwd(),
            self.crcfwd(),
            self.cfen(),
            self.max_fl(),
            self.nlc(),
            self.grs()
        )
    }
}
#[doc = "Receive Descriptor Active Register - Ring 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdar(pub u32);
impl Rdar {
    #[doc = "Receive Descriptor Active"]
    #[must_use]
    #[inline(always)]
    pub const fn rdar(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Descriptor Active"]
    #[inline(always)]
    pub const fn set_rdar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Rdar {
    #[inline(always)]
    fn default() -> Rdar {
        Rdar(0)
    }
}
impl core::fmt::Debug for Rdar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdar").field("rdar", &self.rdar()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rdar {{ rdar: {=bool:?} }}", self.rdar())
    }
}
#[doc = "Receive Descriptor Ring 0 Start Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdsr(pub u32);
impl Rdsr {
    #[doc = "Pointer to the beginning of the receive buffer descriptor queue."]
    #[must_use]
    #[inline(always)]
    pub const fn r_des_start(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Pointer to the beginning of the receive buffer descriptor queue."]
    #[inline(always)]
    pub const fn set_r_des_start(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Rdsr {
    #[inline(always)]
    fn default() -> Rdsr {
        Rdsr(0)
    }
}
impl core::fmt::Debug for Rdsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdsr")
            .field("r_des_start", &self.r_des_start())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rdsr {{ r_des_start: {=u32:?} }}", self.r_des_start())
    }
}
#[doc = "Rx Broadcast Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRBcPkt(pub u32);
impl RmonRBcPkt {
    #[doc = "Number of receive broadcast packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of receive broadcast packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRBcPkt {
    #[inline(always)]
    fn default() -> RmonRBcPkt {
        RmonRBcPkt(0)
    }
}
impl core::fmt::Debug for RmonRBcPkt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRBcPkt")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRBcPkt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRBcPkt {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx Packets with CRC/Align Error Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRCrcAlign(pub u32);
impl RmonRCrcAlign {
    #[doc = "Number of receive packets with CRC or align error"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of receive packets with CRC or align error"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRCrcAlign {
    #[inline(always)]
    fn default() -> RmonRCrcAlign {
        RmonRCrcAlign(0)
    }
}
impl core::fmt::Debug for RmonRCrcAlign {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRCrcAlign")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRCrcAlign {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRCrcAlign {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRFrag(pub u32);
impl RmonRFrag {
    #[doc = "Number of receive packets with less than 64 bytes and bad CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of receive packets with less than 64 bytes and bad CRC"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRFrag {
    #[inline(always)]
    fn default() -> RmonRFrag {
        RmonRFrag(0)
    }
}
impl core::fmt::Debug for RmonRFrag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRFrag")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRFrag {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRFrag {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRJab(pub u32);
impl RmonRJab {
    #[doc = "Number of receive packets greater than MAX_FL and bad CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of receive packets greater than MAX_FL and bad CRC"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRJab {
    #[inline(always)]
    fn default() -> RmonRJab {
        RmonRJab(0)
    }
}
impl core::fmt::Debug for RmonRJab {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRJab")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRJab {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRJab {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx Multicast Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRMcPkt(pub u32);
impl RmonRMcPkt {
    #[doc = "Number of receive multicast packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of receive multicast packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRMcPkt {
    #[inline(always)]
    fn default() -> RmonRMcPkt {
        RmonRMcPkt(0)
    }
}
impl core::fmt::Debug for RmonRMcPkt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRMcPkt")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRMcPkt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRMcPkt {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx Octets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonROctets(pub u32);
impl RmonROctets {
    #[doc = "Number of receive octets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Number of receive octets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RmonROctets {
    #[inline(always)]
    fn default() -> RmonROctets {
        RmonROctets(0)
    }
}
impl core::fmt::Debug for RmonROctets {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonROctets")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonROctets {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonROctets {{ count: {=u32:?} }}", self.count())
    }
}
#[doc = "Rx Packets Greater Than MAX_FL and Good CRC Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonROversize(pub u32);
impl RmonROversize {
    #[doc = "Number of receive packets greater than MAX_FL and good CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of receive packets greater than MAX_FL and good CRC"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonROversize {
    #[inline(always)]
    fn default() -> RmonROversize {
        RmonROversize(0)
    }
}
impl core::fmt::Debug for RmonROversize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonROversize")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonROversize {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonROversize {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx 1024- to 2047-Byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRP1024to2047(pub u32);
impl RmonRP1024to2047 {
    #[doc = "Number of 1024- to 2047-byte recieve packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 1024- to 2047-byte recieve packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRP1024to2047 {
    #[inline(always)]
    fn default() -> RmonRP1024to2047 {
        RmonRP1024to2047(0)
    }
}
impl core::fmt::Debug for RmonRP1024to2047 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRP1024to2047")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRP1024to2047 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRP1024to2047 {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx 128- to 255-Byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRP128to255(pub u32);
impl RmonRP128to255 {
    #[doc = "Number of 128- to 255-byte recieve packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 128- to 255-byte recieve packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRP128to255 {
    #[inline(always)]
    fn default() -> RmonRP128to255 {
        RmonRP128to255(0)
    }
}
impl core::fmt::Debug for RmonRP128to255 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRP128to255")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRP128to255 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRP128to255 {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx 256- to 511-Byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRP256to511(pub u32);
impl RmonRP256to511 {
    #[doc = "Number of 256- to 511-byte recieve packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 256- to 511-byte recieve packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRP256to511 {
    #[inline(always)]
    fn default() -> RmonRP256to511 {
        RmonRP256to511(0)
    }
}
impl core::fmt::Debug for RmonRP256to511 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRP256to511")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRP256to511 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRP256to511 {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx 512- to 1023-Byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRP512to1023(pub u32);
impl RmonRP512to1023 {
    #[doc = "Number of 512- to 1023-byte recieve packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 512- to 1023-byte recieve packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRP512to1023 {
    #[inline(always)]
    fn default() -> RmonRP512to1023 {
        RmonRP512to1023(0)
    }
}
impl core::fmt::Debug for RmonRP512to1023 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRP512to1023")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRP512to1023 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRP512to1023 {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx 64-Byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRP64(pub u32);
impl RmonRP64 {
    #[doc = "Number of 64-byte receive packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 64-byte receive packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRP64 {
    #[inline(always)]
    fn default() -> RmonRP64 {
        RmonRP64(0)
    }
}
impl core::fmt::Debug for RmonRP64 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRP64")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRP64 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRP64 {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx 65- to 127-Byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRP65to127(pub u32);
impl RmonRP65to127 {
    #[doc = "Number of 65- to 127-byte recieve packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 65- to 127-byte recieve packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRP65to127 {
    #[inline(always)]
    fn default() -> RmonRP65to127 {
        RmonRP65to127(0)
    }
}
impl core::fmt::Debug for RmonRP65to127 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRP65to127")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRP65to127 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRP65to127 {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx Packets Greater than 2048 Bytes Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRPGte2048(pub u32);
impl RmonRPGte2048 {
    #[doc = "Number of greater-than-2048-byte recieve packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of greater-than-2048-byte recieve packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRPGte2048 {
    #[inline(always)]
    fn default() -> RmonRPGte2048 {
        RmonRPGte2048(0)
    }
}
impl core::fmt::Debug for RmonRPGte2048 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRPGte2048")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRPGte2048 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRPGte2048 {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx Packet Count Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRPackets(pub u32);
impl RmonRPackets {
    #[doc = "Number of packets received"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of packets received"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRPackets {
    #[inline(always)]
    fn default() -> RmonRPackets {
        RmonRPackets(0)
    }
}
impl core::fmt::Debug for RmonRPackets {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRPackets")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRPackets {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRPackets {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRUndersize(pub u32);
impl RmonRUndersize {
    #[doc = "Number of receive packets with less than 64 bytes and good CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of receive packets with less than 64 bytes and good CRC"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRUndersize {
    #[inline(always)]
    fn default() -> RmonRUndersize {
        RmonRUndersize(0)
    }
}
impl core::fmt::Debug for RmonRUndersize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRUndersize")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRUndersize {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRUndersize {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Tx Broadcast Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTBcPkt(pub u32);
impl RmonTBcPkt {
    #[doc = "Number of broadcast packets"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of broadcast packets"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTBcPkt {
    #[inline(always)]
    fn default() -> RmonTBcPkt {
        RmonTBcPkt(0)
    }
}
impl core::fmt::Debug for RmonTBcPkt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTBcPkt")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTBcPkt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTBcPkt {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Collision Count Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTCol(pub u32);
impl RmonTCol {
    #[doc = "Number of transmit collisions"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of transmit collisions"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTCol {
    #[inline(always)]
    fn default() -> RmonTCol {
        RmonTCol(0)
    }
}
impl core::fmt::Debug for RmonTCol {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTCol")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTCol {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTCol {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Packets with CRC/Align Error Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTCrcAlign(pub u32);
impl RmonTCrcAlign {
    #[doc = "Number of packets with CRC/align error"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of packets with CRC/align error"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTCrcAlign {
    #[inline(always)]
    fn default() -> RmonTCrcAlign {
        RmonTCrcAlign(0)
    }
}
impl core::fmt::Debug for RmonTCrcAlign {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTCrcAlign")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTCrcAlign {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTCrcAlign {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTFrag(pub u32);
impl RmonTFrag {
    #[doc = "Number of packets less than 64 bytes with bad CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of packets less than 64 bytes with bad CRC"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTFrag {
    #[inline(always)]
    fn default() -> RmonTFrag {
        RmonTFrag(0)
    }
}
impl core::fmt::Debug for RmonTFrag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTFrag")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTFrag {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTFrag {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTJab(pub u32);
impl RmonTJab {
    #[doc = "Number of transmit packets greater than MAX_FL bytes and bad CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of transmit packets greater than MAX_FL bytes and bad CRC"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTJab {
    #[inline(always)]
    fn default() -> RmonTJab {
        RmonTJab(0)
    }
}
impl core::fmt::Debug for RmonTJab {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTJab")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTJab {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTJab {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Multicast Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTMcPkt(pub u32);
impl RmonTMcPkt {
    #[doc = "Number of multicast packets"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of multicast packets"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTMcPkt {
    #[inline(always)]
    fn default() -> RmonTMcPkt {
        RmonTMcPkt(0)
    }
}
impl core::fmt::Debug for RmonTMcPkt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTMcPkt")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTMcPkt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTMcPkt {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Octets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTOctets(pub u32);
impl RmonTOctets {
    #[doc = "Number of transmit octets"]
    #[must_use]
    #[inline(always)]
    pub const fn txocts(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Number of transmit octets"]
    #[inline(always)]
    pub const fn set_txocts(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RmonTOctets {
    #[inline(always)]
    fn default() -> RmonTOctets {
        RmonTOctets(0)
    }
}
impl core::fmt::Debug for RmonTOctets {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTOctets")
            .field("txocts", &self.txocts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTOctets {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTOctets {{ txocts: {=u32:?} }}", self.txocts())
    }
}
#[doc = "Tx Packets GT MAX_FL bytes and Good CRC Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTOversize(pub u32);
impl RmonTOversize {
    #[doc = "Number of transmit packets greater than MAX_FL bytes with good CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of transmit packets greater than MAX_FL bytes with good CRC"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTOversize {
    #[inline(always)]
    fn default() -> RmonTOversize {
        RmonTOversize(0)
    }
}
impl core::fmt::Debug for RmonTOversize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTOversize")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTOversize {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTOversize {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx 1024- to 2047-byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTP1024to2047(pub u32);
impl RmonTP1024to2047 {
    #[doc = "Number of 1024- to 2047-byte transmit packets"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 1024- to 2047-byte transmit packets"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTP1024to2047 {
    #[inline(always)]
    fn default() -> RmonTP1024to2047 {
        RmonTP1024to2047(0)
    }
}
impl core::fmt::Debug for RmonTP1024to2047 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTP1024to2047")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTP1024to2047 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTP1024to2047 {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx 128- to 255-byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTP128to255(pub u32);
impl RmonTP128to255 {
    #[doc = "Number of 128- to 255-byte transmit packets"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 128- to 255-byte transmit packets"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTP128to255 {
    #[inline(always)]
    fn default() -> RmonTP128to255 {
        RmonTP128to255(0)
    }
}
impl core::fmt::Debug for RmonTP128to255 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTP128to255")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTP128to255 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTP128to255 {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx 256- to 511-byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTP256to511(pub u32);
impl RmonTP256to511 {
    #[doc = "Number of 256- to 511-byte transmit packets"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 256- to 511-byte transmit packets"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTP256to511 {
    #[inline(always)]
    fn default() -> RmonTP256to511 {
        RmonTP256to511(0)
    }
}
impl core::fmt::Debug for RmonTP256to511 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTP256to511")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTP256to511 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTP256to511 {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx 512- to 1023-byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTP512to1023(pub u32);
impl RmonTP512to1023 {
    #[doc = "Number of 512- to 1023-byte transmit packets"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 512- to 1023-byte transmit packets"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTP512to1023 {
    #[inline(always)]
    fn default() -> RmonTP512to1023 {
        RmonTP512to1023(0)
    }
}
impl core::fmt::Debug for RmonTP512to1023 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTP512to1023")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTP512to1023 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTP512to1023 {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx 64-Byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTP64(pub u32);
impl RmonTP64 {
    #[doc = "Number of 64-byte transmit packets"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 64-byte transmit packets"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTP64 {
    #[inline(always)]
    fn default() -> RmonTP64 {
        RmonTP64(0)
    }
}
impl core::fmt::Debug for RmonTP64 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTP64")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTP64 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTP64 {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx 65- to 127-byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTP65to127(pub u32);
impl RmonTP65to127 {
    #[doc = "Number of 65- to 127-byte transmit packets"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 65- to 127-byte transmit packets"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTP65to127 {
    #[inline(always)]
    fn default() -> RmonTP65to127 {
        RmonTP65to127(0)
    }
}
impl core::fmt::Debug for RmonTP65to127 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTP65to127")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTP65to127 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTP65to127 {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Packets Greater Than 2048 Bytes Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTPGte2048(pub u32);
impl RmonTPGte2048 {
    #[doc = "Number of transmit packets greater than 2048 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of transmit packets greater than 2048 bytes"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTPGte2048 {
    #[inline(always)]
    fn default() -> RmonTPGte2048 {
        RmonTPGte2048(0)
    }
}
impl core::fmt::Debug for RmonTPGte2048 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTPGte2048")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTPGte2048 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTPGte2048 {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Packet Count Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTPackets(pub u32);
impl RmonTPackets {
    #[doc = "Packet count"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Packet count"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTPackets {
    #[inline(always)]
    fn default() -> RmonTPackets {
        RmonTPackets(0)
    }
}
impl core::fmt::Debug for RmonTPackets {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTPackets")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTPackets {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTPackets {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Packets Less Than Bytes and Good CRC Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTUndersize(pub u32);
impl RmonTUndersize {
    #[doc = "Number of transmit packets less than 64 bytes with good CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of transmit packets less than 64 bytes with good CRC"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTUndersize {
    #[inline(always)]
    fn default() -> RmonTUndersize {
        RmonTUndersize(0)
    }
}
impl core::fmt::Debug for RmonTUndersize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTUndersize")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTUndersize {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTUndersize {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Receive FIFO Section Empty Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsem(pub u32);
impl Rsem {
    #[doc = "Value Of The Receive FIFO Section Empty Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_section_empty(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value Of The Receive FIFO Section Empty Threshold"]
    #[inline(always)]
    pub const fn set_rx_section_empty(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "RX Status FIFO Section Empty Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn stat_section_empty(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "RX Status FIFO Section Empty Threshold"]
    #[inline(always)]
    pub const fn set_stat_section_empty(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Rsem {
    #[inline(always)]
    fn default() -> Rsem {
        Rsem(0)
    }
}
impl core::fmt::Debug for Rsem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsem")
            .field("rx_section_empty", &self.rx_section_empty())
            .field("stat_section_empty", &self.stat_section_empty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsem {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rsem {{ rx_section_empty: {=u8:?}, stat_section_empty: {=u8:?} }}",
            self.rx_section_empty(),
            self.stat_section_empty()
        )
    }
}
#[doc = "Receive FIFO Section Full Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsfl(pub u32);
impl Rsfl {
    #[doc = "Value Of Receive FIFO Section Full Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_section_full(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value Of Receive FIFO Section Full Threshold"]
    #[inline(always)]
    pub const fn set_rx_section_full(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rsfl {
    #[inline(always)]
    fn default() -> Rsfl {
        Rsfl(0)
    }
}
impl core::fmt::Debug for Rsfl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsfl")
            .field("rx_section_full", &self.rx_section_full())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsfl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rsfl {{ rx_section_full: {=u8:?} }}",
            self.rx_section_full()
        )
    }
}
#[doc = "Receive Interrupt Coalescing Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxic0(pub u32);
impl Rxic0 {
    #[doc = "Interrupt coalescing timer threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn ictt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Interrupt coalescing timer threshold"]
    #[inline(always)]
    pub const fn set_ictt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Interrupt coalescing frame count threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn icft(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0xff;
        val as u8
    }
    #[doc = "Interrupt coalescing frame count threshold"]
    #[inline(always)]
    pub const fn set_icft(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
    }
    #[doc = "Interrupt Coalescing Timer Clock Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn iccs(&self) -> super::vals::Rxic0Iccs {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Rxic0Iccs::from_bits(val as u8)
    }
    #[doc = "Interrupt Coalescing Timer Clock Source Select"]
    #[inline(always)]
    pub const fn set_iccs(&mut self, val: super::vals::Rxic0Iccs) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt Coalescing Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn icen(&self) -> super::vals::Rxic0Icen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Rxic0Icen::from_bits(val as u8)
    }
    #[doc = "Interrupt Coalescing Enable"]
    #[inline(always)]
    pub const fn set_icen(&mut self, val: super::vals::Rxic0Icen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Rxic0 {
    #[inline(always)]
    fn default() -> Rxic0 {
        Rxic0(0)
    }
}
impl core::fmt::Debug for Rxic0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxic0")
            .field("ictt", &self.ictt())
            .field("icft", &self.icft())
            .field("iccs", &self.iccs())
            .field("icen", &self.icen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxic0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rxic0 {{ ictt: {=u16:?}, icft: {=u8:?}, iccs: {:?}, icen: {:?} }}",
            self.ictt(),
            self.icft(),
            self.iccs(),
            self.icen()
        )
    }
}
#[doc = "Transmit Accelerator Function Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tacc(pub u32);
impl Tacc {
    #[doc = "TX FIFO Shift-16"]
    #[must_use]
    #[inline(always)]
    pub const fn shift16(&self) -> super::vals::TaccShift16 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::TaccShift16::from_bits(val as u8)
    }
    #[doc = "TX FIFO Shift-16"]
    #[inline(always)]
    pub const fn set_shift16(&mut self, val: super::vals::TaccShift16) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables insertion of IP header checksum."]
    #[must_use]
    #[inline(always)]
    pub const fn ipchk(&self) -> super::vals::Ipchk {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ipchk::from_bits(val as u8)
    }
    #[doc = "Enables insertion of IP header checksum."]
    #[inline(always)]
    pub const fn set_ipchk(&mut self, val: super::vals::Ipchk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables insertion of protocol checksum."]
    #[must_use]
    #[inline(always)]
    pub const fn prochk(&self) -> super::vals::Prochk {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Prochk::from_bits(val as u8)
    }
    #[doc = "Enables insertion of protocol checksum."]
    #[inline(always)]
    pub const fn set_prochk(&mut self, val: super::vals::Prochk) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for Tacc {
    #[inline(always)]
    fn default() -> Tacc {
        Tacc(0)
    }
}
impl core::fmt::Debug for Tacc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tacc")
            .field("shift16", &self.shift16())
            .field("ipchk", &self.ipchk())
            .field("prochk", &self.prochk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tacc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tacc {{ shift16: {:?}, ipchk: {:?}, prochk: {:?} }}",
            self.shift16(),
            self.ipchk(),
            self.prochk()
        )
    }
}
#[doc = "Transmit FIFO Almost Empty Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Taem(pub u32);
impl Taem {
    #[doc = "Value of Transmit FIFO Almost Empty Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_almost_empty(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value of Transmit FIFO Almost Empty Threshold"]
    #[inline(always)]
    pub const fn set_tx_almost_empty(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Taem {
    #[inline(always)]
    fn default() -> Taem {
        Taem(0)
    }
}
impl core::fmt::Debug for Taem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Taem")
            .field("tx_almost_empty", &self.tx_almost_empty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Taem {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Taem {{ tx_almost_empty: {=u8:?} }}",
            self.tx_almost_empty()
        )
    }
}
#[doc = "Transmit FIFO Almost Full Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tafl(pub u32);
impl Tafl {
    #[doc = "Value Of The Transmit FIFO Almost Full Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_almost_full(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value Of The Transmit FIFO Almost Full Threshold"]
    #[inline(always)]
    pub const fn set_tx_almost_full(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Tafl {
    #[inline(always)]
    fn default() -> Tafl {
        Tafl(0)
    }
}
impl core::fmt::Debug for Tafl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tafl")
            .field("tx_almost_full", &self.tx_almost_full())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tafl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tafl {{ tx_almost_full: {=u8:?} }}",
            self.tx_almost_full()
        )
    }
}
#[doc = "Timer Compare Capture Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tccr0(pub u32);
impl Tccr0 {
    #[doc = "Timer Capture Compare"]
    #[must_use]
    #[inline(always)]
    pub const fn tcc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer Capture Compare"]
    #[inline(always)]
    pub const fn set_tcc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tccr0 {
    #[inline(always)]
    fn default() -> Tccr0 {
        Tccr0(0)
    }
}
impl core::fmt::Debug for Tccr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tccr0").field("tcc", &self.tcc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tccr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tccr0 {{ tcc: {=u32:?} }}", self.tcc())
    }
}
#[doc = "Timer Compare Capture Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tccr1(pub u32);
impl Tccr1 {
    #[doc = "Timer Capture Compare"]
    #[must_use]
    #[inline(always)]
    pub const fn tcc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer Capture Compare"]
    #[inline(always)]
    pub const fn set_tcc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tccr1 {
    #[inline(always)]
    fn default() -> Tccr1 {
        Tccr1(0)
    }
}
impl core::fmt::Debug for Tccr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tccr1").field("tcc", &self.tcc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tccr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tccr1 {{ tcc: {=u32:?} }}", self.tcc())
    }
}
#[doc = "Timer Compare Capture Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tccr2(pub u32);
impl Tccr2 {
    #[doc = "Timer Capture Compare"]
    #[must_use]
    #[inline(always)]
    pub const fn tcc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer Capture Compare"]
    #[inline(always)]
    pub const fn set_tcc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tccr2 {
    #[inline(always)]
    fn default() -> Tccr2 {
        Tccr2(0)
    }
}
impl core::fmt::Debug for Tccr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tccr2").field("tcc", &self.tcc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tccr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tccr2 {{ tcc: {=u32:?} }}", self.tcc())
    }
}
#[doc = "Timer Compare Capture Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tccr3(pub u32);
impl Tccr3 {
    #[doc = "Timer Capture Compare"]
    #[must_use]
    #[inline(always)]
    pub const fn tcc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer Capture Compare"]
    #[inline(always)]
    pub const fn set_tcc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tccr3 {
    #[inline(always)]
    fn default() -> Tccr3 {
        Tccr3(0)
    }
}
impl core::fmt::Debug for Tccr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tccr3").field("tcc", &self.tcc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tccr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tccr3 {{ tcc: {=u32:?} }}", self.tcc())
    }
}
#[doc = "Transmit Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc = "Graceful Transmit Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn gts(&self) -> super::vals::Gts {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gts::from_bits(val as u8)
    }
    #[doc = "Graceful Transmit Stop"]
    #[inline(always)]
    pub const fn set_gts(&mut self, val: super::vals::Gts) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Full-Duplex Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fden(&self) -> super::vals::Fden {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Fden::from_bits(val as u8)
    }
    #[doc = "Full-Duplex Enable"]
    #[inline(always)]
    pub const fn set_fden(&mut self, val: super::vals::Fden) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit Frame Control Pause"]
    #[must_use]
    #[inline(always)]
    pub const fn tfc_pause(&self) -> super::vals::TfcPause {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::TfcPause::from_bits(val as u8)
    }
    #[doc = "Transmit Frame Control Pause"]
    #[inline(always)]
    pub const fn set_tfc_pause(&mut self, val: super::vals::TfcPause) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive Frame Control Pause"]
    #[must_use]
    #[inline(always)]
    pub const fn rfc_pause(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Frame Control Pause"]
    #[inline(always)]
    pub const fn set_rfc_pause(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Source MAC Address Select On Transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn addsel(&self) -> super::vals::Addsel {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::Addsel::from_bits(val as u8)
    }
    #[doc = "Source MAC Address Select On Transmit"]
    #[inline(always)]
    pub const fn set_addsel(&mut self, val: super::vals::Addsel) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
    #[doc = "Set MAC Address On Transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn addins(&self) -> super::vals::Addins {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Addins::from_bits(val as u8)
    }
    #[doc = "Set MAC Address On Transmit"]
    #[inline(always)]
    pub const fn set_addins(&mut self, val: super::vals::Addins) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Forward Frame From Application With CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn crcfwd(&self) -> super::vals::TcrCrcfwd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::TcrCrcfwd::from_bits(val as u8)
    }
    #[doc = "Forward Frame From Application With CRC"]
    #[inline(always)]
    pub const fn set_crcfwd(&mut self, val: super::vals::TcrCrcfwd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Tcr {
    #[inline(always)]
    fn default() -> Tcr {
        Tcr(0)
    }
}
impl core::fmt::Debug for Tcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr")
            .field("gts", &self.gts())
            .field("fden", &self.fden())
            .field("tfc_pause", &self.tfc_pause())
            .field("rfc_pause", &self.rfc_pause())
            .field("addsel", &self.addsel())
            .field("addins", &self.addins())
            .field("crcfwd", &self.crcfwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcr {{ gts: {:?}, fden: {:?}, tfc_pause: {:?}, rfc_pause: {=bool:?}, addsel: {:?}, addins: {:?}, crcfwd: {:?} }}",
            self.gts(),
            self.fden(),
            self.tfc_pause(),
            self.rfc_pause(),
            self.addsel(),
            self.addins(),
            self.crcfwd()
        )
    }
}
#[doc = "Timer Control Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcsr0(pub u32);
impl Tcsr0 {
    #[doc = "Timer DMA Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdre(&self) -> super::vals::Tcsr0Tdre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tcsr0Tdre::from_bits(val as u8)
    }
    #[doc = "Timer DMA Request Enable"]
    #[inline(always)]
    pub const fn set_tdre(&mut self, val: super::vals::Tcsr0Tdre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Timer Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn tmode(&self) -> super::vals::Tcsr0Tmode {
        let val = (self.0 >> 2usize) & 0x0f;
        super::vals::Tcsr0Tmode::from_bits(val as u8)
    }
    #[doc = "Timer Mode"]
    #[inline(always)]
    pub const fn set_tmode(&mut self, val: super::vals::Tcsr0Tmode) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val.to_bits() as u32) & 0x0f) << 2usize);
    }
    #[doc = "Timer Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> super::vals::Tcsr0Tie {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Tcsr0Tie::from_bits(val as u8)
    }
    #[doc = "Timer Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: super::vals::Tcsr0Tie) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Timer Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tf(&self) -> super::vals::Tcsr0Tf {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Tcsr0Tf::from_bits(val as u8)
    }
    #[doc = "Timer Flag"]
    #[inline(always)]
    pub const fn set_tf(&mut self, val: super::vals::Tcsr0Tf) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer PulseWidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tpwc(&self) -> super::vals::Tcsr0Tpwc {
        let val = (self.0 >> 11usize) & 0x1f;
        super::vals::Tcsr0Tpwc::from_bits(val as u8)
    }
    #[doc = "Timer PulseWidth Control"]
    #[inline(always)]
    pub const fn set_tpwc(&mut self, val: super::vals::Tcsr0Tpwc) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val.to_bits() as u32) & 0x1f) << 11usize);
    }
}
impl Default for Tcsr0 {
    #[inline(always)]
    fn default() -> Tcsr0 {
        Tcsr0(0)
    }
}
impl core::fmt::Debug for Tcsr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcsr0")
            .field("tdre", &self.tdre())
            .field("tmode", &self.tmode())
            .field("tie", &self.tie())
            .field("tf", &self.tf())
            .field("tpwc", &self.tpwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcsr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcsr0 {{ tdre: {:?}, tmode: {:?}, tie: {:?}, tf: {:?}, tpwc: {:?} }}",
            self.tdre(),
            self.tmode(),
            self.tie(),
            self.tf(),
            self.tpwc()
        )
    }
}
#[doc = "Timer Control Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcsr1(pub u32);
impl Tcsr1 {
    #[doc = "Timer DMA Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdre(&self) -> super::vals::Tcsr1Tdre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tcsr1Tdre::from_bits(val as u8)
    }
    #[doc = "Timer DMA Request Enable"]
    #[inline(always)]
    pub const fn set_tdre(&mut self, val: super::vals::Tcsr1Tdre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Timer Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn tmode(&self) -> super::vals::Tcsr1Tmode {
        let val = (self.0 >> 2usize) & 0x0f;
        super::vals::Tcsr1Tmode::from_bits(val as u8)
    }
    #[doc = "Timer Mode"]
    #[inline(always)]
    pub const fn set_tmode(&mut self, val: super::vals::Tcsr1Tmode) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val.to_bits() as u32) & 0x0f) << 2usize);
    }
    #[doc = "Timer Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> super::vals::Tcsr1Tie {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Tcsr1Tie::from_bits(val as u8)
    }
    #[doc = "Timer Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: super::vals::Tcsr1Tie) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Timer Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tf(&self) -> super::vals::Tcsr1Tf {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Tcsr1Tf::from_bits(val as u8)
    }
    #[doc = "Timer Flag"]
    #[inline(always)]
    pub const fn set_tf(&mut self, val: super::vals::Tcsr1Tf) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer PulseWidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tpwc(&self) -> super::vals::Tcsr1Tpwc {
        let val = (self.0 >> 11usize) & 0x1f;
        super::vals::Tcsr1Tpwc::from_bits(val as u8)
    }
    #[doc = "Timer PulseWidth Control"]
    #[inline(always)]
    pub const fn set_tpwc(&mut self, val: super::vals::Tcsr1Tpwc) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val.to_bits() as u32) & 0x1f) << 11usize);
    }
}
impl Default for Tcsr1 {
    #[inline(always)]
    fn default() -> Tcsr1 {
        Tcsr1(0)
    }
}
impl core::fmt::Debug for Tcsr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcsr1")
            .field("tdre", &self.tdre())
            .field("tmode", &self.tmode())
            .field("tie", &self.tie())
            .field("tf", &self.tf())
            .field("tpwc", &self.tpwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcsr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcsr1 {{ tdre: {:?}, tmode: {:?}, tie: {:?}, tf: {:?}, tpwc: {:?} }}",
            self.tdre(),
            self.tmode(),
            self.tie(),
            self.tf(),
            self.tpwc()
        )
    }
}
#[doc = "Timer Control Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcsr2(pub u32);
impl Tcsr2 {
    #[doc = "Timer DMA Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdre(&self) -> super::vals::Tcsr2Tdre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tcsr2Tdre::from_bits(val as u8)
    }
    #[doc = "Timer DMA Request Enable"]
    #[inline(always)]
    pub const fn set_tdre(&mut self, val: super::vals::Tcsr2Tdre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Timer Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn tmode(&self) -> super::vals::Tcsr2Tmode {
        let val = (self.0 >> 2usize) & 0x0f;
        super::vals::Tcsr2Tmode::from_bits(val as u8)
    }
    #[doc = "Timer Mode"]
    #[inline(always)]
    pub const fn set_tmode(&mut self, val: super::vals::Tcsr2Tmode) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val.to_bits() as u32) & 0x0f) << 2usize);
    }
    #[doc = "Timer Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> super::vals::Tcsr2Tie {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Tcsr2Tie::from_bits(val as u8)
    }
    #[doc = "Timer Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: super::vals::Tcsr2Tie) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Timer Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tf(&self) -> super::vals::Tcsr2Tf {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Tcsr2Tf::from_bits(val as u8)
    }
    #[doc = "Timer Flag"]
    #[inline(always)]
    pub const fn set_tf(&mut self, val: super::vals::Tcsr2Tf) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer PulseWidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tpwc(&self) -> super::vals::Tcsr2Tpwc {
        let val = (self.0 >> 11usize) & 0x1f;
        super::vals::Tcsr2Tpwc::from_bits(val as u8)
    }
    #[doc = "Timer PulseWidth Control"]
    #[inline(always)]
    pub const fn set_tpwc(&mut self, val: super::vals::Tcsr2Tpwc) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val.to_bits() as u32) & 0x1f) << 11usize);
    }
}
impl Default for Tcsr2 {
    #[inline(always)]
    fn default() -> Tcsr2 {
        Tcsr2(0)
    }
}
impl core::fmt::Debug for Tcsr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcsr2")
            .field("tdre", &self.tdre())
            .field("tmode", &self.tmode())
            .field("tie", &self.tie())
            .field("tf", &self.tf())
            .field("tpwc", &self.tpwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcsr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcsr2 {{ tdre: {:?}, tmode: {:?}, tie: {:?}, tf: {:?}, tpwc: {:?} }}",
            self.tdre(),
            self.tmode(),
            self.tie(),
            self.tf(),
            self.tpwc()
        )
    }
}
#[doc = "Timer Control Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcsr3(pub u32);
impl Tcsr3 {
    #[doc = "Timer DMA Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdre(&self) -> super::vals::Tcsr3Tdre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tcsr3Tdre::from_bits(val as u8)
    }
    #[doc = "Timer DMA Request Enable"]
    #[inline(always)]
    pub const fn set_tdre(&mut self, val: super::vals::Tcsr3Tdre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Timer Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn tmode(&self) -> super::vals::Tcsr3Tmode {
        let val = (self.0 >> 2usize) & 0x0f;
        super::vals::Tcsr3Tmode::from_bits(val as u8)
    }
    #[doc = "Timer Mode"]
    #[inline(always)]
    pub const fn set_tmode(&mut self, val: super::vals::Tcsr3Tmode) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val.to_bits() as u32) & 0x0f) << 2usize);
    }
    #[doc = "Timer Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> super::vals::Tcsr3Tie {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Tcsr3Tie::from_bits(val as u8)
    }
    #[doc = "Timer Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: super::vals::Tcsr3Tie) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Timer Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tf(&self) -> super::vals::Tcsr3Tf {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Tcsr3Tf::from_bits(val as u8)
    }
    #[doc = "Timer Flag"]
    #[inline(always)]
    pub const fn set_tf(&mut self, val: super::vals::Tcsr3Tf) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer PulseWidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tpwc(&self) -> super::vals::Tcsr3Tpwc {
        let val = (self.0 >> 11usize) & 0x1f;
        super::vals::Tcsr3Tpwc::from_bits(val as u8)
    }
    #[doc = "Timer PulseWidth Control"]
    #[inline(always)]
    pub const fn set_tpwc(&mut self, val: super::vals::Tcsr3Tpwc) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val.to_bits() as u32) & 0x1f) << 11usize);
    }
}
impl Default for Tcsr3 {
    #[inline(always)]
    fn default() -> Tcsr3 {
        Tcsr3(0)
    }
}
impl core::fmt::Debug for Tcsr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcsr3")
            .field("tdre", &self.tdre())
            .field("tmode", &self.tmode())
            .field("tie", &self.tie())
            .field("tf", &self.tf())
            .field("tpwc", &self.tpwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcsr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcsr3 {{ tdre: {:?}, tmode: {:?}, tie: {:?}, tf: {:?}, tpwc: {:?} }}",
            self.tdre(),
            self.tmode(),
            self.tie(),
            self.tf(),
            self.tpwc()
        )
    }
}
#[doc = "Transmit Descriptor Active Register - Ring 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdar(pub u32);
impl Tdar {
    #[doc = "Transmit Descriptor Active"]
    #[must_use]
    #[inline(always)]
    pub const fn tdar(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Descriptor Active"]
    #[inline(always)]
    pub const fn set_tdar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Tdar {
    #[inline(always)]
    fn default() -> Tdar {
        Tdar(0)
    }
}
impl core::fmt::Debug for Tdar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdar").field("tdar", &self.tdar()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdar {{ tdar: {=bool:?} }}", self.tdar())
    }
}
#[doc = "Transmit Buffer Descriptor Ring 0 Start Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdsr(pub u32);
impl Tdsr {
    #[doc = "Pointer to the beginning of the transmit buffer descriptor queue."]
    #[must_use]
    #[inline(always)]
    pub const fn x_des_start(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Pointer to the beginning of the transmit buffer descriptor queue."]
    #[inline(always)]
    pub const fn set_x_des_start(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Tdsr {
    #[inline(always)]
    fn default() -> Tdsr {
        Tdsr(0)
    }
}
impl core::fmt::Debug for Tdsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdsr")
            .field("x_des_start", &self.x_des_start())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdsr {{ x_des_start: {=u32:?} }}", self.x_des_start())
    }
}
#[doc = "Transmit FIFO Watermark Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfwr(pub u32);
impl Tfwr {
    #[doc = "Transmit FIFO Write"]
    #[must_use]
    #[inline(always)]
    pub const fn tfwr(&self) -> super::vals::Tfwr {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Tfwr::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Write"]
    #[inline(always)]
    pub const fn set_tfwr(&mut self, val: super::vals::Tfwr) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Store And Forward Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn strfwd(&self) -> super::vals::Strfwd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Strfwd::from_bits(val as u8)
    }
    #[doc = "Store And Forward Enable"]
    #[inline(always)]
    pub const fn set_strfwd(&mut self, val: super::vals::Strfwd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Tfwr {
    #[inline(always)]
    fn default() -> Tfwr {
        Tfwr(0)
    }
}
impl core::fmt::Debug for Tfwr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tfwr")
            .field("tfwr", &self.tfwr())
            .field("strfwd", &self.strfwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tfwr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tfwr {{ tfwr: {:?}, strfwd: {:?} }}",
            self.tfwr(),
            self.strfwd()
        )
    }
}
#[doc = "Timer Global Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tgsr(pub u32);
impl Tgsr {
    #[doc = "Copy Of Timer Flag For Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn tf0(&self) -> super::vals::Tf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tf0::from_bits(val as u8)
    }
    #[doc = "Copy Of Timer Flag For Channel 0"]
    #[inline(always)]
    pub const fn set_tf0(&mut self, val: super::vals::Tf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Copy Of Timer Flag For Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn tf1(&self) -> super::vals::Tf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tf1::from_bits(val as u8)
    }
    #[doc = "Copy Of Timer Flag For Channel 1"]
    #[inline(always)]
    pub const fn set_tf1(&mut self, val: super::vals::Tf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Copy Of Timer Flag For Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn tf2(&self) -> super::vals::Tf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Tf2::from_bits(val as u8)
    }
    #[doc = "Copy Of Timer Flag For Channel 2"]
    #[inline(always)]
    pub const fn set_tf2(&mut self, val: super::vals::Tf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Copy Of Timer Flag For Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn tf3(&self) -> super::vals::Tf3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Tf3::from_bits(val as u8)
    }
    #[doc = "Copy Of Timer Flag For Channel 3"]
    #[inline(always)]
    pub const fn set_tf3(&mut self, val: super::vals::Tf3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Tgsr {
    #[inline(always)]
    fn default() -> Tgsr {
        Tgsr(0)
    }
}
impl core::fmt::Debug for Tgsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tgsr")
            .field("tf0", &self.tf0())
            .field("tf1", &self.tf1())
            .field("tf2", &self.tf2())
            .field("tf3", &self.tf3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tgsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tgsr {{ tf0: {:?}, tf1: {:?}, tf2: {:?}, tf3: {:?} }}",
            self.tf0(),
            self.tf1(),
            self.tf2(),
            self.tf3()
        )
    }
}
#[doc = "Transmit Inter-Packet Gap"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tipg(pub u32);
impl Tipg {
    #[doc = "Transmit Inter-Packet Gap"]
    #[must_use]
    #[inline(always)]
    pub const fn ipg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Transmit Inter-Packet Gap"]
    #[inline(always)]
    pub const fn set_ipg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Tipg {
    #[inline(always)]
    fn default() -> Tipg {
        Tipg(0)
    }
}
impl core::fmt::Debug for Tipg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tipg").field("ipg", &self.ipg()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tipg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tipg {{ ipg: {=u8:?} }}", self.ipg())
    }
}
#[doc = "Transmit FIFO Section Empty Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsem(pub u32);
impl Tsem {
    #[doc = "Value Of The Transmit FIFO Section Empty Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_section_empty(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value Of The Transmit FIFO Section Empty Threshold"]
    #[inline(always)]
    pub const fn set_tx_section_empty(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Tsem {
    #[inline(always)]
    fn default() -> Tsem {
        Tsem(0)
    }
}
impl core::fmt::Debug for Tsem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsem")
            .field("tx_section_empty", &self.tx_section_empty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsem {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tsem {{ tx_section_empty: {=u8:?} }}",
            self.tx_section_empty()
        )
    }
}
#[doc = "Transmit Interrupt Coalescing Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txic0(pub u32);
impl Txic0 {
    #[doc = "Interrupt coalescing timer threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn ictt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Interrupt coalescing timer threshold"]
    #[inline(always)]
    pub const fn set_ictt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Interrupt coalescing frame count threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn icft(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0xff;
        val as u8
    }
    #[doc = "Interrupt coalescing frame count threshold"]
    #[inline(always)]
    pub const fn set_icft(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
    }
    #[doc = "Interrupt Coalescing Timer Clock Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn iccs(&self) -> super::vals::Txic0Iccs {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Txic0Iccs::from_bits(val as u8)
    }
    #[doc = "Interrupt Coalescing Timer Clock Source Select"]
    #[inline(always)]
    pub const fn set_iccs(&mut self, val: super::vals::Txic0Iccs) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt Coalescing Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn icen(&self) -> super::vals::Txic0Icen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Txic0Icen::from_bits(val as u8)
    }
    #[doc = "Interrupt Coalescing Enable"]
    #[inline(always)]
    pub const fn set_icen(&mut self, val: super::vals::Txic0Icen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Txic0 {
    #[inline(always)]
    fn default() -> Txic0 {
        Txic0(0)
    }
}
impl core::fmt::Debug for Txic0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txic0")
            .field("ictt", &self.ictt())
            .field("icft", &self.icft())
            .field("iccs", &self.iccs())
            .field("icen", &self.icen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txic0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txic0 {{ ictt: {=u16:?}, icft: {=u8:?}, iccs: {:?}, icen: {:?} }}",
            self.ictt(),
            self.icft(),
            self.iccs(),
            self.icen()
        )
    }
}
