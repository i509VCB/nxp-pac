#[doc = "FLEXCAN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can {
    ptr: *mut u8,
}
unsafe impl Send for Can {}
unsafe impl Sync for Can {}
impl Can {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Module Configuration Register"]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control 1 Register"]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Free Running Timer Register"]
    #[inline(always)]
    pub const fn timer(self) -> crate::common::Reg<regs::Timer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Rx Mailboxes Global Mask Register"]
    #[inline(always)]
    pub const fn rxmgmask(self) -> crate::common::Reg<regs::Rxmgmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Rx Buffer 14 Mask Register"]
    #[inline(always)]
    pub const fn rx14mask(self) -> crate::common::Reg<regs::Rx14mask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Rx Buffer 15 Mask Register"]
    #[inline(always)]
    pub const fn rx15mask(self) -> crate::common::Reg<regs::Rx15mask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Error Counter Register"]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<regs::Ecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Error and Status 1 Register"]
    #[inline(always)]
    pub const fn esr1(self) -> crate::common::Reg<regs::Esr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Interrupt Masks 2 Register"]
    #[inline(always)]
    pub const fn imask2(self) -> crate::common::Reg<regs::Imask2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Interrupt Masks 1 Register"]
    #[inline(always)]
    pub const fn imask1(self) -> crate::common::Reg<regs::Imask1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Interrupt Flags 2 Register"]
    #[inline(always)]
    pub const fn iflag2(self) -> crate::common::Reg<regs::Iflag2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Interrupt Flags 1 Register"]
    #[inline(always)]
    pub const fn iflag1(self) -> crate::common::Reg<regs::Iflag1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Control 2 Register"]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Error and Status 2 Register"]
    #[inline(always)]
    pub const fn esr2(self) -> crate::common::Reg<regs::Esr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "CRC Register"]
    #[inline(always)]
    pub const fn crcr(self) -> crate::common::Reg<regs::Crcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Rx FIFO Global Mask Register"]
    #[inline(always)]
    pub const fn rxfgmask(self) -> crate::common::Reg<regs::Rxfgmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Rx FIFO Information Register"]
    #[inline(always)]
    pub const fn rxfir(self) -> crate::common::Reg<regs::Rxfir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Debug 1 register"]
    #[inline(always)]
    pub const fn dbg1(self) -> crate::common::Reg<regs::Dbg1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Debug 2 register"]
    #[inline(always)]
    pub const fn dbg2(self) -> crate::common::Reg<regs::Dbg2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Rx Individual Mask Registers"]
    #[inline(always)]
    pub const fn rximr(self, n: usize) -> crate::common::Reg<regs::Rximr, crate::common::RW> {
        assert!(n < 64usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0880usize + n * 4usize) as _)
        }
    }
    #[doc = "Glitch Filter Width Registers"]
    #[inline(always)]
    pub const fn gfwr(self) -> crate::common::Reg<regs::Gfwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09e0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
