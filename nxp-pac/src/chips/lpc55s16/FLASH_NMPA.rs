#[doc = "FLASH_NMPA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_NMPA {
    ptr: *mut u8,
}
unsafe impl Send for FLASH_NMPA {}
unsafe impl Sync for FLASH_NMPA {}
impl FLASH_NMPA {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPO0 register 0 description."]
    #[inline(always)]
    pub const fn GPO0_0(self) -> crate::common::Reg<regs::GPO0_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "GPO0 array description."]
    #[inline(always)]
    pub const fn GPO0_ARRAY0(self) -> crate::common::Reg<regs::GPO0_ARRAY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "GPO0 register 1 description."]
    #[inline(always)]
    pub const fn GPO0_1(self) -> crate::common::Reg<regs::GPO0_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "GPO0 array description."]
    #[inline(always)]
    pub const fn GPO0_ARRAY1(self) -> crate::common::Reg<regs::GPO0_ARRAY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "GPO0 register 2 description."]
    #[inline(always)]
    pub const fn GPO0_2(self) -> crate::common::Reg<regs::GPO0_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "GPO0 array description."]
    #[inline(always)]
    pub const fn GPO0_ARRAY2(self) -> crate::common::Reg<regs::GPO0_ARRAY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "GPO0 register 3 description."]
    #[inline(always)]
    pub const fn GPO0_3(self) -> crate::common::Reg<regs::GPO0_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "GPO0 array description."]
    #[inline(always)]
    pub const fn GPO0_ARRAY3(self) -> crate::common::Reg<regs::GPO0_ARRAY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "GPO1 register 0 description."]
    #[inline(always)]
    pub const fn GPO1_0(self) -> crate::common::Reg<regs::GPO1_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "GPO1 array description."]
    #[inline(always)]
    pub const fn GPO1_ARRAY0(self) -> crate::common::Reg<regs::GPO1_ARRAY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "GPO1 register 1 description."]
    #[inline(always)]
    pub const fn GPO1_1(self) -> crate::common::Reg<regs::GPO1_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "GPO1 array description."]
    #[inline(always)]
    pub const fn GPO1_ARRAY1(self) -> crate::common::Reg<regs::GPO1_ARRAY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "GPO1 register 2 description."]
    #[inline(always)]
    pub const fn GPO1_2(self) -> crate::common::Reg<regs::GPO1_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "GPO1 array description."]
    #[inline(always)]
    pub const fn GPO1_ARRAY2(self) -> crate::common::Reg<regs::GPO1_ARRAY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "GPO1 register 3 description."]
    #[inline(always)]
    pub const fn GPO1_3(self) -> crate::common::Reg<regs::GPO1_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "GPO1 array description."]
    #[inline(always)]
    pub const fn GPO1_ARRAY3(self) -> crate::common::Reg<regs::GPO1_ARRAY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "GPO2 register 0 description."]
    #[inline(always)]
    pub const fn GPO2_0(self) -> crate::common::Reg<regs::GPO2_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "GPO2 array description."]
    #[inline(always)]
    pub const fn GPO2_ARRAY0(self) -> crate::common::Reg<regs::GPO2_ARRAY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "GPO2 register 1 description."]
    #[inline(always)]
    pub const fn GPO2_1(self) -> crate::common::Reg<regs::GPO2_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "GPO2 array description."]
    #[inline(always)]
    pub const fn GPO2_ARRAY1(self) -> crate::common::Reg<regs::GPO2_ARRAY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "GPO2 register 2 description."]
    #[inline(always)]
    pub const fn GPO2_2(self) -> crate::common::Reg<regs::GPO2_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "GPO2 array description."]
    #[inline(always)]
    pub const fn GPO2_ARRAY2(self) -> crate::common::Reg<regs::GPO2_ARRAY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "GPO2 register 3 description."]
    #[inline(always)]
    pub const fn GPO2_3(self) -> crate::common::Reg<regs::GPO2_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "GPO2 array description."]
    #[inline(always)]
    pub const fn GPO2_ARRAY3(self) -> crate::common::Reg<regs::GPO2_ARRAY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "GPO3 register 0 description."]
    #[inline(always)]
    pub const fn GPO3_0(self) -> crate::common::Reg<regs::GPO3_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "GPO3 array description."]
    #[inline(always)]
    pub const fn GPO3_ARRAY0(self) -> crate::common::Reg<regs::GPO3_ARRAY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "GPO3 register 1 description."]
    #[inline(always)]
    pub const fn GPO3_1(self) -> crate::common::Reg<regs::GPO3_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "GPO3 array description."]
    #[inline(always)]
    pub const fn GPO3_ARRAY1(self) -> crate::common::Reg<regs::GPO3_ARRAY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "GPO3 register 2 description."]
    #[inline(always)]
    pub const fn GPO3_2(self) -> crate::common::Reg<regs::GPO3_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "GPO3 array description."]
    #[inline(always)]
    pub const fn GPO3_ARRAY2(self) -> crate::common::Reg<regs::GPO3_ARRAY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "GPO3 register 3 description."]
    #[inline(always)]
    pub const fn GPO3_3(self) -> crate::common::Reg<regs::GPO3_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "GPO3 array description."]
    #[inline(always)]
    pub const fn GPO3_ARRAY3(self) -> crate::common::Reg<regs::GPO3_ARRAY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "checksum of the GPO data in words 0."]
    #[inline(always)]
    pub const fn GPO_CHECKSUM_0(
        self,
    ) -> crate::common::Reg<regs::GPO_CHECKSUM_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "checksum of the GPO data in words \\[3:0\\]."]
    #[inline(always)]
    pub const fn GPO_CHECKSUM_ARRAY0(
        self,
    ) -> crate::common::Reg<regs::GPO_CHECKSUM_ARRAY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "checksum of the GPO data in words 1."]
    #[inline(always)]
    pub const fn GPO_CHECKSUM_1(
        self,
    ) -> crate::common::Reg<regs::GPO_CHECKSUM_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "checksum of the GPO data in words \\[3:0\\]."]
    #[inline(always)]
    pub const fn GPO_CHECKSUM_ARRAY1(
        self,
    ) -> crate::common::Reg<regs::GPO_CHECKSUM_ARRAY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "checksum of the GPO data in words 2."]
    #[inline(always)]
    pub const fn GPO_CHECKSUM_2(
        self,
    ) -> crate::common::Reg<regs::GPO_CHECKSUM_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "checksum of the GPO data in words \\[3:0\\]."]
    #[inline(always)]
    pub const fn GPO_CHECKSUM_ARRAY2(
        self,
    ) -> crate::common::Reg<regs::GPO_CHECKSUM_ARRAY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "checksum of the GPO data in words 3."]
    #[inline(always)]
    pub const fn GPO_CHECKSUM_3(
        self,
    ) -> crate::common::Reg<regs::GPO_CHECKSUM_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "checksum of the GPO data in words \\[3:0\\]."]
    #[inline(always)]
    pub const fn GPO_CHECKSUM_ARRAY3(
        self,
    ) -> crate::common::Reg<regs::GPO_CHECKSUM_ARRAY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn FINAL_TEST_BATCH_ID_0(
        self,
    ) -> crate::common::Reg<regs::FINAL_TEST_BATCH_ID_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn FINAL_TEST_BATCH_ID_ARRAY0(
        self,
    ) -> crate::common::Reg<regs::FINAL_TEST_BATCH_ID_ARRAY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn FINAL_TEST_BATCH_ID_1(
        self,
    ) -> crate::common::Reg<regs::FINAL_TEST_BATCH_ID_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn FINAL_TEST_BATCH_ID_ARRAY1(
        self,
    ) -> crate::common::Reg<regs::FINAL_TEST_BATCH_ID_ARRAY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn FINAL_TEST_BATCH_ID_2(
        self,
    ) -> crate::common::Reg<regs::FINAL_TEST_BATCH_ID_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn FINAL_TEST_BATCH_ID_ARRAY2(
        self,
    ) -> crate::common::Reg<regs::FINAL_TEST_BATCH_ID_ARRAY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn FINAL_TEST_BATCH_ID_3(
        self,
    ) -> crate::common::Reg<regs::FINAL_TEST_BATCH_ID_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn FINAL_TEST_BATCH_ID_ARRAY3(
        self,
    ) -> crate::common::Reg<regs::FINAL_TEST_BATCH_ID_ARRAY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn DEVICE_TYPE(self) -> crate::common::Reg<regs::DEVICE_TYPE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn FINAL_TEST_PROGRAM_VERSION(
        self,
    ) -> crate::common::Reg<regs::FINAL_TEST_PROGRAM_VERSION, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn FINAL_TEST_DATE(
        self,
    ) -> crate::common::Reg<regs::FINAL_TEST_DATE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn FINAL_TEST_TIME(
        self,
    ) -> crate::common::Reg<regs::FINAL_TEST_TIME, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn UUID_0(self) -> crate::common::Reg<regs::UUID_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn UUID_ARRAY0(self) -> crate::common::Reg<regs::UUID_ARRAY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn UUID_1(self) -> crate::common::Reg<regs::UUID_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn UUID_ARRAY1(self) -> crate::common::Reg<regs::UUID_ARRAY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn UUID_2(self) -> crate::common::Reg<regs::UUID_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn UUID_ARRAY2(self) -> crate::common::Reg<regs::UUID_ARRAY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn UUID_3(self) -> crate::common::Reg<regs::UUID_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn UUID_ARRAY3(self) -> crate::common::Reg<regs::UUID_ARRAY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn WAFER_TEST1_PROGRAM_VERSION(
        self,
    ) -> crate::common::Reg<regs::WAFER_TEST1_PROGRAM_VERSION, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn WAFER_TEST1_DATE(
        self,
    ) -> crate::common::Reg<regs::WAFER_TEST1_DATE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn WAFER_TEST1_TIME(
        self,
    ) -> crate::common::Reg<regs::WAFER_TEST1_TIME, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn WAFER_TEST2_PROGRAM_VERSION(
        self,
    ) -> crate::common::Reg<regs::WAFER_TEST2_PROGRAM_VERSION, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn WAFER_TEST2_DATE(
        self,
    ) -> crate::common::Reg<regs::WAFER_TEST2_DATE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn WAFER_TEST2_TIME(
        self,
    ) -> crate::common::Reg<regs::WAFER_TEST2_TIME, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn USBCFG(self) -> crate::common::Reg<regs::USBCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PERIPHENCFG(self) -> crate::common::Reg<regs::PERIPHENCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn RAMSIZECFG(self) -> crate::common::Reg<regs::RAMSIZECFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn FLASHSIZECFG(self) -> crate::common::Reg<regs::FLASHSIZECFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn RINGO_0(self) -> crate::common::Reg<regs::RINGO_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn RINGO_1(self) -> crate::common::Reg<regs::RINGO_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn RINGO_2(self) -> crate::common::Reg<regs::RINGO_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn FRO_192MHZ(self) -> crate::common::Reg<regs::FRO_192MHZ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn XO_32MHZ(self) -> crate::common::Reg<regs::XO_32MHZ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn XO_32KHZ(self) -> crate::common::Reg<regs::XO_32KHZ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn FRO_1MHZ(self) -> crate::common::Reg<regs::FRO_1MHZ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn DCDC_POWER_PROFILE_HIGH_0(
        self,
    ) -> crate::common::Reg<regs::DCDC_POWER_PROFILE_HIGH_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn DCDC_POWER_PROFILE_HIGH_ARRAY0(
        self,
    ) -> crate::common::Reg<regs::DCDC_POWER_PROFILE_HIGH_ARRAY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn DCDC_POWER_PROFILE_HIGH_1(
        self,
    ) -> crate::common::Reg<regs::DCDC_POWER_PROFILE_HIGH_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn DCDC_POWER_PROFILE_HIGH_ARRAY1(
        self,
    ) -> crate::common::Reg<regs::DCDC_POWER_PROFILE_HIGH_ARRAY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn DCDC_POWER_PROFILE_LOW_0(
        self,
    ) -> crate::common::Reg<regs::DCDC_POWER_PROFILE_LOW_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn DCDC_POWER_PROFILE_LOW_ARRAY0(
        self,
    ) -> crate::common::Reg<regs::DCDC_POWER_PROFILE_LOW_ARRAY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn DCDC_POWER_PROFILE_LOW_1(
        self,
    ) -> crate::common::Reg<regs::DCDC_POWER_PROFILE_LOW_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn DCDC_POWER_PROFILE_LOW_ARRAY1(
        self,
    ) -> crate::common::Reg<regs::DCDC_POWER_PROFILE_LOW_ARRAY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn DCDC_POWER_PROFILE_MEDIUM_0(
        self,
    ) -> crate::common::Reg<regs::DCDC_POWER_PROFILE_MEDIUM_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn DCDC_POWER_PROFILE_MEDIUM_ARRAY0(
        self,
    ) -> crate::common::Reg<regs::DCDC_POWER_PROFILE_MEDIUM_ARRAY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn DCDC_POWER_PROFILE_MEDIUM_1(
        self,
    ) -> crate::common::Reg<regs::DCDC_POWER_PROFILE_MEDIUM_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn DCDC_POWER_PROFILE_MEDIUM_ARRAY1(
        self,
    ) -> crate::common::Reg<regs::DCDC_POWER_PROFILE_MEDIUM_ARRAY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn BOD(self) -> crate::common::Reg<regs::BOD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn LDO_AO(self) -> crate::common::Reg<regs::LDO_AO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn SDIO_DELAY(self) -> crate::common::Reg<regs::SDIO_DELAY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn AUX_BIAS_CURVE_AMBIENT_0(
        self,
    ) -> crate::common::Reg<regs::AUX_BIAS_CURVE_AMBIENT_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Aux Bias Curve Ambient (30degC)."]
    #[inline(always)]
    pub const fn AUX_BIAS_CURVE_AMBIENT_ARRAY0(
        self,
    ) -> crate::common::Reg<regs::AUX_BIAS_CURVE_AMBIENT_ARRAY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn AUX_BIAS_CURVE_AMBIENT_1(
        self,
    ) -> crate::common::Reg<regs::AUX_BIAS_CURVE_AMBIENT_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Aux Bias Curve Ambient (30degC)."]
    #[inline(always)]
    pub const fn AUX_BIAS_CURVE_AMBIENT_ARRAY1(
        self,
    ) -> crate::common::Reg<regs::AUX_BIAS_CURVE_AMBIENT_ARRAY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn AUX_BIAS_CURVE_AMBIENT_2(
        self,
    ) -> crate::common::Reg<regs::AUX_BIAS_CURVE_AMBIENT_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Aux Bias Curve Ambient (30degC)."]
    #[inline(always)]
    pub const fn AUX_BIAS_CURVE_AMBIENT_ARRAY2(
        self,
    ) -> crate::common::Reg<regs::AUX_BIAS_CURVE_AMBIENT_ARRAY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn AUX_BIAS_CURVE_AMBIENT_3(
        self,
    ) -> crate::common::Reg<regs::AUX_BIAS_CURVE_AMBIENT_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Aux Bias Curve Ambient (30degC)."]
    #[inline(always)]
    pub const fn AUX_BIAS_CURVE_AMBIENT_ARRAY3(
        self,
    ) -> crate::common::Reg<regs::AUX_BIAS_CURVE_AMBIENT_ARRAY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn AUX_BIAS_CURVE_TEMP_0(
        self,
    ) -> crate::common::Reg<regs::AUX_BIAS_CURVE_TEMP_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Aux Bias Curve TEMP (105degC)."]
    #[inline(always)]
    pub const fn AUX_BIAS_CURVE_TEMP_ARRAY0(
        self,
    ) -> crate::common::Reg<regs::AUX_BIAS_CURVE_TEMP_ARRAY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn AUX_BIAS_CURVE_TEMP_1(
        self,
    ) -> crate::common::Reg<regs::AUX_BIAS_CURVE_TEMP_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Aux Bias Curve TEMP (105degC)."]
    #[inline(always)]
    pub const fn AUX_BIAS_CURVE_TEMP_ARRAY1(
        self,
    ) -> crate::common::Reg<regs::AUX_BIAS_CURVE_TEMP_ARRAY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn AUX_BIAS_CURVE_TEMP_2(
        self,
    ) -> crate::common::Reg<regs::AUX_BIAS_CURVE_TEMP_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Aux Bias Curve TEMP (105degC)."]
    #[inline(always)]
    pub const fn AUX_BIAS_CURVE_TEMP_ARRAY2(
        self,
    ) -> crate::common::Reg<regs::AUX_BIAS_CURVE_TEMP_ARRAY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn AUX_BIAS_CURVE_TEMP_3(
        self,
    ) -> crate::common::Reg<regs::AUX_BIAS_CURVE_TEMP_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Aux Bias Curve TEMP (105degC)."]
    #[inline(always)]
    pub const fn AUX_BIAS_CURVE_TEMP_ARRAY3(
        self,
    ) -> crate::common::Reg<regs::AUX_BIAS_CURVE_TEMP_ARRAY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn TEMP_SENS_VBE1VBE8_REF_1(
        self,
    ) -> crate::common::Reg<regs::TEMP_SENS_VBE1VBE8_REF_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn TEMP_SENS_VBE1VBE8_REF_2(
        self,
    ) -> crate::common::Reg<regs::TEMP_SENS_VBE1VBE8_REF_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn TEMP_SENS_SLOPE(
        self,
    ) -> crate::common::Reg<regs::TEMP_SENS_SLOPE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn TEMP_SENS_OFFSET(
        self,
    ) -> crate::common::Reg<regs::TEMP_SENS_OFFSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PVT_MONITOR_0_ARRAY0(
        self,
    ) -> crate::common::Reg<regs::PVT_MONITOR_0_ARRAY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PVT_MONITOR_0_RINGO(
        self,
    ) -> crate::common::Reg<regs::PVT_MONITOR_0_RINGO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PVT_MONITOR_0_ARRAY1(
        self,
    ) -> crate::common::Reg<regs::PVT_MONITOR_0_ARRAY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PVT_MONITOR_0_DELAYS_LSB(
        self,
    ) -> crate::common::Reg<regs::PVT_MONITOR_0_DELAYS_LSB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PVT_MONITOR_0_ARRAY2(
        self,
    ) -> crate::common::Reg<regs::PVT_MONITOR_0_ARRAY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PVT_MONITOR_0_DELAYS_MSB(
        self,
    ) -> crate::common::Reg<regs::PVT_MONITOR_0_DELAYS_MSB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PVT_MONITOR_1_ARRAY0(
        self,
    ) -> crate::common::Reg<regs::PVT_MONITOR_1_ARRAY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PVT_MONITOR_1_RINGO(
        self,
    ) -> crate::common::Reg<regs::PVT_MONITOR_1_RINGO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PVT_MONITOR_1_ARRAY1(
        self,
    ) -> crate::common::Reg<regs::PVT_MONITOR_1_ARRAY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PVT_MONITOR_1_DELAYS_LSB(
        self,
    ) -> crate::common::Reg<regs::PVT_MONITOR_1_DELAYS_LSB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PVT_MONITOR_1_ARRAY2(
        self,
    ) -> crate::common::Reg<regs::PVT_MONITOR_1_ARRAY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PVT_MONITOR_1_DELAYS_MSB(
        self,
    ) -> crate::common::Reg<regs::PVT_MONITOR_1_DELAYS_MSB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn NXP_DEVICE_PRIVATE_KEY(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::NXP_DEVICE_PRIVATE_KEY, crate::common::RW> {
        assert!(n < 13usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize + n * 4usize) as _)
        }
    }
    #[doc = "NXP Device Certificate (ECDSA_sign - r\\[255:128\\])."]
    #[inline(always)]
    pub const fn NXP_DEVICE_CERTIFICATE_0(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::NXP_DEVICE_CERTIFICATE_0, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize + n * 4usize) as _)
        }
    }
    #[doc = "NXP Device Certificate (ECDSA_sign - r\\[127:0\\])."]
    #[inline(always)]
    pub const fn NXP_DEVICE_CERTIFICATE_1(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::NXP_DEVICE_CERTIFICATE_1, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize + n * 4usize) as _)
        }
    }
    #[doc = "NXP Device Certificate (ECDSA_sign - s\\[255:128\\])."]
    #[inline(always)]
    pub const fn NXP_DEVICE_CERTIFICATE_2(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::NXP_DEVICE_CERTIFICATE_2, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize + n * 4usize) as _)
        }
    }
    #[doc = "NXP Device Certificate (ECDSA_sign - s\\[127:0\\])."]
    #[inline(always)]
    pub const fn NXP_DEVICE_CERTIFICATE_3(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::NXP_DEVICE_CERTIFICATE_3, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize + n * 4usize) as _)
        }
    }
    #[doc = "SHA-256 DIGEST (9EC00 - 9FDBC) ROM Patch Area + NXP Area (IMPORTANT NOTE: Pages used for Repair (N-8 to N-3) are excluded from the computation) SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]."]
    #[inline(always)]
    pub const fn SHA256_DIGEST(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SHA256_DIGEST, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn ECID_BACKUP_0(self) -> crate::common::Reg<regs::ECID_BACKUP_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "ECID backup (the original is in page n-1)."]
    #[inline(always)]
    pub const fn ECID_BACKUP_ARRAY0(
        self,
    ) -> crate::common::Reg<regs::ECID_BACKUP_ARRAY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn ECID_BACKUP_1(self) -> crate::common::Reg<regs::ECID_BACKUP_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "ECID backup (the original is in page n-1)."]
    #[inline(always)]
    pub const fn ECID_BACKUP_ARRAY1(
        self,
    ) -> crate::common::Reg<regs::ECID_BACKUP_ARRAY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn ECID_BACKUP_2(self) -> crate::common::Reg<regs::ECID_BACKUP_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "ECID backup (the original is in page n-1)."]
    #[inline(always)]
    pub const fn ECID_BACKUP_ARRAY2(
        self,
    ) -> crate::common::Reg<regs::ECID_BACKUP_ARRAY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn ECID_BACKUP_3(self) -> crate::common::Reg<regs::ECID_BACKUP_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "ECID backup (the original is in page n-1)."]
    #[inline(always)]
    pub const fn ECID_BACKUP_ARRAY3(
        self,
    ) -> crate::common::Reg<regs::ECID_BACKUP_ARRAY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "Checksum of the whole page."]
    #[inline(always)]
    pub const fn CHECKSUM(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn DIS_ROM_HIDING(
        self,
    ) -> crate::common::Reg<regs::DIS_ROM_HIDING, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cacusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PUF_SRAM(self) -> crate::common::Reg<regs::PUF_SRAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cbcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
