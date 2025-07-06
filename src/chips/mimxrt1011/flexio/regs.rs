#[doc = "FlexIO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "FlexIO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn flexen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FlexIO Enable"]
    #[inline(always)]
    pub const fn set_flexen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn swrst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_swrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Fast Access"]
    #[must_use]
    #[inline(always)]
    pub const fn fastacc(&self) -> super::vals::Fastacc {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Fastacc::from_bits(val as u8)
    }
    #[doc = "Fast Access"]
    #[inline(always)]
    pub const fn set_fastacc(&mut self, val: super::vals::Fastacc) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbge(&self) -> super::vals::Dbge {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dbge::from_bits(val as u8)
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbge(&mut self, val: super::vals::Dbge) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Doze Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dozen(&self) -> super::vals::Dozen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Dozen::from_bits(val as u8)
    }
    #[doc = "Doze Enable"]
    #[inline(always)]
    pub const fn set_dozen(&mut self, val: super::vals::Dozen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("flexen", &self.flexen())
            .field("swrst", &self.swrst())
            .field("fastacc", &self.fastacc())
            .field("dbge", &self.dbge())
            .field("dozen", &self.dozen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ flexen: {=bool:?}, swrst: {=bool:?}, fastacc: {:?}, dbge: {:?}, dozen: {:?} }}",
            self.flexen(),
            self.swrst(),
            self.fastacc(),
            self.dbge(),
            self.dozen()
        )
    }
}
#[doc = "Parameter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Shifter Number"]
    #[must_use]
    #[inline(always)]
    pub const fn shifter(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Number"]
    #[inline(always)]
    pub const fn set_shifter(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Timer Number"]
    #[must_use]
    #[inline(always)]
    pub const fn timer(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Timer Number"]
    #[inline(always)]
    pub const fn set_timer(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Pin Number"]
    #[must_use]
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Pin Number"]
    #[inline(always)]
    pub const fn set_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Trigger Number"]
    #[must_use]
    #[inline(always)]
    pub const fn trigger(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Trigger Number"]
    #[inline(always)]
    pub const fn set_trigger(&mut self, val: u8) {
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
            .field("shifter", &self.shifter())
            .field("timer", &self.timer())
            .field("pin", &self.pin())
            .field("trigger", &self.trigger())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ shifter: {=u8:?}, timer: {=u8:?}, pin: {=u8:?}, trigger: {=u8:?} }}",
            self.shifter(),
            self.timer(),
            self.pin(),
            self.trigger()
        )
    }
}
#[doc = "Pin State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pin(pub u32);
impl Pin {
    #[doc = "Pin Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Pin Data Input"]
    #[inline(always)]
    pub const fn set_pdi(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pin {
    #[inline(always)]
    fn default() -> Pin {
        Pin(0)
    }
}
impl core::fmt::Debug for Pin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pin").field("pdi", &self.pdi()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pin {{ pdi: {=u32:?} }}", self.pdi())
    }
}
#[doc = "Shifter Buffer N Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftbuf(pub u32);
impl Shiftbuf {
    #[doc = "Shift Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn shiftbuf(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer"]
    #[inline(always)]
    pub const fn set_shiftbuf(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Shiftbuf {
    #[inline(always)]
    fn default() -> Shiftbuf {
        Shiftbuf(0)
    }
}
impl core::fmt::Debug for Shiftbuf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftbuf")
            .field("shiftbuf", &self.shiftbuf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftbuf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Shiftbuf {{ shiftbuf: {=u32:?} }}", self.shiftbuf())
    }
}
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftbufbbs(pub u32);
impl Shiftbufbbs {
    #[doc = "Shift Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn shiftbufbbs(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer"]
    #[inline(always)]
    pub const fn set_shiftbufbbs(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Shiftbufbbs {
    #[inline(always)]
    fn default() -> Shiftbufbbs {
        Shiftbufbbs(0)
    }
}
impl core::fmt::Debug for Shiftbufbbs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftbufbbs")
            .field("shiftbufbbs", &self.shiftbufbbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftbufbbs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftbufbbs {{ shiftbufbbs: {=u32:?} }}",
            self.shiftbufbbs()
        )
    }
}
#[doc = "Shifter Buffer N Bit Swapped Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftbufbis(pub u32);
impl Shiftbufbis {
    #[doc = "Shift Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn shiftbufbis(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer"]
    #[inline(always)]
    pub const fn set_shiftbufbis(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Shiftbufbis {
    #[inline(always)]
    fn default() -> Shiftbufbis {
        Shiftbufbis(0)
    }
}
impl core::fmt::Debug for Shiftbufbis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftbufbis")
            .field("shiftbufbis", &self.shiftbufbis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftbufbis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftbufbis {{ shiftbufbis: {=u32:?} }}",
            self.shiftbufbis()
        )
    }
}
#[doc = "Shifter Buffer N Byte Swapped Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftbufbys(pub u32);
impl Shiftbufbys {
    #[doc = "Shift Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn shiftbufbys(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer"]
    #[inline(always)]
    pub const fn set_shiftbufbys(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Shiftbufbys {
    #[inline(always)]
    fn default() -> Shiftbufbys {
        Shiftbufbys(0)
    }
}
impl core::fmt::Debug for Shiftbufbys {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftbufbys")
            .field("shiftbufbys", &self.shiftbufbys())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftbufbys {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftbufbys {{ shiftbufbys: {=u32:?} }}",
            self.shiftbufbys()
        )
    }
}
#[doc = "Shifter Buffer N Half Word Swapped Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftbufhws(pub u32);
impl Shiftbufhws {
    #[doc = "Shift Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn shiftbufhws(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer"]
    #[inline(always)]
    pub const fn set_shiftbufhws(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Shiftbufhws {
    #[inline(always)]
    fn default() -> Shiftbufhws {
        Shiftbufhws(0)
    }
}
impl core::fmt::Debug for Shiftbufhws {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftbufhws")
            .field("shiftbufhws", &self.shiftbufhws())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftbufhws {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftbufhws {{ shiftbufhws: {=u32:?} }}",
            self.shiftbufhws()
        )
    }
}
#[doc = "Shifter Buffer N Nibble Byte Swapped Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftbufnbs(pub u32);
impl Shiftbufnbs {
    #[doc = "Shift Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn shiftbufnbs(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer"]
    #[inline(always)]
    pub const fn set_shiftbufnbs(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Shiftbufnbs {
    #[inline(always)]
    fn default() -> Shiftbufnbs {
        Shiftbufnbs(0)
    }
}
impl core::fmt::Debug for Shiftbufnbs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftbufnbs")
            .field("shiftbufnbs", &self.shiftbufnbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftbufnbs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftbufnbs {{ shiftbufnbs: {=u32:?} }}",
            self.shiftbufnbs()
        )
    }
}
#[doc = "Shifter Buffer N Nibble Swapped Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftbufnis(pub u32);
impl Shiftbufnis {
    #[doc = "Shift Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn shiftbufnis(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer"]
    #[inline(always)]
    pub const fn set_shiftbufnis(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Shiftbufnis {
    #[inline(always)]
    fn default() -> Shiftbufnis {
        Shiftbufnis(0)
    }
}
impl core::fmt::Debug for Shiftbufnis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftbufnis")
            .field("shiftbufnis", &self.shiftbufnis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftbufnis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftbufnis {{ shiftbufnis: {=u32:?} }}",
            self.shiftbufnis()
        )
    }
}
#[doc = "Shifter Configuration N Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftcfg(pub u32);
impl Shiftcfg {
    #[doc = "Shifter Start bit"]
    #[must_use]
    #[inline(always)]
    pub const fn sstart(&self) -> super::vals::Sstart {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sstart::from_bits(val as u8)
    }
    #[doc = "Shifter Start bit"]
    #[inline(always)]
    pub const fn set_sstart(&mut self, val: super::vals::Sstart) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Shifter Stop bit"]
    #[must_use]
    #[inline(always)]
    pub const fn sstop(&self) -> super::vals::Sstop {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sstop::from_bits(val as u8)
    }
    #[doc = "Shifter Stop bit"]
    #[inline(always)]
    pub const fn set_sstop(&mut self, val: super::vals::Sstop) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Input Source"]
    #[must_use]
    #[inline(always)]
    pub const fn insrc(&self) -> super::vals::Insrc {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Insrc::from_bits(val as u8)
    }
    #[doc = "Input Source"]
    #[inline(always)]
    pub const fn set_insrc(&mut self, val: super::vals::Insrc) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Parallel Width"]
    #[must_use]
    #[inline(always)]
    pub const fn pwidth(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Parallel Width"]
    #[inline(always)]
    pub const fn set_pwidth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Shiftcfg {
    #[inline(always)]
    fn default() -> Shiftcfg {
        Shiftcfg(0)
    }
}
impl core::fmt::Debug for Shiftcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftcfg")
            .field("sstart", &self.sstart())
            .field("sstop", &self.sstop())
            .field("insrc", &self.insrc())
            .field("pwidth", &self.pwidth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftcfg {{ sstart: {:?}, sstop: {:?}, insrc: {:?}, pwidth: {=u8:?} }}",
            self.sstart(),
            self.sstop(),
            self.insrc(),
            self.pwidth()
        )
    }
}
#[doc = "Shifter Control N Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftctl(pub u32);
impl Shiftctl {
    #[doc = "Shifter Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> super::vals::Smod {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Smod::from_bits(val as u8)
    }
    #[doc = "Shifter Mode"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: super::vals::Smod) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Shifter Pin Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pinpol(&self) -> super::vals::ShiftctlPinpol {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::ShiftctlPinpol::from_bits(val as u8)
    }
    #[doc = "Shifter Pin Polarity"]
    #[inline(always)]
    pub const fn set_pinpol(&mut self, val: super::vals::ShiftctlPinpol) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Shifter Pin Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pinsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Shifter Pin Select"]
    #[inline(always)]
    pub const fn set_pinsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Shifter Pin Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pincfg(&self) -> super::vals::ShiftctlPincfg {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::ShiftctlPincfg::from_bits(val as u8)
    }
    #[doc = "Shifter Pin Configuration"]
    #[inline(always)]
    pub const fn set_pincfg(&mut self, val: super::vals::ShiftctlPincfg) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Timer Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn timpol(&self) -> super::vals::Timpol {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Timpol::from_bits(val as u8)
    }
    #[doc = "Timer Polarity"]
    #[inline(always)]
    pub const fn set_timpol(&mut self, val: super::vals::Timpol) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Timer Select"]
    #[must_use]
    #[inline(always)]
    pub const fn timsel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Timer Select"]
    #[inline(always)]
    pub const fn set_timsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for Shiftctl {
    #[inline(always)]
    fn default() -> Shiftctl {
        Shiftctl(0)
    }
}
impl core::fmt::Debug for Shiftctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftctl")
            .field("smod", &self.smod())
            .field("pinpol", &self.pinpol())
            .field("pinsel", &self.pinsel())
            .field("pincfg", &self.pincfg())
            .field("timpol", &self.timpol())
            .field("timsel", &self.timsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftctl {{ smod: {:?}, pinpol: {:?}, pinsel: {=u8:?}, pincfg: {:?}, timpol: {:?}, timsel: {=u8:?} }}",
            self.smod(),
            self.pinpol(),
            self.pinsel(),
            self.pincfg(),
            self.timpol(),
            self.timsel()
        )
    }
}
#[doc = "Shifter Error Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shifteien(pub u32);
impl Shifteien {
    #[doc = "Shifter Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn seie(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_seie(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Shifteien {
    #[inline(always)]
    fn default() -> Shifteien {
        Shifteien(0)
    }
}
impl core::fmt::Debug for Shifteien {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shifteien")
            .field("seie", &self.seie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shifteien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Shifteien {{ seie: {=u8:?} }}", self.seie())
    }
}
#[doc = "Shifter Error Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shifterr(pub u32);
impl Shifterr {
    #[doc = "Shifter Error Flags"]
    #[must_use]
    #[inline(always)]
    pub const fn sef(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Error Flags"]
    #[inline(always)]
    pub const fn set_sef(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Shifterr {
    #[inline(always)]
    fn default() -> Shifterr {
        Shifterr(0)
    }
}
impl core::fmt::Debug for Shifterr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shifterr")
            .field("sef", &self.sef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shifterr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Shifterr {{ sef: {=u8:?} }}", self.sef())
    }
}
#[doc = "Shifter Status DMA Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftsden(pub u32);
impl Shiftsden {
    #[doc = "Shifter Status DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ssde(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Status DMA Enable"]
    #[inline(always)]
    pub const fn set_ssde(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Shiftsden {
    #[inline(always)]
    fn default() -> Shiftsden {
        Shiftsden(0)
    }
}
impl core::fmt::Debug for Shiftsden {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftsden")
            .field("ssde", &self.ssde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftsden {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Shiftsden {{ ssde: {=u8:?} }}", self.ssde())
    }
}
#[doc = "Shifter Status Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftsien(pub u32);
impl Shiftsien {
    #[doc = "Shifter Status Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ssie(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Status Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ssie(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Shiftsien {
    #[inline(always)]
    fn default() -> Shiftsien {
        Shiftsien(0)
    }
}
impl core::fmt::Debug for Shiftsien {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftsien")
            .field("ssie", &self.ssie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftsien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Shiftsien {{ ssie: {=u8:?} }}", self.ssie())
    }
}
#[doc = "Shifter Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftstat(pub u32);
impl Shiftstat {
    #[doc = "Shifter Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ssf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Status Flag"]
    #[inline(always)]
    pub const fn set_ssf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Shiftstat {
    #[inline(always)]
    fn default() -> Shiftstat {
        Shiftstat(0)
    }
}
impl core::fmt::Debug for Shiftstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftstat")
            .field("ssf", &self.ssf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Shiftstat {{ ssf: {=u8:?} }}", self.ssf())
    }
}
#[doc = "Shifter State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftstate(pub u32);
impl Shiftstate {
    #[doc = "Current State Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Current State Pointer"]
    #[inline(always)]
    pub const fn set_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Shiftstate {
    #[inline(always)]
    fn default() -> Shiftstate {
        Shiftstate(0)
    }
}
impl core::fmt::Debug for Shiftstate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftstate")
            .field("state", &self.state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftstate {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Shiftstate {{ state: {=u8:?} }}", self.state())
    }
}
#[doc = "Timer Configuration N Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timcfg(pub u32);
impl Timcfg {
    #[doc = "Timer Start Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn tstart(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Start Bit"]
    #[inline(always)]
    pub const fn set_tstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Timer Stop Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn tstop(&self) -> super::vals::Tstop {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Tstop::from_bits(val as u8)
    }
    #[doc = "Timer Stop Bit"]
    #[inline(always)]
    pub const fn set_tstop(&mut self, val: super::vals::Tstop) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Timer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn timena(&self) -> super::vals::Timena {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Timena::from_bits(val as u8)
    }
    #[doc = "Timer Enable"]
    #[inline(always)]
    pub const fn set_timena(&mut self, val: super::vals::Timena) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Timer Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn timdis(&self) -> super::vals::Timdis {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Timdis::from_bits(val as u8)
    }
    #[doc = "Timer Disable"]
    #[inline(always)]
    pub const fn set_timdis(&mut self, val: super::vals::Timdis) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Timer Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn timrst(&self) -> super::vals::Timrst {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Timrst::from_bits(val as u8)
    }
    #[doc = "Timer Reset"]
    #[inline(always)]
    pub const fn set_timrst(&mut self, val: super::vals::Timrst) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Timer Decrement"]
    #[must_use]
    #[inline(always)]
    pub const fn timdec(&self) -> super::vals::Timdec {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Timdec::from_bits(val as u8)
    }
    #[doc = "Timer Decrement"]
    #[inline(always)]
    pub const fn set_timdec(&mut self, val: super::vals::Timdec) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Timer Output"]
    #[must_use]
    #[inline(always)]
    pub const fn timout(&self) -> super::vals::Timout {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Timout::from_bits(val as u8)
    }
    #[doc = "Timer Output"]
    #[inline(always)]
    pub const fn set_timout(&mut self, val: super::vals::Timout) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for Timcfg {
    #[inline(always)]
    fn default() -> Timcfg {
        Timcfg(0)
    }
}
impl core::fmt::Debug for Timcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timcfg")
            .field("tstart", &self.tstart())
            .field("tstop", &self.tstop())
            .field("timena", &self.timena())
            .field("timdis", &self.timdis())
            .field("timrst", &self.timrst())
            .field("timdec", &self.timdec())
            .field("timout", &self.timout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timcfg {{ tstart: {=bool:?}, tstop: {:?}, timena: {:?}, timdis: {:?}, timrst: {:?}, timdec: {:?}, timout: {:?} }}",
            self.tstart(),
            self.tstop(),
            self.timena(),
            self.timdis(),
            self.timrst(),
            self.timdec(),
            self.timout()
        )
    }
}
#[doc = "Timer Compare N Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timcmp(pub u32);
impl Timcmp {
    #[doc = "Timer Compare Value"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timer Compare Value"]
    #[inline(always)]
    pub const fn set_cmp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Timcmp {
    #[inline(always)]
    fn default() -> Timcmp {
        Timcmp(0)
    }
}
impl core::fmt::Debug for Timcmp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timcmp").field("cmp", &self.cmp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timcmp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timcmp {{ cmp: {=u16:?} }}", self.cmp())
    }
}
#[doc = "Timer Control N Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timctl(pub u32);
impl Timctl {
    #[doc = "Timer Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn timod(&self) -> super::vals::Timod {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Timod::from_bits(val as u8)
    }
    #[doc = "Timer Mode"]
    #[inline(always)]
    pub const fn set_timod(&mut self, val: super::vals::Timod) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Timer Pin Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pinpol(&self) -> super::vals::TimctlPinpol {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::TimctlPinpol::from_bits(val as u8)
    }
    #[doc = "Timer Pin Polarity"]
    #[inline(always)]
    pub const fn set_pinpol(&mut self, val: super::vals::TimctlPinpol) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer Pin Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pinsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Timer Pin Select"]
    #[inline(always)]
    pub const fn set_pinsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Timer Pin Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pincfg(&self) -> super::vals::TimctlPincfg {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::TimctlPincfg::from_bits(val as u8)
    }
    #[doc = "Timer Pin Configuration"]
    #[inline(always)]
    pub const fn set_pincfg(&mut self, val: super::vals::TimctlPincfg) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Trigger Source"]
    #[must_use]
    #[inline(always)]
    pub const fn trgsrc(&self) -> super::vals::Trgsrc {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Trgsrc::from_bits(val as u8)
    }
    #[doc = "Trigger Source"]
    #[inline(always)]
    pub const fn set_trgsrc(&mut self, val: super::vals::Trgsrc) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Trigger Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn trgpol(&self) -> super::vals::Trgpol {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Trgpol::from_bits(val as u8)
    }
    #[doc = "Trigger Polarity"]
    #[inline(always)]
    pub const fn set_trgpol(&mut self, val: super::vals::Trgpol) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Trigger Select"]
    #[must_use]
    #[inline(always)]
    pub const fn trgsel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Trigger Select"]
    #[inline(always)]
    pub const fn set_trgsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Timctl {
    #[inline(always)]
    fn default() -> Timctl {
        Timctl(0)
    }
}
impl core::fmt::Debug for Timctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timctl")
            .field("timod", &self.timod())
            .field("pinpol", &self.pinpol())
            .field("pinsel", &self.pinsel())
            .field("pincfg", &self.pincfg())
            .field("trgsrc", &self.trgsrc())
            .field("trgpol", &self.trgpol())
            .field("trgsel", &self.trgsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timctl {{ timod: {:?}, pinpol: {:?}, pinsel: {=u8:?}, pincfg: {:?}, trgsrc: {:?}, trgpol: {:?}, trgsel: {=u8:?} }}",
            self.timod(),
            self.pinpol(),
            self.pinsel(),
            self.pincfg(),
            self.trgsrc(),
            self.trgpol(),
            self.trgsel()
        )
    }
}
#[doc = "Timer Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timien(pub u32);
impl Timien {
    #[doc = "Timer Status Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn teie(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Timer Status Interrupt Enable"]
    #[inline(always)]
    pub const fn set_teie(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Timien {
    #[inline(always)]
    fn default() -> Timien {
        Timien(0)
    }
}
impl core::fmt::Debug for Timien {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timien")
            .field("teie", &self.teie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timien {{ teie: {=u8:?} }}", self.teie())
    }
}
#[doc = "Timer Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timstat(pub u32);
impl Timstat {
    #[doc = "Timer Status Flags"]
    #[must_use]
    #[inline(always)]
    pub const fn tsf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Timer Status Flags"]
    #[inline(always)]
    pub const fn set_tsf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Timstat {
    #[inline(always)]
    fn default() -> Timstat {
        Timstat(0)
    }
}
impl core::fmt::Debug for Timstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timstat").field("tsf", &self.tsf()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timstat {{ tsf: {=u8:?} }}", self.tsf())
    }
}
#[doc = "Version ID Register"]
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
