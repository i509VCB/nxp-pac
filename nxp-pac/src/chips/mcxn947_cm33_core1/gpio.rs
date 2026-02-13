#[doc = "GPIO"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
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
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Pin Control Nonsecure"]
    #[inline(always)]
    pub const fn pcns(self) -> crate::common::Reg<regs::Pcns, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt Control Nonsecure"]
    #[inline(always)]
    pub const fn icns(self) -> crate::common::Reg<regs::Icns, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Pin Control Nonprivilege"]
    #[inline(always)]
    pub const fn pcnp(self) -> crate::common::Reg<regs::Pcnp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Interrupt Control Nonprivilege"]
    #[inline(always)]
    pub const fn icnp(self) -> crate::common::Reg<regs::Icnp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn pdor(self) -> crate::common::Reg<regs::Pdor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn psor(self) -> crate::common::Reg<regs::Psor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn pcor(self) -> crate::common::Reg<regs::Pcor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn ptor(self) -> crate::common::Reg<regs::Ptor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn pdir(self) -> crate::common::Reg<regs::Pdir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn pddr(self) -> crate::common::Reg<regs::Pddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn pidr(self) -> crate::common::Reg<regs::Pidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Pin Data"]
    #[inline(always)]
    pub const fn pdr(self, n: usize) -> crate::common::Reg<regs::Pdr, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 1usize) as _) }
    }
    #[doc = "Interrupt Control 0"]
    #[inline(always)]
    pub const fn icr0(self) -> crate::common::Reg<regs::Icr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Interrupt Control 1"]
    #[inline(always)]
    pub const fn icr1(self) -> crate::common::Reg<regs::Icr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Interrupt Control 2"]
    #[inline(always)]
    pub const fn icr2(self) -> crate::common::Reg<regs::Icr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Interrupt Control 3"]
    #[inline(always)]
    pub const fn icr3(self) -> crate::common::Reg<regs::Icr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Interrupt Control 4"]
    #[inline(always)]
    pub const fn icr4(self) -> crate::common::Reg<regs::Icr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Interrupt Control 5"]
    #[inline(always)]
    pub const fn icr5(self) -> crate::common::Reg<regs::Icr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Interrupt Control 6"]
    #[inline(always)]
    pub const fn icr6(self) -> crate::common::Reg<regs::Icr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Interrupt Control 7"]
    #[inline(always)]
    pub const fn icr7(self) -> crate::common::Reg<regs::Icr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Interrupt Control 8"]
    #[inline(always)]
    pub const fn icr8(self) -> crate::common::Reg<regs::Icr8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Interrupt Control 9"]
    #[inline(always)]
    pub const fn icr9(self) -> crate::common::Reg<regs::Icr9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Interrupt Control 10"]
    #[inline(always)]
    pub const fn icr10(self) -> crate::common::Reg<regs::Icr10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Interrupt Control 11"]
    #[inline(always)]
    pub const fn icr11(self) -> crate::common::Reg<regs::Icr11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Interrupt Control 12"]
    #[inline(always)]
    pub const fn icr12(self) -> crate::common::Reg<regs::Icr12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Interrupt Control 13"]
    #[inline(always)]
    pub const fn icr13(self) -> crate::common::Reg<regs::Icr13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Interrupt Control 14"]
    #[inline(always)]
    pub const fn icr14(self) -> crate::common::Reg<regs::Icr14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Interrupt Control 15"]
    #[inline(always)]
    pub const fn icr15(self) -> crate::common::Reg<regs::Icr15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Interrupt Control 16"]
    #[inline(always)]
    pub const fn icr16(self) -> crate::common::Reg<regs::Icr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Interrupt Control 17"]
    #[inline(always)]
    pub const fn icr17(self) -> crate::common::Reg<regs::Icr17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Interrupt Control 18"]
    #[inline(always)]
    pub const fn icr18(self) -> crate::common::Reg<regs::Icr18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Interrupt Control 19"]
    #[inline(always)]
    pub const fn icr19(self) -> crate::common::Reg<regs::Icr19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Interrupt Control 20"]
    #[inline(always)]
    pub const fn icr20(self) -> crate::common::Reg<regs::Icr20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Interrupt Control 21"]
    #[inline(always)]
    pub const fn icr21(self) -> crate::common::Reg<regs::Icr21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Interrupt Control 22"]
    #[inline(always)]
    pub const fn icr22(self) -> crate::common::Reg<regs::Icr22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Interrupt Control 23"]
    #[inline(always)]
    pub const fn icr23(self) -> crate::common::Reg<regs::Icr23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Interrupt Control 24"]
    #[inline(always)]
    pub const fn icr24(self) -> crate::common::Reg<regs::Icr24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Interrupt Control 25"]
    #[inline(always)]
    pub const fn icr25(self) -> crate::common::Reg<regs::Icr25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Interrupt Control 26"]
    #[inline(always)]
    pub const fn icr26(self) -> crate::common::Reg<regs::Icr26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Interrupt Control 27"]
    #[inline(always)]
    pub const fn icr27(self) -> crate::common::Reg<regs::Icr27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Interrupt Control 28"]
    #[inline(always)]
    pub const fn icr28(self) -> crate::common::Reg<regs::Icr28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Interrupt Control 29"]
    #[inline(always)]
    pub const fn icr29(self) -> crate::common::Reg<regs::Icr29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Interrupt Control 30"]
    #[inline(always)]
    pub const fn icr30(self) -> crate::common::Reg<regs::Icr30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Interrupt Control 31"]
    #[inline(always)]
    pub const fn icr31(self) -> crate::common::Reg<regs::Icr31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Global Interrupt Control Low"]
    #[inline(always)]
    pub const fn giclr(self) -> crate::common::Reg<regs::Giclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Global Interrupt Control High"]
    #[inline(always)]
    pub const fn gichr(self) -> crate::common::Reg<regs::Gichr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn isfr(self, n: usize) -> crate::common::Reg<regs::Isfr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
