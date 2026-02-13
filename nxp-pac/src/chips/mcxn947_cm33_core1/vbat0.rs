#[doc = "VBAT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbat0 {
    ptr: *mut u8,
}
unsafe impl Send for Vbat0 {}
unsafe impl Sync for Vbat0 {}
impl Vbat0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID"]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<regs::Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Status A"]
    #[inline(always)]
    pub const fn statusa(self) -> crate::common::Reg<regs::Statusa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status B"]
    #[inline(always)]
    pub const fn statusb(self) -> crate::common::Reg<regs::Statusb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt Enable A"]
    #[inline(always)]
    pub const fn irqena(self) -> crate::common::Reg<regs::Irqena, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Interrupt Enable B"]
    #[inline(always)]
    pub const fn irqenb(self) -> crate::common::Reg<regs::Irqenb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Wake-up Enable A"]
    #[inline(always)]
    pub const fn wakena(self) -> crate::common::Reg<regs::Wakena, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Wake-up Enable B"]
    #[inline(always)]
    pub const fn wakenb(self) -> crate::common::Reg<regs::Wakenb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Tamper Enable A"]
    #[inline(always)]
    pub const fn tampera(self) -> crate::common::Reg<regs::Tampera, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Tamper Enable B"]
    #[inline(always)]
    pub const fn tamperb(self) -> crate::common::Reg<regs::Tamperb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Lock A"]
    #[inline(always)]
    pub const fn locka(self) -> crate::common::Reg<regs::Locka, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Lock B"]
    #[inline(always)]
    pub const fn lockb(self) -> crate::common::Reg<regs::Lockb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Wake-up Configuration"]
    #[inline(always)]
    pub const fn wakecfg(self) -> crate::common::Reg<regs::Wakecfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Oscillator Control A"]
    #[inline(always)]
    pub const fn oscctla(self) -> crate::common::Reg<regs::Oscctla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Oscillator Control B"]
    #[inline(always)]
    pub const fn oscctlb(self) -> crate::common::Reg<regs::Oscctlb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Oscillator Configuration A"]
    #[inline(always)]
    pub const fn osccfga(self) -> crate::common::Reg<regs::Osccfga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Oscillator Configuration B"]
    #[inline(always)]
    pub const fn osccfgb(self) -> crate::common::Reg<regs::Osccfgb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Oscillator Lock A"]
    #[inline(always)]
    pub const fn osclcka(self) -> crate::common::Reg<regs::Osclcka, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Oscillator Lock B"]
    #[inline(always)]
    pub const fn osclckb(self) -> crate::common::Reg<regs::Osclckb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Oscillator Clock Enable"]
    #[inline(always)]
    pub const fn oscclke(self) -> crate::common::Reg<regs::Oscclke, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "FRO16K Control A"]
    #[inline(always)]
    pub const fn froctla(self) -> crate::common::Reg<regs::Froctla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "FRO16K Control B"]
    #[inline(always)]
    pub const fn froctlb(self) -> crate::common::Reg<regs::Froctlb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "FRO16K Lock A"]
    #[inline(always)]
    pub const fn frolcka(self) -> crate::common::Reg<regs::Frolcka, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "FRO16K Lock B"]
    #[inline(always)]
    pub const fn frolckb(self) -> crate::common::Reg<regs::Frolckb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "FRO16K Clock Enable"]
    #[inline(always)]
    pub const fn froclke(self) -> crate::common::Reg<regs::Froclke, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "LDO_RAM Control A"]
    #[inline(always)]
    pub const fn ldoctla(self) -> crate::common::Reg<regs::Ldoctla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "LDO_RAM Control B"]
    #[inline(always)]
    pub const fn ldoctlb(self) -> crate::common::Reg<regs::Ldoctlb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "LDO_RAM Lock A"]
    #[inline(always)]
    pub const fn ldolcka(self) -> crate::common::Reg<regs::Ldolcka, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "LDO_RAM Lock B"]
    #[inline(always)]
    pub const fn ldolckb(self) -> crate::common::Reg<regs::Ldolckb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x031cusize) as _) }
    }
    #[doc = "RAM Control"]
    #[inline(always)]
    pub const fn ldoramc(self) -> crate::common::Reg<regs::Ldoramc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "Bandgap Timer 0"]
    #[inline(always)]
    pub const fn ldotimer0(self) -> crate::common::Reg<regs::Ldotimer0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "Bandgap Timer 1"]
    #[inline(always)]
    pub const fn ldotimer1(self) -> crate::common::Reg<regs::Ldotimer1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0338usize) as _) }
    }
    #[doc = "CLKMON Control A"]
    #[inline(always)]
    pub const fn monctla(self) -> crate::common::Reg<regs::Monctla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "CLKMON Control B"]
    #[inline(always)]
    pub const fn monctlb(self) -> crate::common::Reg<regs::Monctlb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "CLKMON Configuration A"]
    #[inline(always)]
    pub const fn moncfga(self) -> crate::common::Reg<regs::Moncfga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "CLKMON Configuration B"]
    #[inline(always)]
    pub const fn moncfgb(self) -> crate::common::Reg<regs::Moncfgb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "CLKMON Lock A"]
    #[inline(always)]
    pub const fn monlcka(self) -> crate::common::Reg<regs::Monlcka, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "CLKMON Lock B"]
    #[inline(always)]
    pub const fn monlckb(self) -> crate::common::Reg<regs::Monlckb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "TAMPER Control A"]
    #[inline(always)]
    pub const fn tamctla(self) -> crate::common::Reg<regs::Tamctla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "TAMPER Control B"]
    #[inline(always)]
    pub const fn tamctlb(self) -> crate::common::Reg<regs::Tamctlb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "TAMPER Lock A"]
    #[inline(always)]
    pub const fn tamlcka(self) -> crate::common::Reg<regs::Tamlcka, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
    }
    #[doc = "TAMPER Lock B"]
    #[inline(always)]
    pub const fn tamlckb(self) -> crate::common::Reg<regs::Tamlckb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize) as _) }
    }
    #[doc = "Switch Control A"]
    #[inline(always)]
    pub const fn swictla(self) -> crate::common::Reg<regs::Swictla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "Switch Control B"]
    #[inline(always)]
    pub const fn swictlb(self) -> crate::common::Reg<regs::Swictlb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "Switch Lock A"]
    #[inline(always)]
    pub const fn swilcka(self) -> crate::common::Reg<regs::Swilcka, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[doc = "Switch Lock B"]
    #[inline(always)]
    pub const fn swilckb(self) -> crate::common::Reg<regs::Swilckb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x061cusize) as _) }
    }
    #[doc = "Array of registers: WAKEUPA, WAKEUPB"]
    #[inline(always)]
    pub const fn wakeup(self, n: usize) -> Wakeup {
        assert!(n < 2usize);
        unsafe { Wakeup::from_ptr(self.ptr.wrapping_add(0x0700usize + n * 8usize) as _) }
    }
    #[doc = "Wakeup Lock A"]
    #[inline(always)]
    pub const fn waklcka(self) -> crate::common::Reg<regs::Waklcka, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f8usize) as _) }
    }
    #[doc = "Wakeup Lock B"]
    #[inline(always)]
    pub const fn waklckb(self) -> crate::common::Reg<regs::Waklckb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07fcusize) as _) }
    }
}
#[doc = "Array of registers: WAKEUPA, WAKEUPB"]
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
    #[doc = "Wakeup 0 Register A"]
    #[inline(always)]
    pub const fn wakeupa(self) -> crate::common::Reg<regs::Wakeupa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Wakeup 0 Register B"]
    #[inline(always)]
    pub const fn wakeupb(self) -> crate::common::Reg<regs::Wakeupb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs;
pub mod vals;
