#[doc = "EIM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eim0 {
    ptr: *mut u8,
}
unsafe impl Send for Eim0 {}
unsafe impl Sync for Eim0 {}
impl Eim0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Error Injection Module Configuration Register"]
    #[inline(always)]
    pub const fn eimcr(self) -> crate::common::Reg<regs::Eimcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Error Injection Channel Enable register"]
    #[inline(always)]
    pub const fn eichen(self) -> crate::common::Reg<regs::Eichen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 0, Word0"]
    #[inline(always)]
    pub const fn eichd0_word0(self) -> crate::common::Reg<regs::Eichd0Word0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 0, Word1"]
    #[inline(always)]
    pub const fn eichd0_word1(self) -> crate::common::Reg<regs::Eichd0Word1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 1, Word0"]
    #[inline(always)]
    pub const fn eichd1_word0(self) -> crate::common::Reg<regs::Eichd1Word0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 1, Word1"]
    #[inline(always)]
    pub const fn eichd1_word1(self) -> crate::common::Reg<regs::Eichd1Word1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 2, Word0"]
    #[inline(always)]
    pub const fn eichd2_word0(self) -> crate::common::Reg<regs::Eichd2Word0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 2, Word1"]
    #[inline(always)]
    pub const fn eichd2_word1(self) -> crate::common::Reg<regs::Eichd2Word1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 3, Word0"]
    #[inline(always)]
    pub const fn eichd3_word0(self) -> crate::common::Reg<regs::Eichd3Word0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 3, Word1"]
    #[inline(always)]
    pub const fn eichd3_word1(self) -> crate::common::Reg<regs::Eichd3Word1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 4, Word0"]
    #[inline(always)]
    pub const fn eichd4_word0(self) -> crate::common::Reg<regs::Eichd4Word0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 4, Word1"]
    #[inline(always)]
    pub const fn eichd4_word1(self) -> crate::common::Reg<regs::Eichd4Word1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 5, Word0"]
    #[inline(always)]
    pub const fn eichd5_word0(self) -> crate::common::Reg<regs::Eichd5Word0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 5, Word1"]
    #[inline(always)]
    pub const fn eichd5_word1(self) -> crate::common::Reg<regs::Eichd5Word1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 6, Word0"]
    #[inline(always)]
    pub const fn eichd6_word0(self) -> crate::common::Reg<regs::Eichd6Word0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 6, Word1"]
    #[inline(always)]
    pub const fn eichd6_word1(self) -> crate::common::Reg<regs::Eichd6Word1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 7, Word0"]
    #[inline(always)]
    pub const fn eichd7_word0(self) -> crate::common::Reg<regs::Eichd7Word0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 7, Word1"]
    #[inline(always)]
    pub const fn eichd7_word1(self) -> crate::common::Reg<regs::Eichd7Word1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 8, Word0"]
    #[inline(always)]
    pub const fn eichd8_word0(self) -> crate::common::Reg<regs::Eichd8Word0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 8, Word1"]
    #[inline(always)]
    pub const fn eichd8_word1(self) -> crate::common::Reg<regs::Eichd8Word1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
}
pub mod regs;
