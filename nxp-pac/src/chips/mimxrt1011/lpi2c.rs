#[doc = "LPI2C"]
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
    #[doc = "Version ID"]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<regs::Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameter"]
    #[inline(always)]
    pub const fn param(self) -> crate::common::Reg<regs::Param, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Master Control"]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Master Status"]
    #[inline(always)]
    pub const fn msr(self) -> crate::common::Reg<regs::Msr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Master Interrupt Enable"]
    #[inline(always)]
    pub const fn mier(self) -> crate::common::Reg<regs::Mier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Master DMA Enable"]
    #[inline(always)]
    pub const fn mder(self) -> crate::common::Reg<regs::Mder, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Master Configuration 0"]
    #[inline(always)]
    pub const fn mcfgr0(self) -> crate::common::Reg<regs::Mcfgr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Master Configuration 1"]
    #[inline(always)]
    pub const fn mcfgr1(self) -> crate::common::Reg<regs::Mcfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Master Configuration 2"]
    #[inline(always)]
    pub const fn mcfgr2(self) -> crate::common::Reg<regs::Mcfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Master Configuration 3"]
    #[inline(always)]
    pub const fn mcfgr3(self) -> crate::common::Reg<regs::Mcfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Master Data Match"]
    #[inline(always)]
    pub const fn mdmr(self) -> crate::common::Reg<regs::Mdmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Master Clock Configuration 0"]
    #[inline(always)]
    pub const fn mccr0(self) -> crate::common::Reg<regs::Mccr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Master Clock Configuration 1"]
    #[inline(always)]
    pub const fn mccr1(self) -> crate::common::Reg<regs::Mccr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Master FIFO Control"]
    #[inline(always)]
    pub const fn mfcr(self) -> crate::common::Reg<regs::Mfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Master FIFO Status"]
    #[inline(always)]
    pub const fn mfsr(self) -> crate::common::Reg<regs::Mfsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Master Transmit Data"]
    #[inline(always)]
    pub const fn mtdr(self) -> crate::common::Reg<regs::Mtdr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Master Receive Data"]
    #[inline(always)]
    pub const fn mrdr(self) -> crate::common::Reg<regs::Mrdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Slave Control"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Slave Status"]
    #[inline(always)]
    pub const fn ssr(self) -> crate::common::Reg<regs::Ssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Slave Interrupt Enable"]
    #[inline(always)]
    pub const fn sier(self) -> crate::common::Reg<regs::Sier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Slave DMA Enable"]
    #[inline(always)]
    pub const fn sder(self) -> crate::common::Reg<regs::Sder, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Slave Configuration 1"]
    #[inline(always)]
    pub const fn scfgr1(self) -> crate::common::Reg<regs::Scfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Slave Configuration 2"]
    #[inline(always)]
    pub const fn scfgr2(self) -> crate::common::Reg<regs::Scfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Slave Address Match"]
    #[inline(always)]
    pub const fn samr(self) -> crate::common::Reg<regs::Samr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Slave Address Status"]
    #[inline(always)]
    pub const fn sasr(self) -> crate::common::Reg<regs::Sasr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Slave Transmit ACK"]
    #[inline(always)]
    pub const fn star(self) -> crate::common::Reg<regs::Star, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Slave Transmit Data"]
    #[inline(always)]
    pub const fn stdr(self) -> crate::common::Reg<regs::Stdr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Slave Receive Data"]
    #[inline(always)]
    pub const fn srdr(self) -> crate::common::Reg<regs::Srdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
}
pub mod regs;
pub mod vals;
