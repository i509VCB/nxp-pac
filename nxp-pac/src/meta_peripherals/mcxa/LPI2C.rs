#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Low-Power Inter-Integrated Circuit."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c {
    ptr: *mut u8,
}
unsafe impl Send for Lpi2c {}
unsafe impl Sync for Lpi2c {}
impl Lpi2c {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID."]
    #[inline(always)]
    pub const fn verid(self) -> crate::pac::common::Reg<Verid, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameter."]
    #[inline(always)]
    pub const fn param(self) -> crate::pac::common::Reg<Param, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Controller Control."]
    #[inline(always)]
    pub const fn mcr(self) -> crate::pac::common::Reg<Mcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Controller Status."]
    #[inline(always)]
    pub const fn msr(self) -> crate::pac::common::Reg<Msr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Controller Interrupt Enable."]
    #[inline(always)]
    pub const fn mier(self) -> crate::pac::common::Reg<Mier, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Controller DMA Enable."]
    #[inline(always)]
    pub const fn mder(self) -> crate::pac::common::Reg<Mder, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Controller Configuration 0."]
    #[inline(always)]
    pub const fn mcfgr0(self) -> crate::pac::common::Reg<Mcfgr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Controller Configuration 1."]
    #[inline(always)]
    pub const fn mcfgr1(self) -> crate::pac::common::Reg<Mcfgr1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Controller Configuration 2."]
    #[inline(always)]
    pub const fn mcfgr2(self) -> crate::pac::common::Reg<Mcfgr2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Controller Configuration 3."]
    #[inline(always)]
    pub const fn mcfgr3(self) -> crate::pac::common::Reg<Mcfgr3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Controller Data Match."]
    #[inline(always)]
    pub const fn mdmr(self) -> crate::pac::common::Reg<Mdmr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Controller Clock Configuration 0."]
    #[inline(always)]
    pub const fn mccr0(self) -> crate::pac::common::Reg<Mccr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Controller Clock Configuration 1."]
    #[inline(always)]
    pub const fn mccr1(self) -> crate::pac::common::Reg<Mccr1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Controller FIFO Control."]
    #[inline(always)]
    pub const fn mfcr(self) -> crate::pac::common::Reg<Mfcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Controller FIFO Status."]
    #[inline(always)]
    pub const fn mfsr(self) -> crate::pac::common::Reg<Mfsr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Controller Transmit Data."]
    #[inline(always)]
    pub const fn mtdr(self) -> crate::pac::common::Reg<Mtdr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Controller Receive Data."]
    #[inline(always)]
    pub const fn mrdr(self) -> crate::pac::common::Reg<Mrdr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Controller Receive Data Read Only."]
    #[inline(always)]
    pub const fn mrdror(self) -> crate::pac::common::Reg<Mrdror, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Target Control."]
    #[inline(always)]
    pub const fn scr(self) -> crate::pac::common::Reg<Scr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Target Status."]
    #[inline(always)]
    pub const fn ssr(self) -> crate::pac::common::Reg<Ssr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Target Interrupt Enable."]
    #[inline(always)]
    pub const fn sier(self) -> crate::pac::common::Reg<Sier, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Target DMA Enable."]
    #[inline(always)]
    pub const fn sder(self) -> crate::pac::common::Reg<Sder, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Target Configuration 0."]
    #[inline(always)]
    pub const fn scfgr0(self) -> crate::pac::common::Reg<Scfgr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Target Configuration 1."]
    #[inline(always)]
    pub const fn scfgr1(self) -> crate::pac::common::Reg<Scfgr1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Target Configuration 2."]
    #[inline(always)]
    pub const fn scfgr2(self) -> crate::pac::common::Reg<Scfgr2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Target Address Match."]
    #[inline(always)]
    pub const fn samr(self) -> crate::pac::common::Reg<Samr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Target Address Status."]
    #[inline(always)]
    pub const fn sasr(self) -> crate::pac::common::Reg<Sasr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Target Transmit ACK."]
    #[inline(always)]
    pub const fn star(self) -> crate::pac::common::Reg<Star, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Target Transmit Data."]
    #[inline(always)]
    pub const fn stdr(self) -> crate::pac::common::Reg<Stdr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Target Receive Data."]
    #[inline(always)]
    pub const fn srdr(self) -> crate::pac::common::Reg<Srdr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Target Receive Data Read Only."]
    #[inline(always)]
    pub const fn srdror(self) -> crate::pac::common::Reg<Srdror, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
}
#[doc = "Controller Clock Configuration 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mccr0(pub u32);
impl Mccr0 {
    #[doc = "Clock Low Period."]
    #[must_use]
    #[inline(always)]
    pub const fn clklo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock Low Period."]
    #[inline(always)]
    pub const fn set_clklo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Clock High Period."]
    #[must_use]
    #[inline(always)]
    pub const fn clkhi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock High Period."]
    #[inline(always)]
    pub const fn set_clkhi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Setup Hold Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn sethold(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Setup Hold Delay."]
    #[inline(always)]
    pub const fn set_sethold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Data Valid Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn datavd(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Data Valid Delay."]
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
#[doc = "Controller Clock Configuration 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mccr1(pub u32);
impl Mccr1 {
    #[doc = "Clock Low Period."]
    #[must_use]
    #[inline(always)]
    pub const fn clklo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock Low Period."]
    #[inline(always)]
    pub const fn set_clklo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Clock High Period."]
    #[must_use]
    #[inline(always)]
    pub const fn clkhi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock High Period."]
    #[inline(always)]
    pub const fn set_clkhi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Setup Hold Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn sethold(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Setup Hold Delay."]
    #[inline(always)]
    pub const fn set_sethold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Data Valid Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn datavd(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Data Valid Delay."]
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
#[doc = "Controller Configuration 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcfgr0(pub u32);
impl Mcfgr0 {
    #[doc = "Host Request Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Host Request Enable."]
    #[inline(always)]
    pub const fn set_hren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Request Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn hrpol(&self) -> Hrpol {
        let val = (self.0 >> 1usize) & 0x01;
        Hrpol::from_bits(val as u8)
    }
    #[doc = "Host Request Polarity."]
    #[inline(always)]
    pub const fn set_hrpol(&mut self, val: Hrpol) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Host Request Select."]
    #[must_use]
    #[inline(always)]
    pub const fn hrsel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Host Request Select."]
    #[inline(always)]
    pub const fn set_hrsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Host Request Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn hrdir(&self) -> Hrdir {
        let val = (self.0 >> 3usize) & 0x01;
        Hrdir::from_bits(val as u8)
    }
    #[doc = "Host Request Direction."]
    #[inline(always)]
    pub const fn set_hrdir(&mut self, val: Hrdir) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Circular FIFO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cirfifo(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Circular FIFO Enable."]
    #[inline(always)]
    pub const fn set_cirfifo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive Data Match Only."]
    #[must_use]
    #[inline(always)]
    pub const fn rdmo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Match Only."]
    #[inline(always)]
    pub const fn set_rdmo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Relaxed Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn relax(&self) -> Relax {
        let val = (self.0 >> 16usize) & 0x01;
        Relax::from_bits(val as u8)
    }
    #[doc = "Relaxed Mode."]
    #[inline(always)]
    pub const fn set_relax(&mut self, val: Relax) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Abort Transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn abort(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Abort Transfer."]
    #[inline(always)]
    pub const fn set_abort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
            .field("hrdir", &self.hrdir())
            .field("cirfifo", &self.cirfifo())
            .field("rdmo", &self.rdmo())
            .field("relax", &self.relax())
            .field("abort", &self.abort())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcfgr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcfgr0 {{ hren: {=bool:?}, hrpol: {:?}, hrsel: {=bool:?}, hrdir: {:?}, cirfifo: {=bool:?}, rdmo: {=bool:?}, relax: {:?}, abort: {=bool:?} }}",
            self.hren(),
            self.hrpol(),
            self.hrsel(),
            self.hrdir(),
            self.cirfifo(),
            self.rdmo(),
            self.relax(),
            self.abort()
        )
    }
}
#[doc = "Controller Configuration 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcfgr1(pub u32);
impl Mcfgr1 {
    #[doc = "Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn prescale(&self) -> Prescale {
        let val = (self.0 >> 0usize) & 0x07;
        Prescale::from_bits(val as u8)
    }
    #[doc = "Prescaler."]
    #[inline(always)]
    pub const fn set_prescale(&mut self, val: Prescale) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Automatic Stop Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn autostop(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic Stop Generation."]
    #[inline(always)]
    pub const fn set_autostop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Ignore NACK."]
    #[must_use]
    #[inline(always)]
    pub const fn ignack(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore NACK."]
    #[inline(always)]
    pub const fn set_ignack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Timeout Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn timecfg(&self) -> Timecfg {
        let val = (self.0 >> 10usize) & 0x01;
        Timecfg::from_bits(val as u8)
    }
    #[doc = "Timeout Configuration."]
    #[inline(always)]
    pub const fn set_timecfg(&mut self, val: Timecfg) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Stop Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn stopcfg(&self) -> Stopcfg {
        let val = (self.0 >> 11usize) & 0x01;
        Stopcfg::from_bits(val as u8)
    }
    #[doc = "Stop Configuration."]
    #[inline(always)]
    pub const fn set_stopcfg(&mut self, val: Stopcfg) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Start Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn startcfg(&self) -> Startcfg {
        let val = (self.0 >> 12usize) & 0x01;
        Startcfg::from_bits(val as u8)
    }
    #[doc = "Start Configuration."]
    #[inline(always)]
    pub const fn set_startcfg(&mut self, val: Startcfg) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Match Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn matcfg(&self) -> Matcfg {
        let val = (self.0 >> 16usize) & 0x07;
        Matcfg::from_bits(val as u8)
    }
    #[doc = "Match Configuration."]
    #[inline(always)]
    pub const fn set_matcfg(&mut self, val: Matcfg) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Pin Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pincfg(&self) -> Pincfg {
        let val = (self.0 >> 24usize) & 0x07;
        Pincfg::from_bits(val as u8)
    }
    #[doc = "Pin Configuration."]
    #[inline(always)]
    pub const fn set_pincfg(&mut self, val: Pincfg) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "Force HS Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn frchs(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Force HS Mode."]
    #[inline(always)]
    pub const fn set_frchs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
            .field("stopcfg", &self.stopcfg())
            .field("startcfg", &self.startcfg())
            .field("matcfg", &self.matcfg())
            .field("pincfg", &self.pincfg())
            .field("frchs", &self.frchs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcfgr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcfgr1 {{ prescale: {:?}, autostop: {=bool:?}, ignack: {=bool:?}, timecfg: {:?}, stopcfg: {:?}, startcfg: {:?}, matcfg: {:?}, pincfg: {:?}, frchs: {=bool:?} }}",
            self.prescale(),
            self.autostop(),
            self.ignack(),
            self.timecfg(),
            self.stopcfg(),
            self.startcfg(),
            self.matcfg(),
            self.pincfg(),
            self.frchs()
        )
    }
}
#[doc = "Controller Configuration 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcfgr2(pub u32);
impl Mcfgr2 {
    #[doc = "Bus Idle Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn busidle(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Bus Idle Timeout."]
    #[inline(always)]
    pub const fn set_busidle(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Glitch Filter SCL."]
    #[must_use]
    #[inline(always)]
    pub const fn filtscl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Glitch Filter SCL."]
    #[inline(always)]
    pub const fn set_filtscl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Glitch Filter SDA."]
    #[must_use]
    #[inline(always)]
    pub const fn filtsda(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Glitch Filter SDA."]
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
#[doc = "Controller Configuration 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcfgr3(pub u32);
impl Mcfgr3 {
    #[doc = "Pin Low Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn pinlow(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "Pin Low Timeout."]
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
#[doc = "Controller Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc = "Controller Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn men(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Controller Enable."]
    #[inline(always)]
    pub const fn set_men(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Doze Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dozen(&self) -> Dozen {
        let val = (self.0 >> 2usize) & 0x01;
        Dozen::from_bits(val as u8)
    }
    #[doc = "Doze Mode Enable."]
    #[inline(always)]
    pub const fn set_dozen(&mut self, val: Dozen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Debug Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable."]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reset Transmit FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn rtf(&self) -> McrRtf {
        let val = (self.0 >> 8usize) & 0x01;
        McrRtf::from_bits(val as u8)
    }
    #[doc = "Reset Transmit FIFO."]
    #[inline(always)]
    pub const fn set_rtf(&mut self, val: McrRtf) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Reset Receive FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn rrf(&self) -> McrRrf {
        let val = (self.0 >> 9usize) & 0x01;
        McrRrf::from_bits(val as u8)
    }
    #[doc = "Reset Receive FIFO."]
    #[inline(always)]
    pub const fn set_rrf(&mut self, val: McrRrf) {
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
#[doc = "Controller DMA Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mder(pub u32);
impl Mder {
    #[doc = "Transmit Data DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tdde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data DMA Enable."]
    #[inline(always)]
    pub const fn set_tdde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rdde(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data DMA Enable."]
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
#[doc = "Controller Data Match."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdmr(pub u32);
impl Mdmr {
    #[doc = "Match 0 Value."]
    #[must_use]
    #[inline(always)]
    pub const fn match0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Match 0 Value."]
    #[inline(always)]
    pub const fn set_match0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Match 1 Value."]
    #[must_use]
    #[inline(always)]
    pub const fn match1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Match 1 Value."]
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
#[doc = "Controller FIFO Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mfcr(pub u32);
impl Mfcr {
    #[doc = "Transmit FIFO Watermark."]
    #[must_use]
    #[inline(always)]
    pub const fn txwater(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Transmit FIFO Watermark."]
    #[inline(always)]
    pub const fn set_txwater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Receive FIFO Watermark."]
    #[must_use]
    #[inline(always)]
    pub const fn rxwater(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Receive FIFO Watermark."]
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
#[doc = "Controller FIFO Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mfsr(pub u32);
impl Mfsr {
    #[doc = "Transmit FIFO Count."]
    #[must_use]
    #[inline(always)]
    pub const fn txcount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit FIFO Count."]
    #[inline(always)]
    pub const fn set_txcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Receive FIFO Count."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Receive FIFO Count."]
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
#[doc = "Controller Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mier(pub u32);
impl Mier {
    #[doc = "Transmit Data Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tdie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rdie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "End Packet Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn epie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "End Packet Interrupt Enable."]
    #[inline(always)]
    pub const fn set_epie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Stop Detect Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sdie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Detect Interrupt Enable."]
    #[inline(always)]
    pub const fn set_sdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "NACK Detect Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ndie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "NACK Detect Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ndie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Arbitration Lost Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn alie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration Lost Interrupt Enable."]
    #[inline(always)]
    pub const fn set_alie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FIFO Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn feie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_feie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pin Low Timeout Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pltie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pin Low Timeout Interrupt Enable."]
    #[inline(always)]
    pub const fn set_pltie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Data Match Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Data Match Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dmie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Start Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn stie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Start Interrupt Enable."]
    #[inline(always)]
    pub const fn set_stie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
            .field("stie", &self.stie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mier {{ tdie: {=bool:?}, rdie: {=bool:?}, epie: {=bool:?}, sdie: {=bool:?}, ndie: {=bool:?}, alie: {=bool:?}, feie: {=bool:?}, pltie: {=bool:?}, dmie: {=bool:?}, stie: {=bool:?} }}",
            self.tdie(),
            self.rdie(),
            self.epie(),
            self.sdie(),
            self.ndie(),
            self.alie(),
            self.feie(),
            self.pltie(),
            self.dmie(),
            self.stie()
        )
    }
}
#[doc = "Controller Receive Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrdr(pub u32);
impl Mrdr {
    #[doc = "Receive Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn rxempty(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Empty."]
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
#[doc = "Controller Receive Data Read Only."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrdror(pub u32);
impl Mrdror {
    #[doc = "Receive Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "RX Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn rxempty(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "RX Empty."]
    #[inline(always)]
    pub const fn set_rxempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Mrdror {
    #[inline(always)]
    fn default() -> Mrdror {
        Mrdror(0)
    }
}
impl core::fmt::Debug for Mrdror {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mrdror")
            .field("data", &self.data())
            .field("rxempty", &self.rxempty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mrdror {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mrdror {{ data: {=u8:?}, rxempty: {=bool:?} }}",
            self.data(),
            self.rxempty()
        )
    }
}
#[doc = "Controller Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msr(pub u32);
impl Msr {
    #[doc = "Transmit Data Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tdf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Flag."]
    #[inline(always)]
    pub const fn set_tdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rdf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Flag."]
    #[inline(always)]
    pub const fn set_rdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "End Packet Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn epf(&self) -> Epf {
        let val = (self.0 >> 8usize) & 0x01;
        Epf::from_bits(val as u8)
    }
    #[doc = "End Packet Flag."]
    #[inline(always)]
    pub const fn set_epf(&mut self, val: Epf) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Stop Detect Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sdf(&self) -> MsrSdf {
        let val = (self.0 >> 9usize) & 0x01;
        MsrSdf::from_bits(val as u8)
    }
    #[doc = "Stop Detect Flag."]
    #[inline(always)]
    pub const fn set_sdf(&mut self, val: MsrSdf) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "NACK Detect Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ndf(&self) -> Ndf {
        let val = (self.0 >> 10usize) & 0x01;
        Ndf::from_bits(val as u8)
    }
    #[doc = "NACK Detect Flag."]
    #[inline(always)]
    pub const fn set_ndf(&mut self, val: Ndf) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Arbitration Lost Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn alf(&self) -> Alf {
        let val = (self.0 >> 11usize) & 0x01;
        Alf::from_bits(val as u8)
    }
    #[doc = "Arbitration Lost Flag."]
    #[inline(always)]
    pub const fn set_alf(&mut self, val: Alf) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "FIFO Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fef(&self) -> MsrFef {
        let val = (self.0 >> 12usize) & 0x01;
        MsrFef::from_bits(val as u8)
    }
    #[doc = "FIFO Error Flag."]
    #[inline(always)]
    pub const fn set_fef(&mut self, val: MsrFef) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pin Low Timeout Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pltf(&self) -> Pltf {
        let val = (self.0 >> 13usize) & 0x01;
        Pltf::from_bits(val as u8)
    }
    #[doc = "Pin Low Timeout Flag."]
    #[inline(always)]
    pub const fn set_pltf(&mut self, val: Pltf) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Data Match Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dmf(&self) -> Dmf {
        let val = (self.0 >> 14usize) & 0x01;
        Dmf::from_bits(val as u8)
    }
    #[doc = "Data Match Flag."]
    #[inline(always)]
    pub const fn set_dmf(&mut self, val: Dmf) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Start Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn stf(&self) -> Stf {
        let val = (self.0 >> 15usize) & 0x01;
        Stf::from_bits(val as u8)
    }
    #[doc = "Start Flag."]
    #[inline(always)]
    pub const fn set_stf(&mut self, val: Stf) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Controller Busy Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn mbf(&self) -> Mbf {
        let val = (self.0 >> 24usize) & 0x01;
        Mbf::from_bits(val as u8)
    }
    #[doc = "Controller Busy Flag."]
    #[inline(always)]
    pub const fn set_mbf(&mut self, val: Mbf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus Busy Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bbf(&self) -> MsrBbf {
        let val = (self.0 >> 25usize) & 0x01;
        MsrBbf::from_bits(val as u8)
    }
    #[doc = "Bus Busy Flag."]
    #[inline(always)]
    pub const fn set_bbf(&mut self, val: MsrBbf) {
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
            .field("stf", &self.stf())
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
            "Msr {{ tdf: {=bool:?}, rdf: {=bool:?}, epf: {:?}, sdf: {:?}, ndf: {:?}, alf: {:?}, fef: {:?}, pltf: {:?}, dmf: {:?}, stf: {:?}, mbf: {:?}, bbf: {:?} }}",
            self.tdf(),
            self.rdf(),
            self.epf(),
            self.sdf(),
            self.ndf(),
            self.alf(),
            self.fef(),
            self.pltf(),
            self.dmf(),
            self.stf(),
            self.mbf(),
            self.bbf()
        )
    }
}
#[doc = "Controller Transmit Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtdr(pub u32);
impl Mtdr {
    #[doc = "Transmit Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Command Data."]
    #[must_use]
    #[inline(always)]
    pub const fn cmd(&self) -> Cmd {
        let val = (self.0 >> 8usize) & 0x07;
        Cmd::from_bits(val as u8)
    }
    #[doc = "Command Data."]
    #[inline(always)]
    pub const fn set_cmd(&mut self, val: Cmd) {
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
#[doc = "Parameter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Controller Transmit FIFO Size."]
    #[must_use]
    #[inline(always)]
    pub const fn mtxfifo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Controller Transmit FIFO Size."]
    #[inline(always)]
    pub const fn set_mtxfifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Controller Receive FIFO Size."]
    #[must_use]
    #[inline(always)]
    pub const fn mrxfifo(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Controller Receive FIFO Size."]
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
#[doc = "Target Address Match."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Samr(pub u32);
impl Samr {
    #[doc = "Address 0 Value."]
    #[must_use]
    #[inline(always)]
    pub const fn addr0(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[doc = "Address 0 Value."]
    #[inline(always)]
    pub const fn set_addr0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
    #[doc = "Address 1 Value."]
    #[must_use]
    #[inline(always)]
    pub const fn addr1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x03ff;
        val as u16
    }
    #[doc = "Address 1 Value."]
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
#[doc = "Target Address Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sasr(pub u32);
impl Sasr {
    #[doc = "Received Address."]
    #[must_use]
    #[inline(always)]
    pub const fn raddr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Received Address."]
    #[inline(always)]
    pub const fn set_raddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Address Not Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn anv(&self) -> Anv {
        let val = (self.0 >> 14usize) & 0x01;
        Anv::from_bits(val as u8)
    }
    #[doc = "Address Not Valid."]
    #[inline(always)]
    pub const fn set_anv(&mut self, val: Anv) {
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
#[doc = "Target Configuration 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scfgr0(pub u32);
impl Scfgr0 {
    #[doc = "Read Request."]
    #[must_use]
    #[inline(always)]
    pub const fn rdreq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read Request."]
    #[inline(always)]
    pub const fn set_rdreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Read Acknowledge Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rdack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Read Acknowledge Flag."]
    #[inline(always)]
    pub const fn set_rdack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Scfgr0 {
    #[inline(always)]
    fn default() -> Scfgr0 {
        Scfgr0(0)
    }
}
impl core::fmt::Debug for Scfgr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scfgr0")
            .field("rdreq", &self.rdreq())
            .field("rdack", &self.rdack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scfgr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scfgr0 {{ rdreq: {=bool:?}, rdack: {=bool:?} }}",
            self.rdreq(),
            self.rdack()
        )
    }
}
#[doc = "Target Configuration 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scfgr1(pub u32);
impl Scfgr1 {
    #[doc = "Address SCL Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn adrstall(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Address SCL Stall."]
    #[inline(always)]
    pub const fn set_adrstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX SCL Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn rxstall(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX SCL Stall."]
    #[inline(always)]
    pub const fn set_rxstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Data SCL Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn txdstall(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data SCL Stall."]
    #[inline(always)]
    pub const fn set_txdstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ACK SCL Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn ackstall(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ACK SCL Stall."]
    #[inline(always)]
    pub const fn set_ackstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive NACK."]
    #[must_use]
    #[inline(always)]
    pub const fn rxnack(&self) -> Rxnack {
        let val = (self.0 >> 4usize) & 0x01;
        Rxnack::from_bits(val as u8)
    }
    #[doc = "Receive NACK."]
    #[inline(always)]
    pub const fn set_rxnack(&mut self, val: Rxnack) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "General Call Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gcen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "General Call Enable."]
    #[inline(always)]
    pub const fn set_gcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SMBus Alert Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn saen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SMBus Alert Enable."]
    #[inline(always)]
    pub const fn set_saen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmit Flag Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn txcfg(&self) -> Txcfg {
        let val = (self.0 >> 10usize) & 0x01;
        Txcfg::from_bits(val as u8)
    }
    #[doc = "Transmit Flag Configuration."]
    #[inline(always)]
    pub const fn set_txcfg(&mut self, val: Txcfg) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Receive Data Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcfg(&self) -> Rxcfg {
        let val = (self.0 >> 11usize) & 0x01;
        Rxcfg::from_bits(val as u8)
    }
    #[doc = "Receive Data Configuration."]
    #[inline(always)]
    pub const fn set_rxcfg(&mut self, val: Rxcfg) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Ignore NACK."]
    #[must_use]
    #[inline(always)]
    pub const fn ignack(&self) -> Scfgr1Ignack {
        let val = (self.0 >> 12usize) & 0x01;
        Scfgr1Ignack::from_bits(val as u8)
    }
    #[doc = "Ignore NACK."]
    #[inline(always)]
    pub const fn set_ignack(&mut self, val: Scfgr1Ignack) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "HS Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hsmen(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "HS Mode Enable."]
    #[inline(always)]
    pub const fn set_hsmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Address Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn addrcfg(&self) -> Addrcfg {
        let val = (self.0 >> 16usize) & 0x07;
        Addrcfg::from_bits(val as u8)
    }
    #[doc = "Address Configuration."]
    #[inline(always)]
    pub const fn set_addrcfg(&mut self, val: Addrcfg) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Receive All."]
    #[must_use]
    #[inline(always)]
    pub const fn rxall(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Receive All."]
    #[inline(always)]
    pub const fn set_rxall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Repeated Start Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn rscfg(&self) -> Rscfg {
        let val = (self.0 >> 25usize) & 0x01;
        Rscfg::from_bits(val as u8)
    }
    #[doc = "Repeated Start Configuration."]
    #[inline(always)]
    pub const fn set_rscfg(&mut self, val: Rscfg) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Stop Detect Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn sdcfg(&self) -> Sdcfg {
        let val = (self.0 >> 26usize) & 0x01;
        Sdcfg::from_bits(val as u8)
    }
    #[doc = "Stop Detect Configuration."]
    #[inline(always)]
    pub const fn set_sdcfg(&mut self, val: Sdcfg) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
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
            .field("rxnack", &self.rxnack())
            .field("gcen", &self.gcen())
            .field("saen", &self.saen())
            .field("txcfg", &self.txcfg())
            .field("rxcfg", &self.rxcfg())
            .field("ignack", &self.ignack())
            .field("hsmen", &self.hsmen())
            .field("addrcfg", &self.addrcfg())
            .field("rxall", &self.rxall())
            .field("rscfg", &self.rscfg())
            .field("sdcfg", &self.sdcfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scfgr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scfgr1 {{ adrstall: {=bool:?}, rxstall: {=bool:?}, txdstall: {=bool:?}, ackstall: {=bool:?}, rxnack: {:?}, gcen: {=bool:?}, saen: {=bool:?}, txcfg: {:?}, rxcfg: {:?}, ignack: {:?}, hsmen: {=bool:?}, addrcfg: {:?}, rxall: {=bool:?}, rscfg: {:?}, sdcfg: {:?} }}",
            self.adrstall(),
            self.rxstall(),
            self.txdstall(),
            self.ackstall(),
            self.rxnack(),
            self.gcen(),
            self.saen(),
            self.txcfg(),
            self.rxcfg(),
            self.ignack(),
            self.hsmen(),
            self.addrcfg(),
            self.rxall(),
            self.rscfg(),
            self.sdcfg()
        )
    }
}
#[doc = "Target Configuration 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scfgr2(pub u32);
impl Scfgr2 {
    #[doc = "Clock Hold Time."]
    #[must_use]
    #[inline(always)]
    pub const fn clkhold(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock Hold Time."]
    #[inline(always)]
    pub const fn set_clkhold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Data Valid Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn datavd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Data Valid Delay."]
    #[inline(always)]
    pub const fn set_datavd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Glitch Filter SCL."]
    #[must_use]
    #[inline(always)]
    pub const fn filtscl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Glitch Filter SCL."]
    #[inline(always)]
    pub const fn set_filtscl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Glitch Filter SDA."]
    #[must_use]
    #[inline(always)]
    pub const fn filtsda(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Glitch Filter SDA."]
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
#[doc = "Target Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc = "Target Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Target Enable."]
    #[inline(always)]
    pub const fn set_sen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Filter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn filten(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Filter Enable."]
    #[inline(always)]
    pub const fn set_filten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Filter Doze Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn filtdz(&self) -> Filtdz {
        let val = (self.0 >> 5usize) & 0x01;
        Filtdz::from_bits(val as u8)
    }
    #[doc = "Filter Doze Enable."]
    #[inline(always)]
    pub const fn set_filtdz(&mut self, val: Filtdz) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Reset Transmit FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn rtf(&self) -> ScrRtf {
        let val = (self.0 >> 8usize) & 0x01;
        ScrRtf::from_bits(val as u8)
    }
    #[doc = "Reset Transmit FIFO."]
    #[inline(always)]
    pub const fn set_rtf(&mut self, val: ScrRtf) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Reset Receive FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn rrf(&self) -> ScrRrf {
        let val = (self.0 >> 9usize) & 0x01;
        ScrRrf::from_bits(val as u8)
    }
    #[doc = "Reset Receive FIFO."]
    #[inline(always)]
    pub const fn set_rrf(&mut self, val: ScrRrf) {
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
#[doc = "Target DMA Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sder(pub u32);
impl Sder {
    #[doc = "Transmit Data DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tdde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data DMA Enable."]
    #[inline(always)]
    pub const fn set_tdde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rdde(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data DMA Enable."]
    #[inline(always)]
    pub const fn set_rdde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Address Valid DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn avde(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Address Valid DMA Enable."]
    #[inline(always)]
    pub const fn set_avde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Repeated Start DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rsde(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Repeated Start DMA Enable."]
    #[inline(always)]
    pub const fn set_rsde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Stop Detect DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sdde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Detect DMA Enable."]
    #[inline(always)]
    pub const fn set_sdde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("rsde", &self.rsde())
            .field("sdde", &self.sdde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sder {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sder {{ tdde: {=bool:?}, rdde: {=bool:?}, avde: {=bool:?}, rsde: {=bool:?}, sdde: {=bool:?} }}",
            self.tdde(),
            self.rdde(),
            self.avde(),
            self.rsde(),
            self.sdde()
        )
    }
}
#[doc = "Target Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sier(pub u32);
impl Sier {
    #[doc = "Transmit Data Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tdie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rdie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Address Valid Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn avie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Address Valid Interrupt Enable."]
    #[inline(always)]
    pub const fn set_avie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit ACK Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn taie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit ACK Interrupt Enable."]
    #[inline(always)]
    pub const fn set_taie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Repeated Start Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rsie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Repeated Start Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Stop Detect Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sdie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Detect Interrupt Enable."]
    #[inline(always)]
    pub const fn set_sdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn beie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_beie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "FIFO Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn feie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_feie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Address Match 0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn am0ie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Address Match 0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_am0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Address Match 1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn am1ie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Address Match 1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_am1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "General Call Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gcie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "General Call Interrupt Enable."]
    #[inline(always)]
    pub const fn set_gcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SMBus Alert Response Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sarie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SMBus Alert Response Interrupt Enable."]
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
#[doc = "Target Receive Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srdr(pub u32);
impl Srdr {
    #[doc = "Received Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Received Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Received Address."]
    #[must_use]
    #[inline(always)]
    pub const fn raddr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Received Address."]
    #[inline(always)]
    pub const fn set_raddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Receive Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn rxempty(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Empty."]
    #[inline(always)]
    pub const fn set_rxempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Start of Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn sof(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Frame."]
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
            .field("raddr", &self.raddr())
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
            "Srdr {{ data: {=u8:?}, raddr: {=u8:?}, rxempty: {=bool:?}, sof: {=bool:?} }}",
            self.data(),
            self.raddr(),
            self.rxempty(),
            self.sof()
        )
    }
}
#[doc = "Target Receive Data Read Only."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srdror(pub u32);
impl Srdror {
    #[doc = "Receive Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Received Address."]
    #[must_use]
    #[inline(always)]
    pub const fn raddr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Received Address."]
    #[inline(always)]
    pub const fn set_raddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Receive Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn rxempty(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Empty."]
    #[inline(always)]
    pub const fn set_rxempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Start of Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn sof(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Frame."]
    #[inline(always)]
    pub const fn set_sof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Srdror {
    #[inline(always)]
    fn default() -> Srdror {
        Srdror(0)
    }
}
impl core::fmt::Debug for Srdror {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srdror")
            .field("data", &self.data())
            .field("raddr", &self.raddr())
            .field("rxempty", &self.rxempty())
            .field("sof", &self.sof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srdror {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Srdror {{ data: {=u8:?}, raddr: {=u8:?}, rxempty: {=bool:?}, sof: {=bool:?} }}",
            self.data(),
            self.raddr(),
            self.rxempty(),
            self.sof()
        )
    }
}
#[doc = "Target Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssr(pub u32);
impl Ssr {
    #[doc = "Transmit Data Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tdf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Flag."]
    #[inline(always)]
    pub const fn set_tdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rdf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Flag."]
    #[inline(always)]
    pub const fn set_rdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Address Valid Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn avf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Address Valid Flag."]
    #[inline(always)]
    pub const fn set_avf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit ACK Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn taf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit ACK Flag."]
    #[inline(always)]
    pub const fn set_taf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Repeated Start Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rsf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Repeated Start Flag."]
    #[inline(always)]
    pub const fn set_rsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Stop Detect Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sdf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Detect Flag."]
    #[inline(always)]
    pub const fn set_sdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bef(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Error Flag."]
    #[inline(always)]
    pub const fn set_bef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "FIFO Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fef(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Flag."]
    #[inline(always)]
    pub const fn set_fef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Address Match 0 Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn am0f(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Address Match 0 Flag."]
    #[inline(always)]
    pub const fn set_am0f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Address Match 1 Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn am1f(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Address Match 1 Flag."]
    #[inline(always)]
    pub const fn set_am1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "General Call Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn gcf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "General Call Flag."]
    #[inline(always)]
    pub const fn set_gcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SMBus Alert Response Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sarf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SMBus Alert Response Flag."]
    #[inline(always)]
    pub const fn set_sarf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Target Busy Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sbf(&self) -> Sbf {
        let val = (self.0 >> 24usize) & 0x01;
        Sbf::from_bits(val as u8)
    }
    #[doc = "Target Busy Flag."]
    #[inline(always)]
    pub const fn set_sbf(&mut self, val: Sbf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus Busy Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bbf(&self) -> SsrBbf {
        let val = (self.0 >> 25usize) & 0x01;
        SsrBbf::from_bits(val as u8)
    }
    #[doc = "Bus Busy Flag."]
    #[inline(always)]
    pub const fn set_bbf(&mut self, val: SsrBbf) {
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
#[doc = "Target Transmit ACK."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Star(pub u32);
impl Star {
    #[doc = "Transmit NACK."]
    #[must_use]
    #[inline(always)]
    pub const fn txnack(&self) -> Txnack {
        let val = (self.0 >> 0usize) & 0x01;
        Txnack::from_bits(val as u8)
    }
    #[doc = "Transmit NACK."]
    #[inline(always)]
    pub const fn set_txnack(&mut self, val: Txnack) {
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
#[doc = "Target Transmit Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stdr(pub u32);
impl Stdr {
    #[doc = "Transmit Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Data."]
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
#[doc = "Version ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number."]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number."]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number."]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number."]
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Addrcfg {
    #[doc = "Address match 0 (7-bit)."]
    AddressMatch07Bit = 0x0,
    #[doc = "Address match 0 (10-bit)."]
    AddressMatch010Bit = 0x01,
    #[doc = "Address match 0 (7-bit) or address match 1 (7-bit)."]
    AddressMatch07BitOrAddressMatch17Bit = 0x02,
    #[doc = "Address match 0 (10-bit) or address match 1 (10-bit)."]
    AddressMatch010BitOrAddressMatch110Bit = 0x03,
    #[doc = "Address match 0 (7-bit) or address match 1 (10-bit)."]
    AddressMatch07BitOrAddressMatch110Bit = 0x04,
    #[doc = "Address match 0 (10-bit) or address match 1 (7-bit)."]
    AddressMatch010BitOrAddressMatch17Bit = 0x05,
    #[doc = "From address match 0 (7-bit) to address match 1 (7-bit)."]
    FromAddressMatch07BitToAddressMatch17Bit = 0x06,
    #[doc = "From address match 0 (10-bit) to address match 1 (10-bit)."]
    FromAddressMatch010BitToAddressMatch110Bit = 0x07,
}
impl Addrcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addrcfg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addrcfg {
    #[inline(always)]
    fn from(val: u8) -> Addrcfg {
        Addrcfg::from_bits(val)
    }
}
impl From<Addrcfg> for u8 {
    #[inline(always)]
    fn from(val: Addrcfg) -> u8 {
        Addrcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Alf {
    #[doc = "Controller did not lose arbitration."]
    IntNo = 0x0,
    #[doc = "Controller lost arbitration."]
    IntYes = 0x01,
}
impl Alf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Alf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Alf {
    #[inline(always)]
    fn from(val: u8) -> Alf {
        Alf::from_bits(val)
    }
}
impl From<Alf> for u8 {
    #[inline(always)]
    fn from(val: Alf) -> u8 {
        Alf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Anv {
    #[doc = "Valid."]
    Valid = 0x0,
    #[doc = "Not valid."]
    NotValid = 0x01,
}
impl Anv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Anv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Anv {
    #[inline(always)]
    fn from(val: u8) -> Anv {
        Anv::from_bits(val)
    }
}
impl From<Anv> for u8 {
    #[inline(always)]
    fn from(val: Anv) -> u8 {
        Anv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmd {
    #[doc = "Transmit value in DATA\\[7:0\\]"]
    TRANSMIT = 0x0,
    #[doc = "Receive (DATA\\[7:0\\] + 1) bytes."]
    RECEIVE = 0x01,
    #[doc = "Generate Stop condition on I2C bus."]
    STOP = 0x02,
    #[doc = "Receive and discard (DATA\\[7:0\\] + 1) bytes."]
    RECEIVE_AND_DISCARD = 0x03,
    #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\]"]
    START = 0x04,
    #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] expecting a NACK response"]
    START_EXPECT_NACK = 0x05,
    #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] using HS mode"]
    START_HS = 0x06,
    #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] using HS mode expecting a NACK response"]
    START_HS_EXPECT_NACK = 0x07,
}
impl Cmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmd {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmd {
    #[inline(always)]
    fn from(val: u8) -> Cmd {
        Cmd::from_bits(val)
    }
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(val: Cmd) -> u8 {
        Cmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmf {
    #[doc = "Matching data not received."]
    IntNo = 0x0,
    #[doc = "Matching data received."]
    IntYes = 0x01,
}
impl Dmf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmf {
    #[inline(always)]
    fn from(val: u8) -> Dmf {
        Dmf::from_bits(val)
    }
}
impl From<Dmf> for u8 {
    #[inline(always)]
    fn from(val: Dmf) -> u8 {
        Dmf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozen {
    #[doc = "Enable."]
    Enabled = 0x0,
    #[doc = "Disable."]
    Disabled = 0x01,
}
impl Dozen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozen {
    #[inline(always)]
    fn from(val: u8) -> Dozen {
        Dozen::from_bits(val)
    }
}
impl From<Dozen> for u8 {
    #[inline(always)]
    fn from(val: Dozen) -> u8 {
        Dozen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Epf {
    #[doc = "No Stop or repeated Start generated."]
    IntNo = 0x0,
    #[doc = "Stop or repeated Start generated."]
    IntYes = 0x01,
}
impl Epf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Epf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Epf {
    #[inline(always)]
    fn from(val: u8) -> Epf {
        Epf::from_bits(val)
    }
}
impl From<Epf> for u8 {
    #[inline(always)]
    fn from(val: Epf) -> u8 {
        Epf::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Controller only, with standard feature set."]
    pub const MasterOnly: Self = Self(0x02);
    #[doc = "Controller and target, with standard feature set."]
    pub const MasterAndSlave: Self = Self(0x03);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x02 => f.write_str("MasterOnly"),
            0x03 => f.write_str("MasterAndSlave"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x02 => defmt::write!(f, "MasterOnly"),
            0x03 => defmt::write!(f, "MasterAndSlave"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Filtdz {
    #[doc = "Enable."]
    FilterEnabled = 0x0,
    #[doc = "Disable."]
    FilterDisabled = 0x01,
}
impl Filtdz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filtdz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filtdz {
    #[inline(always)]
    fn from(val: u8) -> Filtdz {
        Filtdz::from_bits(val)
    }
}
impl From<Filtdz> for u8 {
    #[inline(always)]
    fn from(val: Filtdz) -> u8 {
        Filtdz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hrdir {
    #[doc = "HREQ pin is input (for LPI2C controller)."]
    Input = 0x0,
    #[doc = "HREQ pin is output (for LPI2C target)."]
    Output = 0x01,
}
impl Hrdir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hrdir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hrdir {
    #[inline(always)]
    fn from(val: u8) -> Hrdir {
        Hrdir::from_bits(val)
    }
}
impl From<Hrdir> for u8 {
    #[inline(always)]
    fn from(val: Hrdir) -> u8 {
        Hrdir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hrpol {
    #[doc = "Active low."]
    ActiveLow = 0x0,
    #[doc = "Active high."]
    ActiveHigh = 0x01,
}
impl Hrpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hrpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hrpol {
    #[inline(always)]
    fn from(val: u8) -> Hrpol {
        Hrpol::from_bits(val)
    }
}
impl From<Hrpol> for u8 {
    #[inline(always)]
    fn from(val: Hrpol) -> u8 {
        Hrpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Matcfg {
    #[doc = "Match is disabled."]
    Disabled = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Match is enabled: first data word equals MDMR\\[MATCH0\\] OR MDMR\\[MATCH1\\]."]
    FirstDataWordEqualsMatch0OrMatch1 = 0x02,
    #[doc = "Match is enabled: any data word equals MDMR\\[MATCH0\\] OR MDMR\\[MATCH1\\]."]
    AnyDataWordEqualsMatch0OrMatch1 = 0x03,
    #[doc = "Match is enabled: (first data word equals MDMR\\[MATCH0\\]) AND (second data word equals MDMR\\[MATCH1)."]
    FirstDataWordMatch0AndSecondDataWordMatch1 = 0x04,
    #[doc = "Match is enabled: (any data word equals MDMR\\[MATCH0\\]) AND (next data word equals MDMR\\[MATCH1)."]
    AnyDataWordMatch0NextDataWordMatch1 = 0x05,
    #[doc = "Match is enabled: (first data word AND MDMR\\[MATCH1\\]) equals (MDMR\\[MATCH0\\] AND MDMR\\[MATCH1\\])."]
    FirstDataWordAndMatch1EqualsMatch0AndMatch1 = 0x06,
    #[doc = "Match is enabled: (any data word AND MDMR\\[MATCH1\\]) equals (MDMR\\[MATCH0\\] AND MDMR\\[MATCH1\\])."]
    AnyDataWordAndMatch1EqualsMatch0AndMatch1 = 0x07,
}
impl Matcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Matcfg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Matcfg {
    #[inline(always)]
    fn from(val: u8) -> Matcfg {
        Matcfg::from_bits(val)
    }
}
impl From<Matcfg> for u8 {
    #[inline(always)]
    fn from(val: Matcfg) -> u8 {
        Matcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbf {
    #[doc = "Idle."]
    Idle = 0x0,
    #[doc = "Busy."]
    Busy = 0x01,
}
impl Mbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbf {
    #[inline(always)]
    fn from(val: u8) -> Mbf {
        Mbf::from_bits(val)
    }
}
impl From<Mbf> for u8 {
    #[inline(always)]
    fn from(val: Mbf) -> u8 {
        Mbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum McrRrf {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Reset receive FIFO."]
    Reset = 0x01,
}
impl McrRrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> McrRrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for McrRrf {
    #[inline(always)]
    fn from(val: u8) -> McrRrf {
        McrRrf::from_bits(val)
    }
}
impl From<McrRrf> for u8 {
    #[inline(always)]
    fn from(val: McrRrf) -> u8 {
        McrRrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum McrRtf {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Reset transmit FIFO."]
    Reset = 0x01,
}
impl McrRtf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> McrRtf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for McrRtf {
    #[inline(always)]
    fn from(val: u8) -> McrRtf {
        McrRtf::from_bits(val)
    }
}
impl From<McrRtf> for u8 {
    #[inline(always)]
    fn from(val: McrRtf) -> u8 {
        McrRtf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MsrBbf {
    #[doc = "Idle."]
    Idle = 0x0,
    #[doc = "Busy."]
    Busy = 0x01,
}
impl MsrBbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MsrBbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MsrBbf {
    #[inline(always)]
    fn from(val: u8) -> MsrBbf {
        MsrBbf::from_bits(val)
    }
}
impl From<MsrBbf> for u8 {
    #[inline(always)]
    fn from(val: MsrBbf) -> u8 {
        MsrBbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MsrFef {
    #[doc = "No FIFO error."]
    IntNo = 0x0,
    #[doc = "FIFO error."]
    IntYes = 0x01,
}
impl MsrFef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MsrFef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MsrFef {
    #[inline(always)]
    fn from(val: u8) -> MsrFef {
        MsrFef::from_bits(val)
    }
}
impl From<MsrFef> for u8 {
    #[inline(always)]
    fn from(val: MsrFef) -> u8 {
        MsrFef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MsrSdf {
    #[doc = "No Stop condition generated."]
    IntNo = 0x0,
    #[doc = "Stop condition generated."]
    IntYes = 0x01,
}
impl MsrSdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MsrSdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MsrSdf {
    #[inline(always)]
    fn from(val: u8) -> MsrSdf {
        MsrSdf::from_bits(val)
    }
}
impl From<MsrSdf> for u8 {
    #[inline(always)]
    fn from(val: MsrSdf) -> u8 {
        MsrSdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ndf {
    #[doc = "No unexpected NACK detected."]
    IntNo = 0x0,
    #[doc = "Unexpected NACK detected."]
    IntYes = 0x01,
}
impl Ndf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ndf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ndf {
    #[inline(always)]
    fn from(val: u8) -> Ndf {
        Ndf::from_bits(val)
    }
}
impl From<Ndf> for u8 {
    #[inline(always)]
    fn from(val: Ndf) -> u8 {
        Ndf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pincfg {
    #[doc = "Two-pin open drain mode."]
    OpenDrain2Pin = 0x0,
    #[doc = "Two-pin output only mode (Ultra-Fast mode)."]
    Output2PinOnly = 0x01,
    #[doc = "Two-pin push-pull mode."]
    PushPull2Pin = 0x02,
    #[doc = "Four-pin push-pull mode."]
    PushPull4Pin = 0x03,
    #[doc = "Two-pin open-drain mode with separate LPI2C target."]
    OpenDrain2PinWLpi2cSlave = 0x04,
    #[doc = "Two-pin output only mode (Ultra-Fast mode) with separate LPI2C target."]
    Output2PinOnlyWLpi2cSlave = 0x05,
    #[doc = "Two-pin push-pull mode with separate LPI2C target."]
    PushPull2PinWLpi2cSlave = 0x06,
    #[doc = "Four-pin push-pull mode (inverted outputs)."]
    PushPull4PinWLpi2cSlave = 0x07,
}
impl Pincfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pincfg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pincfg {
    #[inline(always)]
    fn from(val: u8) -> Pincfg {
        Pincfg::from_bits(val)
    }
}
impl From<Pincfg> for u8 {
    #[inline(always)]
    fn from(val: Pincfg) -> u8 {
        Pincfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pltf {
    #[doc = "Pin low timeout did not occur."]
    IntNo = 0x0,
    #[doc = "Pin low timeout occurred."]
    IntYes = 0x01,
}
impl Pltf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pltf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pltf {
    #[inline(always)]
    fn from(val: u8) -> Pltf {
        Pltf::from_bits(val)
    }
}
impl From<Pltf> for u8 {
    #[inline(always)]
    fn from(val: Pltf) -> u8 {
        Pltf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prescale {
    #[doc = "Divide by 1."]
    DivideBy1 = 0x0,
    #[doc = "Divide by 2."]
    DivideBy2 = 0x01,
    #[doc = "Divide by 4."]
    DivideBy4 = 0x02,
    #[doc = "Divide by 8."]
    DivideBy8 = 0x03,
    #[doc = "Divide by 16."]
    DivideBy16 = 0x04,
    #[doc = "Divide by 32."]
    DivideBy32 = 0x05,
    #[doc = "Divide by 64."]
    DivideBy64 = 0x06,
    #[doc = "Divide by 128."]
    DivideBy128 = 0x07,
}
impl Prescale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prescale {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prescale {
    #[inline(always)]
    fn from(val: u8) -> Prescale {
        Prescale::from_bits(val)
    }
}
impl From<Prescale> for u8 {
    #[inline(always)]
    fn from(val: Prescale) -> u8 {
        Prescale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Relax {
    #[doc = "Normal transfer."]
    NormalTransfer = 0x0,
    #[doc = "Relaxed transfer."]
    RelaxedTransfer = 0x01,
}
impl Relax {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Relax {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Relax {
    #[inline(always)]
    fn from(val: u8) -> Relax {
        Relax::from_bits(val)
    }
}
impl From<Relax> for u8 {
    #[inline(always)]
    fn from(val: Relax) -> u8 {
        Relax::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rscfg {
    #[doc = "Any repeated Start condition following an address match."]
    AnyRepeatedStartAfterAddressMatch = 0x0,
    #[doc = "Any repeated Start condition."]
    AnyRepeatedStart = 0x01,
}
impl Rscfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rscfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rscfg {
    #[inline(always)]
    fn from(val: u8) -> Rscfg {
        Rscfg::from_bits(val)
    }
}
impl From<Rscfg> for u8 {
    #[inline(always)]
    fn from(val: Rscfg) -> u8 {
        Rscfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxcfg {
    #[doc = "Return received data, clear SSR\\[RDF\\]."]
    ReturnsReceivedDataAndClearsRxDataFlag = 0x0,
    #[doc = "Return SASR and clear SSR\\[AVF\\] when SSR\\[AVF\\] is set, return received data and clear SSR\\[RDF\\] when SSR\\[AFV\\] is not set."]
    WhenAddressValidFlagSetReturnsAddressStatusAndClearsAddressValidFlag = 0x01,
}
impl Rxcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxcfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxcfg {
    #[inline(always)]
    fn from(val: u8) -> Rxcfg {
        Rxcfg::from_bits(val)
    }
}
impl From<Rxcfg> for u8 {
    #[inline(always)]
    fn from(val: Rxcfg) -> u8 {
        Rxcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxnack {
    #[doc = "ACK or NACK always determined by STAR\\[TXNACK\\]."]
    SetByTxnack = 0x0,
    #[doc = "NACK always generated on address overrun or receive data overrun, otherwise ACK or NACK is determined by STAR\\[TXNACK\\]."]
    AlwaysGeneratedOnAddressOrReceiveDataOverrun = 0x01,
}
impl Rxnack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxnack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxnack {
    #[inline(always)]
    fn from(val: u8) -> Rxnack {
        Rxnack::from_bits(val)
    }
}
impl From<Rxnack> for u8 {
    #[inline(always)]
    fn from(val: Rxnack) -> u8 {
        Rxnack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbf {
    #[doc = "Idle."]
    Idle = 0x0,
    #[doc = "Busy."]
    Busy = 0x01,
}
impl Sbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbf {
    #[inline(always)]
    fn from(val: u8) -> Sbf {
        Sbf::from_bits(val)
    }
}
impl From<Sbf> for u8 {
    #[inline(always)]
    fn from(val: Sbf) -> u8 {
        Sbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scfgr1Ignack {
    #[doc = "End transfer on NACK."]
    EndsTransferOnNack = 0x0,
    #[doc = "Do not end transfer on NACK."]
    DoesNotEndTransferOnNack = 0x01,
}
impl Scfgr1Ignack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scfgr1Ignack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scfgr1Ignack {
    #[inline(always)]
    fn from(val: u8) -> Scfgr1Ignack {
        Scfgr1Ignack::from_bits(val)
    }
}
impl From<Scfgr1Ignack> for u8 {
    #[inline(always)]
    fn from(val: Scfgr1Ignack) -> u8 {
        Scfgr1Ignack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrRrf {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "SRDR is now empty."]
    NowEmpty = 0x01,
}
impl ScrRrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrRrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrRrf {
    #[inline(always)]
    fn from(val: u8) -> ScrRrf {
        ScrRrf::from_bits(val)
    }
}
impl From<ScrRrf> for u8 {
    #[inline(always)]
    fn from(val: ScrRrf) -> u8 {
        ScrRrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrRtf {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "STDR is now empty."]
    NowEmpty = 0x01,
}
impl ScrRtf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrRtf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrRtf {
    #[inline(always)]
    fn from(val: u8) -> ScrRtf {
        ScrRtf::from_bits(val)
    }
}
impl From<ScrRtf> for u8 {
    #[inline(always)]
    fn from(val: ScrRtf) -> u8 {
        ScrRtf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdcfg {
    #[doc = "Any Stop condition following an address match."]
    AnyStopAfterAddressMatch = 0x0,
    #[doc = "Any Stop condition."]
    AnyStop = 0x01,
}
impl Sdcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdcfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdcfg {
    #[inline(always)]
    fn from(val: u8) -> Sdcfg {
        Sdcfg::from_bits(val)
    }
}
impl From<Sdcfg> for u8 {
    #[inline(always)]
    fn from(val: Sdcfg) -> u8 {
        Sdcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrBbf {
    #[doc = "Idle."]
    Idle = 0x0,
    #[doc = "Busy."]
    Busy = 0x01,
}
impl SsrBbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrBbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrBbf {
    #[inline(always)]
    fn from(val: u8) -> SsrBbf {
        SsrBbf::from_bits(val)
    }
}
impl From<SsrBbf> for u8 {
    #[inline(always)]
    fn from(val: SsrBbf) -> u8 {
        SsrBbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Startcfg {
    #[doc = "Sets when both I2C bus and LPI2C controller are idle."]
    BothI2cAndLpi2cIdle = 0x0,
    #[doc = "Sets when I2C bus is idle."]
    I2cIdle = 0x01,
}
impl Startcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Startcfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Startcfg {
    #[inline(always)]
    fn from(val: u8) -> Startcfg {
        Startcfg::from_bits(val)
    }
}
impl From<Startcfg> for u8 {
    #[inline(always)]
    fn from(val: Startcfg) -> u8 {
        Startcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stf {
    #[doc = "Start condition not detected."]
    IntNo = 0x0,
    #[doc = "Start condition detected."]
    IntYes = 0x01,
}
impl Stf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stf {
    #[inline(always)]
    fn from(val: u8) -> Stf {
        Stf::from_bits(val)
    }
}
impl From<Stf> for u8 {
    #[inline(always)]
    fn from(val: Stf) -> u8 {
        Stf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stopcfg {
    #[doc = "Any Stop condition."]
    AnyStop = 0x0,
    #[doc = "Last Stop condition."]
    LastStop = 0x01,
}
impl Stopcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stopcfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stopcfg {
    #[inline(always)]
    fn from(val: u8) -> Stopcfg {
        Stopcfg::from_bits(val)
    }
}
impl From<Stopcfg> for u8 {
    #[inline(always)]
    fn from(val: Stopcfg) -> u8 {
        Stopcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timecfg {
    #[doc = "SCL."]
    IfSclLow = 0x0,
    #[doc = "SCL or SDA."]
    IfSclOrSdaLow = 0x01,
}
impl Timecfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timecfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timecfg {
    #[inline(always)]
    fn from(val: u8) -> Timecfg {
        Timecfg::from_bits(val)
    }
}
impl From<Timecfg> for u8 {
    #[inline(always)]
    fn from(val: Timecfg) -> u8 {
        Timecfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txcfg {
    #[doc = "SSR\\[TDF\\] is set only during a target-transmit transfer when STDR is empty."]
    AssertsDuringSlaveTransmitTransferWhenTxDataEmpty = 0x0,
    #[doc = "SSR\\[TDF\\] is set whenever STDR is empty."]
    AssertsWhenTxDataEmpty = 0x01,
}
impl Txcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txcfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txcfg {
    #[inline(always)]
    fn from(val: u8) -> Txcfg {
        Txcfg::from_bits(val)
    }
}
impl From<Txcfg> for u8 {
    #[inline(always)]
    fn from(val: Txcfg) -> u8 {
        Txcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txnack {
    #[doc = "Transmit ACK."]
    TransmitAck = 0x0,
    #[doc = "Transmit NACK."]
    TransmitNack = 0x01,
}
impl Txnack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txnack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txnack {
    #[inline(always)]
    fn from(val: u8) -> Txnack {
        Txnack::from_bits(val)
    }
}
impl From<Txnack> for u8 {
    #[inline(always)]
    fn from(val: Txnack) -> u8 {
        Txnack::to_bits(val)
    }
}
