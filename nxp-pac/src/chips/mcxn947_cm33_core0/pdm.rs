#[doc = "MICFIL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdm {
    ptr: *mut u8,
}
unsafe impl Send for Pdm {}
unsafe impl Sync for Pdm {}
impl Pdm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MICFIL Control 1"]
    #[inline(always)]
    pub const fn ctrl_1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MICFIL Control 2"]
    #[inline(always)]
    pub const fn ctrl_2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "MICFIL Status"]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "MICFIL FIFO Control"]
    #[inline(always)]
    pub const fn fifo_ctrl(self) -> crate::common::Reg<regs::FifoCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "MICFIL FIFO Status"]
    #[inline(always)]
    pub const fn fifo_stat(self) -> crate::common::Reg<regs::FifoStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "MICFIL Output Result"]
    #[inline(always)]
    pub const fn datach(self, n: usize) -> crate::common::Reg<regs::Datach, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize + n * 4usize) as _) }
    }
    #[doc = "MICFIL DC Remover Control"]
    #[inline(always)]
    pub const fn dc_ctrl(self) -> crate::common::Reg<regs::DcCtrl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "MICFIL Output DC Remover Control"]
    #[inline(always)]
    pub const fn dc_out_ctrl(self) -> crate::common::Reg<regs::DcOutCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "MICFIL Range Control"]
    #[inline(always)]
    pub const fn range_ctrl(self) -> crate::common::Reg<regs::RangeCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "MICFIL Range Status"]
    #[inline(always)]
    pub const fn range_stat(self) -> crate::common::Reg<regs::RangeStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Frame Synchronization Control"]
    #[inline(always)]
    pub const fn fsync_ctrl(self) -> crate::common::Reg<regs::FsyncCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Version ID"]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<regs::Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Parameter"]
    #[inline(always)]
    pub const fn param(self) -> crate::common::Reg<regs::Param, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
}
pub mod regs;
pub mod vals;
