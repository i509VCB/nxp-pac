#[doc = "General Purpose I/O (GPIO)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SECGPIO {
    ptr: *mut u8,
}
unsafe impl Send for SECGPIO {}
unsafe impl Sync for SECGPIO {}
impl SECGPIO {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_0(self) -> crate::common::Reg<regs::B0_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_1(self) -> crate::common::Reg<regs::B0_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_2(self) -> crate::common::Reg<regs::B0_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_3(self) -> crate::common::Reg<regs::B0_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_4(self) -> crate::common::Reg<regs::B0_4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_5(self) -> crate::common::Reg<regs::B0_5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_6(self) -> crate::common::Reg<regs::B0_6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_7(self) -> crate::common::Reg<regs::B0_7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_8(self) -> crate::common::Reg<regs::B0_8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_9(self) -> crate::common::Reg<regs::B0_9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_10(self) -> crate::common::Reg<regs::B0_10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_11(self) -> crate::common::Reg<regs::B0_11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_12(self) -> crate::common::Reg<regs::B0_12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_13(self) -> crate::common::Reg<regs::B0_13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dusize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_14(self) -> crate::common::Reg<regs::B0_14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_15(self) -> crate::common::Reg<regs::B0_15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fusize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_16(self) -> crate::common::Reg<regs::B0_16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_17(self) -> crate::common::Reg<regs::B0_17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_18(self) -> crate::common::Reg<regs::B0_18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_19(self) -> crate::common::Reg<regs::B0_19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_20(self) -> crate::common::Reg<regs::B0_20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_21(self) -> crate::common::Reg<regs::B0_21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_22(self) -> crate::common::Reg<regs::B0_22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_23(self) -> crate::common::Reg<regs::B0_23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x17usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_24(self) -> crate::common::Reg<regs::B0_24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_25(self) -> crate::common::Reg<regs::B0_25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x19usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_26(self) -> crate::common::Reg<regs::B0_26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_27(self) -> crate::common::Reg<regs::B0_27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1busize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_28(self) -> crate::common::Reg<regs::B0_28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_29(self) -> crate::common::Reg<regs::B0_29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1dusize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_30(self) -> crate::common::Reg<regs::B0_30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B0_31(self) -> crate::common::Reg<regs::B0_31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1fusize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_0(self) -> crate::common::Reg<regs::W0_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_1(self) -> crate::common::Reg<regs::W0_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_2(self) -> crate::common::Reg<regs::W0_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_3(self) -> crate::common::Reg<regs::W0_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_4(self) -> crate::common::Reg<regs::W0_4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_5(self) -> crate::common::Reg<regs::W0_5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_6(self) -> crate::common::Reg<regs::W0_6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_7(self) -> crate::common::Reg<regs::W0_7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_8(self) -> crate::common::Reg<regs::W0_8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_9(self) -> crate::common::Reg<regs::W0_9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1024usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_10(self) -> crate::common::Reg<regs::W0_10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_11(self) -> crate::common::Reg<regs::W0_11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x102cusize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_12(self) -> crate::common::Reg<regs::W0_12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1030usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_13(self) -> crate::common::Reg<regs::W0_13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1034usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_14(self) -> crate::common::Reg<regs::W0_14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1038usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_15(self) -> crate::common::Reg<regs::W0_15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103cusize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_16(self) -> crate::common::Reg<regs::W0_16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1040usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_17(self) -> crate::common::Reg<regs::W0_17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1044usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_18(self) -> crate::common::Reg<regs::W0_18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_19(self) -> crate::common::Reg<regs::W0_19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x104cusize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_20(self) -> crate::common::Reg<regs::W0_20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1050usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_21(self) -> crate::common::Reg<regs::W0_21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1054usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_22(self) -> crate::common::Reg<regs::W0_22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1058usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_23(self) -> crate::common::Reg<regs::W0_23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x105cusize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_24(self) -> crate::common::Reg<regs::W0_24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1060usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_25(self) -> crate::common::Reg<regs::W0_25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1064usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_26(self) -> crate::common::Reg<regs::W0_26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1068usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_27(self) -> crate::common::Reg<regs::W0_27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x106cusize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_28(self) -> crate::common::Reg<regs::W0_28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1070usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_29(self) -> crate::common::Reg<regs::W0_29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1074usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_30(self) -> crate::common::Reg<regs::W0_30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1078usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W0_31(self) -> crate::common::Reg<regs::W0_31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x107cusize) as _) }
    }
    #[doc = "Direction registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn DIR0(self) -> crate::common::Reg<regs::DIR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2000usize) as _) }
    }
    #[doc = "Mask register for all port GPIO pins."]
    #[inline(always)]
    pub const fn MASK0(self) -> crate::common::Reg<regs::MASK0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2080usize) as _) }
    }
    #[doc = "Port pin register for all port GPIO pins."]
    #[inline(always)]
    pub const fn PIN0(self) -> crate::common::Reg<regs::PIN0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2100usize) as _) }
    }
    #[doc = "Masked port register for all port GPIO pins."]
    #[inline(always)]
    pub const fn MPIN0(self) -> crate::common::Reg<regs::MPIN0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2180usize) as _) }
    }
    #[doc = "Write: Set register for port. Read: output bits for port."]
    #[inline(always)]
    pub const fn SET0(self) -> crate::common::Reg<regs::SET0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2200usize) as _) }
    }
    #[doc = "Clear port for all port GPIO pins."]
    #[inline(always)]
    pub const fn CLR0(self) -> crate::common::Reg<regs::CLR0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2280usize) as _) }
    }
    #[doc = "Toggle port for all port GPIO pins."]
    #[inline(always)]
    pub const fn NOT0(self) -> crate::common::Reg<regs::NOT0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2300usize) as _) }
    }
    #[doc = "Set pin direction bits for port."]
    #[inline(always)]
    pub const fn DIRSET0(self) -> crate::common::Reg<regs::DIRSET0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2380usize) as _) }
    }
    #[doc = "Clear pin direction bits for port."]
    #[inline(always)]
    pub const fn DIRCLR0(self) -> crate::common::Reg<regs::DIRCLR0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2400usize) as _) }
    }
    #[doc = "Toggle pin direction bits for port."]
    #[inline(always)]
    pub const fn DIRNOT0(self) -> crate::common::Reg<regs::DIRNOT0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2480usize) as _) }
    }
}
pub mod regs;
