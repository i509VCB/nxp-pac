#[doc = "FLASH_CMPA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_CMPA {
    ptr: *mut u8,
}
unsafe impl Send for FLASH_CMPA {}
unsafe impl Sync for FLASH_CMPA {}
impl FLASH_CMPA {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn BOOT_CFG(self) -> crate::common::Reg<regs::BOOT_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn SPI_FLASH_CFG(self) -> crate::common::Reg<regs::SPI_FLASH_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn USB_ID(self) -> crate::common::Reg<regs::USB_ID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn SDIO_CFG(self) -> crate::common::Reg<regs::SDIO_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn CC_SOCU_PIN(self) -> crate::common::Reg<regs::CC_SOCU_PIN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn CC_SOCU_DFLT(self) -> crate::common::Reg<regs::CC_SOCU_DFLT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn VENDOR_USAGE(self) -> crate::common::Reg<regs::VENDOR_USAGE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Secure boot configuration flags."]
    #[inline(always)]
    pub const fn SECURE_BOOT_CFG(
        self,
    ) -> crate::common::Reg<regs::SECURE_BOOT_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_BASE_ADDR(
        self,
    ) -> crate::common::Reg<regs::PRINCE_BASE_ADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Region 0, sub-region enable."]
    #[inline(always)]
    pub const fn PRINCE_SR_0(self) -> crate::common::Reg<regs::PRINCE_SR_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Region 1, sub-region enable."]
    #[inline(always)]
    pub const fn PRINCE_SR_1(self) -> crate::common::Reg<regs::PRINCE_SR_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Region 2, sub-region enable."]
    #[inline(always)]
    pub const fn PRINCE_SR_2(self) -> crate::common::Reg<regs::PRINCE_SR_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Xtal 32kHz capabank triming."]
    #[inline(always)]
    pub const fn XTAL_32KHZ_CAPABANK_TRIM(
        self,
    ) -> crate::common::Reg<regs::XTAL_32KHZ_CAPABANK_TRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Xtal 16MHz capabank triming."]
    #[inline(always)]
    pub const fn XTAL_16MHZ_CAPABANK_TRIM(
        self,
    ) -> crate::common::Reg<regs::XTAL_16MHZ_CAPABANK_TRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "This 32-bit register contains the size of the image to remap, in bytes. The 12 LSBs are ignored, so the size granularity is 4KB."]
    #[inline(always)]
    pub const fn FLASH_REMAP_SIZE(
        self,
    ) -> crate::common::Reg<regs::FLASH_REMAP_SIZE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "This 32-bit register contains the offset by which the image is to be remapped. The 12 LSBs are ignored, so the remap granularity is 4KB."]
    #[inline(always)]
    pub const fn FLASH_REMAP_OFFSET(
        self,
    ) -> crate::common::Reg<regs::FLASH_REMAP_OFFSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "ROTKHindex for Root of Trust Keys Table hash\\[(((7 - index) * 32) + 31):((7 - index) * 32)\\]."]
    #[inline(always)]
    pub const fn ROTKH(self, n: usize) -> crate::common::Reg<regs::ROTKH, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize + n * 4usize) as _) }
    }
    #[doc = "Customer Defined (Programable through ROM API)."]
    #[inline(always)]
    pub const fn CUSTOMER_DEFINED(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::CUSTOMER_DEFINED, crate::common::RW> {
        assert!(n < 56usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]."]
    #[inline(always)]
    pub const fn SHA256_DIGEST(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SHA256_DIGEST, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
