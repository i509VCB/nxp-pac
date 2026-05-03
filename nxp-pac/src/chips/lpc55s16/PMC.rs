#[doc = "PMC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PMC {
    ptr: *mut u8,
}
unsafe impl Send for PMC {}
unsafe impl Sync for PMC {}
impl PMC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Power Management Controller FSM (Finite State Machines) status."]
    #[inline(always)]
    pub const fn STATUS(self) -> crate::common::Reg<regs::STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
    #[inline(always)]
    pub const fn RESETCTRL(self) -> crate::common::Reg<regs::RESETCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DCDC (first) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
    #[inline(always)]
    pub const fn DCDC0(self) -> crate::common::Reg<regs::DCDC0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DCDC (second) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
    #[inline(always)]
    pub const fn DCDC1(self) -> crate::common::Reg<regs::DCDC1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Power Management Unit (PMU) and Always-On domains LDO control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
    #[inline(always)]
    pub const fn LDOPMU(self) -> crate::common::Reg<regs::LDOPMU, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]."]
    #[inline(always)]
    pub const fn BODVBAT(self) -> crate::common::Reg<regs::BODVBAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Analog References fast wake-up Control register \\[Reset by: PoR\\]."]
    #[inline(always)]
    pub const fn REFFASTWKUP(self) -> crate::common::Reg<regs::REFFASTWKUP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "32 KHz Crystal oscillator (XTAL) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]."]
    #[inline(always)]
    pub const fn XTAL32K(self) -> crate::common::Reg<regs::XTAL32K, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
    #[inline(always)]
    pub const fn COMP(self) -> crate::common::Reg<regs::COMP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Deep Power Down wake-up source \\[Reset by: PoR, Pin Reset, Software Reset\\]."]
    #[inline(always)]
    pub const fn WAKEUPIOCTRL(self) -> crate::common::Reg<regs::WAKEUPIOCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Allows to identify the Wake-up I/O source from Deep Power Down mode."]
    #[inline(always)]
    pub const fn WAKEIOCAUSE(self) -> crate::common::Reg<regs::WAKEIOCAUSE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]."]
    #[inline(always)]
    pub const fn STATUSCLK(self) -> crate::common::Reg<regs::STATUSCLK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]."]
    #[inline(always)]
    pub const fn AOREG1(self) -> crate::common::Reg<regs::AOREG1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Dummy Control bus to PMU \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
    #[inline(always)]
    pub const fn MISCCTRL(self) -> crate::common::Reg<regs::MISCCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]."]
    #[inline(always)]
    pub const fn RTCOSC32K(self) -> crate::common::Reg<regs::RTCOSC32K, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]."]
    #[inline(always)]
    pub const fn OSTIMER(self) -> crate::common::Reg<regs::OSTIMER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
    #[inline(always)]
    pub const fn PDRUNCFG0(self) -> crate::common::Reg<regs::PDRUNCFG0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
    #[inline(always)]
    pub const fn PDRUNCFGSET0(self) -> crate::common::Reg<regs::PDRUNCFGSET0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
    #[inline(always)]
    pub const fn PDRUNCFGCLR0(self) -> crate::common::Reg<regs::PDRUNCFGCLR0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "All SRAMs common control signals \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Software Reset\\]."]
    #[inline(always)]
    pub const fn SRAMCTRL(self) -> crate::common::Reg<regs::SRAMCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
