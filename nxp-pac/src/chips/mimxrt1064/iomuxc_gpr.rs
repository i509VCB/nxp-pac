#[doc = "IOMUXC_GPR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IomuxcGpr {
    ptr: *mut u8,
}
unsafe impl Send for IomuxcGpr {}
unsafe impl Sync for IomuxcGpr {}
impl IomuxcGpr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPR0 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "GPR1 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr1(self) -> crate::common::Reg<regs::Gpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "GPR2 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr2(self) -> crate::common::Reg<regs::Gpr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "GPR3 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr3(self) -> crate::common::Reg<regs::Gpr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "GPR4 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr4(self) -> crate::common::Reg<regs::Gpr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "GPR5 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr5(self) -> crate::common::Reg<regs::Gpr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "GPR6 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr6(self) -> crate::common::Reg<regs::Gpr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "GPR7 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr7(self) -> crate::common::Reg<regs::Gpr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "GPR8 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr8(self) -> crate::common::Reg<regs::Gpr8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "GPR9 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr9(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "GPR10 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr10(self) -> crate::common::Reg<regs::Gpr10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "GPR11 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr11(self) -> crate::common::Reg<regs::Gpr11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "GPR12 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr12(self) -> crate::common::Reg<regs::Gpr12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "GPR13 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr13(self) -> crate::common::Reg<regs::Gpr13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "GPR14 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr14(self) -> crate::common::Reg<regs::Gpr14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "GPR15 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr15(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "GPR16 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr16(self) -> crate::common::Reg<regs::Gpr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "GPR17 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr17(self) -> crate::common::Reg<regs::Gpr17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "GPR18 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr18(self) -> crate::common::Reg<regs::Gpr18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "GPR19 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr19(self) -> crate::common::Reg<regs::Gpr19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "GPR20 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr20(self) -> crate::common::Reg<regs::Gpr20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "GPR21 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr21(self) -> crate::common::Reg<regs::Gpr21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "GPR22 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr22(self) -> crate::common::Reg<regs::Gpr22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "GPR23 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr23(self) -> crate::common::Reg<regs::Gpr23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "GPR24 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr24(self) -> crate::common::Reg<regs::Gpr24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "GPR25 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr25(self) -> crate::common::Reg<regs::Gpr25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "GPR26 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr26(self) -> crate::common::Reg<regs::Gpr26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "GPR27 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr27(self) -> crate::common::Reg<regs::Gpr27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "GPR28 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr28(self) -> crate::common::Reg<regs::Gpr28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "GPR29 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr29(self) -> crate::common::Reg<regs::Gpr29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "GPR30 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr30(self) -> crate::common::Reg<regs::Gpr30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "GPR31 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr31(self) -> crate::common::Reg<regs::Gpr31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "GPR32 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr32(self) -> crate::common::Reg<regs::Gpr32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "GPR33 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr33(self) -> crate::common::Reg<regs::Gpr33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "GPR34 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr34(self) -> crate::common::Reg<regs::Gpr34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
}
pub mod regs;
pub mod vals;
