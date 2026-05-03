#[doc = "FLASH_KEY_STORE."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_KEY_STORE {
    ptr: *mut u8,
}
unsafe impl Send for FLASH_KEY_STORE {}
unsafe impl Sync for FLASH_KEY_STORE {}
impl FLASH_KEY_STORE {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Valid Key Sore Header : 0x95959595."]
    #[inline(always)]
    pub const fn HEADER(self) -> crate::common::Reg<regs::HEADER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "puf discharge time in ms."]
    #[inline(always)]
    pub const fn puf_discharge_time_in_ms(
        self,
    ) -> crate::common::Reg<regs::puf_discharge_time_in_ms, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn ACTIVATION_CODE(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ACTIVATION_CODE, crate::common::RW> {
        assert!(n < 298usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_HEADER0(self) -> crate::common::Reg<regs::SBKEY_HEADER0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_KEY_CODE0(
        self,
    ) -> crate::common::Reg<regs::SBKEY_KEY_CODE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_HEADER1(self) -> crate::common::Reg<regs::SBKEY_HEADER1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_KEY_CODE1(
        self,
    ) -> crate::common::Reg<regs::SBKEY_KEY_CODE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_BODY0(self) -> crate::common::Reg<regs::SBKEY_BODY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_KEY_CODE2(
        self,
    ) -> crate::common::Reg<regs::SBKEY_KEY_CODE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_BODY1(self) -> crate::common::Reg<regs::SBKEY_BODY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04bcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_KEY_CODE3(
        self,
    ) -> crate::common::Reg<regs::SBKEY_KEY_CODE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04bcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_BODY2(self) -> crate::common::Reg<regs::SBKEY_BODY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_KEY_CODE4(
        self,
    ) -> crate::common::Reg<regs::SBKEY_KEY_CODE4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_BODY3(self) -> crate::common::Reg<regs::SBKEY_BODY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_KEY_CODE5(
        self,
    ) -> crate::common::Reg<regs::SBKEY_KEY_CODE5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_BODY4(self) -> crate::common::Reg<regs::SBKEY_BODY4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_KEY_CODE6(
        self,
    ) -> crate::common::Reg<regs::SBKEY_KEY_CODE6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_BODY5(self) -> crate::common::Reg<regs::SBKEY_BODY5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ccusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_KEY_CODE7(
        self,
    ) -> crate::common::Reg<regs::SBKEY_KEY_CODE7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ccusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_BODY6(self) -> crate::common::Reg<regs::SBKEY_BODY6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_KEY_CODE8(
        self,
    ) -> crate::common::Reg<regs::SBKEY_KEY_CODE8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_BODY7(self) -> crate::common::Reg<regs::SBKEY_BODY7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_KEY_CODE9(
        self,
    ) -> crate::common::Reg<regs::SBKEY_KEY_CODE9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_BODY8(self) -> crate::common::Reg<regs::SBKEY_BODY8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_KEY_CODE10(
        self,
    ) -> crate::common::Reg<regs::SBKEY_KEY_CODE10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_BODY9(self) -> crate::common::Reg<regs::SBKEY_BODY9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04dcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_KEY_CODE11(
        self,
    ) -> crate::common::Reg<regs::SBKEY_KEY_CODE11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04dcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_BODY10(self) -> crate::common::Reg<regs::SBKEY_BODY10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_KEY_CODE12(
        self,
    ) -> crate::common::Reg<regs::SBKEY_KEY_CODE12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_BODY11(self) -> crate::common::Reg<regs::SBKEY_BODY11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn SBKEY_KEY_CODE13(
        self,
    ) -> crate::common::Reg<regs::SBKEY_KEY_CODE13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_HEADER0(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_HEADER0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_KEY_CODE0(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_KEY_CODE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_HEADER1(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_HEADER1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ecusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_KEY_CODE1(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_KEY_CODE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ecusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_BODY0(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_BODY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_KEY_CODE2(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_KEY_CODE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_BODY1(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_BODY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_KEY_CODE3(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_KEY_CODE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_BODY2(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_BODY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_KEY_CODE4(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_KEY_CODE4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_BODY3(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_BODY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04fcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_KEY_CODE5(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_KEY_CODE5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04fcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_BODY4(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_BODY4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_KEY_CODE6(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_KEY_CODE6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_BODY5(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_BODY5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_KEY_CODE7(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_KEY_CODE7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_BODY6(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_BODY6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_KEY_CODE8(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_KEY_CODE8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_BODY7(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_BODY7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_KEY_CODE9(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_KEY_CODE9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_BODY8(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_BODY8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_KEY_CODE10(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_KEY_CODE10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_BODY9(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_BODY9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_KEY_CODE11(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_KEY_CODE11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_BODY10(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_BODY10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_KEY_CODE12(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_KEY_CODE12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_BODY11(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_BODY11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn USER_KEK_KEY_CODE13(
        self,
    ) -> crate::common::Reg<regs::USER_KEK_KEY_CODE13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_HEADER0(self) -> crate::common::Reg<regs::UDS_HEADER0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_KEY_CODE0(self) -> crate::common::Reg<regs::UDS_KEY_CODE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_HEADER1(self) -> crate::common::Reg<regs::UDS_HEADER1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_KEY_CODE1(self) -> crate::common::Reg<regs::UDS_KEY_CODE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_BODY0(self) -> crate::common::Reg<regs::UDS_BODY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_KEY_CODE2(self) -> crate::common::Reg<regs::UDS_KEY_CODE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_BODY1(self) -> crate::common::Reg<regs::UDS_BODY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_KEY_CODE3(self) -> crate::common::Reg<regs::UDS_KEY_CODE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_BODY2(self) -> crate::common::Reg<regs::UDS_BODY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0530usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_KEY_CODE4(self) -> crate::common::Reg<regs::UDS_KEY_CODE4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0530usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_BODY3(self) -> crate::common::Reg<regs::UDS_BODY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_KEY_CODE5(self) -> crate::common::Reg<regs::UDS_KEY_CODE5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_BODY4(self) -> crate::common::Reg<regs::UDS_BODY4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0538usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_KEY_CODE6(self) -> crate::common::Reg<regs::UDS_KEY_CODE6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0538usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_BODY5(self) -> crate::common::Reg<regs::UDS_BODY5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x053cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_KEY_CODE7(self) -> crate::common::Reg<regs::UDS_KEY_CODE7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x053cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_BODY6(self) -> crate::common::Reg<regs::UDS_BODY6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_KEY_CODE8(self) -> crate::common::Reg<regs::UDS_KEY_CODE8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_BODY7(self) -> crate::common::Reg<regs::UDS_BODY7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_KEY_CODE9(self) -> crate::common::Reg<regs::UDS_KEY_CODE9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_BODY8(self) -> crate::common::Reg<regs::UDS_BODY8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_KEY_CODE10(
        self,
    ) -> crate::common::Reg<regs::UDS_KEY_CODE10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_BODY9(self) -> crate::common::Reg<regs::UDS_BODY9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x054cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_KEY_CODE11(
        self,
    ) -> crate::common::Reg<regs::UDS_KEY_CODE11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x054cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_BODY10(self) -> crate::common::Reg<regs::UDS_BODY10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0550usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_KEY_CODE12(
        self,
    ) -> crate::common::Reg<regs::UDS_KEY_CODE12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0550usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_BODY11(self) -> crate::common::Reg<regs::UDS_BODY11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0554usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn UDS_KEY_CODE13(
        self,
    ) -> crate::common::Reg<regs::UDS_KEY_CODE13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0554usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_HEADER0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_HEADER0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0558usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_KEY_CODE0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_KEY_CODE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0558usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_HEADER1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_HEADER1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x055cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_KEY_CODE1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_KEY_CODE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x055cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_BODY0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_BODY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_KEY_CODE2(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_KEY_CODE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_BODY1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_BODY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_KEY_CODE3(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_KEY_CODE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_BODY2(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_BODY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_KEY_CODE4(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_KEY_CODE4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_BODY3(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_BODY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_KEY_CODE5(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_KEY_CODE5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_BODY4(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_BODY4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0570usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_KEY_CODE6(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_KEY_CODE6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0570usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_BODY5(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_BODY5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0574usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_KEY_CODE7(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_KEY_CODE7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0574usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_BODY6(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_BODY6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0578usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_KEY_CODE8(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_KEY_CODE8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0578usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_BODY7(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_BODY7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x057cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_KEY_CODE9(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_KEY_CODE9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x057cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_BODY8(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_BODY8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_KEY_CODE10(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_KEY_CODE10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_BODY9(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_BODY9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_KEY_CODE11(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_KEY_CODE11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_BODY10(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_BODY10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_KEY_CODE12(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_KEY_CODE12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_BODY11(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_BODY11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x058cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_KEY_CODE13(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_KEY_CODE13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x058cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_HEADER0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_HEADER0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_KEY_CODE0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_KEY_CODE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_HEADER1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_HEADER1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0594usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_KEY_CODE1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_KEY_CODE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0594usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_BODY0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_BODY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0598usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_KEY_CODE2(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_KEY_CODE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0598usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_BODY1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_BODY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x059cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_KEY_CODE3(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_KEY_CODE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x059cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_BODY2(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_BODY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_KEY_CODE4(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_KEY_CODE4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_BODY3(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_BODY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_KEY_CODE5(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_KEY_CODE5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_BODY4(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_BODY4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_KEY_CODE6(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_KEY_CODE6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_BODY5(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_BODY5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05acusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_KEY_CODE7(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_KEY_CODE7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05acusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_BODY6(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_BODY6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_KEY_CODE8(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_KEY_CODE8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_BODY7(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_BODY7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_KEY_CODE9(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_KEY_CODE9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_BODY8(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_BODY8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_KEY_CODE10(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_KEY_CODE10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_BODY9(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_BODY9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05bcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_KEY_CODE11(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_KEY_CODE11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05bcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_BODY10(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_BODY10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_KEY_CODE12(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_KEY_CODE12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_BODY11(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_BODY11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_KEY_CODE13(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_KEY_CODE13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_HEADER0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_HEADER0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_KEY_CODE0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_KEY_CODE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_HEADER1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_HEADER1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ccusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_KEY_CODE1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_KEY_CODE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ccusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_BODY0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_BODY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_KEY_CODE2(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_KEY_CODE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_BODY1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_BODY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_KEY_CODE3(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_KEY_CODE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_BODY2(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_BODY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_KEY_CODE4(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_KEY_CODE4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_BODY3(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_BODY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05dcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_KEY_CODE5(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_KEY_CODE5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05dcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_BODY4(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_BODY4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_KEY_CODE6(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_KEY_CODE6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_BODY5(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_BODY5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_KEY_CODE7(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_KEY_CODE7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_BODY6(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_BODY6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_KEY_CODE8(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_KEY_CODE8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_BODY7(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_BODY7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ecusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_KEY_CODE9(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_KEY_CODE9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ecusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_BODY8(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_BODY8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_KEY_CODE10(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_KEY_CODE10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_BODY9(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_BODY9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_KEY_CODE11(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_KEY_CODE11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_BODY10(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_BODY10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_KEY_CODE12(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_KEY_CODE12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_BODY11(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_BODY11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05fcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_KEY_CODE13(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_KEY_CODE13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05fcusize) as _) }
    }
}
pub mod regs;
