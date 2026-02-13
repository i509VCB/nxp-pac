#[doc = "General Purpose I/O (GPIO)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Secgpio {
    ptr: *mut u8,
}
unsafe impl Send for Secgpio {}
unsafe impl Sync for Secgpio {}
impl Secgpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_0(self) -> crate::common::Reg<regs::B00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_1(self) -> crate::common::Reg<regs::B01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_2(self) -> crate::common::Reg<regs::B02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_3(self) -> crate::common::Reg<regs::B03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_4(self) -> crate::common::Reg<regs::B04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_5(self) -> crate::common::Reg<regs::B05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_6(self) -> crate::common::Reg<regs::B06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_7(self) -> crate::common::Reg<regs::B07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_8(self) -> crate::common::Reg<regs::B08, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_9(self) -> crate::common::Reg<regs::B09, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_10(self) -> crate::common::Reg<regs::B010, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_11(self) -> crate::common::Reg<regs::B011, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_12(self) -> crate::common::Reg<regs::B012, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_13(self) -> crate::common::Reg<regs::B013, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dusize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_14(self) -> crate::common::Reg<regs::B014, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_15(self) -> crate::common::Reg<regs::B015, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fusize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_16(self) -> crate::common::Reg<regs::B016, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_17(self) -> crate::common::Reg<regs::B017, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_18(self) -> crate::common::Reg<regs::B018, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_19(self) -> crate::common::Reg<regs::B019, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_20(self) -> crate::common::Reg<regs::B020, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_21(self) -> crate::common::Reg<regs::B021, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_22(self) -> crate::common::Reg<regs::B022, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_23(self) -> crate::common::Reg<regs::B023, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x17usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_24(self) -> crate::common::Reg<regs::B024, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_25(self) -> crate::common::Reg<regs::B025, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x19usize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_26(self) -> crate::common::Reg<regs::B026, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_27(self) -> crate::common::Reg<regs::B027, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1busize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_28(self) -> crate::common::Reg<regs::B028, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_29(self) -> crate::common::Reg<regs::B029, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1dusize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_30(self) -> crate::common::Reg<regs::B030, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_31(self) -> crate::common::Reg<regs::B031, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1fusize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_0(self) -> crate::common::Reg<regs::W00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_1(self) -> crate::common::Reg<regs::W01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_2(self) -> crate::common::Reg<regs::W02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_3(self) -> crate::common::Reg<regs::W03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_4(self) -> crate::common::Reg<regs::W04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_5(self) -> crate::common::Reg<regs::W05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_6(self) -> crate::common::Reg<regs::W06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_7(self) -> crate::common::Reg<regs::W07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_8(self) -> crate::common::Reg<regs::W08, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_9(self) -> crate::common::Reg<regs::W09, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1024usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_10(self) -> crate::common::Reg<regs::W010, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_11(self) -> crate::common::Reg<regs::W011, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x102cusize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_12(self) -> crate::common::Reg<regs::W012, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1030usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_13(self) -> crate::common::Reg<regs::W013, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1034usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_14(self) -> crate::common::Reg<regs::W014, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1038usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_15(self) -> crate::common::Reg<regs::W015, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103cusize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_16(self) -> crate::common::Reg<regs::W016, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1040usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_17(self) -> crate::common::Reg<regs::W017, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1044usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_18(self) -> crate::common::Reg<regs::W018, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_19(self) -> crate::common::Reg<regs::W019, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x104cusize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_20(self) -> crate::common::Reg<regs::W020, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1050usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_21(self) -> crate::common::Reg<regs::W021, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1054usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_22(self) -> crate::common::Reg<regs::W022, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1058usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_23(self) -> crate::common::Reg<regs::W023, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x105cusize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_24(self) -> crate::common::Reg<regs::W024, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1060usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_25(self) -> crate::common::Reg<regs::W025, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1064usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_26(self) -> crate::common::Reg<regs::W026, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1068usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_27(self) -> crate::common::Reg<regs::W027, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x106cusize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_28(self) -> crate::common::Reg<regs::W028, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1070usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_29(self) -> crate::common::Reg<regs::W029, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1074usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_30(self) -> crate::common::Reg<regs::W030, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1078usize) as _) }
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_31(self) -> crate::common::Reg<regs::W031, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x107cusize) as _) }
    }
    #[doc = "Direction registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn dir0(self) -> crate::common::Reg<regs::Dir0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2000usize) as _) }
    }
    #[doc = "Mask register for all port GPIO pins"]
    #[inline(always)]
    pub const fn mask0(self) -> crate::common::Reg<regs::Mask0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2080usize) as _) }
    }
    #[doc = "Port pin register for all port GPIO pins"]
    #[inline(always)]
    pub const fn pin0(self) -> crate::common::Reg<regs::Pin0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2100usize) as _) }
    }
    #[doc = "Masked port register for all port GPIO pins"]
    #[inline(always)]
    pub const fn mpin0(self) -> crate::common::Reg<regs::Mpin0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2180usize) as _) }
    }
    #[doc = "Write: Set register for port. Read: output bits for port"]
    #[inline(always)]
    pub const fn set0(self) -> crate::common::Reg<regs::Set0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2200usize) as _) }
    }
    #[doc = "Clear port for all port GPIO pins"]
    #[inline(always)]
    pub const fn clr0(self) -> crate::common::Reg<regs::Clr0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2280usize) as _) }
    }
    #[doc = "Toggle port for all port GPIO pins"]
    #[inline(always)]
    pub const fn not0(self) -> crate::common::Reg<regs::Not0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2300usize) as _) }
    }
    #[doc = "Set pin direction bits for port"]
    #[inline(always)]
    pub const fn dirset0(self) -> crate::common::Reg<regs::Dirset0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2380usize) as _) }
    }
    #[doc = "Clear pin direction bits for port"]
    #[inline(always)]
    pub const fn dirclr0(self) -> crate::common::Reg<regs::Dirclr0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2400usize) as _) }
    }
    #[doc = "Toggle pin direction bits for port"]
    #[inline(always)]
    pub const fn dirnot0(self) -> crate::common::Reg<regs::Dirnot0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2480usize) as _) }
    }
}
pub mod regs;
