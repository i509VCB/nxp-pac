#[doc = "Intrusion and Tamper Response Controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itrc0 {
    ptr: *mut u8,
}
unsafe impl Send for Itrc0 {}
unsafe impl Sync for Itrc0 {}
impl Itrc0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ITRC outputs and IN0 to IN15 Status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "ITRC IN16 to IN47 Status."]
    #[inline(always)]
    pub const fn status1(self) -> crate::common::Reg<regs::Status1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Array of registers: OUT_SEL\\[%s\\]."]
    #[inline(always)]
    pub const fn ou_tx_sel(self, n: usize) -> OuTxSel {
        assert!(n < 7usize);
        unsafe { OuTxSel::from_ptr(self.ptr.wrapping_add(0x08usize + n * 8usize) as _) }
    }
    #[doc = "Array of registers: OUT_SEL_1\\[%s\\]."]
    #[inline(always)]
    pub const fn ou_tx_sel_1(self, n: usize) -> OuTxSel1 {
        assert!(n < 7usize);
        unsafe { OuTxSel1::from_ptr(self.ptr.wrapping_add(0x48usize + n * 8usize) as _) }
    }
    #[doc = "Array of registers: OUT_SEL_2\\[%s\\]."]
    #[inline(always)]
    pub const fn ou_tx_sel_2(self, n: usize) -> OuTxSel2 {
        assert!(n < 7usize);
        unsafe { OuTxSel2::from_ptr(self.ptr.wrapping_add(0x88usize + n * 8usize) as _) }
    }
    #[doc = "Software event 0."]
    #[inline(always)]
    pub const fn sw_event0(self) -> crate::common::Reg<regs::SwEvent0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Software event 1."]
    #[inline(always)]
    pub const fn sw_event1(self) -> crate::common::Reg<regs::SwEvent1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
}
#[doc = "Array of registers: OUT_SEL\\[%s\\]."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OuTxSel {
    ptr: *mut u8,
}
unsafe impl Send for OuTxSel {}
unsafe impl Sync for OuTxSel {}
impl OuTxSel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Trigger Source IN0 to IN15 selector."]
    #[inline(always)]
    pub const fn out_sel(self, n: usize) -> crate::common::Reg<regs::OutSel, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
}
#[doc = "Array of registers: OUT_SEL_1\\[%s\\]."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OuTxSel1 {
    ptr: *mut u8,
}
unsafe impl Send for OuTxSel1 {}
unsafe impl Sync for OuTxSel1 {}
impl OuTxSel1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Trigger Source IN16 to IN31 selector."]
    #[inline(always)]
    pub const fn out_sel_1(self, n: usize) -> crate::common::Reg<regs::OutSel1, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
}
#[doc = "Array of registers: OUT_SEL_2\\[%s\\]."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OuTxSel2 {
    ptr: *mut u8,
}
unsafe impl Send for OuTxSel2 {}
unsafe impl Sync for OuTxSel2 {}
impl OuTxSel2 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Trigger source IN32 to IN47 selector."]
    #[inline(always)]
    pub const fn out_sel_2(self, n: usize) -> crate::common::Reg<regs::OutSel2, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
