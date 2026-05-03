#[doc = "PUFCTRL."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PUF {
    ptr: *mut u8,
}
unsafe impl Send for PUF {}
unsafe impl Sync for PUF {}
impl PUF {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PUF Control register."]
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "PUF Key Index register."]
    #[inline(always)]
    pub const fn KEYINDEX(self) -> crate::common::Reg<regs::KEYINDEX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "PUF Key Size register."]
    #[inline(always)]
    pub const fn KEYSIZE(self) -> crate::common::Reg<regs::KEYSIZE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "PUF Status register."]
    #[inline(always)]
    pub const fn STAT(self) -> crate::common::Reg<regs::STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "PUF Allow register."]
    #[inline(always)]
    pub const fn ALLOW(self) -> crate::common::Reg<regs::ALLOW, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "PUF Key Input register."]
    #[inline(always)]
    pub const fn KEYINPUT(self) -> crate::common::Reg<regs::KEYINPUT, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "PUF Code Input register."]
    #[inline(always)]
    pub const fn CODEINPUT(self) -> crate::common::Reg<regs::CODEINPUT, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "PUF Code Output register."]
    #[inline(always)]
    pub const fn CODEOUTPUT(self) -> crate::common::Reg<regs::CODEOUTPUT, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "PUF Key Output Index register."]
    #[inline(always)]
    pub const fn KEYOUTINDEX(self) -> crate::common::Reg<regs::KEYOUTINDEX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "PUF Key Output register."]
    #[inline(always)]
    pub const fn KEYOUTPUT(self) -> crate::common::Reg<regs::KEYOUTPUT, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "PUF Interface Status and clear register."]
    #[inline(always)]
    pub const fn IFSTAT(self) -> crate::common::Reg<regs::IFSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "PUF version register."]
    #[inline(always)]
    pub const fn VERSION(self) -> crate::common::Reg<regs::VERSION, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "PUF Interrupt Enable."]
    #[inline(always)]
    pub const fn INTEN(self) -> crate::common::Reg<regs::INTEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "PUF interrupt status."]
    #[inline(always)]
    pub const fn INTSTAT(self) -> crate::common::Reg<regs::INTSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "PUF RAM Power Control."]
    #[inline(always)]
    pub const fn PWRCTRL(self) -> crate::common::Reg<regs::PWRCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "PUF config register for block bits."]
    #[inline(always)]
    pub const fn CFG(self) -> crate::common::Reg<regs::CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Only reset in case of full IC reset."]
    #[inline(always)]
    pub const fn KEYLOCK(self) -> crate::common::Reg<regs::KEYLOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn KEYENABLE(self) -> crate::common::Reg<regs::KEYENABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Reinitialize Keys shift registers counters."]
    #[inline(always)]
    pub const fn KEYRESET(self) -> crate::common::Reg<regs::KEYRESET, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn IDXBLK_L(self) -> crate::common::Reg<regs::IDXBLK_L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn IDXBLK_H_DP(self) -> crate::common::Reg<regs::IDXBLK_H_DP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Only reset in case of full IC reset."]
    #[inline(always)]
    pub const fn KEYMASK(self, n: usize) -> crate::common::Reg<regs::KEYMASK, crate::common::W> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn IDXBLK_H(self) -> crate::common::Reg<regs::IDXBLK_H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn IDXBLK_L_DP(self) -> crate::common::Reg<regs::IDXBLK_L_DP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn SHIFT_STATUS(self) -> crate::common::Reg<regs::SHIFT_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
}
pub mod regs;
