#[doc = "Low-Power Inter-Integrated Circuit"]
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
    #[doc = "Controller Control"]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Controller Status"]
    #[inline(always)]
    pub const fn msr(self) -> crate::common::Reg<regs::Msr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Controller Interrupt Enable"]
    #[inline(always)]
    pub const fn mier(self) -> crate::common::Reg<regs::Mier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Controller DMA Enable"]
    #[inline(always)]
    pub const fn mder(self) -> crate::common::Reg<regs::Mder, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Controller Configuration 0"]
    #[inline(always)]
    pub const fn mcfgr0(self) -> crate::common::Reg<regs::Mcfgr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Controller Configuration 1"]
    #[inline(always)]
    pub const fn mcfgr1(self) -> crate::common::Reg<regs::Mcfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Controller Configuration 2"]
    #[inline(always)]
    pub const fn mcfgr2(self) -> crate::common::Reg<regs::Mcfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Controller Configuration 3"]
    #[inline(always)]
    pub const fn mcfgr3(self) -> crate::common::Reg<regs::Mcfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Controller Data Match"]
    #[inline(always)]
    pub const fn mdmr(self) -> crate::common::Reg<regs::Mdmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Controller Clock Configuration 0"]
    #[inline(always)]
    pub const fn mccr0(self) -> crate::common::Reg<regs::Mccr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Controller Clock Configuration 1"]
    #[inline(always)]
    pub const fn mccr1(self) -> crate::common::Reg<regs::Mccr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Controller FIFO Control"]
    #[inline(always)]
    pub const fn mfcr(self) -> crate::common::Reg<regs::Mfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Controller FIFO Status"]
    #[inline(always)]
    pub const fn mfsr(self) -> crate::common::Reg<regs::Mfsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Controller Transmit Data"]
    #[inline(always)]
    pub const fn mtdr(self) -> crate::common::Reg<regs::Mtdr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Controller Receive Data"]
    #[inline(always)]
    pub const fn mrdr(self) -> crate::common::Reg<regs::Mrdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Controller Receive Data Read Only"]
    #[inline(always)]
    pub const fn mrdror(self) -> crate::common::Reg<regs::Mrdror, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Target Control"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Target Status"]
    #[inline(always)]
    pub const fn ssr(self) -> crate::common::Reg<regs::Ssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Target Interrupt Enable"]
    #[inline(always)]
    pub const fn sier(self) -> crate::common::Reg<regs::Sier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Target DMA Enable"]
    #[inline(always)]
    pub const fn sder(self) -> crate::common::Reg<regs::Sder, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Target Configuration 0"]
    #[inline(always)]
    pub const fn scfgr0(self) -> crate::common::Reg<regs::Scfgr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Target Configuration 1"]
    #[inline(always)]
    pub const fn scfgr1(self) -> crate::common::Reg<regs::Scfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Target Configuration 2"]
    #[inline(always)]
    pub const fn scfgr2(self) -> crate::common::Reg<regs::Scfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Target Address Match"]
    #[inline(always)]
    pub const fn samr(self) -> crate::common::Reg<regs::Samr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Target Address Status"]
    #[inline(always)]
    pub const fn sasr(self) -> crate::common::Reg<regs::Sasr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Target Transmit ACK"]
    #[inline(always)]
    pub const fn star(self) -> crate::common::Reg<regs::Star, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Target Transmit Data"]
    #[inline(always)]
    pub const fn stdr(self) -> crate::common::Reg<regs::Stdr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Target Receive Data"]
    #[inline(always)]
    pub const fn srdr(self) -> crate::common::Reg<regs::Srdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Target Receive Data Read Only"]
    #[inline(always)]
    pub const fn srdror(self) -> crate::common::Reg<regs::Srdror, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Controller Transmit Command Burst"]
    #[inline(always)]
    pub const fn mtcbr(self, n: usize) -> crate::common::Reg<regs::Mtcbr, crate::common::W> {
        assert!(n < 128usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "Transmit Data Burst"]
    #[inline(always)]
    pub const fn mtdbr(self, n: usize) -> crate::common::Reg<regs::Mtdbr, crate::common::W> {
        assert!(n < 253usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
