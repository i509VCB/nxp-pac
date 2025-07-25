#[doc = "PORT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port4 {
    ptr: *mut u8,
}
unsafe impl Send for Port4 {}
unsafe impl Sync for Port4 {}
impl Port4 {
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Global Pin Control Low"]
    #[inline(always)]
    pub const fn gpclr(self) -> crate::common::Reg<regs::Gpclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Global Pin Control High"]
    #[inline(always)]
    pub const fn gpchr(self) -> crate::common::Reg<regs::Gpchr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Configuration"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "EFT Detect Flag"]
    #[inline(always)]
    pub const fn edfr(self) -> crate::common::Reg<regs::Edfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn edier(self) -> crate::common::Reg<regs::Edier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "EFT Detect Clear"]
    #[inline(always)]
    pub const fn edcr(self) -> crate::common::Reg<regs::Edcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Pin Control 0"]
    #[inline(always)]
    pub const fn pcr0(self) -> crate::common::Reg<regs::Pcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Pin Control 1"]
    #[inline(always)]
    pub const fn pcr1(self) -> crate::common::Reg<regs::Pcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Pin Control 2"]
    #[inline(always)]
    pub const fn pcr2(self) -> crate::common::Reg<regs::Pcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Pin Control 3"]
    #[inline(always)]
    pub const fn pcr3(self) -> crate::common::Reg<regs::Pcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Pin Control 4"]
    #[inline(always)]
    pub const fn pcr4(self) -> crate::common::Reg<regs::Pcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Pin Control 5"]
    #[inline(always)]
    pub const fn pcr5(self) -> crate::common::Reg<regs::Pcr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "Pin Control 6"]
    #[inline(always)]
    pub const fn pcr6(self) -> crate::common::Reg<regs::Pcr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "Pin Control 7"]
    #[inline(always)]
    pub const fn pcr7(self) -> crate::common::Reg<regs::Pcr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "Pin Control 12"]
    #[inline(always)]
    pub const fn pcr12(self) -> crate::common::Reg<regs::Pcr12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "Pin Control 13"]
    #[inline(always)]
    pub const fn pcr13(self) -> crate::common::Reg<regs::Pcr13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "Pin Control 14"]
    #[inline(always)]
    pub const fn pcr14(self) -> crate::common::Reg<regs::Pcr14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "Pin Control 15"]
    #[inline(always)]
    pub const fn pcr15(self) -> crate::common::Reg<regs::Pcr15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "Pin Control 16"]
    #[inline(always)]
    pub const fn pcr16(self) -> crate::common::Reg<regs::Pcr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "Pin Control 17"]
    #[inline(always)]
    pub const fn pcr17(self) -> crate::common::Reg<regs::Pcr17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "Pin Control 18"]
    #[inline(always)]
    pub const fn pcr18(self) -> crate::common::Reg<regs::Pcr18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "Pin Control 19"]
    #[inline(always)]
    pub const fn pcr19(self) -> crate::common::Reg<regs::Pcr19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "Pin Control 20"]
    #[inline(always)]
    pub const fn pcr20(self) -> crate::common::Reg<regs::Pcr20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "Pin Control 21"]
    #[inline(always)]
    pub const fn pcr21(self) -> crate::common::Reg<regs::Pcr21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "Pin Control 22"]
    #[inline(always)]
    pub const fn pcr22(self) -> crate::common::Reg<regs::Pcr22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "Pin Control 23"]
    #[inline(always)]
    pub const fn pcr23(self) -> crate::common::Reg<regs::Pcr23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
