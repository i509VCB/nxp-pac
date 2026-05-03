#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Core Mode Controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmc {
    ptr: *mut u8,
}
unsafe impl Send for Cmc {}
unsafe impl Sync for Cmc {}
impl Cmc {
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
    pub const fn param(self) -> crate::pac::common::Reg<u32, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Clock Control."]
    #[inline(always)]
    pub const fn ckctrl(self) -> crate::pac::common::Reg<Ckctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Clock Status."]
    #[inline(always)]
    pub const fn ckstat(self) -> crate::pac::common::Reg<Ckstat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Power Mode Protection."]
    #[inline(always)]
    pub const fn pmprot(self) -> crate::pac::common::Reg<Pmprot, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Global Power Mode Control."]
    #[inline(always)]
    pub const fn gpmctrl(self) -> crate::pac::common::Reg<Gpmctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Power Mode Control."]
    #[inline(always)]
    pub const fn pmctrlmain(self) -> crate::pac::common::Reg<Pmctrlmain, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "System Reset Status."]
    #[inline(always)]
    pub const fn srs(self) -> crate::pac::common::Reg<Srs, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Reset Pin Control."]
    #[inline(always)]
    pub const fn rpc(self) -> crate::pac::common::Reg<Rpc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Sticky System Reset Status."]
    #[inline(always)]
    pub const fn ssrs(self) -> crate::pac::common::Reg<Ssrs, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "System Reset Interrupt Enable."]
    #[inline(always)]
    pub const fn srie(self) -> crate::pac::common::Reg<Srie, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "System Reset Interrupt Flag."]
    #[inline(always)]
    pub const fn srif(self) -> crate::pac::common::Reg<Srif, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Mode."]
    #[inline(always)]
    pub const fn mr0(self) -> crate::pac::common::Reg<Mr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Force Mode."]
    #[inline(always)]
    pub const fn fm0(self) -> crate::pac::common::Reg<Fm0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "SRAM Shut Down Register."]
    #[inline(always)]
    pub const fn sramdis0(self) -> crate::pac::common::Reg<Sramdis0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "SRAM Deep Sleep Register."]
    #[inline(always)]
    pub const fn sramret0(self) -> crate::pac::common::Reg<Sramret0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Flash Control."]
    #[inline(always)]
    pub const fn flashcr(self) -> crate::pac::common::Reg<Flashcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "BootROM Status Register."]
    #[inline(always)]
    pub const fn bsr(self) -> crate::pac::common::Reg<Bsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Core Control."]
    #[inline(always)]
    pub const fn corectl(self) -> crate::pac::common::Reg<Corectl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Debug Control."]
    #[inline(always)]
    pub const fn dbgctl(self) -> crate::pac::common::Reg<Dbgctl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Unlock Register."]
    #[inline(always)]
    pub const fn unlock(self) -> crate::pac::common::Reg<Unlock, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Test Register."]
    #[inline(always)]
    pub const fn test(self) -> crate::pac::common::Reg<Test, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
}
#[doc = "BootROM Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bsr(pub u32);
impl Bsr {
    #[doc = "Provides status information written by the BootROM."]
    #[must_use]
    #[inline(always)]
    pub const fn stat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Provides status information written by the BootROM."]
    #[inline(always)]
    pub const fn set_stat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bsr {
    #[inline(always)]
    fn default() -> Bsr {
        Bsr(0)
    }
}
impl core::fmt::Debug for Bsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bsr").field("stat", &self.stat()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bsr {{ stat: {=u32:?} }}", self.stat())
    }
}
#[doc = "Clock Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ckctrl(pub u32);
impl Ckctrl {
    #[doc = "Clocking Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ckmode(&self) -> Ckmode {
        let val = (self.0 >> 0usize) & 0x0f;
        Ckmode::from_bits(val as u8)
    }
    #[doc = "Clocking Mode."]
    #[inline(always)]
    pub const fn set_ckmode(&mut self, val: Ckmode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ckctrl {
    #[inline(always)]
    fn default() -> Ckctrl {
        Ckctrl(0)
    }
}
impl core::fmt::Debug for Ckctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ckctrl")
            .field("ckmode", &self.ckmode())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ckctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ckctrl {{ ckmode: {:?}, lock: {=bool:?} }}",
            self.ckmode(),
            self.lock()
        )
    }
}
#[doc = "Clock Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ckstat(pub u32);
impl Ckstat {
    #[doc = "Low Power Status."]
    #[must_use]
    #[inline(always)]
    pub const fn ckmode(&self) -> Ckmode {
        let val = (self.0 >> 0usize) & 0x0f;
        Ckmode::from_bits(val as u8)
    }
    #[doc = "Low Power Status."]
    #[inline(always)]
    pub const fn set_ckmode(&mut self, val: Ckmode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Wake-up Source."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Wake-up Source."]
    #[inline(always)]
    pub const fn set_wakeup(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Clock Status Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Status Valid."]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ckstat {
    #[inline(always)]
    fn default() -> Ckstat {
        Ckstat(0)
    }
}
impl core::fmt::Debug for Ckstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ckstat")
            .field("ckmode", &self.ckmode())
            .field("wakeup", &self.wakeup())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ckstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ckstat {{ ckmode: {:?}, wakeup: {=u8:?}, valid: {=bool:?} }}",
            self.ckmode(),
            self.wakeup(),
            self.valid()
        )
    }
}
#[doc = "Core Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Corectl(pub u32);
impl Corectl {
    #[doc = "Non-maskable Pin Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn npie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Non-maskable Pin Interrupt Enable."]
    #[inline(always)]
    pub const fn set_npie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Corectl {
    #[inline(always)]
    fn default() -> Corectl {
        Corectl(0)
    }
}
impl core::fmt::Debug for Corectl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Corectl")
            .field("npie", &self.npie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Corectl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Corectl {{ npie: {=bool:?} }}", self.npie())
    }
}
#[doc = "Debug Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgctl(pub u32);
impl Dbgctl {
    #[doc = "Sleep Or Debug."]
    #[must_use]
    #[inline(always)]
    pub const fn sod(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Sleep Or Debug."]
    #[inline(always)]
    pub const fn set_sod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Dbgctl {
    #[inline(always)]
    fn default() -> Dbgctl {
        Dbgctl(0)
    }
}
impl core::fmt::Debug for Dbgctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbgctl").field("sod", &self.sod()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dbgctl {{ sod: {=bool:?} }}", self.sod())
    }
}
#[doc = "Flash Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flashcr(pub u32);
impl Flashcr {
    #[doc = "Flash Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn flashdis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Disable."]
    #[inline(always)]
    pub const fn set_flashdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Flash Doze."]
    #[must_use]
    #[inline(always)]
    pub const fn flashdoze(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Doze."]
    #[inline(always)]
    pub const fn set_flashdoze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Flash Wake."]
    #[must_use]
    #[inline(always)]
    pub const fn flashwake(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Wake."]
    #[inline(always)]
    pub const fn set_flashwake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Flashcr {
    #[inline(always)]
    fn default() -> Flashcr {
        Flashcr(0)
    }
}
impl core::fmt::Debug for Flashcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flashcr")
            .field("flashdis", &self.flashdis())
            .field("flashdoze", &self.flashdoze())
            .field("flashwake", &self.flashwake())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flashcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flashcr {{ flashdis: {=bool:?}, flashdoze: {=bool:?}, flashwake: {=bool:?} }}",
            self.flashdis(),
            self.flashdoze(),
            self.flashwake()
        )
    }
}
#[doc = "Force Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fm0(pub u32);
impl Fm0 {
    #[doc = "Boot Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn forcecfg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Boot Configuration."]
    #[inline(always)]
    pub const fn set_forcecfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Fm0 {
    #[inline(always)]
    fn default() -> Fm0 {
        Fm0(0)
    }
}
impl core::fmt::Debug for Fm0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fm0")
            .field("forcecfg", &self.forcecfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fm0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fm0 {{ forcecfg: {=bool:?} }}", self.forcecfg())
    }
}
#[doc = "Global Power Mode Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpmctrl(pub u32);
impl Gpmctrl {
    #[doc = "Low-Power Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn lpmode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Low-Power Mode."]
    #[inline(always)]
    pub const fn set_lpmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Gpmctrl {
    #[inline(always)]
    fn default() -> Gpmctrl {
        Gpmctrl(0)
    }
}
impl core::fmt::Debug for Gpmctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpmctrl")
            .field("lpmode", &self.lpmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpmctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpmctrl {{ lpmode: {=u8:?} }}", self.lpmode())
    }
}
#[doc = "Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mr0(pub u32);
impl Mr0 {
    #[doc = "Boot Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ispmode_n(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Boot Configuration."]
    #[inline(always)]
    pub const fn set_ispmode_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Mr0 {
    #[inline(always)]
    fn default() -> Mr0 {
        Mr0(0)
    }
}
impl core::fmt::Debug for Mr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mr0")
            .field("ispmode_n", &self.ispmode_n())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mr0 {{ ispmode_n: {=bool:?} }}", self.ispmode_n())
    }
}
#[doc = "Power Mode Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmctrlmain(pub u32);
impl Pmctrlmain {
    #[doc = "Low-Power Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn lpmode(&self) -> PmctrlmainLpmode {
        let val = (self.0 >> 0usize) & 0x0f;
        PmctrlmainLpmode::from_bits(val as u8)
    }
    #[doc = "Low-Power Mode."]
    #[inline(always)]
    pub const fn set_lpmode(&mut self, val: PmctrlmainLpmode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Pmctrlmain {
    #[inline(always)]
    fn default() -> Pmctrlmain {
        Pmctrlmain(0)
    }
}
impl core::fmt::Debug for Pmctrlmain {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmctrlmain")
            .field("lpmode", &self.lpmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmctrlmain {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pmctrlmain {{ lpmode: {:?} }}", self.lpmode())
    }
}
#[doc = "Power Mode Protection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmprot(pub u32);
impl Pmprot {
    #[doc = "Low-Power Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn lpmode(&self) -> PmprotLpmode {
        let val = (self.0 >> 0usize) & 0x0f;
        PmprotLpmode::from_bits(val as u8)
    }
    #[doc = "Low-Power Mode."]
    #[inline(always)]
    pub const fn set_lpmode(&mut self, val: PmprotLpmode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pmprot {
    #[inline(always)]
    fn default() -> Pmprot {
        Pmprot(0)
    }
}
impl core::fmt::Debug for Pmprot {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmprot")
            .field("lpmode", &self.lpmode())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmprot {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pmprot {{ lpmode: {:?}, lock: {=bool:?} }}",
            self.lpmode(),
            self.lock()
        )
    }
}
#[doc = "Reset Pin Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rpc(pub u32);
impl Rpc {
    #[doc = "Reset Filter Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn filtcfg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Reset Filter Configuration."]
    #[inline(always)]
    pub const fn set_filtcfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Filter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn filten(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Filter Enable."]
    #[inline(always)]
    pub const fn set_filten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Low-Power Filter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lpfen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Low-Power Filter Enable."]
    #[inline(always)]
    pub const fn set_lpfen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Rpc {
    #[inline(always)]
    fn default() -> Rpc {
        Rpc(0)
    }
}
impl core::fmt::Debug for Rpc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rpc")
            .field("filtcfg", &self.filtcfg())
            .field("filten", &self.filten())
            .field("lpfen", &self.lpfen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rpc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rpc {{ filtcfg: {=u8:?}, filten: {=bool:?}, lpfen: {=bool:?} }}",
            self.filtcfg(),
            self.filten(),
            self.lpfen()
        )
    }
}
#[doc = "SRAM Shut Down Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramdis0(pub u32);
impl Sramdis0 {
    #[doc = "Shut Down Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dis(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Shut Down Enable."]
    #[inline(always)]
    pub const fn set_dis(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Sramdis0 {
    #[inline(always)]
    fn default() -> Sramdis0 {
        Sramdis0(0)
    }
}
impl core::fmt::Debug for Sramdis0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramdis0")
            .field("dis[0]", &self.dis(0usize))
            .field("dis[1]", &self.dis(1usize))
            .field("dis[2]", &self.dis(2usize))
            .field("dis[3]", &self.dis(3usize))
            .field("dis[4]", &self.dis(4usize))
            .field("dis[5]", &self.dis(5usize))
            .field("dis[6]", &self.dis(6usize))
            .field("dis[7]", &self.dis(7usize))
            .field("dis[8]", &self.dis(8usize))
            .field("dis[9]", &self.dis(9usize))
            .field("dis[10]", &self.dis(10usize))
            .field("dis[11]", &self.dis(11usize))
            .field("dis[12]", &self.dis(12usize))
            .field("dis[13]", &self.dis(13usize))
            .field("dis[14]", &self.dis(14usize))
            .field("dis[15]", &self.dis(15usize))
            .field("dis[16]", &self.dis(16usize))
            .field("dis[17]", &self.dis(17usize))
            .field("dis[18]", &self.dis(18usize))
            .field("dis[19]", &self.dis(19usize))
            .field("dis[20]", &self.dis(20usize))
            .field("dis[21]", &self.dis(21usize))
            .field("dis[22]", &self.dis(22usize))
            .field("dis[23]", &self.dis(23usize))
            .field("dis[24]", &self.dis(24usize))
            .field("dis[25]", &self.dis(25usize))
            .field("dis[26]", &self.dis(26usize))
            .field("dis[27]", &self.dis(27usize))
            .field("dis[28]", &self.dis(28usize))
            .field("dis[29]", &self.dis(29usize))
            .field("dis[30]", &self.dis(30usize))
            .field("dis[31]", &self.dis(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramdis0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sramdis0 {{ dis[0]: {=bool:?}, dis[1]: {=bool:?}, dis[2]: {=bool:?}, dis[3]: {=bool:?}, dis[4]: {=bool:?}, dis[5]: {=bool:?}, dis[6]: {=bool:?}, dis[7]: {=bool:?}, dis[8]: {=bool:?}, dis[9]: {=bool:?}, dis[10]: {=bool:?}, dis[11]: {=bool:?}, dis[12]: {=bool:?}, dis[13]: {=bool:?}, dis[14]: {=bool:?}, dis[15]: {=bool:?}, dis[16]: {=bool:?}, dis[17]: {=bool:?}, dis[18]: {=bool:?}, dis[19]: {=bool:?}, dis[20]: {=bool:?}, dis[21]: {=bool:?}, dis[22]: {=bool:?}, dis[23]: {=bool:?}, dis[24]: {=bool:?}, dis[25]: {=bool:?}, dis[26]: {=bool:?}, dis[27]: {=bool:?}, dis[28]: {=bool:?}, dis[29]: {=bool:?}, dis[30]: {=bool:?}, dis[31]: {=bool:?} }}",
            self.dis(0usize),
            self.dis(1usize),
            self.dis(2usize),
            self.dis(3usize),
            self.dis(4usize),
            self.dis(5usize),
            self.dis(6usize),
            self.dis(7usize),
            self.dis(8usize),
            self.dis(9usize),
            self.dis(10usize),
            self.dis(11usize),
            self.dis(12usize),
            self.dis(13usize),
            self.dis(14usize),
            self.dis(15usize),
            self.dis(16usize),
            self.dis(17usize),
            self.dis(18usize),
            self.dis(19usize),
            self.dis(20usize),
            self.dis(21usize),
            self.dis(22usize),
            self.dis(23usize),
            self.dis(24usize),
            self.dis(25usize),
            self.dis(26usize),
            self.dis(27usize),
            self.dis(28usize),
            self.dis(29usize),
            self.dis(30usize),
            self.dis(31usize)
        )
    }
}
#[doc = "SRAM Deep Sleep Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramret0(pub u32);
impl Sramret0 {
    #[doc = "Deep Sleep Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ret(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Deep Sleep Enable."]
    #[inline(always)]
    pub const fn set_ret(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Sramret0 {
    #[inline(always)]
    fn default() -> Sramret0 {
        Sramret0(0)
    }
}
impl core::fmt::Debug for Sramret0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramret0")
            .field("ret[0]", &self.ret(0usize))
            .field("ret[1]", &self.ret(1usize))
            .field("ret[2]", &self.ret(2usize))
            .field("ret[3]", &self.ret(3usize))
            .field("ret[4]", &self.ret(4usize))
            .field("ret[5]", &self.ret(5usize))
            .field("ret[6]", &self.ret(6usize))
            .field("ret[7]", &self.ret(7usize))
            .field("ret[8]", &self.ret(8usize))
            .field("ret[9]", &self.ret(9usize))
            .field("ret[10]", &self.ret(10usize))
            .field("ret[11]", &self.ret(11usize))
            .field("ret[12]", &self.ret(12usize))
            .field("ret[13]", &self.ret(13usize))
            .field("ret[14]", &self.ret(14usize))
            .field("ret[15]", &self.ret(15usize))
            .field("ret[16]", &self.ret(16usize))
            .field("ret[17]", &self.ret(17usize))
            .field("ret[18]", &self.ret(18usize))
            .field("ret[19]", &self.ret(19usize))
            .field("ret[20]", &self.ret(20usize))
            .field("ret[21]", &self.ret(21usize))
            .field("ret[22]", &self.ret(22usize))
            .field("ret[23]", &self.ret(23usize))
            .field("ret[24]", &self.ret(24usize))
            .field("ret[25]", &self.ret(25usize))
            .field("ret[26]", &self.ret(26usize))
            .field("ret[27]", &self.ret(27usize))
            .field("ret[28]", &self.ret(28usize))
            .field("ret[29]", &self.ret(29usize))
            .field("ret[30]", &self.ret(30usize))
            .field("ret[31]", &self.ret(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramret0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sramret0 {{ ret[0]: {=bool:?}, ret[1]: {=bool:?}, ret[2]: {=bool:?}, ret[3]: {=bool:?}, ret[4]: {=bool:?}, ret[5]: {=bool:?}, ret[6]: {=bool:?}, ret[7]: {=bool:?}, ret[8]: {=bool:?}, ret[9]: {=bool:?}, ret[10]: {=bool:?}, ret[11]: {=bool:?}, ret[12]: {=bool:?}, ret[13]: {=bool:?}, ret[14]: {=bool:?}, ret[15]: {=bool:?}, ret[16]: {=bool:?}, ret[17]: {=bool:?}, ret[18]: {=bool:?}, ret[19]: {=bool:?}, ret[20]: {=bool:?}, ret[21]: {=bool:?}, ret[22]: {=bool:?}, ret[23]: {=bool:?}, ret[24]: {=bool:?}, ret[25]: {=bool:?}, ret[26]: {=bool:?}, ret[27]: {=bool:?}, ret[28]: {=bool:?}, ret[29]: {=bool:?}, ret[30]: {=bool:?}, ret[31]: {=bool:?} }}",
            self.ret(0usize),
            self.ret(1usize),
            self.ret(2usize),
            self.ret(3usize),
            self.ret(4usize),
            self.ret(5usize),
            self.ret(6usize),
            self.ret(7usize),
            self.ret(8usize),
            self.ret(9usize),
            self.ret(10usize),
            self.ret(11usize),
            self.ret(12usize),
            self.ret(13usize),
            self.ret(14usize),
            self.ret(15usize),
            self.ret(16usize),
            self.ret(17usize),
            self.ret(18usize),
            self.ret(19usize),
            self.ret(20usize),
            self.ret(21usize),
            self.ret(22usize),
            self.ret(23usize),
            self.ret(24usize),
            self.ret(25usize),
            self.ret(26usize),
            self.ret(27usize),
            self.ret(28usize),
            self.ret(29usize),
            self.ret(30usize),
            self.ret(31usize)
        )
    }
}
#[doc = "System Reset Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srie(pub u32);
impl Srie {
    #[doc = "Pin Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn pin(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pin Reset."]
    #[inline(always)]
    pub const fn set_pin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DAP Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn dap(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DAP Reset."]
    #[inline(always)]
    pub const fn set_dap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Low Power Acknowledge Timeout Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lpack(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Acknowledge Timeout Reset."]
    #[inline(always)]
    pub const fn set_lpack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "System Clock Generation Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn scg(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "System Clock Generation Reset."]
    #[inline(always)]
    pub const fn set_scg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Watchdog 0 Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog 0 Reset."]
    #[inline(always)]
    pub const fn set_wdog0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn sw(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_sw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Lockup Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lockup(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lockup Reset."]
    #[inline(always)]
    pub const fn set_lockup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Watchdog 1 Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog 1 Reset."]
    #[inline(always)]
    pub const fn set_wdog1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Code Watchdog 0 Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 0 Reset."]
    #[inline(always)]
    pub const fn set_cdog0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Code Watchdog 1 Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 1 Reset."]
    #[inline(always)]
    pub const fn set_cdog1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Srie {
    #[inline(always)]
    fn default() -> Srie {
        Srie(0)
    }
}
impl core::fmt::Debug for Srie {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srie")
            .field("pin", &self.pin())
            .field("dap", &self.dap())
            .field("lpack", &self.lpack())
            .field("scg", &self.scg())
            .field("wdog0", &self.wdog0())
            .field("sw", &self.sw())
            .field("lockup", &self.lockup())
            .field("wdog1", &self.wdog1())
            .field("cdog0", &self.cdog0())
            .field("cdog1", &self.cdog1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srie {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Srie {{ pin: {=bool:?}, dap: {=bool:?}, lpack: {=bool:?}, scg: {=bool:?}, wdog0: {=bool:?}, sw: {=bool:?}, lockup: {=bool:?}, wdog1: {=bool:?}, cdog0: {=bool:?}, cdog1: {=bool:?} }}",
            self.pin(),
            self.dap(),
            self.lpack(),
            self.scg(),
            self.wdog0(),
            self.sw(),
            self.lockup(),
            self.wdog1(),
            self.cdog0(),
            self.cdog1()
        )
    }
}
#[doc = "System Reset Interrupt Flag."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srif(pub u32);
impl Srif {
    #[doc = "Pin Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn pin(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pin Reset."]
    #[inline(always)]
    pub const fn set_pin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DAP Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn dap(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DAP Reset."]
    #[inline(always)]
    pub const fn set_dap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Low Power Acknowledge Timeout Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lpack(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Acknowledge Timeout Reset."]
    #[inline(always)]
    pub const fn set_lpack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Watchdog 0 Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog 0 Reset."]
    #[inline(always)]
    pub const fn set_wdog0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn sw(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_sw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Lockup Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lockup(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lockup Reset."]
    #[inline(always)]
    pub const fn set_lockup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Watchdog 1 Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog 1 Reset."]
    #[inline(always)]
    pub const fn set_wdog1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Code Watchdog 0 Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 0 Reset."]
    #[inline(always)]
    pub const fn set_cdog0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Code Watchdog 1 Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 1 Reset."]
    #[inline(always)]
    pub const fn set_cdog1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Srif {
    #[inline(always)]
    fn default() -> Srif {
        Srif(0)
    }
}
impl core::fmt::Debug for Srif {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srif")
            .field("pin", &self.pin())
            .field("dap", &self.dap())
            .field("lpack", &self.lpack())
            .field("wdog0", &self.wdog0())
            .field("sw", &self.sw())
            .field("lockup", &self.lockup())
            .field("wdog1", &self.wdog1())
            .field("cdog0", &self.cdog0())
            .field("cdog1", &self.cdog1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srif {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Srif {{ pin: {=bool:?}, dap: {=bool:?}, lpack: {=bool:?}, wdog0: {=bool:?}, sw: {=bool:?}, lockup: {=bool:?}, wdog1: {=bool:?}, cdog0: {=bool:?}, cdog1: {=bool:?} }}",
            self.pin(),
            self.dap(),
            self.lpack(),
            self.wdog0(),
            self.sw(),
            self.lockup(),
            self.wdog1(),
            self.cdog0(),
            self.cdog1()
        )
    }
}
#[doc = "System Reset Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srs(pub u32);
impl Srs {
    #[doc = "Wake-up Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Reset."]
    #[inline(always)]
    pub const fn set_wakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Power-on Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn por(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Power-on Reset."]
    #[inline(always)]
    pub const fn set_por(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Low Voltage Detect Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lvd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Low Voltage Detect Reset."]
    #[inline(always)]
    pub const fn set_lvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "High Voltage Detect Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn hvd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "High Voltage Detect Reset."]
    #[inline(always)]
    pub const fn set_hvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Warm Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn warm(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Warm Reset."]
    #[inline(always)]
    pub const fn set_warm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Fatal Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn fatal(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Fatal Reset."]
    #[inline(always)]
    pub const fn set_fatal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Pin Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn pin(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pin Reset."]
    #[inline(always)]
    pub const fn set_pin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Debug Access Port Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn dap(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Access Port Reset."]
    #[inline(always)]
    pub const fn set_dap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Reset Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn rstack(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Timeout."]
    #[inline(always)]
    pub const fn set_rstack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Low Power Acknowledge Timeout Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lpack(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Acknowledge Timeout Reset."]
    #[inline(always)]
    pub const fn set_lpack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "System Clock Generation Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn scg(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "System Clock Generation Reset."]
    #[inline(always)]
    pub const fn set_scg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Watchdog 0 Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog 0 Reset."]
    #[inline(always)]
    pub const fn set_wdog0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn sw(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_sw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Lockup Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lockup(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lockup Reset."]
    #[inline(always)]
    pub const fn set_lockup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Watchdog 1 Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog 1 Reset."]
    #[inline(always)]
    pub const fn set_wdog1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Code Watchdog 0 Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 0 Reset."]
    #[inline(always)]
    pub const fn set_cdog0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Code Watchdog 1 Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 1 Reset."]
    #[inline(always)]
    pub const fn set_cdog1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "JTAG System Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn jtag(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "JTAG System Reset."]
    #[inline(always)]
    pub const fn set_jtag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Security Violation Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn secvio(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation Reset."]
    #[inline(always)]
    pub const fn set_secvio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Tamper Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn tamper(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Reset."]
    #[inline(always)]
    pub const fn set_tamper(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Srs {
    #[inline(always)]
    fn default() -> Srs {
        Srs(0)
    }
}
impl core::fmt::Debug for Srs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srs")
            .field("wakeup", &self.wakeup())
            .field("por", &self.por())
            .field("lvd", &self.lvd())
            .field("hvd", &self.hvd())
            .field("warm", &self.warm())
            .field("fatal", &self.fatal())
            .field("pin", &self.pin())
            .field("dap", &self.dap())
            .field("rstack", &self.rstack())
            .field("lpack", &self.lpack())
            .field("scg", &self.scg())
            .field("wdog0", &self.wdog0())
            .field("sw", &self.sw())
            .field("lockup", &self.lockup())
            .field("wdog1", &self.wdog1())
            .field("cdog0", &self.cdog0())
            .field("cdog1", &self.cdog1())
            .field("jtag", &self.jtag())
            .field("secvio", &self.secvio())
            .field("tamper", &self.tamper())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Srs {{ wakeup: {=bool:?}, por: {=bool:?}, lvd: {=bool:?}, hvd: {=bool:?}, warm: {=bool:?}, fatal: {=bool:?}, pin: {=bool:?}, dap: {=bool:?}, rstack: {=bool:?}, lpack: {=bool:?}, scg: {=bool:?}, wdog0: {=bool:?}, sw: {=bool:?}, lockup: {=bool:?}, wdog1: {=bool:?}, cdog0: {=bool:?}, cdog1: {=bool:?}, jtag: {=bool:?}, secvio: {=bool:?}, tamper: {=bool:?} }}",
            self.wakeup(),
            self.por(),
            self.lvd(),
            self.hvd(),
            self.warm(),
            self.fatal(),
            self.pin(),
            self.dap(),
            self.rstack(),
            self.lpack(),
            self.scg(),
            self.wdog0(),
            self.sw(),
            self.lockup(),
            self.wdog1(),
            self.cdog0(),
            self.cdog1(),
            self.jtag(),
            self.secvio(),
            self.tamper()
        )
    }
}
#[doc = "Sticky System Reset Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssrs(pub u32);
impl Ssrs {
    #[doc = "Wake-up Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Reset."]
    #[inline(always)]
    pub const fn set_wakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Power-on Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn por(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Power-on Reset."]
    #[inline(always)]
    pub const fn set_por(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Low Voltage Detect Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lvd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Low Voltage Detect Reset."]
    #[inline(always)]
    pub const fn set_lvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "High Voltage Detect Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn hvd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "High Voltage Detect Reset."]
    #[inline(always)]
    pub const fn set_hvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Warm Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn warm(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Warm Reset."]
    #[inline(always)]
    pub const fn set_warm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Fatal Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn fatal(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Fatal Reset."]
    #[inline(always)]
    pub const fn set_fatal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Pin Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn pin(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pin Reset."]
    #[inline(always)]
    pub const fn set_pin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DAP Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn dap(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DAP Reset."]
    #[inline(always)]
    pub const fn set_dap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Reset Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn rstack(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Timeout."]
    #[inline(always)]
    pub const fn set_rstack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Low Power Acknowledge Timeout Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lpack(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Acknowledge Timeout Reset."]
    #[inline(always)]
    pub const fn set_lpack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "System Clock Generation Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn scg(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "System Clock Generation Reset."]
    #[inline(always)]
    pub const fn set_scg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Watchdog 0 Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog 0 Reset."]
    #[inline(always)]
    pub const fn set_wdog0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn sw(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_sw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Lockup Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lockup(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lockup Reset."]
    #[inline(always)]
    pub const fn set_lockup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Watchdog 1 Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog 1 Reset."]
    #[inline(always)]
    pub const fn set_wdog1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Code Watchdog 0 Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 0 Reset."]
    #[inline(always)]
    pub const fn set_cdog0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Code Watchdog 1 Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 1 Reset."]
    #[inline(always)]
    pub const fn set_cdog1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "JTAG System Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn jtag(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "JTAG System Reset."]
    #[inline(always)]
    pub const fn set_jtag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Security Violation Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn secvio(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation Reset."]
    #[inline(always)]
    pub const fn set_secvio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Tamper Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn tamper(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Reset."]
    #[inline(always)]
    pub const fn set_tamper(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ssrs {
    #[inline(always)]
    fn default() -> Ssrs {
        Ssrs(0)
    }
}
impl core::fmt::Debug for Ssrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssrs")
            .field("wakeup", &self.wakeup())
            .field("por", &self.por())
            .field("lvd", &self.lvd())
            .field("hvd", &self.hvd())
            .field("warm", &self.warm())
            .field("fatal", &self.fatal())
            .field("pin", &self.pin())
            .field("dap", &self.dap())
            .field("rstack", &self.rstack())
            .field("lpack", &self.lpack())
            .field("scg", &self.scg())
            .field("wdog0", &self.wdog0())
            .field("sw", &self.sw())
            .field("lockup", &self.lockup())
            .field("wdog1", &self.wdog1())
            .field("cdog0", &self.cdog0())
            .field("cdog1", &self.cdog1())
            .field("jtag", &self.jtag())
            .field("secvio", &self.secvio())
            .field("tamper", &self.tamper())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssrs {{ wakeup: {=bool:?}, por: {=bool:?}, lvd: {=bool:?}, hvd: {=bool:?}, warm: {=bool:?}, fatal: {=bool:?}, pin: {=bool:?}, dap: {=bool:?}, rstack: {=bool:?}, lpack: {=bool:?}, scg: {=bool:?}, wdog0: {=bool:?}, sw: {=bool:?}, lockup: {=bool:?}, wdog1: {=bool:?}, cdog0: {=bool:?}, cdog1: {=bool:?}, jtag: {=bool:?}, secvio: {=bool:?}, tamper: {=bool:?} }}",
            self.wakeup(),
            self.por(),
            self.lvd(),
            self.hvd(),
            self.warm(),
            self.fatal(),
            self.pin(),
            self.dap(),
            self.rstack(),
            self.lpack(),
            self.scg(),
            self.wdog0(),
            self.sw(),
            self.lockup(),
            self.wdog1(),
            self.cdog0(),
            self.cdog1(),
            self.jtag(),
            self.secvio(),
            self.tamper()
        )
    }
}
#[doc = "Test Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Test(pub u32);
impl Test {
    #[doc = "Observe."]
    #[must_use]
    #[inline(always)]
    pub const fn observe(&self) -> Observe {
        let val = (self.0 >> 0usize) & 0x0f;
        Observe::from_bits(val as u8)
    }
    #[doc = "Observe."]
    #[inline(always)]
    pub const fn set_observe(&mut self, val: Observe) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Test {
    #[inline(always)]
    fn default() -> Test {
        Test(0)
    }
}
impl core::fmt::Debug for Test {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Test")
            .field("observe", &self.observe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Test {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Test {{ observe: {:?} }}", self.observe())
    }
}
#[doc = "Unlock Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Unlock(pub u32);
impl Unlock {
    #[doc = "Allow Writes."]
    #[must_use]
    #[inline(always)]
    pub const fn alwr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Writes."]
    #[inline(always)]
    pub const fn set_alwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Value."]
    #[must_use]
    #[inline(always)]
    pub const fn value(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Value."]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Unlock {
    #[inline(always)]
    fn default() -> Unlock {
        Unlock(0)
    }
}
impl core::fmt::Debug for Unlock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Unlock")
            .field("alwr", &self.alwr())
            .field("value", &self.value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Unlock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Unlock {{ alwr: {=bool:?}, value: {=u16:?} }}",
            self.alwr(),
            self.value()
        )
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ckmode {
    #[doc = "Core clock is on."]
    Ckmode0000 = 0x0,
    #[doc = "Core clock is off."]
    Ckmode0001 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Core and platform clocks are off."]
    Ckmode0011 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Core, platform, and peripheral clocks are off, but no change in Low-Power mode."]
    Ckmode0111 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Core, platform, and peripheral clocks are off, and core enters Low-Power mode."]
    Ckmode1111 = 0x0f,
}
impl Ckmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ckmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ckmode {
    #[inline(always)]
    fn from(val: u8) -> Ckmode {
        Ckmode::from_bits(val)
    }
}
impl From<Ckmode> for u8 {
    #[inline(always)]
    fn from(val: Ckmode) -> u8 {
        Ckmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Observe {
    #[doc = "Core Active (not sleeping, halted or in reset)."]
    Test0 = 0x0,
    #[doc = "Core clock gated."]
    Test1 = 0x01,
    #[doc = "Bus master clock gated."]
    Test10 = 0x02,
    #[doc = "Bus slave clock gated."]
    Test11 = 0x03,
    #[doc = "Peripherals clock gated."]
    Test100 = 0x04,
    #[doc = "Flash clock gated."]
    Test101 = 0x05,
    #[doc = "All clocks gated."]
    Test110 = 0x06,
    #[doc = "CMC clock gated."]
    Test111 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Observe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Observe {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Observe {
    #[inline(always)]
    fn from(val: u8) -> Observe {
        Observe::from_bits(val)
    }
}
impl From<Observe> for u8 {
    #[inline(always)]
    fn from(val: Observe) -> u8 {
        Observe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmctrlmainLpmode {
    #[doc = "Active."]
    Lpmode0000 = 0x0,
    #[doc = "Sleep."]
    Lpmode0001 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Deep Sleep."]
    Lpmode0011 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Power Down."]
    Lpmode0111 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Deep-Power Down."]
    Lpmode1111 = 0x0f,
}
impl PmctrlmainLpmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmctrlmainLpmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmctrlmainLpmode {
    #[inline(always)]
    fn from(val: u8) -> PmctrlmainLpmode {
        PmctrlmainLpmode::from_bits(val)
    }
}
impl From<PmctrlmainLpmode> for u8 {
    #[inline(always)]
    fn from(val: PmctrlmainLpmode) -> u8 {
        PmctrlmainLpmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmprotLpmode {
    #[doc = "Not allowed."]
    Disabled = 0x0,
    #[doc = "Allowed."]
    En = 0x01,
    #[doc = "Allowed."]
    En1 = 0x02,
    #[doc = "Allowed."]
    En2 = 0x03,
    #[doc = "Allowed."]
    En3 = 0x04,
    #[doc = "Allowed."]
    En4 = 0x05,
    #[doc = "Allowed."]
    En5 = 0x06,
    #[doc = "Allowed."]
    En6 = 0x07,
    #[doc = "Allowed."]
    En7 = 0x08,
    #[doc = "Allowed."]
    En8 = 0x09,
    #[doc = "Allowed."]
    En9 = 0x0a,
    #[doc = "Allowed."]
    En10 = 0x0b,
    #[doc = "Allowed."]
    En11 = 0x0c,
    #[doc = "Allowed."]
    En12 = 0x0d,
    #[doc = "Allowed."]
    En13 = 0x0e,
    #[doc = "Allowed."]
    En14 = 0x0f,
}
impl PmprotLpmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmprotLpmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmprotLpmode {
    #[inline(always)]
    fn from(val: u8) -> PmprotLpmode {
        PmprotLpmode::from_bits(val)
    }
}
impl From<PmprotLpmode> for u8 {
    #[inline(always)]
    fn from(val: PmprotLpmode) -> u8 {
        PmprotLpmode::to_bits(val)
    }
}
