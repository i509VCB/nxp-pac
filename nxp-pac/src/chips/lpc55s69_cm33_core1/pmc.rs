#[doc = "PMC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmc {
    ptr: *mut u8,
}
unsafe impl Send for Pmc {}
unsafe impl Sync for Pmc {}
impl Pmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Power Management Controller FSM (Finite State Machines) status"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn resetctrl(self) -> crate::common::Reg<regs::Resetctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DCDC (first) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn dcdc0(self) -> crate::common::Reg<regs::Dcdc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DCDC (second) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn dcdc1(self) -> crate::common::Reg<regs::Dcdc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Power Management Unit (PMU) and Always-On domains LDO control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn ldopmu(self) -> crate::common::Reg<regs::Ldopmu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn bodvbat(self) -> crate::common::Reg<regs::Bodvbat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Analog References fast wake-up Control register \\[Reset by: PoR\\]"]
    #[inline(always)]
    pub const fn reffastwkup(self) -> crate::common::Reg<regs::Reffastwkup, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "32 KHz Crystal oscillator (XTAL) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    #[inline(always)]
    pub const fn xtal32k(self) -> crate::common::Reg<regs::Xtal32k, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn comp(self) -> crate::common::Reg<regs::Comp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Deep Power Down wake-up source \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn wakeupioctrl(self) -> crate::common::Reg<regs::Wakeupioctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Allows to identify the Wake-up I/O source from Deep Power Down mode"]
    #[inline(always)]
    pub const fn wakeiocause(self) -> crate::common::Reg<regs::Wakeiocause, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    #[inline(always)]
    pub const fn statusclk(self) -> crate::common::Reg<regs::Statusclk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    #[inline(always)]
    pub const fn aoreg1(self) -> crate::common::Reg<regs::Aoreg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Dummy Control bus to PMU \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn miscctrl(self) -> crate::common::Reg<regs::Miscctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    #[inline(always)]
    pub const fn rtcosc32k(self) -> crate::common::Reg<regs::Rtcosc32k, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    #[inline(always)]
    pub const fn ostimer(self) -> crate::common::Reg<regs::Ostimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn pdruncfg0(self) -> crate::common::Reg<regs::Pdruncfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn pdruncfgset0(self) -> crate::common::Reg<regs::Pdruncfgset0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn pdruncfgclr0(self) -> crate::common::Reg<regs::Pdruncfgclr0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "All SRAMs common control signals \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn sramctrl(self) -> crate::common::Reg<regs::Sramctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
