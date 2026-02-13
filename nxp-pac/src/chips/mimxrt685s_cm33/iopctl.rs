#[doc = "LPC-Next0 IO pad controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iopctl {
    ptr: *mut u8,
}
unsafe impl Send for Iopctl {}
unsafe impl Sync for Iopctl {}
impl Iopctl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_0(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_1(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_2(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_3(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_4(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_5(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_6(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_7(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_8(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_9(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_10(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_11(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_12(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_13(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_14(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_15(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_16(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_17(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_18(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_19(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_20(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_21(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_22(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_23(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_24(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_25(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_26(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_27(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_28(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_29(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_30(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_31(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_0(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_1(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_2(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_3(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_4(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_5(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_6(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_7(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_8(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_9(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_10(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_11(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_12(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_13(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_14(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_15(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_16(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_17(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_18(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_19(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_20(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_21(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_22(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_23(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_24(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_25(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_26(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_27(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_28(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_29(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_30(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_31(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_0(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_1(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_2(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_3(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_4(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_5(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_6(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_7(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_8(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_9(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_10(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_11(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_12(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_13(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_14(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_15(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_16(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_17(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_18(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_19(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_20(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_21(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_22(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_23(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_24(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_25(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_26(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_27(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_28(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_29(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_30(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_31(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_0(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_1(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_2(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_3(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_4(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_5(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_6(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_7(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_8(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_9(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_10(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_11(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_12(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_13(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_14(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_15(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_16(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_17(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_18(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_19(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_20(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_21(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_22(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_23(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_24(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_25(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_26(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_27(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_28(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_29(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_30(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_31(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_0(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_1(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_2(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_3(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_4(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_5(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_6(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_7(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_8(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_9(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_10(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_11(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_12(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_13(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_14(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_15(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_16(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_17(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_18(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_19(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_20(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_21(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_22(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_23(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_24(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_25(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_26(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_27(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_28(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_29(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_30(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_31(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_0(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_1(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_2(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_3(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_4(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_5(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_6(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_7(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x029cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_8(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_9(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_10(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_11(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_12(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_13(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_14(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_15(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_16(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_17(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_18(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_19(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_20(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_21(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_22(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_23(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02dcusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_24(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_25(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_26(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_27(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ecusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_28(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_29(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_30(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_31(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02fcusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_0(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_1(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_2(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_3(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_4(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_5(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_6(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_7(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x031cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_8(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_9(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_10(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0328usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_11(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x032cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_12(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_13(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0334usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_14(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0338usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_15(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x033cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_16(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_17(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0344usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_18(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0348usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_19(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x034cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_20(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0350usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_21(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0354usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_22(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0358usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_23(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x035cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_24(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_25(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0364usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_26(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0368usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_27(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x036cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_28(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_29(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0374usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_30(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_31(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x037cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_0(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_1(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_2(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_3(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_4(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0390usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_5(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_6(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_7(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x039cusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_8(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_9(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_10(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_11(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03acusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_12(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_13(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_14(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_15(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03bcusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_16(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_17(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_18(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_19(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ccusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_20(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_21(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_22(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_23(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03dcusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_24(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_25(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_26(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_27(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_28(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_29(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_30(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_31(self) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "Special Registers (No GPIO Function)"]
    #[inline(always)]
    pub const fn fc15_i2c_scl(self) -> crate::common::Reg<regs::Fc15I2cScl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "Special Registers (No GPIO Function)"]
    #[inline(always)]
    pub const fn fc15_i2c_sda(self) -> crate::common::Reg<regs::Fc15I2cSda, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
}
pub mod regs;
pub mod vals;
