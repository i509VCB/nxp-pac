#[doc = "LPC-Next0 OTP controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ocotp {
    ptr: *mut u8,
}
unsafe impl Send for Ocotp {}
unsafe impl Sync for Ocotp {}
impl Ocotp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "OTP shadow register N"]
    #[inline(always)]
    pub const fn otp_shadow(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::OtpShadow, crate::common::RW> {
        assert!(n < 496usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Control/address register"]
    #[inline(always)]
    pub const fn otp_ctrl(self) -> crate::common::Reg<regs::OtpCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "Power-down register"]
    #[inline(always)]
    pub const fn otp_pdn(self) -> crate::common::Reg<regs::OtpPdn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "OTP programming data register"]
    #[inline(always)]
    pub const fn otp_write_data(self) -> crate::common::Reg<regs::OtpWriteData, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0808usize) as _) }
    }
    #[doc = "OTP read start register"]
    #[inline(always)]
    pub const fn otp_read_ctrl(self) -> crate::common::Reg<regs::OtpReadCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
    #[doc = "OTP read data register"]
    #[inline(always)]
    pub const fn otp_read_data(self) -> crate::common::Reg<regs::OtpReadData, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0810usize) as _) }
    }
    #[doc = "OTP clock divider register"]
    #[inline(always)]
    pub const fn otp_clk_div(self) -> crate::common::Reg<regs::OtpClkDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0814usize) as _) }
    }
    #[doc = "CRC address range register"]
    #[inline(always)]
    pub const fn otp_crc_addr(self) -> crate::common::Reg<regs::OtpCrcAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x081cusize) as _) }
    }
    #[doc = "CRC result register"]
    #[inline(always)]
    pub const fn otp_crc_value(self) -> crate::common::Reg<regs::OtpCrcValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0820usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn otp_status(self) -> crate::common::Reg<regs::OtpStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0824usize) as _) }
    }
    #[doc = "VERSION ID register"]
    #[inline(always)]
    pub const fn otp_version(self) -> crate::common::Reg<regs::OtpVersion, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x082cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
