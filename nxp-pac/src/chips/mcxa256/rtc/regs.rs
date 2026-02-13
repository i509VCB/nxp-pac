#[doc = "RTC Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn swr(&self) -> super::vals::Swr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Swr::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_swr(&mut self, val: super::vals::Swr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Update Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn um(&self) -> super::vals::Um {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Um::from_bits(val as u8)
    }
    #[doc = "Update Mode"]
    #[inline(always)]
    pub const fn set_um(&mut self, val: super::vals::Um) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "LPO Select"]
    #[must_use]
    #[inline(always)]
    pub const fn lpos(&self) -> super::vals::Lpos {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Lpos::from_bits(val as u8)
    }
    #[doc = "LPO Select"]
    #[inline(always)]
    pub const fn set_lpos(&mut self, val: super::vals::Lpos) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("swr", &self.swr())
            .field("um", &self.um())
            .field("lpos", &self.lpos())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ swr: {:?}, um: {:?}, lpos: {:?} }}",
            self.swr(),
            self.um(),
            self.lpos()
        )
    }
}
#[doc = "RTC Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Time Invalid Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tiie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Time Invalid Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tiie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Time Overflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn toie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Time Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_toie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Time Alarm Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn taie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Time Alarm Interrupt Enable"]
    #[inline(always)]
    pub const fn set_taie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Time Seconds Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tsie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Time Seconds Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Timer Seconds Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn tsic(&self) -> super::vals::Tsic {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Tsic::from_bits(val as u8)
    }
    #[doc = "Timer Seconds Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_tsic(&mut self, val: super::vals::Tsic) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
}
impl Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        Ier(0)
    }
}
impl core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ier")
            .field("tiie", &self.tiie())
            .field("toie", &self.toie())
            .field("taie", &self.taie())
            .field("tsie", &self.tsie())
            .field("tsic", &self.tsic())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ier {{ tiie: {=bool:?}, toie: {=bool:?}, taie: {=bool:?}, tsie: {=bool:?}, tsic: {:?} }}",
            self.tiie(),
            self.toie(),
            self.taie(),
            self.tsie(),
            self.tsic()
        )
    }
}
#[doc = "RTC Lock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lr(pub u32);
impl Lr {
    #[doc = "Time Compensation Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn tcl(&self) -> super::vals::Tcl {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Tcl::from_bits(val as u8)
    }
    #[doc = "Time Compensation Lock"]
    #[inline(always)]
    pub const fn set_tcl(&mut self, val: super::vals::Tcl) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Control Register Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn crl(&self) -> super::vals::Crl {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Crl::from_bits(val as u8)
    }
    #[doc = "Control Register Lock"]
    #[inline(always)]
    pub const fn set_crl(&mut self, val: super::vals::Crl) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Status Register Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn srl(&self) -> super::vals::Srl {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Srl::from_bits(val as u8)
    }
    #[doc = "Status Register Lock"]
    #[inline(always)]
    pub const fn set_srl(&mut self, val: super::vals::Srl) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Lock Register Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lrl(&self) -> super::vals::Lrl {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Lrl::from_bits(val as u8)
    }
    #[doc = "Lock Register Lock"]
    #[inline(always)]
    pub const fn set_lrl(&mut self, val: super::vals::Lrl) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
}
impl Default for Lr {
    #[inline(always)]
    fn default() -> Lr {
        Lr(0)
    }
}
impl core::fmt::Debug for Lr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lr")
            .field("tcl", &self.tcl())
            .field("crl", &self.crl())
            .field("srl", &self.srl())
            .field("lrl", &self.lrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lr {{ tcl: {:?}, crl: {:?}, srl: {:?}, lrl: {:?} }}",
            self.tcl(),
            self.crl(),
            self.srl(),
            self.lrl()
        )
    }
}
#[doc = "RTC Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Time Invalid Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Time Invalid Flag"]
    #[inline(always)]
    pub const fn set_tif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Time Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tof(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Time Overflow Flag"]
    #[inline(always)]
    pub const fn set_tof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Time Alarm Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn taf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Time Alarm Flag"]
    #[inline(always)]
    pub const fn set_taf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Time Counter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tce(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Time Counter Enable"]
    #[inline(always)]
    pub const fn set_tce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("tif", &self.tif())
            .field("tof", &self.tof())
            .field("taf", &self.taf())
            .field("tce", &self.tce())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ tif: {=bool:?}, tof: {=bool:?}, taf: {=bool:?}, tce: {=bool:?} }}",
            self.tif(),
            self.tof(),
            self.taf(),
            self.tce()
        )
    }
}
#[doc = "RTC Time Alarm"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tar(pub u32);
impl Tar {
    #[doc = "Time Alarm Register"]
    #[must_use]
    #[inline(always)]
    pub const fn tar(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Time Alarm Register"]
    #[inline(always)]
    pub const fn set_tar(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tar {
    #[inline(always)]
    fn default() -> Tar {
        Tar(0)
    }
}
impl core::fmt::Debug for Tar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tar").field("tar", &self.tar()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tar {{ tar: {=u32:?} }}", self.tar())
    }
}
#[doc = "RTC Time Compensation"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc = "Time Compensation Register"]
    #[must_use]
    #[inline(always)]
    pub const fn tcr(&self) -> super::vals::Tcr {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Tcr::from_bits(val as u8)
    }
    #[doc = "Time Compensation Register"]
    #[inline(always)]
    pub const fn set_tcr(&mut self, val: super::vals::Tcr) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Compensation Interval Register"]
    #[must_use]
    #[inline(always)]
    pub const fn cir(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Compensation Interval Register"]
    #[inline(always)]
    pub const fn set_cir(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Time Compensation Value"]
    #[must_use]
    #[inline(always)]
    pub const fn tcv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Time Compensation Value"]
    #[inline(always)]
    pub const fn set_tcv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Compensation Interval Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn cic(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Compensation Interval Counter"]
    #[inline(always)]
    pub const fn set_cic(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
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
            .field("tcr", &self.tcr())
            .field("cir", &self.cir())
            .field("tcv", &self.tcv())
            .field("cic", &self.cic())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcr {{ tcr: {:?}, cir: {=u8:?}, tcv: {=u8:?}, cic: {=u8:?} }}",
            self.tcr(),
            self.cir(),
            self.tcv(),
            self.cic()
        )
    }
}
#[doc = "RTC Time Prescaler"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpr(pub u32);
impl Tpr {
    #[doc = "Time Prescaler Register"]
    #[must_use]
    #[inline(always)]
    pub const fn tpr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Time Prescaler Register"]
    #[inline(always)]
    pub const fn set_tpr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tpr {
    #[inline(always)]
    fn default() -> Tpr {
        Tpr(0)
    }
}
impl core::fmt::Debug for Tpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tpr").field("tpr", &self.tpr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tpr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tpr {{ tpr: {=u16:?} }}", self.tpr())
    }
}
#[doc = "RTC Time Seconds"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsr(pub u32);
impl Tsr {
    #[doc = "Time Seconds Register"]
    #[must_use]
    #[inline(always)]
    pub const fn tsr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Time Seconds Register"]
    #[inline(always)]
    pub const fn set_tsr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tsr {
    #[inline(always)]
    fn default() -> Tsr {
        Tsr(0)
    }
}
impl core::fmt::Debug for Tsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsr").field("tsr", &self.tsr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsr {{ tsr: {=u32:?} }}", self.tsr())
    }
}
