#[doc = "Miscalleneous control signals for in Cortex M33 (CPU0)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPU0_LOCK_REG(pub u32);
impl CPU0_LOCK_REG {
    #[doc = "Cortex M33 (CPU0) VTOR_NS register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK_NS_VTOR(&self) -> super::vals::CPU0_LOCK_REG_LOCK_NS_VTOR {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CPU0_LOCK_REG_LOCK_NS_VTOR::from_bits(val as u8)
    }
    #[doc = "Cortex M33 (CPU0) VTOR_NS register write-lock."]
    #[inline(always)]
    pub const fn set_LOCK_NS_VTOR(&mut self, val: super::vals::CPU0_LOCK_REG_LOCK_NS_VTOR) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Cortex M33 (CPU0) non-secure MPU register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK_NS_MPU(&self) -> super::vals::CPU0_LOCK_REG_LOCK_NS_MPU {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::CPU0_LOCK_REG_LOCK_NS_MPU::from_bits(val as u8)
    }
    #[doc = "Cortex M33 (CPU0) non-secure MPU register write-lock."]
    #[inline(always)]
    pub const fn set_LOCK_NS_MPU(&mut self, val: super::vals::CPU0_LOCK_REG_LOCK_NS_MPU) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Cortex M33 (CPU0) VTOR_S, AIRCR.PRIS, IRCR.BFHFNMINS registers write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK_S_VTAIRCR(&self) -> super::vals::LOCK_S_VTAIRCR {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::LOCK_S_VTAIRCR::from_bits(val as u8)
    }
    #[doc = "Cortex M33 (CPU0) VTOR_S, AIRCR.PRIS, IRCR.BFHFNMINS registers write-lock."]
    #[inline(always)]
    pub const fn set_LOCK_S_VTAIRCR(&mut self, val: super::vals::LOCK_S_VTAIRCR) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Cortex M33 (CPU0) Secure MPU registers write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK_S_MPU(&self) -> super::vals::LOCK_S_MPU {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::LOCK_S_MPU::from_bits(val as u8)
    }
    #[doc = "Cortex M33 (CPU0) Secure MPU registers write-lock."]
    #[inline(always)]
    pub const fn set_LOCK_S_MPU(&mut self, val: super::vals::LOCK_S_MPU) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Cortex M33 (CPU0) SAU registers write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK_SAU(&self) -> super::vals::LOCK_SAU {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::LOCK_SAU::from_bits(val as u8)
    }
    #[doc = "Cortex M33 (CPU0) SAU registers write-lock."]
    #[inline(always)]
    pub const fn set_LOCK_SAU(&mut self, val: super::vals::LOCK_SAU) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CPU0_LOCK_REG write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU0_LOCK_REG_LOCK(&self) -> super::vals::CPU0_LOCK_REG_LOCK {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::CPU0_LOCK_REG_LOCK::from_bits(val as u8)
    }
    #[doc = "CPU0_LOCK_REG write-lock."]
    #[inline(always)]
    pub const fn set_CPU0_LOCK_REG_LOCK(&mut self, val: super::vals::CPU0_LOCK_REG_LOCK) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for CPU0_LOCK_REG {
    #[inline(always)]
    fn default() -> CPU0_LOCK_REG {
        CPU0_LOCK_REG(0)
    }
}
impl core::fmt::Debug for CPU0_LOCK_REG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU0_LOCK_REG")
            .field("LOCK_NS_VTOR", &self.LOCK_NS_VTOR())
            .field("LOCK_NS_MPU", &self.LOCK_NS_MPU())
            .field("LOCK_S_VTAIRCR", &self.LOCK_S_VTAIRCR())
            .field("LOCK_S_MPU", &self.LOCK_S_MPU())
            .field("LOCK_SAU", &self.LOCK_SAU())
            .field("CPU0_LOCK_REG_LOCK", &self.CPU0_LOCK_REG_LOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPU0_LOCK_REG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPU0_LOCK_REG {{ LOCK_NS_VTOR: {:?}, LOCK_NS_MPU: {:?}, LOCK_S_VTAIRCR: {:?}, LOCK_S_MPU: {:?}, LOCK_SAU: {:?}, CPU0_LOCK_REG_LOCK: {:?} }}",
            self.LOCK_NS_VTOR(),
            self.LOCK_NS_MPU(),
            self.LOCK_S_VTAIRCR(),
            self.LOCK_S_MPU(),
            self.LOCK_SAU(),
            self.CPU0_LOCK_REG_LOCK()
        )
    }
}
#[doc = "Miscalleneous control signals for in micro-Cortex M33 (CPU1)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPU1_LOCK_REG(pub u32);
impl CPU1_LOCK_REG {
    #[doc = "micro-Cortex M33 (CPU1) VTOR_NS register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK_NS_VTOR(&self) -> super::vals::CPU1_LOCK_REG_LOCK_NS_VTOR {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CPU1_LOCK_REG_LOCK_NS_VTOR::from_bits(val as u8)
    }
    #[doc = "micro-Cortex M33 (CPU1) VTOR_NS register write-lock."]
    #[inline(always)]
    pub const fn set_LOCK_NS_VTOR(&mut self, val: super::vals::CPU1_LOCK_REG_LOCK_NS_VTOR) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "micro-Cortex M33 (CPU1) non-secure MPU register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK_NS_MPU(&self) -> super::vals::CPU1_LOCK_REG_LOCK_NS_MPU {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::CPU1_LOCK_REG_LOCK_NS_MPU::from_bits(val as u8)
    }
    #[doc = "micro-Cortex M33 (CPU1) non-secure MPU register write-lock."]
    #[inline(always)]
    pub const fn set_LOCK_NS_MPU(&mut self, val: super::vals::CPU1_LOCK_REG_LOCK_NS_MPU) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CPU1_LOCK_REG write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU1_LOCK_REG_LOCK(&self) -> super::vals::CPU1_LOCK_REG_LOCK {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::CPU1_LOCK_REG_LOCK::from_bits(val as u8)
    }
    #[doc = "CPU1_LOCK_REG write-lock."]
    #[inline(always)]
    pub const fn set_CPU1_LOCK_REG_LOCK(&mut self, val: super::vals::CPU1_LOCK_REG_LOCK) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for CPU1_LOCK_REG {
    #[inline(always)]
    fn default() -> CPU1_LOCK_REG {
        CPU1_LOCK_REG(0)
    }
}
impl core::fmt::Debug for CPU1_LOCK_REG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU1_LOCK_REG")
            .field("LOCK_NS_VTOR", &self.LOCK_NS_VTOR())
            .field("LOCK_NS_MPU", &self.LOCK_NS_MPU())
            .field("CPU1_LOCK_REG_LOCK", &self.CPU1_LOCK_REG_LOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPU1_LOCK_REG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPU1_LOCK_REG {{ LOCK_NS_VTOR: {:?}, LOCK_NS_MPU: {:?}, CPU1_LOCK_REG_LOCK: {:?} }}",
            self.LOCK_NS_VTOR(),
            self.LOCK_NS_MPU(),
            self.CPU1_LOCK_REG_LOCK()
        )
    }
}
#[doc = "master secure level anti-pole register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MASTER_SEC_ANTI_POL_REG(pub u32);
impl MASTER_SEC_ANTI_POL_REG {
    #[doc = "Micro-Cortex M33 (CPU1) Code bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1C)."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU1C(&self) -> super::vals::MASTER_SEC_ANTI_POL_REG_CPU1C {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MASTER_SEC_ANTI_POL_REG_CPU1C::from_bits(val as u8)
    }
    #[doc = "Micro-Cortex M33 (CPU1) Code bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1C)."]
    #[inline(always)]
    pub const fn set_CPU1C(&mut self, val: super::vals::MASTER_SEC_ANTI_POL_REG_CPU1C) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Micro-Cortex M33 (CPU1) System bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1S)."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU1S(&self) -> super::vals::MASTER_SEC_ANTI_POL_REG_CPU1S {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MASTER_SEC_ANTI_POL_REG_CPU1S::from_bits(val as u8)
    }
    #[doc = "Micro-Cortex M33 (CPU1) System bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1S)."]
    #[inline(always)]
    pub const fn set_CPU1S(&mut self, val: super::vals::MASTER_SEC_ANTI_POL_REG_CPU1S) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "USB Full Speed Device. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSD)."]
    #[must_use]
    #[inline(always)]
    pub const fn USBFSD(&self) -> super::vals::MASTER_SEC_ANTI_POL_REG_USBFSD {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MASTER_SEC_ANTI_POL_REG_USBFSD::from_bits(val as u8)
    }
    #[doc = "USB Full Speed Device. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSD)."]
    #[inline(always)]
    pub const fn set_USBFSD(&mut self, val: super::vals::MASTER_SEC_ANTI_POL_REG_USBFSD) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)."]
    #[must_use]
    #[inline(always)]
    pub const fn SDMA0(&self) -> super::vals::MASTER_SEC_ANTI_POL_REG_SDMA0 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MASTER_SEC_ANTI_POL_REG_SDMA0::from_bits(val as u8)
    }
    #[doc = "System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)."]
    #[inline(always)]
    pub const fn set_SDMA0(&mut self, val: super::vals::MASTER_SEC_ANTI_POL_REG_SDMA0) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "SDIO. Must be equal to NOT(MASTER_SEC_LEVEL.SDIO)."]
    #[must_use]
    #[inline(always)]
    pub const fn SDIO(&self) -> super::vals::MASTER_SEC_ANTI_POL_REG_SDIO {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::MASTER_SEC_ANTI_POL_REG_SDIO::from_bits(val as u8)
    }
    #[doc = "SDIO. Must be equal to NOT(MASTER_SEC_LEVEL.SDIO)."]
    #[inline(always)]
    pub const fn set_SDIO(&mut self, val: super::vals::MASTER_SEC_ANTI_POL_REG_SDIO) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Power Quad. Must be equal to NOT(MASTER_SEC_LEVEL.PQ)."]
    #[must_use]
    #[inline(always)]
    pub const fn PQ(&self) -> super::vals::MASTER_SEC_ANTI_POL_REG_PQ {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::MASTER_SEC_ANTI_POL_REG_PQ::from_bits(val as u8)
    }
    #[doc = "Power Quad. Must be equal to NOT(MASTER_SEC_LEVEL.PQ)."]
    #[inline(always)]
    pub const fn set_PQ(&mut self, val: super::vals::MASTER_SEC_ANTI_POL_REG_PQ) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)."]
    #[must_use]
    #[inline(always)]
    pub const fn HASH(&self) -> super::vals::MASTER_SEC_ANTI_POL_REG_HASH {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::MASTER_SEC_ANTI_POL_REG_HASH::from_bits(val as u8)
    }
    #[doc = "Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)."]
    #[inline(always)]
    pub const fn set_HASH(&mut self, val: super::vals::MASTER_SEC_ANTI_POL_REG_HASH) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "USB Full speed Host. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSH)."]
    #[must_use]
    #[inline(always)]
    pub const fn USBFSH(&self) -> super::vals::MASTER_SEC_ANTI_POL_REG_USBFSH {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::MASTER_SEC_ANTI_POL_REG_USBFSH::from_bits(val as u8)
    }
    #[doc = "USB Full speed Host. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSH)."]
    #[inline(always)]
    pub const fn set_USBFSH(&mut self, val: super::vals::MASTER_SEC_ANTI_POL_REG_USBFSH) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)."]
    #[must_use]
    #[inline(always)]
    pub const fn SDMA1(&self) -> super::vals::MASTER_SEC_ANTI_POL_REG_SDMA1 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::MASTER_SEC_ANTI_POL_REG_SDMA1::from_bits(val as u8)
    }
    #[doc = "System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)."]
    #[inline(always)]
    pub const fn set_SDMA1(&mut self, val: super::vals::MASTER_SEC_ANTI_POL_REG_SDMA1) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "MASTER_SEC_ANTI_POL_REG register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn MASTER_SEC_LEVEL_ANTIPOL_LOCK(
        &self,
    ) -> super::vals::MASTER_SEC_LEVEL_ANTIPOL_LOCK {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::MASTER_SEC_LEVEL_ANTIPOL_LOCK::from_bits(val as u8)
    }
    #[doc = "MASTER_SEC_ANTI_POL_REG register write-lock."]
    #[inline(always)]
    pub const fn set_MASTER_SEC_LEVEL_ANTIPOL_LOCK(
        &mut self,
        val: super::vals::MASTER_SEC_LEVEL_ANTIPOL_LOCK,
    ) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for MASTER_SEC_ANTI_POL_REG {
    #[inline(always)]
    fn default() -> MASTER_SEC_ANTI_POL_REG {
        MASTER_SEC_ANTI_POL_REG(0)
    }
}
impl core::fmt::Debug for MASTER_SEC_ANTI_POL_REG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASTER_SEC_ANTI_POL_REG")
            .field("CPU1C", &self.CPU1C())
            .field("CPU1S", &self.CPU1S())
            .field("USBFSD", &self.USBFSD())
            .field("SDMA0", &self.SDMA0())
            .field("SDIO", &self.SDIO())
            .field("PQ", &self.PQ())
            .field("HASH", &self.HASH())
            .field("USBFSH", &self.USBFSH())
            .field("SDMA1", &self.SDMA1())
            .field(
                "MASTER_SEC_LEVEL_ANTIPOL_LOCK",
                &self.MASTER_SEC_LEVEL_ANTIPOL_LOCK(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MASTER_SEC_ANTI_POL_REG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MASTER_SEC_ANTI_POL_REG {{ CPU1C: {:?}, CPU1S: {:?}, USBFSD: {:?}, SDMA0: {:?}, SDIO: {:?}, PQ: {:?}, HASH: {:?}, USBFSH: {:?}, SDMA1: {:?}, MASTER_SEC_LEVEL_ANTIPOL_LOCK: {:?} }}",
            self.CPU1C(),
            self.CPU1S(),
            self.USBFSD(),
            self.SDMA0(),
            self.SDIO(),
            self.PQ(),
            self.HASH(),
            self.USBFSH(),
            self.SDMA1(),
            self.MASTER_SEC_LEVEL_ANTIPOL_LOCK()
        )
    }
}
#[doc = "master secure level register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MASTER_SEC_LEVEL(pub u32);
impl MASTER_SEC_LEVEL {
    #[doc = "Micro-Cortex M33 (CPU1) Code bus."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU1C(&self) -> super::vals::MASTER_SEC_LEVEL_CPU1C {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MASTER_SEC_LEVEL_CPU1C::from_bits(val as u8)
    }
    #[doc = "Micro-Cortex M33 (CPU1) Code bus."]
    #[inline(always)]
    pub const fn set_CPU1C(&mut self, val: super::vals::MASTER_SEC_LEVEL_CPU1C) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Micro-Cortex M33 (CPU1) System bus."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU1S(&self) -> super::vals::MASTER_SEC_LEVEL_CPU1S {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MASTER_SEC_LEVEL_CPU1S::from_bits(val as u8)
    }
    #[doc = "Micro-Cortex M33 (CPU1) System bus."]
    #[inline(always)]
    pub const fn set_CPU1S(&mut self, val: super::vals::MASTER_SEC_LEVEL_CPU1S) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "USB Full Speed Device."]
    #[must_use]
    #[inline(always)]
    pub const fn USBFSD(&self) -> super::vals::MASTER_SEC_LEVEL_USBFSD {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MASTER_SEC_LEVEL_USBFSD::from_bits(val as u8)
    }
    #[doc = "USB Full Speed Device."]
    #[inline(always)]
    pub const fn set_USBFSD(&mut self, val: super::vals::MASTER_SEC_LEVEL_USBFSD) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "System DMA 0."]
    #[must_use]
    #[inline(always)]
    pub const fn SDMA0(&self) -> super::vals::MASTER_SEC_LEVEL_SDMA0 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MASTER_SEC_LEVEL_SDMA0::from_bits(val as u8)
    }
    #[doc = "System DMA 0."]
    #[inline(always)]
    pub const fn set_SDMA0(&mut self, val: super::vals::MASTER_SEC_LEVEL_SDMA0) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "SDIO."]
    #[must_use]
    #[inline(always)]
    pub const fn SDIO(&self) -> super::vals::MASTER_SEC_LEVEL_SDIO {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::MASTER_SEC_LEVEL_SDIO::from_bits(val as u8)
    }
    #[doc = "SDIO."]
    #[inline(always)]
    pub const fn set_SDIO(&mut self, val: super::vals::MASTER_SEC_LEVEL_SDIO) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Power Quad."]
    #[must_use]
    #[inline(always)]
    pub const fn PQ(&self) -> super::vals::MASTER_SEC_LEVEL_PQ {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::MASTER_SEC_LEVEL_PQ::from_bits(val as u8)
    }
    #[doc = "Power Quad."]
    #[inline(always)]
    pub const fn set_PQ(&mut self, val: super::vals::MASTER_SEC_LEVEL_PQ) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Hash."]
    #[must_use]
    #[inline(always)]
    pub const fn HASH(&self) -> super::vals::MASTER_SEC_LEVEL_HASH {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::MASTER_SEC_LEVEL_HASH::from_bits(val as u8)
    }
    #[doc = "Hash."]
    #[inline(always)]
    pub const fn set_HASH(&mut self, val: super::vals::MASTER_SEC_LEVEL_HASH) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "USB Full speed Host."]
    #[must_use]
    #[inline(always)]
    pub const fn USBFSH(&self) -> super::vals::MASTER_SEC_LEVEL_USBFSH {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::MASTER_SEC_LEVEL_USBFSH::from_bits(val as u8)
    }
    #[doc = "USB Full speed Host."]
    #[inline(always)]
    pub const fn set_USBFSH(&mut self, val: super::vals::MASTER_SEC_LEVEL_USBFSH) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "System DMA 1 security level."]
    #[must_use]
    #[inline(always)]
    pub const fn SDMA1(&self) -> super::vals::MASTER_SEC_LEVEL_SDMA1 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::MASTER_SEC_LEVEL_SDMA1::from_bits(val as u8)
    }
    #[doc = "System DMA 1 security level."]
    #[inline(always)]
    pub const fn set_SDMA1(&mut self, val: super::vals::MASTER_SEC_LEVEL_SDMA1) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "MASTER_SEC_LEVEL write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn MASTER_SEC_LEVEL_LOCK(&self) -> super::vals::MASTER_SEC_LEVEL_LOCK {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::MASTER_SEC_LEVEL_LOCK::from_bits(val as u8)
    }
    #[doc = "MASTER_SEC_LEVEL write-lock."]
    #[inline(always)]
    pub const fn set_MASTER_SEC_LEVEL_LOCK(&mut self, val: super::vals::MASTER_SEC_LEVEL_LOCK) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for MASTER_SEC_LEVEL {
    #[inline(always)]
    fn default() -> MASTER_SEC_LEVEL {
        MASTER_SEC_LEVEL(0)
    }
}
impl core::fmt::Debug for MASTER_SEC_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASTER_SEC_LEVEL")
            .field("CPU1C", &self.CPU1C())
            .field("CPU1S", &self.CPU1S())
            .field("USBFSD", &self.USBFSD())
            .field("SDMA0", &self.SDMA0())
            .field("SDIO", &self.SDIO())
            .field("PQ", &self.PQ())
            .field("HASH", &self.HASH())
            .field("USBFSH", &self.USBFSH())
            .field("SDMA1", &self.SDMA1())
            .field("MASTER_SEC_LEVEL_LOCK", &self.MASTER_SEC_LEVEL_LOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MASTER_SEC_LEVEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MASTER_SEC_LEVEL {{ CPU1C: {:?}, CPU1S: {:?}, USBFSD: {:?}, SDMA0: {:?}, SDIO: {:?}, PQ: {:?}, HASH: {:?}, USBFSH: {:?}, SDMA1: {:?}, MASTER_SEC_LEVEL_LOCK: {:?} }}",
            self.CPU1C(),
            self.CPU1S(),
            self.USBFSD(),
            self.SDMA0(),
            self.SDIO(),
            self.PQ(),
            self.HASH(),
            self.USBFSH(),
            self.SDMA1(),
            self.MASTER_SEC_LEVEL_LOCK()
        )
    }
}
#[doc = "secure control duplicate register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MISC_CTRL_DP_REG(pub u32);
impl MISC_CTRL_DP_REG {
    #[doc = "Write lock."]
    #[must_use]
    #[inline(always)]
    pub const fn WRITE_LOCK(&self) -> super::vals::MISC_CTRL_DP_REG_WRITE_LOCK {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MISC_CTRL_DP_REG_WRITE_LOCK::from_bits(val as u8)
    }
    #[doc = "Write lock."]
    #[inline(always)]
    pub const fn set_WRITE_LOCK(&mut self, val: super::vals::MISC_CTRL_DP_REG_WRITE_LOCK) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Enable secure check for AHB matrix."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE_SECURE_CHECKING(
        &self,
    ) -> super::vals::MISC_CTRL_DP_REG_ENABLE_SECURE_CHECKING {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::MISC_CTRL_DP_REG_ENABLE_SECURE_CHECKING::from_bits(val as u8)
    }
    #[doc = "Enable secure check for AHB matrix."]
    #[inline(always)]
    pub const fn set_ENABLE_SECURE_CHECKING(
        &mut self,
        val: super::vals::MISC_CTRL_DP_REG_ENABLE_SECURE_CHECKING,
    ) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable secure privilege check for AHB matrix."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE_S_PRIV_CHECK(&self) -> super::vals::MISC_CTRL_DP_REG_ENABLE_S_PRIV_CHECK {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MISC_CTRL_DP_REG_ENABLE_S_PRIV_CHECK::from_bits(val as u8)
    }
    #[doc = "Enable secure privilege check for AHB matrix."]
    #[inline(always)]
    pub const fn set_ENABLE_S_PRIV_CHECK(
        &mut self,
        val: super::vals::MISC_CTRL_DP_REG_ENABLE_S_PRIV_CHECK,
    ) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Enable non-secure privilege check for AHB matrix."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE_NS_PRIV_CHECK(&self) -> super::vals::MISC_CTRL_DP_REG_ENABLE_NS_PRIV_CHECK {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MISC_CTRL_DP_REG_ENABLE_NS_PRIV_CHECK::from_bits(val as u8)
    }
    #[doc = "Enable non-secure privilege check for AHB matrix."]
    #[inline(always)]
    pub const fn set_ENABLE_NS_PRIV_CHECK(
        &mut self,
        val: super::vals::MISC_CTRL_DP_REG_ENABLE_NS_PRIV_CHECK,
    ) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Disable secure violation abort."]
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE_VIOLATION_ABORT(
        &self,
    ) -> super::vals::MISC_CTRL_DP_REG_DISABLE_VIOLATION_ABORT {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MISC_CTRL_DP_REG_DISABLE_VIOLATION_ABORT::from_bits(val as u8)
    }
    #[doc = "Disable secure violation abort."]
    #[inline(always)]
    pub const fn set_DISABLE_VIOLATION_ABORT(
        &mut self,
        val: super::vals::MISC_CTRL_DP_REG_DISABLE_VIOLATION_ABORT,
    ) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Disable simple master strict mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE_SIMPLE_MASTER_STRICT_MODE(
        &self,
    ) -> super::vals::MISC_CTRL_DP_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MISC_CTRL_DP_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE::from_bits(val as u8)
    }
    #[doc = "Disable simple master strict mode."]
    #[inline(always)]
    pub const fn set_DISABLE_SIMPLE_MASTER_STRICT_MODE(
        &mut self,
        val: super::vals::MISC_CTRL_DP_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE,
    ) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Disable smart master strict mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE_SMART_MASTER_STRICT_MODE(
        &self,
    ) -> super::vals::MISC_CTRL_DP_REG_DISABLE_SMART_MASTER_STRICT_MODE {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::MISC_CTRL_DP_REG_DISABLE_SMART_MASTER_STRICT_MODE::from_bits(val as u8)
    }
    #[doc = "Disable smart master strict mode."]
    #[inline(always)]
    pub const fn set_DISABLE_SMART_MASTER_STRICT_MODE(
        &mut self,
        val: super::vals::MISC_CTRL_DP_REG_DISABLE_SMART_MASTER_STRICT_MODE,
    ) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Disable IDAU."]
    #[must_use]
    #[inline(always)]
    pub const fn IDAU_ALL_NS(&self) -> super::vals::MISC_CTRL_DP_REG_IDAU_ALL_NS {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::MISC_CTRL_DP_REG_IDAU_ALL_NS::from_bits(val as u8)
    }
    #[doc = "Disable IDAU."]
    #[inline(always)]
    pub const fn set_IDAU_ALL_NS(&mut self, val: super::vals::MISC_CTRL_DP_REG_IDAU_ALL_NS) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for MISC_CTRL_DP_REG {
    #[inline(always)]
    fn default() -> MISC_CTRL_DP_REG {
        MISC_CTRL_DP_REG(0)
    }
}
impl core::fmt::Debug for MISC_CTRL_DP_REG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC_CTRL_DP_REG")
            .field("WRITE_LOCK", &self.WRITE_LOCK())
            .field("ENABLE_SECURE_CHECKING", &self.ENABLE_SECURE_CHECKING())
            .field("ENABLE_S_PRIV_CHECK", &self.ENABLE_S_PRIV_CHECK())
            .field("ENABLE_NS_PRIV_CHECK", &self.ENABLE_NS_PRIV_CHECK())
            .field("DISABLE_VIOLATION_ABORT", &self.DISABLE_VIOLATION_ABORT())
            .field(
                "DISABLE_SIMPLE_MASTER_STRICT_MODE",
                &self.DISABLE_SIMPLE_MASTER_STRICT_MODE(),
            )
            .field(
                "DISABLE_SMART_MASTER_STRICT_MODE",
                &self.DISABLE_SMART_MASTER_STRICT_MODE(),
            )
            .field("IDAU_ALL_NS", &self.IDAU_ALL_NS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MISC_CTRL_DP_REG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MISC_CTRL_DP_REG {{ WRITE_LOCK: {:?}, ENABLE_SECURE_CHECKING: {:?}, ENABLE_S_PRIV_CHECK: {:?}, ENABLE_NS_PRIV_CHECK: {:?}, DISABLE_VIOLATION_ABORT: {:?}, DISABLE_SIMPLE_MASTER_STRICT_MODE: {:?}, DISABLE_SMART_MASTER_STRICT_MODE: {:?}, IDAU_ALL_NS: {:?} }}",
            self.WRITE_LOCK(),
            self.ENABLE_SECURE_CHECKING(),
            self.ENABLE_S_PRIV_CHECK(),
            self.ENABLE_NS_PRIV_CHECK(),
            self.DISABLE_VIOLATION_ABORT(),
            self.DISABLE_SIMPLE_MASTER_STRICT_MODE(),
            self.DISABLE_SMART_MASTER_STRICT_MODE(),
            self.IDAU_ALL_NS()
        )
    }
}
#[doc = "secure control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MISC_CTRL_REG(pub u32);
impl MISC_CTRL_REG {
    #[doc = "Write lock."]
    #[must_use]
    #[inline(always)]
    pub const fn WRITE_LOCK(&self) -> super::vals::MISC_CTRL_REG_WRITE_LOCK {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MISC_CTRL_REG_WRITE_LOCK::from_bits(val as u8)
    }
    #[doc = "Write lock."]
    #[inline(always)]
    pub const fn set_WRITE_LOCK(&mut self, val: super::vals::MISC_CTRL_REG_WRITE_LOCK) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Enable secure check for AHB matrix."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE_SECURE_CHECKING(
        &self,
    ) -> super::vals::MISC_CTRL_REG_ENABLE_SECURE_CHECKING {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::MISC_CTRL_REG_ENABLE_SECURE_CHECKING::from_bits(val as u8)
    }
    #[doc = "Enable secure check for AHB matrix."]
    #[inline(always)]
    pub const fn set_ENABLE_SECURE_CHECKING(
        &mut self,
        val: super::vals::MISC_CTRL_REG_ENABLE_SECURE_CHECKING,
    ) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable secure privilege check for AHB matrix."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE_S_PRIV_CHECK(&self) -> super::vals::MISC_CTRL_REG_ENABLE_S_PRIV_CHECK {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MISC_CTRL_REG_ENABLE_S_PRIV_CHECK::from_bits(val as u8)
    }
    #[doc = "Enable secure privilege check for AHB matrix."]
    #[inline(always)]
    pub const fn set_ENABLE_S_PRIV_CHECK(
        &mut self,
        val: super::vals::MISC_CTRL_REG_ENABLE_S_PRIV_CHECK,
    ) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Enable non-secure privilege check for AHB matrix."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE_NS_PRIV_CHECK(&self) -> super::vals::MISC_CTRL_REG_ENABLE_NS_PRIV_CHECK {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MISC_CTRL_REG_ENABLE_NS_PRIV_CHECK::from_bits(val as u8)
    }
    #[doc = "Enable non-secure privilege check for AHB matrix."]
    #[inline(always)]
    pub const fn set_ENABLE_NS_PRIV_CHECK(
        &mut self,
        val: super::vals::MISC_CTRL_REG_ENABLE_NS_PRIV_CHECK,
    ) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Disable secure violation abort."]
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE_VIOLATION_ABORT(
        &self,
    ) -> super::vals::MISC_CTRL_REG_DISABLE_VIOLATION_ABORT {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MISC_CTRL_REG_DISABLE_VIOLATION_ABORT::from_bits(val as u8)
    }
    #[doc = "Disable secure violation abort."]
    #[inline(always)]
    pub const fn set_DISABLE_VIOLATION_ABORT(
        &mut self,
        val: super::vals::MISC_CTRL_REG_DISABLE_VIOLATION_ABORT,
    ) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Disable simple master strict mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE_SIMPLE_MASTER_STRICT_MODE(
        &self,
    ) -> super::vals::MISC_CTRL_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MISC_CTRL_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE::from_bits(val as u8)
    }
    #[doc = "Disable simple master strict mode."]
    #[inline(always)]
    pub const fn set_DISABLE_SIMPLE_MASTER_STRICT_MODE(
        &mut self,
        val: super::vals::MISC_CTRL_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE,
    ) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Disable smart master strict mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE_SMART_MASTER_STRICT_MODE(
        &self,
    ) -> super::vals::MISC_CTRL_REG_DISABLE_SMART_MASTER_STRICT_MODE {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::MISC_CTRL_REG_DISABLE_SMART_MASTER_STRICT_MODE::from_bits(val as u8)
    }
    #[doc = "Disable smart master strict mode."]
    #[inline(always)]
    pub const fn set_DISABLE_SMART_MASTER_STRICT_MODE(
        &mut self,
        val: super::vals::MISC_CTRL_REG_DISABLE_SMART_MASTER_STRICT_MODE,
    ) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Disable IDAU."]
    #[must_use]
    #[inline(always)]
    pub const fn IDAU_ALL_NS(&self) -> super::vals::MISC_CTRL_REG_IDAU_ALL_NS {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::MISC_CTRL_REG_IDAU_ALL_NS::from_bits(val as u8)
    }
    #[doc = "Disable IDAU."]
    #[inline(always)]
    pub const fn set_IDAU_ALL_NS(&mut self, val: super::vals::MISC_CTRL_REG_IDAU_ALL_NS) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for MISC_CTRL_REG {
    #[inline(always)]
    fn default() -> MISC_CTRL_REG {
        MISC_CTRL_REG(0)
    }
}
impl core::fmt::Debug for MISC_CTRL_REG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC_CTRL_REG")
            .field("WRITE_LOCK", &self.WRITE_LOCK())
            .field("ENABLE_SECURE_CHECKING", &self.ENABLE_SECURE_CHECKING())
            .field("ENABLE_S_PRIV_CHECK", &self.ENABLE_S_PRIV_CHECK())
            .field("ENABLE_NS_PRIV_CHECK", &self.ENABLE_NS_PRIV_CHECK())
            .field("DISABLE_VIOLATION_ABORT", &self.DISABLE_VIOLATION_ABORT())
            .field(
                "DISABLE_SIMPLE_MASTER_STRICT_MODE",
                &self.DISABLE_SIMPLE_MASTER_STRICT_MODE(),
            )
            .field(
                "DISABLE_SMART_MASTER_STRICT_MODE",
                &self.DISABLE_SMART_MASTER_STRICT_MODE(),
            )
            .field("IDAU_ALL_NS", &self.IDAU_ALL_NS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MISC_CTRL_REG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MISC_CTRL_REG {{ WRITE_LOCK: {:?}, ENABLE_SECURE_CHECKING: {:?}, ENABLE_S_PRIV_CHECK: {:?}, ENABLE_NS_PRIV_CHECK: {:?}, DISABLE_VIOLATION_ABORT: {:?}, DISABLE_SIMPLE_MASTER_STRICT_MODE: {:?}, DISABLE_SMART_MASTER_STRICT_MODE: {:?}, IDAU_ALL_NS: {:?} }}",
            self.WRITE_LOCK(),
            self.ENABLE_SECURE_CHECKING(),
            self.ENABLE_S_PRIV_CHECK(),
            self.ENABLE_NS_PRIV_CHECK(),
            self.DISABLE_VIOLATION_ABORT(),
            self.DISABLE_SIMPLE_MASTER_STRICT_MODE(),
            self.DISABLE_SMART_MASTER_STRICT_MODE(),
            self.IDAU_ALL_NS()
        )
    }
}
#[doc = "Secure Interrupt mask for CPU1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CPU_INT_MASK0(pub u32);
impl SEC_CPU_INT_MASK0 {
    #[doc = "Watchdog Timer, Brown Out Detectors and Flash Controller interrupts."]
    #[must_use]
    #[inline(always)]
    pub const fn SYS_IRQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Timer, Brown Out Detectors and Flash Controller interrupts."]
    #[inline(always)]
    pub const fn set_SYS_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "System DMA 0 (non-secure) interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn SDMA0_IRQ(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "System DMA 0 (non-secure) interrupt."]
    #[inline(always)]
    pub const fn set_SDMA0_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GPIO Group 0 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_GLOBALINT0_IRQ(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO Group 0 interrupt."]
    #[inline(always)]
    pub const fn set_GPIO_GLOBALINT0_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GPIO Group 1 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_GLOBALINT1_IRQ(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO Group 1 interrupt."]
    #[inline(always)]
    pub const fn set_GPIO_GLOBALINT1_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_INT0_IRQ0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline(always)]
    pub const fn set_GPIO_INT0_IRQ0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_INT0_IRQ1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline(always)]
    pub const fn set_GPIO_INT0_IRQ1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Pin interrupt 2 or pattern match engine slice 2 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_INT0_IRQ2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Pin interrupt 2 or pattern match engine slice 2 interrupt."]
    #[inline(always)]
    pub const fn set_GPIO_INT0_IRQ2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin interrupt 3 or pattern match engine slice 3 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_INT0_IRQ3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Pin interrupt 3 or pattern match engine slice 3 interrupt."]
    #[inline(always)]
    pub const fn set_GPIO_INT0_IRQ3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Micro Tick Timer interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn UTICK_IRQ(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Micro Tick Timer interrupt."]
    #[inline(always)]
    pub const fn set_UTICK_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Multi-Rate Timer interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn MRT_IRQ(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Rate Timer interrupt."]
    #[inline(always)]
    pub const fn set_MRT_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Standard counter/timer 0 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn CTIMER0_IRQ(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Standard counter/timer 0 interrupt."]
    #[inline(always)]
    pub const fn set_CTIMER0_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Standard counter/timer 1 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn CTIMER1_IRQ(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Standard counter/timer 1 interrupt."]
    #[inline(always)]
    pub const fn set_CTIMER1_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SCTimer/PWM interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn SCT_IRQ(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SCTimer/PWM interrupt."]
    #[inline(always)]
    pub const fn set_SCT_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Standard counter/timer 3 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn CTIMER3_IRQ(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Standard counter/timer 3 interrupt."]
    #[inline(always)]
    pub const fn set_CTIMER3_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
    #[must_use]
    #[inline(always)]
    pub const fn FLEXCOMM0_IRQ(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub const fn set_FLEXCOMM0_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
    #[must_use]
    #[inline(always)]
    pub const fn FLEXCOMM1_IRQ(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub const fn set_FLEXCOMM1_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
    #[must_use]
    #[inline(always)]
    pub const fn FLEXCOMM2_IRQ(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub const fn set_FLEXCOMM2_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
    #[must_use]
    #[inline(always)]
    pub const fn FLEXCOMM3_IRQ(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub const fn set_FLEXCOMM3_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
    #[must_use]
    #[inline(always)]
    pub const fn FLEXCOMM4_IRQ(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub const fn set_FLEXCOMM4_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
    #[must_use]
    #[inline(always)]
    pub const fn FLEXCOMM5_IRQ(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub const fn set_FLEXCOMM5_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
    #[must_use]
    #[inline(always)]
    pub const fn FLEXCOMM6_IRQ(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub const fn set_FLEXCOMM6_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
    #[must_use]
    #[inline(always)]
    pub const fn FLEXCOMM7_IRQ(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub const fn set_FLEXCOMM7_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "General Purpose ADC interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_IRQ(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose ADC interrupt."]
    #[inline(always)]
    pub const fn set_ADC_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Analog Comparator interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ACMP_IRQ(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator interrupt."]
    #[inline(always)]
    pub const fn set_ACMP_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB Full Speed Controller Clock request interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn USB0_NEEDCLK(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB Full Speed Controller Clock request interrupt."]
    #[inline(always)]
    pub const fn set_USB0_NEEDCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "USB Full Speed Controller interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn USB0_IRQ(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "USB Full Speed Controller interrupt."]
    #[inline(always)]
    pub const fn set_USB0_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC_IRQ(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ."]
    #[inline(always)]
    pub const fn set_RTC_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Mailbox interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn MAILBOX_IRQ(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Mailbox interrupt."]
    #[inline(always)]
    pub const fn set_MAILBOX_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SEC_CPU_INT_MASK0 {
    #[inline(always)]
    fn default() -> SEC_CPU_INT_MASK0 {
        SEC_CPU_INT_MASK0(0)
    }
}
impl core::fmt::Debug for SEC_CPU_INT_MASK0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CPU_INT_MASK0")
            .field("SYS_IRQ", &self.SYS_IRQ())
            .field("SDMA0_IRQ", &self.SDMA0_IRQ())
            .field("GPIO_GLOBALINT0_IRQ", &self.GPIO_GLOBALINT0_IRQ())
            .field("GPIO_GLOBALINT1_IRQ", &self.GPIO_GLOBALINT1_IRQ())
            .field("GPIO_INT0_IRQ0", &self.GPIO_INT0_IRQ0())
            .field("GPIO_INT0_IRQ1", &self.GPIO_INT0_IRQ1())
            .field("GPIO_INT0_IRQ2", &self.GPIO_INT0_IRQ2())
            .field("GPIO_INT0_IRQ3", &self.GPIO_INT0_IRQ3())
            .field("UTICK_IRQ", &self.UTICK_IRQ())
            .field("MRT_IRQ", &self.MRT_IRQ())
            .field("CTIMER0_IRQ", &self.CTIMER0_IRQ())
            .field("CTIMER1_IRQ", &self.CTIMER1_IRQ())
            .field("SCT_IRQ", &self.SCT_IRQ())
            .field("CTIMER3_IRQ", &self.CTIMER3_IRQ())
            .field("FLEXCOMM0_IRQ", &self.FLEXCOMM0_IRQ())
            .field("FLEXCOMM1_IRQ", &self.FLEXCOMM1_IRQ())
            .field("FLEXCOMM2_IRQ", &self.FLEXCOMM2_IRQ())
            .field("FLEXCOMM3_IRQ", &self.FLEXCOMM3_IRQ())
            .field("FLEXCOMM4_IRQ", &self.FLEXCOMM4_IRQ())
            .field("FLEXCOMM5_IRQ", &self.FLEXCOMM5_IRQ())
            .field("FLEXCOMM6_IRQ", &self.FLEXCOMM6_IRQ())
            .field("FLEXCOMM7_IRQ", &self.FLEXCOMM7_IRQ())
            .field("ADC_IRQ", &self.ADC_IRQ())
            .field("RESERVED0", &self.RESERVED0())
            .field("ACMP_IRQ", &self.ACMP_IRQ())
            .field("RESERVED1", &self.RESERVED1())
            .field("RESERVED2", &self.RESERVED2())
            .field("USB0_NEEDCLK", &self.USB0_NEEDCLK())
            .field("USB0_IRQ", &self.USB0_IRQ())
            .field("RTC_IRQ", &self.RTC_IRQ())
            .field("RESERVED3", &self.RESERVED3())
            .field("MAILBOX_IRQ", &self.MAILBOX_IRQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CPU_INT_MASK0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CPU_INT_MASK0 {{ SYS_IRQ: {=bool:?}, SDMA0_IRQ: {=bool:?}, GPIO_GLOBALINT0_IRQ: {=bool:?}, GPIO_GLOBALINT1_IRQ: {=bool:?}, GPIO_INT0_IRQ0: {=bool:?}, GPIO_INT0_IRQ1: {=bool:?}, GPIO_INT0_IRQ2: {=bool:?}, GPIO_INT0_IRQ3: {=bool:?}, UTICK_IRQ: {=bool:?}, MRT_IRQ: {=bool:?}, CTIMER0_IRQ: {=bool:?}, CTIMER1_IRQ: {=bool:?}, SCT_IRQ: {=bool:?}, CTIMER3_IRQ: {=bool:?}, FLEXCOMM0_IRQ: {=bool:?}, FLEXCOMM1_IRQ: {=bool:?}, FLEXCOMM2_IRQ: {=bool:?}, FLEXCOMM3_IRQ: {=bool:?}, FLEXCOMM4_IRQ: {=bool:?}, FLEXCOMM5_IRQ: {=bool:?}, FLEXCOMM6_IRQ: {=bool:?}, FLEXCOMM7_IRQ: {=bool:?}, ADC_IRQ: {=bool:?}, RESERVED0: {=bool:?}, ACMP_IRQ: {=bool:?}, RESERVED1: {=bool:?}, RESERVED2: {=bool:?}, USB0_NEEDCLK: {=bool:?}, USB0_IRQ: {=bool:?}, RTC_IRQ: {=bool:?}, RESERVED3: {=bool:?}, MAILBOX_IRQ: {=bool:?} }}",
            self.SYS_IRQ(),
            self.SDMA0_IRQ(),
            self.GPIO_GLOBALINT0_IRQ(),
            self.GPIO_GLOBALINT1_IRQ(),
            self.GPIO_INT0_IRQ0(),
            self.GPIO_INT0_IRQ1(),
            self.GPIO_INT0_IRQ2(),
            self.GPIO_INT0_IRQ3(),
            self.UTICK_IRQ(),
            self.MRT_IRQ(),
            self.CTIMER0_IRQ(),
            self.CTIMER1_IRQ(),
            self.SCT_IRQ(),
            self.CTIMER3_IRQ(),
            self.FLEXCOMM0_IRQ(),
            self.FLEXCOMM1_IRQ(),
            self.FLEXCOMM2_IRQ(),
            self.FLEXCOMM3_IRQ(),
            self.FLEXCOMM4_IRQ(),
            self.FLEXCOMM5_IRQ(),
            self.FLEXCOMM6_IRQ(),
            self.FLEXCOMM7_IRQ(),
            self.ADC_IRQ(),
            self.RESERVED0(),
            self.ACMP_IRQ(),
            self.RESERVED1(),
            self.RESERVED2(),
            self.USB0_NEEDCLK(),
            self.USB0_IRQ(),
            self.RTC_IRQ(),
            self.RESERVED3(),
            self.MAILBOX_IRQ()
        )
    }
}
#[doc = "Secure Interrupt mask for CPU1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CPU_INT_MASK1(pub u32);
impl SEC_CPU_INT_MASK1 {
    #[doc = "Pin interrupt 4 or pattern match engine slice 4 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_INT0_IRQ4(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pin interrupt 4 or pattern match engine slice 4 interrupt."]
    #[inline(always)]
    pub const fn set_GPIO_INT0_IRQ4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pin interrupt 5 or pattern match engine slice 5 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_INT0_IRQ5(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pin interrupt 5 or pattern match engine slice 5 interrupt."]
    #[inline(always)]
    pub const fn set_GPIO_INT0_IRQ5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pin interrupt 6 or pattern match engine slice 6 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_INT0_IRQ6(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Pin interrupt 6 or pattern match engine slice 6 interrupt."]
    #[inline(always)]
    pub const fn set_GPIO_INT0_IRQ6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Pin interrupt 7 or pattern match engine slice 7 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_INT0_IRQ7(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Pin interrupt 7 or pattern match engine slice 7 interrupt."]
    #[inline(always)]
    pub const fn set_GPIO_INT0_IRQ7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Standard counter/timer 2 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn CTIMER2_IRQ(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Standard counter/timer 2 interrupt."]
    #[inline(always)]
    pub const fn set_CTIMER2_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Standard counter/timer 4 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn CTIMER4_IRQ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Standard counter/timer 4 interrupt."]
    #[inline(always)]
    pub const fn set_CTIMER4_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OS Event Timer and OS Event Timer Wakeup interrupts."]
    #[must_use]
    #[inline(always)]
    pub const fn OS_EVENT_TIMER_IRQ(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OS Event Timer and OS Event Timer Wakeup interrupts."]
    #[inline(always)]
    pub const fn set_OS_EVENT_TIMER_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SDIO Controller interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn SDIO_IRQ(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SDIO Controller interrupt."]
    #[inline(always)]
    pub const fn set_SDIO_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "USB High Speed PHY Controller interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn USB1_PHY_IRQ(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "USB High Speed PHY Controller interrupt."]
    #[inline(always)]
    pub const fn set_USB1_PHY_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "USB High Speed Controller interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn USB1_IRQ(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB High Speed Controller interrupt."]
    #[inline(always)]
    pub const fn set_USB1_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "USB High Speed Controller Clock request interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn USB1_NEEDCLK(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "USB High Speed Controller Clock request interrupt."]
    #[inline(always)]
    pub const fn set_USB1_NEEDCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Secure fault Hyper Visor call interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_HYPERVISOR_CALL_IRQ(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Secure fault Hyper Visor call interrupt."]
    #[inline(always)]
    pub const fn set_SEC_HYPERVISOR_CALL_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Secure Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_GPIO_INT0_IRQ0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Secure Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline(always)]
    pub const fn set_SEC_GPIO_INT0_IRQ0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Secure Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_GPIO_INT0_IRQ1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Secure Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline(always)]
    pub const fn set_SEC_GPIO_INT0_IRQ1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Programmable Look-Up Controller interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn PLU_IRQ(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Programmable Look-Up Controller interrupt."]
    #[inline(always)]
    pub const fn set_PLU_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Security Violation interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_VIO_IRQ(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation interrupt."]
    #[inline(always)]
    pub const fn set_SEC_VIO_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "HASH-AES interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn SHA_IRQ(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "HASH-AES interrupt."]
    #[inline(always)]
    pub const fn set_SHA_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "CASPER interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn CASPER_IRQ(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "CASPER interrupt."]
    #[inline(always)]
    pub const fn set_CASPER_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "PUF interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn PUFKEY_IRQ(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PUF interrupt."]
    #[inline(always)]
    pub const fn set_PUFKEY_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Power Quad interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn PQ_IRQ(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Power Quad interrupt."]
    #[inline(always)]
    pub const fn set_PQ_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "System DMA 1 (Secure) interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn SDMA1_IRQ(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "System DMA 1 (Secure) interrupt."]
    #[inline(always)]
    pub const fn set_SDMA1_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "High Speed SPI interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn LSPI_HS_IRQ(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "High Speed SPI interrupt."]
    #[inline(always)]
    pub const fn set_LSPI_HS_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for SEC_CPU_INT_MASK1 {
    #[inline(always)]
    fn default() -> SEC_CPU_INT_MASK1 {
        SEC_CPU_INT_MASK1(0)
    }
}
impl core::fmt::Debug for SEC_CPU_INT_MASK1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CPU_INT_MASK1")
            .field("GPIO_INT0_IRQ4", &self.GPIO_INT0_IRQ4())
            .field("GPIO_INT0_IRQ5", &self.GPIO_INT0_IRQ5())
            .field("GPIO_INT0_IRQ6", &self.GPIO_INT0_IRQ6())
            .field("GPIO_INT0_IRQ7", &self.GPIO_INT0_IRQ7())
            .field("CTIMER2_IRQ", &self.CTIMER2_IRQ())
            .field("CTIMER4_IRQ", &self.CTIMER4_IRQ())
            .field("OS_EVENT_TIMER_IRQ", &self.OS_EVENT_TIMER_IRQ())
            .field("RESERVED0", &self.RESERVED0())
            .field("RESERVED1", &self.RESERVED1())
            .field("RESERVED2", &self.RESERVED2())
            .field("SDIO_IRQ", &self.SDIO_IRQ())
            .field("RESERVED3", &self.RESERVED3())
            .field("RESERVED4", &self.RESERVED4())
            .field("RESERVED5", &self.RESERVED5())
            .field("USB1_PHY_IRQ", &self.USB1_PHY_IRQ())
            .field("USB1_IRQ", &self.USB1_IRQ())
            .field("USB1_NEEDCLK", &self.USB1_NEEDCLK())
            .field("SEC_HYPERVISOR_CALL_IRQ", &self.SEC_HYPERVISOR_CALL_IRQ())
            .field("SEC_GPIO_INT0_IRQ0", &self.SEC_GPIO_INT0_IRQ0())
            .field("SEC_GPIO_INT0_IRQ1", &self.SEC_GPIO_INT0_IRQ1())
            .field("PLU_IRQ", &self.PLU_IRQ())
            .field("SEC_VIO_IRQ", &self.SEC_VIO_IRQ())
            .field("SHA_IRQ", &self.SHA_IRQ())
            .field("CASPER_IRQ", &self.CASPER_IRQ())
            .field("PUFKEY_IRQ", &self.PUFKEY_IRQ())
            .field("PQ_IRQ", &self.PQ_IRQ())
            .field("SDMA1_IRQ", &self.SDMA1_IRQ())
            .field("LSPI_HS_IRQ", &self.LSPI_HS_IRQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CPU_INT_MASK1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CPU_INT_MASK1 {{ GPIO_INT0_IRQ4: {=bool:?}, GPIO_INT0_IRQ5: {=bool:?}, GPIO_INT0_IRQ6: {=bool:?}, GPIO_INT0_IRQ7: {=bool:?}, CTIMER2_IRQ: {=bool:?}, CTIMER4_IRQ: {=bool:?}, OS_EVENT_TIMER_IRQ: {=bool:?}, RESERVED0: {=bool:?}, RESERVED1: {=bool:?}, RESERVED2: {=bool:?}, SDIO_IRQ: {=bool:?}, RESERVED3: {=bool:?}, RESERVED4: {=bool:?}, RESERVED5: {=bool:?}, USB1_PHY_IRQ: {=bool:?}, USB1_IRQ: {=bool:?}, USB1_NEEDCLK: {=bool:?}, SEC_HYPERVISOR_CALL_IRQ: {=bool:?}, SEC_GPIO_INT0_IRQ0: {=bool:?}, SEC_GPIO_INT0_IRQ1: {=bool:?}, PLU_IRQ: {=bool:?}, SEC_VIO_IRQ: {=bool:?}, SHA_IRQ: {=bool:?}, CASPER_IRQ: {=bool:?}, PUFKEY_IRQ: {=bool:?}, PQ_IRQ: {=bool:?}, SDMA1_IRQ: {=bool:?}, LSPI_HS_IRQ: {=bool:?} }}",
            self.GPIO_INT0_IRQ4(),
            self.GPIO_INT0_IRQ5(),
            self.GPIO_INT0_IRQ6(),
            self.GPIO_INT0_IRQ7(),
            self.CTIMER2_IRQ(),
            self.CTIMER4_IRQ(),
            self.OS_EVENT_TIMER_IRQ(),
            self.RESERVED0(),
            self.RESERVED1(),
            self.RESERVED2(),
            self.SDIO_IRQ(),
            self.RESERVED3(),
            self.RESERVED4(),
            self.RESERVED5(),
            self.USB1_PHY_IRQ(),
            self.USB1_IRQ(),
            self.USB1_NEEDCLK(),
            self.SEC_HYPERVISOR_CALL_IRQ(),
            self.SEC_GPIO_INT0_IRQ0(),
            self.SEC_GPIO_INT0_IRQ1(),
            self.PLU_IRQ(),
            self.SEC_VIO_IRQ(),
            self.SHA_IRQ(),
            self.CASPER_IRQ(),
            self.PUFKEY_IRQ(),
            self.PQ_IRQ(),
            self.SDMA1_IRQ(),
            self.LSPI_HS_IRQ()
        )
    }
}
#[doc = "Security access rules for AHB peripherals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_AHB_PORT10_SLAVE0_RULE(pub u32);
impl SEC_CTRL_AHB_PORT10_SLAVE0_RULE {
    #[doc = "ADC."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_RULE(&self) -> super::vals::ADC_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::ADC_RULE::from_bits(val as u8)
    }
    #[doc = "ADC."]
    #[inline(always)]
    pub const fn set_ADC_RULE(&mut self, val: super::vals::ADC_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "USB Full Speed Host registers."]
    #[must_use]
    #[inline(always)]
    pub const fn USB_FS_HOST_RULE(&self) -> super::vals::USB_FS_HOST_RULE {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::USB_FS_HOST_RULE::from_bits(val as u8)
    }
    #[doc = "USB Full Speed Host registers."]
    #[inline(always)]
    pub const fn set_USB_FS_HOST_RULE(&mut self, val: super::vals::USB_FS_HOST_RULE) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "USB High speed host registers."]
    #[must_use]
    #[inline(always)]
    pub const fn USB_HS_HOST_RULE(&self) -> super::vals::USB_HS_HOST_RULE {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::USB_HS_HOST_RULE::from_bits(val as u8)
    }
    #[doc = "USB High speed host registers."]
    #[inline(always)]
    pub const fn set_USB_HS_HOST_RULE(&mut self, val: super::vals::USB_HS_HOST_RULE) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "SHA-2 crypto registers."]
    #[must_use]
    #[inline(always)]
    pub const fn HASH_RULE(&self) -> super::vals::HASH_RULE {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::HASH_RULE::from_bits(val as u8)
    }
    #[doc = "SHA-2 crypto registers."]
    #[inline(always)]
    pub const fn set_HASH_RULE(&mut self, val: super::vals::HASH_RULE) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "RSA/ECC crypto accelerator."]
    #[must_use]
    #[inline(always)]
    pub const fn CASPER_RULE(&self) -> super::vals::CASPER_RULE {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::CASPER_RULE::from_bits(val as u8)
    }
    #[doc = "RSA/ECC crypto accelerator."]
    #[inline(always)]
    pub const fn set_CASPER_RULE(&mut self, val: super::vals::CASPER_RULE) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Power Quad (CPU0 processor hardware accelerator)."]
    #[must_use]
    #[inline(always)]
    pub const fn PQ_RULE(&self) -> super::vals::PQ_RULE {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::PQ_RULE::from_bits(val as u8)
    }
    #[doc = "Power Quad (CPU0 processor hardware accelerator)."]
    #[inline(always)]
    pub const fn set_PQ_RULE(&mut self, val: super::vals::PQ_RULE) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "DMA Controller (Secure)."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA1_RULE(&self) -> super::vals::DMA1_RULE {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::DMA1_RULE::from_bits(val as u8)
    }
    #[doc = "DMA Controller (Secure)."]
    #[inline(always)]
    pub const fn set_DMA1_RULE(&mut self, val: super::vals::DMA1_RULE) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_AHB_PORT10_SLAVE0_RULE {
    #[inline(always)]
    fn default() -> SEC_CTRL_AHB_PORT10_SLAVE0_RULE {
        SEC_CTRL_AHB_PORT10_SLAVE0_RULE(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_AHB_PORT10_SLAVE0_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_AHB_PORT10_SLAVE0_RULE")
            .field("ADC_RULE", &self.ADC_RULE())
            .field("USB_FS_HOST_RULE", &self.USB_FS_HOST_RULE())
            .field("USB_HS_HOST_RULE", &self.USB_HS_HOST_RULE())
            .field("HASH_RULE", &self.HASH_RULE())
            .field("CASPER_RULE", &self.CASPER_RULE())
            .field("PQ_RULE", &self.PQ_RULE())
            .field("DMA1_RULE", &self.DMA1_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_AHB_PORT10_SLAVE0_RULE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_AHB_PORT10_SLAVE0_RULE {{ ADC_RULE: {:?}, USB_FS_HOST_RULE: {:?}, USB_HS_HOST_RULE: {:?}, HASH_RULE: {:?}, CASPER_RULE: {:?}, PQ_RULE: {:?}, DMA1_RULE: {:?} }}",
            self.ADC_RULE(),
            self.USB_FS_HOST_RULE(),
            self.USB_HS_HOST_RULE(),
            self.HASH_RULE(),
            self.CASPER_RULE(),
            self.PQ_RULE(),
            self.DMA1_RULE()
        )
    }
}
#[doc = "Security access rules for AHB peripherals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_AHB_PORT10_SLAVE1_RULE(pub u32);
impl SEC_CTRL_AHB_PORT10_SLAVE1_RULE {
    #[doc = "Secure High Speed GPIO."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO1_RULE(&self) -> super::vals::GPIO1_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::GPIO1_RULE::from_bits(val as u8)
    }
    #[doc = "Secure High Speed GPIO."]
    #[inline(always)]
    pub const fn set_GPIO1_RULE(&mut self, val: super::vals::GPIO1_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "AHB Secure Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn AHB_SEC_CTRL_RULE(&self) -> super::vals::AHB_SEC_CTRL_RULE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::AHB_SEC_CTRL_RULE::from_bits(val as u8)
    }
    #[doc = "AHB Secure Controller."]
    #[inline(always)]
    pub const fn set_AHB_SEC_CTRL_RULE(&mut self, val: super::vals::AHB_SEC_CTRL_RULE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for SEC_CTRL_AHB_PORT10_SLAVE1_RULE {
    #[inline(always)]
    fn default() -> SEC_CTRL_AHB_PORT10_SLAVE1_RULE {
        SEC_CTRL_AHB_PORT10_SLAVE1_RULE(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_AHB_PORT10_SLAVE1_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_AHB_PORT10_SLAVE1_RULE")
            .field("GPIO1_RULE", &self.GPIO1_RULE())
            .field("AHB_SEC_CTRL_RULE", &self.AHB_SEC_CTRL_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_AHB_PORT10_SLAVE1_RULE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_AHB_PORT10_SLAVE1_RULE {{ GPIO1_RULE: {:?}, AHB_SEC_CTRL_RULE: {:?} }}",
            self.GPIO1_RULE(),
            self.AHB_SEC_CTRL_RULE()
        )
    }
}
#[doc = "Security access rules for AHB peripherals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_AHB_PORT8_SLAVE0_RULE(pub u32);
impl SEC_CTRL_AHB_PORT8_SLAVE0_RULE {
    #[doc = "DMA Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA0_RULE(&self) -> super::vals::DMA0_RULE {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::DMA0_RULE::from_bits(val as u8)
    }
    #[doc = "DMA Controller."]
    #[inline(always)]
    pub const fn set_DMA0_RULE(&mut self, val: super::vals::DMA0_RULE) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "USB Full-speed device."]
    #[must_use]
    #[inline(always)]
    pub const fn FS_USB_DEV_RULE(&self) -> super::vals::FS_USB_DEV_RULE {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::FS_USB_DEV_RULE::from_bits(val as u8)
    }
    #[doc = "USB Full-speed device."]
    #[inline(always)]
    pub const fn set_FS_USB_DEV_RULE(&mut self, val: super::vals::FS_USB_DEV_RULE) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "SCTimer."]
    #[must_use]
    #[inline(always)]
    pub const fn SCT_RULE(&self) -> super::vals::SCT_RULE {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SCT_RULE::from_bits(val as u8)
    }
    #[doc = "SCTimer."]
    #[inline(always)]
    pub const fn set_SCT_RULE(&mut self, val: super::vals::SCT_RULE) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Flexcomm interface 0."]
    #[must_use]
    #[inline(always)]
    pub const fn FLEXCOMM0_RULE(&self) -> super::vals::FLEXCOMM0_RULE {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::FLEXCOMM0_RULE::from_bits(val as u8)
    }
    #[doc = "Flexcomm interface 0."]
    #[inline(always)]
    pub const fn set_FLEXCOMM0_RULE(&mut self, val: super::vals::FLEXCOMM0_RULE) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Flexcomm interface 1."]
    #[must_use]
    #[inline(always)]
    pub const fn FLEXCOMM1_RULE(&self) -> super::vals::FLEXCOMM1_RULE {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::FLEXCOMM1_RULE::from_bits(val as u8)
    }
    #[doc = "Flexcomm interface 1."]
    #[inline(always)]
    pub const fn set_FLEXCOMM1_RULE(&mut self, val: super::vals::FLEXCOMM1_RULE) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_AHB_PORT8_SLAVE0_RULE {
    #[inline(always)]
    fn default() -> SEC_CTRL_AHB_PORT8_SLAVE0_RULE {
        SEC_CTRL_AHB_PORT8_SLAVE0_RULE(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_AHB_PORT8_SLAVE0_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_AHB_PORT8_SLAVE0_RULE")
            .field("DMA0_RULE", &self.DMA0_RULE())
            .field("FS_USB_DEV_RULE", &self.FS_USB_DEV_RULE())
            .field("SCT_RULE", &self.SCT_RULE())
            .field("FLEXCOMM0_RULE", &self.FLEXCOMM0_RULE())
            .field("FLEXCOMM1_RULE", &self.FLEXCOMM1_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_AHB_PORT8_SLAVE0_RULE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_AHB_PORT8_SLAVE0_RULE {{ DMA0_RULE: {:?}, FS_USB_DEV_RULE: {:?}, SCT_RULE: {:?}, FLEXCOMM0_RULE: {:?}, FLEXCOMM1_RULE: {:?} }}",
            self.DMA0_RULE(),
            self.FS_USB_DEV_RULE(),
            self.SCT_RULE(),
            self.FLEXCOMM0_RULE(),
            self.FLEXCOMM1_RULE()
        )
    }
}
#[doc = "Security access rules for AHB peripherals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_AHB_PORT8_SLAVE1_RULE(pub u32);
impl SEC_CTRL_AHB_PORT8_SLAVE1_RULE {
    #[doc = "Flexcomm interface 2."]
    #[must_use]
    #[inline(always)]
    pub const fn FLEXCOMM2_RULE(&self) -> super::vals::FLEXCOMM2_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::FLEXCOMM2_RULE::from_bits(val as u8)
    }
    #[doc = "Flexcomm interface 2."]
    #[inline(always)]
    pub const fn set_FLEXCOMM2_RULE(&mut self, val: super::vals::FLEXCOMM2_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Flexcomm interface 3."]
    #[must_use]
    #[inline(always)]
    pub const fn FLEXCOMM3_RULE(&self) -> super::vals::FLEXCOMM3_RULE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::FLEXCOMM3_RULE::from_bits(val as u8)
    }
    #[doc = "Flexcomm interface 3."]
    #[inline(always)]
    pub const fn set_FLEXCOMM3_RULE(&mut self, val: super::vals::FLEXCOMM3_RULE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Flexcomm interface 4."]
    #[must_use]
    #[inline(always)]
    pub const fn FLEXCOMM4_RULE(&self) -> super::vals::FLEXCOMM4_RULE {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::FLEXCOMM4_RULE::from_bits(val as u8)
    }
    #[doc = "Flexcomm interface 4."]
    #[inline(always)]
    pub const fn set_FLEXCOMM4_RULE(&mut self, val: super::vals::FLEXCOMM4_RULE) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Inter CPU communication Mailbox."]
    #[must_use]
    #[inline(always)]
    pub const fn MAILBOX_RULE(&self) -> super::vals::MAILBOX_RULE {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::MAILBOX_RULE::from_bits(val as u8)
    }
    #[doc = "Inter CPU communication Mailbox."]
    #[inline(always)]
    pub const fn set_MAILBOX_RULE(&mut self, val: super::vals::MAILBOX_RULE) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "High Speed GPIO."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO0_RULE(&self) -> super::vals::GPIO0_RULE {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::GPIO0_RULE::from_bits(val as u8)
    }
    #[doc = "High Speed GPIO."]
    #[inline(always)]
    pub const fn set_GPIO0_RULE(&mut self, val: super::vals::GPIO0_RULE) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for SEC_CTRL_AHB_PORT8_SLAVE1_RULE {
    #[inline(always)]
    fn default() -> SEC_CTRL_AHB_PORT8_SLAVE1_RULE {
        SEC_CTRL_AHB_PORT8_SLAVE1_RULE(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_AHB_PORT8_SLAVE1_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_AHB_PORT8_SLAVE1_RULE")
            .field("FLEXCOMM2_RULE", &self.FLEXCOMM2_RULE())
            .field("FLEXCOMM3_RULE", &self.FLEXCOMM3_RULE())
            .field("FLEXCOMM4_RULE", &self.FLEXCOMM4_RULE())
            .field("MAILBOX_RULE", &self.MAILBOX_RULE())
            .field("GPIO0_RULE", &self.GPIO0_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_AHB_PORT8_SLAVE1_RULE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_AHB_PORT8_SLAVE1_RULE {{ FLEXCOMM2_RULE: {:?}, FLEXCOMM3_RULE: {:?}, FLEXCOMM4_RULE: {:?}, MAILBOX_RULE: {:?}, GPIO0_RULE: {:?} }}",
            self.FLEXCOMM2_RULE(),
            self.FLEXCOMM3_RULE(),
            self.FLEXCOMM4_RULE(),
            self.MAILBOX_RULE(),
            self.GPIO0_RULE()
        )
    }
}
#[doc = "Security access rules for AHB peripherals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_AHB_PORT9_SLAVE0_RULE(pub u32);
impl SEC_CTRL_AHB_PORT9_SLAVE0_RULE {
    #[doc = "USB high Speed device registers."]
    #[must_use]
    #[inline(always)]
    pub const fn USB_HS_DEV_RULE(&self) -> super::vals::USB_HS_DEV_RULE {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::USB_HS_DEV_RULE::from_bits(val as u8)
    }
    #[doc = "USB high Speed device registers."]
    #[inline(always)]
    pub const fn set_USB_HS_DEV_RULE(&mut self, val: super::vals::USB_HS_DEV_RULE) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CRC engine."]
    #[must_use]
    #[inline(always)]
    pub const fn CRC_RULE(&self) -> super::vals::CRC_RULE {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::CRC_RULE::from_bits(val as u8)
    }
    #[doc = "CRC engine."]
    #[inline(always)]
    pub const fn set_CRC_RULE(&mut self, val: super::vals::CRC_RULE) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Flexcomm interface 5."]
    #[must_use]
    #[inline(always)]
    pub const fn FLEXCOMM5_RULE(&self) -> super::vals::FLEXCOMM5_RULE {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::FLEXCOMM5_RULE::from_bits(val as u8)
    }
    #[doc = "Flexcomm interface 5."]
    #[inline(always)]
    pub const fn set_FLEXCOMM5_RULE(&mut self, val: super::vals::FLEXCOMM5_RULE) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Flexcomm interface 6."]
    #[must_use]
    #[inline(always)]
    pub const fn FLEXCOMM6_RULE(&self) -> super::vals::FLEXCOMM6_RULE {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::FLEXCOMM6_RULE::from_bits(val as u8)
    }
    #[doc = "Flexcomm interface 6."]
    #[inline(always)]
    pub const fn set_FLEXCOMM6_RULE(&mut self, val: super::vals::FLEXCOMM6_RULE) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_AHB_PORT9_SLAVE0_RULE {
    #[inline(always)]
    fn default() -> SEC_CTRL_AHB_PORT9_SLAVE0_RULE {
        SEC_CTRL_AHB_PORT9_SLAVE0_RULE(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_AHB_PORT9_SLAVE0_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_AHB_PORT9_SLAVE0_RULE")
            .field("USB_HS_DEV_RULE", &self.USB_HS_DEV_RULE())
            .field("CRC_RULE", &self.CRC_RULE())
            .field("FLEXCOMM5_RULE", &self.FLEXCOMM5_RULE())
            .field("FLEXCOMM6_RULE", &self.FLEXCOMM6_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_AHB_PORT9_SLAVE0_RULE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_AHB_PORT9_SLAVE0_RULE {{ USB_HS_DEV_RULE: {:?}, CRC_RULE: {:?}, FLEXCOMM5_RULE: {:?}, FLEXCOMM6_RULE: {:?} }}",
            self.USB_HS_DEV_RULE(),
            self.CRC_RULE(),
            self.FLEXCOMM5_RULE(),
            self.FLEXCOMM6_RULE()
        )
    }
}
#[doc = "Security access rules for AHB peripherals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_AHB_PORT9_SLAVE1_RULE(pub u32);
impl SEC_CTRL_AHB_PORT9_SLAVE1_RULE {
    #[doc = "Flexcomm interface 7."]
    #[must_use]
    #[inline(always)]
    pub const fn FLEXCOMM7_RULE(&self) -> super::vals::FLEXCOMM7_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::FLEXCOMM7_RULE::from_bits(val as u8)
    }
    #[doc = "Flexcomm interface 7."]
    #[inline(always)]
    pub const fn set_FLEXCOMM7_RULE(&mut self, val: super::vals::FLEXCOMM7_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SDMMC card interface."]
    #[must_use]
    #[inline(always)]
    pub const fn SDIO_RULE(&self) -> super::vals::SDIO_RULE {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SDIO_RULE::from_bits(val as u8)
    }
    #[doc = "SDMMC card interface."]
    #[inline(always)]
    pub const fn set_SDIO_RULE(&mut self, val: super::vals::SDIO_RULE) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Debug mailbox (aka ISP-AP)."]
    #[must_use]
    #[inline(always)]
    pub const fn DBG_MAILBOX_RULE(&self) -> super::vals::DBG_MAILBOX_RULE {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::DBG_MAILBOX_RULE::from_bits(val as u8)
    }
    #[doc = "Debug mailbox (aka ISP-AP)."]
    #[inline(always)]
    pub const fn set_DBG_MAILBOX_RULE(&mut self, val: super::vals::DBG_MAILBOX_RULE) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "High Speed SPI."]
    #[must_use]
    #[inline(always)]
    pub const fn HS_LSPI_RULE(&self) -> super::vals::HS_LSPI_RULE {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::HS_LSPI_RULE::from_bits(val as u8)
    }
    #[doc = "High Speed SPI."]
    #[inline(always)]
    pub const fn set_HS_LSPI_RULE(&mut self, val: super::vals::HS_LSPI_RULE) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_AHB_PORT9_SLAVE1_RULE {
    #[inline(always)]
    fn default() -> SEC_CTRL_AHB_PORT9_SLAVE1_RULE {
        SEC_CTRL_AHB_PORT9_SLAVE1_RULE(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_AHB_PORT9_SLAVE1_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_AHB_PORT9_SLAVE1_RULE")
            .field("FLEXCOMM7_RULE", &self.FLEXCOMM7_RULE())
            .field("SDIO_RULE", &self.SDIO_RULE())
            .field("DBG_MAILBOX_RULE", &self.DBG_MAILBOX_RULE())
            .field("HS_LSPI_RULE", &self.HS_LSPI_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_AHB_PORT9_SLAVE1_RULE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_AHB_PORT9_SLAVE1_RULE {{ FLEXCOMM7_RULE: {:?}, SDIO_RULE: {:?}, DBG_MAILBOX_RULE: {:?}, HS_LSPI_RULE: {:?} }}",
            self.FLEXCOMM7_RULE(),
            self.SDIO_RULE(),
            self.DBG_MAILBOX_RULE(),
            self.HS_LSPI_RULE()
        )
    }
}
#[doc = "Security access rules for AHB_SEC_CTRL_AHB."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_AHB_SEC_CTRL_MEM_RULE(pub u32);
impl SEC_CTRL_AHB_SEC_CTRL_MEM_RULE {
    #[doc = "Address space: 0x400A_0000 - 0x400A_CFFF."]
    #[must_use]
    #[inline(always)]
    pub const fn AHB_SEC_CTRL_SECT_0_RULE(&self) -> super::vals::AHB_SEC_CTRL_SECT_0_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::AHB_SEC_CTRL_SECT_0_RULE::from_bits(val as u8)
    }
    #[doc = "Address space: 0x400A_0000 - 0x400A_CFFF."]
    #[inline(always)]
    pub const fn set_AHB_SEC_CTRL_SECT_0_RULE(
        &mut self,
        val: super::vals::AHB_SEC_CTRL_SECT_0_RULE,
    ) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Address space: 0x400A_D000 - 0x400A_DFFF."]
    #[must_use]
    #[inline(always)]
    pub const fn AHB_SEC_CTRL_SECT_1_RULE(&self) -> super::vals::AHB_SEC_CTRL_SECT_1_RULE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::AHB_SEC_CTRL_SECT_1_RULE::from_bits(val as u8)
    }
    #[doc = "Address space: 0x400A_D000 - 0x400A_DFFF."]
    #[inline(always)]
    pub const fn set_AHB_SEC_CTRL_SECT_1_RULE(
        &mut self,
        val: super::vals::AHB_SEC_CTRL_SECT_1_RULE,
    ) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Address space: 0x400A_E000 - 0x400A_EFFF."]
    #[must_use]
    #[inline(always)]
    pub const fn AHB_SEC_CTRL_SECT_2_RULE(&self) -> super::vals::AHB_SEC_CTRL_SECT_2_RULE {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::AHB_SEC_CTRL_SECT_2_RULE::from_bits(val as u8)
    }
    #[doc = "Address space: 0x400A_E000 - 0x400A_EFFF."]
    #[inline(always)]
    pub const fn set_AHB_SEC_CTRL_SECT_2_RULE(
        &mut self,
        val: super::vals::AHB_SEC_CTRL_SECT_2_RULE,
    ) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Address space: 0x400A_F000 - 0x400A_FFFF."]
    #[must_use]
    #[inline(always)]
    pub const fn AHB_SEC_CTRL_SECT_3_RULE(&self) -> super::vals::AHB_SEC_CTRL_SECT_3_RULE {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::AHB_SEC_CTRL_SECT_3_RULE::from_bits(val as u8)
    }
    #[doc = "Address space: 0x400A_F000 - 0x400A_FFFF."]
    #[inline(always)]
    pub const fn set_AHB_SEC_CTRL_SECT_3_RULE(
        &mut self,
        val: super::vals::AHB_SEC_CTRL_SECT_3_RULE,
    ) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for SEC_CTRL_AHB_SEC_CTRL_MEM_RULE {
    #[inline(always)]
    fn default() -> SEC_CTRL_AHB_SEC_CTRL_MEM_RULE {
        SEC_CTRL_AHB_SEC_CTRL_MEM_RULE(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_AHB_SEC_CTRL_MEM_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_AHB_SEC_CTRL_MEM_RULE")
            .field("AHB_SEC_CTRL_SECT_0_RULE", &self.AHB_SEC_CTRL_SECT_0_RULE())
            .field("AHB_SEC_CTRL_SECT_1_RULE", &self.AHB_SEC_CTRL_SECT_1_RULE())
            .field("AHB_SEC_CTRL_SECT_2_RULE", &self.AHB_SEC_CTRL_SECT_2_RULE())
            .field("AHB_SEC_CTRL_SECT_3_RULE", &self.AHB_SEC_CTRL_SECT_3_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_AHB_SEC_CTRL_MEM_RULE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_AHB_SEC_CTRL_MEM_RULE {{ AHB_SEC_CTRL_SECT_0_RULE: {:?}, AHB_SEC_CTRL_SECT_1_RULE: {:?}, AHB_SEC_CTRL_SECT_2_RULE: {:?}, AHB_SEC_CTRL_SECT_3_RULE: {:?} }}",
            self.AHB_SEC_CTRL_SECT_0_RULE(),
            self.AHB_SEC_CTRL_SECT_1_RULE(),
            self.AHB_SEC_CTRL_SECT_2_RULE(),
            self.AHB_SEC_CTRL_SECT_3_RULE()
        )
    }
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_APB_BRIDGE0_MEM_CTRL0(pub u32);
impl SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 {
    #[doc = "System Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn SYSCON_RULE(&self) -> super::vals::SYSCON_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SYSCON_RULE::from_bits(val as u8)
    }
    #[doc = "System Configuration."]
    #[inline(always)]
    pub const fn set_SYSCON_RULE(&mut self, val: super::vals::SYSCON_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "I/O Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCON_RULE(&self) -> super::vals::IOCON_RULE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::IOCON_RULE::from_bits(val as u8)
    }
    #[doc = "I/O Configuration."]
    #[inline(always)]
    pub const fn set_IOCON_RULE(&mut self, val: super::vals::IOCON_RULE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "GPIO input Interrupt 0."]
    #[must_use]
    #[inline(always)]
    pub const fn GINT0_RULE(&self) -> super::vals::GINT0_RULE {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::GINT0_RULE::from_bits(val as u8)
    }
    #[doc = "GPIO input Interrupt 0."]
    #[inline(always)]
    pub const fn set_GINT0_RULE(&mut self, val: super::vals::GINT0_RULE) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "GPIO input Interrupt 1."]
    #[must_use]
    #[inline(always)]
    pub const fn GINT1_RULE(&self) -> super::vals::GINT1_RULE {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::GINT1_RULE::from_bits(val as u8)
    }
    #[doc = "GPIO input Interrupt 1."]
    #[inline(always)]
    pub const fn set_GINT1_RULE(&mut self, val: super::vals::GINT1_RULE) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Pin Interrupt and Pattern match."]
    #[must_use]
    #[inline(always)]
    pub const fn PINT_RULE(&self) -> super::vals::PINT_RULE {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::PINT_RULE::from_bits(val as u8)
    }
    #[doc = "Pin Interrupt and Pattern match."]
    #[inline(always)]
    pub const fn set_PINT_RULE(&mut self, val: super::vals::PINT_RULE) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Secure Pin Interrupt and Pattern match."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_PINT_RULE(&self) -> super::vals::SEC_PINT_RULE {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SEC_PINT_RULE::from_bits(val as u8)
    }
    #[doc = "Secure Pin Interrupt and Pattern match."]
    #[inline(always)]
    pub const fn set_SEC_PINT_RULE(&mut self, val: super::vals::SEC_PINT_RULE) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Peripheral input multiplexing."]
    #[must_use]
    #[inline(always)]
    pub const fn INPUTMUX_RULE(&self) -> super::vals::INPUTMUX_RULE {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::INPUTMUX_RULE::from_bits(val as u8)
    }
    #[doc = "Peripheral input multiplexing."]
    #[inline(always)]
    pub const fn set_INPUTMUX_RULE(&mut self, val: super::vals::INPUTMUX_RULE) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 {
    #[inline(always)]
    fn default() -> SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 {
        SEC_CTRL_APB_BRIDGE0_MEM_CTRL0(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_APB_BRIDGE0_MEM_CTRL0")
            .field("SYSCON_RULE", &self.SYSCON_RULE())
            .field("IOCON_RULE", &self.IOCON_RULE())
            .field("GINT0_RULE", &self.GINT0_RULE())
            .field("GINT1_RULE", &self.GINT1_RULE())
            .field("PINT_RULE", &self.PINT_RULE())
            .field("SEC_PINT_RULE", &self.SEC_PINT_RULE())
            .field("INPUTMUX_RULE", &self.INPUTMUX_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 {{ SYSCON_RULE: {:?}, IOCON_RULE: {:?}, GINT0_RULE: {:?}, GINT1_RULE: {:?}, PINT_RULE: {:?}, SEC_PINT_RULE: {:?}, INPUTMUX_RULE: {:?} }}",
            self.SYSCON_RULE(),
            self.IOCON_RULE(),
            self.GINT0_RULE(),
            self.GINT1_RULE(),
            self.PINT_RULE(),
            self.SEC_PINT_RULE(),
            self.INPUTMUX_RULE()
        )
    }
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_APB_BRIDGE0_MEM_CTRL1(pub u32);
impl SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 {
    #[doc = "Standard counter/Timer 0."]
    #[must_use]
    #[inline(always)]
    pub const fn CTIMER0_RULE(&self) -> super::vals::CTIMER0_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CTIMER0_RULE::from_bits(val as u8)
    }
    #[doc = "Standard counter/Timer 0."]
    #[inline(always)]
    pub const fn set_CTIMER0_RULE(&mut self, val: super::vals::CTIMER0_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Standard counter/Timer 1."]
    #[must_use]
    #[inline(always)]
    pub const fn CTIMER1_RULE(&self) -> super::vals::CTIMER1_RULE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::CTIMER1_RULE::from_bits(val as u8)
    }
    #[doc = "Standard counter/Timer 1."]
    #[inline(always)]
    pub const fn set_CTIMER1_RULE(&mut self, val: super::vals::CTIMER1_RULE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Windiwed wtachdog Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn WWDT_RULE(&self) -> super::vals::WWDT_RULE {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::WWDT_RULE::from_bits(val as u8)
    }
    #[doc = "Windiwed wtachdog Timer."]
    #[inline(always)]
    pub const fn set_WWDT_RULE(&mut self, val: super::vals::WWDT_RULE) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Multi-rate Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn MRT_RULE(&self) -> super::vals::MRT_RULE {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::MRT_RULE::from_bits(val as u8)
    }
    #[doc = "Multi-rate Timer."]
    #[inline(always)]
    pub const fn set_MRT_RULE(&mut self, val: super::vals::MRT_RULE) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Micro-Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn UTICK_RULE(&self) -> super::vals::UTICK_RULE {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::UTICK_RULE::from_bits(val as u8)
    }
    #[doc = "Micro-Timer."]
    #[inline(always)]
    pub const fn set_UTICK_RULE(&mut self, val: super::vals::UTICK_RULE) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 {
    #[inline(always)]
    fn default() -> SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 {
        SEC_CTRL_APB_BRIDGE0_MEM_CTRL1(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_APB_BRIDGE0_MEM_CTRL1")
            .field("CTIMER0_RULE", &self.CTIMER0_RULE())
            .field("CTIMER1_RULE", &self.CTIMER1_RULE())
            .field("WWDT_RULE", &self.WWDT_RULE())
            .field("MRT_RULE", &self.MRT_RULE())
            .field("UTICK_RULE", &self.UTICK_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 {{ CTIMER0_RULE: {:?}, CTIMER1_RULE: {:?}, WWDT_RULE: {:?}, MRT_RULE: {:?}, UTICK_RULE: {:?} }}",
            self.CTIMER0_RULE(),
            self.CTIMER1_RULE(),
            self.WWDT_RULE(),
            self.MRT_RULE(),
            self.UTICK_RULE()
        )
    }
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_APB_BRIDGE0_MEM_CTRL2(pub u32);
impl SEC_CTRL_APB_BRIDGE0_MEM_CTRL2 {
    #[doc = "Analog Modules controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ANACTRL_RULE(&self) -> super::vals::ANACTRL_RULE {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::ANACTRL_RULE::from_bits(val as u8)
    }
    #[doc = "Analog Modules controller."]
    #[inline(always)]
    pub const fn set_ANACTRL_RULE(&mut self, val: super::vals::ANACTRL_RULE) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for SEC_CTRL_APB_BRIDGE0_MEM_CTRL2 {
    #[inline(always)]
    fn default() -> SEC_CTRL_APB_BRIDGE0_MEM_CTRL2 {
        SEC_CTRL_APB_BRIDGE0_MEM_CTRL2(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_APB_BRIDGE0_MEM_CTRL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_APB_BRIDGE0_MEM_CTRL2")
            .field("ANACTRL_RULE", &self.ANACTRL_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_APB_BRIDGE0_MEM_CTRL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_APB_BRIDGE0_MEM_CTRL2 {{ ANACTRL_RULE: {:?} }}",
            self.ANACTRL_RULE()
        )
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_APB_BRIDGE1_MEM_CTRL0(pub u32);
impl SEC_CTRL_APB_BRIDGE1_MEM_CTRL0 {
    #[doc = "Power Management Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn PMC_RULE(&self) -> super::vals::PMC_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::PMC_RULE::from_bits(val as u8)
    }
    #[doc = "Power Management Controller."]
    #[inline(always)]
    pub const fn set_PMC_RULE(&mut self, val: super::vals::PMC_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "System Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn SYSCTRL_RULE(&self) -> super::vals::SYSCTRL_RULE {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SYSCTRL_RULE::from_bits(val as u8)
    }
    #[doc = "System Controller."]
    #[inline(always)]
    pub const fn set_SYSCTRL_RULE(&mut self, val: super::vals::SYSCTRL_RULE) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for SEC_CTRL_APB_BRIDGE1_MEM_CTRL0 {
    #[inline(always)]
    fn default() -> SEC_CTRL_APB_BRIDGE1_MEM_CTRL0 {
        SEC_CTRL_APB_BRIDGE1_MEM_CTRL0(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_APB_BRIDGE1_MEM_CTRL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_APB_BRIDGE1_MEM_CTRL0")
            .field("PMC_RULE", &self.PMC_RULE())
            .field("SYSCTRL_RULE", &self.SYSCTRL_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_APB_BRIDGE1_MEM_CTRL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_APB_BRIDGE1_MEM_CTRL0 {{ PMC_RULE: {:?}, SYSCTRL_RULE: {:?} }}",
            self.PMC_RULE(),
            self.SYSCTRL_RULE()
        )
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_APB_BRIDGE1_MEM_CTRL1(pub u32);
impl SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 {
    #[doc = "Standard counter/Timer 2."]
    #[must_use]
    #[inline(always)]
    pub const fn CTIMER2_RULE(&self) -> super::vals::CTIMER2_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CTIMER2_RULE::from_bits(val as u8)
    }
    #[doc = "Standard counter/Timer 2."]
    #[inline(always)]
    pub const fn set_CTIMER2_RULE(&mut self, val: super::vals::CTIMER2_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Standard counter/Timer 3."]
    #[must_use]
    #[inline(always)]
    pub const fn CTIMER3_RULE(&self) -> super::vals::CTIMER3_RULE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::CTIMER3_RULE::from_bits(val as u8)
    }
    #[doc = "Standard counter/Timer 3."]
    #[inline(always)]
    pub const fn set_CTIMER3_RULE(&mut self, val: super::vals::CTIMER3_RULE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Standard counter/Timer 4."]
    #[must_use]
    #[inline(always)]
    pub const fn CTIMER4_RULE(&self) -> super::vals::CTIMER4_RULE {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::CTIMER4_RULE::from_bits(val as u8)
    }
    #[doc = "Standard counter/Timer 4."]
    #[inline(always)]
    pub const fn set_CTIMER4_RULE(&mut self, val: super::vals::CTIMER4_RULE) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Real Time Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC_RULE(&self) -> super::vals::RTC_RULE {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::RTC_RULE::from_bits(val as u8)
    }
    #[doc = "Real Time Counter."]
    #[inline(always)]
    pub const fn set_RTC_RULE(&mut self, val: super::vals::RTC_RULE) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "OS Event Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn OSEVENT_RULE(&self) -> super::vals::OSEVENT_RULE {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::OSEVENT_RULE::from_bits(val as u8)
    }
    #[doc = "OS Event Timer."]
    #[inline(always)]
    pub const fn set_OSEVENT_RULE(&mut self, val: super::vals::OSEVENT_RULE) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 {
    #[inline(always)]
    fn default() -> SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 {
        SEC_CTRL_APB_BRIDGE1_MEM_CTRL1(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_APB_BRIDGE1_MEM_CTRL1")
            .field("CTIMER2_RULE", &self.CTIMER2_RULE())
            .field("CTIMER3_RULE", &self.CTIMER3_RULE())
            .field("CTIMER4_RULE", &self.CTIMER4_RULE())
            .field("RTC_RULE", &self.RTC_RULE())
            .field("OSEVENT_RULE", &self.OSEVENT_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 {{ CTIMER2_RULE: {:?}, CTIMER3_RULE: {:?}, CTIMER4_RULE: {:?}, RTC_RULE: {:?}, OSEVENT_RULE: {:?} }}",
            self.CTIMER2_RULE(),
            self.CTIMER3_RULE(),
            self.CTIMER4_RULE(),
            self.RTC_RULE(),
            self.OSEVENT_RULE()
        )
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_APB_BRIDGE1_MEM_CTRL2(pub u32);
impl SEC_CTRL_APB_BRIDGE1_MEM_CTRL2 {
    #[doc = "Flash Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASH_CTRL_RULE(&self) -> super::vals::FLASH_CTRL_RULE {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::FLASH_CTRL_RULE::from_bits(val as u8)
    }
    #[doc = "Flash Controller."]
    #[inline(always)]
    pub const fn set_FLASH_CTRL_RULE(&mut self, val: super::vals::FLASH_CTRL_RULE) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Prince."]
    #[must_use]
    #[inline(always)]
    pub const fn PRINCE_RULE(&self) -> super::vals::PRINCE_RULE {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::PRINCE_RULE::from_bits(val as u8)
    }
    #[doc = "Prince."]
    #[inline(always)]
    pub const fn set_PRINCE_RULE(&mut self, val: super::vals::PRINCE_RULE) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for SEC_CTRL_APB_BRIDGE1_MEM_CTRL2 {
    #[inline(always)]
    fn default() -> SEC_CTRL_APB_BRIDGE1_MEM_CTRL2 {
        SEC_CTRL_APB_BRIDGE1_MEM_CTRL2(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_APB_BRIDGE1_MEM_CTRL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_APB_BRIDGE1_MEM_CTRL2")
            .field("FLASH_CTRL_RULE", &self.FLASH_CTRL_RULE())
            .field("PRINCE_RULE", &self.PRINCE_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_APB_BRIDGE1_MEM_CTRL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_APB_BRIDGE1_MEM_CTRL2 {{ FLASH_CTRL_RULE: {:?}, PRINCE_RULE: {:?} }}",
            self.FLASH_CTRL_RULE(),
            self.PRINCE_RULE()
        )
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_APB_BRIDGE1_MEM_CTRL3(pub u32);
impl SEC_CTRL_APB_BRIDGE1_MEM_CTRL3 {
    #[doc = "USB High Speed Phy controller."]
    #[must_use]
    #[inline(always)]
    pub const fn USBHPHY_RULE(&self) -> super::vals::USBHPHY_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::USBHPHY_RULE::from_bits(val as u8)
    }
    #[doc = "USB High Speed Phy controller."]
    #[inline(always)]
    pub const fn set_USBHPHY_RULE(&mut self, val: super::vals::USBHPHY_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "True Random Number Generator."]
    #[must_use]
    #[inline(always)]
    pub const fn RNG_RULE(&self) -> super::vals::RNG_RULE {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RNG_RULE::from_bits(val as u8)
    }
    #[doc = "True Random Number Generator."]
    #[inline(always)]
    pub const fn set_RNG_RULE(&mut self, val: super::vals::RNG_RULE) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "PUF."]
    #[must_use]
    #[inline(always)]
    pub const fn PUF_RULE(&self) -> super::vals::PUF_RULE {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::PUF_RULE::from_bits(val as u8)
    }
    #[doc = "PUF."]
    #[inline(always)]
    pub const fn set_PUF_RULE(&mut self, val: super::vals::PUF_RULE) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Programmable Look-Up logic."]
    #[must_use]
    #[inline(always)]
    pub const fn PLU_RULE(&self) -> super::vals::PLU_RULE {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::PLU_RULE::from_bits(val as u8)
    }
    #[doc = "Programmable Look-Up logic."]
    #[inline(always)]
    pub const fn set_PLU_RULE(&mut self, val: super::vals::PLU_RULE) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for SEC_CTRL_APB_BRIDGE1_MEM_CTRL3 {
    #[inline(always)]
    fn default() -> SEC_CTRL_APB_BRIDGE1_MEM_CTRL3 {
        SEC_CTRL_APB_BRIDGE1_MEM_CTRL3(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_APB_BRIDGE1_MEM_CTRL3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_APB_BRIDGE1_MEM_CTRL3")
            .field("USBHPHY_RULE", &self.USBHPHY_RULE())
            .field("RNG_RULE", &self.RNG_RULE())
            .field("PUF_RULE", &self.PUF_RULE())
            .field("PLU_RULE", &self.PLU_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_APB_BRIDGE1_MEM_CTRL3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_APB_BRIDGE1_MEM_CTRL3 {{ USBHPHY_RULE: {:?}, RNG_RULE: {:?}, PUF_RULE: {:?}, PLU_RULE: {:?} }}",
            self.USBHPHY_RULE(),
            self.RNG_RULE(),
            self.PUF_RULE(),
            self.PLU_RULE()
        )
    }
}
#[doc = "Security access rules for both APB Bridges slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_APB_BRIDGE_SLAVE_RULE(pub u32);
impl SEC_CTRL_APB_BRIDGE_SLAVE_RULE {
    #[doc = "Security access rules for the whole APB Bridge 0."]
    #[must_use]
    #[inline(always)]
    pub const fn APBBRIDGE0_RULE(&self) -> super::vals::APBBRIDGE0_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::APBBRIDGE0_RULE::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole APB Bridge 0."]
    #[inline(always)]
    pub const fn set_APBBRIDGE0_RULE(&mut self, val: super::vals::APBBRIDGE0_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Security access rules for the whole APB Bridge 1."]
    #[must_use]
    #[inline(always)]
    pub const fn APBBRIDGE1_RULE(&self) -> super::vals::APBBRIDGE1_RULE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::APBBRIDGE1_RULE::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole APB Bridge 1."]
    #[inline(always)]
    pub const fn set_APBBRIDGE1_RULE(&mut self, val: super::vals::APBBRIDGE1_RULE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for SEC_CTRL_APB_BRIDGE_SLAVE_RULE {
    #[inline(always)]
    fn default() -> SEC_CTRL_APB_BRIDGE_SLAVE_RULE {
        SEC_CTRL_APB_BRIDGE_SLAVE_RULE(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_APB_BRIDGE_SLAVE_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_APB_BRIDGE_SLAVE_RULE")
            .field("APBBRIDGE0_RULE", &self.APBBRIDGE0_RULE())
            .field("APBBRIDGE1_RULE", &self.APBBRIDGE1_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_APB_BRIDGE_SLAVE_RULE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_APB_BRIDGE_SLAVE_RULE {{ APBBRIDGE0_RULE: {:?}, APBBRIDGE1_RULE: {:?} }}",
            self.APBBRIDGE0_RULE(),
            self.APBBRIDGE1_RULE()
        )
    }
}
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_FLASH_MEM_RULE0(pub u32);
impl SEC_CTRL_FLASH_MEM_RULE0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE0(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE0(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE1(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE1(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE2(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE2(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE3(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE3(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE4(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE4(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE5(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE5(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE6(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE6(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE7(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE7(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE0_RULE7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_FLASH_MEM_RULE0 {
    #[inline(always)]
    fn default() -> SEC_CTRL_FLASH_MEM_RULE0 {
        SEC_CTRL_FLASH_MEM_RULE0(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_FLASH_MEM_RULE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_FLASH_MEM_RULE0")
            .field("RULE0", &self.RULE0())
            .field("RULE1", &self.RULE1())
            .field("RULE2", &self.RULE2())
            .field("RULE3", &self.RULE3())
            .field("RULE4", &self.RULE4())
            .field("RULE5", &self.RULE5())
            .field("RULE6", &self.RULE6())
            .field("RULE7", &self.RULE7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_FLASH_MEM_RULE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_FLASH_MEM_RULE0 {{ RULE0: {:?}, RULE1: {:?}, RULE2: {:?}, RULE3: {:?}, RULE4: {:?}, RULE5: {:?}, RULE6: {:?}, RULE7: {:?} }}",
            self.RULE0(),
            self.RULE1(),
            self.RULE2(),
            self.RULE3(),
            self.RULE4(),
            self.RULE5(),
            self.RULE6(),
            self.RULE7()
        )
    }
}
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_FLASH_MEM_RULE1(pub u32);
impl SEC_CTRL_FLASH_MEM_RULE1 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE0(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE0(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE1(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE1(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE2(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE2(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE3(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE3(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE4(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE4(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE5(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE5(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE6(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE6(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE7(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE7(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE1_RULE7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_FLASH_MEM_RULE1 {
    #[inline(always)]
    fn default() -> SEC_CTRL_FLASH_MEM_RULE1 {
        SEC_CTRL_FLASH_MEM_RULE1(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_FLASH_MEM_RULE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_FLASH_MEM_RULE1")
            .field("RULE0", &self.RULE0())
            .field("RULE1", &self.RULE1())
            .field("RULE2", &self.RULE2())
            .field("RULE3", &self.RULE3())
            .field("RULE4", &self.RULE4())
            .field("RULE5", &self.RULE5())
            .field("RULE6", &self.RULE6())
            .field("RULE7", &self.RULE7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_FLASH_MEM_RULE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_FLASH_MEM_RULE1 {{ RULE0: {:?}, RULE1: {:?}, RULE2: {:?}, RULE3: {:?}, RULE4: {:?}, RULE5: {:?}, RULE6: {:?}, RULE7: {:?} }}",
            self.RULE0(),
            self.RULE1(),
            self.RULE2(),
            self.RULE3(),
            self.RULE4(),
            self.RULE5(),
            self.RULE6(),
            self.RULE7()
        )
    }
}
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_FLASH_MEM_RULE2(pub u32);
impl SEC_CTRL_FLASH_MEM_RULE2 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE0(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE0(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE1(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE1(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE2(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE2(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE3(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE3(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE4(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE4(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE5(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE5(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE6(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE6(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE7(&self) -> super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE7(&mut self, val: super::vals::SEC_CTRL_FLASH_MEM_RULE2_RULE7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_FLASH_MEM_RULE2 {
    #[inline(always)]
    fn default() -> SEC_CTRL_FLASH_MEM_RULE2 {
        SEC_CTRL_FLASH_MEM_RULE2(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_FLASH_MEM_RULE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_FLASH_MEM_RULE2")
            .field("RULE0", &self.RULE0())
            .field("RULE1", &self.RULE1())
            .field("RULE2", &self.RULE2())
            .field("RULE3", &self.RULE3())
            .field("RULE4", &self.RULE4())
            .field("RULE5", &self.RULE5())
            .field("RULE6", &self.RULE6())
            .field("RULE7", &self.RULE7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_FLASH_MEM_RULE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_FLASH_MEM_RULE2 {{ RULE0: {:?}, RULE1: {:?}, RULE2: {:?}, RULE3: {:?}, RULE4: {:?}, RULE5: {:?}, RULE6: {:?}, RULE7: {:?} }}",
            self.RULE0(),
            self.RULE1(),
            self.RULE2(),
            self.RULE3(),
            self.RULE4(),
            self.RULE5(),
            self.RULE6(),
            self.RULE7()
        )
    }
}
#[doc = "Security access rules for Flash and ROM slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_FLASH_ROM_SLAVE_RULE(pub u32);
impl SEC_CTRL_FLASH_ROM_SLAVE_RULE {
    #[doc = "Security access rules for the whole FLASH : 0x0000_0000 - 0x0009_FFFF."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASH_RULE(&self) -> super::vals::FLASH_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::FLASH_RULE::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole FLASH : 0x0000_0000 - 0x0009_FFFF."]
    #[inline(always)]
    pub const fn set_FLASH_RULE(&mut self, val: super::vals::FLASH_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Security access rules for the whole ROM : 0x0300_0000 - 0x0301_FFFF."]
    #[must_use]
    #[inline(always)]
    pub const fn ROM_RULE(&self) -> super::vals::ROM_RULE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::ROM_RULE::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole ROM : 0x0300_0000 - 0x0301_FFFF."]
    #[inline(always)]
    pub const fn set_ROM_RULE(&mut self, val: super::vals::ROM_RULE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for SEC_CTRL_FLASH_ROM_SLAVE_RULE {
    #[inline(always)]
    fn default() -> SEC_CTRL_FLASH_ROM_SLAVE_RULE {
        SEC_CTRL_FLASH_ROM_SLAVE_RULE(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_FLASH_ROM_SLAVE_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_FLASH_ROM_SLAVE_RULE")
            .field("FLASH_RULE", &self.FLASH_RULE())
            .field("ROM_RULE", &self.ROM_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_FLASH_ROM_SLAVE_RULE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_FLASH_ROM_SLAVE_RULE {{ FLASH_RULE: {:?}, ROM_RULE: {:?} }}",
            self.FLASH_RULE(),
            self.ROM_RULE()
        )
    }
}
#[doc = "Security access rules for RAM0 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_RAM0_MEM_RULE0(pub u32);
impl SEC_CTRL_RAM0_MEM_RULE0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE0(&self) -> super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE0(&mut self, val: super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE1(&self) -> super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE1(&mut self, val: super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE2(&self) -> super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE2(&mut self, val: super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE3(&self) -> super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE3(&mut self, val: super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE4(&self) -> super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE4(&mut self, val: super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE5(&self) -> super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE5(&mut self, val: super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE6(&self) -> super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE6(&mut self, val: super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE7(&self) -> super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE7(&mut self, val: super::vals::SEC_CTRL_RAM0_MEM_RULE0_RULE7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_RAM0_MEM_RULE0 {
    #[inline(always)]
    fn default() -> SEC_CTRL_RAM0_MEM_RULE0 {
        SEC_CTRL_RAM0_MEM_RULE0(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_RAM0_MEM_RULE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAM0_MEM_RULE0")
            .field("RULE0", &self.RULE0())
            .field("RULE1", &self.RULE1())
            .field("RULE2", &self.RULE2())
            .field("RULE3", &self.RULE3())
            .field("RULE4", &self.RULE4())
            .field("RULE5", &self.RULE5())
            .field("RULE6", &self.RULE6())
            .field("RULE7", &self.RULE7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_RAM0_MEM_RULE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_RAM0_MEM_RULE0 {{ RULE0: {:?}, RULE1: {:?}, RULE2: {:?}, RULE3: {:?}, RULE4: {:?}, RULE5: {:?}, RULE6: {:?}, RULE7: {:?} }}",
            self.RULE0(),
            self.RULE1(),
            self.RULE2(),
            self.RULE3(),
            self.RULE4(),
            self.RULE5(),
            self.RULE6(),
            self.RULE7()
        )
    }
}
#[doc = "Security access rules for RAM0 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_RAM0_MEM_RULE1(pub u32);
impl SEC_CTRL_RAM0_MEM_RULE1 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE0(&self) -> super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE0(&mut self, val: super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE1(&self) -> super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE1(&mut self, val: super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE2(&self) -> super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE2(&mut self, val: super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE3(&self) -> super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE3(&mut self, val: super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE4(&self) -> super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE4(&mut self, val: super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE5(&self) -> super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE5(&mut self, val: super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE6(&self) -> super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE6(&mut self, val: super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE7(&self) -> super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE7(&mut self, val: super::vals::SEC_CTRL_RAM0_MEM_RULE1_RULE7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_RAM0_MEM_RULE1 {
    #[inline(always)]
    fn default() -> SEC_CTRL_RAM0_MEM_RULE1 {
        SEC_CTRL_RAM0_MEM_RULE1(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_RAM0_MEM_RULE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAM0_MEM_RULE1")
            .field("RULE0", &self.RULE0())
            .field("RULE1", &self.RULE1())
            .field("RULE2", &self.RULE2())
            .field("RULE3", &self.RULE3())
            .field("RULE4", &self.RULE4())
            .field("RULE5", &self.RULE5())
            .field("RULE6", &self.RULE6())
            .field("RULE7", &self.RULE7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_RAM0_MEM_RULE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_RAM0_MEM_RULE1 {{ RULE0: {:?}, RULE1: {:?}, RULE2: {:?}, RULE3: {:?}, RULE4: {:?}, RULE5: {:?}, RULE6: {:?}, RULE7: {:?} }}",
            self.RULE0(),
            self.RULE1(),
            self.RULE2(),
            self.RULE3(),
            self.RULE4(),
            self.RULE5(),
            self.RULE6(),
            self.RULE7()
        )
    }
}
#[doc = "Security access rules for RAM0 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_RAM0_SLAVE_RULE(pub u32);
impl SEC_CTRL_RAM0_SLAVE_RULE {
    #[doc = "Security access rules for the whole RAM0 : 0x2000_0000 - 0x2000_FFFF."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM0_RULE(&self) -> super::vals::RAM0_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RAM0_RULE::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole RAM0 : 0x2000_0000 - 0x2000_FFFF."]
    #[inline(always)]
    pub const fn set_RAM0_RULE(&mut self, val: super::vals::RAM0_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SEC_CTRL_RAM0_SLAVE_RULE {
    #[inline(always)]
    fn default() -> SEC_CTRL_RAM0_SLAVE_RULE {
        SEC_CTRL_RAM0_SLAVE_RULE(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_RAM0_SLAVE_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAM0_SLAVE_RULE")
            .field("RAM0_RULE", &self.RAM0_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_RAM0_SLAVE_RULE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_RAM0_SLAVE_RULE {{ RAM0_RULE: {:?} }}",
            self.RAM0_RULE()
        )
    }
}
#[doc = "Security access rules for RAM1 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_RAM1_MEM_RULE0(pub u32);
impl SEC_CTRL_RAM1_MEM_RULE0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE0(&self) -> super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE0(&mut self, val: super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE1(&self) -> super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE1(&mut self, val: super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE2(&self) -> super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE2(&mut self, val: super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE3(&self) -> super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE3(&mut self, val: super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE4(&self) -> super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE4(&mut self, val: super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE5(&self) -> super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE5(&mut self, val: super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE6(&self) -> super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE6(&mut self, val: super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE7(&self) -> super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE7(&mut self, val: super::vals::SEC_CTRL_RAM1_MEM_RULE0_RULE7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_RAM1_MEM_RULE0 {
    #[inline(always)]
    fn default() -> SEC_CTRL_RAM1_MEM_RULE0 {
        SEC_CTRL_RAM1_MEM_RULE0(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_RAM1_MEM_RULE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAM1_MEM_RULE0")
            .field("RULE0", &self.RULE0())
            .field("RULE1", &self.RULE1())
            .field("RULE2", &self.RULE2())
            .field("RULE3", &self.RULE3())
            .field("RULE4", &self.RULE4())
            .field("RULE5", &self.RULE5())
            .field("RULE6", &self.RULE6())
            .field("RULE7", &self.RULE7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_RAM1_MEM_RULE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_RAM1_MEM_RULE0 {{ RULE0: {:?}, RULE1: {:?}, RULE2: {:?}, RULE3: {:?}, RULE4: {:?}, RULE5: {:?}, RULE6: {:?}, RULE7: {:?} }}",
            self.RULE0(),
            self.RULE1(),
            self.RULE2(),
            self.RULE3(),
            self.RULE4(),
            self.RULE5(),
            self.RULE6(),
            self.RULE7()
        )
    }
}
#[doc = "Security access rules for RAM1 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_RAM1_MEM_RULE1(pub u32);
impl SEC_CTRL_RAM1_MEM_RULE1 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE0(&self) -> super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE0(&mut self, val: super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE1(&self) -> super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE1(&mut self, val: super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE2(&self) -> super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE2(&mut self, val: super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE3(&self) -> super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE3(&mut self, val: super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE4(&self) -> super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE4(&mut self, val: super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE5(&self) -> super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE5(&mut self, val: super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE6(&self) -> super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE6(&mut self, val: super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE7(&self) -> super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE7(&mut self, val: super::vals::SEC_CTRL_RAM1_MEM_RULE1_RULE7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_RAM1_MEM_RULE1 {
    #[inline(always)]
    fn default() -> SEC_CTRL_RAM1_MEM_RULE1 {
        SEC_CTRL_RAM1_MEM_RULE1(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_RAM1_MEM_RULE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAM1_MEM_RULE1")
            .field("RULE0", &self.RULE0())
            .field("RULE1", &self.RULE1())
            .field("RULE2", &self.RULE2())
            .field("RULE3", &self.RULE3())
            .field("RULE4", &self.RULE4())
            .field("RULE5", &self.RULE5())
            .field("RULE6", &self.RULE6())
            .field("RULE7", &self.RULE7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_RAM1_MEM_RULE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_RAM1_MEM_RULE1 {{ RULE0: {:?}, RULE1: {:?}, RULE2: {:?}, RULE3: {:?}, RULE4: {:?}, RULE5: {:?}, RULE6: {:?}, RULE7: {:?} }}",
            self.RULE0(),
            self.RULE1(),
            self.RULE2(),
            self.RULE3(),
            self.RULE4(),
            self.RULE5(),
            self.RULE6(),
            self.RULE7()
        )
    }
}
#[doc = "Security access rules for RAM1 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_RAM1_SLAVE_RULE(pub u32);
impl SEC_CTRL_RAM1_SLAVE_RULE {
    #[doc = "Security access rules for the whole RAM1 : 0x2001_0000 - 0x2001_FFFF\" name=\"0."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM1_RULE(&self) -> super::vals::RAM1_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RAM1_RULE::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole RAM1 : 0x2001_0000 - 0x2001_FFFF\" name=\"0."]
    #[inline(always)]
    pub const fn set_RAM1_RULE(&mut self, val: super::vals::RAM1_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SEC_CTRL_RAM1_SLAVE_RULE {
    #[inline(always)]
    fn default() -> SEC_CTRL_RAM1_SLAVE_RULE {
        SEC_CTRL_RAM1_SLAVE_RULE(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_RAM1_SLAVE_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAM1_SLAVE_RULE")
            .field("RAM1_RULE", &self.RAM1_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_RAM1_SLAVE_RULE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_RAM1_SLAVE_RULE {{ RAM1_RULE: {:?} }}",
            self.RAM1_RULE()
        )
    }
}
#[doc = "Security access rules for RAM2 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_RAM2_MEM_RULE0(pub u32);
impl SEC_CTRL_RAM2_MEM_RULE0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE0(&self) -> super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE0(&mut self, val: super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE1(&self) -> super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE1(&mut self, val: super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE2(&self) -> super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE2(&mut self, val: super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE3(&self) -> super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE3(&mut self, val: super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE4(&self) -> super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE4(&mut self, val: super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE5(&self) -> super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE5(&mut self, val: super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE6(&self) -> super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE6(&mut self, val: super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE7(&self) -> super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE7(&mut self, val: super::vals::SEC_CTRL_RAM2_MEM_RULE0_RULE7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_RAM2_MEM_RULE0 {
    #[inline(always)]
    fn default() -> SEC_CTRL_RAM2_MEM_RULE0 {
        SEC_CTRL_RAM2_MEM_RULE0(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_RAM2_MEM_RULE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAM2_MEM_RULE0")
            .field("RULE0", &self.RULE0())
            .field("RULE1", &self.RULE1())
            .field("RULE2", &self.RULE2())
            .field("RULE3", &self.RULE3())
            .field("RULE4", &self.RULE4())
            .field("RULE5", &self.RULE5())
            .field("RULE6", &self.RULE6())
            .field("RULE7", &self.RULE7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_RAM2_MEM_RULE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_RAM2_MEM_RULE0 {{ RULE0: {:?}, RULE1: {:?}, RULE2: {:?}, RULE3: {:?}, RULE4: {:?}, RULE5: {:?}, RULE6: {:?}, RULE7: {:?} }}",
            self.RULE0(),
            self.RULE1(),
            self.RULE2(),
            self.RULE3(),
            self.RULE4(),
            self.RULE5(),
            self.RULE6(),
            self.RULE7()
        )
    }
}
#[doc = "Security access rules for RAM2 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_RAM2_MEM_RULE1(pub u32);
impl SEC_CTRL_RAM2_MEM_RULE1 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE0(&self) -> super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE0(&mut self, val: super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE1(&self) -> super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE1(&mut self, val: super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE2(&self) -> super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE2(&mut self, val: super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE3(&self) -> super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE3(&mut self, val: super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE4(&self) -> super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE4(&mut self, val: super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE5(&self) -> super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE5(&mut self, val: super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE6(&self) -> super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE6(&mut self, val: super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE7(&self) -> super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE7(&mut self, val: super::vals::SEC_CTRL_RAM2_MEM_RULE1_RULE7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_RAM2_MEM_RULE1 {
    #[inline(always)]
    fn default() -> SEC_CTRL_RAM2_MEM_RULE1 {
        SEC_CTRL_RAM2_MEM_RULE1(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_RAM2_MEM_RULE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAM2_MEM_RULE1")
            .field("RULE0", &self.RULE0())
            .field("RULE1", &self.RULE1())
            .field("RULE2", &self.RULE2())
            .field("RULE3", &self.RULE3())
            .field("RULE4", &self.RULE4())
            .field("RULE5", &self.RULE5())
            .field("RULE6", &self.RULE6())
            .field("RULE7", &self.RULE7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_RAM2_MEM_RULE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_RAM2_MEM_RULE1 {{ RULE0: {:?}, RULE1: {:?}, RULE2: {:?}, RULE3: {:?}, RULE4: {:?}, RULE5: {:?}, RULE6: {:?}, RULE7: {:?} }}",
            self.RULE0(),
            self.RULE1(),
            self.RULE2(),
            self.RULE3(),
            self.RULE4(),
            self.RULE5(),
            self.RULE6(),
            self.RULE7()
        )
    }
}
#[doc = "Security access rules for RAM2 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_RAM2_SLAVE_RULE(pub u32);
impl SEC_CTRL_RAM2_SLAVE_RULE {
    #[doc = "Security access rules for the whole RAM2 : 0x2002_0000 - 0x2002_FFFF."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM2_RULE(&self) -> super::vals::RAM2_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RAM2_RULE::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole RAM2 : 0x2002_0000 - 0x2002_FFFF."]
    #[inline(always)]
    pub const fn set_RAM2_RULE(&mut self, val: super::vals::RAM2_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SEC_CTRL_RAM2_SLAVE_RULE {
    #[inline(always)]
    fn default() -> SEC_CTRL_RAM2_SLAVE_RULE {
        SEC_CTRL_RAM2_SLAVE_RULE(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_RAM2_SLAVE_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAM2_SLAVE_RULE")
            .field("RAM2_RULE", &self.RAM2_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_RAM2_SLAVE_RULE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_RAM2_SLAVE_RULE {{ RAM2_RULE: {:?} }}",
            self.RAM2_RULE()
        )
    }
}
#[doc = "Security access rules for RAM3 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_RAM3_MEM_RULE0(pub u32);
impl SEC_CTRL_RAM3_MEM_RULE0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE0(&self) -> super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE0(&mut self, val: super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE1(&self) -> super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE1(&mut self, val: super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE2(&self) -> super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE2(&mut self, val: super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE3(&self) -> super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE3(&mut self, val: super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE4(&self) -> super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE4(&mut self, val: super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE5(&self) -> super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE5(&mut self, val: super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE6(&self) -> super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE6(&mut self, val: super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE7(&self) -> super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE7(&mut self, val: super::vals::SEC_CTRL_RAM3_MEM_RULE0_RULE7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_RAM3_MEM_RULE0 {
    #[inline(always)]
    fn default() -> SEC_CTRL_RAM3_MEM_RULE0 {
        SEC_CTRL_RAM3_MEM_RULE0(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_RAM3_MEM_RULE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAM3_MEM_RULE0")
            .field("RULE0", &self.RULE0())
            .field("RULE1", &self.RULE1())
            .field("RULE2", &self.RULE2())
            .field("RULE3", &self.RULE3())
            .field("RULE4", &self.RULE4())
            .field("RULE5", &self.RULE5())
            .field("RULE6", &self.RULE6())
            .field("RULE7", &self.RULE7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_RAM3_MEM_RULE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_RAM3_MEM_RULE0 {{ RULE0: {:?}, RULE1: {:?}, RULE2: {:?}, RULE3: {:?}, RULE4: {:?}, RULE5: {:?}, RULE6: {:?}, RULE7: {:?} }}",
            self.RULE0(),
            self.RULE1(),
            self.RULE2(),
            self.RULE3(),
            self.RULE4(),
            self.RULE5(),
            self.RULE6(),
            self.RULE7()
        )
    }
}
#[doc = "Security access rules for RAM3 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_RAM3_MEM_RULE1(pub u32);
impl SEC_CTRL_RAM3_MEM_RULE1 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE0(&self) -> super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE0(&mut self, val: super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE1(&self) -> super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE1(&mut self, val: super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE2(&self) -> super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE2(&mut self, val: super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE3(&self) -> super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE3(&mut self, val: super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE4(&self) -> super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE4(&mut self, val: super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE5(&self) -> super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE5(&mut self, val: super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE6(&self) -> super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE6(&mut self, val: super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE7(&self) -> super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE7(&mut self, val: super::vals::SEC_CTRL_RAM3_MEM_RULE1_RULE7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_RAM3_MEM_RULE1 {
    #[inline(always)]
    fn default() -> SEC_CTRL_RAM3_MEM_RULE1 {
        SEC_CTRL_RAM3_MEM_RULE1(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_RAM3_MEM_RULE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAM3_MEM_RULE1")
            .field("RULE0", &self.RULE0())
            .field("RULE1", &self.RULE1())
            .field("RULE2", &self.RULE2())
            .field("RULE3", &self.RULE3())
            .field("RULE4", &self.RULE4())
            .field("RULE5", &self.RULE5())
            .field("RULE6", &self.RULE6())
            .field("RULE7", &self.RULE7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_RAM3_MEM_RULE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_RAM3_MEM_RULE1 {{ RULE0: {:?}, RULE1: {:?}, RULE2: {:?}, RULE3: {:?}, RULE4: {:?}, RULE5: {:?}, RULE6: {:?}, RULE7: {:?} }}",
            self.RULE0(),
            self.RULE1(),
            self.RULE2(),
            self.RULE3(),
            self.RULE4(),
            self.RULE5(),
            self.RULE6(),
            self.RULE7()
        )
    }
}
#[doc = "Security access rules for RAM3 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_RAM3_SLAVE_RULE(pub u32);
impl SEC_CTRL_RAM3_SLAVE_RULE {
    #[doc = "Security access rules for the whole RAM3: 0x2003_0000 - 0x2003_FFFF."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM3_RULE(&self) -> super::vals::RAM3_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RAM3_RULE::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole RAM3: 0x2003_0000 - 0x2003_FFFF."]
    #[inline(always)]
    pub const fn set_RAM3_RULE(&mut self, val: super::vals::RAM3_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SEC_CTRL_RAM3_SLAVE_RULE {
    #[inline(always)]
    fn default() -> SEC_CTRL_RAM3_SLAVE_RULE {
        SEC_CTRL_RAM3_SLAVE_RULE(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_RAM3_SLAVE_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAM3_SLAVE_RULE")
            .field("RAM3_RULE", &self.RAM3_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_RAM3_SLAVE_RULE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_RAM3_SLAVE_RULE {{ RAM3_RULE: {:?} }}",
            self.RAM3_RULE()
        )
    }
}
#[doc = "Security access rules for RAM4 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_RAM4_MEM_RULE0(pub u32);
impl SEC_CTRL_RAM4_MEM_RULE0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE0(&self) -> super::vals::SEC_CTRL_RAM4_MEM_RULE0_RULE0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_CTRL_RAM4_MEM_RULE0_RULE0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE0(&mut self, val: super::vals::SEC_CTRL_RAM4_MEM_RULE0_RULE0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE1(&self) -> super::vals::SEC_CTRL_RAM4_MEM_RULE0_RULE1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SEC_CTRL_RAM4_MEM_RULE0_RULE1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE1(&mut self, val: super::vals::SEC_CTRL_RAM4_MEM_RULE0_RULE1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE2(&self) -> super::vals::SEC_CTRL_RAM4_MEM_RULE0_RULE2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CTRL_RAM4_MEM_RULE0_RULE2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE2(&mut self, val: super::vals::SEC_CTRL_RAM4_MEM_RULE0_RULE2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE3(&self) -> super::vals::SEC_CTRL_RAM4_MEM_RULE0_RULE3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SEC_CTRL_RAM4_MEM_RULE0_RULE3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE3(&mut self, val: super::vals::SEC_CTRL_RAM4_MEM_RULE0_RULE3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for SEC_CTRL_RAM4_MEM_RULE0 {
    #[inline(always)]
    fn default() -> SEC_CTRL_RAM4_MEM_RULE0 {
        SEC_CTRL_RAM4_MEM_RULE0(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_RAM4_MEM_RULE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAM4_MEM_RULE0")
            .field("RULE0", &self.RULE0())
            .field("RULE1", &self.RULE1())
            .field("RULE2", &self.RULE2())
            .field("RULE3", &self.RULE3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_RAM4_MEM_RULE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_RAM4_MEM_RULE0 {{ RULE0: {:?}, RULE1: {:?}, RULE2: {:?}, RULE3: {:?} }}",
            self.RULE0(),
            self.RULE1(),
            self.RULE2(),
            self.RULE3()
        )
    }
}
#[doc = "Security access rules for RAM4 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_RAM4_SLAVE_RULE(pub u32);
impl SEC_CTRL_RAM4_SLAVE_RULE {
    #[doc = "Security access rules for the whole RAM4 : 0x2004_0000 - 0x2004_3FFF."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM4_RULE(&self) -> super::vals::RAM4_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RAM4_RULE::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole RAM4 : 0x2004_0000 - 0x2004_3FFF."]
    #[inline(always)]
    pub const fn set_RAM4_RULE(&mut self, val: super::vals::RAM4_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SEC_CTRL_RAM4_SLAVE_RULE {
    #[inline(always)]
    fn default() -> SEC_CTRL_RAM4_SLAVE_RULE {
        SEC_CTRL_RAM4_SLAVE_RULE(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_RAM4_SLAVE_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAM4_SLAVE_RULE")
            .field("RAM4_RULE", &self.RAM4_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_RAM4_SLAVE_RULE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_RAM4_SLAVE_RULE {{ RAM4_RULE: {:?} }}",
            self.RAM4_RULE()
        )
    }
}
#[doc = "Security access rules for RAMX slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_RAMX_MEM_RULE0(pub u32);
impl SEC_CTRL_RAMX_MEM_RULE0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE0(&self) -> super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE0(&mut self, val: super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE1(&self) -> super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE1(&mut self, val: super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE2(&self) -> super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE2(&mut self, val: super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE3(&self) -> super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE3(&mut self, val: super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE4(&self) -> super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE4(&mut self, val: super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE5(&self) -> super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE5(&mut self, val: super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE6(&self) -> super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE6(&mut self, val: super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE7(&self) -> super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE7(&mut self, val: super::vals::SEC_CTRL_RAMX_MEM_RULE0_RULE7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_RAMX_MEM_RULE0 {
    #[inline(always)]
    fn default() -> SEC_CTRL_RAMX_MEM_RULE0 {
        SEC_CTRL_RAMX_MEM_RULE0(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_RAMX_MEM_RULE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAMX_MEM_RULE0")
            .field("RULE0", &self.RULE0())
            .field("RULE1", &self.RULE1())
            .field("RULE2", &self.RULE2())
            .field("RULE3", &self.RULE3())
            .field("RULE4", &self.RULE4())
            .field("RULE5", &self.RULE5())
            .field("RULE6", &self.RULE6())
            .field("RULE7", &self.RULE7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_RAMX_MEM_RULE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_RAMX_MEM_RULE0 {{ RULE0: {:?}, RULE1: {:?}, RULE2: {:?}, RULE3: {:?}, RULE4: {:?}, RULE5: {:?}, RULE6: {:?}, RULE7: {:?} }}",
            self.RULE0(),
            self.RULE1(),
            self.RULE2(),
            self.RULE3(),
            self.RULE4(),
            self.RULE5(),
            self.RULE6(),
            self.RULE7()
        )
    }
}
#[doc = "Security access rules for RAMX slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_RAMX_SLAVE_RULE(pub u32);
impl SEC_CTRL_RAMX_SLAVE_RULE {
    #[doc = "Security access rules for the whole RAMX : 0x0400_0000 - 0x0400_7FFF."]
    #[must_use]
    #[inline(always)]
    pub const fn RAMX_RULE(&self) -> super::vals::RAMX_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RAMX_RULE::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole RAMX : 0x0400_0000 - 0x0400_7FFF."]
    #[inline(always)]
    pub const fn set_RAMX_RULE(&mut self, val: super::vals::RAMX_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SEC_CTRL_RAMX_SLAVE_RULE {
    #[inline(always)]
    fn default() -> SEC_CTRL_RAMX_SLAVE_RULE {
        SEC_CTRL_RAMX_SLAVE_RULE(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_RAMX_SLAVE_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAMX_SLAVE_RULE")
            .field("RAMX_RULE", &self.RAMX_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_RAMX_SLAVE_RULE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_RAMX_SLAVE_RULE {{ RAMX_RULE: {:?} }}",
            self.RAMX_RULE()
        )
    }
}
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_ROM_MEM_RULE0(pub u32);
impl SEC_CTRL_ROM_MEM_RULE0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE0(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE0(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE1(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE1(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE2(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE2(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE3(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE3(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE4(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE4(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE5(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE5(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE6(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE6(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE7(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE7(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE0_RULE7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_ROM_MEM_RULE0 {
    #[inline(always)]
    fn default() -> SEC_CTRL_ROM_MEM_RULE0 {
        SEC_CTRL_ROM_MEM_RULE0(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_ROM_MEM_RULE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_ROM_MEM_RULE0")
            .field("RULE0", &self.RULE0())
            .field("RULE1", &self.RULE1())
            .field("RULE2", &self.RULE2())
            .field("RULE3", &self.RULE3())
            .field("RULE4", &self.RULE4())
            .field("RULE5", &self.RULE5())
            .field("RULE6", &self.RULE6())
            .field("RULE7", &self.RULE7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_ROM_MEM_RULE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_ROM_MEM_RULE0 {{ RULE0: {:?}, RULE1: {:?}, RULE2: {:?}, RULE3: {:?}, RULE4: {:?}, RULE5: {:?}, RULE6: {:?}, RULE7: {:?} }}",
            self.RULE0(),
            self.RULE1(),
            self.RULE2(),
            self.RULE3(),
            self.RULE4(),
            self.RULE5(),
            self.RULE6(),
            self.RULE7()
        )
    }
}
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_ROM_MEM_RULE1(pub u32);
impl SEC_CTRL_ROM_MEM_RULE1 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE0(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE0(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE1(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE1(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE2(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE2(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE3(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE3(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE4(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE4(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE5(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE5(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE6(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE6(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE7(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE7(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE1_RULE7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_ROM_MEM_RULE1 {
    #[inline(always)]
    fn default() -> SEC_CTRL_ROM_MEM_RULE1 {
        SEC_CTRL_ROM_MEM_RULE1(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_ROM_MEM_RULE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_ROM_MEM_RULE1")
            .field("RULE0", &self.RULE0())
            .field("RULE1", &self.RULE1())
            .field("RULE2", &self.RULE2())
            .field("RULE3", &self.RULE3())
            .field("RULE4", &self.RULE4())
            .field("RULE5", &self.RULE5())
            .field("RULE6", &self.RULE6())
            .field("RULE7", &self.RULE7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_ROM_MEM_RULE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_ROM_MEM_RULE1 {{ RULE0: {:?}, RULE1: {:?}, RULE2: {:?}, RULE3: {:?}, RULE4: {:?}, RULE5: {:?}, RULE6: {:?}, RULE7: {:?} }}",
            self.RULE0(),
            self.RULE1(),
            self.RULE2(),
            self.RULE3(),
            self.RULE4(),
            self.RULE5(),
            self.RULE6(),
            self.RULE7()
        )
    }
}
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_ROM_MEM_RULE2(pub u32);
impl SEC_CTRL_ROM_MEM_RULE2 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE0(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE0(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE1(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE1(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE2(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE2(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE3(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE3(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE4(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE4(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE5(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE5(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE6(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE6(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE7(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE7(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE2_RULE7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_ROM_MEM_RULE2 {
    #[inline(always)]
    fn default() -> SEC_CTRL_ROM_MEM_RULE2 {
        SEC_CTRL_ROM_MEM_RULE2(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_ROM_MEM_RULE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_ROM_MEM_RULE2")
            .field("RULE0", &self.RULE0())
            .field("RULE1", &self.RULE1())
            .field("RULE2", &self.RULE2())
            .field("RULE3", &self.RULE3())
            .field("RULE4", &self.RULE4())
            .field("RULE5", &self.RULE5())
            .field("RULE6", &self.RULE6())
            .field("RULE7", &self.RULE7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_ROM_MEM_RULE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_ROM_MEM_RULE2 {{ RULE0: {:?}, RULE1: {:?}, RULE2: {:?}, RULE3: {:?}, RULE4: {:?}, RULE5: {:?}, RULE6: {:?}, RULE7: {:?} }}",
            self.RULE0(),
            self.RULE1(),
            self.RULE2(),
            self.RULE3(),
            self.RULE4(),
            self.RULE5(),
            self.RULE6(),
            self.RULE7()
        )
    }
}
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_ROM_MEM_RULE3(pub u32);
impl SEC_CTRL_ROM_MEM_RULE3 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE0(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE0(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE1(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE1(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE2(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE2(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE3(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE3(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE4(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE4(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE5(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE5(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE6(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE6(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn RULE7(&self) -> super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'."]
    #[inline(always)]
    pub const fn set_RULE7(&mut self, val: super::vals::SEC_CTRL_ROM_MEM_RULE3_RULE7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SEC_CTRL_ROM_MEM_RULE3 {
    #[inline(always)]
    fn default() -> SEC_CTRL_ROM_MEM_RULE3 {
        SEC_CTRL_ROM_MEM_RULE3(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_ROM_MEM_RULE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_ROM_MEM_RULE3")
            .field("RULE0", &self.RULE0())
            .field("RULE1", &self.RULE1())
            .field("RULE2", &self.RULE2())
            .field("RULE3", &self.RULE3())
            .field("RULE4", &self.RULE4())
            .field("RULE5", &self.RULE5())
            .field("RULE6", &self.RULE6())
            .field("RULE7", &self.RULE7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_ROM_MEM_RULE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_ROM_MEM_RULE3 {{ RULE0: {:?}, RULE1: {:?}, RULE2: {:?}, RULE3: {:?}, RULE4: {:?}, RULE5: {:?}, RULE6: {:?}, RULE7: {:?} }}",
            self.RULE0(),
            self.RULE1(),
            self.RULE2(),
            self.RULE3(),
            self.RULE4(),
            self.RULE5(),
            self.RULE6(),
            self.RULE7()
        )
    }
}
#[doc = "Security access rules for RAM_USB_HS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_USB_HS_MEM_RULE(pub u32);
impl SEC_CTRL_USB_HS_MEM_RULE {
    #[doc = "Address space: 0x4010_0000 - 0x4010_0FFF."]
    #[must_use]
    #[inline(always)]
    pub const fn SRAM_SECT_0_RULE(&self) -> super::vals::SRAM_SECT_0_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SRAM_SECT_0_RULE::from_bits(val as u8)
    }
    #[doc = "Address space: 0x4010_0000 - 0x4010_0FFF."]
    #[inline(always)]
    pub const fn set_SRAM_SECT_0_RULE(&mut self, val: super::vals::SRAM_SECT_0_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Address space: 0x4010_1000 - 0x4010_1FFF."]
    #[must_use]
    #[inline(always)]
    pub const fn SRAM_SECT_1_RULE(&self) -> super::vals::SRAM_SECT_1_RULE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SRAM_SECT_1_RULE::from_bits(val as u8)
    }
    #[doc = "Address space: 0x4010_1000 - 0x4010_1FFF."]
    #[inline(always)]
    pub const fn set_SRAM_SECT_1_RULE(&mut self, val: super::vals::SRAM_SECT_1_RULE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Address space: 0x4010_2000 - 0x4010_2FFF."]
    #[must_use]
    #[inline(always)]
    pub const fn SRAM_SECT_2_RULE(&self) -> super::vals::SRAM_SECT_2_RULE {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SRAM_SECT_2_RULE::from_bits(val as u8)
    }
    #[doc = "Address space: 0x4010_2000 - 0x4010_2FFF."]
    #[inline(always)]
    pub const fn set_SRAM_SECT_2_RULE(&mut self, val: super::vals::SRAM_SECT_2_RULE) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Address space: 0x4010_3000 - 0x4010_3FFF."]
    #[must_use]
    #[inline(always)]
    pub const fn SRAM_SECT_3_RULE(&self) -> super::vals::SRAM_SECT_3_RULE {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SRAM_SECT_3_RULE::from_bits(val as u8)
    }
    #[doc = "Address space: 0x4010_3000 - 0x4010_3FFF."]
    #[inline(always)]
    pub const fn set_SRAM_SECT_3_RULE(&mut self, val: super::vals::SRAM_SECT_3_RULE) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for SEC_CTRL_USB_HS_MEM_RULE {
    #[inline(always)]
    fn default() -> SEC_CTRL_USB_HS_MEM_RULE {
        SEC_CTRL_USB_HS_MEM_RULE(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_USB_HS_MEM_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_USB_HS_MEM_RULE")
            .field("SRAM_SECT_0_RULE", &self.SRAM_SECT_0_RULE())
            .field("SRAM_SECT_1_RULE", &self.SRAM_SECT_1_RULE())
            .field("SRAM_SECT_2_RULE", &self.SRAM_SECT_2_RULE())
            .field("SRAM_SECT_3_RULE", &self.SRAM_SECT_3_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_USB_HS_MEM_RULE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_USB_HS_MEM_RULE {{ SRAM_SECT_0_RULE: {:?}, SRAM_SECT_1_RULE: {:?}, SRAM_SECT_2_RULE: {:?}, SRAM_SECT_3_RULE: {:?} }}",
            self.SRAM_SECT_0_RULE(),
            self.SRAM_SECT_1_RULE(),
            self.SRAM_SECT_2_RULE(),
            self.SRAM_SECT_3_RULE()
        )
    }
}
#[doc = "Security access rules for USB High speed RAM slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_CTRL_USB_HS_SLAVE_RULE(pub u32);
impl SEC_CTRL_USB_HS_SLAVE_RULE {
    #[doc = "Security access rules for the whole USB High Speed RAM : 0x4010_0000 - 0x4010_3FFF."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_USB_HS_RULE(&self) -> super::vals::RAM_USB_HS_RULE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RAM_USB_HS_RULE::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole USB High Speed RAM : 0x4010_0000 - 0x4010_3FFF."]
    #[inline(always)]
    pub const fn set_RAM_USB_HS_RULE(&mut self, val: super::vals::RAM_USB_HS_RULE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SEC_CTRL_USB_HS_SLAVE_RULE {
    #[inline(always)]
    fn default() -> SEC_CTRL_USB_HS_SLAVE_RULE {
        SEC_CTRL_USB_HS_SLAVE_RULE(0)
    }
}
impl core::fmt::Debug for SEC_CTRL_USB_HS_SLAVE_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_USB_HS_SLAVE_RULE")
            .field("RAM_USB_HS_RULE", &self.RAM_USB_HS_RULE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CTRL_USB_HS_SLAVE_RULE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_CTRL_USB_HS_SLAVE_RULE {{ RAM_USB_HS_RULE: {:?} }}",
            self.RAM_USB_HS_RULE()
        )
    }
}
#[doc = "Secure GPIO mask for port 0 pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_GPIO_MASK0(pub u32);
impl SEC_GPIO_MASK0 {
    #[doc = "Secure mask for pin P0_0."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN0_SEC_MASK(&self) -> super::vals::PIO0_PIN0_SEC_MASK {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PIO0_PIN0_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_0."]
    #[inline(always)]
    pub const fn set_PIO0_PIN0_SEC_MASK(&mut self, val: super::vals::PIO0_PIN0_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure mask for pin P0_1."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN1_SEC_MASK(&self) -> super::vals::PIO0_PIN1_SEC_MASK {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PIO0_PIN1_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_1."]
    #[inline(always)]
    pub const fn set_PIO0_PIN1_SEC_MASK(&mut self, val: super::vals::PIO0_PIN1_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Secure mask for pin P0_2."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN2_SEC_MASK(&self) -> super::vals::PIO0_PIN2_SEC_MASK {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PIO0_PIN2_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_2."]
    #[inline(always)]
    pub const fn set_PIO0_PIN2_SEC_MASK(&mut self, val: super::vals::PIO0_PIN2_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Secure mask for pin P0_3."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN3_SEC_MASK(&self) -> super::vals::PIO0_PIN3_SEC_MASK {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PIO0_PIN3_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_3."]
    #[inline(always)]
    pub const fn set_PIO0_PIN3_SEC_MASK(&mut self, val: super::vals::PIO0_PIN3_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Secure mask for pin P0_4."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN4_SEC_MASK(&self) -> super::vals::PIO0_PIN4_SEC_MASK {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::PIO0_PIN4_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_4."]
    #[inline(always)]
    pub const fn set_PIO0_PIN4_SEC_MASK(&mut self, val: super::vals::PIO0_PIN4_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Secure mask for pin P0_5."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN5_SEC_MASK(&self) -> super::vals::PIO0_PIN5_SEC_MASK {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::PIO0_PIN5_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_5."]
    #[inline(always)]
    pub const fn set_PIO0_PIN5_SEC_MASK(&mut self, val: super::vals::PIO0_PIN5_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Secure mask for pin P0_6."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN6_SEC_MASK(&self) -> super::vals::PIO0_PIN6_SEC_MASK {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_PIN6_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_6."]
    #[inline(always)]
    pub const fn set_PIO0_PIN6_SEC_MASK(&mut self, val: super::vals::PIO0_PIN6_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Secure mask for pin P0_7."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN7_SEC_MASK(&self) -> super::vals::PIO0_PIN7_SEC_MASK {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::PIO0_PIN7_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_7."]
    #[inline(always)]
    pub const fn set_PIO0_PIN7_SEC_MASK(&mut self, val: super::vals::PIO0_PIN7_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Secure mask for pin P0_8."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN8_SEC_MASK(&self) -> super::vals::PIO0_PIN8_SEC_MASK {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_PIN8_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_8."]
    #[inline(always)]
    pub const fn set_PIO0_PIN8_SEC_MASK(&mut self, val: super::vals::PIO0_PIN8_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Secure mask for pin P0_9."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN9_SEC_MASK(&self) -> super::vals::PIO0_PIN9_SEC_MASK {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_PIN9_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_9."]
    #[inline(always)]
    pub const fn set_PIO0_PIN9_SEC_MASK(&mut self, val: super::vals::PIO0_PIN9_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Secure mask for pin P0_10."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN10_SEC_MASK(&self) -> super::vals::PIO0_PIN10_SEC_MASK {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PIO0_PIN10_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_10."]
    #[inline(always)]
    pub const fn set_PIO0_PIN10_SEC_MASK(&mut self, val: super::vals::PIO0_PIN10_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Secure mask for pin P0_11."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN11_SEC_MASK(&self) -> super::vals::PIO0_PIN11_SEC_MASK {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PIO0_PIN11_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_11."]
    #[inline(always)]
    pub const fn set_PIO0_PIN11_SEC_MASK(&mut self, val: super::vals::PIO0_PIN11_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Secure mask for pin P0_12."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN12_SEC_MASK(&self) -> super::vals::PIO0_PIN12_SEC_MASK {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PIO0_PIN12_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_12."]
    #[inline(always)]
    pub const fn set_PIO0_PIN12_SEC_MASK(&mut self, val: super::vals::PIO0_PIN12_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Secure mask for pin P0_13."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN13_SEC_MASK(&self) -> super::vals::PIO0_PIN13_SEC_MASK {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::PIO0_PIN13_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_13."]
    #[inline(always)]
    pub const fn set_PIO0_PIN13_SEC_MASK(&mut self, val: super::vals::PIO0_PIN13_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Secure mask for pin P0_14."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN14_SEC_MASK(&self) -> super::vals::PIO0_PIN14_SEC_MASK {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::PIO0_PIN14_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_14."]
    #[inline(always)]
    pub const fn set_PIO0_PIN14_SEC_MASK(&mut self, val: super::vals::PIO0_PIN14_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Secure mask for pin P0_15."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN15_SEC_MASK(&self) -> super::vals::PIO0_PIN15_SEC_MASK {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::PIO0_PIN15_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_15."]
    #[inline(always)]
    pub const fn set_PIO0_PIN15_SEC_MASK(&mut self, val: super::vals::PIO0_PIN15_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Secure mask for pin P0_16."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN16_SEC_MASK(&self) -> super::vals::PIO0_PIN16_SEC_MASK {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::PIO0_PIN16_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_16."]
    #[inline(always)]
    pub const fn set_PIO0_PIN16_SEC_MASK(&mut self, val: super::vals::PIO0_PIN16_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Secure mask for pin P0_17."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN17_SEC_MASK(&self) -> super::vals::PIO0_PIN17_SEC_MASK {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::PIO0_PIN17_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_17."]
    #[inline(always)]
    pub const fn set_PIO0_PIN17_SEC_MASK(&mut self, val: super::vals::PIO0_PIN17_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Secure mask for pin P0_18."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN18_SEC_MASK(&self) -> super::vals::PIO0_PIN18_SEC_MASK {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PIO0_PIN18_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_18."]
    #[inline(always)]
    pub const fn set_PIO0_PIN18_SEC_MASK(&mut self, val: super::vals::PIO0_PIN18_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Secure mask for pin P0_19."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN19_SEC_MASK(&self) -> super::vals::PIO0_PIN19_SEC_MASK {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PIO0_PIN19_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_19."]
    #[inline(always)]
    pub const fn set_PIO0_PIN19_SEC_MASK(&mut self, val: super::vals::PIO0_PIN19_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Secure mask for pin P0_20."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN20_SEC_MASK(&self) -> super::vals::PIO0_PIN20_SEC_MASK {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PIO0_PIN20_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_20."]
    #[inline(always)]
    pub const fn set_PIO0_PIN20_SEC_MASK(&mut self, val: super::vals::PIO0_PIN20_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Secure mask for pin P0_21."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN21_SEC_MASK(&self) -> super::vals::PIO0_PIN21_SEC_MASK {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::PIO0_PIN21_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_21."]
    #[inline(always)]
    pub const fn set_PIO0_PIN21_SEC_MASK(&mut self, val: super::vals::PIO0_PIN21_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Secure mask for pin P0_22."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN22_SEC_MASK(&self) -> super::vals::PIO0_PIN22_SEC_MASK {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::PIO0_PIN22_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_22."]
    #[inline(always)]
    pub const fn set_PIO0_PIN22_SEC_MASK(&mut self, val: super::vals::PIO0_PIN22_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Secure mask for pin P0_23."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN23_SEC_MASK(&self) -> super::vals::PIO0_PIN23_SEC_MASK {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::PIO0_PIN23_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_23."]
    #[inline(always)]
    pub const fn set_PIO0_PIN23_SEC_MASK(&mut self, val: super::vals::PIO0_PIN23_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Secure mask for pin P0_24."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN24_SEC_MASK(&self) -> super::vals::PIO0_PIN24_SEC_MASK {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::PIO0_PIN24_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_24."]
    #[inline(always)]
    pub const fn set_PIO0_PIN24_SEC_MASK(&mut self, val: super::vals::PIO0_PIN24_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Secure mask for pin P0_25."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN25_SEC_MASK(&self) -> super::vals::PIO0_PIN25_SEC_MASK {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::PIO0_PIN25_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_25."]
    #[inline(always)]
    pub const fn set_PIO0_PIN25_SEC_MASK(&mut self, val: super::vals::PIO0_PIN25_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Secure mask for pin P0_26."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN26_SEC_MASK(&self) -> super::vals::PIO0_PIN26_SEC_MASK {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::PIO0_PIN26_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_26."]
    #[inline(always)]
    pub const fn set_PIO0_PIN26_SEC_MASK(&mut self, val: super::vals::PIO0_PIN26_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Secure mask for pin P0_27."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN27_SEC_MASK(&self) -> super::vals::PIO0_PIN27_SEC_MASK {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::PIO0_PIN27_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_27."]
    #[inline(always)]
    pub const fn set_PIO0_PIN27_SEC_MASK(&mut self, val: super::vals::PIO0_PIN27_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Secure mask for pin P0_28."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN28_SEC_MASK(&self) -> super::vals::PIO0_PIN28_SEC_MASK {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::PIO0_PIN28_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_28."]
    #[inline(always)]
    pub const fn set_PIO0_PIN28_SEC_MASK(&mut self, val: super::vals::PIO0_PIN28_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Secure mask for pin P0_29."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN29_SEC_MASK(&self) -> super::vals::PIO0_PIN29_SEC_MASK {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::PIO0_PIN29_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_29."]
    #[inline(always)]
    pub const fn set_PIO0_PIN29_SEC_MASK(&mut self, val: super::vals::PIO0_PIN29_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Secure mask for pin P0_30."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN30_SEC_MASK(&self) -> super::vals::PIO0_PIN30_SEC_MASK {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::PIO0_PIN30_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_30."]
    #[inline(always)]
    pub const fn set_PIO0_PIN30_SEC_MASK(&mut self, val: super::vals::PIO0_PIN30_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Secure mask for pin P0_31."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO0_PIN31_SEC_MASK(&self) -> super::vals::PIO0_PIN31_SEC_MASK {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PIO0_PIN31_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_31."]
    #[inline(always)]
    pub const fn set_PIO0_PIN31_SEC_MASK(&mut self, val: super::vals::PIO0_PIN31_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SEC_GPIO_MASK0 {
    #[inline(always)]
    fn default() -> SEC_GPIO_MASK0 {
        SEC_GPIO_MASK0(0)
    }
}
impl core::fmt::Debug for SEC_GPIO_MASK0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_GPIO_MASK0")
            .field("PIO0_PIN0_SEC_MASK", &self.PIO0_PIN0_SEC_MASK())
            .field("PIO0_PIN1_SEC_MASK", &self.PIO0_PIN1_SEC_MASK())
            .field("PIO0_PIN2_SEC_MASK", &self.PIO0_PIN2_SEC_MASK())
            .field("PIO0_PIN3_SEC_MASK", &self.PIO0_PIN3_SEC_MASK())
            .field("PIO0_PIN4_SEC_MASK", &self.PIO0_PIN4_SEC_MASK())
            .field("PIO0_PIN5_SEC_MASK", &self.PIO0_PIN5_SEC_MASK())
            .field("PIO0_PIN6_SEC_MASK", &self.PIO0_PIN6_SEC_MASK())
            .field("PIO0_PIN7_SEC_MASK", &self.PIO0_PIN7_SEC_MASK())
            .field("PIO0_PIN8_SEC_MASK", &self.PIO0_PIN8_SEC_MASK())
            .field("PIO0_PIN9_SEC_MASK", &self.PIO0_PIN9_SEC_MASK())
            .field("PIO0_PIN10_SEC_MASK", &self.PIO0_PIN10_SEC_MASK())
            .field("PIO0_PIN11_SEC_MASK", &self.PIO0_PIN11_SEC_MASK())
            .field("PIO0_PIN12_SEC_MASK", &self.PIO0_PIN12_SEC_MASK())
            .field("PIO0_PIN13_SEC_MASK", &self.PIO0_PIN13_SEC_MASK())
            .field("PIO0_PIN14_SEC_MASK", &self.PIO0_PIN14_SEC_MASK())
            .field("PIO0_PIN15_SEC_MASK", &self.PIO0_PIN15_SEC_MASK())
            .field("PIO0_PIN16_SEC_MASK", &self.PIO0_PIN16_SEC_MASK())
            .field("PIO0_PIN17_SEC_MASK", &self.PIO0_PIN17_SEC_MASK())
            .field("PIO0_PIN18_SEC_MASK", &self.PIO0_PIN18_SEC_MASK())
            .field("PIO0_PIN19_SEC_MASK", &self.PIO0_PIN19_SEC_MASK())
            .field("PIO0_PIN20_SEC_MASK", &self.PIO0_PIN20_SEC_MASK())
            .field("PIO0_PIN21_SEC_MASK", &self.PIO0_PIN21_SEC_MASK())
            .field("PIO0_PIN22_SEC_MASK", &self.PIO0_PIN22_SEC_MASK())
            .field("PIO0_PIN23_SEC_MASK", &self.PIO0_PIN23_SEC_MASK())
            .field("PIO0_PIN24_SEC_MASK", &self.PIO0_PIN24_SEC_MASK())
            .field("PIO0_PIN25_SEC_MASK", &self.PIO0_PIN25_SEC_MASK())
            .field("PIO0_PIN26_SEC_MASK", &self.PIO0_PIN26_SEC_MASK())
            .field("PIO0_PIN27_SEC_MASK", &self.PIO0_PIN27_SEC_MASK())
            .field("PIO0_PIN28_SEC_MASK", &self.PIO0_PIN28_SEC_MASK())
            .field("PIO0_PIN29_SEC_MASK", &self.PIO0_PIN29_SEC_MASK())
            .field("PIO0_PIN30_SEC_MASK", &self.PIO0_PIN30_SEC_MASK())
            .field("PIO0_PIN31_SEC_MASK", &self.PIO0_PIN31_SEC_MASK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_GPIO_MASK0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_GPIO_MASK0 {{ PIO0_PIN0_SEC_MASK: {:?}, PIO0_PIN1_SEC_MASK: {:?}, PIO0_PIN2_SEC_MASK: {:?}, PIO0_PIN3_SEC_MASK: {:?}, PIO0_PIN4_SEC_MASK: {:?}, PIO0_PIN5_SEC_MASK: {:?}, PIO0_PIN6_SEC_MASK: {:?}, PIO0_PIN7_SEC_MASK: {:?}, PIO0_PIN8_SEC_MASK: {:?}, PIO0_PIN9_SEC_MASK: {:?}, PIO0_PIN10_SEC_MASK: {:?}, PIO0_PIN11_SEC_MASK: {:?}, PIO0_PIN12_SEC_MASK: {:?}, PIO0_PIN13_SEC_MASK: {:?}, PIO0_PIN14_SEC_MASK: {:?}, PIO0_PIN15_SEC_MASK: {:?}, PIO0_PIN16_SEC_MASK: {:?}, PIO0_PIN17_SEC_MASK: {:?}, PIO0_PIN18_SEC_MASK: {:?}, PIO0_PIN19_SEC_MASK: {:?}, PIO0_PIN20_SEC_MASK: {:?}, PIO0_PIN21_SEC_MASK: {:?}, PIO0_PIN22_SEC_MASK: {:?}, PIO0_PIN23_SEC_MASK: {:?}, PIO0_PIN24_SEC_MASK: {:?}, PIO0_PIN25_SEC_MASK: {:?}, PIO0_PIN26_SEC_MASK: {:?}, PIO0_PIN27_SEC_MASK: {:?}, PIO0_PIN28_SEC_MASK: {:?}, PIO0_PIN29_SEC_MASK: {:?}, PIO0_PIN30_SEC_MASK: {:?}, PIO0_PIN31_SEC_MASK: {:?} }}",
            self.PIO0_PIN0_SEC_MASK(),
            self.PIO0_PIN1_SEC_MASK(),
            self.PIO0_PIN2_SEC_MASK(),
            self.PIO0_PIN3_SEC_MASK(),
            self.PIO0_PIN4_SEC_MASK(),
            self.PIO0_PIN5_SEC_MASK(),
            self.PIO0_PIN6_SEC_MASK(),
            self.PIO0_PIN7_SEC_MASK(),
            self.PIO0_PIN8_SEC_MASK(),
            self.PIO0_PIN9_SEC_MASK(),
            self.PIO0_PIN10_SEC_MASK(),
            self.PIO0_PIN11_SEC_MASK(),
            self.PIO0_PIN12_SEC_MASK(),
            self.PIO0_PIN13_SEC_MASK(),
            self.PIO0_PIN14_SEC_MASK(),
            self.PIO0_PIN15_SEC_MASK(),
            self.PIO0_PIN16_SEC_MASK(),
            self.PIO0_PIN17_SEC_MASK(),
            self.PIO0_PIN18_SEC_MASK(),
            self.PIO0_PIN19_SEC_MASK(),
            self.PIO0_PIN20_SEC_MASK(),
            self.PIO0_PIN21_SEC_MASK(),
            self.PIO0_PIN22_SEC_MASK(),
            self.PIO0_PIN23_SEC_MASK(),
            self.PIO0_PIN24_SEC_MASK(),
            self.PIO0_PIN25_SEC_MASK(),
            self.PIO0_PIN26_SEC_MASK(),
            self.PIO0_PIN27_SEC_MASK(),
            self.PIO0_PIN28_SEC_MASK(),
            self.PIO0_PIN29_SEC_MASK(),
            self.PIO0_PIN30_SEC_MASK(),
            self.PIO0_PIN31_SEC_MASK()
        )
    }
}
#[doc = "Secure GPIO mask for port 1 pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_GPIO_MASK1(pub u32);
impl SEC_GPIO_MASK1 {
    #[doc = "Secure mask for pin P1_0."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN0_SEC_MASK(&self) -> super::vals::PIO1_PIN0_SEC_MASK {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PIO1_PIN0_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_0."]
    #[inline(always)]
    pub const fn set_PIO1_PIN0_SEC_MASK(&mut self, val: super::vals::PIO1_PIN0_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure mask for pin P1_1."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN1_SEC_MASK(&self) -> super::vals::PIO1_PIN1_SEC_MASK {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PIO1_PIN1_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_1."]
    #[inline(always)]
    pub const fn set_PIO1_PIN1_SEC_MASK(&mut self, val: super::vals::PIO1_PIN1_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Secure mask for pin P1_2."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN2_SEC_MASK(&self) -> super::vals::PIO1_PIN2_SEC_MASK {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PIO1_PIN2_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_2."]
    #[inline(always)]
    pub const fn set_PIO1_PIN2_SEC_MASK(&mut self, val: super::vals::PIO1_PIN2_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Secure mask for pin P1_3."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN3_SEC_MASK(&self) -> super::vals::PIO1_PIN3_SEC_MASK {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PIO1_PIN3_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_3."]
    #[inline(always)]
    pub const fn set_PIO1_PIN3_SEC_MASK(&mut self, val: super::vals::PIO1_PIN3_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Secure mask for pin P1_4."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN4_SEC_MASK(&self) -> super::vals::PIO1_PIN4_SEC_MASK {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::PIO1_PIN4_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_4."]
    #[inline(always)]
    pub const fn set_PIO1_PIN4_SEC_MASK(&mut self, val: super::vals::PIO1_PIN4_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Secure mask for pin P1_5."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN5_SEC_MASK(&self) -> super::vals::PIO1_PIN5_SEC_MASK {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::PIO1_PIN5_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_5."]
    #[inline(always)]
    pub const fn set_PIO1_PIN5_SEC_MASK(&mut self, val: super::vals::PIO1_PIN5_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Secure mask for pin P1_6."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN6_SEC_MASK(&self) -> super::vals::PIO1_PIN6_SEC_MASK {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_PIN6_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_6."]
    #[inline(always)]
    pub const fn set_PIO1_PIN6_SEC_MASK(&mut self, val: super::vals::PIO1_PIN6_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Secure mask for pin P1_7."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN7_SEC_MASK(&self) -> super::vals::PIO1_PIN7_SEC_MASK {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::PIO1_PIN7_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_7."]
    #[inline(always)]
    pub const fn set_PIO1_PIN7_SEC_MASK(&mut self, val: super::vals::PIO1_PIN7_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Secure mask for pin P1_8."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN8_SEC_MASK(&self) -> super::vals::PIO1_PIN8_SEC_MASK {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_PIN8_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_8."]
    #[inline(always)]
    pub const fn set_PIO1_PIN8_SEC_MASK(&mut self, val: super::vals::PIO1_PIN8_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Secure mask for pin P1_9."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN9_SEC_MASK(&self) -> super::vals::PIO1_PIN9_SEC_MASK {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_PIN9_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_9."]
    #[inline(always)]
    pub const fn set_PIO1_PIN9_SEC_MASK(&mut self, val: super::vals::PIO1_PIN9_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Secure mask for pin P1_10."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN10_SEC_MASK(&self) -> super::vals::PIO1_PIN10_SEC_MASK {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PIO1_PIN10_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_10."]
    #[inline(always)]
    pub const fn set_PIO1_PIN10_SEC_MASK(&mut self, val: super::vals::PIO1_PIN10_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Secure mask for pin P1_11."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN11_SEC_MASK(&self) -> super::vals::PIO1_PIN11_SEC_MASK {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PIO1_PIN11_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_11."]
    #[inline(always)]
    pub const fn set_PIO1_PIN11_SEC_MASK(&mut self, val: super::vals::PIO1_PIN11_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Secure mask for pin P1_12."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN12_SEC_MASK(&self) -> super::vals::PIO1_PIN12_SEC_MASK {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PIO1_PIN12_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_12."]
    #[inline(always)]
    pub const fn set_PIO1_PIN12_SEC_MASK(&mut self, val: super::vals::PIO1_PIN12_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Secure mask for pin P1_13."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN13_SEC_MASK(&self) -> super::vals::PIO1_PIN13_SEC_MASK {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::PIO1_PIN13_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_13."]
    #[inline(always)]
    pub const fn set_PIO1_PIN13_SEC_MASK(&mut self, val: super::vals::PIO1_PIN13_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Secure mask for pin P1_14."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN14_SEC_MASK(&self) -> super::vals::PIO1_PIN14_SEC_MASK {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::PIO1_PIN14_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_14."]
    #[inline(always)]
    pub const fn set_PIO1_PIN14_SEC_MASK(&mut self, val: super::vals::PIO1_PIN14_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Secure mask for pin P1_15."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN15_SEC_MASK(&self) -> super::vals::PIO1_PIN15_SEC_MASK {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::PIO1_PIN15_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_15."]
    #[inline(always)]
    pub const fn set_PIO1_PIN15_SEC_MASK(&mut self, val: super::vals::PIO1_PIN15_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Secure mask for pin P1_16."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN16_SEC_MASK(&self) -> super::vals::PIO1_PIN16_SEC_MASK {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::PIO1_PIN16_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_16."]
    #[inline(always)]
    pub const fn set_PIO1_PIN16_SEC_MASK(&mut self, val: super::vals::PIO1_PIN16_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Secure mask for pin P1_17."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN17_SEC_MASK(&self) -> super::vals::PIO1_PIN17_SEC_MASK {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::PIO1_PIN17_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_17."]
    #[inline(always)]
    pub const fn set_PIO1_PIN17_SEC_MASK(&mut self, val: super::vals::PIO1_PIN17_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Secure mask for pin P1_18."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN18_SEC_MASK(&self) -> super::vals::PIO1_PIN18_SEC_MASK {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PIO1_PIN18_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_18."]
    #[inline(always)]
    pub const fn set_PIO1_PIN18_SEC_MASK(&mut self, val: super::vals::PIO1_PIN18_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Secure mask for pin P1_19."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN19_SEC_MASK(&self) -> super::vals::PIO1_PIN19_SEC_MASK {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PIO1_PIN19_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_19."]
    #[inline(always)]
    pub const fn set_PIO1_PIN19_SEC_MASK(&mut self, val: super::vals::PIO1_PIN19_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Secure mask for pin P1_20."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN20_SEC_MASK(&self) -> super::vals::PIO1_PIN20_SEC_MASK {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PIO1_PIN20_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_20."]
    #[inline(always)]
    pub const fn set_PIO1_PIN20_SEC_MASK(&mut self, val: super::vals::PIO1_PIN20_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Secure mask for pin P1_21."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN21_SEC_MASK(&self) -> super::vals::PIO1_PIN21_SEC_MASK {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::PIO1_PIN21_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_21."]
    #[inline(always)]
    pub const fn set_PIO1_PIN21_SEC_MASK(&mut self, val: super::vals::PIO1_PIN21_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Secure mask for pin P1_22."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN22_SEC_MASK(&self) -> super::vals::PIO1_PIN22_SEC_MASK {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::PIO1_PIN22_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_22."]
    #[inline(always)]
    pub const fn set_PIO1_PIN22_SEC_MASK(&mut self, val: super::vals::PIO1_PIN22_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Secure mask for pin P1_23."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN23_SEC_MASK(&self) -> super::vals::PIO1_PIN23_SEC_MASK {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::PIO1_PIN23_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_23."]
    #[inline(always)]
    pub const fn set_PIO1_PIN23_SEC_MASK(&mut self, val: super::vals::PIO1_PIN23_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Secure mask for pin P1_24."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN24_SEC_MASK(&self) -> super::vals::PIO1_PIN24_SEC_MASK {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::PIO1_PIN24_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_24."]
    #[inline(always)]
    pub const fn set_PIO1_PIN24_SEC_MASK(&mut self, val: super::vals::PIO1_PIN24_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Secure mask for pin P1_25."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN25_SEC_MASK(&self) -> super::vals::PIO1_PIN25_SEC_MASK {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::PIO1_PIN25_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_25."]
    #[inline(always)]
    pub const fn set_PIO1_PIN25_SEC_MASK(&mut self, val: super::vals::PIO1_PIN25_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Secure mask for pin P1_26."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN26_SEC_MASK(&self) -> super::vals::PIO1_PIN26_SEC_MASK {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::PIO1_PIN26_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_26."]
    #[inline(always)]
    pub const fn set_PIO1_PIN26_SEC_MASK(&mut self, val: super::vals::PIO1_PIN26_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Secure mask for pin P1_27."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN27_SEC_MASK(&self) -> super::vals::PIO1_PIN27_SEC_MASK {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::PIO1_PIN27_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_27."]
    #[inline(always)]
    pub const fn set_PIO1_PIN27_SEC_MASK(&mut self, val: super::vals::PIO1_PIN27_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Secure mask for pin P1_28."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN28_SEC_MASK(&self) -> super::vals::PIO1_PIN28_SEC_MASK {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::PIO1_PIN28_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_28."]
    #[inline(always)]
    pub const fn set_PIO1_PIN28_SEC_MASK(&mut self, val: super::vals::PIO1_PIN28_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Secure mask for pin P1_29."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN29_SEC_MASK(&self) -> super::vals::PIO1_PIN29_SEC_MASK {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::PIO1_PIN29_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_29."]
    #[inline(always)]
    pub const fn set_PIO1_PIN29_SEC_MASK(&mut self, val: super::vals::PIO1_PIN29_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Secure mask for pin P1_30."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN30_SEC_MASK(&self) -> super::vals::PIO1_PIN30_SEC_MASK {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::PIO1_PIN30_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_30."]
    #[inline(always)]
    pub const fn set_PIO1_PIN30_SEC_MASK(&mut self, val: super::vals::PIO1_PIN30_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Secure mask for pin P1_31."]
    #[must_use]
    #[inline(always)]
    pub const fn PIO1_PIN31_SEC_MASK(&self) -> super::vals::PIO1_PIN31_SEC_MASK {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PIO1_PIN31_SEC_MASK::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_31."]
    #[inline(always)]
    pub const fn set_PIO1_PIN31_SEC_MASK(&mut self, val: super::vals::PIO1_PIN31_SEC_MASK) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SEC_GPIO_MASK1 {
    #[inline(always)]
    fn default() -> SEC_GPIO_MASK1 {
        SEC_GPIO_MASK1(0)
    }
}
impl core::fmt::Debug for SEC_GPIO_MASK1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_GPIO_MASK1")
            .field("PIO1_PIN0_SEC_MASK", &self.PIO1_PIN0_SEC_MASK())
            .field("PIO1_PIN1_SEC_MASK", &self.PIO1_PIN1_SEC_MASK())
            .field("PIO1_PIN2_SEC_MASK", &self.PIO1_PIN2_SEC_MASK())
            .field("PIO1_PIN3_SEC_MASK", &self.PIO1_PIN3_SEC_MASK())
            .field("PIO1_PIN4_SEC_MASK", &self.PIO1_PIN4_SEC_MASK())
            .field("PIO1_PIN5_SEC_MASK", &self.PIO1_PIN5_SEC_MASK())
            .field("PIO1_PIN6_SEC_MASK", &self.PIO1_PIN6_SEC_MASK())
            .field("PIO1_PIN7_SEC_MASK", &self.PIO1_PIN7_SEC_MASK())
            .field("PIO1_PIN8_SEC_MASK", &self.PIO1_PIN8_SEC_MASK())
            .field("PIO1_PIN9_SEC_MASK", &self.PIO1_PIN9_SEC_MASK())
            .field("PIO1_PIN10_SEC_MASK", &self.PIO1_PIN10_SEC_MASK())
            .field("PIO1_PIN11_SEC_MASK", &self.PIO1_PIN11_SEC_MASK())
            .field("PIO1_PIN12_SEC_MASK", &self.PIO1_PIN12_SEC_MASK())
            .field("PIO1_PIN13_SEC_MASK", &self.PIO1_PIN13_SEC_MASK())
            .field("PIO1_PIN14_SEC_MASK", &self.PIO1_PIN14_SEC_MASK())
            .field("PIO1_PIN15_SEC_MASK", &self.PIO1_PIN15_SEC_MASK())
            .field("PIO1_PIN16_SEC_MASK", &self.PIO1_PIN16_SEC_MASK())
            .field("PIO1_PIN17_SEC_MASK", &self.PIO1_PIN17_SEC_MASK())
            .field("PIO1_PIN18_SEC_MASK", &self.PIO1_PIN18_SEC_MASK())
            .field("PIO1_PIN19_SEC_MASK", &self.PIO1_PIN19_SEC_MASK())
            .field("PIO1_PIN20_SEC_MASK", &self.PIO1_PIN20_SEC_MASK())
            .field("PIO1_PIN21_SEC_MASK", &self.PIO1_PIN21_SEC_MASK())
            .field("PIO1_PIN22_SEC_MASK", &self.PIO1_PIN22_SEC_MASK())
            .field("PIO1_PIN23_SEC_MASK", &self.PIO1_PIN23_SEC_MASK())
            .field("PIO1_PIN24_SEC_MASK", &self.PIO1_PIN24_SEC_MASK())
            .field("PIO1_PIN25_SEC_MASK", &self.PIO1_PIN25_SEC_MASK())
            .field("PIO1_PIN26_SEC_MASK", &self.PIO1_PIN26_SEC_MASK())
            .field("PIO1_PIN27_SEC_MASK", &self.PIO1_PIN27_SEC_MASK())
            .field("PIO1_PIN28_SEC_MASK", &self.PIO1_PIN28_SEC_MASK())
            .field("PIO1_PIN29_SEC_MASK", &self.PIO1_PIN29_SEC_MASK())
            .field("PIO1_PIN30_SEC_MASK", &self.PIO1_PIN30_SEC_MASK())
            .field("PIO1_PIN31_SEC_MASK", &self.PIO1_PIN31_SEC_MASK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_GPIO_MASK1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_GPIO_MASK1 {{ PIO1_PIN0_SEC_MASK: {:?}, PIO1_PIN1_SEC_MASK: {:?}, PIO1_PIN2_SEC_MASK: {:?}, PIO1_PIN3_SEC_MASK: {:?}, PIO1_PIN4_SEC_MASK: {:?}, PIO1_PIN5_SEC_MASK: {:?}, PIO1_PIN6_SEC_MASK: {:?}, PIO1_PIN7_SEC_MASK: {:?}, PIO1_PIN8_SEC_MASK: {:?}, PIO1_PIN9_SEC_MASK: {:?}, PIO1_PIN10_SEC_MASK: {:?}, PIO1_PIN11_SEC_MASK: {:?}, PIO1_PIN12_SEC_MASK: {:?}, PIO1_PIN13_SEC_MASK: {:?}, PIO1_PIN14_SEC_MASK: {:?}, PIO1_PIN15_SEC_MASK: {:?}, PIO1_PIN16_SEC_MASK: {:?}, PIO1_PIN17_SEC_MASK: {:?}, PIO1_PIN18_SEC_MASK: {:?}, PIO1_PIN19_SEC_MASK: {:?}, PIO1_PIN20_SEC_MASK: {:?}, PIO1_PIN21_SEC_MASK: {:?}, PIO1_PIN22_SEC_MASK: {:?}, PIO1_PIN23_SEC_MASK: {:?}, PIO1_PIN24_SEC_MASK: {:?}, PIO1_PIN25_SEC_MASK: {:?}, PIO1_PIN26_SEC_MASK: {:?}, PIO1_PIN27_SEC_MASK: {:?}, PIO1_PIN28_SEC_MASK: {:?}, PIO1_PIN29_SEC_MASK: {:?}, PIO1_PIN30_SEC_MASK: {:?}, PIO1_PIN31_SEC_MASK: {:?} }}",
            self.PIO1_PIN0_SEC_MASK(),
            self.PIO1_PIN1_SEC_MASK(),
            self.PIO1_PIN2_SEC_MASK(),
            self.PIO1_PIN3_SEC_MASK(),
            self.PIO1_PIN4_SEC_MASK(),
            self.PIO1_PIN5_SEC_MASK(),
            self.PIO1_PIN6_SEC_MASK(),
            self.PIO1_PIN7_SEC_MASK(),
            self.PIO1_PIN8_SEC_MASK(),
            self.PIO1_PIN9_SEC_MASK(),
            self.PIO1_PIN10_SEC_MASK(),
            self.PIO1_PIN11_SEC_MASK(),
            self.PIO1_PIN12_SEC_MASK(),
            self.PIO1_PIN13_SEC_MASK(),
            self.PIO1_PIN14_SEC_MASK(),
            self.PIO1_PIN15_SEC_MASK(),
            self.PIO1_PIN16_SEC_MASK(),
            self.PIO1_PIN17_SEC_MASK(),
            self.PIO1_PIN18_SEC_MASK(),
            self.PIO1_PIN19_SEC_MASK(),
            self.PIO1_PIN20_SEC_MASK(),
            self.PIO1_PIN21_SEC_MASK(),
            self.PIO1_PIN22_SEC_MASK(),
            self.PIO1_PIN23_SEC_MASK(),
            self.PIO1_PIN24_SEC_MASK(),
            self.PIO1_PIN25_SEC_MASK(),
            self.PIO1_PIN26_SEC_MASK(),
            self.PIO1_PIN27_SEC_MASK(),
            self.PIO1_PIN28_SEC_MASK(),
            self.PIO1_PIN29_SEC_MASK(),
            self.PIO1_PIN30_SEC_MASK(),
            self.PIO1_PIN31_SEC_MASK()
        )
    }
}
#[doc = "Security General Purpose register access control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_MASK_LOCK(pub u32);
impl SEC_MASK_LOCK {
    #[doc = "SEC_GPIO_MASK0 register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_GPIO_MASK0_LOCK(&self) -> super::vals::SEC_GPIO_MASK0_LOCK {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SEC_GPIO_MASK0_LOCK::from_bits(val as u8)
    }
    #[doc = "SEC_GPIO_MASK0 register write-lock."]
    #[inline(always)]
    pub const fn set_SEC_GPIO_MASK0_LOCK(&mut self, val: super::vals::SEC_GPIO_MASK0_LOCK) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SEC_GPIO_MASK1 register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_GPIO_MASK1_LOCK(&self) -> super::vals::SEC_GPIO_MASK1_LOCK {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SEC_GPIO_MASK1_LOCK::from_bits(val as u8)
    }
    #[doc = "SEC_GPIO_MASK1 register write-lock."]
    #[inline(always)]
    pub const fn set_SEC_GPIO_MASK1_LOCK(&mut self, val: super::vals::SEC_GPIO_MASK1_LOCK) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "SEC_CPU_INT_MASK0 register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_CPU1_INT_MASK0_LOCK(&self) -> super::vals::SEC_CPU1_INT_MASK0_LOCK {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SEC_CPU1_INT_MASK0_LOCK::from_bits(val as u8)
    }
    #[doc = "SEC_CPU_INT_MASK0 register write-lock."]
    #[inline(always)]
    pub const fn set_SEC_CPU1_INT_MASK0_LOCK(&mut self, val: super::vals::SEC_CPU1_INT_MASK0_LOCK) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "SEC_CPU_INT_MASK1 register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_CPU1_INT_MASK1_LOCK(&self) -> super::vals::SEC_CPU1_INT_MASK1_LOCK {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::SEC_CPU1_INT_MASK1_LOCK::from_bits(val as u8)
    }
    #[doc = "SEC_CPU_INT_MASK1 register write-lock."]
    #[inline(always)]
    pub const fn set_SEC_CPU1_INT_MASK1_LOCK(&mut self, val: super::vals::SEC_CPU1_INT_MASK1_LOCK) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
}
impl Default for SEC_MASK_LOCK {
    #[inline(always)]
    fn default() -> SEC_MASK_LOCK {
        SEC_MASK_LOCK(0)
    }
}
impl core::fmt::Debug for SEC_MASK_LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_MASK_LOCK")
            .field("SEC_GPIO_MASK0_LOCK", &self.SEC_GPIO_MASK0_LOCK())
            .field("SEC_GPIO_MASK1_LOCK", &self.SEC_GPIO_MASK1_LOCK())
            .field("SEC_CPU1_INT_MASK0_LOCK", &self.SEC_CPU1_INT_MASK0_LOCK())
            .field("SEC_CPU1_INT_MASK1_LOCK", &self.SEC_CPU1_INT_MASK1_LOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_MASK_LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_MASK_LOCK {{ SEC_GPIO_MASK0_LOCK: {:?}, SEC_GPIO_MASK1_LOCK: {:?}, SEC_CPU1_INT_MASK0_LOCK: {:?}, SEC_CPU1_INT_MASK1_LOCK: {:?} }}",
            self.SEC_GPIO_MASK0_LOCK(),
            self.SEC_GPIO_MASK1_LOCK(),
            self.SEC_CPU1_INT_MASK0_LOCK(),
            self.SEC_CPU1_INT_MASK1_LOCK()
        )
    }
}
#[doc = "security violation address/information registers valid flags."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC_VIO_INFO_VALID(pub u32);
impl SEC_VIO_INFO_VALID {
    #[doc = "violation information valid flag for AHB port 0. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn VIO_INFO_VALID0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 0. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_VIO_INFO_VALID0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "violation information valid flag for AHB port 1. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn VIO_INFO_VALID1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 1. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_VIO_INFO_VALID1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "violation information valid flag for AHB port 2. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn VIO_INFO_VALID2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 2. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_VIO_INFO_VALID2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "violation information valid flag for AHB port 3. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn VIO_INFO_VALID3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 3. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_VIO_INFO_VALID3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "violation information valid flag for AHB port 4. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn VIO_INFO_VALID4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 4. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_VIO_INFO_VALID4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "violation information valid flag for AHB port 5. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn VIO_INFO_VALID5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 5. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_VIO_INFO_VALID5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "violation information valid flag for AHB port 6. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn VIO_INFO_VALID6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 6. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_VIO_INFO_VALID6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "violation information valid flag for AHB port 7. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn VIO_INFO_VALID7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 7. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_VIO_INFO_VALID7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "violation information valid flag for AHB port 8. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn VIO_INFO_VALID8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 8. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_VIO_INFO_VALID8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "violation information valid flag for AHB port 9. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn VIO_INFO_VALID9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 9. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_VIO_INFO_VALID9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "violation information valid flag for AHB port 10. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn VIO_INFO_VALID10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 10. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_VIO_INFO_VALID10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "violation information valid flag for AHB port 11. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn VIO_INFO_VALID11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 11. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_VIO_INFO_VALID11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for SEC_VIO_INFO_VALID {
    #[inline(always)]
    fn default() -> SEC_VIO_INFO_VALID {
        SEC_VIO_INFO_VALID(0)
    }
}
impl core::fmt::Debug for SEC_VIO_INFO_VALID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_VIO_INFO_VALID")
            .field("VIO_INFO_VALID0", &self.VIO_INFO_VALID0())
            .field("VIO_INFO_VALID1", &self.VIO_INFO_VALID1())
            .field("VIO_INFO_VALID2", &self.VIO_INFO_VALID2())
            .field("VIO_INFO_VALID3", &self.VIO_INFO_VALID3())
            .field("VIO_INFO_VALID4", &self.VIO_INFO_VALID4())
            .field("VIO_INFO_VALID5", &self.VIO_INFO_VALID5())
            .field("VIO_INFO_VALID6", &self.VIO_INFO_VALID6())
            .field("VIO_INFO_VALID7", &self.VIO_INFO_VALID7())
            .field("VIO_INFO_VALID8", &self.VIO_INFO_VALID8())
            .field("VIO_INFO_VALID9", &self.VIO_INFO_VALID9())
            .field("VIO_INFO_VALID10", &self.VIO_INFO_VALID10())
            .field("VIO_INFO_VALID11", &self.VIO_INFO_VALID11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_VIO_INFO_VALID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SEC_VIO_INFO_VALID {{ VIO_INFO_VALID0: {=bool:?}, VIO_INFO_VALID1: {=bool:?}, VIO_INFO_VALID2: {=bool:?}, VIO_INFO_VALID3: {=bool:?}, VIO_INFO_VALID4: {=bool:?}, VIO_INFO_VALID5: {=bool:?}, VIO_INFO_VALID6: {=bool:?}, VIO_INFO_VALID7: {=bool:?}, VIO_INFO_VALID8: {=bool:?}, VIO_INFO_VALID9: {=bool:?}, VIO_INFO_VALID10: {=bool:?}, VIO_INFO_VALID11: {=bool:?} }}",
            self.VIO_INFO_VALID0(),
            self.VIO_INFO_VALID1(),
            self.VIO_INFO_VALID2(),
            self.VIO_INFO_VALID3(),
            self.VIO_INFO_VALID4(),
            self.VIO_INFO_VALID5(),
            self.VIO_INFO_VALID6(),
            self.VIO_INFO_VALID7(),
            self.VIO_INFO_VALID8(),
            self.VIO_INFO_VALID9(),
            self.VIO_INFO_VALID10(),
            self.VIO_INFO_VALID11()
        )
    }
}
#[doc = "most recent security violation address for AHB port n."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct sec_vio_addr(pub u32);
impl sec_vio_addr {
    #[doc = "security violation address for AHB port."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_VIO_ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "security violation address for AHB port."]
    #[inline(always)]
    pub const fn set_SEC_VIO_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for sec_vio_addr {
    #[inline(always)]
    fn default() -> sec_vio_addr {
        sec_vio_addr(0)
    }
}
impl core::fmt::Debug for sec_vio_addr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("sec_vio_addr")
            .field("SEC_VIO_ADDR", &self.SEC_VIO_ADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for sec_vio_addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "sec_vio_addr {{ SEC_VIO_ADDR: {=u32:?} }}",
            self.SEC_VIO_ADDR()
        )
    }
}
#[doc = "most recent security violation miscellaneous information for AHB port n."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct sec_vio_misc_info(pub u32);
impl sec_vio_misc_info {
    #[doc = "security violation access read/write indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_VIO_INFO_WRITE(&self) -> super::vals::SEC_VIO_INFO_WRITE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SEC_VIO_INFO_WRITE::from_bits(val as u8)
    }
    #[doc = "security violation access read/write indicator."]
    #[inline(always)]
    pub const fn set_SEC_VIO_INFO_WRITE(&mut self, val: super::vals::SEC_VIO_INFO_WRITE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "security violation access data/code indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_VIO_INFO_DATA_ACCESS(&self) -> super::vals::SEC_VIO_INFO_DATA_ACCESS {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SEC_VIO_INFO_DATA_ACCESS::from_bits(val as u8)
    }
    #[doc = "security violation access data/code indicator."]
    #[inline(always)]
    pub const fn set_SEC_VIO_INFO_DATA_ACCESS(
        &mut self,
        val: super::vals::SEC_VIO_INFO_DATA_ACCESS,
    ) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "bit \\[5:4\\]: master sec level and privilege level bit \\[7:6\\]: anti-pol value for master sec level and privilege level."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_VIO_INFO_MASTER_SEC_LEVEL(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "bit \\[5:4\\]: master sec level and privilege level bit \\[7:6\\]: anti-pol value for master sec level and privilege level."]
    #[inline(always)]
    pub const fn set_SEC_VIO_INFO_MASTER_SEC_LEVEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "security violation master number."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_VIO_INFO_MASTER(&self) -> super::vals::SEC_VIO_INFO_MASTER {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::SEC_VIO_INFO_MASTER::from_bits(val as u8)
    }
    #[doc = "security violation master number."]
    #[inline(always)]
    pub const fn set_SEC_VIO_INFO_MASTER(&mut self, val: super::vals::SEC_VIO_INFO_MASTER) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
}
impl Default for sec_vio_misc_info {
    #[inline(always)]
    fn default() -> sec_vio_misc_info {
        sec_vio_misc_info(0)
    }
}
impl core::fmt::Debug for sec_vio_misc_info {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("sec_vio_misc_info")
            .field("SEC_VIO_INFO_WRITE", &self.SEC_VIO_INFO_WRITE())
            .field("SEC_VIO_INFO_DATA_ACCESS", &self.SEC_VIO_INFO_DATA_ACCESS())
            .field(
                "SEC_VIO_INFO_MASTER_SEC_LEVEL",
                &self.SEC_VIO_INFO_MASTER_SEC_LEVEL(),
            )
            .field("SEC_VIO_INFO_MASTER", &self.SEC_VIO_INFO_MASTER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for sec_vio_misc_info {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "sec_vio_misc_info {{ SEC_VIO_INFO_WRITE: {:?}, SEC_VIO_INFO_DATA_ACCESS: {:?}, SEC_VIO_INFO_MASTER_SEC_LEVEL: {=u8:?}, SEC_VIO_INFO_MASTER: {:?} }}",
            self.SEC_VIO_INFO_WRITE(),
            self.SEC_VIO_INFO_DATA_ACCESS(),
            self.SEC_VIO_INFO_MASTER_SEC_LEVEL(),
            self.SEC_VIO_INFO_MASTER()
        )
    }
}
