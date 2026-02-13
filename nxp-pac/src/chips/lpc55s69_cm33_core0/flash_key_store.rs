#[doc = "FLASH_KEY_STORE"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashKeyStore {
    ptr: *mut u8,
}
unsafe impl Send for FlashKeyStore {}
unsafe impl Sync for FlashKeyStore {}
impl FlashKeyStore {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Valid Key Sore Header : 0x95959595"]
    #[inline(always)]
    pub const fn header(self) -> crate::common::Reg<regs::Header, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "puf discharge time in ms."]
    #[inline(always)]
    pub const fn puf_discharge_time_in_ms(
        self,
    ) -> crate::common::Reg<regs::PufDischargeTimeInMs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn activation_code(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ActivationCode, crate::common::RW> {
        assert!(n < 298usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_header0(self) -> crate::common::Reg<regs::SbkeyHeader0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_key_code0(
        self,
    ) -> crate::common::Reg<regs::SbkeyKeyCode0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_header1(self) -> crate::common::Reg<regs::SbkeyHeader1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_key_code1(
        self,
    ) -> crate::common::Reg<regs::SbkeyKeyCode1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_body0(self) -> crate::common::Reg<regs::SbkeyBody0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_key_code2(
        self,
    ) -> crate::common::Reg<regs::SbkeyKeyCode2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_body1(self) -> crate::common::Reg<regs::SbkeyBody1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04bcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_key_code3(
        self,
    ) -> crate::common::Reg<regs::SbkeyKeyCode3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04bcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_body2(self) -> crate::common::Reg<regs::SbkeyBody2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_key_code4(
        self,
    ) -> crate::common::Reg<regs::SbkeyKeyCode4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_body3(self) -> crate::common::Reg<regs::SbkeyBody3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_key_code5(
        self,
    ) -> crate::common::Reg<regs::SbkeyKeyCode5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_body4(self) -> crate::common::Reg<regs::SbkeyBody4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_key_code6(
        self,
    ) -> crate::common::Reg<regs::SbkeyKeyCode6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_body5(self) -> crate::common::Reg<regs::SbkeyBody5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ccusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_key_code7(
        self,
    ) -> crate::common::Reg<regs::SbkeyKeyCode7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ccusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_body6(self) -> crate::common::Reg<regs::SbkeyBody6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_key_code8(
        self,
    ) -> crate::common::Reg<regs::SbkeyKeyCode8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_body7(self) -> crate::common::Reg<regs::SbkeyBody7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_key_code9(
        self,
    ) -> crate::common::Reg<regs::SbkeyKeyCode9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_body8(self) -> crate::common::Reg<regs::SbkeyBody8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_key_code10(
        self,
    ) -> crate::common::Reg<regs::SbkeyKeyCode10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_body9(self) -> crate::common::Reg<regs::SbkeyBody9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04dcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_key_code11(
        self,
    ) -> crate::common::Reg<regs::SbkeyKeyCode11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04dcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_body10(self) -> crate::common::Reg<regs::SbkeyBody10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_key_code12(
        self,
    ) -> crate::common::Reg<regs::SbkeyKeyCode12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_body11(self) -> crate::common::Reg<regs::SbkeyBody11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn sbkey_key_code13(
        self,
    ) -> crate::common::Reg<regs::SbkeyKeyCode13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_header0(
        self,
    ) -> crate::common::Reg<regs::UserKekHeader0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_key_code0(
        self,
    ) -> crate::common::Reg<regs::UserKekKeyCode0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_header1(
        self,
    ) -> crate::common::Reg<regs::UserKekHeader1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ecusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_key_code1(
        self,
    ) -> crate::common::Reg<regs::UserKekKeyCode1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ecusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_body0(self) -> crate::common::Reg<regs::UserKekBody0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_key_code2(
        self,
    ) -> crate::common::Reg<regs::UserKekKeyCode2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_body1(self) -> crate::common::Reg<regs::UserKekBody1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_key_code3(
        self,
    ) -> crate::common::Reg<regs::UserKekKeyCode3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_body2(self) -> crate::common::Reg<regs::UserKekBody2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_key_code4(
        self,
    ) -> crate::common::Reg<regs::UserKekKeyCode4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_body3(self) -> crate::common::Reg<regs::UserKekBody3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04fcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_key_code5(
        self,
    ) -> crate::common::Reg<regs::UserKekKeyCode5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04fcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_body4(self) -> crate::common::Reg<regs::UserKekBody4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_key_code6(
        self,
    ) -> crate::common::Reg<regs::UserKekKeyCode6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_body5(self) -> crate::common::Reg<regs::UserKekBody5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_key_code7(
        self,
    ) -> crate::common::Reg<regs::UserKekKeyCode7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_body6(self) -> crate::common::Reg<regs::UserKekBody6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_key_code8(
        self,
    ) -> crate::common::Reg<regs::UserKekKeyCode8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_body7(self) -> crate::common::Reg<regs::UserKekBody7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_key_code9(
        self,
    ) -> crate::common::Reg<regs::UserKekKeyCode9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_body8(self) -> crate::common::Reg<regs::UserKekBody8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_key_code10(
        self,
    ) -> crate::common::Reg<regs::UserKekKeyCode10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_body9(self) -> crate::common::Reg<regs::UserKekBody9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_key_code11(
        self,
    ) -> crate::common::Reg<regs::UserKekKeyCode11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_body10(
        self,
    ) -> crate::common::Reg<regs::UserKekBody10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_key_code12(
        self,
    ) -> crate::common::Reg<regs::UserKekKeyCode12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_body11(
        self,
    ) -> crate::common::Reg<regs::UserKekBody11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn user_kek_key_code13(
        self,
    ) -> crate::common::Reg<regs::UserKekKeyCode13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_header0(self) -> crate::common::Reg<regs::UdsHeader0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_key_code0(self) -> crate::common::Reg<regs::UdsKeyCode0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_header1(self) -> crate::common::Reg<regs::UdsHeader1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_key_code1(self) -> crate::common::Reg<regs::UdsKeyCode1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_body0(self) -> crate::common::Reg<regs::UdsBody0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_key_code2(self) -> crate::common::Reg<regs::UdsKeyCode2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_body1(self) -> crate::common::Reg<regs::UdsBody1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_key_code3(self) -> crate::common::Reg<regs::UdsKeyCode3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_body2(self) -> crate::common::Reg<regs::UdsBody2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0530usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_key_code4(self) -> crate::common::Reg<regs::UdsKeyCode4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0530usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_body3(self) -> crate::common::Reg<regs::UdsBody3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_key_code5(self) -> crate::common::Reg<regs::UdsKeyCode5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_body4(self) -> crate::common::Reg<regs::UdsBody4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0538usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_key_code6(self) -> crate::common::Reg<regs::UdsKeyCode6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0538usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_body5(self) -> crate::common::Reg<regs::UdsBody5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x053cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_key_code7(self) -> crate::common::Reg<regs::UdsKeyCode7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x053cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_body6(self) -> crate::common::Reg<regs::UdsBody6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_key_code8(self) -> crate::common::Reg<regs::UdsKeyCode8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_body7(self) -> crate::common::Reg<regs::UdsBody7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_key_code9(self) -> crate::common::Reg<regs::UdsKeyCode9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_body8(self) -> crate::common::Reg<regs::UdsBody8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_key_code10(self) -> crate::common::Reg<regs::UdsKeyCode10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_body9(self) -> crate::common::Reg<regs::UdsBody9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x054cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_key_code11(self) -> crate::common::Reg<regs::UdsKeyCode11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x054cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_body10(self) -> crate::common::Reg<regs::UdsBody10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0550usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_key_code12(self) -> crate::common::Reg<regs::UdsKeyCode12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0550usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_body11(self) -> crate::common::Reg<regs::UdsBody11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0554usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn uds_key_code13(self) -> crate::common::Reg<regs::UdsKeyCode13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0554usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_header0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0Header0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0558usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_key_code0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0KeyCode0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0558usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_header1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0Header1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x055cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_key_code1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0KeyCode1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x055cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_body0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0Body0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_key_code2(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0KeyCode2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_body1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0Body1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_key_code3(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0KeyCode3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_body2(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0Body2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_key_code4(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0KeyCode4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_body3(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0Body3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_key_code5(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0KeyCode5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_body4(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0Body4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0570usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_key_code6(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0KeyCode6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0570usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_body5(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0Body5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0574usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_key_code7(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0KeyCode7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0574usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_body6(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0Body6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0578usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_key_code8(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0KeyCode8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0578usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_body7(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0Body7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x057cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_key_code9(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0KeyCode9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x057cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_body8(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0Body8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_key_code10(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0KeyCode10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_body9(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0Body9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_key_code11(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0KeyCode11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_body10(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0Body10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_key_code12(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0KeyCode12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_body11(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0Body11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x058cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region0_key_code13(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0KeyCode13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x058cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_header0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1Header0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_key_code0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1KeyCode0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_header1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1Header1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0594usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_key_code1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1KeyCode1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0594usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_body0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1Body0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0598usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_key_code2(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1KeyCode2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0598usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_body1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1Body1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x059cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_key_code3(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1KeyCode3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x059cusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_body2(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1Body2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_key_code4(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1KeyCode4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_body3(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1Body3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_key_code5(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1KeyCode5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_body4(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1Body4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_key_code6(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1KeyCode6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_body5(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1Body5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05acusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_key_code7(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1KeyCode7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05acusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_body6(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1Body6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_key_code8(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1KeyCode8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_body7(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1Body7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_key_code9(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1KeyCode9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_body8(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1Body8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_key_code10(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1KeyCode10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_body9(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1Body9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05bcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_key_code11(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1KeyCode11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05bcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_body10(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1Body10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_key_code12(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1KeyCode12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_body11(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1Body11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region1_key_code13(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1KeyCode13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_header0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2Header0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_key_code0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2KeyCode0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_header1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2Header1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ccusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_key_code1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2KeyCode1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ccusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_body0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2Body0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_key_code2(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2KeyCode2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_body1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2Body1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_key_code3(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2KeyCode3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_body2(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2Body2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_key_code4(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2KeyCode4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_body3(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2Body3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05dcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_key_code5(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2KeyCode5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05dcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_body4(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2Body4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_key_code6(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2KeyCode6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_body5(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2Body5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_key_code7(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2KeyCode7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_body6(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2Body6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_key_code8(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2KeyCode8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_body7(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2Body7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ecusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_key_code9(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2KeyCode9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ecusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_body8(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2Body8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_key_code10(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2KeyCode10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_body9(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2Body9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_key_code11(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2KeyCode11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f4usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_body10(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2Body10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_key_code12(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2KeyCode12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f8usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_body11(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2Body11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05fcusize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn prince_region2_key_code13(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2KeyCode13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05fcusize) as _) }
    }
}
pub mod regs;
