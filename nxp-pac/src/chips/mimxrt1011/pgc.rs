#[doc = "PGC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pgc {
    ptr: *mut u8,
}
unsafe impl Send for Pgc {}
unsafe impl Sync for Pgc {}
impl Pgc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PGC Mega Control Register"]
    #[inline(always)]
    pub const fn mega_ctrl(self) -> crate::common::Reg<regs::MegaCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "PGC Mega Power Up Sequence Control Register"]
    #[inline(always)]
    pub const fn mega_pupscr(self) -> crate::common::Reg<regs::MegaPupscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "PGC Mega Pull Down Sequence Control Register"]
    #[inline(always)]
    pub const fn mega_pdnscr(self) -> crate::common::Reg<regs::MegaPdnscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "PGC Mega Power Gating Controller Status Register"]
    #[inline(always)]
    pub const fn mega_sr(self) -> crate::common::Reg<regs::MegaSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "PGC CPU Control Register"]
    #[inline(always)]
    pub const fn cpu_ctrl(self) -> crate::common::Reg<regs::CpuCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "PGC CPU Power Up Sequence Control Register"]
    #[inline(always)]
    pub const fn cpu_pupscr(self) -> crate::common::Reg<regs::CpuPupscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "PGC CPU Pull Down Sequence Control Register"]
    #[inline(always)]
    pub const fn cpu_pdnscr(self) -> crate::common::Reg<regs::CpuPdnscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "PGC CPU Power Gating Controller Status Register"]
    #[inline(always)]
    pub const fn cpu_sr(self) -> crate::common::Reg<regs::CpuSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
}
pub mod regs;
pub mod vals;
