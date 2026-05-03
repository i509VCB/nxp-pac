#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "RTC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RTC Time Seconds."]
    #[inline(always)]
    pub const fn tsr(self) -> crate::pac::common::Reg<Tsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RTC Time Prescaler."]
    #[inline(always)]
    pub const fn tpr(self) -> crate::pac::common::Reg<Tpr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RTC Time Alarm."]
    #[inline(always)]
    pub const fn tar(self) -> crate::pac::common::Reg<Tar, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RTC Time Compensation."]
    #[inline(always)]
    pub const fn tcr(self) -> crate::pac::common::Reg<Tcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RTC Control."]
    #[inline(always)]
    pub const fn cr(self) -> crate::pac::common::Reg<Cr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RTC Status."]
    #[inline(always)]
    pub const fn sr(self) -> crate::pac::common::Reg<Sr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "RTC Lock."]
    #[inline(always)]
    pub const fn lr(self) -> crate::pac::common::Reg<Lr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "RTC Interrupt Enable."]
    #[inline(always)]
    pub const fn ier(self) -> crate::pac::common::Reg<Ier, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
#[doc = "RTC Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn swr(&self) -> Swr {
        let val = (self.0 >> 0usize) & 0x01;
        Swr::from_bits(val as u8)
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_swr(&mut self, val: Swr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Update Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn um(&self) -> Um {
        let val = (self.0 >> 3usize) & 0x01;
        Um::from_bits(val as u8)
    }
    #[doc = "Update Mode."]
    #[inline(always)]
    pub const fn set_um(&mut self, val: Um) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "LPO Select."]
    #[must_use]
    #[inline(always)]
    pub const fn lpos(&self) -> Lpos {
        let val = (self.0 >> 7usize) & 0x01;
        Lpos::from_bits(val as u8)
    }
    #[doc = "LPO Select."]
    #[inline(always)]
    pub const fn set_lpos(&mut self, val: Lpos) {
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
#[doc = "RTC Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Time Invalid Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tiie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Time Invalid Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tiie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Time Overflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn toie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Time Overflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_toie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Time Alarm Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn taie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Time Alarm Interrupt Enable."]
    #[inline(always)]
    pub const fn set_taie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Time Seconds Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tsie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Time Seconds Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Timer Seconds Interrupt Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn tsic(&self) -> Tsic {
        let val = (self.0 >> 16usize) & 0x07;
        Tsic::from_bits(val as u8)
    }
    #[doc = "Timer Seconds Interrupt Configuration."]
    #[inline(always)]
    pub const fn set_tsic(&mut self, val: Tsic) {
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
#[doc = "RTC Lock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lr(pub u32);
impl Lr {
    #[doc = "Time Compensation Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn tcl(&self) -> Tcl {
        let val = (self.0 >> 3usize) & 0x01;
        Tcl::from_bits(val as u8)
    }
    #[doc = "Time Compensation Lock."]
    #[inline(always)]
    pub const fn set_tcl(&mut self, val: Tcl) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Control Register Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn crl(&self) -> Crl {
        let val = (self.0 >> 4usize) & 0x01;
        Crl::from_bits(val as u8)
    }
    #[doc = "Control Register Lock."]
    #[inline(always)]
    pub const fn set_crl(&mut self, val: Crl) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Status Register Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn srl(&self) -> Srl {
        let val = (self.0 >> 5usize) & 0x01;
        Srl::from_bits(val as u8)
    }
    #[doc = "Status Register Lock."]
    #[inline(always)]
    pub const fn set_srl(&mut self, val: Srl) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Lock Register Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lrl(&self) -> Lrl {
        let val = (self.0 >> 6usize) & 0x01;
        Lrl::from_bits(val as u8)
    }
    #[doc = "Lock Register Lock."]
    #[inline(always)]
    pub const fn set_lrl(&mut self, val: Lrl) {
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
#[doc = "RTC Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Time Invalid Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tif(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Time Invalid Flag."]
    #[inline(always)]
    pub const fn set_tif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Time Overflow Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tof(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Time Overflow Flag."]
    #[inline(always)]
    pub const fn set_tof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Time Alarm Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn taf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Time Alarm Flag."]
    #[inline(always)]
    pub const fn set_taf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Time Counter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tce(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Time Counter Enable."]
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
#[doc = "RTC Time Alarm."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tar(pub u32);
impl Tar {
    #[doc = "Time Alarm Register."]
    #[must_use]
    #[inline(always)]
    pub const fn tar(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Time Alarm Register."]
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
#[doc = "RTC Time Compensation."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc = "Time Compensation Register."]
    #[must_use]
    #[inline(always)]
    pub const fn tcr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Time Compensation Register."]
    #[inline(always)]
    pub const fn set_tcr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Compensation Interval Register."]
    #[must_use]
    #[inline(always)]
    pub const fn cir(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Compensation Interval Register."]
    #[inline(always)]
    pub const fn set_cir(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Time Compensation Value."]
    #[must_use]
    #[inline(always)]
    pub const fn tcv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Time Compensation Value."]
    #[inline(always)]
    pub const fn set_tcv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Compensation Interval Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn cic(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Compensation Interval Counter."]
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
            "Tcr {{ tcr: {=u8:?}, cir: {=u8:?}, tcv: {=u8:?}, cic: {=u8:?} }}",
            self.tcr(),
            self.cir(),
            self.tcv(),
            self.cic()
        )
    }
}
#[doc = "RTC Time Prescaler."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpr(pub u32);
impl Tpr {
    #[doc = "Time Prescaler Register."]
    #[must_use]
    #[inline(always)]
    pub const fn tpr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Time Prescaler Register."]
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
#[doc = "RTC Time Seconds."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsr(pub u32);
impl Tsr {
    #[doc = "Time Seconds Register."]
    #[must_use]
    #[inline(always)]
    pub const fn tsr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Time Seconds Register."]
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crl {
    #[doc = "Control Register is locked and writes are ignored."]
    Crl0 = 0x0,
    #[doc = "Control Register is not locked and writes complete as normal."]
    Crl1 = 0x01,
}
impl Crl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crl {
    #[inline(always)]
    fn from(val: u8) -> Crl {
        Crl::from_bits(val)
    }
}
impl From<Crl> for u8 {
    #[inline(always)]
    fn from(val: Crl) -> u8 {
        Crl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpos {
    #[doc = "RTC prescaler increments using 32.768 kHz clock."]
    Lpos0 = 0x0,
    #[doc = "RTC prescaler increments using 1 kHz LPO, bits \\[4:0\\] of the prescaler are ignored."]
    Lpos1 = 0x01,
}
impl Lpos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpos {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpos {
    #[inline(always)]
    fn from(val: u8) -> Lpos {
        Lpos::from_bits(val)
    }
}
impl From<Lpos> for u8 {
    #[inline(always)]
    fn from(val: Lpos) -> u8 {
        Lpos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lrl {
    #[doc = "Lock Register is locked and writes are ignored."]
    Lrl0 = 0x0,
    #[doc = "Lock Register is not locked and writes complete as normal."]
    Lrl1 = 0x01,
}
impl Lrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lrl {
    #[inline(always)]
    fn from(val: u8) -> Lrl {
        Lrl::from_bits(val)
    }
}
impl From<Lrl> for u8 {
    #[inline(always)]
    fn from(val: Lrl) -> u8 {
        Lrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srl {
    #[doc = "Status Register is locked and writes are ignored."]
    Srl0 = 0x0,
    #[doc = "Status Register is not locked and writes complete as normal."]
    Srl1 = 0x01,
}
impl Srl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srl {
    #[inline(always)]
    fn from(val: u8) -> Srl {
        Srl::from_bits(val)
    }
}
impl From<Srl> for u8 {
    #[inline(always)]
    fn from(val: Srl) -> u8 {
        Srl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swr {
    #[doc = "No effect."]
    Swr0 = 0x0,
    #[doc = "Resets all RTC registers except for the SWR bit . The SWR bit is cleared by POR and by software explicitly clearing it."]
    Swr1 = 0x01,
}
impl Swr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swr {
    #[inline(always)]
    fn from(val: u8) -> Swr {
        Swr::from_bits(val)
    }
}
impl From<Swr> for u8 {
    #[inline(always)]
    fn from(val: Swr) -> u8 {
        Swr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcl {
    #[doc = "Time Compensation Register is locked and writes are ignored."]
    Tcl0 = 0x0,
    #[doc = "Time Compensation Register is not locked and writes complete as normal."]
    Tcl1 = 0x01,
}
impl Tcl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcl {
    #[inline(always)]
    fn from(val: u8) -> Tcl {
        Tcl::from_bits(val)
    }
}
impl From<Tcl> for u8 {
    #[inline(always)]
    fn from(val: Tcl) -> u8 {
        Tcl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsic {
    #[doc = "1 Hz."]
    Tsic0 = 0x0,
    #[doc = "2 Hz."]
    Tsic1 = 0x01,
    #[doc = "4 Hz."]
    Tsic2 = 0x02,
    #[doc = "8 Hz."]
    Tsic3 = 0x03,
    #[doc = "16 Hz."]
    Tsic4 = 0x04,
    #[doc = "32 Hz."]
    Tsic5 = 0x05,
    #[doc = "64 Hz."]
    Tsic6 = 0x06,
    #[doc = "128 Hz."]
    Tsic7 = 0x07,
}
impl Tsic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsic {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsic {
    #[inline(always)]
    fn from(val: u8) -> Tsic {
        Tsic::from_bits(val)
    }
}
impl From<Tsic> for u8 {
    #[inline(always)]
    fn from(val: Tsic) -> u8 {
        Tsic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Um {
    #[doc = "Registers cannot be written when locked."]
    Um0 = 0x0,
    #[doc = "Registers can be written when locked under limited conditions."]
    Um1 = 0x01,
}
impl Um {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Um {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Um {
    #[inline(always)]
    fn from(val: u8) -> Um {
        Um::from_bits(val)
    }
}
impl From<Um> for u8 {
    #[inline(always)]
    fn from(val: Um) -> u8 {
        Um::to_bits(val)
    }
}
