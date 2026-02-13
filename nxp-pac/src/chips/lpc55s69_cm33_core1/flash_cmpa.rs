#[doc = "FLASH_CMPA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashCmpa {
    ptr: *mut u8,
}
unsafe impl Send for FlashCmpa {}
unsafe impl Sync for FlashCmpa {}
impl FlashCmpa {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn boot_cfg(self) -> crate::common::Reg<regs::BootCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn spi_flash_cfg(self) -> crate::common::Reg<regs::SpiFlashCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn usb_id(self) -> crate::common::Reg<regs::UsbId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn sdio_cfg(self) -> crate::common::Reg<regs::SdioCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn cc_socu_pin(self) -> crate::common::Reg<regs::CcSocuPin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn cc_socu_dflt(self) -> crate::common::Reg<regs::CcSocuDflt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn vendor_usage(self) -> crate::common::Reg<regs::VendorUsage, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Secure boot configuration flags."]
    #[inline(always)]
    pub const fn secure_boot_cfg(
        self,
    ) -> crate::common::Reg<regs::SecureBootCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_base_addr(
        self,
    ) -> crate::common::Reg<regs::PrinceBaseAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Region 0, sub-region enable"]
    #[inline(always)]
    pub const fn prince_sr_0(self) -> crate::common::Reg<regs::PrinceSr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Region 1, sub-region enable"]
    #[inline(always)]
    pub const fn prince_sr_1(self) -> crate::common::Reg<regs::PrinceSr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Region 2, sub-region enable"]
    #[inline(always)]
    pub const fn prince_sr_2(self) -> crate::common::Reg<regs::PrinceSr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Xtal 32kHz capabank triming."]
    #[inline(always)]
    pub const fn xtal_32khz_capabank_trim(
        self,
    ) -> crate::common::Reg<regs::Xtal32khzCapabankTrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Xtal 16MHz capabank triming."]
    #[inline(always)]
    pub const fn xtal_16mhz_capabank_trim(
        self,
    ) -> crate::common::Reg<regs::Xtal16mhzCapabankTrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "ROTKHindex for Root of Trust Keys Table hash\\[(((7 - index) * 32) + 31):((7 - index) * 32)\\]"]
    #[inline(always)]
    pub const fn rotkh(self, n: usize) -> crate::common::Reg<regs::Rotkh, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize + n * 4usize) as _) }
    }
    #[doc = "Customer Defined (Programable through ROM API)"]
    #[inline(always)]
    pub const fn customer_defined(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::CustomerDefined, crate::common::RW> {
        assert!(n < 56usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
    #[inline(always)]
    pub const fn sha256_digest(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Sha256Digest, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
