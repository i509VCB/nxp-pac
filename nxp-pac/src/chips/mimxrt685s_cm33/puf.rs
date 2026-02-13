#[doc = "PUF Controller"]
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
    #[doc = "PUF Control"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "PUF Key Index"]
    #[inline(always)]
    pub const fn keyindex(self) -> crate::common::Reg<regs::Keyindex, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "PUF Key Size"]
    #[inline(always)]
    pub const fn keysize(self) -> crate::common::Reg<regs::Keysize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "PUF Status"]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "PUF Allow"]
    #[inline(always)]
    pub const fn allow(self) -> crate::common::Reg<regs::Allow, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "PUF Key Input"]
    #[inline(always)]
    pub const fn keyinput(self) -> crate::common::Reg<regs::Keyinput, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "PUF Code Input"]
    #[inline(always)]
    pub const fn codeinput(self) -> crate::common::Reg<regs::Codeinput, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "PUF Code Output"]
    #[inline(always)]
    pub const fn codeoutput(self) -> crate::common::Reg<regs::Codeoutput, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "PUF Key Output Index"]
    #[inline(always)]
    pub const fn keyoutindex(self) -> crate::common::Reg<regs::Keyoutindex, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "PUF Key Output"]
    #[inline(always)]
    pub const fn keyoutput(self) -> crate::common::Reg<regs::Keyoutput, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "PUF Interface Status and Clear"]
    #[inline(always)]
    pub const fn ifstat(self) -> crate::common::Reg<regs::Ifstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "PUF Version"]
    #[inline(always)]
    pub const fn version(self) -> crate::common::Reg<regs::Version, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "PUF Interrupt Enable"]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "PUF Interrupt Status"]
    #[inline(always)]
    pub const fn intstat(self) -> crate::common::Reg<regs::Intstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "PUF Power Control"]
    #[inline(always)]
    pub const fn pwrctrl(self) -> crate::common::Reg<regs::Pwrctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "PUF Configuration"]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Key Lock"]
    #[inline(always)]
    pub const fn keylock(self) -> crate::common::Reg<regs::Keylock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Key Enable"]
    #[inline(always)]
    pub const fn keyenable(self) -> crate::common::Reg<regs::Keyenable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Key Reset"]
    #[inline(always)]
    pub const fn keyreset(self) -> crate::common::Reg<regs::Keyreset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Index Block Low"]
    #[inline(always)]
    pub const fn idxblk_l(self) -> crate::common::Reg<regs::IdxblkL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Index Block High Duplicate"]
    #[inline(always)]
    pub const fn idxblk_h_dp(self) -> crate::common::Reg<regs::IdxblkHDp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Key Mask x"]
    #[inline(always)]
    pub const fn keymask(self, n: usize) -> crate::common::Reg<regs::Keymask, crate::common::W> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize + n * 4usize) as _)
        }
    }
    #[doc = "Index Block High"]
    #[inline(always)]
    pub const fn idxblk_h(self) -> crate::common::Reg<regs::IdxblkH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "Index Block Low Duplicate"]
    #[inline(always)]
    pub const fn idxblk_l_dp(self) -> crate::common::Reg<regs::IdxblkLDp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
}
pub mod regs;
pub mod vals;
