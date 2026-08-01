#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "TDET."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdet {
    ptr: *mut u8,
}
unsafe impl Send for Tdet {}
unsafe impl Sync for Tdet {}
impl Tdet {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn cr(self) -> crate::pac::common::Reg<Cr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn sr(self) -> crate::pac::common::Reg<Sr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn lr(self) -> crate::pac::common::Reg<Lr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn ier(self) -> crate::pac::common::Reg<Ier, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Tamper Seconds."]
    #[inline(always)]
    pub const fn tsr(self) -> crate::pac::common::Reg<Tsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Tamper Enable."]
    #[inline(always)]
    pub const fn ter(self) -> crate::pac::common::Reg<Ter, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Pin Direction."]
    #[inline(always)]
    pub const fn pdr(self) -> crate::pac::common::Reg<Pdr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Pin Polarity."]
    #[inline(always)]
    pub const fn ppr(self) -> crate::pac::common::Reg<Ppr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Active Tamper."]
    #[inline(always)]
    pub const fn atr(self, n: usize) -> crate::pac::common::Reg<Atr, crate::pac::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize + n * 4usize) as _)
        }
    }
    #[doc = "Pin Glitch Filter."]
    #[inline(always)]
    pub const fn pgfr(self, n: usize) -> crate::pac::common::Reg<Pgfr, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _)
        }
    }
}
#[doc = "Active Tamper."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atr(pub u32);
impl Atr {
    #[doc = "Active Tamper Shift Register."]
    #[must_use]
    #[inline(always)]
    pub const fn atsr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Active Tamper Shift Register."]
    #[inline(always)]
    pub const fn set_atsr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Active Tamper Polynomial."]
    #[must_use]
    #[inline(always)]
    pub const fn atp(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Active Tamper Polynomial."]
    #[inline(always)]
    pub const fn set_atp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Atr {
    #[inline(always)]
    fn default() -> Atr {
        Atr(0)
    }
}
impl core::fmt::Debug for Atr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Atr")
            .field("atsr", &self.atsr())
            .field("atp", &self.atp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Atr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Atr {{ atsr: {=u16:?}, atp: {=u16:?} }}",
            self.atsr(),
            self.atp()
        )
    }
}
#[doc = "Control."]
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
    #[doc = "Digital Tamper Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn den(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Digital Tamper Enable."]
    #[inline(always)]
    pub const fn set_den(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Tamper Force System Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn tfsr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Force System Reset."]
    #[inline(always)]
    pub const fn set_tfsr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
    #[doc = "Active Tamper Clock Source."]
    #[must_use]
    #[inline(always)]
    pub const fn atcs0(&self) -> Atcs0 {
        let val = (self.0 >> 4usize) & 0x01;
        Atcs0::from_bits(val as u8)
    }
    #[doc = "Active Tamper Clock Source."]
    #[inline(always)]
    pub const fn set_atcs0(&mut self, val: Atcs0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Active Tamper Clock Source."]
    #[must_use]
    #[inline(always)]
    pub const fn atcs1(&self) -> Atcs1 {
        let val = (self.0 >> 5usize) & 0x01;
        Atcs1::from_bits(val as u8)
    }
    #[doc = "Active Tamper Clock Source."]
    #[inline(always)]
    pub const fn set_atcs1(&mut self, val: Atcs1) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Disable Prescaler On Tamper."]
    #[must_use]
    #[inline(always)]
    pub const fn distam(&self) -> Distam {
        let val = (self.0 >> 8usize) & 0x01;
        Distam::from_bits(val as u8)
    }
    #[doc = "Disable Prescaler On Tamper."]
    #[inline(always)]
    pub const fn set_distam(&mut self, val: Distam) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Digital Tamper Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn dpr(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "Digital Tamper Prescaler."]
    #[inline(always)]
    pub const fn set_dpr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
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
            .field("den", &self.den())
            .field("tfsr", &self.tfsr())
            .field("um", &self.um())
            .field("atcs0", &self.atcs0())
            .field("atcs1", &self.atcs1())
            .field("distam", &self.distam())
            .field("dpr", &self.dpr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ swr: {:?}, den: {=bool:?}, tfsr: {=bool:?}, um: {:?}, atcs0: {:?}, atcs1: {:?}, distam: {:?}, dpr: {=u16:?} }}",
            self.swr(),
            self.den(),
            self.tfsr(),
            self.um(),
            self.atcs0(),
            self.atcs1(),
            self.distam(),
            self.dpr()
        )
    }
}
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Digital Tamper Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dtie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Digital Tamper Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dtie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tiie(&self, n: usize) -> bool {
        assert!(n < 10usize);
        let offs = 2usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tiie(&mut self, n: usize, val: bool) {
        assert!(n < 10usize);
        let offs = 2usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Tamper Pin n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpie(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tpie(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("dtie", &self.dtie())
            .field("tiie[0]", &self.tiie(0usize))
            .field("tiie[1]", &self.tiie(1usize))
            .field("tiie[2]", &self.tiie(2usize))
            .field("tiie[3]", &self.tiie(3usize))
            .field("tiie[4]", &self.tiie(4usize))
            .field("tiie[5]", &self.tiie(5usize))
            .field("tiie[6]", &self.tiie(6usize))
            .field("tiie[7]", &self.tiie(7usize))
            .field("tiie[8]", &self.tiie(8usize))
            .field("tiie[9]", &self.tiie(9usize))
            .field("tpie[0]", &self.tpie(0usize))
            .field("tpie[1]", &self.tpie(1usize))
            .field("tpie[2]", &self.tpie(2usize))
            .field("tpie[3]", &self.tpie(3usize))
            .field("tpie[4]", &self.tpie(4usize))
            .field("tpie[5]", &self.tpie(5usize))
            .field("tpie[6]", &self.tpie(6usize))
            .field("tpie[7]", &self.tpie(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ier {{ dtie: {=bool:?}, tiie[0]: {=bool:?}, tiie[1]: {=bool:?}, tiie[2]: {=bool:?}, tiie[3]: {=bool:?}, tiie[4]: {=bool:?}, tiie[5]: {=bool:?}, tiie[6]: {=bool:?}, tiie[7]: {=bool:?}, tiie[8]: {=bool:?}, tiie[9]: {=bool:?}, tpie[0]: {=bool:?}, tpie[1]: {=bool:?}, tpie[2]: {=bool:?}, tpie[3]: {=bool:?}, tpie[4]: {=bool:?}, tpie[5]: {=bool:?}, tpie[6]: {=bool:?}, tpie[7]: {=bool:?} }}",
            self.dtie(),
            self.tiie(0usize),
            self.tiie(1usize),
            self.tiie(2usize),
            self.tiie(3usize),
            self.tiie(4usize),
            self.tiie(5usize),
            self.tiie(6usize),
            self.tiie(7usize),
            self.tiie(8usize),
            self.tiie(9usize),
            self.tpie(0usize),
            self.tpie(1usize),
            self.tpie(2usize),
            self.tpie(3usize),
            self.tpie(4usize),
            self.tpie(5usize),
            self.tpie(6usize),
            self.tpie(7usize)
        )
    }
}
#[doc = "Lock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lr(pub u32);
impl Lr {
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
    #[doc = "Interrupt Enable Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn iel(&self) -> Iel {
        let val = (self.0 >> 7usize) & 0x01;
        Iel::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable Lock."]
    #[inline(always)]
    pub const fn set_iel(&mut self, val: Iel) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Tamper Seconds Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn tsl(&self) -> Tsl {
        let val = (self.0 >> 8usize) & 0x01;
        Tsl::from_bits(val as u8)
    }
    #[doc = "Tamper Seconds Lock."]
    #[inline(always)]
    pub const fn set_tsl(&mut self, val: Tsl) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Tamper Enable Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn tel(&self) -> Tel {
        let val = (self.0 >> 9usize) & 0x01;
        Tel::from_bits(val as u8)
    }
    #[doc = "Tamper Enable Lock."]
    #[inline(always)]
    pub const fn set_tel(&mut self, val: Tel) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Pin Direction Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn pdl(&self) -> Pdl {
        let val = (self.0 >> 10usize) & 0x01;
        Pdl::from_bits(val as u8)
    }
    #[doc = "Pin Direction Lock."]
    #[inline(always)]
    pub const fn set_pdl(&mut self, val: Pdl) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pin Polarity Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn ppl(&self) -> Ppl {
        let val = (self.0 >> 11usize) & 0x01;
        Ppl::from_bits(val as u8)
    }
    #[doc = "Pin Polarity Lock."]
    #[inline(always)]
    pub const fn set_ppl(&mut self, val: Ppl) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Active Tamper Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn atl0(&self) -> Atl0 {
        let val = (self.0 >> 12usize) & 0x01;
        Atl0::from_bits(val as u8)
    }
    #[doc = "Active Tamper Lock."]
    #[inline(always)]
    pub const fn set_atl0(&mut self, val: Atl0) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Active Tamper Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn atl1(&self) -> Atl1 {
        let val = (self.0 >> 13usize) & 0x01;
        Atl1::from_bits(val as u8)
    }
    #[doc = "Active Tamper Lock."]
    #[inline(always)]
    pub const fn set_atl1(&mut self, val: Atl1) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Glitch Filter Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn gfl(&self, n: usize) -> Gfl {
        assert!(n < 8usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Gfl::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Lock."]
    #[inline(always)]
    pub const fn set_gfl(&mut self, n: usize, val: Gfl) {
        assert!(n < 8usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
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
            .field("crl", &self.crl())
            .field("srl", &self.srl())
            .field("lrl", &self.lrl())
            .field("iel", &self.iel())
            .field("tsl", &self.tsl())
            .field("tel", &self.tel())
            .field("pdl", &self.pdl())
            .field("ppl", &self.ppl())
            .field("atl0", &self.atl0())
            .field("atl1", &self.atl1())
            .field("gfl[0]", &self.gfl(0usize))
            .field("gfl[1]", &self.gfl(1usize))
            .field("gfl[2]", &self.gfl(2usize))
            .field("gfl[3]", &self.gfl(3usize))
            .field("gfl[4]", &self.gfl(4usize))
            .field("gfl[5]", &self.gfl(5usize))
            .field("gfl[6]", &self.gfl(6usize))
            .field("gfl[7]", &self.gfl(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lr {{ crl: {:?}, srl: {:?}, lrl: {:?}, iel: {:?}, tsl: {:?}, tel: {:?}, pdl: {:?}, ppl: {:?}, atl0: {:?}, atl1: {:?}, gfl[0]: {:?}, gfl[1]: {:?}, gfl[2]: {:?}, gfl[3]: {:?}, gfl[4]: {:?}, gfl[5]: {:?}, gfl[6]: {:?}, gfl[7]: {:?} }}",
            self.crl(),
            self.srl(),
            self.lrl(),
            self.iel(),
            self.tsl(),
            self.tel(),
            self.pdl(),
            self.ppl(),
            self.atl0(),
            self.atl1(),
            self.gfl(0usize),
            self.gfl(1usize),
            self.gfl(2usize),
            self.gfl(3usize),
            self.gfl(4usize),
            self.gfl(5usize),
            self.gfl(6usize),
            self.gfl(7usize)
        )
    }
}
#[doc = "Pin Direction."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr(pub u32);
impl Pdr {
    #[doc = "Tamper Pin Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn tpd(&self, n: usize) -> Tpd {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Tpd::from_bits(val as u8)
    }
    #[doc = "Tamper Pin Direction."]
    #[inline(always)]
    pub const fn set_tpd(&mut self, n: usize, val: Tpd) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Tamper Pin Output Data."]
    #[must_use]
    #[inline(always)]
    pub const fn tpod(&self, n: usize) -> Tpod {
        assert!(n < 8usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Tpod::from_bits(val as u8)
    }
    #[doc = "Tamper Pin Output Data."]
    #[inline(always)]
    pub const fn set_tpod(&mut self, n: usize, val: Tpod) {
        assert!(n < 8usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Pdr {
    #[inline(always)]
    fn default() -> Pdr {
        Pdr(0)
    }
}
impl core::fmt::Debug for Pdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdr")
            .field("tpd[0]", &self.tpd(0usize))
            .field("tpd[1]", &self.tpd(1usize))
            .field("tpd[2]", &self.tpd(2usize))
            .field("tpd[3]", &self.tpd(3usize))
            .field("tpd[4]", &self.tpd(4usize))
            .field("tpd[5]", &self.tpd(5usize))
            .field("tpd[6]", &self.tpd(6usize))
            .field("tpd[7]", &self.tpd(7usize))
            .field("tpod[0]", &self.tpod(0usize))
            .field("tpod[1]", &self.tpod(1usize))
            .field("tpod[2]", &self.tpod(2usize))
            .field("tpod[3]", &self.tpod(3usize))
            .field("tpod[4]", &self.tpod(4usize))
            .field("tpod[5]", &self.tpod(5usize))
            .field("tpod[6]", &self.tpod(6usize))
            .field("tpod[7]", &self.tpod(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdr {{ tpd[0]: {:?}, tpd[1]: {:?}, tpd[2]: {:?}, tpd[3]: {:?}, tpd[4]: {:?}, tpd[5]: {:?}, tpd[6]: {:?}, tpd[7]: {:?}, tpod[0]: {:?}, tpod[1]: {:?}, tpod[2]: {:?}, tpod[3]: {:?}, tpod[4]: {:?}, tpod[5]: {:?}, tpod[6]: {:?}, tpod[7]: {:?} }}",
            self.tpd(0usize),
            self.tpd(1usize),
            self.tpd(2usize),
            self.tpd(3usize),
            self.tpd(4usize),
            self.tpd(5usize),
            self.tpd(6usize),
            self.tpd(7usize),
            self.tpod(0usize),
            self.tpod(1usize),
            self.tpod(2usize),
            self.tpod(3usize),
            self.tpod(4usize),
            self.tpod(5usize),
            self.tpod(6usize),
            self.tpod(7usize)
        )
    }
}
#[doc = "Pin Glitch Filter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pgfr(pub u32);
impl Pgfr {
    #[doc = "Glitch Filter Width."]
    #[must_use]
    #[inline(always)]
    pub const fn gfw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Glitch Filter Width."]
    #[inline(always)]
    pub const fn set_gfw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Glitch Filter Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn gfp(&self) -> Gfp {
        let val = (self.0 >> 6usize) & 0x01;
        Gfp::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Prescaler."]
    #[inline(always)]
    pub const fn set_gfp(&mut self, val: Gfp) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Glitch Filter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gfe(&self) -> Gfe {
        let val = (self.0 >> 7usize) & 0x01;
        Gfe::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Enable."]
    #[inline(always)]
    pub const fn set_gfe(&mut self, val: Gfe) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Tamper Pin Sample Width."]
    #[must_use]
    #[inline(always)]
    pub const fn tpsw(&self) -> Tpsw {
        let val = (self.0 >> 8usize) & 0x03;
        Tpsw::from_bits(val as u8)
    }
    #[doc = "Tamper Pin Sample Width."]
    #[inline(always)]
    pub const fn set_tpsw(&mut self, val: Tpsw) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Tamper Pin Sample Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn tpsf(&self) -> Tpsf {
        let val = (self.0 >> 10usize) & 0x03;
        Tpsf::from_bits(val as u8)
    }
    #[doc = "Tamper Pin Sample Frequency."]
    #[inline(always)]
    pub const fn set_tpsf(&mut self, val: Tpsf) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Tamper Pin Expected."]
    #[must_use]
    #[inline(always)]
    pub const fn tpex(&self) -> Tpex {
        let val = (self.0 >> 16usize) & 0x03;
        Tpex::from_bits(val as u8)
    }
    #[doc = "Tamper Pin Expected."]
    #[inline(always)]
    pub const fn set_tpex(&mut self, val: Tpex) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Tamper Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpe(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pull Enable."]
    #[inline(always)]
    pub const fn set_tpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Tamper Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tps(&self) -> Tps {
        let val = (self.0 >> 25usize) & 0x01;
        Tps::from_bits(val as u8)
    }
    #[doc = "Tamper Pull Select."]
    #[inline(always)]
    pub const fn set_tps(&mut self, val: Tps) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Pgfr {
    #[inline(always)]
    fn default() -> Pgfr {
        Pgfr(0)
    }
}
impl core::fmt::Debug for Pgfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pgfr")
            .field("gfw", &self.gfw())
            .field("gfp", &self.gfp())
            .field("gfe", &self.gfe())
            .field("tpsw", &self.tpsw())
            .field("tpsf", &self.tpsf())
            .field("tpex", &self.tpex())
            .field("tpe", &self.tpe())
            .field("tps", &self.tps())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pgfr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pgfr {{ gfw: {=u8:?}, gfp: {:?}, gfe: {:?}, tpsw: {:?}, tpsf: {:?}, tpex: {:?}, tpe: {=bool:?}, tps: {:?} }}",
            self.gfw(),
            self.gfp(),
            self.gfe(),
            self.tpsw(),
            self.tpsf(),
            self.tpex(),
            self.tpe(),
            self.tps()
        )
    }
}
#[doc = "Pin Polarity."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppr(pub u32);
impl Ppr {
    #[doc = "Tamper Pin n Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tpp(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Polarity."]
    #[inline(always)]
    pub const fn set_tpp(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Tamper Pin n Input Data."]
    #[must_use]
    #[inline(always)]
    pub const fn tpid(&self, n: usize) -> Tpid {
        assert!(n < 8usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Tpid::from_bits(val as u8)
    }
    #[doc = "Tamper Pin n Input Data."]
    #[inline(always)]
    pub const fn set_tpid(&mut self, n: usize, val: Tpid) {
        assert!(n < 8usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Ppr {
    #[inline(always)]
    fn default() -> Ppr {
        Ppr(0)
    }
}
impl core::fmt::Debug for Ppr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppr")
            .field("tpp[0]", &self.tpp(0usize))
            .field("tpp[1]", &self.tpp(1usize))
            .field("tpp[2]", &self.tpp(2usize))
            .field("tpp[3]", &self.tpp(3usize))
            .field("tpp[4]", &self.tpp(4usize))
            .field("tpp[5]", &self.tpp(5usize))
            .field("tpp[6]", &self.tpp(6usize))
            .field("tpp[7]", &self.tpp(7usize))
            .field("tpid[0]", &self.tpid(0usize))
            .field("tpid[1]", &self.tpid(1usize))
            .field("tpid[2]", &self.tpid(2usize))
            .field("tpid[3]", &self.tpid(3usize))
            .field("tpid[4]", &self.tpid(4usize))
            .field("tpid[5]", &self.tpid(5usize))
            .field("tpid[6]", &self.tpid(6usize))
            .field("tpid[7]", &self.tpid(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ppr {{ tpp[0]: {=bool:?}, tpp[1]: {=bool:?}, tpp[2]: {=bool:?}, tpp[3]: {=bool:?}, tpp[4]: {=bool:?}, tpp[5]: {=bool:?}, tpp[6]: {=bool:?}, tpp[7]: {=bool:?}, tpid[0]: {:?}, tpid[1]: {:?}, tpid[2]: {:?}, tpid[3]: {:?}, tpid[4]: {:?}, tpid[5]: {:?}, tpid[6]: {:?}, tpid[7]: {:?} }}",
            self.tpp(0usize),
            self.tpp(1usize),
            self.tpp(2usize),
            self.tpp(3usize),
            self.tpp(4usize),
            self.tpp(5usize),
            self.tpp(6usize),
            self.tpp(7usize),
            self.tpid(0usize),
            self.tpid(1usize),
            self.tpid(2usize),
            self.tpid(3usize),
            self.tpid(4usize),
            self.tpid(5usize),
            self.tpid(6usize),
            self.tpid(7usize)
        )
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Digital Tamper Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dtf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Digital Tamper Flag."]
    #[inline(always)]
    pub const fn set_dtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Tamper Acknowledge Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn taf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Acknowledge Flag."]
    #[inline(always)]
    pub const fn set_taf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Tamper Input n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tif(&self, n: usize) -> bool {
        assert!(n < 10usize);
        let offs = 2usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag."]
    #[inline(always)]
    pub const fn set_tif(&mut self, n: usize, val: bool) {
        assert!(n < 10usize);
        let offs = 2usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Tamper Pin n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tpf(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Flag."]
    #[inline(always)]
    pub const fn set_tpf(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("dtf", &self.dtf())
            .field("taf", &self.taf())
            .field("tif[0]", &self.tif(0usize))
            .field("tif[1]", &self.tif(1usize))
            .field("tif[2]", &self.tif(2usize))
            .field("tif[3]", &self.tif(3usize))
            .field("tif[4]", &self.tif(4usize))
            .field("tif[5]", &self.tif(5usize))
            .field("tif[6]", &self.tif(6usize))
            .field("tif[7]", &self.tif(7usize))
            .field("tif[8]", &self.tif(8usize))
            .field("tif[9]", &self.tif(9usize))
            .field("tpf[0]", &self.tpf(0usize))
            .field("tpf[1]", &self.tpf(1usize))
            .field("tpf[2]", &self.tpf(2usize))
            .field("tpf[3]", &self.tpf(3usize))
            .field("tpf[4]", &self.tpf(4usize))
            .field("tpf[5]", &self.tpf(5usize))
            .field("tpf[6]", &self.tpf(6usize))
            .field("tpf[7]", &self.tpf(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ dtf: {=bool:?}, taf: {=bool:?}, tif[0]: {=bool:?}, tif[1]: {=bool:?}, tif[2]: {=bool:?}, tif[3]: {=bool:?}, tif[4]: {=bool:?}, tif[5]: {=bool:?}, tif[6]: {=bool:?}, tif[7]: {=bool:?}, tif[8]: {=bool:?}, tif[9]: {=bool:?}, tpf[0]: {=bool:?}, tpf[1]: {=bool:?}, tpf[2]: {=bool:?}, tpf[3]: {=bool:?}, tpf[4]: {=bool:?}, tpf[5]: {=bool:?}, tpf[6]: {=bool:?}, tpf[7]: {=bool:?} }}",
            self.dtf(),
            self.taf(),
            self.tif(0usize),
            self.tif(1usize),
            self.tif(2usize),
            self.tif(3usize),
            self.tif(4usize),
            self.tif(5usize),
            self.tif(6usize),
            self.tif(7usize),
            self.tif(8usize),
            self.tif(9usize),
            self.tpf(0usize),
            self.tpf(1usize),
            self.tpf(2usize),
            self.tpf(3usize),
            self.tpf(4usize),
            self.tpf(5usize),
            self.tpf(6usize),
            self.tpf(7usize)
        )
    }
}
#[doc = "Tamper Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ter(pub u32);
impl Ter {
    #[doc = "Tamper Input Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self, n: usize) -> bool {
        assert!(n < 10usize);
        let offs = 2usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable."]
    #[inline(always)]
    pub const fn set_tie(&mut self, n: usize, val: bool) {
        assert!(n < 10usize);
        let offs = 2usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Tamper Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpe(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin Enable."]
    #[inline(always)]
    pub const fn set_tpe(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Ter {
    #[inline(always)]
    fn default() -> Ter {
        Ter(0)
    }
}
impl core::fmt::Debug for Ter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ter")
            .field("tie[0]", &self.tie(0usize))
            .field("tie[1]", &self.tie(1usize))
            .field("tie[2]", &self.tie(2usize))
            .field("tie[3]", &self.tie(3usize))
            .field("tie[4]", &self.tie(4usize))
            .field("tie[5]", &self.tie(5usize))
            .field("tie[6]", &self.tie(6usize))
            .field("tie[7]", &self.tie(7usize))
            .field("tie[8]", &self.tie(8usize))
            .field("tie[9]", &self.tie(9usize))
            .field("tpe[0]", &self.tpe(0usize))
            .field("tpe[1]", &self.tpe(1usize))
            .field("tpe[2]", &self.tpe(2usize))
            .field("tpe[3]", &self.tpe(3usize))
            .field("tpe[4]", &self.tpe(4usize))
            .field("tpe[5]", &self.tpe(5usize))
            .field("tpe[6]", &self.tpe(6usize))
            .field("tpe[7]", &self.tpe(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ter {{ tie[0]: {=bool:?}, tie[1]: {=bool:?}, tie[2]: {=bool:?}, tie[3]: {=bool:?}, tie[4]: {=bool:?}, tie[5]: {=bool:?}, tie[6]: {=bool:?}, tie[7]: {=bool:?}, tie[8]: {=bool:?}, tie[9]: {=bool:?}, tpe[0]: {=bool:?}, tpe[1]: {=bool:?}, tpe[2]: {=bool:?}, tpe[3]: {=bool:?}, tpe[4]: {=bool:?}, tpe[5]: {=bool:?}, tpe[6]: {=bool:?}, tpe[7]: {=bool:?} }}",
            self.tie(0usize),
            self.tie(1usize),
            self.tie(2usize),
            self.tie(3usize),
            self.tie(4usize),
            self.tie(5usize),
            self.tie(6usize),
            self.tie(7usize),
            self.tie(8usize),
            self.tie(9usize),
            self.tpe(0usize),
            self.tpe(1usize),
            self.tpe(2usize),
            self.tpe(3usize),
            self.tpe(4usize),
            self.tpe(5usize),
            self.tpe(6usize),
            self.tpe(7usize)
        )
    }
}
#[doc = "Tamper Seconds."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsr(pub u32);
impl Tsr {
    #[doc = "Tamper Time Seconds."]
    #[must_use]
    #[inline(always)]
    pub const fn tts(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Tamper Time Seconds."]
    #[inline(always)]
    pub const fn set_tts(&mut self, val: u32) {
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
        f.debug_struct("Tsr").field("tts", &self.tts()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsr {{ tts: {=u32:?} }}", self.tts())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Atcs0 {
    #[doc = "1 Hz prescaler clock."]
    Freq1Hz = 0x0,
    #[doc = "64 Hz prescaler clock."]
    Freq64Hz = 0x01,
}
impl Atcs0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Atcs0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Atcs0 {
    #[inline(always)]
    fn from(val: u8) -> Atcs0 {
        Atcs0::from_bits(val)
    }
}
impl From<Atcs0> for u8 {
    #[inline(always)]
    fn from(val: Atcs0) -> u8 {
        Atcs0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Atcs1 {
    #[doc = "1 Hz prescaler clock."]
    Freq1Hz = 0x0,
    #[doc = "64 Hz prescaler clock."]
    Freq64Hz = 0x01,
}
impl Atcs1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Atcs1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Atcs1 {
    #[inline(always)]
    fn from(val: u8) -> Atcs1 {
        Atcs1::from_bits(val)
    }
}
impl From<Atcs1> for u8 {
    #[inline(always)]
    fn from(val: Atcs1) -> u8 {
        Atcs1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Atl0 {
    #[doc = "Locked and writes are ignored."]
    Lock = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NotLock = 0x01,
}
impl Atl0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Atl0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Atl0 {
    #[inline(always)]
    fn from(val: u8) -> Atl0 {
        Atl0::from_bits(val)
    }
}
impl From<Atl0> for u8 {
    #[inline(always)]
    fn from(val: Atl0) -> u8 {
        Atl0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Atl1 {
    #[doc = "Locked and writes are ignored."]
    Lock = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NotLock = 0x01,
}
impl Atl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Atl1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Atl1 {
    #[inline(always)]
    fn from(val: u8) -> Atl1 {
        Atl1::from_bits(val)
    }
}
impl From<Atl1> for u8 {
    #[inline(always)]
    fn from(val: Atl1) -> u8 {
        Atl1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crl {
    #[doc = "Locked and writes are ignored."]
    Lock = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NotLock = 0x01,
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
pub enum Distam {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Automatically disables the prescaler after tamper detection."]
    AutoDis = 0x01,
}
impl Distam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Distam {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Distam {
    #[inline(always)]
    fn from(val: u8) -> Distam {
        Distam::from_bits(val)
    }
}
impl From<Distam> for u8 {
    #[inline(always)]
    fn from(val: Distam) -> u8 {
        Distam::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfe {
    #[doc = "Bypasses."]
    Bypass = 0x0,
    #[doc = "Enables."]
    Enable = 0x01,
}
impl Gfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfe {
    #[inline(always)]
    fn from(val: u8) -> Gfe {
        Gfe::from_bits(val)
    }
}
impl From<Gfe> for u8 {
    #[inline(always)]
    fn from(val: Gfe) -> u8 {
        Gfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl {
    #[doc = "Locked and writes are ignored."]
    Lock = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NotLock = 0x01,
}
impl Gfl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl {
    #[inline(always)]
    fn from(val: u8) -> Gfl {
        Gfl::from_bits(val)
    }
}
impl From<Gfl> for u8 {
    #[inline(always)]
    fn from(val: Gfl) -> u8 {
        Gfl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfp {
    #[doc = "512 Hz prescaler clock."]
    Freq512Hz = 0x0,
    #[doc = "32.768 kHz clock."]
    Freq32Khz = 0x01,
}
impl Gfp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfp {
    #[inline(always)]
    fn from(val: u8) -> Gfp {
        Gfp::from_bits(val)
    }
}
impl From<Gfp> for u8 {
    #[inline(always)]
    fn from(val: Gfp) -> u8 {
        Gfp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iel {
    #[doc = "Locked and writes are ignored."]
    Lock = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NotLock = 0x01,
}
impl Iel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iel {
    #[inline(always)]
    fn from(val: u8) -> Iel {
        Iel::from_bits(val)
    }
}
impl From<Iel> for u8 {
    #[inline(always)]
    fn from(val: Iel) -> u8 {
        Iel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lrl {
    #[doc = "Locked and writes are ignored."]
    Lock = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NotLock = 0x01,
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
pub enum Pdl {
    #[doc = "Locked and writes are ignored."]
    Lock = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NotLock = 0x01,
}
impl Pdl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdl {
    #[inline(always)]
    fn from(val: u8) -> Pdl {
        Pdl::from_bits(val)
    }
}
impl From<Pdl> for u8 {
    #[inline(always)]
    fn from(val: Pdl) -> u8 {
        Pdl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ppl {
    #[doc = "Locked and writes are ignored."]
    Lock = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NotLock = 0x01,
}
impl Ppl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ppl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ppl {
    #[inline(always)]
    fn from(val: u8) -> Ppl {
        Ppl::from_bits(val)
    }
}
impl From<Ppl> for u8 {
    #[inline(always)]
    fn from(val: Ppl) -> u8 {
        Ppl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srl {
    #[doc = "Locked and writes are ignored."]
    Lock = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NotLock = 0x01,
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
    NoEffect = 0x0,
    #[doc = "Perform a software reset."]
    SwReset = 0x01,
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
pub enum Tel {
    #[doc = "Locked and writes are ignored."]
    Lock = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NotLock = 0x01,
}
impl Tel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tel {
    #[inline(always)]
    fn from(val: u8) -> Tel {
        Tel::from_bits(val)
    }
}
impl From<Tel> for u8 {
    #[inline(always)]
    fn from(val: Tel) -> u8 {
        Tel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpd {
    #[doc = "Input."]
    Input = 0x0,
    #[doc = "Output and drives the inverse of the expected value (tamper pin is asserted)."]
    Output = 0x01,
}
impl Tpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpd {
    #[inline(always)]
    fn from(val: u8) -> Tpd {
        Tpd::from_bits(val)
    }
}
impl From<Tpd> for u8 {
    #[inline(always)]
    fn from(val: Tpd) -> u8 {
        Tpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpex {
    #[doc = "Zero/passive tamper."]
    Zero = 0x0,
    #[doc = "Active Tamper 0 output."]
    ValTamp0 = 0x01,
    #[doc = "Active Tamper 1 output."]
    ValTamp1 = 0x02,
    #[doc = "Active Tamper 0 output XORed with Active Tamper 1 output."]
    Tamp0XorTamp1 = 0x03,
}
impl Tpex {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpex {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpex {
    #[inline(always)]
    fn from(val: u8) -> Tpex {
        Tpex::from_bits(val)
    }
}
impl From<Tpex> for u8 {
    #[inline(always)]
    fn from(val: Tpex) -> u8 {
        Tpex::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpid {
    #[doc = "Zero."]
    Zero = 0x0,
    #[doc = "One."]
    One = 0x01,
}
impl Tpid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpid {
    #[inline(always)]
    fn from(val: u8) -> Tpid {
        Tpid::from_bits(val)
    }
}
impl From<Tpid> for u8 {
    #[inline(always)]
    fn from(val: Tpid) -> u8 {
        Tpid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpod {
    #[doc = "Zero."]
    Zero = 0x0,
    #[doc = "One."]
    One = 0x01,
}
impl Tpod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpod {
    #[inline(always)]
    fn from(val: u8) -> Tpod {
        Tpod::from_bits(val)
    }
}
impl From<Tpod> for u8 {
    #[inline(always)]
    fn from(val: Tpod) -> u8 {
        Tpod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tps {
    #[doc = "Asserts."]
    Assert = 0x0,
    #[doc = "Negates."]
    Negate = 0x01,
}
impl Tps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tps {
    #[inline(always)]
    fn from(val: u8) -> Tps {
        Tps::from_bits(val)
    }
}
impl From<Tps> for u8 {
    #[inline(always)]
    fn from(val: Tps) -> u8 {
        Tps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpsf {
    #[doc = "Every 8 cycles."]
    Cycles8 = 0x0,
    #[doc = "Every 32 cycles."]
    Cycles32 = 0x01,
    #[doc = "Every 128 cycles."]
    Cycles128 = 0x02,
    #[doc = "Every 512 cycles."]
    Cycles512 = 0x03,
}
impl Tpsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpsf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpsf {
    #[inline(always)]
    fn from(val: u8) -> Tpsf {
        Tpsf::from_bits(val)
    }
}
impl From<Tpsf> for u8 {
    #[inline(always)]
    fn from(val: Tpsf) -> u8 {
        Tpsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpsw {
    #[doc = "Continuous monitoring, pin sampling disabled."]
    Disable = 0x0,
    #[doc = "2 cycles for pull enable and 1 cycle for input buffer enable."]
    Cycles2 = 0x01,
    #[doc = "4 cycles for pull enable and 2 cycles for input buffer enable."]
    Cycles4 = 0x02,
    #[doc = "8 cycles for pull enable and 4 cycles for input buffer enable."]
    Cycles8 = 0x03,
}
impl Tpsw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpsw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpsw {
    #[inline(always)]
    fn from(val: u8) -> Tpsw {
        Tpsw::from_bits(val)
    }
}
impl From<Tpsw> for u8 {
    #[inline(always)]
    fn from(val: Tpsw) -> u8 {
        Tpsw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsl {
    #[doc = "Locked and writes are ignored."]
    Lock = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NotLock = 0x01,
}
impl Tsl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsl {
    #[inline(always)]
    fn from(val: u8) -> Tsl {
        Tsl::from_bits(val)
    }
}
impl From<Tsl> for u8 {
    #[inline(always)]
    fn from(val: Tsl) -> u8 {
        Tsl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Um {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Allows the clearing of interrupts."]
    ClearInts = 0x01,
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
