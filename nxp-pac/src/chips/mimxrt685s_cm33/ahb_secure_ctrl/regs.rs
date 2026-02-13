#[doc = "Security access rules for AHB peripheral slaves area 0x40100000--0x4010FFFF"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeriph0SlaveRule0(pub u32);
impl AhbPeriph0SlaveRule0 {
    #[doc = "0x40100000--0x40103FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio_rule(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "0x40100000--0x40103FFF"]
    #[inline(always)]
    pub const fn set_hsgpio_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "0x40104000--0x40104FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_rule(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "0x40104000--0x40104FFF"]
    #[inline(always)]
    pub const fn set_dma0_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "0x40105000--0x40105FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_rule(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "0x40105000--0x40105FFF"]
    #[inline(always)]
    pub const fn set_dma1_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "0x40106000--0x40106FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm0_rule(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "0x40106000--0x40106FFF"]
    #[inline(always)]
    pub const fn set_flexcomm0_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "0x40107000--0x40107FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm1_rule(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "0x40107000--0x40107FFF"]
    #[inline(always)]
    pub const fn set_flexcomm1_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "0x40108000--0x40108FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm2_rule(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "0x40108000--0x40108FFF"]
    #[inline(always)]
    pub const fn set_flexcomm2_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "0x40109000--0x40109FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm3_rule(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "0x40109000--0x40109FFF"]
    #[inline(always)]
    pub const fn set_flexcomm3_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "0x4010F000--0x4010FFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn debug_mailbox_rule(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "0x4010F000--0x4010FFFF"]
    #[inline(always)]
    pub const fn set_debug_mailbox_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for AhbPeriph0SlaveRule0 {
    #[inline(always)]
    fn default() -> AhbPeriph0SlaveRule0 {
        AhbPeriph0SlaveRule0(0)
    }
}
impl core::fmt::Debug for AhbPeriph0SlaveRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeriph0SlaveRule0")
            .field("hsgpio_rule", &self.hsgpio_rule())
            .field("dma0_rule", &self.dma0_rule())
            .field("dma1_rule", &self.dma1_rule())
            .field("flexcomm0_rule", &self.flexcomm0_rule())
            .field("flexcomm1_rule", &self.flexcomm1_rule())
            .field("flexcomm2_rule", &self.flexcomm2_rule())
            .field("flexcomm3_rule", &self.flexcomm3_rule())
            .field("debug_mailbox_rule", &self.debug_mailbox_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeriph0SlaveRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeriph0SlaveRule0 {{ hsgpio_rule: {=u8:?}, dma0_rule: {=u8:?}, dma1_rule: {=u8:?}, flexcomm0_rule: {=u8:?}, flexcomm1_rule: {=u8:?}, flexcomm2_rule: {=u8:?}, flexcomm3_rule: {=u8:?}, debug_mailbox_rule: {=u8:?} }}",
            self.hsgpio_rule(),
            self.dma0_rule(),
            self.dma1_rule(),
            self.flexcomm0_rule(),
            self.flexcomm1_rule(),
            self.flexcomm2_rule(),
            self.flexcomm3_rule(),
            self.debug_mailbox_rule()
        )
    }
}
#[doc = "the memory map is 0x40120000--0x40127FFF"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeriph1SlaveRule0(pub u32);
impl AhbPeriph1SlaveRule0 {
    #[doc = "Security access rules for AHB peripheral slaves area 0x40120000--0x40120FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn crc_rule(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Security access rules for AHB peripheral slaves area 0x40120000--0x40120FFF"]
    #[inline(always)]
    pub const fn set_crc_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "0x40121000--0x40121FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn dmic_rule(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "0x40121000--0x40121FFF"]
    #[inline(always)]
    pub const fn set_dmic_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "0x40122000--0x40122FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm4_rule(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "0x40122000--0x40122FFF"]
    #[inline(always)]
    pub const fn set_flexcomm4_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "0x40123000--0x40123FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm5_rule(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "0x40123000--0x40123FFF"]
    #[inline(always)]
    pub const fn set_flexcomm5_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "0x40124000--0x40124FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm6_rule(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "0x40124000--0x40124FFF"]
    #[inline(always)]
    pub const fn set_flexcomm6_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "0x40125000--0x40125FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm7_rule(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "0x40125000--0x40125FFF"]
    #[inline(always)]
    pub const fn set_flexcomm7_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "0x40126000--0x40126FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm14_rule(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "0x40126000--0x40126FFF"]
    #[inline(always)]
    pub const fn set_flexcomm14_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "0x40127000--0x40127FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm15_rule(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "0x40127000--0x40127FFF"]
    #[inline(always)]
    pub const fn set_flexcomm15_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for AhbPeriph1SlaveRule0 {
    #[inline(always)]
    fn default() -> AhbPeriph1SlaveRule0 {
        AhbPeriph1SlaveRule0(0)
    }
}
impl core::fmt::Debug for AhbPeriph1SlaveRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeriph1SlaveRule0")
            .field("crc_rule", &self.crc_rule())
            .field("dmic_rule", &self.dmic_rule())
            .field("flexcomm4_rule", &self.flexcomm4_rule())
            .field("flexcomm5_rule", &self.flexcomm5_rule())
            .field("flexcomm6_rule", &self.flexcomm6_rule())
            .field("flexcomm7_rule", &self.flexcomm7_rule())
            .field("flexcomm14_rule", &self.flexcomm14_rule())
            .field("flexcomm15_rule", &self.flexcomm15_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeriph1SlaveRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeriph1SlaveRule0 {{ crc_rule: {=u8:?}, dmic_rule: {=u8:?}, flexcomm4_rule: {=u8:?}, flexcomm5_rule: {=u8:?}, flexcomm6_rule: {=u8:?}, flexcomm7_rule: {=u8:?}, flexcomm14_rule: {=u8:?}, flexcomm15_rule: {=u8:?} }}",
            self.crc_rule(),
            self.dmic_rule(),
            self.flexcomm4_rule(),
            self.flexcomm5_rule(),
            self.flexcomm6_rule(),
            self.flexcomm7_rule(),
            self.flexcomm14_rule(),
            self.flexcomm15_rule()
        )
    }
}
#[doc = "Security access rules for AHB peripheral slaves area 0x40140000--0x4014BFFF"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeriph2SlaveRule0(pub u32);
impl AhbPeriph2SlaveRule0 {
    #[doc = "0x40140000--0x40143FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs_ram_rule(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "0x40140000--0x40143FFF"]
    #[inline(always)]
    pub const fn set_usb_hs_ram_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "0x40144000--0x40144FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs_dev_rule(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "0x40144000--0x40144FFF"]
    #[inline(always)]
    pub const fn set_usb_hs_dev_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "0x40145000--0x40145FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs_host_rule(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "0x40145000--0x40145FFF"]
    #[inline(always)]
    pub const fn set_usb_hs_host_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "0x40146000--0x40146FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn sct_rule(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "0x40146000--0x40146FFF"]
    #[inline(always)]
    pub const fn set_sct_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for AhbPeriph2SlaveRule0 {
    #[inline(always)]
    fn default() -> AhbPeriph2SlaveRule0 {
        AhbPeriph2SlaveRule0(0)
    }
}
impl core::fmt::Debug for AhbPeriph2SlaveRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeriph2SlaveRule0")
            .field("usb_hs_ram_rule", &self.usb_hs_ram_rule())
            .field("usb_hs_dev_rule", &self.usb_hs_dev_rule())
            .field("usb_hs_host_rule", &self.usb_hs_host_rule())
            .field("sct_rule", &self.sct_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeriph2SlaveRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeriph2SlaveRule0 {{ usb_hs_ram_rule: {=u8:?}, usb_hs_dev_rule: {=u8:?}, usb_hs_host_rule: {=u8:?}, sct_rule: {=u8:?} }}",
            self.usb_hs_ram_rule(),
            self.usb_hs_dev_rule(),
            self.usb_hs_host_rule(),
            self.sct_rule()
        )
    }
}
#[doc = "Security access rules for AHB peripheral slaves area 0x40150000--0x40158FFF"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeriph3SlaveRule0(pub u32);
impl AhbPeriph3SlaveRule0 {
    #[doc = "0x40150000--0x40150FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn pq_copro_rule(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "0x40150000--0x40150FFF"]
    #[inline(always)]
    pub const fn set_pq_copro_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "0x40151000--0x40151FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn casper_copro_rule(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "0x40151000--0x40151FFF"]
    #[inline(always)]
    pub const fn set_casper_copro_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "0x40152000--0x40152FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn casper_ram_rule(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "0x40152000--0x40152FFF"]
    #[inline(always)]
    pub const fn set_casper_ram_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "0x40154000--0x40157FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn secure_gpio_rule(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "0x40154000--0x40157FFF"]
    #[inline(always)]
    pub const fn set_secure_gpio_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "0x40158000--0x40158FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn hash_rule(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "0x40158000--0x40158FFF"]
    #[inline(always)]
    pub const fn set_hash_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for AhbPeriph3SlaveRule0 {
    #[inline(always)]
    fn default() -> AhbPeriph3SlaveRule0 {
        AhbPeriph3SlaveRule0(0)
    }
}
impl core::fmt::Debug for AhbPeriph3SlaveRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeriph3SlaveRule0")
            .field("pq_copro_rule", &self.pq_copro_rule())
            .field("casper_copro_rule", &self.casper_copro_rule())
            .field("casper_ram_rule", &self.casper_ram_rule())
            .field("secure_gpio_rule", &self.secure_gpio_rule())
            .field("hash_rule", &self.hash_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeriph3SlaveRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeriph3SlaveRule0 {{ pq_copro_rule: {=u8:?}, casper_copro_rule: {=u8:?}, casper_ram_rule: {=u8:?}, secure_gpio_rule: {=u8:?}, hash_rule: {=u8:?} }}",
            self.pq_copro_rule(),
            self.casper_copro_rule(),
            self.casper_ram_rule(),
            self.secure_gpio_rule(),
            self.hash_rule()
        )
    }
}
#[doc = "0x40110000--0x4011FFFF"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridge0MemRule0(pub u32);
impl AipsBridge0MemRule0 {
    #[doc = "0x4011 0000--0x4011 0FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn mu0_m33_rule(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "0x4011 0000--0x4011 0FFF"]
    #[inline(always)]
    pub const fn set_mu0_m33_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "0x4011 1000--0x4011 1FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn mu0_dsp_rule(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "0x4011 1000--0x4011 1FFF"]
    #[inline(always)]
    pub const fn set_mu0_dsp_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "0x4011 2000--0x4011 2FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn semaphore_rule(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "0x4011 2000--0x4011 2FFF"]
    #[inline(always)]
    pub const fn set_semaphore_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "0x4011 3000--0x4011 3FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn os_event_timer_m33_rule(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "0x4011 3000--0x4011 3FFF"]
    #[inline(always)]
    pub const fn set_os_event_timer_m33_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "0x4011 4000--0x4011 4FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn os_event_timer_dsp_rule(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "0x4011 4000--0x4011 4FFF"]
    #[inline(always)]
    pub const fn set_os_event_timer_dsp_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for AipsBridge0MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridge0MemRule0 {
        AipsBridge0MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridge0MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridge0MemRule0")
            .field("mu0_m33_rule", &self.mu0_m33_rule())
            .field("mu0_dsp_rule", &self.mu0_dsp_rule())
            .field("semaphore_rule", &self.semaphore_rule())
            .field("os_event_timer_m33_rule", &self.os_event_timer_m33_rule())
            .field("os_event_timer_dsp_rule", &self.os_event_timer_dsp_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridge0MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridge0MemRule0 {{ mu0_m33_rule: {=u8:?}, mu0_dsp_rule: {=u8:?}, semaphore_rule: {=u8:?}, os_event_timer_m33_rule: {=u8:?}, os_event_timer_dsp_rule: {=u8:?} }}",
            self.mu0_m33_rule(),
            self.mu0_dsp_rule(),
            self.semaphore_rule(),
            self.os_event_timer_m33_rule(),
            self.os_event_timer_dsp_rule()
        )
    }
}
#[doc = "Security access rules for AIPS Bridge peripherals. Each AIPS bridge sector is 4 Kbytes, there're 16 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridge1MemRule0(pub u32);
impl AipsBridge1MemRule0 {
    #[doc = "0x4013 0000--0x4013 0FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn otp_rule0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "0x4013 0000--0x4013 0FFF"]
    #[inline(always)]
    pub const fn set_otp_rule0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "0x4013 1000--0x4013 1FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn otp_rule1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "0x4013 1000--0x4013 1FFF"]
    #[inline(always)]
    pub const fn set_otp_rule1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "0x4013 2000--0x4013 2FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn otp_rule2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "0x4013 2000--0x4013 2FFF"]
    #[inline(always)]
    pub const fn set_otp_rule2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "0x4013 3000--0x4013 3FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn otp_rule3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "0x4013 3000--0x4013 3FFF"]
    #[inline(always)]
    pub const fn set_otp_rule3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "0x4013 4000--0x4013 4FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_and_otfad_rule(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "0x4013 4000--0x4013 4FFF"]
    #[inline(always)]
    pub const fn set_flexspi_and_otfad_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "0x4013 6000--0x4013 6FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio0_rule(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "0x4013 6000--0x4013 6FFF"]
    #[inline(always)]
    pub const fn set_sdio0_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "0x4013 7000--0x4013 7FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio1_rule(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "0x4013 7000--0x4013 7FFF"]
    #[inline(always)]
    pub const fn set_sdio1_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridge1MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridge1MemRule0 {
        AipsBridge1MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridge1MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridge1MemRule0")
            .field("otp_rule0", &self.otp_rule0())
            .field("otp_rule1", &self.otp_rule1())
            .field("otp_rule2", &self.otp_rule2())
            .field("otp_rule3", &self.otp_rule3())
            .field("flexspi_and_otfad_rule", &self.flexspi_and_otfad_rule())
            .field("sdio0_rule", &self.sdio0_rule())
            .field("sdio1_rule", &self.sdio1_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridge1MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridge1MemRule0 {{ otp_rule0: {=u8:?}, otp_rule1: {=u8:?}, otp_rule2: {=u8:?}, otp_rule3: {=u8:?}, flexspi_and_otfad_rule: {=u8:?}, sdio0_rule: {=u8:?}, sdio1_rule: {=u8:?} }}",
            self.otp_rule0(),
            self.otp_rule1(),
            self.otp_rule2(),
            self.otp_rule3(),
            self.flexspi_and_otfad_rule(),
            self.sdio0_rule(),
            self.sdio1_rule()
        )
    }
}
#[doc = "Security access rules for AIPS Bridge peripherals. Each AIPS bridge sector is 4 Kbytes, there're 16 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridge1MemRule1(pub u32);
impl AipsBridge1MemRule1 {
    #[doc = "0x4013 8000--0x4013 8FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn rng_rule(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "0x4013 8000--0x4013 8FFF"]
    #[inline(always)]
    pub const fn set_rng_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "0x4013 9000--0x4013 9FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp0_rule(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "0x4013 9000--0x4013 9FFF"]
    #[inline(always)]
    pub const fn set_acmp0_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "0x4013 A000--0x4013 AFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0_rule(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "0x4013 A000--0x4013 AFFF"]
    #[inline(always)]
    pub const fn set_adc0_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "0x4013 B000--0x4013 BFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs_phy_rule(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "0x4013 B000--0x4013 BFFF"]
    #[inline(always)]
    pub const fn set_usb_hs_phy_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for AipsBridge1MemRule1 {
    #[inline(always)]
    fn default() -> AipsBridge1MemRule1 {
        AipsBridge1MemRule1(0)
    }
}
impl core::fmt::Debug for AipsBridge1MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridge1MemRule1")
            .field("rng_rule", &self.rng_rule())
            .field("acmp0_rule", &self.acmp0_rule())
            .field("adc0_rule", &self.adc0_rule())
            .field("usb_hs_phy_rule", &self.usb_hs_phy_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridge1MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridge1MemRule1 {{ rng_rule: {=u8:?}, acmp0_rule: {=u8:?}, adc0_rule: {=u8:?}, usb_hs_phy_rule: {=u8:?} }}",
            self.rng_rule(),
            self.acmp0_rule(),
            self.adc0_rule(),
            self.usb_hs_phy_rule()
        )
    }
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbGrp0MemRule0(pub u32);
impl ApbGrp0MemRule0 {
    #[doc = "0x4000 0000--0x4000 0FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn rstctl0_rule(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "0x4000 0000--0x4000 0FFF"]
    #[inline(always)]
    pub const fn set_rstctl0_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "0x4000 1000--0x4000 1FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn clkctl0_rule(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "0x4000 1000--0x4000 1FFF"]
    #[inline(always)]
    pub const fn set_clkctl0_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "0x4000 2000--0x4000 2FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn sysctl0_rule(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "0x4000 2000--0x4000 2FFF"]
    #[inline(always)]
    pub const fn set_sysctl0_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "0x4000 4000--0x4000 4FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn iopctl_rule(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "0x4000 4000--0x4000 4FFF"]
    #[inline(always)]
    pub const fn set_iopctl_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "0x4000 6000--0x4000 6FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn pufctrl_rule(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "0x4000 6000--0x4000 6FFF"]
    #[inline(always)]
    pub const fn set_pufctrl_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for ApbGrp0MemRule0 {
    #[inline(always)]
    fn default() -> ApbGrp0MemRule0 {
        ApbGrp0MemRule0(0)
    }
}
impl core::fmt::Debug for ApbGrp0MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbGrp0MemRule0")
            .field("rstctl0_rule", &self.rstctl0_rule())
            .field("clkctl0_rule", &self.clkctl0_rule())
            .field("sysctl0_rule", &self.sysctl0_rule())
            .field("iopctl_rule", &self.iopctl_rule())
            .field("pufctrl_rule", &self.pufctrl_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbGrp0MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbGrp0MemRule0 {{ rstctl0_rule: {=u8:?}, clkctl0_rule: {=u8:?}, sysctl0_rule: {=u8:?}, iopctl_rule: {=u8:?}, pufctrl_rule: {=u8:?} }}",
            self.rstctl0_rule(),
            self.clkctl0_rule(),
            self.sysctl0_rule(),
            self.iopctl_rule(),
            self.pufctrl_rule()
        )
    }
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbGrp0MemRule1(pub u32);
impl ApbGrp0MemRule1 {
    #[doc = "0x4000 E000--0x4000 EFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0_rule(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "0x4000 E000--0x4000 EFFF"]
    #[inline(always)]
    pub const fn set_wwdt0_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "0x4000 F000--0x4000 FFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn utick_rule(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "0x4000 F000--0x4000 FFFF"]
    #[inline(always)]
    pub const fn set_utick_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for ApbGrp0MemRule1 {
    #[inline(always)]
    fn default() -> ApbGrp0MemRule1 {
        ApbGrp0MemRule1(0)
    }
}
impl core::fmt::Debug for ApbGrp0MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbGrp0MemRule1")
            .field("wwdt0_rule", &self.wwdt0_rule())
            .field("utick_rule", &self.utick_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbGrp0MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbGrp0MemRule1 {{ wwdt0_rule: {=u8:?}, utick_rule: {=u8:?} }}",
            self.wwdt0_rule(),
            self.utick_rule()
        )
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbGrp1MemRule0(pub u32);
impl ApbGrp1MemRule0 {
    #[doc = "0x4002 0000--0x4002 0FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn rstctl1_rule(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "0x4002 0000--0x4002 0FFF"]
    #[inline(always)]
    pub const fn set_rstctl1_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "0x4002 1000--0x4002 1FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn clkctl1_rule(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "0x4002 1000--0x4002 1FFF"]
    #[inline(always)]
    pub const fn set_clkctl1_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "0x4002 2000--0x4002 2FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn sysctl1_rule(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "0x4002 2000--0x4002 2FFF"]
    #[inline(always)]
    pub const fn set_sysctl1_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "0x4002 5000--0x4002 5FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_intr_ctrl_rule(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "0x4002 5000--0x4002 5FFF"]
    #[inline(always)]
    pub const fn set_gpio_intr_ctrl_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "0x4002 6000--0x4002 6FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn periph_input_mux_rule(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "0x4002 6000--0x4002 6FFF"]
    #[inline(always)]
    pub const fn set_periph_input_mux_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for ApbGrp1MemRule0 {
    #[inline(always)]
    fn default() -> ApbGrp1MemRule0 {
        ApbGrp1MemRule0(0)
    }
}
impl core::fmt::Debug for ApbGrp1MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbGrp1MemRule0")
            .field("rstctl1_rule", &self.rstctl1_rule())
            .field("clkctl1_rule", &self.clkctl1_rule())
            .field("sysctl1_rule", &self.sysctl1_rule())
            .field("gpio_intr_ctrl_rule", &self.gpio_intr_ctrl_rule())
            .field("periph_input_mux_rule", &self.periph_input_mux_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbGrp1MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbGrp1MemRule0 {{ rstctl1_rule: {=u8:?}, clkctl1_rule: {=u8:?}, sysctl1_rule: {=u8:?}, gpio_intr_ctrl_rule: {=u8:?}, periph_input_mux_rule: {=u8:?} }}",
            self.rstctl1_rule(),
            self.clkctl1_rule(),
            self.sysctl1_rule(),
            self.gpio_intr_ctrl_rule(),
            self.periph_input_mux_rule()
        )
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbGrp1MemRule1(pub u32);
impl ApbGrp1MemRule1 {
    #[doc = "0x4002 8000--0x4002 8FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit0_rule(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "0x4002 8000--0x4002 8FFF"]
    #[inline(always)]
    pub const fn set_ct32bit0_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "0x4002 9000--0x4002 9FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit1_rule(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "0x4002 9000--0x4002 9FFF"]
    #[inline(always)]
    pub const fn set_ct32bit1_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "0x4002 A000--0x4002 AFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit2_rule(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "0x4002 A000--0x4002 AFFF"]
    #[inline(always)]
    pub const fn set_ct32bit2_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "0x4002 B000--0x4002 BFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit3_rule(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "0x4002 B000--0x4002 BFFF"]
    #[inline(always)]
    pub const fn set_ct32bit3_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "0x4002 C000--0x4002 CFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit4_rule(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "0x4002 C000--0x4002 CFFF"]
    #[inline(always)]
    pub const fn set_ct32bit4_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "0x4002 D000--0x4002 DFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn mrt_rule(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "0x4002 D000--0x4002 DFFF"]
    #[inline(always)]
    pub const fn set_mrt_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "0x4002 E000--0x4002 EFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1_rule(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "0x4002 E000--0x4002 EFFF"]
    #[inline(always)]
    pub const fn set_wwdt1_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "0x4002 F000--0x4002 FFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn freqme_rule(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "0x4002 F000--0x4002 FFFF"]
    #[inline(always)]
    pub const fn set_freqme_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for ApbGrp1MemRule1 {
    #[inline(always)]
    fn default() -> ApbGrp1MemRule1 {
        ApbGrp1MemRule1(0)
    }
}
impl core::fmt::Debug for ApbGrp1MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbGrp1MemRule1")
            .field("ct32bit0_rule", &self.ct32bit0_rule())
            .field("ct32bit1_rule", &self.ct32bit1_rule())
            .field("ct32bit2_rule", &self.ct32bit2_rule())
            .field("ct32bit3_rule", &self.ct32bit3_rule())
            .field("ct32bit4_rule", &self.ct32bit4_rule())
            .field("mrt_rule", &self.mrt_rule())
            .field("wwdt1_rule", &self.wwdt1_rule())
            .field("freqme_rule", &self.freqme_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbGrp1MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbGrp1MemRule1 {{ ct32bit0_rule: {=u8:?}, ct32bit1_rule: {=u8:?}, ct32bit2_rule: {=u8:?}, ct32bit3_rule: {=u8:?}, ct32bit4_rule: {=u8:?}, mrt_rule: {=u8:?}, wwdt1_rule: {=u8:?}, freqme_rule: {=u8:?} }}",
            self.ct32bit0_rule(),
            self.ct32bit1_rule(),
            self.ct32bit2_rule(),
            self.ct32bit3_rule(),
            self.ct32bit4_rule(),
            self.mrt_rule(),
            self.wwdt1_rule(),
            self.freqme_rule()
        )
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbGrp1MemRule2(pub u32);
impl ApbGrp1MemRule2 {
    #[doc = "0x4003 0000--0x4003 0FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_rule(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "0x4003 0000--0x4003 0FFF"]
    #[inline(always)]
    pub const fn set_rtc_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "0x4003 6000--0x4003 6FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0_rule(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "0x4003 6000--0x4003 6FFF"]
    #[inline(always)]
    pub const fn set_i3c0_rule(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for ApbGrp1MemRule2 {
    #[inline(always)]
    fn default() -> ApbGrp1MemRule2 {
        ApbGrp1MemRule2(0)
    }
}
impl core::fmt::Debug for ApbGrp1MemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbGrp1MemRule2")
            .field("rtc_rule", &self.rtc_rule())
            .field("i3c0_rule", &self.i3c0_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbGrp1MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbGrp1MemRule2 {{ rtc_rule: {=u8:?}, i3c0_rule: {=u8:?} }}",
            self.rtc_rule(),
            self.i3c0_rule()
        )
    }
}
#[doc = "m33 lock control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm33LockReg(pub u32);
impl Cm33LockReg {
    #[doc = "m33 LOCKNSVTOR write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_vtor(&self) -> super::vals::LockNsVtor {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::LockNsVtor::from_bits(val as u8)
    }
    #[doc = "m33 LOCKNSVTOR write-lock."]
    #[inline(always)]
    pub const fn set_lock_ns_vtor(&mut self, val: super::vals::LockNsVtor) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "m33 LOCKNSMPU write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_mpu(&self) -> super::vals::LockNsMpu {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::LockNsMpu::from_bits(val as u8)
    }
    #[doc = "m33 LOCKNSMPU write-lock."]
    #[inline(always)]
    pub const fn set_lock_ns_mpu(&mut self, val: super::vals::LockNsMpu) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "m33 LOCKSVTOR write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_s_vtor(&self) -> super::vals::LockSVtor {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::LockSVtor::from_bits(val as u8)
    }
    #[doc = "m33 LOCKSVTOR write-lock."]
    #[inline(always)]
    pub const fn set_lock_s_vtor(&mut self, val: super::vals::LockSVtor) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "m33 LOCKSMPU write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_s_mpu(&self) -> super::vals::LockSMpu {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::LockSMpu::from_bits(val as u8)
    }
    #[doc = "m33 LOCKSMPU write-lock."]
    #[inline(always)]
    pub const fn set_lock_s_mpu(&mut self, val: super::vals::LockSMpu) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "m33 LOCKSAU write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_sau(&self) -> super::vals::LockSau {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::LockSau::from_bits(val as u8)
    }
    #[doc = "m33 LOCKSAU write-lock."]
    #[inline(always)]
    pub const fn set_lock_sau(&mut self, val: super::vals::LockSau) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CM33_LOCK_REG_LOCK write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn cm33_lock_reg_lock(&self) -> super::vals::Cm33LockRegLock {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Cm33LockRegLock::from_bits(val as u8)
    }
    #[doc = "CM33_LOCK_REG_LOCK write-lock."]
    #[inline(always)]
    pub const fn set_cm33_lock_reg_lock(&mut self, val: super::vals::Cm33LockRegLock) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Cm33LockReg {
    #[inline(always)]
    fn default() -> Cm33LockReg {
        Cm33LockReg(0)
    }
}
impl core::fmt::Debug for Cm33LockReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm33LockReg")
            .field("lock_ns_vtor", &self.lock_ns_vtor())
            .field("lock_ns_mpu", &self.lock_ns_mpu())
            .field("lock_s_vtor", &self.lock_s_vtor())
            .field("lock_s_mpu", &self.lock_s_mpu())
            .field("lock_sau", &self.lock_sau())
            .field("cm33_lock_reg_lock", &self.cm33_lock_reg_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm33LockReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cm33LockReg {{ lock_ns_vtor: {:?}, lock_ns_mpu: {:?}, lock_s_vtor: {:?}, lock_s_mpu: {:?}, lock_sau: {:?}, cm33_lock_reg_lock: {:?} }}",
            self.lock_ns_vtor(),
            self.lock_ns_mpu(),
            self.lock_s_vtor(),
            self.lock_s_mpu(),
            self.lock_sau(),
            self.cm33_lock_reg_lock()
        )
    }
}
#[doc = "FLEXSPI0 Region 0 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0Region0Rule(pub u32);
impl Flexspi0Region0Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Flexspi0Region0RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexspi0Region0RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Flexspi0Region0RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Flexspi0Region0RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Flexspi0Region0RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Flexspi0Region0RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Flexspi0Region0RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Flexspi0Region0RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Flexspi0Region0RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Flexspi0Region0RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Flexspi0Region0RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Flexspi0Region0RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Flexspi0Region0RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Flexspi0Region0RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Flexspi0Region0RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Flexspi0Region0RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Flexspi0Region0RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Flexspi0Region0RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Flexspi0Region0RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Flexspi0Region0RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Flexspi0Region0RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Flexspi0Region0RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Flexspi0Region0RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Flexspi0Region0RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Flexspi0Region0Rule {
    #[inline(always)]
    fn default() -> Flexspi0Region0Rule {
        Flexspi0Region0Rule(0)
    }
}
impl core::fmt::Debug for Flexspi0Region0Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi0Region0Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi0Region0Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexspi0Region0Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "FLEXSPI0 Region 1 Rule 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0Region1Rule0(pub u32);
impl Flexspi0Region1Rule0 {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Flexspi0Region1Rule0Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexspi0Region1Rule0Rule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Flexspi0Region1Rule0Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Flexspi0Region1Rule0Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Flexspi0Region1Rule0Rule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Flexspi0Region1Rule0Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Flexspi0Region1Rule0Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Flexspi0Region1Rule0Rule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Flexspi0Region1Rule0Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Flexspi0Region1Rule0Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Flexspi0Region1Rule0Rule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Flexspi0Region1Rule0Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for Flexspi0Region1Rule0 {
    #[inline(always)]
    fn default() -> Flexspi0Region1Rule0 {
        Flexspi0Region1Rule0(0)
    }
}
impl core::fmt::Debug for Flexspi0Region1Rule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi0Region1Rule0")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi0Region1Rule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexspi0Region1Rule0 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3()
        )
    }
}
#[doc = "FLEXSPI0 Region 2 Rule 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0Region2Rule0(pub u32);
impl Flexspi0Region2Rule0 {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Flexspi0Region2Rule0Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexspi0Region2Rule0Rule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Flexspi0Region2Rule0Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Flexspi0Region2Rule0Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Flexspi0Region2Rule0Rule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Flexspi0Region2Rule0Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Flexspi0Region2Rule0Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Flexspi0Region2Rule0Rule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Flexspi0Region2Rule0Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Flexspi0Region2Rule0Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Flexspi0Region2Rule0Rule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Flexspi0Region2Rule0Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for Flexspi0Region2Rule0 {
    #[inline(always)]
    fn default() -> Flexspi0Region2Rule0 {
        Flexspi0Region2Rule0(0)
    }
}
impl core::fmt::Debug for Flexspi0Region2Rule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi0Region2Rule0")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi0Region2Rule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexspi0Region2Rule0 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3()
        )
    }
}
#[doc = "FLEXSPI0 Region 3 Rule 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0Region3Rule0(pub u32);
impl Flexspi0Region3Rule0 {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Flexspi0Region3Rule0Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexspi0Region3Rule0Rule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Flexspi0Region3Rule0Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Flexspi0Region3Rule0Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Flexspi0Region3Rule0Rule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Flexspi0Region3Rule0Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Flexspi0Region3Rule0Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Flexspi0Region3Rule0Rule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Flexspi0Region3Rule0Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Flexspi0Region3Rule0Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Flexspi0Region3Rule0Rule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Flexspi0Region3Rule0Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for Flexspi0Region3Rule0 {
    #[inline(always)]
    fn default() -> Flexspi0Region3Rule0 {
        Flexspi0Region3Rule0(0)
    }
}
impl core::fmt::Debug for Flexspi0Region3Rule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi0Region3Rule0")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi0Region3Rule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexspi0Region3Rule0 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3()
        )
    }
}
#[doc = "FLEXSPI0 Region 4 Rule 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0Region4Rule0(pub u32);
impl Flexspi0Region4Rule0 {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Flexspi0Region4Rule0Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexspi0Region4Rule0Rule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Flexspi0Region4Rule0Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Flexspi0Region4Rule0Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Flexspi0Region4Rule0Rule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Flexspi0Region4Rule0Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Flexspi0Region4Rule0Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Flexspi0Region4Rule0Rule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Flexspi0Region4Rule0Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Flexspi0Region4Rule0Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Flexspi0Region4Rule0Rule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Flexspi0Region4Rule0Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for Flexspi0Region4Rule0 {
    #[inline(always)]
    fn default() -> Flexspi0Region4Rule0 {
        Flexspi0Region4Rule0(0)
    }
}
impl core::fmt::Debug for Flexspi0Region4Rule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi0Region4Rule0")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi0Region4Rule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexspi0Region4Rule0 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3()
        )
    }
}
#[doc = "master secure level register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MasterSecLevel(pub u32);
impl MasterSecLevel {
    #[doc = "POWERQUAD master secure level control."]
    #[must_use]
    #[inline(always)]
    pub const fn powerquad_sec(&self) -> super::vals::MasterSecLevelPowerquadSec {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MasterSecLevelPowerquadSec::from_bits(val as u8)
    }
    #[doc = "POWERQUAD master secure level control."]
    #[inline(always)]
    pub const fn set_powerquad_sec(&mut self, val: super::vals::MasterSecLevelPowerquadSec) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "DSP master secure level control."]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_sec(&self) -> super::vals::MasterSecLevelDspSec {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MasterSecLevelDspSec::from_bits(val as u8)
    }
    #[doc = "DSP master secure level control."]
    #[inline(always)]
    pub const fn set_dsp_sec(&mut self, val: super::vals::MasterSecLevelDspSec) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "DMA0 master secure level control."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_sec(&self) -> super::vals::MasterSecLevelDma0Sec {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MasterSecLevelDma0Sec::from_bits(val as u8)
    }
    #[doc = "DMA0 master secure level control."]
    #[inline(always)]
    pub const fn set_dma0_sec(&mut self, val: super::vals::MasterSecLevelDma0Sec) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DMA1 master secure level control."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_sec(&self) -> super::vals::MasterSecLevelDma1Sec {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MasterSecLevelDma1Sec::from_bits(val as u8)
    }
    #[doc = "DMA1 master secure level control."]
    #[inline(always)]
    pub const fn set_dma1_sec(&mut self, val: super::vals::MasterSecLevelDma1Sec) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "SDIO0 master secure level control."]
    #[must_use]
    #[inline(always)]
    pub const fn sdio0_sec(&self) -> super::vals::MasterSecLevelSdio0Sec {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::MasterSecLevelSdio0Sec::from_bits(val as u8)
    }
    #[doc = "SDIO0 master secure level control."]
    #[inline(always)]
    pub const fn set_sdio0_sec(&mut self, val: super::vals::MasterSecLevelSdio0Sec) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "SDIO1 master secure level control."]
    #[must_use]
    #[inline(always)]
    pub const fn sdio1_sec(&self) -> super::vals::MasterSecLevelSdio1Sec {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::MasterSecLevelSdio1Sec::from_bits(val as u8)
    }
    #[doc = "SDIO1 master secure level control."]
    #[inline(always)]
    pub const fn set_sdio1_sec(&mut self, val: super::vals::MasterSecLevelSdio1Sec) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "MASTER_SEC_LEVEL register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn master_sec_level_lock(&self) -> super::vals::MasterSecLevelLock {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::MasterSecLevelLock::from_bits(val as u8)
    }
    #[doc = "MASTER_SEC_LEVEL register write-lock."]
    #[inline(always)]
    pub const fn set_master_sec_level_lock(&mut self, val: super::vals::MasterSecLevelLock) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for MasterSecLevel {
    #[inline(always)]
    fn default() -> MasterSecLevel {
        MasterSecLevel(0)
    }
}
impl core::fmt::Debug for MasterSecLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MasterSecLevel")
            .field("powerquad_sec", &self.powerquad_sec())
            .field("dsp_sec", &self.dsp_sec())
            .field("dma0_sec", &self.dma0_sec())
            .field("dma1_sec", &self.dma1_sec())
            .field("sdio0_sec", &self.sdio0_sec())
            .field("sdio1_sec", &self.sdio1_sec())
            .field("master_sec_level_lock", &self.master_sec_level_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MasterSecLevel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MasterSecLevel {{ powerquad_sec: {:?}, dsp_sec: {:?}, dma0_sec: {:?}, dma1_sec: {:?}, sdio0_sec: {:?}, sdio1_sec: {:?}, master_sec_level_lock: {:?} }}",
            self.powerquad_sec(),
            self.dsp_sec(),
            self.dma0_sec(),
            self.dma1_sec(),
            self.sdio0_sec(),
            self.sdio1_sec(),
            self.master_sec_level_lock()
        )
    }
}
#[doc = "master secure level anti-pole register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MasterSecLevelAntiPol(pub u32);
impl MasterSecLevelAntiPol {
    #[doc = "POWERQUAD master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[must_use]
    #[inline(always)]
    pub const fn powerquad_sec(&self) -> super::vals::MasterSecLevelAntiPolPowerquadSec {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MasterSecLevelAntiPolPowerquadSec::from_bits(val as u8)
    }
    #[doc = "POWERQUAD master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    pub const fn set_powerquad_sec(&mut self, val: super::vals::MasterSecLevelAntiPolPowerquadSec) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "DSP master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_sec(&self) -> super::vals::MasterSecLevelAntiPolDspSec {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MasterSecLevelAntiPolDspSec::from_bits(val as u8)
    }
    #[doc = "DSP master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    pub const fn set_dsp_sec(&mut self, val: super::vals::MasterSecLevelAntiPolDspSec) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "DMA0 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_sec(&self) -> super::vals::MasterSecLevelAntiPolDma0Sec {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MasterSecLevelAntiPolDma0Sec::from_bits(val as u8)
    }
    #[doc = "DMA0 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    pub const fn set_dma0_sec(&mut self, val: super::vals::MasterSecLevelAntiPolDma0Sec) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DMA1 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_sec(&self) -> super::vals::MasterSecLevelAntiPolDma1Sec {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MasterSecLevelAntiPolDma1Sec::from_bits(val as u8)
    }
    #[doc = "DMA1 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    pub const fn set_dma1_sec(&mut self, val: super::vals::MasterSecLevelAntiPolDma1Sec) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "SDIO0 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[must_use]
    #[inline(always)]
    pub const fn sdio0_sec(&self) -> super::vals::MasterSecLevelAntiPolSdio0Sec {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::MasterSecLevelAntiPolSdio0Sec::from_bits(val as u8)
    }
    #[doc = "SDIO0 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    pub const fn set_sdio0_sec(&mut self, val: super::vals::MasterSecLevelAntiPolSdio0Sec) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "SDIO1 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[must_use]
    #[inline(always)]
    pub const fn sdio1_sec(&self) -> super::vals::MasterSecLevelAntiPolSdio1Sec {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::MasterSecLevelAntiPolSdio1Sec::from_bits(val as u8)
    }
    #[doc = "SDIO1 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    pub const fn set_sdio1_sec(&mut self, val: super::vals::MasterSecLevelAntiPolSdio1Sec) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "MASTER_SEC_LEVEL_ANTI_POL register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn master_sec_level_anti_pole_lock(&self) -> super::vals::MasterSecLevelAntiPoleLock {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::MasterSecLevelAntiPoleLock::from_bits(val as u8)
    }
    #[doc = "MASTER_SEC_LEVEL_ANTI_POL register write-lock."]
    #[inline(always)]
    pub const fn set_master_sec_level_anti_pole_lock(
        &mut self,
        val: super::vals::MasterSecLevelAntiPoleLock,
    ) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for MasterSecLevelAntiPol {
    #[inline(always)]
    fn default() -> MasterSecLevelAntiPol {
        MasterSecLevelAntiPol(0)
    }
}
impl core::fmt::Debug for MasterSecLevelAntiPol {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MasterSecLevelAntiPol")
            .field("powerquad_sec", &self.powerquad_sec())
            .field("dsp_sec", &self.dsp_sec())
            .field("dma0_sec", &self.dma0_sec())
            .field("dma1_sec", &self.dma1_sec())
            .field("sdio0_sec", &self.sdio0_sec())
            .field("sdio1_sec", &self.sdio1_sec())
            .field(
                "master_sec_level_anti_pole_lock",
                &self.master_sec_level_anti_pole_lock(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MasterSecLevelAntiPol {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MasterSecLevelAntiPol {{ powerquad_sec: {:?}, dsp_sec: {:?}, dma0_sec: {:?}, dma1_sec: {:?}, sdio0_sec: {:?}, sdio1_sec: {:?}, master_sec_level_anti_pole_lock: {:?} }}",
            self.powerquad_sec(),
            self.dsp_sec(),
            self.dma0_sec(),
            self.dma1_sec(),
            self.sdio0_sec(),
            self.sdio1_sec(),
            self.master_sec_level_anti_pole_lock()
        )
    }
}
#[doc = "secure control duplicate register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscCtrlDpReg(pub u32);
impl MiscCtrlDpReg {
    #[doc = "Write lock."]
    #[must_use]
    #[inline(always)]
    pub const fn write_lock(&self) -> super::vals::MiscCtrlDpRegWriteLock {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MiscCtrlDpRegWriteLock::from_bits(val as u8)
    }
    #[doc = "Write lock."]
    #[inline(always)]
    pub const fn set_write_lock(&mut self, val: super::vals::MiscCtrlDpRegWriteLock) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "AHB bus matrix enable secure checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_secure_checking(&self) -> super::vals::MiscCtrlDpRegEnableSecureChecking {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::MiscCtrlDpRegEnableSecureChecking::from_bits(val as u8)
    }
    #[doc = "AHB bus matrix enable secure checking."]
    #[inline(always)]
    pub const fn set_enable_secure_checking(
        &mut self,
        val: super::vals::MiscCtrlDpRegEnableSecureChecking,
    ) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "AHB bus matrix enable secure privilege check."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_s_priv_check(&self) -> super::vals::MiscCtrlDpRegEnableSPrivCheck {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MiscCtrlDpRegEnableSPrivCheck::from_bits(val as u8)
    }
    #[doc = "AHB bus matrix enable secure privilege check."]
    #[inline(always)]
    pub const fn set_enable_s_priv_check(
        &mut self,
        val: super::vals::MiscCtrlDpRegEnableSPrivCheck,
    ) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "AHB bus matrix enable non-secure privilege check."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ns_priv_check(&self) -> super::vals::MiscCtrlDpRegEnableNsPrivCheck {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MiscCtrlDpRegEnableNsPrivCheck::from_bits(val as u8)
    }
    #[doc = "AHB bus matrix enable non-secure privilege check."]
    #[inline(always)]
    pub const fn set_enable_ns_priv_check(
        &mut self,
        val: super::vals::MiscCtrlDpRegEnableNsPrivCheck,
    ) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Disable secure violation abort."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_violation_abort(&self) -> super::vals::MiscCtrlDpRegDisableViolationAbort {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MiscCtrlDpRegDisableViolationAbort::from_bits(val as u8)
    }
    #[doc = "Disable secure violation abort."]
    #[inline(always)]
    pub const fn set_disable_violation_abort(
        &mut self,
        val: super::vals::MiscCtrlDpRegDisableViolationAbort,
    ) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Disable simple master strict mode."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_simple_master_strict_mode(
        &self,
    ) -> super::vals::MiscCtrlDpRegDisableSimpleMasterStrictMode {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MiscCtrlDpRegDisableSimpleMasterStrictMode::from_bits(val as u8)
    }
    #[doc = "Disable simple master strict mode."]
    #[inline(always)]
    pub const fn set_disable_simple_master_strict_mode(
        &mut self,
        val: super::vals::MiscCtrlDpRegDisableSimpleMasterStrictMode,
    ) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Disable smart master strict mode."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_smart_master_strict_mode(
        &self,
    ) -> super::vals::MiscCtrlDpRegDisableSmartMasterStrictMode {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::MiscCtrlDpRegDisableSmartMasterStrictMode::from_bits(val as u8)
    }
    #[doc = "Disable smart master strict mode."]
    #[inline(always)]
    pub const fn set_disable_smart_master_strict_mode(
        &mut self,
        val: super::vals::MiscCtrlDpRegDisableSmartMasterStrictMode,
    ) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Disable IDAU."]
    #[must_use]
    #[inline(always)]
    pub const fn idau_all_ns(&self) -> super::vals::MiscCtrlDpRegIdauAllNs {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::MiscCtrlDpRegIdauAllNs::from_bits(val as u8)
    }
    #[doc = "Disable IDAU."]
    #[inline(always)]
    pub const fn set_idau_all_ns(&mut self, val: super::vals::MiscCtrlDpRegIdauAllNs) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for MiscCtrlDpReg {
    #[inline(always)]
    fn default() -> MiscCtrlDpReg {
        MiscCtrlDpReg(0)
    }
}
impl core::fmt::Debug for MiscCtrlDpReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscCtrlDpReg")
            .field("write_lock", &self.write_lock())
            .field("enable_secure_checking", &self.enable_secure_checking())
            .field("enable_s_priv_check", &self.enable_s_priv_check())
            .field("enable_ns_priv_check", &self.enable_ns_priv_check())
            .field("disable_violation_abort", &self.disable_violation_abort())
            .field(
                "disable_simple_master_strict_mode",
                &self.disable_simple_master_strict_mode(),
            )
            .field(
                "disable_smart_master_strict_mode",
                &self.disable_smart_master_strict_mode(),
            )
            .field("idau_all_ns", &self.idau_all_ns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscCtrlDpReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MiscCtrlDpReg {{ write_lock: {:?}, enable_secure_checking: {:?}, enable_s_priv_check: {:?}, enable_ns_priv_check: {:?}, disable_violation_abort: {:?}, disable_simple_master_strict_mode: {:?}, disable_smart_master_strict_mode: {:?}, idau_all_ns: {:?} }}",
            self.write_lock(),
            self.enable_secure_checking(),
            self.enable_s_priv_check(),
            self.enable_ns_priv_check(),
            self.disable_violation_abort(),
            self.disable_simple_master_strict_mode(),
            self.disable_smart_master_strict_mode(),
            self.idau_all_ns()
        )
    }
}
#[doc = "secure control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscCtrlReg(pub u32);
impl MiscCtrlReg {
    #[doc = "Write lock."]
    #[must_use]
    #[inline(always)]
    pub const fn write_lock(&self) -> super::vals::MiscCtrlRegWriteLock {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MiscCtrlRegWriteLock::from_bits(val as u8)
    }
    #[doc = "Write lock."]
    #[inline(always)]
    pub const fn set_write_lock(&mut self, val: super::vals::MiscCtrlRegWriteLock) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "AHB bus matrix enable secure checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_secure_checking(&self) -> super::vals::MiscCtrlRegEnableSecureChecking {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::MiscCtrlRegEnableSecureChecking::from_bits(val as u8)
    }
    #[doc = "AHB bus matrix enable secure checking."]
    #[inline(always)]
    pub const fn set_enable_secure_checking(
        &mut self,
        val: super::vals::MiscCtrlRegEnableSecureChecking,
    ) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "AHB bus matrix enable secure privilege check."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_s_priv_check(&self) -> super::vals::MiscCtrlRegEnableSPrivCheck {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MiscCtrlRegEnableSPrivCheck::from_bits(val as u8)
    }
    #[doc = "AHB bus matrix enable secure privilege check."]
    #[inline(always)]
    pub const fn set_enable_s_priv_check(&mut self, val: super::vals::MiscCtrlRegEnableSPrivCheck) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "AHB bus matrix enable non-secure privilege check."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ns_priv_check(&self) -> super::vals::MiscCtrlRegEnableNsPrivCheck {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MiscCtrlRegEnableNsPrivCheck::from_bits(val as u8)
    }
    #[doc = "AHB bus matrix enable non-secure privilege check."]
    #[inline(always)]
    pub const fn set_enable_ns_priv_check(
        &mut self,
        val: super::vals::MiscCtrlRegEnableNsPrivCheck,
    ) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Disable secure violation abort."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_violation_abort(&self) -> super::vals::MiscCtrlRegDisableViolationAbort {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MiscCtrlRegDisableViolationAbort::from_bits(val as u8)
    }
    #[doc = "Disable secure violation abort."]
    #[inline(always)]
    pub const fn set_disable_violation_abort(
        &mut self,
        val: super::vals::MiscCtrlRegDisableViolationAbort,
    ) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Disable simple master strict mode."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_simple_master_strict_mode(
        &self,
    ) -> super::vals::MiscCtrlRegDisableSimpleMasterStrictMode {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MiscCtrlRegDisableSimpleMasterStrictMode::from_bits(val as u8)
    }
    #[doc = "Disable simple master strict mode."]
    #[inline(always)]
    pub const fn set_disable_simple_master_strict_mode(
        &mut self,
        val: super::vals::MiscCtrlRegDisableSimpleMasterStrictMode,
    ) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Disable smart master strict mode."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_smart_master_strict_mode(
        &self,
    ) -> super::vals::MiscCtrlRegDisableSmartMasterStrictMode {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::MiscCtrlRegDisableSmartMasterStrictMode::from_bits(val as u8)
    }
    #[doc = "Disable smart master strict mode."]
    #[inline(always)]
    pub const fn set_disable_smart_master_strict_mode(
        &mut self,
        val: super::vals::MiscCtrlRegDisableSmartMasterStrictMode,
    ) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Disable IDAU."]
    #[must_use]
    #[inline(always)]
    pub const fn idau_all_ns(&self) -> super::vals::MiscCtrlRegIdauAllNs {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::MiscCtrlRegIdauAllNs::from_bits(val as u8)
    }
    #[doc = "Disable IDAU."]
    #[inline(always)]
    pub const fn set_idau_all_ns(&mut self, val: super::vals::MiscCtrlRegIdauAllNs) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for MiscCtrlReg {
    #[inline(always)]
    fn default() -> MiscCtrlReg {
        MiscCtrlReg(0)
    }
}
impl core::fmt::Debug for MiscCtrlReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscCtrlReg")
            .field("write_lock", &self.write_lock())
            .field("enable_secure_checking", &self.enable_secure_checking())
            .field("enable_s_priv_check", &self.enable_s_priv_check())
            .field("enable_ns_priv_check", &self.enable_ns_priv_check())
            .field("disable_violation_abort", &self.disable_violation_abort())
            .field(
                "disable_simple_master_strict_mode",
                &self.disable_simple_master_strict_mode(),
            )
            .field(
                "disable_smart_master_strict_mode",
                &self.disable_smart_master_strict_mode(),
            )
            .field("idau_all_ns", &self.idau_all_ns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscCtrlReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MiscCtrlReg {{ write_lock: {:?}, enable_secure_checking: {:?}, enable_s_priv_check: {:?}, enable_ns_priv_check: {:?}, disable_violation_abort: {:?}, disable_simple_master_strict_mode: {:?}, disable_smart_master_strict_mode: {:?}, idau_all_ns: {:?} }}",
            self.write_lock(),
            self.enable_secure_checking(),
            self.enable_s_priv_check(),
            self.enable_ns_priv_check(),
            self.disable_violation_abort(),
            self.disable_simple_master_strict_mode(),
            self.disable_smart_master_strict_mode(),
            self.idau_all_ns()
        )
    }
}
#[doc = "Security access rules for HiFi 4 memory sectors (0x24000000--0x240FFFFF). Each sector is 32 Kbytes, there're 4 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PifHifi4XMemRule0(pub u32);
impl PifHifi4XMemRule0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
}
impl Default for PifHifi4XMemRule0 {
    #[inline(always)]
    fn default() -> PifHifi4XMemRule0 {
        PifHifi4XMemRule0(0)
    }
}
impl core::fmt::Debug for PifHifi4XMemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PifHifi4XMemRule0")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PifHifi4XMemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PifHifi4XMemRule0 {{ rule0: {=u8:?}, rule1: {=u8:?}, rule4: {=u8:?}, rule5: {=u8:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule4(),
            self.rule5()
        )
    }
}
#[doc = "SRAM Partition 00 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram00Rule(pub u32);
impl Ram00Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram00RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram00RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram00RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram00RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram00RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram00RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram00RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram00RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram00RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram00RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram00RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram00RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram00RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram00RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram00RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram00RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram00RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram00RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram00RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram00RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram00RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram00RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram00RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram00RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram00Rule {
    #[inline(always)]
    fn default() -> Ram00Rule {
        Ram00Rule(0)
    }
}
impl core::fmt::Debug for Ram00Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram00Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram00Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram00Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 01 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram01Rule(pub u32);
impl Ram01Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram01RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram01RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram01RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram01RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram01RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram01RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram01RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram01RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram01RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram01RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram01RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram01RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram01RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram01RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram01RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram01RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram01RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram01RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram01RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram01RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram01RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram01RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram01RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram01RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram01Rule {
    #[inline(always)]
    fn default() -> Ram01Rule {
        Ram01Rule(0)
    }
}
impl core::fmt::Debug for Ram01Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram01Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram01Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram01Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 02 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram02Rule(pub u32);
impl Ram02Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram02RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram02RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram02RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram02RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram02RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram02RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram02RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram02RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram02RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram02RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram02RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram02RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram02RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram02RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram02RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram02RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram02RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram02RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram02RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram02RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram02RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram02RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram02RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram02RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram02Rule {
    #[inline(always)]
    fn default() -> Ram02Rule {
        Ram02Rule(0)
    }
}
impl core::fmt::Debug for Ram02Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram02Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram02Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram02Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 03 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram03Rule(pub u32);
impl Ram03Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram03RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram03RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram03RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram03RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram03RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram03RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram03RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram03RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram03RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram03RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram03RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram03RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram03RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram03RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram03RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram03RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram03RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram03RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram03RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram03RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram03RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram03RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram03RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram03RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram03Rule {
    #[inline(always)]
    fn default() -> Ram03Rule {
        Ram03Rule(0)
    }
}
impl core::fmt::Debug for Ram03Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram03Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram03Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram03Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 04 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram04Rule(pub u32);
impl Ram04Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram04RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram04RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram04RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram04RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram04RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram04RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram04RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram04RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram04RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram04RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram04RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram04RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram04RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram04RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram04RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram04RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram04RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram04RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram04RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram04RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram04RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram04RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram04RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram04RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram04Rule {
    #[inline(always)]
    fn default() -> Ram04Rule {
        Ram04Rule(0)
    }
}
impl core::fmt::Debug for Ram04Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram04Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram04Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram04Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 05 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram05Rule(pub u32);
impl Ram05Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram05RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram05RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram05RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram05RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram05RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram05RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram05RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram05RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram05RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram05RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram05RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram05RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram05RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram05RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram05RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram05RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram05RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram05RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram05RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram05RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram05RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram05RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram05RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram05RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram05Rule {
    #[inline(always)]
    fn default() -> Ram05Rule {
        Ram05Rule(0)
    }
}
impl core::fmt::Debug for Ram05Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram05Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram05Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram05Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 06 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram06Rule(pub u32);
impl Ram06Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram06RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram06RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram06RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram06RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram06RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram06RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram06RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram06RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram06RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram06RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram06RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram06RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram06RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram06RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram06RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram06RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram06RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram06RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram06RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram06RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram06RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram06RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram06RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram06RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram06Rule {
    #[inline(always)]
    fn default() -> Ram06Rule {
        Ram06Rule(0)
    }
}
impl core::fmt::Debug for Ram06Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram06Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram06Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram06Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 07 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram07Rule(pub u32);
impl Ram07Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram07RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram07RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram07RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram07RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram07RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram07RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram07RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram07RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram07RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram07RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram07RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram07RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram07RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram07RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram07RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram07RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram07RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram07RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram07RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram07RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram07RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram07RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram07RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram07RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram07Rule {
    #[inline(always)]
    fn default() -> Ram07Rule {
        Ram07Rule(0)
    }
}
impl core::fmt::Debug for Ram07Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram07Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram07Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram07Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 08 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram08Rule(pub u32);
impl Ram08Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram08RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram08RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram08RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram08RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram08RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram08RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram08RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram08RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram08RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram08RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram08RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram08RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram08RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram08RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram08RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram08RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram08RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram08RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram08RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram08RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram08RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram08RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram08RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram08RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram08Rule {
    #[inline(always)]
    fn default() -> Ram08Rule {
        Ram08Rule(0)
    }
}
impl core::fmt::Debug for Ram08Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram08Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram08Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram08Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 09 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram09Rule(pub u32);
impl Ram09Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram09RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram09RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram09RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram09RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram09RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram09RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram09RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram09RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram09RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram09RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram09RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram09RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram09RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram09RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram09RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram09RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram09RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram09RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram09RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram09RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram09RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram09RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram09RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram09RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram09Rule {
    #[inline(always)]
    fn default() -> Ram09Rule {
        Ram09Rule(0)
    }
}
impl core::fmt::Debug for Ram09Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram09Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram09Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram09Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 10 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram10Rule(pub u32);
impl Ram10Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram10RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram10RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram10RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram10RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram10RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram10RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram10RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram10RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram10RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram10RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram10RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram10RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram10RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram10RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram10RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram10RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram10RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram10RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram10RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram10RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram10RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram10RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram10RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram10RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram10Rule {
    #[inline(always)]
    fn default() -> Ram10Rule {
        Ram10Rule(0)
    }
}
impl core::fmt::Debug for Ram10Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram10Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram10Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram10Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 11 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram11Rule(pub u32);
impl Ram11Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram11RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram11RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram11RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram11RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram11RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram11RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram11RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram11RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram11RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram11RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram11RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram11RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram11RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram11RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram11RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram11RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram11RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram11RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram11RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram11RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram11RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram11RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram11RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram11RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram11Rule {
    #[inline(always)]
    fn default() -> Ram11Rule {
        Ram11Rule(0)
    }
}
impl core::fmt::Debug for Ram11Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram11Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram11Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram11Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 12 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram12Rule(pub u32);
impl Ram12Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram12RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram12RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram12RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram12RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram12RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram12RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram12RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram12RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram12RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram12RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram12RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram12RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram12RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram12RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram12RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram12RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram12RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram12RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram12RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram12RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram12RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram12RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram12RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram12RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram12Rule {
    #[inline(always)]
    fn default() -> Ram12Rule {
        Ram12Rule(0)
    }
}
impl core::fmt::Debug for Ram12Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram12Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram12Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram12Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 13 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram13Rule(pub u32);
impl Ram13Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram13RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram13RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram13RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram13RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram13RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram13RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram13RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram13RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram13RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram13RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram13RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram13RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram13RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram13RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram13RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram13RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram13RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram13RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram13RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram13RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram13RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram13RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram13RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram13RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram13Rule {
    #[inline(always)]
    fn default() -> Ram13Rule {
        Ram13Rule(0)
    }
}
impl core::fmt::Debug for Ram13Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram13Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram13Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram13Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 14 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram14Rule(pub u32);
impl Ram14Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram14RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram14RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram14RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram14RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram14RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram14RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram14RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram14RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram14RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram14RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram14RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram14RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram14RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram14RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram14RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram14RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram14RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram14RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram14RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram14RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram14RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram14RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram14RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram14RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram14Rule {
    #[inline(always)]
    fn default() -> Ram14Rule {
        Ram14Rule(0)
    }
}
impl core::fmt::Debug for Ram14Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram14Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram14Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram14Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 15 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram15Rule(pub u32);
impl Ram15Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram15RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram15RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram15RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram15RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram15RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram15RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram15RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram15RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram15RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram15RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram15RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram15RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram15RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram15RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram15RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram15RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram15RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram15RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram15RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram15RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram15RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram15RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram15RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram15RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram15Rule {
    #[inline(always)]
    fn default() -> Ram15Rule {
        Ram15Rule(0)
    }
}
impl core::fmt::Debug for Ram15Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram15Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram15Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram15Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 16 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram16Rule(pub u32);
impl Ram16Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram16RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram16RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram16RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram16RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram16RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram16RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram16RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram16RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram16RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram16RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram16RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram16RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram16RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram16RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram16RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram16RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram16RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram16RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram16RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram16RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram16RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram16RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram16RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram16RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram16Rule {
    #[inline(always)]
    fn default() -> Ram16Rule {
        Ram16Rule(0)
    }
}
impl core::fmt::Debug for Ram16Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram16Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram16Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram16Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 17 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram17Rule(pub u32);
impl Ram17Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram17RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram17RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram17RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram17RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram17RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram17RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram17RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram17RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram17RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram17RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram17RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram17RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram17RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram17RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram17RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram17RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram17RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram17RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram17RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram17RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram17RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram17RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram17RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram17RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram17Rule {
    #[inline(always)]
    fn default() -> Ram17Rule {
        Ram17Rule(0)
    }
}
impl core::fmt::Debug for Ram17Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram17Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram17Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram17Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 18 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram18Rule(pub u32);
impl Ram18Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram18RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram18RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram18RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram18RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram18RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram18RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram18RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram18RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram18RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram18RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram18RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram18RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram18RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram18RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram18RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram18RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram18RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram18RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram18RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram18RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram18RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram18RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram18RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram18RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram18Rule {
    #[inline(always)]
    fn default() -> Ram18Rule {
        Ram18Rule(0)
    }
}
impl core::fmt::Debug for Ram18Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram18Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram18Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram18Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 19 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram19Rule(pub u32);
impl Ram19Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram19RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram19RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram19RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram19RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram19RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram19RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram19RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram19RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram19RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram19RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram19RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram19RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram19RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram19RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram19RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram19RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram19RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram19RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram19RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram19RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram19RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram19RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram19RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram19RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram19Rule {
    #[inline(always)]
    fn default() -> Ram19Rule {
        Ram19Rule(0)
    }
}
impl core::fmt::Debug for Ram19Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram19Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram19Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram19Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 20 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram20Rule(pub u32);
impl Ram20Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram20RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram20RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram20RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram20RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram20RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram20RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram20RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram20RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram20RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram20RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram20RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram20RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram20RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram20RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram20RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram20RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram20RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram20RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram20RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram20RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram20RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram20RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram20RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram20RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram20Rule {
    #[inline(always)]
    fn default() -> Ram20Rule {
        Ram20Rule(0)
    }
}
impl core::fmt::Debug for Ram20Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram20Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram20Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram20Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 21 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram21Rule(pub u32);
impl Ram21Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram21RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram21RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram21RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram21RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram21RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram21RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram21RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram21RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram21RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram21RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram21RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram21RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram21RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram21RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram21RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram21RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram21RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram21RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram21RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram21RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram21RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram21RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram21RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram21RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram21Rule {
    #[inline(always)]
    fn default() -> Ram21Rule {
        Ram21Rule(0)
    }
}
impl core::fmt::Debug for Ram21Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram21Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram21Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram21Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 22 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram22Rule(pub u32);
impl Ram22Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram22RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram22RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram22RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram22RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram22RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram22RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram22RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram22RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram22RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram22RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram22RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram22RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram22RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram22RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram22RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram22RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram22RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram22RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram22RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram22RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram22RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram22RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram22RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram22RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram22Rule {
    #[inline(always)]
    fn default() -> Ram22Rule {
        Ram22Rule(0)
    }
}
impl core::fmt::Debug for Ram22Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram22Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram22Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram22Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 23 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram23Rule(pub u32);
impl Ram23Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram23RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram23RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram23RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram23RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram23RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram23RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram23RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram23RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram23RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram23RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram23RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram23RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram23RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram23RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram23RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram23RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram23RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram23RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram23RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram23RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram23RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram23RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram23RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram23RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram23Rule {
    #[inline(always)]
    fn default() -> Ram23Rule {
        Ram23Rule(0)
    }
}
impl core::fmt::Debug for Ram23Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram23Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram23Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram23Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 24 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram24Rule(pub u32);
impl Ram24Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram24RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram24RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram24RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram24RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram24RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram24RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram24RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram24RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram24RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram24RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram24RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram24RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram24RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram24RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram24RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram24RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram24RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram24RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram24RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram24RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram24RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram24RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram24RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram24RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram24Rule {
    #[inline(always)]
    fn default() -> Ram24Rule {
        Ram24Rule(0)
    }
}
impl core::fmt::Debug for Ram24Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram24Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram24Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram24Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 25 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram25Rule(pub u32);
impl Ram25Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram25RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram25RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram25RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram25RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram25RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram25RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram25RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram25RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram25RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram25RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram25RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram25RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram25RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram25RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram25RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram25RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram25RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram25RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram25RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram25RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram25RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram25RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram25RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram25RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram25Rule {
    #[inline(always)]
    fn default() -> Ram25Rule {
        Ram25Rule(0)
    }
}
impl core::fmt::Debug for Ram25Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram25Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram25Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram25Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 26 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram26Rule(pub u32);
impl Ram26Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram26RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram26RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram26RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram26RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram26RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram26RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram26RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram26RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram26RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram26RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram26RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram26RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram26RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram26RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram26RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram26RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram26RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram26RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram26RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram26RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram26RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram26RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram26RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram26RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram26Rule {
    #[inline(always)]
    fn default() -> Ram26Rule {
        Ram26Rule(0)
    }
}
impl core::fmt::Debug for Ram26Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram26Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram26Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram26Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 27 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram27Rule(pub u32);
impl Ram27Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram27RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram27RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram27RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram27RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram27RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram27RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram27RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram27RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram27RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram27RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram27RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram27RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram27RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram27RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram27RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram27RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram27RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram27RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram27RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram27RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram27RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram27RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram27RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram27RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram27Rule {
    #[inline(always)]
    fn default() -> Ram27Rule {
        Ram27Rule(0)
    }
}
impl core::fmt::Debug for Ram27Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram27Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram27Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram27Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 28 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram28Rule(pub u32);
impl Ram28Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram28RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram28RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram28RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram28RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram28RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram28RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram28RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram28RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram28RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram28RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram28RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram28RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram28RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram28RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram28RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram28RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram28RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram28RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram28RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram28RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram28RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram28RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram28RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram28RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram28Rule {
    #[inline(always)]
    fn default() -> Ram28Rule {
        Ram28Rule(0)
    }
}
impl core::fmt::Debug for Ram28Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram28Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram28Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram28Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "SRAM Partition 29 Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram29Rule(pub u32);
impl Ram29Rule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Ram29RuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram29RuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Ram29RuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Ram29RuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ram29RuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Ram29RuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Ram29RuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ram29RuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Ram29RuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Ram29RuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ram29RuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Ram29RuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Ram29RuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ram29RuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Ram29RuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Ram29RuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ram29RuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Ram29RuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Ram29RuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ram29RuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Ram29RuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Ram29RuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ram29RuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Ram29RuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ram29Rule {
    #[inline(always)]
    fn default() -> Ram29Rule {
        Ram29Rule(0)
    }
}
impl core::fmt::Debug for Ram29Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram29Rule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram29Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ram29Rule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "Memory ROM Rule(n) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RomMemRule(pub u32);
impl RomMemRule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::RomMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RomMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::RomMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::RomMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::RomMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::RomMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::RomMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RomMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::RomMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::RomMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::RomMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::RomMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::RomMemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::RomMemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::RomMemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::RomMemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::RomMemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::RomMemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::RomMemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::RomMemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::RomMemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::RomMemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::RomMemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::RomMemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for RomMemRule {
    #[inline(always)]
    fn default() -> RomMemRule {
        RomMemRule(0)
    }
}
impl core::fmt::Debug for RomMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RomMemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RomMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RomMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "secure general purpose register 8 used to mask interrupts to DSP for security purpose"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecDspIntMask(pub u32);
impl SecDspIntMask {
    #[doc = "0: INTR5 is invisible to DSP, 1: INTR5 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr5_sec_mask(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR5 is invisible to DSP, 1: INTR5 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr5_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "0: INTR6 is invisible to DSP, 1: INTR6 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr6_sec_mask(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR6 is invisible to DSP, 1: INTR6 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr6_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "0: INTR7 is invisible to DSP, 1: INTR7 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr7_sec_mask(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR7 is invisible to DSP, 1: INTR7 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr7_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "0: INTR8 is invisible to DSP, 1: INTR8 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr8_sec_mask(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR8 is invisible to DSP, 1: INTR8 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr8_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "0: INTR9 is invisible to DSP, 1: INTR9 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr9_sec_mask(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR9 is invisible to DSP, 1: INTR9 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr9_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "0: INTR10 is invisible to DSP, 1: INTR10 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr10_sec_mask(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR10 is invisible to DSP, 1: INTR10 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr10_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0: INTR11 is invisible to DSP, 1: INTR11 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr11_sec_mask(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR11 is invisible to DSP, 1: INTR11 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr11_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0: INTR12 is invisible to DSP, 1: INTR12 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr12_sec_mask(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR12 is invisible to DSP, 1: INTR12 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr12_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "0: INTR13 is invisible to DSP, 1: INTR13 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr13_sec_mask(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR13 is invisible to DSP, 1: INTR13 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr13_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "0: INTR14 is invisible to DSP, 1: INTR14 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr14_sec_mask(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR14 is invisible to DSP, 1: INTR14 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr14_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "0: INTR15 is invisible to DSP, 1: INTR15 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr15_sec_mask(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR15 is invisible to DSP, 1: INTR15 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr15_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "0: INTR16 is invisible to DSP, 1: INTR16 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr16_sec_mask(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR16 is invisible to DSP, 1: INTR16 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr16_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "0: INTR17 is invisible to DSP, 1: INTR17 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr17_sec_mask(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR17 is invisible to DSP, 1: INTR17 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr17_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0: INTR18 is invisible to DSP, 1: INTR18 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr18_sec_mask(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR18 is invisible to DSP, 1: INTR18 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr18_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0: INTR19 is invisible to DSP, 1: INTR19 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr19_sec_mask(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR19 is invisible to DSP, 1: INTR19 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr19_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0: INTR20 is invisible to DSP, 1: INTR20 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr20_sec_mask(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR20 is invisible to DSP, 1: INTR20 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr20_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "0: INTR21 is invisible to DSP, 1: INTR21 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr21_sec_mask(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR21 is invisible to DSP, 1: INTR21 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr21_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "0: INTR22 is invisible to DSP, 1: INTR22 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr22_sec_mask(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR22 is invisible to DSP, 1: INTR22 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr22_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "0: INTR23 is invisible to DSP, 1: INTR23 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr23_sec_mask(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR23 is invisible to DSP, 1: INTR23 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr23_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "0: INTR24 is invisible to DSP, 1: INTR24 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr24_sec_mask(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR24 is invisible to DSP, 1: INTR24 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr24_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "0: INTR25 is invisible to DSP, 1: INTR25 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr25_sec_mask(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR25 is invisible to DSP, 1: INTR25 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr25_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "0: INTR26 is invisible to DSP, 1: INTR26 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr26_sec_mask(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR26 is invisible to DSP, 1: INTR26 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr26_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "0: INTR27 is invisible to DSP, 1: INTR27 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr27_sec_mask(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR27 is invisible to DSP, 1: INTR27 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr27_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "0: INTR28 is invisible to DSP, 1: INTR28 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr28_sec_mask(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR28 is invisible to DSP, 1: INTR28 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr28_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "0: INTR29 is invisible to DSP, 1: INTR29 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr29_sec_mask(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR29 is invisible to DSP, 1: INTR29 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr29_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "0: INTR30 is invisible to DSP, 1: INTR30 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr30_sec_mask(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR30 is invisible to DSP, 1: INTR30 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr30_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "0: INTR31 is invisible to DSP, 1: INTR31 is visible to DSP"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_intr31_sec_mask(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0: INTR31 is invisible to DSP, 1: INTR31 is visible to DSP"]
    #[inline(always)]
    pub const fn set_dsp_intr31_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecDspIntMask {
    #[inline(always)]
    fn default() -> SecDspIntMask {
        SecDspIntMask(0)
    }
}
impl core::fmt::Debug for SecDspIntMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecDspIntMask")
            .field("dsp_intr5_sec_mask", &self.dsp_intr5_sec_mask())
            .field("dsp_intr6_sec_mask", &self.dsp_intr6_sec_mask())
            .field("dsp_intr7_sec_mask", &self.dsp_intr7_sec_mask())
            .field("dsp_intr8_sec_mask", &self.dsp_intr8_sec_mask())
            .field("dsp_intr9_sec_mask", &self.dsp_intr9_sec_mask())
            .field("dsp_intr10_sec_mask", &self.dsp_intr10_sec_mask())
            .field("dsp_intr11_sec_mask", &self.dsp_intr11_sec_mask())
            .field("dsp_intr12_sec_mask", &self.dsp_intr12_sec_mask())
            .field("dsp_intr13_sec_mask", &self.dsp_intr13_sec_mask())
            .field("dsp_intr14_sec_mask", &self.dsp_intr14_sec_mask())
            .field("dsp_intr15_sec_mask", &self.dsp_intr15_sec_mask())
            .field("dsp_intr16_sec_mask", &self.dsp_intr16_sec_mask())
            .field("dsp_intr17_sec_mask", &self.dsp_intr17_sec_mask())
            .field("dsp_intr18_sec_mask", &self.dsp_intr18_sec_mask())
            .field("dsp_intr19_sec_mask", &self.dsp_intr19_sec_mask())
            .field("dsp_intr20_sec_mask", &self.dsp_intr20_sec_mask())
            .field("dsp_intr21_sec_mask", &self.dsp_intr21_sec_mask())
            .field("dsp_intr22_sec_mask", &self.dsp_intr22_sec_mask())
            .field("dsp_intr23_sec_mask", &self.dsp_intr23_sec_mask())
            .field("dsp_intr24_sec_mask", &self.dsp_intr24_sec_mask())
            .field("dsp_intr25_sec_mask", &self.dsp_intr25_sec_mask())
            .field("dsp_intr26_sec_mask", &self.dsp_intr26_sec_mask())
            .field("dsp_intr27_sec_mask", &self.dsp_intr27_sec_mask())
            .field("dsp_intr28_sec_mask", &self.dsp_intr28_sec_mask())
            .field("dsp_intr29_sec_mask", &self.dsp_intr29_sec_mask())
            .field("dsp_intr30_sec_mask", &self.dsp_intr30_sec_mask())
            .field("dsp_intr31_sec_mask", &self.dsp_intr31_sec_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecDspIntMask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecDspIntMask {{ dsp_intr5_sec_mask: {=bool:?}, dsp_intr6_sec_mask: {=bool:?}, dsp_intr7_sec_mask: {=bool:?}, dsp_intr8_sec_mask: {=bool:?}, dsp_intr9_sec_mask: {=bool:?}, dsp_intr10_sec_mask: {=bool:?}, dsp_intr11_sec_mask: {=bool:?}, dsp_intr12_sec_mask: {=bool:?}, dsp_intr13_sec_mask: {=bool:?}, dsp_intr14_sec_mask: {=bool:?}, dsp_intr15_sec_mask: {=bool:?}, dsp_intr16_sec_mask: {=bool:?}, dsp_intr17_sec_mask: {=bool:?}, dsp_intr18_sec_mask: {=bool:?}, dsp_intr19_sec_mask: {=bool:?}, dsp_intr20_sec_mask: {=bool:?}, dsp_intr21_sec_mask: {=bool:?}, dsp_intr22_sec_mask: {=bool:?}, dsp_intr23_sec_mask: {=bool:?}, dsp_intr24_sec_mask: {=bool:?}, dsp_intr25_sec_mask: {=bool:?}, dsp_intr26_sec_mask: {=bool:?}, dsp_intr27_sec_mask: {=bool:?}, dsp_intr28_sec_mask: {=bool:?}, dsp_intr29_sec_mask: {=bool:?}, dsp_intr30_sec_mask: {=bool:?}, dsp_intr31_sec_mask: {=bool:?} }}",
            self.dsp_intr5_sec_mask(),
            self.dsp_intr6_sec_mask(),
            self.dsp_intr7_sec_mask(),
            self.dsp_intr8_sec_mask(),
            self.dsp_intr9_sec_mask(),
            self.dsp_intr10_sec_mask(),
            self.dsp_intr11_sec_mask(),
            self.dsp_intr12_sec_mask(),
            self.dsp_intr13_sec_mask(),
            self.dsp_intr14_sec_mask(),
            self.dsp_intr15_sec_mask(),
            self.dsp_intr16_sec_mask(),
            self.dsp_intr17_sec_mask(),
            self.dsp_intr18_sec_mask(),
            self.dsp_intr19_sec_mask(),
            self.dsp_intr20_sec_mask(),
            self.dsp_intr21_sec_mask(),
            self.dsp_intr22_sec_mask(),
            self.dsp_intr23_sec_mask(),
            self.dsp_intr24_sec_mask(),
            self.dsp_intr25_sec_mask(),
            self.dsp_intr26_sec_mask(),
            self.dsp_intr27_sec_mask(),
            self.dsp_intr28_sec_mask(),
            self.dsp_intr29_sec_mask(),
            self.dsp_intr30_sec_mask(),
            self.dsp_intr31_sec_mask()
        )
    }
}
#[doc = "Secure GPIO mask for port 0 pins. This register is used to block leakage of Secure interface (GPIOs, I2C, UART configured as secure peripherals) pin states to non-secure world."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpioMask0(pub u32);
impl SecGpioMask0 {
    #[doc = "0 : Pin PIO0_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin0_sec_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin0_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0 : Pin PIO0_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin1_sec_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin1_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0 : Pin PIO0_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin2_sec_mask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin2_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "0 : Pin PIO0_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin3_sec_mask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin3_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "0 : Pin PIO0_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin4_sec_mask(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin4_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "0 : Pin PIO0_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin5_sec_mask(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin5_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "0 : Pin PIO0_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin6_sec_mask(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin6_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "0 : Pin PIO0_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin7_sec_mask(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin7_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "0 : Pin PIO0_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin8_sec_mask(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin8_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "0 : Pin PIO0_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin9_sec_mask(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin9_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "0 : Pin PIO0_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin10_sec_mask(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin10_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 : Pin PIO0_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin11_sec_mask(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin11_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0 : Pin PIO0_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin12_sec_mask(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin12_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "0 : Pin PIO0_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin13_sec_mask(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin13_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "0 : Pin PIO0_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin14_sec_mask(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin14_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "0 : Pin PIO0_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin15_sec_mask(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin15_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "0 : Pin PIO0_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin16_sec_mask(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin16_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "0 : Pin PIO0_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin17_sec_mask(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin17_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 : Pin PIO0_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin18_sec_mask(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin18_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 : Pin PIO0_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin19_sec_mask(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin19_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 : Pin PIO0_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin20_sec_mask(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin20_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "0 : Pin PIO0_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin21_sec_mask(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin21_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "0 : Pin PIO0_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin22_sec_mask(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin22_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "0 : Pin PIO0_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin23_sec_mask(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin23_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "0 : Pin PIO0_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin24_sec_mask(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin24_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "0 : Pin PIO0_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin25_sec_mask(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin25_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "0 : Pin PIO0_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin26_sec_mask(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin26_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "0 : Pin PIO0_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin27_sec_mask(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin27_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "0 : Pin PIO0_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin28_sec_mask(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin28_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "0 : Pin PIO0_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin29_sec_mask(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin29_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "0 : Pin PIO0_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin30_sec_mask(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin30_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "0 : Pin PIO0_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin31_sec_mask(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO0_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio0_pin31_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpioMask0 {
    #[inline(always)]
    fn default() -> SecGpioMask0 {
        SecGpioMask0(0)
    }
}
impl core::fmt::Debug for SecGpioMask0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpioMask0")
            .field("pio0_pin0_sec_mask", &self.pio0_pin0_sec_mask())
            .field("pio0_pin1_sec_mask", &self.pio0_pin1_sec_mask())
            .field("pio0_pin2_sec_mask", &self.pio0_pin2_sec_mask())
            .field("pio0_pin3_sec_mask", &self.pio0_pin3_sec_mask())
            .field("pio0_pin4_sec_mask", &self.pio0_pin4_sec_mask())
            .field("pio0_pin5_sec_mask", &self.pio0_pin5_sec_mask())
            .field("pio0_pin6_sec_mask", &self.pio0_pin6_sec_mask())
            .field("pio0_pin7_sec_mask", &self.pio0_pin7_sec_mask())
            .field("pio0_pin8_sec_mask", &self.pio0_pin8_sec_mask())
            .field("pio0_pin9_sec_mask", &self.pio0_pin9_sec_mask())
            .field("pio0_pin10_sec_mask", &self.pio0_pin10_sec_mask())
            .field("pio0_pin11_sec_mask", &self.pio0_pin11_sec_mask())
            .field("pio0_pin12_sec_mask", &self.pio0_pin12_sec_mask())
            .field("pio0_pin13_sec_mask", &self.pio0_pin13_sec_mask())
            .field("pio0_pin14_sec_mask", &self.pio0_pin14_sec_mask())
            .field("pio0_pin15_sec_mask", &self.pio0_pin15_sec_mask())
            .field("pio0_pin16_sec_mask", &self.pio0_pin16_sec_mask())
            .field("pio0_pin17_sec_mask", &self.pio0_pin17_sec_mask())
            .field("pio0_pin18_sec_mask", &self.pio0_pin18_sec_mask())
            .field("pio0_pin19_sec_mask", &self.pio0_pin19_sec_mask())
            .field("pio0_pin20_sec_mask", &self.pio0_pin20_sec_mask())
            .field("pio0_pin21_sec_mask", &self.pio0_pin21_sec_mask())
            .field("pio0_pin22_sec_mask", &self.pio0_pin22_sec_mask())
            .field("pio0_pin23_sec_mask", &self.pio0_pin23_sec_mask())
            .field("pio0_pin24_sec_mask", &self.pio0_pin24_sec_mask())
            .field("pio0_pin25_sec_mask", &self.pio0_pin25_sec_mask())
            .field("pio0_pin26_sec_mask", &self.pio0_pin26_sec_mask())
            .field("pio0_pin27_sec_mask", &self.pio0_pin27_sec_mask())
            .field("pio0_pin28_sec_mask", &self.pio0_pin28_sec_mask())
            .field("pio0_pin29_sec_mask", &self.pio0_pin29_sec_mask())
            .field("pio0_pin30_sec_mask", &self.pio0_pin30_sec_mask())
            .field("pio0_pin31_sec_mask", &self.pio0_pin31_sec_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpioMask0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpioMask0 {{ pio0_pin0_sec_mask: {=bool:?}, pio0_pin1_sec_mask: {=bool:?}, pio0_pin2_sec_mask: {=bool:?}, pio0_pin3_sec_mask: {=bool:?}, pio0_pin4_sec_mask: {=bool:?}, pio0_pin5_sec_mask: {=bool:?}, pio0_pin6_sec_mask: {=bool:?}, pio0_pin7_sec_mask: {=bool:?}, pio0_pin8_sec_mask: {=bool:?}, pio0_pin9_sec_mask: {=bool:?}, pio0_pin10_sec_mask: {=bool:?}, pio0_pin11_sec_mask: {=bool:?}, pio0_pin12_sec_mask: {=bool:?}, pio0_pin13_sec_mask: {=bool:?}, pio0_pin14_sec_mask: {=bool:?}, pio0_pin15_sec_mask: {=bool:?}, pio0_pin16_sec_mask: {=bool:?}, pio0_pin17_sec_mask: {=bool:?}, pio0_pin18_sec_mask: {=bool:?}, pio0_pin19_sec_mask: {=bool:?}, pio0_pin20_sec_mask: {=bool:?}, pio0_pin21_sec_mask: {=bool:?}, pio0_pin22_sec_mask: {=bool:?}, pio0_pin23_sec_mask: {=bool:?}, pio0_pin24_sec_mask: {=bool:?}, pio0_pin25_sec_mask: {=bool:?}, pio0_pin26_sec_mask: {=bool:?}, pio0_pin27_sec_mask: {=bool:?}, pio0_pin28_sec_mask: {=bool:?}, pio0_pin29_sec_mask: {=bool:?}, pio0_pin30_sec_mask: {=bool:?}, pio0_pin31_sec_mask: {=bool:?} }}",
            self.pio0_pin0_sec_mask(),
            self.pio0_pin1_sec_mask(),
            self.pio0_pin2_sec_mask(),
            self.pio0_pin3_sec_mask(),
            self.pio0_pin4_sec_mask(),
            self.pio0_pin5_sec_mask(),
            self.pio0_pin6_sec_mask(),
            self.pio0_pin7_sec_mask(),
            self.pio0_pin8_sec_mask(),
            self.pio0_pin9_sec_mask(),
            self.pio0_pin10_sec_mask(),
            self.pio0_pin11_sec_mask(),
            self.pio0_pin12_sec_mask(),
            self.pio0_pin13_sec_mask(),
            self.pio0_pin14_sec_mask(),
            self.pio0_pin15_sec_mask(),
            self.pio0_pin16_sec_mask(),
            self.pio0_pin17_sec_mask(),
            self.pio0_pin18_sec_mask(),
            self.pio0_pin19_sec_mask(),
            self.pio0_pin20_sec_mask(),
            self.pio0_pin21_sec_mask(),
            self.pio0_pin22_sec_mask(),
            self.pio0_pin23_sec_mask(),
            self.pio0_pin24_sec_mask(),
            self.pio0_pin25_sec_mask(),
            self.pio0_pin26_sec_mask(),
            self.pio0_pin27_sec_mask(),
            self.pio0_pin28_sec_mask(),
            self.pio0_pin29_sec_mask(),
            self.pio0_pin30_sec_mask(),
            self.pio0_pin31_sec_mask()
        )
    }
}
#[doc = "Secure GPIO mask for port 1 pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpioMask1(pub u32);
impl SecGpioMask1 {
    #[doc = "0 : Pin PIO1_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin0_sec_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin0_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0 : Pin PIO1_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin1_sec_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin1_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0 : Pin PIO1_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin2_sec_mask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin2_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "0 : Pin PIO1_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin3_sec_mask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin3_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "0 : Pin PIO1_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin4_sec_mask(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin4_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "0 : Pin PIO1_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin5_sec_mask(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin5_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "0 : Pin PIO1_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin6_sec_mask(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin6_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "0 : Pin PIO1_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin7_sec_mask(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin7_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "0 : Pin PIO1_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin8_sec_mask(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin8_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "0 : Pin PIO1_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin9_sec_mask(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin9_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "0 : Pin PIO1_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin10_sec_mask(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin10_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 : Pin PIO1_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin11_sec_mask(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin11_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0 : Pin PIO1_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin12_sec_mask(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin12_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "0 : Pin PIO1_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin13_sec_mask(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin13_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "0 : Pin PIO1_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin14_sec_mask(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin14_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "0 : Pin PIO1_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin15_sec_mask(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin15_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "0 : Pin PIO1_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin16_sec_mask(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin16_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "0 : Pin PIO1_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin17_sec_mask(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin17_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 : Pin PIO1_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin18_sec_mask(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin18_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 : Pin PIO1_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin19_sec_mask(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin19_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 : Pin PIO1_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin20_sec_mask(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin20_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "0 : Pin PIO1_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin21_sec_mask(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin21_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "0 : Pin PIO1_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin22_sec_mask(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin22_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "0 : Pin PIO1_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin23_sec_mask(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin23_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "0 : Pin PIO1_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin24_sec_mask(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin24_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "0 : Pin PIO1_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin25_sec_mask(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin25_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "0 : Pin PIO1_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin26_sec_mask(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin26_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "0 : Pin PIO1_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin27_sec_mask(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin27_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "0 : Pin PIO1_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin28_sec_mask(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin28_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "0 : Pin PIO1_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin29_sec_mask(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin29_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "0 : Pin PIO1_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin30_sec_mask(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin30_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "0 : Pin PIO1_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin31_sec_mask(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO1_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio1_pin31_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpioMask1 {
    #[inline(always)]
    fn default() -> SecGpioMask1 {
        SecGpioMask1(0)
    }
}
impl core::fmt::Debug for SecGpioMask1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpioMask1")
            .field("pio1_pin0_sec_mask", &self.pio1_pin0_sec_mask())
            .field("pio1_pin1_sec_mask", &self.pio1_pin1_sec_mask())
            .field("pio1_pin2_sec_mask", &self.pio1_pin2_sec_mask())
            .field("pio1_pin3_sec_mask", &self.pio1_pin3_sec_mask())
            .field("pio1_pin4_sec_mask", &self.pio1_pin4_sec_mask())
            .field("pio1_pin5_sec_mask", &self.pio1_pin5_sec_mask())
            .field("pio1_pin6_sec_mask", &self.pio1_pin6_sec_mask())
            .field("pio1_pin7_sec_mask", &self.pio1_pin7_sec_mask())
            .field("pio1_pin8_sec_mask", &self.pio1_pin8_sec_mask())
            .field("pio1_pin9_sec_mask", &self.pio1_pin9_sec_mask())
            .field("pio1_pin10_sec_mask", &self.pio1_pin10_sec_mask())
            .field("pio1_pin11_sec_mask", &self.pio1_pin11_sec_mask())
            .field("pio1_pin12_sec_mask", &self.pio1_pin12_sec_mask())
            .field("pio1_pin13_sec_mask", &self.pio1_pin13_sec_mask())
            .field("pio1_pin14_sec_mask", &self.pio1_pin14_sec_mask())
            .field("pio1_pin15_sec_mask", &self.pio1_pin15_sec_mask())
            .field("pio1_pin16_sec_mask", &self.pio1_pin16_sec_mask())
            .field("pio1_pin17_sec_mask", &self.pio1_pin17_sec_mask())
            .field("pio1_pin18_sec_mask", &self.pio1_pin18_sec_mask())
            .field("pio1_pin19_sec_mask", &self.pio1_pin19_sec_mask())
            .field("pio1_pin20_sec_mask", &self.pio1_pin20_sec_mask())
            .field("pio1_pin21_sec_mask", &self.pio1_pin21_sec_mask())
            .field("pio1_pin22_sec_mask", &self.pio1_pin22_sec_mask())
            .field("pio1_pin23_sec_mask", &self.pio1_pin23_sec_mask())
            .field("pio1_pin24_sec_mask", &self.pio1_pin24_sec_mask())
            .field("pio1_pin25_sec_mask", &self.pio1_pin25_sec_mask())
            .field("pio1_pin26_sec_mask", &self.pio1_pin26_sec_mask())
            .field("pio1_pin27_sec_mask", &self.pio1_pin27_sec_mask())
            .field("pio1_pin28_sec_mask", &self.pio1_pin28_sec_mask())
            .field("pio1_pin29_sec_mask", &self.pio1_pin29_sec_mask())
            .field("pio1_pin30_sec_mask", &self.pio1_pin30_sec_mask())
            .field("pio1_pin31_sec_mask", &self.pio1_pin31_sec_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpioMask1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpioMask1 {{ pio1_pin0_sec_mask: {=bool:?}, pio1_pin1_sec_mask: {=bool:?}, pio1_pin2_sec_mask: {=bool:?}, pio1_pin3_sec_mask: {=bool:?}, pio1_pin4_sec_mask: {=bool:?}, pio1_pin5_sec_mask: {=bool:?}, pio1_pin6_sec_mask: {=bool:?}, pio1_pin7_sec_mask: {=bool:?}, pio1_pin8_sec_mask: {=bool:?}, pio1_pin9_sec_mask: {=bool:?}, pio1_pin10_sec_mask: {=bool:?}, pio1_pin11_sec_mask: {=bool:?}, pio1_pin12_sec_mask: {=bool:?}, pio1_pin13_sec_mask: {=bool:?}, pio1_pin14_sec_mask: {=bool:?}, pio1_pin15_sec_mask: {=bool:?}, pio1_pin16_sec_mask: {=bool:?}, pio1_pin17_sec_mask: {=bool:?}, pio1_pin18_sec_mask: {=bool:?}, pio1_pin19_sec_mask: {=bool:?}, pio1_pin20_sec_mask: {=bool:?}, pio1_pin21_sec_mask: {=bool:?}, pio1_pin22_sec_mask: {=bool:?}, pio1_pin23_sec_mask: {=bool:?}, pio1_pin24_sec_mask: {=bool:?}, pio1_pin25_sec_mask: {=bool:?}, pio1_pin26_sec_mask: {=bool:?}, pio1_pin27_sec_mask: {=bool:?}, pio1_pin28_sec_mask: {=bool:?}, pio1_pin29_sec_mask: {=bool:?}, pio1_pin30_sec_mask: {=bool:?}, pio1_pin31_sec_mask: {=bool:?} }}",
            self.pio1_pin0_sec_mask(),
            self.pio1_pin1_sec_mask(),
            self.pio1_pin2_sec_mask(),
            self.pio1_pin3_sec_mask(),
            self.pio1_pin4_sec_mask(),
            self.pio1_pin5_sec_mask(),
            self.pio1_pin6_sec_mask(),
            self.pio1_pin7_sec_mask(),
            self.pio1_pin8_sec_mask(),
            self.pio1_pin9_sec_mask(),
            self.pio1_pin10_sec_mask(),
            self.pio1_pin11_sec_mask(),
            self.pio1_pin12_sec_mask(),
            self.pio1_pin13_sec_mask(),
            self.pio1_pin14_sec_mask(),
            self.pio1_pin15_sec_mask(),
            self.pio1_pin16_sec_mask(),
            self.pio1_pin17_sec_mask(),
            self.pio1_pin18_sec_mask(),
            self.pio1_pin19_sec_mask(),
            self.pio1_pin20_sec_mask(),
            self.pio1_pin21_sec_mask(),
            self.pio1_pin22_sec_mask(),
            self.pio1_pin23_sec_mask(),
            self.pio1_pin24_sec_mask(),
            self.pio1_pin25_sec_mask(),
            self.pio1_pin26_sec_mask(),
            self.pio1_pin27_sec_mask(),
            self.pio1_pin28_sec_mask(),
            self.pio1_pin29_sec_mask(),
            self.pio1_pin30_sec_mask(),
            self.pio1_pin31_sec_mask()
        )
    }
}
#[doc = "Secure GPIO mask for port 2 pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpioMask2(pub u32);
impl SecGpioMask2 {
    #[doc = "0 : Pin PIO2_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin0_sec_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin0_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0 : Pin PIO2_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin1_sec_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin1_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0 : Pin PIO2_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin2_sec_mask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin2_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "0 : Pin PIO2_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin3_sec_mask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin3_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "0 : Pin PIO2_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin4_sec_mask(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin4_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "0 : Pin PIO2_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin5_sec_mask(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin5_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "0 : Pin PIO2_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin6_sec_mask(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin6_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "0 : Pin PIO2_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin7_sec_mask(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin7_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "0 : Pin PIO2_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin8_sec_mask(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin8_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "0 : Pin PIO2_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin9_sec_mask(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin9_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "0 : Pin PIO2_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin10_sec_mask(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin10_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 : Pin PIO2_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin11_sec_mask(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin11_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0 : Pin PIO2_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin12_sec_mask(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin12_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "0 : Pin PIO2_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin13_sec_mask(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin13_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "0 : Pin PIO2_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin14_sec_mask(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin14_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "0 : Pin PIO2_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin15_sec_mask(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin15_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "0 : Pin PIO2_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin16_sec_mask(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin16_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "0 : Pin PIO2_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin17_sec_mask(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin17_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 : Pin PIO2_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin18_sec_mask(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin18_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 : Pin PIO2_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin19_sec_mask(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin19_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 : Pin PIO2_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin20_sec_mask(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin20_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "0 : Pin PIO2_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin21_sec_mask(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin21_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "0 : Pin PIO2_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin22_sec_mask(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin22_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "0 : Pin PIO2_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin23_sec_mask(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin23_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "0 : Pin PIO2_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin24_sec_mask(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin24_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "0 : Pin PIO2_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin25_sec_mask(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin25_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "0 : Pin PIO2_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin26_sec_mask(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin26_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "0 : Pin PIO2_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin27_sec_mask(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin27_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "0 : Pin PIO2_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin28_sec_mask(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin28_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "0 : Pin PIO2_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin29_sec_mask(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin29_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "0 : Pin PIO2_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin30_sec_mask(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin30_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "0 : Pin PIO2_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio2_pin31_sec_mask(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO2_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio2_pin31_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpioMask2 {
    #[inline(always)]
    fn default() -> SecGpioMask2 {
        SecGpioMask2(0)
    }
}
impl core::fmt::Debug for SecGpioMask2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpioMask2")
            .field("pio2_pin0_sec_mask", &self.pio2_pin0_sec_mask())
            .field("pio2_pin1_sec_mask", &self.pio2_pin1_sec_mask())
            .field("pio2_pin2_sec_mask", &self.pio2_pin2_sec_mask())
            .field("pio2_pin3_sec_mask", &self.pio2_pin3_sec_mask())
            .field("pio2_pin4_sec_mask", &self.pio2_pin4_sec_mask())
            .field("pio2_pin5_sec_mask", &self.pio2_pin5_sec_mask())
            .field("pio2_pin6_sec_mask", &self.pio2_pin6_sec_mask())
            .field("pio2_pin7_sec_mask", &self.pio2_pin7_sec_mask())
            .field("pio2_pin8_sec_mask", &self.pio2_pin8_sec_mask())
            .field("pio2_pin9_sec_mask", &self.pio2_pin9_sec_mask())
            .field("pio2_pin10_sec_mask", &self.pio2_pin10_sec_mask())
            .field("pio2_pin11_sec_mask", &self.pio2_pin11_sec_mask())
            .field("pio2_pin12_sec_mask", &self.pio2_pin12_sec_mask())
            .field("pio2_pin13_sec_mask", &self.pio2_pin13_sec_mask())
            .field("pio2_pin14_sec_mask", &self.pio2_pin14_sec_mask())
            .field("pio2_pin15_sec_mask", &self.pio2_pin15_sec_mask())
            .field("pio2_pin16_sec_mask", &self.pio2_pin16_sec_mask())
            .field("pio2_pin17_sec_mask", &self.pio2_pin17_sec_mask())
            .field("pio2_pin18_sec_mask", &self.pio2_pin18_sec_mask())
            .field("pio2_pin19_sec_mask", &self.pio2_pin19_sec_mask())
            .field("pio2_pin20_sec_mask", &self.pio2_pin20_sec_mask())
            .field("pio2_pin21_sec_mask", &self.pio2_pin21_sec_mask())
            .field("pio2_pin22_sec_mask", &self.pio2_pin22_sec_mask())
            .field("pio2_pin23_sec_mask", &self.pio2_pin23_sec_mask())
            .field("pio2_pin24_sec_mask", &self.pio2_pin24_sec_mask())
            .field("pio2_pin25_sec_mask", &self.pio2_pin25_sec_mask())
            .field("pio2_pin26_sec_mask", &self.pio2_pin26_sec_mask())
            .field("pio2_pin27_sec_mask", &self.pio2_pin27_sec_mask())
            .field("pio2_pin28_sec_mask", &self.pio2_pin28_sec_mask())
            .field("pio2_pin29_sec_mask", &self.pio2_pin29_sec_mask())
            .field("pio2_pin30_sec_mask", &self.pio2_pin30_sec_mask())
            .field("pio2_pin31_sec_mask", &self.pio2_pin31_sec_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpioMask2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpioMask2 {{ pio2_pin0_sec_mask: {=bool:?}, pio2_pin1_sec_mask: {=bool:?}, pio2_pin2_sec_mask: {=bool:?}, pio2_pin3_sec_mask: {=bool:?}, pio2_pin4_sec_mask: {=bool:?}, pio2_pin5_sec_mask: {=bool:?}, pio2_pin6_sec_mask: {=bool:?}, pio2_pin7_sec_mask: {=bool:?}, pio2_pin8_sec_mask: {=bool:?}, pio2_pin9_sec_mask: {=bool:?}, pio2_pin10_sec_mask: {=bool:?}, pio2_pin11_sec_mask: {=bool:?}, pio2_pin12_sec_mask: {=bool:?}, pio2_pin13_sec_mask: {=bool:?}, pio2_pin14_sec_mask: {=bool:?}, pio2_pin15_sec_mask: {=bool:?}, pio2_pin16_sec_mask: {=bool:?}, pio2_pin17_sec_mask: {=bool:?}, pio2_pin18_sec_mask: {=bool:?}, pio2_pin19_sec_mask: {=bool:?}, pio2_pin20_sec_mask: {=bool:?}, pio2_pin21_sec_mask: {=bool:?}, pio2_pin22_sec_mask: {=bool:?}, pio2_pin23_sec_mask: {=bool:?}, pio2_pin24_sec_mask: {=bool:?}, pio2_pin25_sec_mask: {=bool:?}, pio2_pin26_sec_mask: {=bool:?}, pio2_pin27_sec_mask: {=bool:?}, pio2_pin28_sec_mask: {=bool:?}, pio2_pin29_sec_mask: {=bool:?}, pio2_pin30_sec_mask: {=bool:?}, pio2_pin31_sec_mask: {=bool:?} }}",
            self.pio2_pin0_sec_mask(),
            self.pio2_pin1_sec_mask(),
            self.pio2_pin2_sec_mask(),
            self.pio2_pin3_sec_mask(),
            self.pio2_pin4_sec_mask(),
            self.pio2_pin5_sec_mask(),
            self.pio2_pin6_sec_mask(),
            self.pio2_pin7_sec_mask(),
            self.pio2_pin8_sec_mask(),
            self.pio2_pin9_sec_mask(),
            self.pio2_pin10_sec_mask(),
            self.pio2_pin11_sec_mask(),
            self.pio2_pin12_sec_mask(),
            self.pio2_pin13_sec_mask(),
            self.pio2_pin14_sec_mask(),
            self.pio2_pin15_sec_mask(),
            self.pio2_pin16_sec_mask(),
            self.pio2_pin17_sec_mask(),
            self.pio2_pin18_sec_mask(),
            self.pio2_pin19_sec_mask(),
            self.pio2_pin20_sec_mask(),
            self.pio2_pin21_sec_mask(),
            self.pio2_pin22_sec_mask(),
            self.pio2_pin23_sec_mask(),
            self.pio2_pin24_sec_mask(),
            self.pio2_pin25_sec_mask(),
            self.pio2_pin26_sec_mask(),
            self.pio2_pin27_sec_mask(),
            self.pio2_pin28_sec_mask(),
            self.pio2_pin29_sec_mask(),
            self.pio2_pin30_sec_mask(),
            self.pio2_pin31_sec_mask()
        )
    }
}
#[doc = "Secure GPIO mask for port 3 pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpioMask3(pub u32);
impl SecGpioMask3 {
    #[doc = "0 : Pin PIO3_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin0_sec_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin0_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0 : Pin PIO3_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin1_sec_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin1_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0 : Pin PIO3_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin2_sec_mask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin2_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "0 : Pin PIO3_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin3_sec_mask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin3_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "0 : Pin PIO3_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin4_sec_mask(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin4_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "0 : Pin PIO3_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin5_sec_mask(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin5_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "0 : Pin PIO3_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin6_sec_mask(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin6_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "0 : Pin PIO3_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin7_sec_mask(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin7_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "0 : Pin PIO3_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin8_sec_mask(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin8_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "0 : Pin PIO3_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin9_sec_mask(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin9_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "0 : Pin PIO3_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin10_sec_mask(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin10_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 : Pin PIO3_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin11_sec_mask(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin11_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0 : Pin PIO3_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin12_sec_mask(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin12_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "0 : Pin PIO3_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin13_sec_mask(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin13_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "0 : Pin PIO3_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin14_sec_mask(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin14_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "0 : Pin PIO3_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin15_sec_mask(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin15_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "0 : Pin PIO3_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin16_sec_mask(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin16_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "0 : Pin PIO3_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin17_sec_mask(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin17_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 : Pin PIO3_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin18_sec_mask(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin18_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 : Pin PIO3_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin19_sec_mask(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin19_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 : Pin PIO3_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin20_sec_mask(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin20_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "0 : Pin PIO3_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin21_sec_mask(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin21_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "0 : Pin PIO3_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin22_sec_mask(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin22_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "0 : Pin PIO3_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin23_sec_mask(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin23_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "0 : Pin PIO3_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin24_sec_mask(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin24_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "0 : Pin PIO3_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin25_sec_mask(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin25_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "0 : Pin PIO3_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin26_sec_mask(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin26_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "0 : Pin PIO3_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin27_sec_mask(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin27_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "0 : Pin PIO3_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin28_sec_mask(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin28_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "0 : Pin PIO3_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin29_sec_mask(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin29_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "0 : Pin PIO3_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin30_sec_mask(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin30_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "0 : Pin PIO3_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio3_pin31_sec_mask(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO3_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio3_pin31_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpioMask3 {
    #[inline(always)]
    fn default() -> SecGpioMask3 {
        SecGpioMask3(0)
    }
}
impl core::fmt::Debug for SecGpioMask3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpioMask3")
            .field("pio3_pin0_sec_mask", &self.pio3_pin0_sec_mask())
            .field("pio3_pin1_sec_mask", &self.pio3_pin1_sec_mask())
            .field("pio3_pin2_sec_mask", &self.pio3_pin2_sec_mask())
            .field("pio3_pin3_sec_mask", &self.pio3_pin3_sec_mask())
            .field("pio3_pin4_sec_mask", &self.pio3_pin4_sec_mask())
            .field("pio3_pin5_sec_mask", &self.pio3_pin5_sec_mask())
            .field("pio3_pin6_sec_mask", &self.pio3_pin6_sec_mask())
            .field("pio3_pin7_sec_mask", &self.pio3_pin7_sec_mask())
            .field("pio3_pin8_sec_mask", &self.pio3_pin8_sec_mask())
            .field("pio3_pin9_sec_mask", &self.pio3_pin9_sec_mask())
            .field("pio3_pin10_sec_mask", &self.pio3_pin10_sec_mask())
            .field("pio3_pin11_sec_mask", &self.pio3_pin11_sec_mask())
            .field("pio3_pin12_sec_mask", &self.pio3_pin12_sec_mask())
            .field("pio3_pin13_sec_mask", &self.pio3_pin13_sec_mask())
            .field("pio3_pin14_sec_mask", &self.pio3_pin14_sec_mask())
            .field("pio3_pin15_sec_mask", &self.pio3_pin15_sec_mask())
            .field("pio3_pin16_sec_mask", &self.pio3_pin16_sec_mask())
            .field("pio3_pin17_sec_mask", &self.pio3_pin17_sec_mask())
            .field("pio3_pin18_sec_mask", &self.pio3_pin18_sec_mask())
            .field("pio3_pin19_sec_mask", &self.pio3_pin19_sec_mask())
            .field("pio3_pin20_sec_mask", &self.pio3_pin20_sec_mask())
            .field("pio3_pin21_sec_mask", &self.pio3_pin21_sec_mask())
            .field("pio3_pin22_sec_mask", &self.pio3_pin22_sec_mask())
            .field("pio3_pin23_sec_mask", &self.pio3_pin23_sec_mask())
            .field("pio3_pin24_sec_mask", &self.pio3_pin24_sec_mask())
            .field("pio3_pin25_sec_mask", &self.pio3_pin25_sec_mask())
            .field("pio3_pin26_sec_mask", &self.pio3_pin26_sec_mask())
            .field("pio3_pin27_sec_mask", &self.pio3_pin27_sec_mask())
            .field("pio3_pin28_sec_mask", &self.pio3_pin28_sec_mask())
            .field("pio3_pin29_sec_mask", &self.pio3_pin29_sec_mask())
            .field("pio3_pin30_sec_mask", &self.pio3_pin30_sec_mask())
            .field("pio3_pin31_sec_mask", &self.pio3_pin31_sec_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpioMask3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpioMask3 {{ pio3_pin0_sec_mask: {=bool:?}, pio3_pin1_sec_mask: {=bool:?}, pio3_pin2_sec_mask: {=bool:?}, pio3_pin3_sec_mask: {=bool:?}, pio3_pin4_sec_mask: {=bool:?}, pio3_pin5_sec_mask: {=bool:?}, pio3_pin6_sec_mask: {=bool:?}, pio3_pin7_sec_mask: {=bool:?}, pio3_pin8_sec_mask: {=bool:?}, pio3_pin9_sec_mask: {=bool:?}, pio3_pin10_sec_mask: {=bool:?}, pio3_pin11_sec_mask: {=bool:?}, pio3_pin12_sec_mask: {=bool:?}, pio3_pin13_sec_mask: {=bool:?}, pio3_pin14_sec_mask: {=bool:?}, pio3_pin15_sec_mask: {=bool:?}, pio3_pin16_sec_mask: {=bool:?}, pio3_pin17_sec_mask: {=bool:?}, pio3_pin18_sec_mask: {=bool:?}, pio3_pin19_sec_mask: {=bool:?}, pio3_pin20_sec_mask: {=bool:?}, pio3_pin21_sec_mask: {=bool:?}, pio3_pin22_sec_mask: {=bool:?}, pio3_pin23_sec_mask: {=bool:?}, pio3_pin24_sec_mask: {=bool:?}, pio3_pin25_sec_mask: {=bool:?}, pio3_pin26_sec_mask: {=bool:?}, pio3_pin27_sec_mask: {=bool:?}, pio3_pin28_sec_mask: {=bool:?}, pio3_pin29_sec_mask: {=bool:?}, pio3_pin30_sec_mask: {=bool:?}, pio3_pin31_sec_mask: {=bool:?} }}",
            self.pio3_pin0_sec_mask(),
            self.pio3_pin1_sec_mask(),
            self.pio3_pin2_sec_mask(),
            self.pio3_pin3_sec_mask(),
            self.pio3_pin4_sec_mask(),
            self.pio3_pin5_sec_mask(),
            self.pio3_pin6_sec_mask(),
            self.pio3_pin7_sec_mask(),
            self.pio3_pin8_sec_mask(),
            self.pio3_pin9_sec_mask(),
            self.pio3_pin10_sec_mask(),
            self.pio3_pin11_sec_mask(),
            self.pio3_pin12_sec_mask(),
            self.pio3_pin13_sec_mask(),
            self.pio3_pin14_sec_mask(),
            self.pio3_pin15_sec_mask(),
            self.pio3_pin16_sec_mask(),
            self.pio3_pin17_sec_mask(),
            self.pio3_pin18_sec_mask(),
            self.pio3_pin19_sec_mask(),
            self.pio3_pin20_sec_mask(),
            self.pio3_pin21_sec_mask(),
            self.pio3_pin22_sec_mask(),
            self.pio3_pin23_sec_mask(),
            self.pio3_pin24_sec_mask(),
            self.pio3_pin25_sec_mask(),
            self.pio3_pin26_sec_mask(),
            self.pio3_pin27_sec_mask(),
            self.pio3_pin28_sec_mask(),
            self.pio3_pin29_sec_mask(),
            self.pio3_pin30_sec_mask(),
            self.pio3_pin31_sec_mask()
        )
    }
}
#[doc = "Secure GPIO mask for port 4 pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpioMask4(pub u32);
impl SecGpioMask4 {
    #[doc = "0 : Pin PIO4_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin0_sec_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin0_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0 : Pin PIO4_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin1_sec_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin1_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0 : Pin PIO4_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin2_sec_mask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin2_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "0 : Pin PIO4_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin3_sec_mask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin3_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "0 : Pin PIO4_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin4_sec_mask(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin4_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "0 : Pin PIO4_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin5_sec_mask(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin5_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "0 : Pin PIO4_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin6_sec_mask(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin6_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "0 : Pin PIO4_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin7_sec_mask(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin7_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "0 : Pin PIO4_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin8_sec_mask(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin8_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "0 : Pin PIO4_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin9_sec_mask(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin9_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "0 : Pin PIO4_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin10_sec_mask(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin10_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 : Pin PIO4_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin11_sec_mask(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin11_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0 : Pin PIO4_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin12_sec_mask(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin12_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "0 : Pin PIO4_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin13_sec_mask(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin13_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "0 : Pin PIO4_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin14_sec_mask(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin14_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "0 : Pin PIO4_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin15_sec_mask(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin15_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "0 : Pin PIO4_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin16_sec_mask(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin16_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "0 : Pin PIO4_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin17_sec_mask(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin17_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 : Pin PIO4_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin18_sec_mask(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin18_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 : Pin PIO4_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin19_sec_mask(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin19_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 : Pin PIO4_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin20_sec_mask(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin20_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "0 : Pin PIO4_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin21_sec_mask(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin21_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "0 : Pin PIO4_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin22_sec_mask(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin22_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "0 : Pin PIO4_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin23_sec_mask(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin23_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "0 : Pin PIO4_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin24_sec_mask(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin24_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "0 : Pin PIO4_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin25_sec_mask(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin25_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "0 : Pin PIO4_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin26_sec_mask(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin26_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "0 : Pin PIO4_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin27_sec_mask(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin27_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "0 : Pin PIO4_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin28_sec_mask(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin28_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "0 : Pin PIO4_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin29_sec_mask(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin29_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "0 : Pin PIO4_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin30_sec_mask(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin30_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "0 : Pin PIO4_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio4_pin31_sec_mask(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO4_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio4_pin31_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpioMask4 {
    #[inline(always)]
    fn default() -> SecGpioMask4 {
        SecGpioMask4(0)
    }
}
impl core::fmt::Debug for SecGpioMask4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpioMask4")
            .field("pio4_pin0_sec_mask", &self.pio4_pin0_sec_mask())
            .field("pio4_pin1_sec_mask", &self.pio4_pin1_sec_mask())
            .field("pio4_pin2_sec_mask", &self.pio4_pin2_sec_mask())
            .field("pio4_pin3_sec_mask", &self.pio4_pin3_sec_mask())
            .field("pio4_pin4_sec_mask", &self.pio4_pin4_sec_mask())
            .field("pio4_pin5_sec_mask", &self.pio4_pin5_sec_mask())
            .field("pio4_pin6_sec_mask", &self.pio4_pin6_sec_mask())
            .field("pio4_pin7_sec_mask", &self.pio4_pin7_sec_mask())
            .field("pio4_pin8_sec_mask", &self.pio4_pin8_sec_mask())
            .field("pio4_pin9_sec_mask", &self.pio4_pin9_sec_mask())
            .field("pio4_pin10_sec_mask", &self.pio4_pin10_sec_mask())
            .field("pio4_pin11_sec_mask", &self.pio4_pin11_sec_mask())
            .field("pio4_pin12_sec_mask", &self.pio4_pin12_sec_mask())
            .field("pio4_pin13_sec_mask", &self.pio4_pin13_sec_mask())
            .field("pio4_pin14_sec_mask", &self.pio4_pin14_sec_mask())
            .field("pio4_pin15_sec_mask", &self.pio4_pin15_sec_mask())
            .field("pio4_pin16_sec_mask", &self.pio4_pin16_sec_mask())
            .field("pio4_pin17_sec_mask", &self.pio4_pin17_sec_mask())
            .field("pio4_pin18_sec_mask", &self.pio4_pin18_sec_mask())
            .field("pio4_pin19_sec_mask", &self.pio4_pin19_sec_mask())
            .field("pio4_pin20_sec_mask", &self.pio4_pin20_sec_mask())
            .field("pio4_pin21_sec_mask", &self.pio4_pin21_sec_mask())
            .field("pio4_pin22_sec_mask", &self.pio4_pin22_sec_mask())
            .field("pio4_pin23_sec_mask", &self.pio4_pin23_sec_mask())
            .field("pio4_pin24_sec_mask", &self.pio4_pin24_sec_mask())
            .field("pio4_pin25_sec_mask", &self.pio4_pin25_sec_mask())
            .field("pio4_pin26_sec_mask", &self.pio4_pin26_sec_mask())
            .field("pio4_pin27_sec_mask", &self.pio4_pin27_sec_mask())
            .field("pio4_pin28_sec_mask", &self.pio4_pin28_sec_mask())
            .field("pio4_pin29_sec_mask", &self.pio4_pin29_sec_mask())
            .field("pio4_pin30_sec_mask", &self.pio4_pin30_sec_mask())
            .field("pio4_pin31_sec_mask", &self.pio4_pin31_sec_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpioMask4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpioMask4 {{ pio4_pin0_sec_mask: {=bool:?}, pio4_pin1_sec_mask: {=bool:?}, pio4_pin2_sec_mask: {=bool:?}, pio4_pin3_sec_mask: {=bool:?}, pio4_pin4_sec_mask: {=bool:?}, pio4_pin5_sec_mask: {=bool:?}, pio4_pin6_sec_mask: {=bool:?}, pio4_pin7_sec_mask: {=bool:?}, pio4_pin8_sec_mask: {=bool:?}, pio4_pin9_sec_mask: {=bool:?}, pio4_pin10_sec_mask: {=bool:?}, pio4_pin11_sec_mask: {=bool:?}, pio4_pin12_sec_mask: {=bool:?}, pio4_pin13_sec_mask: {=bool:?}, pio4_pin14_sec_mask: {=bool:?}, pio4_pin15_sec_mask: {=bool:?}, pio4_pin16_sec_mask: {=bool:?}, pio4_pin17_sec_mask: {=bool:?}, pio4_pin18_sec_mask: {=bool:?}, pio4_pin19_sec_mask: {=bool:?}, pio4_pin20_sec_mask: {=bool:?}, pio4_pin21_sec_mask: {=bool:?}, pio4_pin22_sec_mask: {=bool:?}, pio4_pin23_sec_mask: {=bool:?}, pio4_pin24_sec_mask: {=bool:?}, pio4_pin25_sec_mask: {=bool:?}, pio4_pin26_sec_mask: {=bool:?}, pio4_pin27_sec_mask: {=bool:?}, pio4_pin28_sec_mask: {=bool:?}, pio4_pin29_sec_mask: {=bool:?}, pio4_pin30_sec_mask: {=bool:?}, pio4_pin31_sec_mask: {=bool:?} }}",
            self.pio4_pin0_sec_mask(),
            self.pio4_pin1_sec_mask(),
            self.pio4_pin2_sec_mask(),
            self.pio4_pin3_sec_mask(),
            self.pio4_pin4_sec_mask(),
            self.pio4_pin5_sec_mask(),
            self.pio4_pin6_sec_mask(),
            self.pio4_pin7_sec_mask(),
            self.pio4_pin8_sec_mask(),
            self.pio4_pin9_sec_mask(),
            self.pio4_pin10_sec_mask(),
            self.pio4_pin11_sec_mask(),
            self.pio4_pin12_sec_mask(),
            self.pio4_pin13_sec_mask(),
            self.pio4_pin14_sec_mask(),
            self.pio4_pin15_sec_mask(),
            self.pio4_pin16_sec_mask(),
            self.pio4_pin17_sec_mask(),
            self.pio4_pin18_sec_mask(),
            self.pio4_pin19_sec_mask(),
            self.pio4_pin20_sec_mask(),
            self.pio4_pin21_sec_mask(),
            self.pio4_pin22_sec_mask(),
            self.pio4_pin23_sec_mask(),
            self.pio4_pin24_sec_mask(),
            self.pio4_pin25_sec_mask(),
            self.pio4_pin26_sec_mask(),
            self.pio4_pin27_sec_mask(),
            self.pio4_pin28_sec_mask(),
            self.pio4_pin29_sec_mask(),
            self.pio4_pin30_sec_mask(),
            self.pio4_pin31_sec_mask()
        )
    }
}
#[doc = "Secure GPIO mask for port 5 pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpioMask5(pub u32);
impl SecGpioMask5 {
    #[doc = "0 : Pin PIO5_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin0_sec_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin0_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0 : Pin PIO5_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin1_sec_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin1_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0 : Pin PIO5_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin2_sec_mask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin2_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "0 : Pin PIO5_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin3_sec_mask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin3_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "0 : Pin PIO5_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin4_sec_mask(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin4_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "0 : Pin PIO5_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin5_sec_mask(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin5_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "0 : Pin PIO5_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin6_sec_mask(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin6_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "0 : Pin PIO5_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin7_sec_mask(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin7_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "0 : Pin PIO5_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin8_sec_mask(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin8_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "0 : Pin PIO5_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin9_sec_mask(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin9_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "0 : Pin PIO5_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin10_sec_mask(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin10_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 : Pin PIO5_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin11_sec_mask(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin11_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0 : Pin PIO5_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin12_sec_mask(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin12_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "0 : Pin PIO5_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin13_sec_mask(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin13_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "0 : Pin PIO5_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin14_sec_mask(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin14_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "0 : Pin PIO5_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin15_sec_mask(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin15_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "0 : Pin PIO5_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin16_sec_mask(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin16_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "0 : Pin PIO5_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin17_sec_mask(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin17_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 : Pin PIO5_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin18_sec_mask(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin18_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 : Pin PIO5_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin19_sec_mask(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin19_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 : Pin PIO5_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin20_sec_mask(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin20_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "0 : Pin PIO5_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin21_sec_mask(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin21_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "0 : Pin PIO5_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin22_sec_mask(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin22_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "0 : Pin PIO5_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin23_sec_mask(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin23_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "0 : Pin PIO5_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin24_sec_mask(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin24_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "0 : Pin PIO5_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin25_sec_mask(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin25_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "0 : Pin PIO5_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin26_sec_mask(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin26_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "0 : Pin PIO5_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin27_sec_mask(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin27_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "0 : Pin PIO5_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin28_sec_mask(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin28_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "0 : Pin PIO5_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin29_sec_mask(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin29_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "0 : Pin PIO5_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin30_sec_mask(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin30_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "0 : Pin PIO5_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio5_pin31_sec_mask(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO5_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio5_pin31_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpioMask5 {
    #[inline(always)]
    fn default() -> SecGpioMask5 {
        SecGpioMask5(0)
    }
}
impl core::fmt::Debug for SecGpioMask5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpioMask5")
            .field("pio5_pin0_sec_mask", &self.pio5_pin0_sec_mask())
            .field("pio5_pin1_sec_mask", &self.pio5_pin1_sec_mask())
            .field("pio5_pin2_sec_mask", &self.pio5_pin2_sec_mask())
            .field("pio5_pin3_sec_mask", &self.pio5_pin3_sec_mask())
            .field("pio5_pin4_sec_mask", &self.pio5_pin4_sec_mask())
            .field("pio5_pin5_sec_mask", &self.pio5_pin5_sec_mask())
            .field("pio5_pin6_sec_mask", &self.pio5_pin6_sec_mask())
            .field("pio5_pin7_sec_mask", &self.pio5_pin7_sec_mask())
            .field("pio5_pin8_sec_mask", &self.pio5_pin8_sec_mask())
            .field("pio5_pin9_sec_mask", &self.pio5_pin9_sec_mask())
            .field("pio5_pin10_sec_mask", &self.pio5_pin10_sec_mask())
            .field("pio5_pin11_sec_mask", &self.pio5_pin11_sec_mask())
            .field("pio5_pin12_sec_mask", &self.pio5_pin12_sec_mask())
            .field("pio5_pin13_sec_mask", &self.pio5_pin13_sec_mask())
            .field("pio5_pin14_sec_mask", &self.pio5_pin14_sec_mask())
            .field("pio5_pin15_sec_mask", &self.pio5_pin15_sec_mask())
            .field("pio5_pin16_sec_mask", &self.pio5_pin16_sec_mask())
            .field("pio5_pin17_sec_mask", &self.pio5_pin17_sec_mask())
            .field("pio5_pin18_sec_mask", &self.pio5_pin18_sec_mask())
            .field("pio5_pin19_sec_mask", &self.pio5_pin19_sec_mask())
            .field("pio5_pin20_sec_mask", &self.pio5_pin20_sec_mask())
            .field("pio5_pin21_sec_mask", &self.pio5_pin21_sec_mask())
            .field("pio5_pin22_sec_mask", &self.pio5_pin22_sec_mask())
            .field("pio5_pin23_sec_mask", &self.pio5_pin23_sec_mask())
            .field("pio5_pin24_sec_mask", &self.pio5_pin24_sec_mask())
            .field("pio5_pin25_sec_mask", &self.pio5_pin25_sec_mask())
            .field("pio5_pin26_sec_mask", &self.pio5_pin26_sec_mask())
            .field("pio5_pin27_sec_mask", &self.pio5_pin27_sec_mask())
            .field("pio5_pin28_sec_mask", &self.pio5_pin28_sec_mask())
            .field("pio5_pin29_sec_mask", &self.pio5_pin29_sec_mask())
            .field("pio5_pin30_sec_mask", &self.pio5_pin30_sec_mask())
            .field("pio5_pin31_sec_mask", &self.pio5_pin31_sec_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpioMask5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpioMask5 {{ pio5_pin0_sec_mask: {=bool:?}, pio5_pin1_sec_mask: {=bool:?}, pio5_pin2_sec_mask: {=bool:?}, pio5_pin3_sec_mask: {=bool:?}, pio5_pin4_sec_mask: {=bool:?}, pio5_pin5_sec_mask: {=bool:?}, pio5_pin6_sec_mask: {=bool:?}, pio5_pin7_sec_mask: {=bool:?}, pio5_pin8_sec_mask: {=bool:?}, pio5_pin9_sec_mask: {=bool:?}, pio5_pin10_sec_mask: {=bool:?}, pio5_pin11_sec_mask: {=bool:?}, pio5_pin12_sec_mask: {=bool:?}, pio5_pin13_sec_mask: {=bool:?}, pio5_pin14_sec_mask: {=bool:?}, pio5_pin15_sec_mask: {=bool:?}, pio5_pin16_sec_mask: {=bool:?}, pio5_pin17_sec_mask: {=bool:?}, pio5_pin18_sec_mask: {=bool:?}, pio5_pin19_sec_mask: {=bool:?}, pio5_pin20_sec_mask: {=bool:?}, pio5_pin21_sec_mask: {=bool:?}, pio5_pin22_sec_mask: {=bool:?}, pio5_pin23_sec_mask: {=bool:?}, pio5_pin24_sec_mask: {=bool:?}, pio5_pin25_sec_mask: {=bool:?}, pio5_pin26_sec_mask: {=bool:?}, pio5_pin27_sec_mask: {=bool:?}, pio5_pin28_sec_mask: {=bool:?}, pio5_pin29_sec_mask: {=bool:?}, pio5_pin30_sec_mask: {=bool:?}, pio5_pin31_sec_mask: {=bool:?} }}",
            self.pio5_pin0_sec_mask(),
            self.pio5_pin1_sec_mask(),
            self.pio5_pin2_sec_mask(),
            self.pio5_pin3_sec_mask(),
            self.pio5_pin4_sec_mask(),
            self.pio5_pin5_sec_mask(),
            self.pio5_pin6_sec_mask(),
            self.pio5_pin7_sec_mask(),
            self.pio5_pin8_sec_mask(),
            self.pio5_pin9_sec_mask(),
            self.pio5_pin10_sec_mask(),
            self.pio5_pin11_sec_mask(),
            self.pio5_pin12_sec_mask(),
            self.pio5_pin13_sec_mask(),
            self.pio5_pin14_sec_mask(),
            self.pio5_pin15_sec_mask(),
            self.pio5_pin16_sec_mask(),
            self.pio5_pin17_sec_mask(),
            self.pio5_pin18_sec_mask(),
            self.pio5_pin19_sec_mask(),
            self.pio5_pin20_sec_mask(),
            self.pio5_pin21_sec_mask(),
            self.pio5_pin22_sec_mask(),
            self.pio5_pin23_sec_mask(),
            self.pio5_pin24_sec_mask(),
            self.pio5_pin25_sec_mask(),
            self.pio5_pin26_sec_mask(),
            self.pio5_pin27_sec_mask(),
            self.pio5_pin28_sec_mask(),
            self.pio5_pin29_sec_mask(),
            self.pio5_pin30_sec_mask(),
            self.pio5_pin31_sec_mask()
        )
    }
}
#[doc = "Secure GPIO mask for port 6 pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpioMask6(pub u32);
impl SecGpioMask6 {
    #[doc = "0 : Pin PIO6_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin0_sec_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin0_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0 : Pin PIO6_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin1_sec_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin1_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0 : Pin PIO6_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin2_sec_mask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin2_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "0 : Pin PIO6_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin3_sec_mask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin3_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "0 : Pin PIO6_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin4_sec_mask(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin4_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "0 : Pin PIO6_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin5_sec_mask(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin5_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "0 : Pin PIO6_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin6_sec_mask(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin6_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "0 : Pin PIO6_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin7_sec_mask(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin7_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "0 : Pin PIO6_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin8_sec_mask(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin8_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "0 : Pin PIO6_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin9_sec_mask(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin9_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "0 : Pin PIO6_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin10_sec_mask(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin10_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 : Pin PIO6_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin11_sec_mask(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin11_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0 : Pin PIO6_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin12_sec_mask(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin12_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "0 : Pin PIO6_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin13_sec_mask(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin13_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "0 : Pin PIO6_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin14_sec_mask(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin14_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "0 : Pin PIO6_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin15_sec_mask(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin15_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "0 : Pin PIO6_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin16_sec_mask(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin16_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "0 : Pin PIO6_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin17_sec_mask(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin17_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 : Pin PIO6_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin18_sec_mask(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin18_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 : Pin PIO6_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin19_sec_mask(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin19_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 : Pin PIO6_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin20_sec_mask(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin20_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "0 : Pin PIO6_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin21_sec_mask(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin21_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "0 : Pin PIO6_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin22_sec_mask(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin22_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "0 : Pin PIO6_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin23_sec_mask(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin23_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "0 : Pin PIO6_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin24_sec_mask(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin24_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "0 : Pin PIO6_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin25_sec_mask(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin25_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "0 : Pin PIO6_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin26_sec_mask(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin26_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "0 : Pin PIO6_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin27_sec_mask(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin27_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "0 : Pin PIO6_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin28_sec_mask(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin28_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "0 : Pin PIO6_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin29_sec_mask(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin29_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "0 : Pin PIO6_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin30_sec_mask(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin30_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "0 : Pin PIO6_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio6_pin31_sec_mask(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO6_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio6_pin31_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpioMask6 {
    #[inline(always)]
    fn default() -> SecGpioMask6 {
        SecGpioMask6(0)
    }
}
impl core::fmt::Debug for SecGpioMask6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpioMask6")
            .field("pio6_pin0_sec_mask", &self.pio6_pin0_sec_mask())
            .field("pio6_pin1_sec_mask", &self.pio6_pin1_sec_mask())
            .field("pio6_pin2_sec_mask", &self.pio6_pin2_sec_mask())
            .field("pio6_pin3_sec_mask", &self.pio6_pin3_sec_mask())
            .field("pio6_pin4_sec_mask", &self.pio6_pin4_sec_mask())
            .field("pio6_pin5_sec_mask", &self.pio6_pin5_sec_mask())
            .field("pio6_pin6_sec_mask", &self.pio6_pin6_sec_mask())
            .field("pio6_pin7_sec_mask", &self.pio6_pin7_sec_mask())
            .field("pio6_pin8_sec_mask", &self.pio6_pin8_sec_mask())
            .field("pio6_pin9_sec_mask", &self.pio6_pin9_sec_mask())
            .field("pio6_pin10_sec_mask", &self.pio6_pin10_sec_mask())
            .field("pio6_pin11_sec_mask", &self.pio6_pin11_sec_mask())
            .field("pio6_pin12_sec_mask", &self.pio6_pin12_sec_mask())
            .field("pio6_pin13_sec_mask", &self.pio6_pin13_sec_mask())
            .field("pio6_pin14_sec_mask", &self.pio6_pin14_sec_mask())
            .field("pio6_pin15_sec_mask", &self.pio6_pin15_sec_mask())
            .field("pio6_pin16_sec_mask", &self.pio6_pin16_sec_mask())
            .field("pio6_pin17_sec_mask", &self.pio6_pin17_sec_mask())
            .field("pio6_pin18_sec_mask", &self.pio6_pin18_sec_mask())
            .field("pio6_pin19_sec_mask", &self.pio6_pin19_sec_mask())
            .field("pio6_pin20_sec_mask", &self.pio6_pin20_sec_mask())
            .field("pio6_pin21_sec_mask", &self.pio6_pin21_sec_mask())
            .field("pio6_pin22_sec_mask", &self.pio6_pin22_sec_mask())
            .field("pio6_pin23_sec_mask", &self.pio6_pin23_sec_mask())
            .field("pio6_pin24_sec_mask", &self.pio6_pin24_sec_mask())
            .field("pio6_pin25_sec_mask", &self.pio6_pin25_sec_mask())
            .field("pio6_pin26_sec_mask", &self.pio6_pin26_sec_mask())
            .field("pio6_pin27_sec_mask", &self.pio6_pin27_sec_mask())
            .field("pio6_pin28_sec_mask", &self.pio6_pin28_sec_mask())
            .field("pio6_pin29_sec_mask", &self.pio6_pin29_sec_mask())
            .field("pio6_pin30_sec_mask", &self.pio6_pin30_sec_mask())
            .field("pio6_pin31_sec_mask", &self.pio6_pin31_sec_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpioMask6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpioMask6 {{ pio6_pin0_sec_mask: {=bool:?}, pio6_pin1_sec_mask: {=bool:?}, pio6_pin2_sec_mask: {=bool:?}, pio6_pin3_sec_mask: {=bool:?}, pio6_pin4_sec_mask: {=bool:?}, pio6_pin5_sec_mask: {=bool:?}, pio6_pin6_sec_mask: {=bool:?}, pio6_pin7_sec_mask: {=bool:?}, pio6_pin8_sec_mask: {=bool:?}, pio6_pin9_sec_mask: {=bool:?}, pio6_pin10_sec_mask: {=bool:?}, pio6_pin11_sec_mask: {=bool:?}, pio6_pin12_sec_mask: {=bool:?}, pio6_pin13_sec_mask: {=bool:?}, pio6_pin14_sec_mask: {=bool:?}, pio6_pin15_sec_mask: {=bool:?}, pio6_pin16_sec_mask: {=bool:?}, pio6_pin17_sec_mask: {=bool:?}, pio6_pin18_sec_mask: {=bool:?}, pio6_pin19_sec_mask: {=bool:?}, pio6_pin20_sec_mask: {=bool:?}, pio6_pin21_sec_mask: {=bool:?}, pio6_pin22_sec_mask: {=bool:?}, pio6_pin23_sec_mask: {=bool:?}, pio6_pin24_sec_mask: {=bool:?}, pio6_pin25_sec_mask: {=bool:?}, pio6_pin26_sec_mask: {=bool:?}, pio6_pin27_sec_mask: {=bool:?}, pio6_pin28_sec_mask: {=bool:?}, pio6_pin29_sec_mask: {=bool:?}, pio6_pin30_sec_mask: {=bool:?}, pio6_pin31_sec_mask: {=bool:?} }}",
            self.pio6_pin0_sec_mask(),
            self.pio6_pin1_sec_mask(),
            self.pio6_pin2_sec_mask(),
            self.pio6_pin3_sec_mask(),
            self.pio6_pin4_sec_mask(),
            self.pio6_pin5_sec_mask(),
            self.pio6_pin6_sec_mask(),
            self.pio6_pin7_sec_mask(),
            self.pio6_pin8_sec_mask(),
            self.pio6_pin9_sec_mask(),
            self.pio6_pin10_sec_mask(),
            self.pio6_pin11_sec_mask(),
            self.pio6_pin12_sec_mask(),
            self.pio6_pin13_sec_mask(),
            self.pio6_pin14_sec_mask(),
            self.pio6_pin15_sec_mask(),
            self.pio6_pin16_sec_mask(),
            self.pio6_pin17_sec_mask(),
            self.pio6_pin18_sec_mask(),
            self.pio6_pin19_sec_mask(),
            self.pio6_pin20_sec_mask(),
            self.pio6_pin21_sec_mask(),
            self.pio6_pin22_sec_mask(),
            self.pio6_pin23_sec_mask(),
            self.pio6_pin24_sec_mask(),
            self.pio6_pin25_sec_mask(),
            self.pio6_pin26_sec_mask(),
            self.pio6_pin27_sec_mask(),
            self.pio6_pin28_sec_mask(),
            self.pio6_pin29_sec_mask(),
            self.pio6_pin30_sec_mask(),
            self.pio6_pin31_sec_mask()
        )
    }
}
#[doc = "Secure GPIO mask for port 7 pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpioMask7(pub u32);
impl SecGpioMask7 {
    #[doc = "0 : Pin PIO7_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin0_sec_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin0_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0 : Pin PIO7_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin1_sec_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin1_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0 : Pin PIO7_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin2_sec_mask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin2_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "0 : Pin PIO7_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin3_sec_mask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin3_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "0 : Pin PIO7_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin4_sec_mask(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin4_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "0 : Pin PIO7_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin5_sec_mask(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin5_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "0 : Pin PIO7_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin6_sec_mask(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin6_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "0 : Pin PIO7_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin7_sec_mask(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin7_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "0 : Pin PIO7_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin8_sec_mask(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin8_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "0 : Pin PIO7_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin9_sec_mask(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin9_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "0 : Pin PIO7_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin10_sec_mask(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin10_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 : Pin PIO7_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin11_sec_mask(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin11_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0 : Pin PIO7_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin12_sec_mask(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin12_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "0 : Pin PIO7_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin13_sec_mask(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin13_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "0 : Pin PIO7_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin14_sec_mask(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin14_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "0 : Pin PIO7_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin15_sec_mask(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin15_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "0 : Pin PIO7_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin16_sec_mask(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin16_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "0 : Pin PIO7_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin17_sec_mask(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin17_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 : Pin PIO7_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin18_sec_mask(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin18_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 : Pin PIO7_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin19_sec_mask(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin19_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 : Pin PIO7_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin20_sec_mask(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin20_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "0 : Pin PIO7_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin21_sec_mask(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin21_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "0 : Pin PIO7_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin22_sec_mask(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin22_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "0 : Pin PIO7_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin23_sec_mask(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin23_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "0 : Pin PIO7_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin24_sec_mask(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin24_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "0 : Pin PIO7_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin25_sec_mask(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin25_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "0 : Pin PIO7_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin26_sec_mask(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin26_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "0 : Pin PIO7_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin27_sec_mask(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin27_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "0 : Pin PIO7_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin28_sec_mask(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin28_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "0 : Pin PIO7_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin29_sec_mask(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin29_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "0 : Pin PIO7_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin30_sec_mask(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin30_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "0 : Pin PIO7_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[must_use]
    #[inline(always)]
    pub const fn pio7_pin31_sec_mask(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0 : Pin PIO7_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline(always)]
    pub const fn set_pio7_pin31_sec_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpioMask7 {
    #[inline(always)]
    fn default() -> SecGpioMask7 {
        SecGpioMask7(0)
    }
}
impl core::fmt::Debug for SecGpioMask7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpioMask7")
            .field("pio7_pin0_sec_mask", &self.pio7_pin0_sec_mask())
            .field("pio7_pin1_sec_mask", &self.pio7_pin1_sec_mask())
            .field("pio7_pin2_sec_mask", &self.pio7_pin2_sec_mask())
            .field("pio7_pin3_sec_mask", &self.pio7_pin3_sec_mask())
            .field("pio7_pin4_sec_mask", &self.pio7_pin4_sec_mask())
            .field("pio7_pin5_sec_mask", &self.pio7_pin5_sec_mask())
            .field("pio7_pin6_sec_mask", &self.pio7_pin6_sec_mask())
            .field("pio7_pin7_sec_mask", &self.pio7_pin7_sec_mask())
            .field("pio7_pin8_sec_mask", &self.pio7_pin8_sec_mask())
            .field("pio7_pin9_sec_mask", &self.pio7_pin9_sec_mask())
            .field("pio7_pin10_sec_mask", &self.pio7_pin10_sec_mask())
            .field("pio7_pin11_sec_mask", &self.pio7_pin11_sec_mask())
            .field("pio7_pin12_sec_mask", &self.pio7_pin12_sec_mask())
            .field("pio7_pin13_sec_mask", &self.pio7_pin13_sec_mask())
            .field("pio7_pin14_sec_mask", &self.pio7_pin14_sec_mask())
            .field("pio7_pin15_sec_mask", &self.pio7_pin15_sec_mask())
            .field("pio7_pin16_sec_mask", &self.pio7_pin16_sec_mask())
            .field("pio7_pin17_sec_mask", &self.pio7_pin17_sec_mask())
            .field("pio7_pin18_sec_mask", &self.pio7_pin18_sec_mask())
            .field("pio7_pin19_sec_mask", &self.pio7_pin19_sec_mask())
            .field("pio7_pin20_sec_mask", &self.pio7_pin20_sec_mask())
            .field("pio7_pin21_sec_mask", &self.pio7_pin21_sec_mask())
            .field("pio7_pin22_sec_mask", &self.pio7_pin22_sec_mask())
            .field("pio7_pin23_sec_mask", &self.pio7_pin23_sec_mask())
            .field("pio7_pin24_sec_mask", &self.pio7_pin24_sec_mask())
            .field("pio7_pin25_sec_mask", &self.pio7_pin25_sec_mask())
            .field("pio7_pin26_sec_mask", &self.pio7_pin26_sec_mask())
            .field("pio7_pin27_sec_mask", &self.pio7_pin27_sec_mask())
            .field("pio7_pin28_sec_mask", &self.pio7_pin28_sec_mask())
            .field("pio7_pin29_sec_mask", &self.pio7_pin29_sec_mask())
            .field("pio7_pin30_sec_mask", &self.pio7_pin30_sec_mask())
            .field("pio7_pin31_sec_mask", &self.pio7_pin31_sec_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpioMask7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpioMask7 {{ pio7_pin0_sec_mask: {=bool:?}, pio7_pin1_sec_mask: {=bool:?}, pio7_pin2_sec_mask: {=bool:?}, pio7_pin3_sec_mask: {=bool:?}, pio7_pin4_sec_mask: {=bool:?}, pio7_pin5_sec_mask: {=bool:?}, pio7_pin6_sec_mask: {=bool:?}, pio7_pin7_sec_mask: {=bool:?}, pio7_pin8_sec_mask: {=bool:?}, pio7_pin9_sec_mask: {=bool:?}, pio7_pin10_sec_mask: {=bool:?}, pio7_pin11_sec_mask: {=bool:?}, pio7_pin12_sec_mask: {=bool:?}, pio7_pin13_sec_mask: {=bool:?}, pio7_pin14_sec_mask: {=bool:?}, pio7_pin15_sec_mask: {=bool:?}, pio7_pin16_sec_mask: {=bool:?}, pio7_pin17_sec_mask: {=bool:?}, pio7_pin18_sec_mask: {=bool:?}, pio7_pin19_sec_mask: {=bool:?}, pio7_pin20_sec_mask: {=bool:?}, pio7_pin21_sec_mask: {=bool:?}, pio7_pin22_sec_mask: {=bool:?}, pio7_pin23_sec_mask: {=bool:?}, pio7_pin24_sec_mask: {=bool:?}, pio7_pin25_sec_mask: {=bool:?}, pio7_pin26_sec_mask: {=bool:?}, pio7_pin27_sec_mask: {=bool:?}, pio7_pin28_sec_mask: {=bool:?}, pio7_pin29_sec_mask: {=bool:?}, pio7_pin30_sec_mask: {=bool:?}, pio7_pin31_sec_mask: {=bool:?} }}",
            self.pio7_pin0_sec_mask(),
            self.pio7_pin1_sec_mask(),
            self.pio7_pin2_sec_mask(),
            self.pio7_pin3_sec_mask(),
            self.pio7_pin4_sec_mask(),
            self.pio7_pin5_sec_mask(),
            self.pio7_pin6_sec_mask(),
            self.pio7_pin7_sec_mask(),
            self.pio7_pin8_sec_mask(),
            self.pio7_pin9_sec_mask(),
            self.pio7_pin10_sec_mask(),
            self.pio7_pin11_sec_mask(),
            self.pio7_pin12_sec_mask(),
            self.pio7_pin13_sec_mask(),
            self.pio7_pin14_sec_mask(),
            self.pio7_pin15_sec_mask(),
            self.pio7_pin16_sec_mask(),
            self.pio7_pin17_sec_mask(),
            self.pio7_pin18_sec_mask(),
            self.pio7_pin19_sec_mask(),
            self.pio7_pin20_sec_mask(),
            self.pio7_pin21_sec_mask(),
            self.pio7_pin22_sec_mask(),
            self.pio7_pin23_sec_mask(),
            self.pio7_pin24_sec_mask(),
            self.pio7_pin25_sec_mask(),
            self.pio7_pin26_sec_mask(),
            self.pio7_pin27_sec_mask(),
            self.pio7_pin28_sec_mask(),
            self.pio7_pin29_sec_mask(),
            self.pio7_pin30_sec_mask(),
            self.pio7_pin31_sec_mask()
        )
    }
}
#[doc = "sec_gp_reg write-lock bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecMaskLock(pub u32);
impl SecMaskLock {
    #[doc = "SEC_GPIO_MASK0 register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_gpio_mask0_lock(&self) -> super::vals::SecGpioMask0Lock {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecGpioMask0Lock::from_bits(val as u8)
    }
    #[doc = "SEC_GPIO_MASK0 register write-lock."]
    #[inline(always)]
    pub const fn set_sec_gpio_mask0_lock(&mut self, val: super::vals::SecGpioMask0Lock) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SEC_GPIO_MASK1 register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_gpio_mask1_lock(&self) -> super::vals::SecGpioMask1Lock {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SecGpioMask1Lock::from_bits(val as u8)
    }
    #[doc = "SEC_GPIO_MASK1 register write-lock."]
    #[inline(always)]
    pub const fn set_sec_gpio_mask1_lock(&mut self, val: super::vals::SecGpioMask1Lock) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "SEC_GPIO_MASK2 register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_gpio_mask2_lock(&self) -> super::vals::SecGpioMask2Lock {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecGpioMask2Lock::from_bits(val as u8)
    }
    #[doc = "SEC_GPIO_MASK2 register write-lock."]
    #[inline(always)]
    pub const fn set_sec_gpio_mask2_lock(&mut self, val: super::vals::SecGpioMask2Lock) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SEC_GPIO_MASK3 register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_gpio_mask3_lock(&self) -> super::vals::SecGpioMask3Lock {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::SecGpioMask3Lock::from_bits(val as u8)
    }
    #[doc = "SEC_GPIO_MASK3 register write-lock."]
    #[inline(always)]
    pub const fn set_sec_gpio_mask3_lock(&mut self, val: super::vals::SecGpioMask3Lock) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "SEC_GPIO_MASK4 register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_gpio_mask4_lock(&self) -> super::vals::SecGpioMask4Lock {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecGpioMask4Lock::from_bits(val as u8)
    }
    #[doc = "SEC_GPIO_MASK4 register write-lock."]
    #[inline(always)]
    pub const fn set_sec_gpio_mask4_lock(&mut self, val: super::vals::SecGpioMask4Lock) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "SEC_GPIO_MASK5 register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_gpio_mask5_lock(&self) -> super::vals::SecGpioMask5Lock {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::SecGpioMask5Lock::from_bits(val as u8)
    }
    #[doc = "SEC_GPIO_MASK5 register write-lock."]
    #[inline(always)]
    pub const fn set_sec_gpio_mask5_lock(&mut self, val: super::vals::SecGpioMask5Lock) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "SEC_GPIO_MASK6 register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_gpio_mask6_lock(&self) -> super::vals::SecGpioMask6Lock {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecGpioMask6Lock::from_bits(val as u8)
    }
    #[doc = "SEC_GPIO_MASK6 register write-lock."]
    #[inline(always)]
    pub const fn set_sec_gpio_mask6_lock(&mut self, val: super::vals::SecGpioMask6Lock) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "SEC_GPIO_MASK7 register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_gpio_mask7_lock(&self) -> super::vals::SecGpioMask7Lock {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::SecGpioMask7Lock::from_bits(val as u8)
    }
    #[doc = "SEC_GPIO_MASK7 register write-lock."]
    #[inline(always)]
    pub const fn set_sec_gpio_mask7_lock(&mut self, val: super::vals::SecGpioMask7Lock) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "SEC_DSP_INT_MASK register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_dsp_int_lock(&self) -> super::vals::SecDspIntLock {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SecDspIntLock::from_bits(val as u8)
    }
    #[doc = "SEC_DSP_INT_MASK register write-lock."]
    #[inline(always)]
    pub const fn set_sec_dsp_int_lock(&mut self, val: super::vals::SecDspIntLock) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for SecMaskLock {
    #[inline(always)]
    fn default() -> SecMaskLock {
        SecMaskLock(0)
    }
}
impl core::fmt::Debug for SecMaskLock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecMaskLock")
            .field("sec_gpio_mask0_lock", &self.sec_gpio_mask0_lock())
            .field("sec_gpio_mask1_lock", &self.sec_gpio_mask1_lock())
            .field("sec_gpio_mask2_lock", &self.sec_gpio_mask2_lock())
            .field("sec_gpio_mask3_lock", &self.sec_gpio_mask3_lock())
            .field("sec_gpio_mask4_lock", &self.sec_gpio_mask4_lock())
            .field("sec_gpio_mask5_lock", &self.sec_gpio_mask5_lock())
            .field("sec_gpio_mask6_lock", &self.sec_gpio_mask6_lock())
            .field("sec_gpio_mask7_lock", &self.sec_gpio_mask7_lock())
            .field("sec_dsp_int_lock", &self.sec_dsp_int_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecMaskLock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecMaskLock {{ sec_gpio_mask0_lock: {:?}, sec_gpio_mask1_lock: {:?}, sec_gpio_mask2_lock: {:?}, sec_gpio_mask3_lock: {:?}, sec_gpio_mask4_lock: {:?}, sec_gpio_mask5_lock: {:?}, sec_gpio_mask6_lock: {:?}, sec_gpio_mask7_lock: {:?}, sec_dsp_int_lock: {:?} }}",
            self.sec_gpio_mask0_lock(),
            self.sec_gpio_mask1_lock(),
            self.sec_gpio_mask2_lock(),
            self.sec_gpio_mask3_lock(),
            self.sec_gpio_mask4_lock(),
            self.sec_gpio_mask5_lock(),
            self.sec_gpio_mask6_lock(),
            self.sec_gpio_mask7_lock(),
            self.sec_dsp_int_lock()
        )
    }
}
#[doc = "most recent security violation address for AHB layer n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioAddr(pub u32);
impl SecVioAddr {
    #[doc = "security violation address for AHB layer"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "security violation address for AHB layer"]
    #[inline(always)]
    pub const fn set_sec_vio_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SecVioAddr {
    #[inline(always)]
    fn default() -> SecVioAddr {
        SecVioAddr(0)
    }
}
impl core::fmt::Debug for SecVioAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioAddr")
            .field("sec_vio_addr", &self.sec_vio_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioAddr {{ sec_vio_addr: {=u32:?} }}",
            self.sec_vio_addr()
        )
    }
}
#[doc = "security violation address/information registers valid flags"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioInfoValid(pub u32);
impl SecVioInfoValid {
    #[doc = "violation information valid flag for AHB layer 0. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 0. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "violation information valid flag for AHB layer 1. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 1. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "violation information valid flag for AHB layer 2. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 2. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "violation information valid flag for AHB layer 3. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 3. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "violation information valid flag for AHB layer 4. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 4. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "violation information valid flag for AHB layer 5. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 5. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "violation information valid flag for AHB layer 6. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 6. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "violation information valid flag for AHB layer 7. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 7. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "violation information valid flag for AHB layer 8. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 8. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "violation information valid flag for AHB layer 9. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 9. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "violation information valid flag for AHB layer 10. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 10. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "violation information valid flag for AHB layer 11. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 11. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "violation information valid flag for AHB layer 12. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 12. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "violation information valid flag for AHB layer 13. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 13. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "violation information valid flag for AHB layer 14. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 14. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "violation information valid flag for AHB layer 15. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 15. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "violation information valid flag for AHB layer 16. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 16. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "violation information valid flag for AHB layer 17. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB layer 17. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for SecVioInfoValid {
    #[inline(always)]
    fn default() -> SecVioInfoValid {
        SecVioInfoValid(0)
    }
}
impl core::fmt::Debug for SecVioInfoValid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioInfoValid")
            .field("vio_info_valid0", &self.vio_info_valid0())
            .field("vio_info_valid1", &self.vio_info_valid1())
            .field("vio_info_valid2", &self.vio_info_valid2())
            .field("vio_info_valid3", &self.vio_info_valid3())
            .field("vio_info_valid4", &self.vio_info_valid4())
            .field("vio_info_valid5", &self.vio_info_valid5())
            .field("vio_info_valid6", &self.vio_info_valid6())
            .field("vio_info_valid7", &self.vio_info_valid7())
            .field("vio_info_valid8", &self.vio_info_valid8())
            .field("vio_info_valid9", &self.vio_info_valid9())
            .field("vio_info_valid10", &self.vio_info_valid10())
            .field("vio_info_valid11", &self.vio_info_valid11())
            .field("vio_info_valid12", &self.vio_info_valid12())
            .field("vio_info_valid13", &self.vio_info_valid13())
            .field("vio_info_valid14", &self.vio_info_valid14())
            .field("vio_info_valid15", &self.vio_info_valid15())
            .field("vio_info_valid16", &self.vio_info_valid16())
            .field("vio_info_valid17", &self.vio_info_valid17())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioInfoValid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioInfoValid {{ vio_info_valid0: {=bool:?}, vio_info_valid1: {=bool:?}, vio_info_valid2: {=bool:?}, vio_info_valid3: {=bool:?}, vio_info_valid4: {=bool:?}, vio_info_valid5: {=bool:?}, vio_info_valid6: {=bool:?}, vio_info_valid7: {=bool:?}, vio_info_valid8: {=bool:?}, vio_info_valid9: {=bool:?}, vio_info_valid10: {=bool:?}, vio_info_valid11: {=bool:?}, vio_info_valid12: {=bool:?}, vio_info_valid13: {=bool:?}, vio_info_valid14: {=bool:?}, vio_info_valid15: {=bool:?}, vio_info_valid16: {=bool:?}, vio_info_valid17: {=bool:?} }}",
            self.vio_info_valid0(),
            self.vio_info_valid1(),
            self.vio_info_valid2(),
            self.vio_info_valid3(),
            self.vio_info_valid4(),
            self.vio_info_valid5(),
            self.vio_info_valid6(),
            self.vio_info_valid7(),
            self.vio_info_valid8(),
            self.vio_info_valid9(),
            self.vio_info_valid10(),
            self.vio_info_valid11(),
            self.vio_info_valid12(),
            self.vio_info_valid13(),
            self.vio_info_valid14(),
            self.vio_info_valid15(),
            self.vio_info_valid16(),
            self.vio_info_valid17()
        )
    }
}
#[doc = "most recent security violation miscellaneous information for AHB layer n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioMiscInfo(pub u32);
impl SecVioMiscInfo {
    #[doc = "security violation access read/write indicator, 0: read, 1: write"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_write(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "security violation access read/write indicator, 0: read, 1: write"]
    #[inline(always)]
    pub const fn set_sec_vio_info_write(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "security violation access data/code indicator, 0: code, 1"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_data_access(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "security violation access data/code indicator, 0: code, 1"]
    #[inline(always)]
    pub const fn set_sec_vio_info_data_access(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "bit \\[5:4\\]: master sec level and privilege level bit \\[7:6\\]: anti-pol value for master sec level and privilege level"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_master_sec_level(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "bit \\[5:4\\]: master sec level and privilege level bit \\[7:6\\]: anti-pol value for master sec level and privilege level"]
    #[inline(always)]
    pub const fn set_sec_vio_info_master_sec_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "security violation master number"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_master(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "security violation master number"]
    #[inline(always)]
    pub const fn set_sec_vio_info_master(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for SecVioMiscInfo {
    #[inline(always)]
    fn default() -> SecVioMiscInfo {
        SecVioMiscInfo(0)
    }
}
impl core::fmt::Debug for SecVioMiscInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioMiscInfo")
            .field("sec_vio_info_write", &self.sec_vio_info_write())
            .field("sec_vio_info_data_access", &self.sec_vio_info_data_access())
            .field(
                "sec_vio_info_master_sec_level",
                &self.sec_vio_info_master_sec_level(),
            )
            .field("sec_vio_info_master", &self.sec_vio_info_master())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioMiscInfo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioMiscInfo {{ sec_vio_info_write: {=bool:?}, sec_vio_info_data_access: {=bool:?}, sec_vio_info_master_sec_level: {=u8:?}, sec_vio_info_master: {=u8:?} }}",
            self.sec_vio_info_write(),
            self.sec_vio_info_data_access(),
            self.sec_vio_info_master_sec_level(),
            self.sec_vio_info_master()
        )
    }
}
#[doc = "0x40148000--0x4014BFFF"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecurityCtrlMemRule0(pub u32);
impl SecurityCtrlMemRule0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for SecurityCtrlMemRule0 {
    #[inline(always)]
    fn default() -> SecurityCtrlMemRule0 {
        SecurityCtrlMemRule0(0)
    }
}
impl core::fmt::Debug for SecurityCtrlMemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecurityCtrlMemRule0")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecurityCtrlMemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecurityCtrlMemRule0 {{ rule0: {=u8:?}, rule1: {=u8:?}, rule2: {=u8:?}, rule3: {=u8:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3()
        )
    }
}
