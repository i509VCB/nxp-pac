#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "VBAT."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbat {
    ptr: *mut u8,
}
unsafe impl Send for Vbat {}
unsafe impl Sync for Vbat {}
impl Vbat {
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
    #[doc = "Status A."]
    #[inline(always)]
    pub const fn statusa(self) -> crate::pac::common::Reg<Statusa, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status B."]
    #[inline(always)]
    pub const fn statusb(self) -> crate::pac::common::Reg<Statusb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt Enable A."]
    #[inline(always)]
    pub const fn irqena(self) -> crate::pac::common::Reg<Irqena, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Interrupt Enable B."]
    #[inline(always)]
    pub const fn irqenb(self) -> crate::pac::common::Reg<Irqenb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Wake-up Enable A."]
    #[inline(always)]
    pub const fn wakena(self) -> crate::pac::common::Reg<Wakena, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Wake-up Enable B."]
    #[inline(always)]
    pub const fn wakenb(self) -> crate::pac::common::Reg<Wakenb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Tamper Enable A."]
    #[inline(always)]
    pub const fn tampera(self) -> crate::pac::common::Reg<Tampera, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Tamper Enable B."]
    #[inline(always)]
    pub const fn tamperb(self) -> crate::pac::common::Reg<Tamperb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Lock A."]
    #[inline(always)]
    pub const fn locka(self) -> crate::pac::common::Reg<Locka, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Lock B."]
    #[inline(always)]
    pub const fn lockb(self) -> crate::pac::common::Reg<Lockb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Wake-up Configuration."]
    #[inline(always)]
    pub const fn wakecfg(self) -> crate::pac::common::Reg<Wakecfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Oscillator Control A."]
    #[inline(always)]
    pub const fn oscctla(self) -> crate::pac::common::Reg<Oscctla, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Oscillator Control B."]
    #[inline(always)]
    pub const fn oscctlb(self) -> crate::pac::common::Reg<Oscctlb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Oscillator Configuration A."]
    #[inline(always)]
    pub const fn osccfga(self) -> crate::pac::common::Reg<Osccfga, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Oscillator Configuration B."]
    #[inline(always)]
    pub const fn osccfgb(self) -> crate::pac::common::Reg<Osccfgb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Oscillator Lock A."]
    #[inline(always)]
    pub const fn osclcka(self) -> crate::pac::common::Reg<Osclcka, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Oscillator Lock B."]
    #[inline(always)]
    pub const fn osclckb(self) -> crate::pac::common::Reg<Osclckb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Oscillator Clock Enable."]
    #[inline(always)]
    pub const fn oscclke(self) -> crate::pac::common::Reg<Oscclke, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "FRO16K Control A."]
    #[inline(always)]
    pub const fn froctla(self) -> crate::pac::common::Reg<Froctla, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "FRO16K Control B."]
    #[inline(always)]
    pub const fn froctlb(self) -> crate::pac::common::Reg<Froctlb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "FRO16K Lock A."]
    #[inline(always)]
    pub const fn frolcka(self) -> crate::pac::common::Reg<Frolcka, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "FRO16K Lock B."]
    #[inline(always)]
    pub const fn frolckb(self) -> crate::pac::common::Reg<Frolckb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "FRO16K Clock Enable."]
    #[inline(always)]
    pub const fn froclke(self) -> crate::pac::common::Reg<Froclke, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "LDO_RAM Control A."]
    #[inline(always)]
    pub const fn ldoctla(self) -> crate::pac::common::Reg<Ldoctla, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "LDO_RAM Control B."]
    #[inline(always)]
    pub const fn ldoctlb(self) -> crate::pac::common::Reg<Ldoctlb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "LDO_RAM Lock A."]
    #[inline(always)]
    pub const fn ldolcka(self) -> crate::pac::common::Reg<Ldolcka, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "LDO_RAM Lock B."]
    #[inline(always)]
    pub const fn ldolckb(self) -> crate::pac::common::Reg<Ldolckb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x031cusize) as _) }
    }
    #[doc = "RAM Control."]
    #[inline(always)]
    pub const fn ldoramc(self) -> crate::pac::common::Reg<Ldoramc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "Bandgap Timer 0."]
    #[inline(always)]
    pub const fn ldotimer0(self) -> crate::pac::common::Reg<Ldotimer0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "Bandgap Timer 1."]
    #[inline(always)]
    pub const fn ldotimer1(self) -> crate::pac::common::Reg<Ldotimer1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0338usize) as _) }
    }
    #[doc = "CLKMON Control A."]
    #[inline(always)]
    pub const fn monctla(self) -> crate::pac::common::Reg<Monctla, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "CLKMON Control B."]
    #[inline(always)]
    pub const fn monctlb(self) -> crate::pac::common::Reg<Monctlb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "CLKMON Configuration A."]
    #[inline(always)]
    pub const fn moncfga(self) -> crate::pac::common::Reg<Moncfga, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "CLKMON Configuration B."]
    #[inline(always)]
    pub const fn moncfgb(self) -> crate::pac::common::Reg<Moncfgb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "CLKMON Lock A."]
    #[inline(always)]
    pub const fn monlcka(self) -> crate::pac::common::Reg<Monlcka, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "CLKMON Lock B."]
    #[inline(always)]
    pub const fn monlckb(self) -> crate::pac::common::Reg<Monlckb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "TAMPER Control A."]
    #[inline(always)]
    pub const fn tamctla(self) -> crate::pac::common::Reg<Tamctla, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "TAMPER Control B."]
    #[inline(always)]
    pub const fn tamctlb(self) -> crate::pac::common::Reg<Tamctlb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "TAMPER Lock A."]
    #[inline(always)]
    pub const fn tamlcka(self) -> crate::pac::common::Reg<Tamlcka, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
    }
    #[doc = "TAMPER Lock B."]
    #[inline(always)]
    pub const fn tamlckb(self) -> crate::pac::common::Reg<Tamlckb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize) as _) }
    }
    #[doc = "Switch Control A."]
    #[inline(always)]
    pub const fn swictla(self) -> crate::pac::common::Reg<Swictla, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "Switch Control B."]
    #[inline(always)]
    pub const fn swictlb(self) -> crate::pac::common::Reg<Swictlb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "Switch Lock A."]
    #[inline(always)]
    pub const fn swilcka(self) -> crate::pac::common::Reg<Swilcka, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[doc = "Switch Lock B."]
    #[inline(always)]
    pub const fn swilckb(self) -> crate::pac::common::Reg<Swilckb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x061cusize) as _) }
    }
    #[doc = "Array of registers: WAKEUPA, WAKEUPB."]
    #[inline(always)]
    pub const fn wakeup(self, n: usize) -> Wakeup {
        assert!(n < 2usize);
        unsafe { Wakeup::from_ptr(self.ptr.wrapping_add(0x0700usize + n * 8usize) as _) }
    }
    #[doc = "Wakeup Lock A."]
    #[inline(always)]
    pub const fn waklcka(self) -> crate::pac::common::Reg<Waklcka, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f8usize) as _) }
    }
    #[doc = "Wakeup Lock B."]
    #[inline(always)]
    pub const fn waklckb(self) -> crate::pac::common::Reg<Waklckb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x07fcusize) as _) }
    }
}
#[doc = "Array of registers: WAKEUPA, WAKEUPB."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeup {
    ptr: *mut u8,
}
unsafe impl Send for Wakeup {}
unsafe impl Sync for Wakeup {}
impl Wakeup {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Wakeup 0 Register A."]
    #[inline(always)]
    pub const fn wakeupa(self) -> crate::pac::common::Reg<Wakeupa, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Wakeup 0 Register B."]
    #[inline(always)]
    pub const fn wakeupb(self) -> crate::pac::common::Reg<Wakeupb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
#[doc = "FRO16K Clock Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Froclke(pub u32);
impl Froclke {
    #[doc = "Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn clke(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock Enable."]
    #[inline(always)]
    pub const fn set_clke(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Froclke {
    #[inline(always)]
    fn default() -> Froclke {
        Froclke(0)
    }
}
impl core::fmt::Debug for Froclke {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Froclke")
            .field("clke", &self.clke())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Froclke {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Froclke {{ clke: {=u8:?} }}", self.clke())
    }
}
#[doc = "FRO16K Control A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Froctla(pub u32);
impl Froctla {
    #[doc = "FRO16K Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fro_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FRO16K Enable."]
    #[inline(always)]
    pub const fn set_fro_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Froctla {
    #[inline(always)]
    fn default() -> Froctla {
        Froctla(0)
    }
}
impl core::fmt::Debug for Froctla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Froctla")
            .field("fro_en", &self.fro_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Froctla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Froctla {{ fro_en: {=bool:?} }}", self.fro_en())
    }
}
#[doc = "FRO16K Control B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Froctlb(pub u32);
impl Froctlb {
    #[doc = "Inverse Value."]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Inverse Value."]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Froctlb {
    #[inline(always)]
    fn default() -> Froctlb {
        Froctlb(0)
    }
}
impl core::fmt::Debug for Froctlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Froctlb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Froctlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Froctlb {{ inverse: {=bool:?} }}", self.inverse())
    }
}
#[doc = "FRO16K Lock A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frolcka(pub u32);
impl Frolcka {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Frolcka {
    #[inline(always)]
    fn default() -> Frolcka {
        Frolcka(0)
    }
}
impl core::fmt::Debug for Frolcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frolcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frolcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frolcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "FRO16K Lock B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frolckb(pub u32);
impl Frolckb {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> FrolckbLock {
        let val = (self.0 >> 0usize) & 0x01;
        FrolckbLock::from_bits(val as u8)
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: FrolckbLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Frolckb {
    #[inline(always)]
    fn default() -> Frolckb {
        Frolckb(0)
    }
}
impl core::fmt::Debug for Frolckb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frolckb")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frolckb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frolckb {{ lock: {:?} }}", self.lock())
    }
}
#[doc = "Interrupt Enable A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqena(pub u32);
impl Irqena {
    #[doc = "POR Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn por_det(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "POR Detect."]
    #[inline(always)]
    pub const fn set_por_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Wakeup Pin Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_flag(&self) -> IrqenaWakeupFlag {
        let val = (self.0 >> 1usize) & 0x01;
        IrqenaWakeupFlag::from_bits(val as u8)
    }
    #[doc = "Wakeup Pin Flag."]
    #[inline(always)]
    pub const fn set_wakeup_flag(&mut self, val: IrqenaWakeupFlag) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bandgap Timer 0."]
    #[must_use]
    #[inline(always)]
    pub const fn timer0_flag(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timer 0."]
    #[inline(always)]
    pub const fn set_timer0_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Bandgap Timer 2."]
    #[must_use]
    #[inline(always)]
    pub const fn timer1_flag(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timer 2."]
    #[inline(always)]
    pub const fn set_timer1_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "LDO Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_rdy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Ready."]
    #[inline(always)]
    pub const fn set_ldo_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OSC32k Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_rdy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OSC32k Ready."]
    #[inline(always)]
    pub const fn set_osc_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Clock Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn clock_det(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Detect."]
    #[inline(always)]
    pub const fn set_clock_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Configuration Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn config_det(&self) -> IrqenaConfigDet {
        let val = (self.0 >> 7usize) & 0x01;
        IrqenaConfigDet::from_bits(val as u8)
    }
    #[doc = "Configuration Detect."]
    #[inline(always)]
    pub const fn set_config_det(&mut self, val: IrqenaConfigDet) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Voltage Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn volt_det(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Detect."]
    #[inline(always)]
    pub const fn set_volt_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Temperature Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn temp_det(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Detect."]
    #[inline(always)]
    pub const fn set_temp_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Light Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn light_det(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Light Detect."]
    #[inline(always)]
    pub const fn set_light_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Input 0 Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn sec0_det(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Input 0 Detect."]
    #[inline(always)]
    pub const fn set_sec0_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn irq_det(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Detect."]
    #[inline(always)]
    pub const fn set_irq_det(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Irqena {
    #[inline(always)]
    fn default() -> Irqena {
        Irqena(0)
    }
}
impl core::fmt::Debug for Irqena {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irqena")
            .field("por_det", &self.por_det())
            .field("wakeup_flag", &self.wakeup_flag())
            .field("timer0_flag", &self.timer0_flag())
            .field("timer1_flag", &self.timer1_flag())
            .field("ldo_rdy", &self.ldo_rdy())
            .field("osc_rdy", &self.osc_rdy())
            .field("clock_det", &self.clock_det())
            .field("config_det", &self.config_det())
            .field("volt_det", &self.volt_det())
            .field("temp_det", &self.temp_det())
            .field("light_det", &self.light_det())
            .field("sec0_det", &self.sec0_det())
            .field("irq_det[0]", &self.irq_det(0usize))
            .field("irq_det[1]", &self.irq_det(1usize))
            .field("irq_det[2]", &self.irq_det(2usize))
            .field("irq_det[3]", &self.irq_det(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irqena {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Irqena {{ por_det: {=bool:?}, wakeup_flag: {:?}, timer0_flag: {=bool:?}, timer1_flag: {=bool:?}, ldo_rdy: {=bool:?}, osc_rdy: {=bool:?}, clock_det: {=bool:?}, config_det: {:?}, volt_det: {=bool:?}, temp_det: {=bool:?}, light_det: {=bool:?}, sec0_det: {=bool:?}, irq_det[0]: {=bool:?}, irq_det[1]: {=bool:?}, irq_det[2]: {=bool:?}, irq_det[3]: {=bool:?} }}",
            self.por_det(),
            self.wakeup_flag(),
            self.timer0_flag(),
            self.timer1_flag(),
            self.ldo_rdy(),
            self.osc_rdy(),
            self.clock_det(),
            self.config_det(),
            self.volt_det(),
            self.temp_det(),
            self.light_det(),
            self.sec0_det(),
            self.irq_det(0usize),
            self.irq_det(1usize),
            self.irq_det(2usize),
            self.irq_det(3usize)
        )
    }
}
#[doc = "Interrupt Enable B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqenb(pub u32);
impl Irqenb {
    #[doc = "Inverse Value."]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Inverse Value."]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Irqenb {
    #[inline(always)]
    fn default() -> Irqenb {
        Irqenb(0)
    }
}
impl core::fmt::Debug for Irqenb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irqenb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irqenb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Irqenb {{ inverse: {=u32:?} }}", self.inverse())
    }
}
#[doc = "LDO_RAM Control A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldoctla(pub u32);
impl Ldoctla {
    #[doc = "Bandgap Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bg_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Enable."]
    #[inline(always)]
    pub const fn set_bg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LDO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Enable."]
    #[inline(always)]
    pub const fn set_ldo_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Refresh Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn refresh_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Refresh Enable."]
    #[inline(always)]
    pub const fn set_refresh_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Ldoctla {
    #[inline(always)]
    fn default() -> Ldoctla {
        Ldoctla(0)
    }
}
impl core::fmt::Debug for Ldoctla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldoctla")
            .field("bg_en", &self.bg_en())
            .field("ldo_en", &self.ldo_en())
            .field("refresh_en", &self.refresh_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldoctla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldoctla {{ bg_en: {=bool:?}, ldo_en: {=bool:?}, refresh_en: {=bool:?} }}",
            self.bg_en(),
            self.ldo_en(),
            self.refresh_en()
        )
    }
}
#[doc = "LDO_RAM Control B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldoctlb(pub u32);
impl Ldoctlb {
    #[doc = "Inverse Value."]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Inverse Value."]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Ldoctlb {
    #[inline(always)]
    fn default() -> Ldoctlb {
        Ldoctlb(0)
    }
}
impl core::fmt::Debug for Ldoctlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldoctlb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldoctlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ldoctlb {{ inverse: {=u8:?} }}", self.inverse())
    }
}
#[doc = "LDO_RAM Lock A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldolcka(pub u32);
impl Ldolcka {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ldolcka {
    #[inline(always)]
    fn default() -> Ldolcka {
        Ldolcka(0)
    }
}
impl core::fmt::Debug for Ldolcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldolcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldolcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ldolcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "LDO_RAM Lock B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldolckb(pub u32);
impl Ldolckb {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> LdolckbLock {
        let val = (self.0 >> 0usize) & 0x01;
        LdolckbLock::from_bits(val as u8)
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: LdolckbLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Ldolckb {
    #[inline(always)]
    fn default() -> Ldolckb {
        Ldolckb(0)
    }
}
impl core::fmt::Debug for Ldolckb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldolckb")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldolckb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ldolckb {{ lock: {:?} }}", self.lock())
    }
}
#[doc = "RAM Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldoramc(pub u32);
impl Ldoramc {
    #[doc = "Isolate SRAM."]
    #[must_use]
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Isolate SRAM."]
    #[inline(always)]
    pub const fn set_iso(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Switch SRAM."]
    #[must_use]
    #[inline(always)]
    pub const fn swi(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Switch SRAM."]
    #[inline(always)]
    pub const fn set_swi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Retention."]
    #[must_use]
    #[inline(always)]
    pub const fn ret(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 8usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Retention."]
    #[inline(always)]
    pub const fn set_ret(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 8usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Ldoramc {
    #[inline(always)]
    fn default() -> Ldoramc {
        Ldoramc(0)
    }
}
impl core::fmt::Debug for Ldoramc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldoramc")
            .field("iso", &self.iso())
            .field("swi", &self.swi())
            .field("ret[0]", &self.ret(0usize))
            .field("ret[1]", &self.ret(1usize))
            .field("ret[2]", &self.ret(2usize))
            .field("ret[3]", &self.ret(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldoramc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldoramc {{ iso: {=bool:?}, swi: {=bool:?}, ret[0]: {=bool:?}, ret[1]: {=bool:?}, ret[2]: {=bool:?}, ret[3]: {=bool:?} }}",
            self.iso(),
            self.swi(),
            self.ret(0usize),
            self.ret(1usize),
            self.ret(2usize),
            self.ret(3usize)
        )
    }
}
#[doc = "Bandgap Timer 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldotimer0(pub u32);
impl Ldotimer0 {
    #[doc = "Timeout Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn timcfg(&self) -> Timcfg {
        let val = (self.0 >> 0usize) & 0x07;
        Timcfg::from_bits(val as u8)
    }
    #[doc = "Timeout Configuration."]
    #[inline(always)]
    pub const fn set_timcfg(&mut self, val: Timcfg) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Bandgap Timeout Period Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn timen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timeout Period Enable."]
    #[inline(always)]
    pub const fn set_timen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ldotimer0 {
    #[inline(always)]
    fn default() -> Ldotimer0 {
        Ldotimer0(0)
    }
}
impl core::fmt::Debug for Ldotimer0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldotimer0")
            .field("timcfg", &self.timcfg())
            .field("timen", &self.timen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldotimer0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldotimer0 {{ timcfg: {:?}, timen: {=bool:?} }}",
            self.timcfg(),
            self.timen()
        )
    }
}
#[doc = "Bandgap Timer 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldotimer1(pub u32);
impl Ldotimer1 {
    #[doc = "Timeout Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn timcfg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Timeout Configuration."]
    #[inline(always)]
    pub const fn set_timcfg(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Bandgap Timeout Period Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn timen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timeout Period Enable."]
    #[inline(always)]
    pub const fn set_timen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ldotimer1 {
    #[inline(always)]
    fn default() -> Ldotimer1 {
        Ldotimer1(0)
    }
}
impl core::fmt::Debug for Ldotimer1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldotimer1")
            .field("timcfg", &self.timcfg())
            .field("timen", &self.timen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldotimer1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldotimer1 {{ timcfg: {=u32:?}, timen: {=bool:?} }}",
            self.timcfg(),
            self.timen()
        )
    }
}
#[doc = "Lock A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Locka(pub u32);
impl Locka {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Locka {
    #[inline(always)]
    fn default() -> Locka {
        Locka(0)
    }
}
impl core::fmt::Debug for Locka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Locka").field("lock", &self.lock()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Locka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Locka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "Lock B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lockb(pub u32);
impl Lockb {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> LockbLock {
        let val = (self.0 >> 0usize) & 0x01;
        LockbLock::from_bits(val as u8)
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: LockbLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lockb {
    #[inline(always)]
    fn default() -> Lockb {
        Lockb(0)
    }
}
impl core::fmt::Debug for Lockb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lockb").field("lock", &self.lock()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lockb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lockb {{ lock: {:?} }}", self.lock())
    }
}
#[doc = "CLKMON Configuration A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Moncfga(pub u32);
impl Moncfga {
    #[doc = "Frequency Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn freq_trim(&self) -> FreqTrim {
        let val = (self.0 >> 0usize) & 0x03;
        FreqTrim::from_bits(val as u8)
    }
    #[doc = "Frequency Trim."]
    #[inline(always)]
    pub const fn set_freq_trim(&mut self, val: FreqTrim) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Divide Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn divide_trim(&self) -> DivideTrim {
        let val = (self.0 >> 2usize) & 0x01;
        DivideTrim::from_bits(val as u8)
    }
    #[doc = "Divide Trim."]
    #[inline(always)]
    pub const fn set_divide_trim(&mut self, val: DivideTrim) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Reserved Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd_trim(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Reserved Trim."]
    #[inline(always)]
    pub const fn set_rsvd_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
    }
}
impl Default for Moncfga {
    #[inline(always)]
    fn default() -> Moncfga {
        Moncfga(0)
    }
}
impl core::fmt::Debug for Moncfga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Moncfga")
            .field("freq_trim", &self.freq_trim())
            .field("divide_trim", &self.divide_trim())
            .field("rsvd_trim", &self.rsvd_trim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Moncfga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Moncfga {{ freq_trim: {:?}, divide_trim: {:?}, rsvd_trim: {=u8:?} }}",
            self.freq_trim(),
            self.divide_trim(),
            self.rsvd_trim()
        )
    }
}
#[doc = "CLKMON Configuration B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Moncfgb(pub u32);
impl Moncfgb {
    #[doc = "Inverse value."]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Inverse value."]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Moncfgb {
    #[inline(always)]
    fn default() -> Moncfgb {
        Moncfgb(0)
    }
}
impl core::fmt::Debug for Moncfgb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Moncfgb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Moncfgb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Moncfgb {{ inverse: {=u8:?} }}", self.inverse())
    }
}
#[doc = "CLKMON Control A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monctla(pub u32);
impl Monctla {
    #[doc = "CLKMON Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mon_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CLKMON Enable."]
    #[inline(always)]
    pub const fn set_mon_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Monctla {
    #[inline(always)]
    fn default() -> Monctla {
        Monctla(0)
    }
}
impl core::fmt::Debug for Monctla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Monctla")
            .field("mon_en", &self.mon_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Monctla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Monctla {{ mon_en: {=bool:?} }}", self.mon_en())
    }
}
#[doc = "CLKMON Control B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monctlb(pub u32);
impl Monctlb {
    #[doc = "Inverse value."]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Inverse value."]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Monctlb {
    #[inline(always)]
    fn default() -> Monctlb {
        Monctlb(0)
    }
}
impl core::fmt::Debug for Monctlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Monctlb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Monctlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Monctlb {{ inverse: {=bool:?} }}", self.inverse())
    }
}
#[doc = "CLKMON Lock A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monlcka(pub u32);
impl Monlcka {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Monlcka {
    #[inline(always)]
    fn default() -> Monlcka {
        Monlcka(0)
    }
}
impl core::fmt::Debug for Monlcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Monlcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Monlcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Monlcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "CLKMON Lock B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monlckb(pub u32);
impl Monlckb {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> MonlckbLock {
        let val = (self.0 >> 0usize) & 0x01;
        MonlckbLock::from_bits(val as u8)
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: MonlckbLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Monlckb {
    #[inline(always)]
    fn default() -> Monlckb {
        Monlckb(0)
    }
}
impl core::fmt::Debug for Monlckb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Monlckb")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Monlckb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Monlckb {{ lock: {:?} }}", self.lock())
    }
}
#[doc = "Oscillator Configuration A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osccfga(pub u32);
impl Osccfga {
    #[doc = "Comparator Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_trim(&self) -> CmpTrim {
        let val = (self.0 >> 0usize) & 0x03;
        CmpTrim::from_bits(val as u8)
    }
    #[doc = "Comparator Trim."]
    #[inline(always)]
    pub const fn set_cmp_trim(&mut self, val: CmpTrim) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CAP2_TRIM."]
    #[must_use]
    #[inline(always)]
    pub const fn cap2_trim(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CAP2_TRIM."]
    #[inline(always)]
    pub const fn set_cap2_trim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Delay Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn dly_trim(&self) -> DlyTrim {
        let val = (self.0 >> 3usize) & 0x0f;
        DlyTrim::from_bits(val as u8)
    }
    #[doc = "Delay Trim."]
    #[inline(always)]
    pub const fn set_dly_trim(&mut self, val: DlyTrim) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val.to_bits() as u32) & 0x0f) << 3usize);
    }
    #[doc = "Capacitor Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_trim(&self) -> CapTrim {
        let val = (self.0 >> 7usize) & 0x03;
        CapTrim::from_bits(val as u8)
    }
    #[doc = "Capacitor Trim."]
    #[inline(always)]
    pub const fn set_cap_trim(&mut self, val: CapTrim) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u32) & 0x03) << 7usize);
    }
    #[doc = "Initialization Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn init_trim(&self) -> InitTrim {
        let val = (self.0 >> 9usize) & 0x07;
        InitTrim::from_bits(val as u8)
    }
    #[doc = "Initialization Trim."]
    #[inline(always)]
    pub const fn set_init_trim(&mut self, val: InitTrim) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
    }
}
impl Default for Osccfga {
    #[inline(always)]
    fn default() -> Osccfga {
        Osccfga(0)
    }
}
impl core::fmt::Debug for Osccfga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osccfga")
            .field("cmp_trim", &self.cmp_trim())
            .field("cap2_trim", &self.cap2_trim())
            .field("dly_trim", &self.dly_trim())
            .field("cap_trim", &self.cap_trim())
            .field("init_trim", &self.init_trim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osccfga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Osccfga {{ cmp_trim: {:?}, cap2_trim: {=bool:?}, dly_trim: {:?}, cap_trim: {:?}, init_trim: {:?} }}",
            self.cmp_trim(),
            self.cap2_trim(),
            self.dly_trim(),
            self.cap_trim(),
            self.init_trim()
        )
    }
}
#[doc = "Oscillator Configuration B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osccfgb(pub u32);
impl Osccfgb {
    #[doc = "Inverse Value."]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Inverse Value."]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Osccfgb {
    #[inline(always)]
    fn default() -> Osccfgb {
        Osccfgb(0)
    }
}
impl core::fmt::Debug for Osccfgb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osccfgb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osccfgb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Osccfgb {{ inverse: {=u16:?} }}", self.inverse())
    }
}
#[doc = "Oscillator Clock Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oscclke(pub u32);
impl Oscclke {
    #[doc = "Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn clke(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock Enable."]
    #[inline(always)]
    pub const fn set_clke(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Oscclke {
    #[inline(always)]
    fn default() -> Oscclke {
        Oscclke(0)
    }
}
impl core::fmt::Debug for Oscclke {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oscclke")
            .field("clke", &self.clke())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oscclke {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Oscclke {{ clke: {=u8:?} }}", self.clke())
    }
}
#[doc = "Oscillator Control A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oscctla(pub u32);
impl Oscctla {
    #[doc = "Crystal Oscillator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Crystal Oscillator Enable."]
    #[inline(always)]
    pub const fn set_osc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Crystal Oscillator Bypass Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_byp_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Crystal Oscillator Bypass Enable."]
    #[inline(always)]
    pub const fn set_osc_byp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Amplifier gain adjustment bits to allow the use of a wide range of external crystal ESR values See the device datasheet for the ranges supported by this device."]
    #[must_use]
    #[inline(always)]
    pub const fn coarse_amp_gain(&self) -> CoarseAmpGain {
        let val = (self.0 >> 2usize) & 0x03;
        CoarseAmpGain::from_bits(val as u8)
    }
    #[doc = "Amplifier gain adjustment bits to allow the use of a wide range of external crystal ESR values See the device datasheet for the ranges supported by this device."]
    #[inline(always)]
    pub const fn set_coarse_amp_gain(&mut self, val: CoarseAmpGain) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Crystal Load Capacitance Selection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_sel_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Crystal Load Capacitance Selection Enable."]
    #[inline(always)]
    pub const fn set_cap_sel_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Crystal Load Capacitance Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn extal_cap_sel(&self) -> ExtalCapSel {
        let val = (self.0 >> 8usize) & 0x0f;
        ExtalCapSel::from_bits(val as u8)
    }
    #[doc = "Crystal Load Capacitance Selection."]
    #[inline(always)]
    pub const fn set_extal_cap_sel(&mut self, val: ExtalCapSel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Crystal Load Capacitance Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_cap_sel(&self) -> XtalCapSel {
        let val = (self.0 >> 12usize) & 0x0f;
        XtalCapSel::from_bits(val as u8)
    }
    #[doc = "Crystal Load Capacitance Selection."]
    #[inline(always)]
    pub const fn set_xtal_cap_sel(&mut self, val: XtalCapSel) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mode_en(&self) -> ModeEn {
        let val = (self.0 >> 16usize) & 0x03;
        ModeEn::from_bits(val as u8)
    }
    #[doc = "Mode Enable."]
    #[inline(always)]
    pub const fn set_mode_en(&mut self, val: ModeEn) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Supply Detector Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn supply_det(&self) -> SupplyDet {
        let val = (self.0 >> 18usize) & 0x03;
        SupplyDet::from_bits(val as u8)
    }
    #[doc = "Supply Detector Trim."]
    #[inline(always)]
    pub const fn set_supply_det(&mut self, val: SupplyDet) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
}
impl Default for Oscctla {
    #[inline(always)]
    fn default() -> Oscctla {
        Oscctla(0)
    }
}
impl core::fmt::Debug for Oscctla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oscctla")
            .field("osc_en", &self.osc_en())
            .field("osc_byp_en", &self.osc_byp_en())
            .field("coarse_amp_gain", &self.coarse_amp_gain())
            .field("cap_sel_en", &self.cap_sel_en())
            .field("extal_cap_sel", &self.extal_cap_sel())
            .field("xtal_cap_sel", &self.xtal_cap_sel())
            .field("mode_en", &self.mode_en())
            .field("supply_det", &self.supply_det())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oscctla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Oscctla {{ osc_en: {=bool:?}, osc_byp_en: {=bool:?}, coarse_amp_gain: {:?}, cap_sel_en: {=bool:?}, extal_cap_sel: {:?}, xtal_cap_sel: {:?}, mode_en: {:?}, supply_det: {:?} }}",
            self.osc_en(),
            self.osc_byp_en(),
            self.coarse_amp_gain(),
            self.cap_sel_en(),
            self.extal_cap_sel(),
            self.xtal_cap_sel(),
            self.mode_en(),
            self.supply_det()
        )
    }
}
#[doc = "Oscillator Control B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oscctlb(pub u32);
impl Oscctlb {
    #[doc = "Inverse Value."]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Inverse Value."]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Oscctlb {
    #[inline(always)]
    fn default() -> Oscctlb {
        Oscctlb(0)
    }
}
impl core::fmt::Debug for Oscctlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oscctlb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oscctlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Oscctlb {{ inverse: {=u32:?} }}", self.inverse())
    }
}
#[doc = "Oscillator Lock A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osclcka(pub u32);
impl Osclcka {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Osclcka {
    #[inline(always)]
    fn default() -> Osclcka {
        Osclcka(0)
    }
}
impl core::fmt::Debug for Osclcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osclcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osclcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Osclcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "Oscillator Lock B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osclckb(pub u32);
impl Osclckb {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> OsclckbLock {
        let val = (self.0 >> 0usize) & 0x01;
        OsclckbLock::from_bits(val as u8)
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: OsclckbLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Osclckb {
    #[inline(always)]
    fn default() -> Osclckb {
        Osclckb(0)
    }
}
impl core::fmt::Debug for Osclckb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osclckb")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osclckb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Osclckb {{ lock: {:?} }}", self.lock())
    }
}
#[doc = "Status A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Statusa(pub u32);
impl Statusa {
    #[doc = "POR Detect Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn por_det(&self) -> StatusaPorDet {
        let val = (self.0 >> 0usize) & 0x01;
        StatusaPorDet::from_bits(val as u8)
    }
    #[doc = "POR Detect Flag."]
    #[inline(always)]
    pub const fn set_por_det(&mut self, val: StatusaPorDet) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Wakeup Pin Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_flag(&self) -> StatusaWakeupFlag {
        let val = (self.0 >> 1usize) & 0x01;
        StatusaWakeupFlag::from_bits(val as u8)
    }
    #[doc = "Wakeup Pin Flag."]
    #[inline(always)]
    pub const fn set_wakeup_flag(&mut self, val: StatusaWakeupFlag) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bandgap Timer 0 Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn timer0_flag(&self) -> StatusaTimer0Flag {
        let val = (self.0 >> 2usize) & 0x01;
        StatusaTimer0Flag::from_bits(val as u8)
    }
    #[doc = "Bandgap Timer 0 Flag."]
    #[inline(always)]
    pub const fn set_timer0_flag(&mut self, val: StatusaTimer0Flag) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Bandgap Timer 1 Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn timer1_flag(&self) -> StatusaTimer1Flag {
        let val = (self.0 >> 3usize) & 0x01;
        StatusaTimer1Flag::from_bits(val as u8)
    }
    #[doc = "Bandgap Timer 1 Flag."]
    #[inline(always)]
    pub const fn set_timer1_flag(&mut self, val: StatusaTimer1Flag) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "LDO Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_rdy(&self) -> StatusaLdoRdy {
        let val = (self.0 >> 4usize) & 0x01;
        StatusaLdoRdy::from_bits(val as u8)
    }
    #[doc = "LDO Ready."]
    #[inline(always)]
    pub const fn set_ldo_rdy(&mut self, val: StatusaLdoRdy) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "OSC32k Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_rdy(&self) -> StatusaOscRdy {
        let val = (self.0 >> 5usize) & 0x01;
        StatusaOscRdy::from_bits(val as u8)
    }
    #[doc = "OSC32k Ready."]
    #[inline(always)]
    pub const fn set_osc_rdy(&mut self, val: StatusaOscRdy) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Clock Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn clock_det(&self) -> StatusaClockDet {
        let val = (self.0 >> 6usize) & 0x01;
        StatusaClockDet::from_bits(val as u8)
    }
    #[doc = "Clock Detect."]
    #[inline(always)]
    pub const fn set_clock_det(&mut self, val: StatusaClockDet) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Configuration Detect Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn config_det(&self) -> StatusaConfigDet {
        let val = (self.0 >> 7usize) & 0x01;
        StatusaConfigDet::from_bits(val as u8)
    }
    #[doc = "Configuration Detect Flag."]
    #[inline(always)]
    pub const fn set_config_det(&mut self, val: StatusaConfigDet) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Voltage Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn volt_det(&self) -> StatusaVoltDet {
        let val = (self.0 >> 8usize) & 0x01;
        StatusaVoltDet::from_bits(val as u8)
    }
    #[doc = "Voltage Detect."]
    #[inline(always)]
    pub const fn set_volt_det(&mut self, val: StatusaVoltDet) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Temperature Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn temp_det(&self) -> StatusaTempDet {
        let val = (self.0 >> 9usize) & 0x01;
        StatusaTempDet::from_bits(val as u8)
    }
    #[doc = "Temperature Detect."]
    #[inline(always)]
    pub const fn set_temp_det(&mut self, val: StatusaTempDet) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Light Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn light_det(&self) -> StatusaLightDet {
        let val = (self.0 >> 10usize) & 0x01;
        StatusaLightDet::from_bits(val as u8)
    }
    #[doc = "Light Detect."]
    #[inline(always)]
    pub const fn set_light_det(&mut self, val: StatusaLightDet) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Input 0 Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn sec0_det(&self) -> StatusaSec0Det {
        let val = (self.0 >> 12usize) & 0x01;
        StatusaSec0Det::from_bits(val as u8)
    }
    #[doc = "Input 0 Detect."]
    #[inline(always)]
    pub const fn set_sec0_det(&mut self, val: StatusaSec0Det) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn irq_det(&self, n: usize) -> StatusaIrqDet {
        assert!(n < 4usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        StatusaIrqDet::from_bits(val as u8)
    }
    #[doc = "Interrupt Detect."]
    #[inline(always)]
    pub const fn set_irq_det(&mut self, n: usize, val: StatusaIrqDet) {
        assert!(n < 4usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Statusa {
    #[inline(always)]
    fn default() -> Statusa {
        Statusa(0)
    }
}
impl core::fmt::Debug for Statusa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Statusa")
            .field("por_det", &self.por_det())
            .field("wakeup_flag", &self.wakeup_flag())
            .field("timer0_flag", &self.timer0_flag())
            .field("timer1_flag", &self.timer1_flag())
            .field("ldo_rdy", &self.ldo_rdy())
            .field("osc_rdy", &self.osc_rdy())
            .field("clock_det", &self.clock_det())
            .field("config_det", &self.config_det())
            .field("volt_det", &self.volt_det())
            .field("temp_det", &self.temp_det())
            .field("light_det", &self.light_det())
            .field("sec0_det", &self.sec0_det())
            .field("irq_det[0]", &self.irq_det(0usize))
            .field("irq_det[1]", &self.irq_det(1usize))
            .field("irq_det[2]", &self.irq_det(2usize))
            .field("irq_det[3]", &self.irq_det(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Statusa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Statusa {{ por_det: {:?}, wakeup_flag: {:?}, timer0_flag: {:?}, timer1_flag: {:?}, ldo_rdy: {:?}, osc_rdy: {:?}, clock_det: {:?}, config_det: {:?}, volt_det: {:?}, temp_det: {:?}, light_det: {:?}, sec0_det: {:?}, irq_det[0]: {:?}, irq_det[1]: {:?}, irq_det[2]: {:?}, irq_det[3]: {:?} }}",
            self.por_det(),
            self.wakeup_flag(),
            self.timer0_flag(),
            self.timer1_flag(),
            self.ldo_rdy(),
            self.osc_rdy(),
            self.clock_det(),
            self.config_det(),
            self.volt_det(),
            self.temp_det(),
            self.light_det(),
            self.sec0_det(),
            self.irq_det(0usize),
            self.irq_det(1usize),
            self.irq_det(2usize),
            self.irq_det(3usize)
        )
    }
}
#[doc = "Status B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Statusb(pub u32);
impl Statusb {
    #[doc = "Inverse value."]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Inverse value."]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Statusb {
    #[inline(always)]
    fn default() -> Statusb {
        Statusb(0)
    }
}
impl core::fmt::Debug for Statusb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Statusb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Statusb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Statusb {{ inverse: {=u32:?} }}", self.inverse())
    }
}
#[doc = "Switch Control A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swictla(pub u32);
impl Swictla {
    #[doc = "Switch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn swi_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Switch Enable."]
    #[inline(always)]
    pub const fn set_swi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Low Power Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lp_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Enable."]
    #[inline(always)]
    pub const fn set_lp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Swictla {
    #[inline(always)]
    fn default() -> Swictla {
        Swictla(0)
    }
}
impl core::fmt::Debug for Swictla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swictla")
            .field("swi_en", &self.swi_en())
            .field("lp_en", &self.lp_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swictla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swictla {{ swi_en: {=bool:?}, lp_en: {=bool:?} }}",
            self.swi_en(),
            self.lp_en()
        )
    }
}
#[doc = "Switch Control B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swictlb(pub u32);
impl Swictlb {
    #[doc = "Inverse Value."]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Inverse Value."]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for Swictlb {
    #[inline(always)]
    fn default() -> Swictlb {
        Swictlb(0)
    }
}
impl core::fmt::Debug for Swictlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swictlb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swictlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Swictlb {{ inverse: {=u8:?} }}", self.inverse())
    }
}
#[doc = "Switch Lock A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swilcka(pub u32);
impl Swilcka {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Swilcka {
    #[inline(always)]
    fn default() -> Swilcka {
        Swilcka(0)
    }
}
impl core::fmt::Debug for Swilcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swilcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swilcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Swilcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "Switch Lock B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swilckb(pub u32);
impl Swilckb {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> SwilckbLock {
        let val = (self.0 >> 0usize) & 0x01;
        SwilckbLock::from_bits(val as u8)
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: SwilckbLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Swilckb {
    #[inline(always)]
    fn default() -> Swilckb {
        Swilckb(0)
    }
}
impl core::fmt::Debug for Swilckb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swilckb")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swilckb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Swilckb {{ lock: {:?} }}", self.lock())
    }
}
#[doc = "TAMPER Control A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tamctla(pub u32);
impl Tamctla {
    #[doc = "Voltage Detect Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn volt_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Detect Enable."]
    #[inline(always)]
    pub const fn set_volt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Temperature Detect Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn temp_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Detect Enable."]
    #[inline(always)]
    pub const fn set_temp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Light Detect Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn light_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Light Detect Enable."]
    #[inline(always)]
    pub const fn set_light_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Tamctla {
    #[inline(always)]
    fn default() -> Tamctla {
        Tamctla(0)
    }
}
impl core::fmt::Debug for Tamctla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tamctla")
            .field("volt_en", &self.volt_en())
            .field("temp_en", &self.temp_en())
            .field("light_en", &self.light_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tamctla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tamctla {{ volt_en: {=bool:?}, temp_en: {=bool:?}, light_en: {=bool:?} }}",
            self.volt_en(),
            self.temp_en(),
            self.light_en()
        )
    }
}
#[doc = "TAMPER Control B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tamctlb(pub u32);
impl Tamctlb {
    #[doc = "Inverse value."]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Inverse value."]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Tamctlb {
    #[inline(always)]
    fn default() -> Tamctlb {
        Tamctlb(0)
    }
}
impl core::fmt::Debug for Tamctlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tamctlb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tamctlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tamctlb {{ inverse: {=u8:?} }}", self.inverse())
    }
}
#[doc = "TAMPER Lock A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tamlcka(pub u32);
impl Tamlcka {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Tamlcka {
    #[inline(always)]
    fn default() -> Tamlcka {
        Tamlcka(0)
    }
}
impl core::fmt::Debug for Tamlcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tamlcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tamlcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tamlcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "TAMPER Lock B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tamlckb(pub u32);
impl Tamlckb {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> TamlckbLock {
        let val = (self.0 >> 0usize) & 0x01;
        TamlckbLock::from_bits(val as u8)
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: TamlckbLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Tamlckb {
    #[inline(always)]
    fn default() -> Tamlckb {
        Tamlckb(0)
    }
}
impl core::fmt::Debug for Tamlckb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tamlckb")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tamlckb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tamlckb {{ lock: {:?} }}", self.lock())
    }
}
#[doc = "Tamper Enable A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tampera(pub u32);
impl Tampera {
    #[doc = "POR Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn por_det(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "POR Detect."]
    #[inline(always)]
    pub const fn set_por_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clock Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn clock_det(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Detect."]
    #[inline(always)]
    pub const fn set_clock_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Configuration Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn config_det(&self) -> TamperaConfigDet {
        let val = (self.0 >> 7usize) & 0x01;
        TamperaConfigDet::from_bits(val as u8)
    }
    #[doc = "Configuration Detect."]
    #[inline(always)]
    pub const fn set_config_det(&mut self, val: TamperaConfigDet) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Voltage Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn volt_det(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Detect."]
    #[inline(always)]
    pub const fn set_volt_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Temperature Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn temp_det(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Detect."]
    #[inline(always)]
    pub const fn set_temp_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Light Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn light_det(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Light Detect."]
    #[inline(always)]
    pub const fn set_light_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Input 0 Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn sec0_det(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Input 0 Detect."]
    #[inline(always)]
    pub const fn set_sec0_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Tampera {
    #[inline(always)]
    fn default() -> Tampera {
        Tampera(0)
    }
}
impl core::fmt::Debug for Tampera {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tampera")
            .field("por_det", &self.por_det())
            .field("clock_det", &self.clock_det())
            .field("config_det", &self.config_det())
            .field("volt_det", &self.volt_det())
            .field("temp_det", &self.temp_det())
            .field("light_det", &self.light_det())
            .field("sec0_det", &self.sec0_det())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tampera {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tampera {{ por_det: {=bool:?}, clock_det: {=bool:?}, config_det: {:?}, volt_det: {=bool:?}, temp_det: {=bool:?}, light_det: {=bool:?}, sec0_det: {=bool:?} }}",
            self.por_det(),
            self.clock_det(),
            self.config_det(),
            self.volt_det(),
            self.temp_det(),
            self.light_det(),
            self.sec0_det()
        )
    }
}
#[doc = "Tamper Enable B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tamperb(pub u32);
impl Tamperb {
    #[doc = "Inverse value."]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Inverse value."]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tamperb {
    #[inline(always)]
    fn default() -> Tamperb {
        Tamperb(0)
    }
}
impl core::fmt::Debug for Tamperb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tamperb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tamperb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tamperb {{ inverse: {=u16:?} }}", self.inverse())
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
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Specification Number."]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
            "Verid {{ feature: {=u16:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
#[doc = "Wake-up Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakecfg(pub u32);
impl Wakecfg {
    #[doc = "Output."]
    #[must_use]
    #[inline(always)]
    pub const fn out(&self) -> Out {
        let val = (self.0 >> 0usize) & 0x01;
        Out::from_bits(val as u8)
    }
    #[doc = "Output."]
    #[inline(always)]
    pub const fn set_out(&mut self, val: Out) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Wakecfg {
    #[inline(always)]
    fn default() -> Wakecfg {
        Wakecfg(0)
    }
}
impl core::fmt::Debug for Wakecfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakecfg").field("out", &self.out()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakecfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wakecfg {{ out: {:?} }}", self.out())
    }
}
#[doc = "Wake-up Enable A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakena(pub u32);
impl Wakena {
    #[doc = "POR Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn por_det(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "POR Detect."]
    #[inline(always)]
    pub const fn set_por_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Wake-up Pin Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_flag(&self) -> WakenaWakeupFlag {
        let val = (self.0 >> 1usize) & 0x01;
        WakenaWakeupFlag::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Flag."]
    #[inline(always)]
    pub const fn set_wakeup_flag(&mut self, val: WakenaWakeupFlag) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bandgap Timer 0."]
    #[must_use]
    #[inline(always)]
    pub const fn timer0_flag(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timer 0."]
    #[inline(always)]
    pub const fn set_timer0_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Bandgap Timer 2."]
    #[must_use]
    #[inline(always)]
    pub const fn timer1_flag(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timer 2."]
    #[inline(always)]
    pub const fn set_timer1_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "LDO Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_rdy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Ready."]
    #[inline(always)]
    pub const fn set_ldo_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OSC32K Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_rdy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OSC32K Ready."]
    #[inline(always)]
    pub const fn set_osc_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Clock Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn clock_det(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Detect."]
    #[inline(always)]
    pub const fn set_clock_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Configuration Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn config_det(&self) -> WakenaConfigDet {
        let val = (self.0 >> 7usize) & 0x01;
        WakenaConfigDet::from_bits(val as u8)
    }
    #[doc = "Configuration Detect."]
    #[inline(always)]
    pub const fn set_config_det(&mut self, val: WakenaConfigDet) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Voltage Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn volt_det(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Detect."]
    #[inline(always)]
    pub const fn set_volt_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Temperature Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn temp_det(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Detect."]
    #[inline(always)]
    pub const fn set_temp_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Light Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn light_det(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Light Detect."]
    #[inline(always)]
    pub const fn set_light_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Input 0 Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn sec0_det(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Input 0 Detect."]
    #[inline(always)]
    pub const fn set_sec0_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn irq_det(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Detect."]
    #[inline(always)]
    pub const fn set_irq_det(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Wakena {
    #[inline(always)]
    fn default() -> Wakena {
        Wakena(0)
    }
}
impl core::fmt::Debug for Wakena {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakena")
            .field("por_det", &self.por_det())
            .field("wakeup_flag", &self.wakeup_flag())
            .field("timer0_flag", &self.timer0_flag())
            .field("timer1_flag", &self.timer1_flag())
            .field("ldo_rdy", &self.ldo_rdy())
            .field("osc_rdy", &self.osc_rdy())
            .field("clock_det", &self.clock_det())
            .field("config_det", &self.config_det())
            .field("volt_det", &self.volt_det())
            .field("temp_det", &self.temp_det())
            .field("light_det", &self.light_det())
            .field("sec0_det", &self.sec0_det())
            .field("irq_det[0]", &self.irq_det(0usize))
            .field("irq_det[1]", &self.irq_det(1usize))
            .field("irq_det[2]", &self.irq_det(2usize))
            .field("irq_det[3]", &self.irq_det(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakena {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wakena {{ por_det: {=bool:?}, wakeup_flag: {:?}, timer0_flag: {=bool:?}, timer1_flag: {=bool:?}, ldo_rdy: {=bool:?}, osc_rdy: {=bool:?}, clock_det: {=bool:?}, config_det: {:?}, volt_det: {=bool:?}, temp_det: {=bool:?}, light_det: {=bool:?}, sec0_det: {=bool:?}, irq_det[0]: {=bool:?}, irq_det[1]: {=bool:?}, irq_det[2]: {=bool:?}, irq_det[3]: {=bool:?} }}",
            self.por_det(),
            self.wakeup_flag(),
            self.timer0_flag(),
            self.timer1_flag(),
            self.ldo_rdy(),
            self.osc_rdy(),
            self.clock_det(),
            self.config_det(),
            self.volt_det(),
            self.temp_det(),
            self.light_det(),
            self.sec0_det(),
            self.irq_det(0usize),
            self.irq_det(1usize),
            self.irq_det(2usize),
            self.irq_det(3usize)
        )
    }
}
#[doc = "Wake-up Enable B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakenb(pub u32);
impl Wakenb {
    #[doc = "Inverse Value."]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Inverse Value."]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Wakenb {
    #[inline(always)]
    fn default() -> Wakenb {
        Wakenb(0)
    }
}
impl core::fmt::Debug for Wakenb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakenb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakenb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wakenb {{ inverse: {=u32:?} }}", self.inverse())
    }
}
#[doc = "Wakeup 0 Register A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeupa(pub u32);
impl Wakeupa {
    #[doc = "Register."]
    #[must_use]
    #[inline(always)]
    pub const fn reg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register."]
    #[inline(always)]
    pub const fn set_reg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Wakeupa {
    #[inline(always)]
    fn default() -> Wakeupa {
        Wakeupa(0)
    }
}
impl core::fmt::Debug for Wakeupa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakeupa").field("reg", &self.reg()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakeupa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wakeupa {{ reg: {=u32:?} }}", self.reg())
    }
}
#[doc = "Wakeup 0 Register B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeupb(pub u32);
impl Wakeupb {
    #[doc = "Inverse value."]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Inverse value."]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Wakeupb {
    #[inline(always)]
    fn default() -> Wakeupb {
        Wakeupb(0)
    }
}
impl core::fmt::Debug for Wakeupb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakeupb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakeupb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wakeupb {{ inverse: {=u32:?} }}", self.inverse())
    }
}
#[doc = "Wakeup Lock A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Waklcka(pub u32);
impl Waklcka {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Waklcka {
    #[inline(always)]
    fn default() -> Waklcka {
        Waklcka(0)
    }
}
impl core::fmt::Debug for Waklcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Waklcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Waklcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Waklcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "Wakeup Lock B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Waklckb(pub u32);
impl Waklckb {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> WaklckbLock {
        let val = (self.0 >> 0usize) & 0x01;
        WaklckbLock::from_bits(val as u8)
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: WaklckbLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Waklckb {
    #[inline(always)]
    fn default() -> Waklckb {
        Waklckb(0)
    }
}
impl core::fmt::Debug for Waklckb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Waklckb")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Waklckb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Waklckb {{ lock: {:?} }}", self.lock())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CapTrim {
    #[doc = "Default (when CAP2_TRIM = 0 and CAP_TRIM\\[1:0\\] = 00 )."]
    Val0 = 0x0,
    #[doc = "-1us (when CAP2_TRIM = 0 and CAP_TRIM\\[1:0\\] = 01)."]
    Val1 = 0x01,
    #[doc = "-2us (when CAP2_TRIM = 0 and CAP_TRIM\\[1:0\\] = 10) or or +3.5us (when CAP2_TRIM = 1 and CAP_TRIM\\[1:0\\] = 10)."]
    Val2 = 0x02,
    #[doc = "-2.5us (when CAP2_TRIM = 0 and CAP_TRIM\\[1:0\\] = 11) or +1us (when CAP2_TRIM = 1 and CAP_TRIM\\[1:0\\] = 11)."]
    Val3 = 0x03,
}
impl CapTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CapTrim {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CapTrim {
    #[inline(always)]
    fn from(val: u8) -> CapTrim {
        CapTrim::from_bits(val)
    }
}
impl From<CapTrim> for u8 {
    #[inline(always)]
    fn from(val: CapTrim) -> u8 {
        CapTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmpTrim {
    #[doc = "760 mV."]
    CmpTrim0 = 0x0,
    #[doc = "770 mV."]
    CmpTrim1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "740 mV."]
    CmpTrim3 = 0x03,
}
impl CmpTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpTrim {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpTrim {
    #[inline(always)]
    fn from(val: u8) -> CmpTrim {
        CmpTrim::from_bits(val)
    }
}
impl From<CmpTrim> for u8 {
    #[inline(always)]
    fn from(val: CmpTrim) -> u8 {
        CmpTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoarseAmpGain {
    #[doc = "ESR Range 0."]
    Gain05 = 0x0,
    #[doc = "ESR Range 1."]
    Gain10 = 0x01,
    #[doc = "ESR Range 2."]
    Gain18 = 0x02,
    #[doc = "ESR Range 3."]
    Gain33 = 0x03,
}
impl CoarseAmpGain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoarseAmpGain {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoarseAmpGain {
    #[inline(always)]
    fn from(val: u8) -> CoarseAmpGain {
        CoarseAmpGain::from_bits(val)
    }
}
impl From<CoarseAmpGain> for u8 {
    #[inline(always)]
    fn from(val: CoarseAmpGain) -> u8 {
        CoarseAmpGain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DivideTrim {
    #[doc = "Clock monitor operates at 1 kHz."]
    Cfg0 = 0x0,
    #[doc = "Clock monitor operates at 64 Hz."]
    Cfg1 = 0x01,
}
impl DivideTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DivideTrim {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DivideTrim {
    #[inline(always)]
    fn from(val: u8) -> DivideTrim {
        DivideTrim::from_bits(val)
    }
}
impl From<DivideTrim> for u8 {
    #[inline(always)]
    fn from(val: DivideTrim) -> u8 {
        DivideTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DlyTrim {
    #[doc = "P current 9(nA) and N Current 6(nA)."]
    DlyTrim0 = 0x0,
    #[doc = "P current 13(nA) and N Current 6(nA)."]
    DlyTrim1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "P current 4(nA) and N Current 6(nA)."]
    DlyTrim3 = 0x03,
    #[doc = "P current 9(nA) and N Current 4(nA)."]
    DlyTrim4 = 0x04,
    #[doc = "P current 13(nA) and N Current 4(nA)."]
    DlyTrim5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "P current 4(nA) and N Current 4(nA)."]
    DlyTrim7 = 0x07,
    #[doc = "P current 9(nA) and N Current 2(nA)."]
    DlyTrim8 = 0x08,
    #[doc = "P current 13(nA) and N Current 2(nA)."]
    DlyTrim9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "P current 4(nA) and N Current 2(nA)."]
    DlyTrim11 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl DlyTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DlyTrim {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DlyTrim {
    #[inline(always)]
    fn from(val: u8) -> DlyTrim {
        DlyTrim::from_bits(val)
    }
}
impl From<DlyTrim> for u8 {
    #[inline(always)]
    fn from(val: DlyTrim) -> u8 {
        DlyTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExtalCapSel {
    #[doc = "0 pF."]
    Sel0 = 0x0,
    #[doc = "2 pF."]
    Sel2 = 0x01,
    #[doc = "4 pF."]
    Sel4 = 0x02,
    #[doc = "6 pF."]
    Sel6 = 0x03,
    #[doc = "8 pF."]
    Sel8 = 0x04,
    #[doc = "10 pF."]
    Sel10 = 0x05,
    #[doc = "12 pF."]
    Sel12 = 0x06,
    #[doc = "14 pF."]
    Sel14 = 0x07,
    #[doc = "16 pF."]
    Sel16 = 0x08,
    #[doc = "18 pF."]
    Sel18 = 0x09,
    #[doc = "20 pF."]
    Sel20 = 0x0a,
    #[doc = "22 pF."]
    Sel22 = 0x0b,
    #[doc = "24 pF."]
    Sel24 = 0x0c,
    #[doc = "26 pF."]
    Sel26 = 0x0d,
    #[doc = "28 pF."]
    Sel28 = 0x0e,
    #[doc = "30 pF."]
    Sel30 = 0x0f,
}
impl ExtalCapSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtalCapSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtalCapSel {
    #[inline(always)]
    fn from(val: u8) -> ExtalCapSel {
        ExtalCapSel::from_bits(val)
    }
}
impl From<ExtalCapSel> for u8 {
    #[inline(always)]
    fn from(val: ExtalCapSel) -> u8 {
        ExtalCapSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqTrim {
    #[doc = "Clock monitor asserts 2 cycle after expected edge."]
    Cfg0 = 0x0,
    #[doc = "Clock monitor asserts 4 cycles after expected edge."]
    Cfg1 = 0x01,
    #[doc = "Clock monitor asserts 6 cycles after expected edge."]
    Cfg2 = 0x02,
    #[doc = "Clock monitor asserts 8 cycles after expected edge."]
    Cfg3 = 0x03,
}
impl FreqTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqTrim {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqTrim {
    #[inline(always)]
    fn from(val: u8) -> FreqTrim {
        FreqTrim::from_bits(val)
    }
}
impl From<FreqTrim> for u8 {
    #[inline(always)]
    fn from(val: FreqTrim) -> u8 {
        FreqTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrolckbLock {
    #[doc = "Block."]
    Enable = 0x0,
    #[doc = "Do not block."]
    Disable = 0x01,
}
impl FrolckbLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrolckbLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrolckbLock {
    #[inline(always)]
    fn from(val: u8) -> FrolckbLock {
        FrolckbLock::from_bits(val)
    }
}
impl From<FrolckbLock> for u8 {
    #[inline(always)]
    fn from(val: FrolckbLock) -> u8 {
        FrolckbLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InitTrim {
    #[doc = "8 s."]
    Sel0 = 0x0,
    #[doc = "4 s."]
    Sel1 = 0x01,
    #[doc = "2 s."]
    Sel2 = 0x02,
    #[doc = "1 s."]
    Sel3 = 0x03,
    #[doc = "0.5 s."]
    Sel4 = 0x04,
    #[doc = "0.25 s."]
    Sel5 = 0x05,
    #[doc = "0.125 s."]
    Sel6 = 0x06,
    #[doc = "0.5 ms."]
    Sel7 = 0x07,
}
impl InitTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InitTrim {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InitTrim {
    #[inline(always)]
    fn from(val: u8) -> InitTrim {
        InitTrim::from_bits(val)
    }
}
impl From<InitTrim> for u8 {
    #[inline(always)]
    fn from(val: InitTrim) -> u8 {
        InitTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IrqenaConfigDet {
    #[doc = "Disable."]
    Clr = 0x0,
    #[doc = "Enable."]
    Set = 0x01,
}
impl IrqenaConfigDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IrqenaConfigDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IrqenaConfigDet {
    #[inline(always)]
    fn from(val: u8) -> IrqenaConfigDet {
        IrqenaConfigDet::from_bits(val)
    }
}
impl From<IrqenaConfigDet> for u8 {
    #[inline(always)]
    fn from(val: IrqenaConfigDet) -> u8 {
        IrqenaConfigDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IrqenaWakeupFlag {
    #[doc = "Disable."]
    Clr = 0x0,
    #[doc = "Enable."]
    Set = 0x01,
}
impl IrqenaWakeupFlag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IrqenaWakeupFlag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IrqenaWakeupFlag {
    #[inline(always)]
    fn from(val: u8) -> IrqenaWakeupFlag {
        IrqenaWakeupFlag::from_bits(val)
    }
}
impl From<IrqenaWakeupFlag> for u8 {
    #[inline(always)]
    fn from(val: IrqenaWakeupFlag) -> u8 {
        IrqenaWakeupFlag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LdolckbLock {
    #[doc = "Block."]
    Enable = 0x0,
    #[doc = "Do not block."]
    Disable = 0x01,
}
impl LdolckbLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LdolckbLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LdolckbLock {
    #[inline(always)]
    fn from(val: u8) -> LdolckbLock {
        LdolckbLock::from_bits(val)
    }
}
impl From<LdolckbLock> for u8 {
    #[inline(always)]
    fn from(val: LdolckbLock) -> u8 {
        LdolckbLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockbLock {
    #[doc = "Enables lock."]
    Enable = 0x0,
    #[doc = "Disables lock."]
    Disable = 0x01,
}
impl LockbLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockbLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockbLock {
    #[inline(always)]
    fn from(val: u8) -> LockbLock {
        LockbLock::from_bits(val)
    }
}
impl From<LockbLock> for u8 {
    #[inline(always)]
    fn from(val: LockbLock) -> u8 {
        LockbLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModeEn {
    #[doc = "Normal mode."]
    Hp = 0x0,
    #[doc = "Startup mode."]
    Lp = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Low power mode."]
    Sw = 0x03,
}
impl ModeEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ModeEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ModeEn {
    #[inline(always)]
    fn from(val: u8) -> ModeEn {
        ModeEn::from_bits(val)
    }
}
impl From<ModeEn> for u8 {
    #[inline(always)]
    fn from(val: ModeEn) -> u8 {
        ModeEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MonlckbLock {
    #[doc = "Lock is enabled."]
    Enable = 0x0,
    #[doc = "Lock is disabled."]
    Disable = 0x01,
}
impl MonlckbLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MonlckbLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MonlckbLock {
    #[inline(always)]
    fn from(val: u8) -> MonlckbLock {
        MonlckbLock::from_bits(val)
    }
}
impl From<MonlckbLock> for u8 {
    #[inline(always)]
    fn from(val: MonlckbLock) -> u8 {
        MonlckbLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OsclckbLock {
    #[doc = "Block."]
    Enable = 0x0,
    #[doc = "Do not block."]
    Disable = 0x01,
}
impl OsclckbLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OsclckbLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OsclckbLock {
    #[inline(always)]
    fn from(val: u8) -> OsclckbLock {
        OsclckbLock::from_bits(val)
    }
}
impl From<OsclckbLock> for u8 {
    #[inline(always)]
    fn from(val: OsclckbLock) -> u8 {
        OsclckbLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out {
    #[doc = "Logic zero (asserted)."]
    On = 0x0,
    #[doc = "Logic one."]
    Off = 0x01,
}
impl Out {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out {
    #[inline(always)]
    fn from(val: u8) -> Out {
        Out::from_bits(val)
    }
}
impl From<Out> for u8 {
    #[inline(always)]
    fn from(val: Out) -> u8 {
        Out::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaClockDet {
    #[doc = "Clock error not detected."]
    Clr = 0x0,
    #[doc = "Clock error detected."]
    Set = 0x01,
}
impl StatusaClockDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaClockDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaClockDet {
    #[inline(always)]
    fn from(val: u8) -> StatusaClockDet {
        StatusaClockDet::from_bits(val)
    }
}
impl From<StatusaClockDet> for u8 {
    #[inline(always)]
    fn from(val: StatusaClockDet) -> u8 {
        StatusaClockDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaConfigDet {
    #[doc = "Not detected."]
    Clr = 0x0,
    #[doc = "Detected."]
    Set = 0x01,
}
impl StatusaConfigDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaConfigDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaConfigDet {
    #[inline(always)]
    fn from(val: u8) -> StatusaConfigDet {
        StatusaConfigDet::from_bits(val)
    }
}
impl From<StatusaConfigDet> for u8 {
    #[inline(always)]
    fn from(val: StatusaConfigDet) -> u8 {
        StatusaConfigDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaIrqDet {
    #[doc = "Not asserted."]
    Clr = 0x0,
    #[doc = "Asserted."]
    Set = 0x01,
}
impl StatusaIrqDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaIrqDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaIrqDet {
    #[inline(always)]
    fn from(val: u8) -> StatusaIrqDet {
        StatusaIrqDet::from_bits(val)
    }
}
impl From<StatusaIrqDet> for u8 {
    #[inline(always)]
    fn from(val: StatusaIrqDet) -> u8 {
        StatusaIrqDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaLdoRdy {
    #[doc = "Disabled (not ready)."]
    Clr = 0x0,
    #[doc = "Enabled (ready)."]
    Set = 0x01,
}
impl StatusaLdoRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaLdoRdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaLdoRdy {
    #[inline(always)]
    fn from(val: u8) -> StatusaLdoRdy {
        StatusaLdoRdy::from_bits(val)
    }
}
impl From<StatusaLdoRdy> for u8 {
    #[inline(always)]
    fn from(val: StatusaLdoRdy) -> u8 {
        StatusaLdoRdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaLightDet {
    #[doc = "Light error not detected."]
    Clr = 0x0,
    #[doc = "Light error detected."]
    Set = 0x01,
}
impl StatusaLightDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaLightDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaLightDet {
    #[inline(always)]
    fn from(val: u8) -> StatusaLightDet {
        StatusaLightDet::from_bits(val)
    }
}
impl From<StatusaLightDet> for u8 {
    #[inline(always)]
    fn from(val: StatusaLightDet) -> u8 {
        StatusaLightDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaOscRdy {
    #[doc = "Disabled (clock not ready)."]
    Clr = 0x0,
    #[doc = "Enabled (clock ready)."]
    Set = 0x01,
}
impl StatusaOscRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaOscRdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaOscRdy {
    #[inline(always)]
    fn from(val: u8) -> StatusaOscRdy {
        StatusaOscRdy::from_bits(val)
    }
}
impl From<StatusaOscRdy> for u8 {
    #[inline(always)]
    fn from(val: StatusaOscRdy) -> u8 {
        StatusaOscRdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaPorDet {
    #[doc = "Not reset."]
    Clr = 0x0,
    #[doc = "Reset."]
    Set = 0x01,
}
impl StatusaPorDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaPorDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaPorDet {
    #[inline(always)]
    fn from(val: u8) -> StatusaPorDet {
        StatusaPorDet::from_bits(val)
    }
}
impl From<StatusaPorDet> for u8 {
    #[inline(always)]
    fn from(val: StatusaPorDet) -> u8 {
        StatusaPorDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaSec0Det {
    #[doc = "Security input 0 not detected."]
    Clr = 0x0,
    #[doc = "Security input 0 detected."]
    Set = 0x01,
}
impl StatusaSec0Det {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaSec0Det {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaSec0Det {
    #[inline(always)]
    fn from(val: u8) -> StatusaSec0Det {
        StatusaSec0Det::from_bits(val)
    }
}
impl From<StatusaSec0Det> for u8 {
    #[inline(always)]
    fn from(val: StatusaSec0Det) -> u8 {
        StatusaSec0Det::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaTempDet {
    #[doc = "Temperature error not detected."]
    Clr = 0x0,
    #[doc = "Temperature error detected."]
    Set = 0x01,
}
impl StatusaTempDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaTempDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaTempDet {
    #[inline(always)]
    fn from(val: u8) -> StatusaTempDet {
        StatusaTempDet::from_bits(val)
    }
}
impl From<StatusaTempDet> for u8 {
    #[inline(always)]
    fn from(val: StatusaTempDet) -> u8 {
        StatusaTempDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaTimer0Flag {
    #[doc = "Not reached."]
    Clr = 0x0,
    #[doc = "Reached."]
    Set = 0x01,
}
impl StatusaTimer0Flag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaTimer0Flag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaTimer0Flag {
    #[inline(always)]
    fn from(val: u8) -> StatusaTimer0Flag {
        StatusaTimer0Flag::from_bits(val)
    }
}
impl From<StatusaTimer0Flag> for u8 {
    #[inline(always)]
    fn from(val: StatusaTimer0Flag) -> u8 {
        StatusaTimer0Flag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaTimer1Flag {
    #[doc = "Not reached."]
    Clr = 0x0,
    #[doc = "Reached."]
    Set = 0x01,
}
impl StatusaTimer1Flag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaTimer1Flag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaTimer1Flag {
    #[inline(always)]
    fn from(val: u8) -> StatusaTimer1Flag {
        StatusaTimer1Flag::from_bits(val)
    }
}
impl From<StatusaTimer1Flag> for u8 {
    #[inline(always)]
    fn from(val: StatusaTimer1Flag) -> u8 {
        StatusaTimer1Flag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaVoltDet {
    #[doc = "Not detected."]
    Clr = 0x0,
    #[doc = "Detected."]
    Set = 0x01,
}
impl StatusaVoltDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaVoltDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaVoltDet {
    #[inline(always)]
    fn from(val: u8) -> StatusaVoltDet {
        StatusaVoltDet::from_bits(val)
    }
}
impl From<StatusaVoltDet> for u8 {
    #[inline(always)]
    fn from(val: StatusaVoltDet) -> u8 {
        StatusaVoltDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaWakeupFlag {
    #[doc = "Not asserted."]
    Clr = 0x0,
    #[doc = "Asserted."]
    Set = 0x01,
}
impl StatusaWakeupFlag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaWakeupFlag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaWakeupFlag {
    #[inline(always)]
    fn from(val: u8) -> StatusaWakeupFlag {
        StatusaWakeupFlag::from_bits(val)
    }
}
impl From<StatusaWakeupFlag> for u8 {
    #[inline(always)]
    fn from(val: StatusaWakeupFlag) -> u8 {
        StatusaWakeupFlag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SupplyDet {
    #[doc = "VBAT supply is less than 3V."]
    SupplyDet0 = 0x0,
    #[doc = "VBAT supply is greater than 3V."]
    SupplyDet1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SupplyDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SupplyDet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SupplyDet {
    #[inline(always)]
    fn from(val: u8) -> SupplyDet {
        SupplyDet::from_bits(val)
    }
}
impl From<SupplyDet> for u8 {
    #[inline(always)]
    fn from(val: SupplyDet) -> u8 {
        SupplyDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwilckbLock {
    #[doc = "Block."]
    Enable = 0x0,
    #[doc = "Do not block."]
    Disable = 0x01,
}
impl SwilckbLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwilckbLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwilckbLock {
    #[inline(always)]
    fn from(val: u8) -> SwilckbLock {
        SwilckbLock::from_bits(val)
    }
}
impl From<SwilckbLock> for u8 {
    #[inline(always)]
    fn from(val: SwilckbLock) -> u8 {
        SwilckbLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TamlckbLock {
    #[doc = "Lock is enabled."]
    Enable = 0x0,
    #[doc = "Lock is disabled."]
    Disable = 0x01,
}
impl TamlckbLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TamlckbLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TamlckbLock {
    #[inline(always)]
    fn from(val: u8) -> TamlckbLock {
        TamlckbLock::from_bits(val)
    }
}
impl From<TamlckbLock> for u8 {
    #[inline(always)]
    fn from(val: TamlckbLock) -> u8 {
        TamlckbLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TamperaConfigDet {
    #[doc = "Tamper disabled."]
    Clr = 0x0,
    #[doc = "Tamper enabled."]
    Set = 0x01,
}
impl TamperaConfigDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TamperaConfigDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TamperaConfigDet {
    #[inline(always)]
    fn from(val: u8) -> TamperaConfigDet {
        TamperaConfigDet::from_bits(val)
    }
}
impl From<TamperaConfigDet> for u8 {
    #[inline(always)]
    fn from(val: TamperaConfigDet) -> u8 {
        TamperaConfigDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timcfg {
    #[doc = "1 s."]
    Cfg1000 = 0x0,
    #[doc = "500 ms."]
    Cfg500 = 0x01,
    #[doc = "250 ms."]
    Cfg250 = 0x02,
    #[doc = "125 ms."]
    Cfg125 = 0x03,
    #[doc = "62.5 ms."]
    Cfg62 = 0x04,
    #[doc = "31.25 ms."]
    Cfg31 = 0x05,
    #[doc = "15.625 ms."]
    Cfg15 = 0x06,
    #[doc = "7.8125 ms."]
    Cfg7 = 0x07,
}
impl Timcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timcfg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timcfg {
    #[inline(always)]
    fn from(val: u8) -> Timcfg {
        Timcfg::from_bits(val)
    }
}
impl From<Timcfg> for u8 {
    #[inline(always)]
    fn from(val: Timcfg) -> u8 {
        Timcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakenaConfigDet {
    #[doc = "Disable."]
    Clr = 0x0,
    #[doc = "Enable."]
    Set = 0x01,
}
impl WakenaConfigDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WakenaConfigDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WakenaConfigDet {
    #[inline(always)]
    fn from(val: u8) -> WakenaConfigDet {
        WakenaConfigDet::from_bits(val)
    }
}
impl From<WakenaConfigDet> for u8 {
    #[inline(always)]
    fn from(val: WakenaConfigDet) -> u8 {
        WakenaConfigDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakenaWakeupFlag {
    #[doc = "Disable."]
    Clr = 0x0,
    #[doc = "Enable."]
    Set = 0x01,
}
impl WakenaWakeupFlag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WakenaWakeupFlag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WakenaWakeupFlag {
    #[inline(always)]
    fn from(val: u8) -> WakenaWakeupFlag {
        WakenaWakeupFlag::from_bits(val)
    }
}
impl From<WakenaWakeupFlag> for u8 {
    #[inline(always)]
    fn from(val: WakenaWakeupFlag) -> u8 {
        WakenaWakeupFlag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WaklckbLock {
    #[doc = "Lock is enabled."]
    Enable = 0x0,
    #[doc = "Lock is disabled."]
    Disable = 0x01,
}
impl WaklckbLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WaklckbLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WaklckbLock {
    #[inline(always)]
    fn from(val: u8) -> WaklckbLock {
        WaklckbLock::from_bits(val)
    }
}
impl From<WaklckbLock> for u8 {
    #[inline(always)]
    fn from(val: WaklckbLock) -> u8 {
        WaklckbLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XtalCapSel {
    #[doc = "0 pF."]
    Sel0 = 0x0,
    #[doc = "2 pF."]
    Sel2 = 0x01,
    #[doc = "4 pF."]
    Sel4 = 0x02,
    #[doc = "6 pF."]
    Sel6 = 0x03,
    #[doc = "8 pF."]
    Sel8 = 0x04,
    #[doc = "10 pF."]
    Sel10 = 0x05,
    #[doc = "12 pF."]
    Sel12 = 0x06,
    #[doc = "14 pF."]
    Sel14 = 0x07,
    #[doc = "16 pF."]
    Sel16 = 0x08,
    #[doc = "18 pF."]
    Sel18 = 0x09,
    #[doc = "20 pF."]
    Sel20 = 0x0a,
    #[doc = "22 pF."]
    Sel22 = 0x0b,
    #[doc = "24 pF."]
    Sel24 = 0x0c,
    #[doc = "26 pF."]
    Sel26 = 0x0d,
    #[doc = "28 pF."]
    Sel28 = 0x0e,
    #[doc = "30 pF."]
    Sel30 = 0x0f,
}
impl XtalCapSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XtalCapSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XtalCapSel {
    #[inline(always)]
    fn from(val: u8) -> XtalCapSel {
        XtalCapSel::from_bits(val)
    }
}
impl From<XtalCapSel> for u8 {
    #[inline(always)]
    fn from(val: XtalCapSel) -> u8 {
        XtalCapSel::to_bits(val)
    }
}
