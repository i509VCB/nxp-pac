#[doc = "PUFCTRL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Puf {
    ptr: *mut u8,
}
unsafe impl Send for Puf {}
unsafe impl Sync for Puf {}
impl Puf {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PUF Control register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "PUF Key Index register"]
    #[inline(always)]
    pub const fn keyindex(self) -> crate::common::Reg<regs::Keyindex, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "PUF Key Size register"]
    #[inline(always)]
    pub const fn keysize(self) -> crate::common::Reg<regs::Keysize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "PUF Status register"]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "PUF Allow register"]
    #[inline(always)]
    pub const fn allow(self) -> crate::common::Reg<regs::Allow, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "PUF Key Input register"]
    #[inline(always)]
    pub const fn keyinput(self) -> crate::common::Reg<regs::Keyinput, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "PUF Code Input register"]
    #[inline(always)]
    pub const fn codeinput(self) -> crate::common::Reg<regs::Codeinput, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "PUF Code Output register"]
    #[inline(always)]
    pub const fn codeoutput(self) -> crate::common::Reg<regs::Codeoutput, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "PUF Key Output Index register"]
    #[inline(always)]
    pub const fn keyoutindex(self) -> crate::common::Reg<regs::Keyoutindex, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "PUF Key Output register"]
    #[inline(always)]
    pub const fn keyoutput(self) -> crate::common::Reg<regs::Keyoutput, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "PUF Interface Status and clear register"]
    #[inline(always)]
    pub const fn ifstat(self) -> crate::common::Reg<regs::Ifstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "PUF version register."]
    #[inline(always)]
    pub const fn version(self) -> crate::common::Reg<regs::Version, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "PUF Interrupt Enable"]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "PUF interrupt status"]
    #[inline(always)]
    pub const fn intstat(self) -> crate::common::Reg<regs::Intstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "PUF RAM Power Control"]
    #[inline(always)]
    pub const fn pwrctrl(self) -> crate::common::Reg<regs::Pwrctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "PUF config register for block bits"]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Only reset in case of full IC reset"]
    #[inline(always)]
    pub const fn keylock(self) -> crate::common::Reg<regs::Keylock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn keyenable(self) -> crate::common::Reg<regs::Keyenable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Reinitialize Keys shift registers counters"]
    #[inline(always)]
    pub const fn keyreset(self) -> crate::common::Reg<regs::Keyreset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn idxblk_l(self) -> crate::common::Reg<regs::IdxblkL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn idxblk_h_dp(self) -> crate::common::Reg<regs::IdxblkHDp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Only reset in case of full IC reset"]
    #[inline(always)]
    pub const fn keymask(self, n: usize) -> crate::common::Reg<regs::Keymask, crate::common::W> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn idxblk_h(self) -> crate::common::Reg<regs::IdxblkH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn idxblk_l_dp(self) -> crate::common::Reg<regs::IdxblkLDp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn shift_status(self) -> crate::common::Reg<regs::ShiftStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
}
pub mod regs;
