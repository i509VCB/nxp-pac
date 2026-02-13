#[doc = "Crossbar Switch"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbara1 {
    ptr: *mut u8,
}
unsafe impl Send for Xbara1 {}
unsafe impl Sync for Xbara1 {}
impl Xbara1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Crossbar A Select Register 0"]
    #[inline(always)]
    pub const fn sel0(self) -> crate::common::Reg<regs::Sel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Crossbar A Select Register 1"]
    #[inline(always)]
    pub const fn sel1(self) -> crate::common::Reg<regs::Sel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Crossbar A Select Register 2"]
    #[inline(always)]
    pub const fn sel2(self) -> crate::common::Reg<regs::Sel2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Crossbar A Select Register 3"]
    #[inline(always)]
    pub const fn sel3(self) -> crate::common::Reg<regs::Sel3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Crossbar A Select Register 4"]
    #[inline(always)]
    pub const fn sel4(self) -> crate::common::Reg<regs::Sel4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Crossbar A Select Register 5"]
    #[inline(always)]
    pub const fn sel5(self) -> crate::common::Reg<regs::Sel5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Crossbar A Select Register 6"]
    #[inline(always)]
    pub const fn sel6(self) -> crate::common::Reg<regs::Sel6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Crossbar A Select Register 7"]
    #[inline(always)]
    pub const fn sel7(self) -> crate::common::Reg<regs::Sel7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Crossbar A Select Register 8"]
    #[inline(always)]
    pub const fn sel8(self) -> crate::common::Reg<regs::Sel8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Crossbar A Select Register 9"]
    #[inline(always)]
    pub const fn sel9(self) -> crate::common::Reg<regs::Sel9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Crossbar A Select Register 10"]
    #[inline(always)]
    pub const fn sel10(self) -> crate::common::Reg<regs::Sel10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Crossbar A Select Register 11"]
    #[inline(always)]
    pub const fn sel11(self) -> crate::common::Reg<regs::Sel11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "Crossbar A Select Register 12"]
    #[inline(always)]
    pub const fn sel12(self) -> crate::common::Reg<regs::Sel12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Crossbar A Select Register 13"]
    #[inline(always)]
    pub const fn sel13(self) -> crate::common::Reg<regs::Sel13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "Crossbar A Select Register 14"]
    #[inline(always)]
    pub const fn sel14(self) -> crate::common::Reg<regs::Sel14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Crossbar A Select Register 15"]
    #[inline(always)]
    pub const fn sel15(self) -> crate::common::Reg<regs::Sel15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "Crossbar A Select Register 16"]
    #[inline(always)]
    pub const fn sel16(self) -> crate::common::Reg<regs::Sel16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Crossbar A Select Register 17"]
    #[inline(always)]
    pub const fn sel17(self) -> crate::common::Reg<regs::Sel17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "Crossbar A Select Register 18"]
    #[inline(always)]
    pub const fn sel18(self) -> crate::common::Reg<regs::Sel18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Crossbar A Select Register 19"]
    #[inline(always)]
    pub const fn sel19(self) -> crate::common::Reg<regs::Sel19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "Crossbar A Select Register 20"]
    #[inline(always)]
    pub const fn sel20(self) -> crate::common::Reg<regs::Sel20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Crossbar A Select Register 21"]
    #[inline(always)]
    pub const fn sel21(self) -> crate::common::Reg<regs::Sel21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "Crossbar A Select Register 22"]
    #[inline(always)]
    pub const fn sel22(self) -> crate::common::Reg<regs::Sel22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Crossbar A Select Register 23"]
    #[inline(always)]
    pub const fn sel23(self) -> crate::common::Reg<regs::Sel23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2eusize) as _) }
    }
    #[doc = "Crossbar A Select Register 24"]
    #[inline(always)]
    pub const fn sel24(self) -> crate::common::Reg<regs::Sel24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Crossbar A Select Register 25"]
    #[inline(always)]
    pub const fn sel25(self) -> crate::common::Reg<regs::Sel25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "Crossbar A Select Register 26"]
    #[inline(always)]
    pub const fn sel26(self) -> crate::common::Reg<regs::Sel26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Crossbar A Select Register 27"]
    #[inline(always)]
    pub const fn sel27(self) -> crate::common::Reg<regs::Sel27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36usize) as _) }
    }
    #[doc = "Crossbar A Select Register 28"]
    #[inline(always)]
    pub const fn sel28(self) -> crate::common::Reg<regs::Sel28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Crossbar A Select Register 29"]
    #[inline(always)]
    pub const fn sel29(self) -> crate::common::Reg<regs::Sel29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3ausize) as _) }
    }
    #[doc = "Crossbar A Select Register 30"]
    #[inline(always)]
    pub const fn sel30(self) -> crate::common::Reg<regs::Sel30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Crossbar A Select Register 31"]
    #[inline(always)]
    pub const fn sel31(self) -> crate::common::Reg<regs::Sel31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3eusize) as _) }
    }
    #[doc = "Crossbar A Select Register 32"]
    #[inline(always)]
    pub const fn sel32(self) -> crate::common::Reg<regs::Sel32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Crossbar A Select Register 33"]
    #[inline(always)]
    pub const fn sel33(self) -> crate::common::Reg<regs::Sel33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize) as _) }
    }
    #[doc = "Crossbar A Select Register 34"]
    #[inline(always)]
    pub const fn sel34(self) -> crate::common::Reg<regs::Sel34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Crossbar A Select Register 35"]
    #[inline(always)]
    pub const fn sel35(self) -> crate::common::Reg<regs::Sel35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x46usize) as _) }
    }
    #[doc = "Crossbar A Select Register 36"]
    #[inline(always)]
    pub const fn sel36(self) -> crate::common::Reg<regs::Sel36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Crossbar A Select Register 37"]
    #[inline(always)]
    pub const fn sel37(self) -> crate::common::Reg<regs::Sel37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4ausize) as _) }
    }
    #[doc = "Crossbar A Select Register 38"]
    #[inline(always)]
    pub const fn sel38(self) -> crate::common::Reg<regs::Sel38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Crossbar A Select Register 39"]
    #[inline(always)]
    pub const fn sel39(self) -> crate::common::Reg<regs::Sel39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4eusize) as _) }
    }
    #[doc = "Crossbar A Select Register 40"]
    #[inline(always)]
    pub const fn sel40(self) -> crate::common::Reg<regs::Sel40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Crossbar A Select Register 41"]
    #[inline(always)]
    pub const fn sel41(self) -> crate::common::Reg<regs::Sel41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x52usize) as _) }
    }
    #[doc = "Crossbar A Select Register 42"]
    #[inline(always)]
    pub const fn sel42(self) -> crate::common::Reg<regs::Sel42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Crossbar A Select Register 43"]
    #[inline(always)]
    pub const fn sel43(self) -> crate::common::Reg<regs::Sel43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x56usize) as _) }
    }
    #[doc = "Crossbar A Select Register 44"]
    #[inline(always)]
    pub const fn sel44(self) -> crate::common::Reg<regs::Sel44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Crossbar A Select Register 45"]
    #[inline(always)]
    pub const fn sel45(self) -> crate::common::Reg<regs::Sel45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5ausize) as _) }
    }
    #[doc = "Crossbar A Select Register 46"]
    #[inline(always)]
    pub const fn sel46(self) -> crate::common::Reg<regs::Sel46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Crossbar A Select Register 47"]
    #[inline(always)]
    pub const fn sel47(self) -> crate::common::Reg<regs::Sel47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5eusize) as _) }
    }
    #[doc = "Crossbar A Select Register 48"]
    #[inline(always)]
    pub const fn sel48(self) -> crate::common::Reg<regs::Sel48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Crossbar A Select Register 49"]
    #[inline(always)]
    pub const fn sel49(self) -> crate::common::Reg<regs::Sel49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x62usize) as _) }
    }
    #[doc = "Crossbar A Select Register 50"]
    #[inline(always)]
    pub const fn sel50(self) -> crate::common::Reg<regs::Sel50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Crossbar A Select Register 51"]
    #[inline(always)]
    pub const fn sel51(self) -> crate::common::Reg<regs::Sel51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x66usize) as _) }
    }
    #[doc = "Crossbar A Select Register 52"]
    #[inline(always)]
    pub const fn sel52(self) -> crate::common::Reg<regs::Sel52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Crossbar A Select Register 53"]
    #[inline(always)]
    pub const fn sel53(self) -> crate::common::Reg<regs::Sel53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6ausize) as _) }
    }
    #[doc = "Crossbar A Select Register 54"]
    #[inline(always)]
    pub const fn sel54(self) -> crate::common::Reg<regs::Sel54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Crossbar A Select Register 55"]
    #[inline(always)]
    pub const fn sel55(self) -> crate::common::Reg<regs::Sel55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6eusize) as _) }
    }
    #[doc = "Crossbar A Select Register 56"]
    #[inline(always)]
    pub const fn sel56(self) -> crate::common::Reg<regs::Sel56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Crossbar A Select Register 57"]
    #[inline(always)]
    pub const fn sel57(self) -> crate::common::Reg<regs::Sel57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x72usize) as _) }
    }
    #[doc = "Crossbar A Select Register 58"]
    #[inline(always)]
    pub const fn sel58(self) -> crate::common::Reg<regs::Sel58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Crossbar A Select Register 59"]
    #[inline(always)]
    pub const fn sel59(self) -> crate::common::Reg<regs::Sel59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x76usize) as _) }
    }
    #[doc = "Crossbar A Select Register 60"]
    #[inline(always)]
    pub const fn sel60(self) -> crate::common::Reg<regs::Sel60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Crossbar A Select Register 61"]
    #[inline(always)]
    pub const fn sel61(self) -> crate::common::Reg<regs::Sel61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7ausize) as _) }
    }
    #[doc = "Crossbar A Select Register 62"]
    #[inline(always)]
    pub const fn sel62(self) -> crate::common::Reg<regs::Sel62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Crossbar A Select Register 63"]
    #[inline(always)]
    pub const fn sel63(self) -> crate::common::Reg<regs::Sel63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7eusize) as _) }
    }
    #[doc = "Crossbar A Select Register 64"]
    #[inline(always)]
    pub const fn sel64(self) -> crate::common::Reg<regs::Sel64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Crossbar A Select Register 65"]
    #[inline(always)]
    pub const fn sel65(self) -> crate::common::Reg<regs::Sel65, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x82usize) as _) }
    }
    #[doc = "Crossbar A Control Register 0"]
    #[inline(always)]
    pub const fn ctrl0(self) -> crate::common::Reg<regs::Ctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Crossbar A Control Register 1"]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x86usize) as _) }
    }
}
pub mod regs;
pub mod vals;
