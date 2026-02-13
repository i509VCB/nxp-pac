#[doc = "Miscalleneous control signals for in Cortex M33 (CPU0)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0LockReg(pub u32);
impl Cpu0LockReg {
    #[doc = "Cortex M33 (CPU0) VTOR_NS register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_vtor(&self) -> super::vals::Cpu0LockRegLockNsVtor {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cpu0LockRegLockNsVtor::from_bits(val as u8)
    }
    #[doc = "Cortex M33 (CPU0) VTOR_NS register write-lock."]
    #[inline(always)]
    pub const fn set_lock_ns_vtor(&mut self, val: super::vals::Cpu0LockRegLockNsVtor) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Cortex M33 (CPU0) non-secure MPU register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_mpu(&self) -> super::vals::Cpu0LockRegLockNsMpu {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Cpu0LockRegLockNsMpu::from_bits(val as u8)
    }
    #[doc = "Cortex M33 (CPU0) non-secure MPU register write-lock."]
    #[inline(always)]
    pub const fn set_lock_ns_mpu(&mut self, val: super::vals::Cpu0LockRegLockNsMpu) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Cortex M33 (CPU0) VTOR_S, AIRCR.PRIS, IRCR.BFHFNMINS registers write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_s_vtaircr(&self) -> super::vals::LockSVtaircr {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::LockSVtaircr::from_bits(val as u8)
    }
    #[doc = "Cortex M33 (CPU0) VTOR_S, AIRCR.PRIS, IRCR.BFHFNMINS registers write-lock."]
    #[inline(always)]
    pub const fn set_lock_s_vtaircr(&mut self, val: super::vals::LockSVtaircr) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Cortex M33 (CPU0) Secure MPU registers write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_s_mpu(&self) -> super::vals::LockSMpu {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::LockSMpu::from_bits(val as u8)
    }
    #[doc = "Cortex M33 (CPU0) Secure MPU registers write-lock."]
    #[inline(always)]
    pub const fn set_lock_s_mpu(&mut self, val: super::vals::LockSMpu) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Cortex M33 (CPU0) SAU registers write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_sau(&self) -> super::vals::LockSau {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::LockSau::from_bits(val as u8)
    }
    #[doc = "Cortex M33 (CPU0) SAU registers write-lock."]
    #[inline(always)]
    pub const fn set_lock_sau(&mut self, val: super::vals::LockSau) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CPU0_LOCK_REG write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_lock_reg_lock(&self) -> super::vals::Cpu0LockRegLock {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Cpu0LockRegLock::from_bits(val as u8)
    }
    #[doc = "CPU0_LOCK_REG write-lock."]
    #[inline(always)]
    pub const fn set_cpu0_lock_reg_lock(&mut self, val: super::vals::Cpu0LockRegLock) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Cpu0LockReg {
    #[inline(always)]
    fn default() -> Cpu0LockReg {
        Cpu0LockReg(0)
    }
}
impl core::fmt::Debug for Cpu0LockReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu0LockReg")
            .field("lock_ns_vtor", &self.lock_ns_vtor())
            .field("lock_ns_mpu", &self.lock_ns_mpu())
            .field("lock_s_vtaircr", &self.lock_s_vtaircr())
            .field("lock_s_mpu", &self.lock_s_mpu())
            .field("lock_sau", &self.lock_sau())
            .field("cpu0_lock_reg_lock", &self.cpu0_lock_reg_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu0LockReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu0LockReg {{ lock_ns_vtor: {:?}, lock_ns_mpu: {:?}, lock_s_vtaircr: {:?}, lock_s_mpu: {:?}, lock_sau: {:?}, cpu0_lock_reg_lock: {:?} }}",
            self.lock_ns_vtor(),
            self.lock_ns_mpu(),
            self.lock_s_vtaircr(),
            self.lock_s_mpu(),
            self.lock_sau(),
            self.cpu0_lock_reg_lock()
        )
    }
}
#[doc = "Miscalleneous control signals for in micro-Cortex M33 (CPU1)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu1LockReg(pub u32);
impl Cpu1LockReg {
    #[doc = "micro-Cortex M33 (CPU1) VTOR_NS register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_vtor(&self) -> super::vals::Cpu1LockRegLockNsVtor {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cpu1LockRegLockNsVtor::from_bits(val as u8)
    }
    #[doc = "micro-Cortex M33 (CPU1) VTOR_NS register write-lock."]
    #[inline(always)]
    pub const fn set_lock_ns_vtor(&mut self, val: super::vals::Cpu1LockRegLockNsVtor) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "micro-Cortex M33 (CPU1) non-secure MPU register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_mpu(&self) -> super::vals::Cpu1LockRegLockNsMpu {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Cpu1LockRegLockNsMpu::from_bits(val as u8)
    }
    #[doc = "micro-Cortex M33 (CPU1) non-secure MPU register write-lock."]
    #[inline(always)]
    pub const fn set_lock_ns_mpu(&mut self, val: super::vals::Cpu1LockRegLockNsMpu) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CPU1_LOCK_REG write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_lock_reg_lock(&self) -> super::vals::Cpu1LockRegLock {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Cpu1LockRegLock::from_bits(val as u8)
    }
    #[doc = "CPU1_LOCK_REG write-lock."]
    #[inline(always)]
    pub const fn set_cpu1_lock_reg_lock(&mut self, val: super::vals::Cpu1LockRegLock) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Cpu1LockReg {
    #[inline(always)]
    fn default() -> Cpu1LockReg {
        Cpu1LockReg(0)
    }
}
impl core::fmt::Debug for Cpu1LockReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu1LockReg")
            .field("lock_ns_vtor", &self.lock_ns_vtor())
            .field("lock_ns_mpu", &self.lock_ns_mpu())
            .field("cpu1_lock_reg_lock", &self.cpu1_lock_reg_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu1LockReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu1LockReg {{ lock_ns_vtor: {:?}, lock_ns_mpu: {:?}, cpu1_lock_reg_lock: {:?} }}",
            self.lock_ns_vtor(),
            self.lock_ns_mpu(),
            self.cpu1_lock_reg_lock()
        )
    }
}
#[doc = "master secure level anti-pole register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MasterSecAntiPolReg(pub u32);
impl MasterSecAntiPolReg {
    #[doc = "Micro-Cortex M33 (CPU1) Code bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1C)"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1c(&self) -> super::vals::MasterSecAntiPolRegCpu1c {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MasterSecAntiPolRegCpu1c::from_bits(val as u8)
    }
    #[doc = "Micro-Cortex M33 (CPU1) Code bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1C)"]
    #[inline(always)]
    pub const fn set_cpu1c(&mut self, val: super::vals::MasterSecAntiPolRegCpu1c) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Micro-Cortex M33 (CPU1) System bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1S)"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1s(&self) -> super::vals::MasterSecAntiPolRegCpu1s {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MasterSecAntiPolRegCpu1s::from_bits(val as u8)
    }
    #[doc = "Micro-Cortex M33 (CPU1) System bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1S)"]
    #[inline(always)]
    pub const fn set_cpu1s(&mut self, val: super::vals::MasterSecAntiPolRegCpu1s) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "USB Full Speed Device. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSD)"]
    #[must_use]
    #[inline(always)]
    pub const fn usbfsd(&self) -> super::vals::MasterSecAntiPolRegUsbfsd {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MasterSecAntiPolRegUsbfsd::from_bits(val as u8)
    }
    #[doc = "USB Full Speed Device. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSD)"]
    #[inline(always)]
    pub const fn set_usbfsd(&mut self, val: super::vals::MasterSecAntiPolRegUsbfsd) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)"]
    #[must_use]
    #[inline(always)]
    pub const fn sdma0(&self) -> super::vals::MasterSecAntiPolRegSdma0 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MasterSecAntiPolRegSdma0::from_bits(val as u8)
    }
    #[doc = "System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)"]
    #[inline(always)]
    pub const fn set_sdma0(&mut self, val: super::vals::MasterSecAntiPolRegSdma0) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "SDIO. Must be equal to NOT(MASTER_SEC_LEVEL.SDIO)"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio(&self) -> super::vals::MasterSecAntiPolRegSdio {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::MasterSecAntiPolRegSdio::from_bits(val as u8)
    }
    #[doc = "SDIO. Must be equal to NOT(MASTER_SEC_LEVEL.SDIO)"]
    #[inline(always)]
    pub const fn set_sdio(&mut self, val: super::vals::MasterSecAntiPolRegSdio) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Power Quad. Must be equal to NOT(MASTER_SEC_LEVEL.PQ)"]
    #[must_use]
    #[inline(always)]
    pub const fn pq(&self) -> super::vals::MasterSecAntiPolRegPq {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::MasterSecAntiPolRegPq::from_bits(val as u8)
    }
    #[doc = "Power Quad. Must be equal to NOT(MASTER_SEC_LEVEL.PQ)"]
    #[inline(always)]
    pub const fn set_pq(&mut self, val: super::vals::MasterSecAntiPolRegPq) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)"]
    #[must_use]
    #[inline(always)]
    pub const fn hash(&self) -> super::vals::MasterSecAntiPolRegHash {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::MasterSecAntiPolRegHash::from_bits(val as u8)
    }
    #[doc = "Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)"]
    #[inline(always)]
    pub const fn set_hash(&mut self, val: super::vals::MasterSecAntiPolRegHash) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "USB Full speed Host. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSH)"]
    #[must_use]
    #[inline(always)]
    pub const fn usbfsh(&self) -> super::vals::MasterSecAntiPolRegUsbfsh {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::MasterSecAntiPolRegUsbfsh::from_bits(val as u8)
    }
    #[doc = "USB Full speed Host. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSH)"]
    #[inline(always)]
    pub const fn set_usbfsh(&mut self, val: super::vals::MasterSecAntiPolRegUsbfsh) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)"]
    #[must_use]
    #[inline(always)]
    pub const fn sdma1(&self) -> super::vals::MasterSecAntiPolRegSdma1 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::MasterSecAntiPolRegSdma1::from_bits(val as u8)
    }
    #[doc = "System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)"]
    #[inline(always)]
    pub const fn set_sdma1(&mut self, val: super::vals::MasterSecAntiPolRegSdma1) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "MASTER_SEC_ANTI_POL_REG register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn master_sec_level_antipol_lock(&self) -> super::vals::MasterSecLevelAntipolLock {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::MasterSecLevelAntipolLock::from_bits(val as u8)
    }
    #[doc = "MASTER_SEC_ANTI_POL_REG register write-lock."]
    #[inline(always)]
    pub const fn set_master_sec_level_antipol_lock(
        &mut self,
        val: super::vals::MasterSecLevelAntipolLock,
    ) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for MasterSecAntiPolReg {
    #[inline(always)]
    fn default() -> MasterSecAntiPolReg {
        MasterSecAntiPolReg(0)
    }
}
impl core::fmt::Debug for MasterSecAntiPolReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MasterSecAntiPolReg")
            .field("cpu1c", &self.cpu1c())
            .field("cpu1s", &self.cpu1s())
            .field("usbfsd", &self.usbfsd())
            .field("sdma0", &self.sdma0())
            .field("sdio", &self.sdio())
            .field("pq", &self.pq())
            .field("hash", &self.hash())
            .field("usbfsh", &self.usbfsh())
            .field("sdma1", &self.sdma1())
            .field(
                "master_sec_level_antipol_lock",
                &self.master_sec_level_antipol_lock(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MasterSecAntiPolReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MasterSecAntiPolReg {{ cpu1c: {:?}, cpu1s: {:?}, usbfsd: {:?}, sdma0: {:?}, sdio: {:?}, pq: {:?}, hash: {:?}, usbfsh: {:?}, sdma1: {:?}, master_sec_level_antipol_lock: {:?} }}",
            self.cpu1c(),
            self.cpu1s(),
            self.usbfsd(),
            self.sdma0(),
            self.sdio(),
            self.pq(),
            self.hash(),
            self.usbfsh(),
            self.sdma1(),
            self.master_sec_level_antipol_lock()
        )
    }
}
#[doc = "master secure level register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MasterSecLevel(pub u32);
impl MasterSecLevel {
    #[doc = "Micro-Cortex M33 (CPU1) Code bus."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1c(&self) -> super::vals::MasterSecLevelCpu1c {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MasterSecLevelCpu1c::from_bits(val as u8)
    }
    #[doc = "Micro-Cortex M33 (CPU1) Code bus."]
    #[inline(always)]
    pub const fn set_cpu1c(&mut self, val: super::vals::MasterSecLevelCpu1c) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Micro-Cortex M33 (CPU1) System bus."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1s(&self) -> super::vals::MasterSecLevelCpu1s {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MasterSecLevelCpu1s::from_bits(val as u8)
    }
    #[doc = "Micro-Cortex M33 (CPU1) System bus."]
    #[inline(always)]
    pub const fn set_cpu1s(&mut self, val: super::vals::MasterSecLevelCpu1s) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "USB Full Speed Device."]
    #[must_use]
    #[inline(always)]
    pub const fn usbfsd(&self) -> super::vals::MasterSecLevelUsbfsd {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MasterSecLevelUsbfsd::from_bits(val as u8)
    }
    #[doc = "USB Full Speed Device."]
    #[inline(always)]
    pub const fn set_usbfsd(&mut self, val: super::vals::MasterSecLevelUsbfsd) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "System DMA 0."]
    #[must_use]
    #[inline(always)]
    pub const fn sdma0(&self) -> super::vals::MasterSecLevelSdma0 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MasterSecLevelSdma0::from_bits(val as u8)
    }
    #[doc = "System DMA 0."]
    #[inline(always)]
    pub const fn set_sdma0(&mut self, val: super::vals::MasterSecLevelSdma0) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "SDIO."]
    #[must_use]
    #[inline(always)]
    pub const fn sdio(&self) -> super::vals::MasterSecLevelSdio {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::MasterSecLevelSdio::from_bits(val as u8)
    }
    #[doc = "SDIO."]
    #[inline(always)]
    pub const fn set_sdio(&mut self, val: super::vals::MasterSecLevelSdio) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Power Quad."]
    #[must_use]
    #[inline(always)]
    pub const fn pq(&self) -> super::vals::MasterSecLevelPq {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::MasterSecLevelPq::from_bits(val as u8)
    }
    #[doc = "Power Quad."]
    #[inline(always)]
    pub const fn set_pq(&mut self, val: super::vals::MasterSecLevelPq) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Hash."]
    #[must_use]
    #[inline(always)]
    pub const fn hash(&self) -> super::vals::MasterSecLevelHash {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::MasterSecLevelHash::from_bits(val as u8)
    }
    #[doc = "Hash."]
    #[inline(always)]
    pub const fn set_hash(&mut self, val: super::vals::MasterSecLevelHash) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "USB Full speed Host."]
    #[must_use]
    #[inline(always)]
    pub const fn usbfsh(&self) -> super::vals::MasterSecLevelUsbfsh {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::MasterSecLevelUsbfsh::from_bits(val as u8)
    }
    #[doc = "USB Full speed Host."]
    #[inline(always)]
    pub const fn set_usbfsh(&mut self, val: super::vals::MasterSecLevelUsbfsh) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "System DMA 1 security level."]
    #[must_use]
    #[inline(always)]
    pub const fn sdma1(&self) -> super::vals::MasterSecLevelSdma1 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::MasterSecLevelSdma1::from_bits(val as u8)
    }
    #[doc = "System DMA 1 security level."]
    #[inline(always)]
    pub const fn set_sdma1(&mut self, val: super::vals::MasterSecLevelSdma1) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "MASTER_SEC_LEVEL write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn master_sec_level_lock(&self) -> super::vals::MasterSecLevelLock {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::MasterSecLevelLock::from_bits(val as u8)
    }
    #[doc = "MASTER_SEC_LEVEL write-lock."]
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
            .field("cpu1c", &self.cpu1c())
            .field("cpu1s", &self.cpu1s())
            .field("usbfsd", &self.usbfsd())
            .field("sdma0", &self.sdma0())
            .field("sdio", &self.sdio())
            .field("pq", &self.pq())
            .field("hash", &self.hash())
            .field("usbfsh", &self.usbfsh())
            .field("sdma1", &self.sdma1())
            .field("master_sec_level_lock", &self.master_sec_level_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MasterSecLevel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MasterSecLevel {{ cpu1c: {:?}, cpu1s: {:?}, usbfsd: {:?}, sdma0: {:?}, sdio: {:?}, pq: {:?}, hash: {:?}, usbfsh: {:?}, sdma1: {:?}, master_sec_level_lock: {:?} }}",
            self.cpu1c(),
            self.cpu1s(),
            self.usbfsd(),
            self.sdma0(),
            self.sdio(),
            self.pq(),
            self.hash(),
            self.usbfsh(),
            self.sdma1(),
            self.master_sec_level_lock()
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
    #[doc = "Enable secure check for AHB matrix."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_secure_checking(&self) -> super::vals::MiscCtrlDpRegEnableSecureChecking {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::MiscCtrlDpRegEnableSecureChecking::from_bits(val as u8)
    }
    #[doc = "Enable secure check for AHB matrix."]
    #[inline(always)]
    pub const fn set_enable_secure_checking(
        &mut self,
        val: super::vals::MiscCtrlDpRegEnableSecureChecking,
    ) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable secure privilege check for AHB matrix."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_s_priv_check(&self) -> super::vals::MiscCtrlDpRegEnableSPrivCheck {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MiscCtrlDpRegEnableSPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable secure privilege check for AHB matrix."]
    #[inline(always)]
    pub const fn set_enable_s_priv_check(
        &mut self,
        val: super::vals::MiscCtrlDpRegEnableSPrivCheck,
    ) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Enable non-secure privilege check for AHB matrix."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ns_priv_check(&self) -> super::vals::MiscCtrlDpRegEnableNsPrivCheck {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MiscCtrlDpRegEnableNsPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable non-secure privilege check for AHB matrix."]
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
    #[doc = "Enable secure check for AHB matrix."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_secure_checking(&self) -> super::vals::MiscCtrlRegEnableSecureChecking {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::MiscCtrlRegEnableSecureChecking::from_bits(val as u8)
    }
    #[doc = "Enable secure check for AHB matrix."]
    #[inline(always)]
    pub const fn set_enable_secure_checking(
        &mut self,
        val: super::vals::MiscCtrlRegEnableSecureChecking,
    ) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable secure privilege check for AHB matrix."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_s_priv_check(&self) -> super::vals::MiscCtrlRegEnableSPrivCheck {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MiscCtrlRegEnableSPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable secure privilege check for AHB matrix."]
    #[inline(always)]
    pub const fn set_enable_s_priv_check(&mut self, val: super::vals::MiscCtrlRegEnableSPrivCheck) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Enable non-secure privilege check for AHB matrix."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ns_priv_check(&self) -> super::vals::MiscCtrlRegEnableNsPrivCheck {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MiscCtrlRegEnableNsPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable non-secure privilege check for AHB matrix."]
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
#[doc = "Secure Interrupt mask for CPU1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCpuIntMask0(pub u32);
impl SecCpuIntMask0 {
    #[doc = "Watchdog Timer, Brown Out Detectors and Flash Controller interrupts"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_irq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Timer, Brown Out Detectors and Flash Controller interrupts"]
    #[inline(always)]
    pub const fn set_sys_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "System DMA 0 (non-secure) interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn sdma0_irq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "System DMA 0 (non-secure) interrupt."]
    #[inline(always)]
    pub const fn set_sdma0_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GPIO Group 0 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_globalint0_irq(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO Group 0 interrupt."]
    #[inline(always)]
    pub const fn set_gpio_globalint0_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GPIO Group 1 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_globalint1_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO Group 1 interrupt."]
    #[inline(always)]
    pub const fn set_gpio_globalint1_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline(always)]
    pub const fn set_gpio_int0_irq0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline(always)]
    pub const fn set_gpio_int0_irq1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Pin interrupt 2 or pattern match engine slice 2 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Pin interrupt 2 or pattern match engine slice 2 interrupt."]
    #[inline(always)]
    pub const fn set_gpio_int0_irq2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin interrupt 3 or pattern match engine slice 3 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Pin interrupt 3 or pattern match engine slice 3 interrupt."]
    #[inline(always)]
    pub const fn set_gpio_int0_irq3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Micro Tick Timer interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn utick_irq(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Micro Tick Timer interrupt."]
    #[inline(always)]
    pub const fn set_utick_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Multi-Rate Timer interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn mrt_irq(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Rate Timer interrupt."]
    #[inline(always)]
    pub const fn set_mrt_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Standard counter/timer 0 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Standard counter/timer 0 interrupt."]
    #[inline(always)]
    pub const fn set_ctimer0_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Standard counter/timer 1 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1_irq(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Standard counter/timer 1 interrupt."]
    #[inline(always)]
    pub const fn set_ctimer1_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SCTimer/PWM interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn sct_irq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SCTimer/PWM interrupt."]
    #[inline(always)]
    pub const fn set_sct_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Standard counter/timer 3 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3_irq(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Standard counter/timer 3 interrupt."]
    #[inline(always)]
    pub const fn set_ctimer3_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm0_irq(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub const fn set_flexcomm0_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm1_irq(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub const fn set_flexcomm1_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm2_irq(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub const fn set_flexcomm2_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm3_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub const fn set_flexcomm3_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm4_irq(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub const fn set_flexcomm4_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm5_irq(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub const fn set_flexcomm5_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm6_irq(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub const fn set_flexcomm6_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm7_irq(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub const fn set_flexcomm7_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "General Purpose ADC interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn adc_irq(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose ADC interrupt."]
    #[inline(always)]
    pub const fn set_adc_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_reserved0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Analog Comparator interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp_irq(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator interrupt."]
    #[inline(always)]
    pub const fn set_acmp_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_reserved1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved2(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_reserved2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB Full Speed Controller Clock request interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0_needclk(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB Full Speed Controller Clock request interrupt."]
    #[inline(always)]
    pub const fn set_usb0_needclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "USB Full Speed Controller interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0_irq(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "USB Full Speed Controller interrupt."]
    #[inline(always)]
    pub const fn set_usb0_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_irq(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ"]
    #[inline(always)]
    pub const fn set_rtc_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved3(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_reserved3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Mailbox interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn mailbox_irq(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Mailbox interrupt."]
    #[inline(always)]
    pub const fn set_mailbox_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecCpuIntMask0 {
    #[inline(always)]
    fn default() -> SecCpuIntMask0 {
        SecCpuIntMask0(0)
    }
}
impl core::fmt::Debug for SecCpuIntMask0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCpuIntMask0")
            .field("sys_irq", &self.sys_irq())
            .field("sdma0_irq", &self.sdma0_irq())
            .field("gpio_globalint0_irq", &self.gpio_globalint0_irq())
            .field("gpio_globalint1_irq", &self.gpio_globalint1_irq())
            .field("gpio_int0_irq0", &self.gpio_int0_irq0())
            .field("gpio_int0_irq1", &self.gpio_int0_irq1())
            .field("gpio_int0_irq2", &self.gpio_int0_irq2())
            .field("gpio_int0_irq3", &self.gpio_int0_irq3())
            .field("utick_irq", &self.utick_irq())
            .field("mrt_irq", &self.mrt_irq())
            .field("ctimer0_irq", &self.ctimer0_irq())
            .field("ctimer1_irq", &self.ctimer1_irq())
            .field("sct_irq", &self.sct_irq())
            .field("ctimer3_irq", &self.ctimer3_irq())
            .field("flexcomm0_irq", &self.flexcomm0_irq())
            .field("flexcomm1_irq", &self.flexcomm1_irq())
            .field("flexcomm2_irq", &self.flexcomm2_irq())
            .field("flexcomm3_irq", &self.flexcomm3_irq())
            .field("flexcomm4_irq", &self.flexcomm4_irq())
            .field("flexcomm5_irq", &self.flexcomm5_irq())
            .field("flexcomm6_irq", &self.flexcomm6_irq())
            .field("flexcomm7_irq", &self.flexcomm7_irq())
            .field("adc_irq", &self.adc_irq())
            .field("reserved0", &self.reserved0())
            .field("acmp_irq", &self.acmp_irq())
            .field("reserved1", &self.reserved1())
            .field("reserved2", &self.reserved2())
            .field("usb0_needclk", &self.usb0_needclk())
            .field("usb0_irq", &self.usb0_irq())
            .field("rtc_irq", &self.rtc_irq())
            .field("reserved3", &self.reserved3())
            .field("mailbox_irq", &self.mailbox_irq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCpuIntMask0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCpuIntMask0 {{ sys_irq: {=bool:?}, sdma0_irq: {=bool:?}, gpio_globalint0_irq: {=bool:?}, gpio_globalint1_irq: {=bool:?}, gpio_int0_irq0: {=bool:?}, gpio_int0_irq1: {=bool:?}, gpio_int0_irq2: {=bool:?}, gpio_int0_irq3: {=bool:?}, utick_irq: {=bool:?}, mrt_irq: {=bool:?}, ctimer0_irq: {=bool:?}, ctimer1_irq: {=bool:?}, sct_irq: {=bool:?}, ctimer3_irq: {=bool:?}, flexcomm0_irq: {=bool:?}, flexcomm1_irq: {=bool:?}, flexcomm2_irq: {=bool:?}, flexcomm3_irq: {=bool:?}, flexcomm4_irq: {=bool:?}, flexcomm5_irq: {=bool:?}, flexcomm6_irq: {=bool:?}, flexcomm7_irq: {=bool:?}, adc_irq: {=bool:?}, reserved0: {=bool:?}, acmp_irq: {=bool:?}, reserved1: {=bool:?}, reserved2: {=bool:?}, usb0_needclk: {=bool:?}, usb0_irq: {=bool:?}, rtc_irq: {=bool:?}, reserved3: {=bool:?}, mailbox_irq: {=bool:?} }}",
            self.sys_irq(),
            self.sdma0_irq(),
            self.gpio_globalint0_irq(),
            self.gpio_globalint1_irq(),
            self.gpio_int0_irq0(),
            self.gpio_int0_irq1(),
            self.gpio_int0_irq2(),
            self.gpio_int0_irq3(),
            self.utick_irq(),
            self.mrt_irq(),
            self.ctimer0_irq(),
            self.ctimer1_irq(),
            self.sct_irq(),
            self.ctimer3_irq(),
            self.flexcomm0_irq(),
            self.flexcomm1_irq(),
            self.flexcomm2_irq(),
            self.flexcomm3_irq(),
            self.flexcomm4_irq(),
            self.flexcomm5_irq(),
            self.flexcomm6_irq(),
            self.flexcomm7_irq(),
            self.adc_irq(),
            self.reserved0(),
            self.acmp_irq(),
            self.reserved1(),
            self.reserved2(),
            self.usb0_needclk(),
            self.usb0_irq(),
            self.rtc_irq(),
            self.reserved3(),
            self.mailbox_irq()
        )
    }
}
#[doc = "Secure Interrupt mask for CPU1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCpuIntMask1(pub u32);
impl SecCpuIntMask1 {
    #[doc = "Pin interrupt 4 or pattern match engine slice 4 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq4(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pin interrupt 4 or pattern match engine slice 4 interrupt."]
    #[inline(always)]
    pub const fn set_gpio_int0_irq4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pin interrupt 5 or pattern match engine slice 5 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq5(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pin interrupt 5 or pattern match engine slice 5 interrupt."]
    #[inline(always)]
    pub const fn set_gpio_int0_irq5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pin interrupt 6 or pattern match engine slice 6 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq6(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Pin interrupt 6 or pattern match engine slice 6 interrupt."]
    #[inline(always)]
    pub const fn set_gpio_int0_irq6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Pin interrupt 7 or pattern match engine slice 7 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq7(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Pin interrupt 7 or pattern match engine slice 7 interrupt."]
    #[inline(always)]
    pub const fn set_gpio_int0_irq7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Standard counter/timer 2 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2_irq(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Standard counter/timer 2 interrupt."]
    #[inline(always)]
    pub const fn set_ctimer2_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Standard counter/timer 4 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4_irq(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Standard counter/timer 4 interrupt."]
    #[inline(always)]
    pub const fn set_ctimer4_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OS Event Timer and OS Event Timer Wakeup interrupts"]
    #[must_use]
    #[inline(always)]
    pub const fn os_event_timer_irq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OS Event Timer and OS Event Timer Wakeup interrupts"]
    #[inline(always)]
    pub const fn set_os_event_timer_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_reserved0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_reserved1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved2(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_reserved2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SDIO Controller interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn sdio_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SDIO Controller interrupt."]
    #[inline(always)]
    pub const fn set_sdio_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_reserved3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_reserved4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved5(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub const fn set_reserved5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "USB High Speed PHY Controller interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_phy_irq(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "USB High Speed PHY Controller interrupt."]
    #[inline(always)]
    pub const fn set_usb1_phy_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "USB High Speed Controller interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_irq(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB High Speed Controller interrupt."]
    #[inline(always)]
    pub const fn set_usb1_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "USB High Speed Controller Clock request interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_needclk(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "USB High Speed Controller Clock request interrupt."]
    #[inline(always)]
    pub const fn set_usb1_needclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Secure fault Hyper Visor call interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_hypervisor_call_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Secure fault Hyper Visor call interrupt."]
    #[inline(always)]
    pub const fn set_sec_hypervisor_call_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Secure Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_gpio_int0_irq0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Secure Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline(always)]
    pub const fn set_sec_gpio_int0_irq0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Secure Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_gpio_int0_irq1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Secure Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline(always)]
    pub const fn set_sec_gpio_int0_irq1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Programmable Look-Up Controller interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn plu_irq(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Programmable Look-Up Controller interrupt."]
    #[inline(always)]
    pub const fn set_plu_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Security Violation interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_irq(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation interrupt."]
    #[inline(always)]
    pub const fn set_sec_vio_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "HASH-AES interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn sha_irq(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "HASH-AES interrupt."]
    #[inline(always)]
    pub const fn set_sha_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "CASPER interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn casper_irq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "CASPER interrupt."]
    #[inline(always)]
    pub const fn set_casper_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "PUF interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn pufkey_irq(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PUF interrupt."]
    #[inline(always)]
    pub const fn set_pufkey_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Power Quad interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn pq_irq(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Power Quad interrupt."]
    #[inline(always)]
    pub const fn set_pq_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "System DMA 1 (Secure) interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn sdma1_irq(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "System DMA 1 (Secure) interrupt"]
    #[inline(always)]
    pub const fn set_sdma1_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "High Speed SPI interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn lspi_hs_irq(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "High Speed SPI interrupt"]
    #[inline(always)]
    pub const fn set_lspi_hs_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for SecCpuIntMask1 {
    #[inline(always)]
    fn default() -> SecCpuIntMask1 {
        SecCpuIntMask1(0)
    }
}
impl core::fmt::Debug for SecCpuIntMask1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCpuIntMask1")
            .field("gpio_int0_irq4", &self.gpio_int0_irq4())
            .field("gpio_int0_irq5", &self.gpio_int0_irq5())
            .field("gpio_int0_irq6", &self.gpio_int0_irq6())
            .field("gpio_int0_irq7", &self.gpio_int0_irq7())
            .field("ctimer2_irq", &self.ctimer2_irq())
            .field("ctimer4_irq", &self.ctimer4_irq())
            .field("os_event_timer_irq", &self.os_event_timer_irq())
            .field("reserved0", &self.reserved0())
            .field("reserved1", &self.reserved1())
            .field("reserved2", &self.reserved2())
            .field("sdio_irq", &self.sdio_irq())
            .field("reserved3", &self.reserved3())
            .field("reserved4", &self.reserved4())
            .field("reserved5", &self.reserved5())
            .field("usb1_phy_irq", &self.usb1_phy_irq())
            .field("usb1_irq", &self.usb1_irq())
            .field("usb1_needclk", &self.usb1_needclk())
            .field("sec_hypervisor_call_irq", &self.sec_hypervisor_call_irq())
            .field("sec_gpio_int0_irq0", &self.sec_gpio_int0_irq0())
            .field("sec_gpio_int0_irq1", &self.sec_gpio_int0_irq1())
            .field("plu_irq", &self.plu_irq())
            .field("sec_vio_irq", &self.sec_vio_irq())
            .field("sha_irq", &self.sha_irq())
            .field("casper_irq", &self.casper_irq())
            .field("pufkey_irq", &self.pufkey_irq())
            .field("pq_irq", &self.pq_irq())
            .field("sdma1_irq", &self.sdma1_irq())
            .field("lspi_hs_irq", &self.lspi_hs_irq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCpuIntMask1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCpuIntMask1 {{ gpio_int0_irq4: {=bool:?}, gpio_int0_irq5: {=bool:?}, gpio_int0_irq6: {=bool:?}, gpio_int0_irq7: {=bool:?}, ctimer2_irq: {=bool:?}, ctimer4_irq: {=bool:?}, os_event_timer_irq: {=bool:?}, reserved0: {=bool:?}, reserved1: {=bool:?}, reserved2: {=bool:?}, sdio_irq: {=bool:?}, reserved3: {=bool:?}, reserved4: {=bool:?}, reserved5: {=bool:?}, usb1_phy_irq: {=bool:?}, usb1_irq: {=bool:?}, usb1_needclk: {=bool:?}, sec_hypervisor_call_irq: {=bool:?}, sec_gpio_int0_irq0: {=bool:?}, sec_gpio_int0_irq1: {=bool:?}, plu_irq: {=bool:?}, sec_vio_irq: {=bool:?}, sha_irq: {=bool:?}, casper_irq: {=bool:?}, pufkey_irq: {=bool:?}, pq_irq: {=bool:?}, sdma1_irq: {=bool:?}, lspi_hs_irq: {=bool:?} }}",
            self.gpio_int0_irq4(),
            self.gpio_int0_irq5(),
            self.gpio_int0_irq6(),
            self.gpio_int0_irq7(),
            self.ctimer2_irq(),
            self.ctimer4_irq(),
            self.os_event_timer_irq(),
            self.reserved0(),
            self.reserved1(),
            self.reserved2(),
            self.sdio_irq(),
            self.reserved3(),
            self.reserved4(),
            self.reserved5(),
            self.usb1_phy_irq(),
            self.usb1_irq(),
            self.usb1_needclk(),
            self.sec_hypervisor_call_irq(),
            self.sec_gpio_int0_irq0(),
            self.sec_gpio_int0_irq1(),
            self.plu_irq(),
            self.sec_vio_irq(),
            self.sha_irq(),
            self.casper_irq(),
            self.pufkey_irq(),
            self.pq_irq(),
            self.sdma1_irq(),
            self.lspi_hs_irq()
        )
    }
}
#[doc = "Security access rules for AHB peripherals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlAhbPort10Slave0Rule(pub u32);
impl SecCtrlAhbPort10Slave0Rule {
    #[doc = "ADC"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_rule(&self) -> super::vals::AdcRule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::AdcRule::from_bits(val as u8)
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub const fn set_adc_rule(&mut self, val: super::vals::AdcRule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "USB Full Speed Host registers."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_fs_host_rule(&self) -> super::vals::UsbFsHostRule {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::UsbFsHostRule::from_bits(val as u8)
    }
    #[doc = "USB Full Speed Host registers."]
    #[inline(always)]
    pub const fn set_usb_fs_host_rule(&mut self, val: super::vals::UsbFsHostRule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "USB High speed host registers"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs_host_rule(&self) -> super::vals::UsbHsHostRule {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::UsbHsHostRule::from_bits(val as u8)
    }
    #[doc = "USB High speed host registers"]
    #[inline(always)]
    pub const fn set_usb_hs_host_rule(&mut self, val: super::vals::UsbHsHostRule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "SHA-2 crypto registers"]
    #[must_use]
    #[inline(always)]
    pub const fn hash_rule(&self) -> super::vals::HashRule {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::HashRule::from_bits(val as u8)
    }
    #[doc = "SHA-2 crypto registers"]
    #[inline(always)]
    pub const fn set_hash_rule(&mut self, val: super::vals::HashRule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "RSA/ECC crypto accelerator"]
    #[must_use]
    #[inline(always)]
    pub const fn casper_rule(&self) -> super::vals::CasperRule {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::CasperRule::from_bits(val as u8)
    }
    #[doc = "RSA/ECC crypto accelerator"]
    #[inline(always)]
    pub const fn set_casper_rule(&mut self, val: super::vals::CasperRule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Power Quad (CPU0 processor hardware accelerator)"]
    #[must_use]
    #[inline(always)]
    pub const fn pq_rule(&self) -> super::vals::PqRule {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::PqRule::from_bits(val as u8)
    }
    #[doc = "Power Quad (CPU0 processor hardware accelerator)"]
    #[inline(always)]
    pub const fn set_pq_rule(&mut self, val: super::vals::PqRule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "DMA Controller (Secure)"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_rule(&self) -> super::vals::Dma1Rule {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Dma1Rule::from_bits(val as u8)
    }
    #[doc = "DMA Controller (Secure)"]
    #[inline(always)]
    pub const fn set_dma1_rule(&mut self, val: super::vals::Dma1Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlAhbPort10Slave0Rule {
    #[inline(always)]
    fn default() -> SecCtrlAhbPort10Slave0Rule {
        SecCtrlAhbPort10Slave0Rule(0)
    }
}
impl core::fmt::Debug for SecCtrlAhbPort10Slave0Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlAhbPort10Slave0Rule")
            .field("adc_rule", &self.adc_rule())
            .field("usb_fs_host_rule", &self.usb_fs_host_rule())
            .field("usb_hs_host_rule", &self.usb_hs_host_rule())
            .field("hash_rule", &self.hash_rule())
            .field("casper_rule", &self.casper_rule())
            .field("pq_rule", &self.pq_rule())
            .field("dma1_rule", &self.dma1_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlAhbPort10Slave0Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlAhbPort10Slave0Rule {{ adc_rule: {:?}, usb_fs_host_rule: {:?}, usb_hs_host_rule: {:?}, hash_rule: {:?}, casper_rule: {:?}, pq_rule: {:?}, dma1_rule: {:?} }}",
            self.adc_rule(),
            self.usb_fs_host_rule(),
            self.usb_hs_host_rule(),
            self.hash_rule(),
            self.casper_rule(),
            self.pq_rule(),
            self.dma1_rule()
        )
    }
}
#[doc = "Security access rules for AHB peripherals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlAhbPort10Slave1Rule(pub u32);
impl SecCtrlAhbPort10Slave1Rule {
    #[doc = "Secure High Speed GPIO"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1_rule(&self) -> super::vals::Gpio1Rule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Gpio1Rule::from_bits(val as u8)
    }
    #[doc = "Secure High Speed GPIO"]
    #[inline(always)]
    pub const fn set_gpio1_rule(&mut self, val: super::vals::Gpio1Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "AHB Secure Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn ahb_sec_ctrl_rule(&self) -> super::vals::AhbSecCtrlRule {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::AhbSecCtrlRule::from_bits(val as u8)
    }
    #[doc = "AHB Secure Controller"]
    #[inline(always)]
    pub const fn set_ahb_sec_ctrl_rule(&mut self, val: super::vals::AhbSecCtrlRule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for SecCtrlAhbPort10Slave1Rule {
    #[inline(always)]
    fn default() -> SecCtrlAhbPort10Slave1Rule {
        SecCtrlAhbPort10Slave1Rule(0)
    }
}
impl core::fmt::Debug for SecCtrlAhbPort10Slave1Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlAhbPort10Slave1Rule")
            .field("gpio1_rule", &self.gpio1_rule())
            .field("ahb_sec_ctrl_rule", &self.ahb_sec_ctrl_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlAhbPort10Slave1Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlAhbPort10Slave1Rule {{ gpio1_rule: {:?}, ahb_sec_ctrl_rule: {:?} }}",
            self.gpio1_rule(),
            self.ahb_sec_ctrl_rule()
        )
    }
}
#[doc = "Security access rules for AHB peripherals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlAhbPort8Slave0Rule(pub u32);
impl SecCtrlAhbPort8Slave0Rule {
    #[doc = "DMA Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_rule(&self) -> super::vals::Dma0Rule {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Dma0Rule::from_bits(val as u8)
    }
    #[doc = "DMA Controller"]
    #[inline(always)]
    pub const fn set_dma0_rule(&mut self, val: super::vals::Dma0Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "USB Full-speed device"]
    #[must_use]
    #[inline(always)]
    pub const fn fs_usb_dev_rule(&self) -> super::vals::FsUsbDevRule {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::FsUsbDevRule::from_bits(val as u8)
    }
    #[doc = "USB Full-speed device"]
    #[inline(always)]
    pub const fn set_fs_usb_dev_rule(&mut self, val: super::vals::FsUsbDevRule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "SCTimer"]
    #[must_use]
    #[inline(always)]
    pub const fn sct_rule(&self) -> super::vals::SctRule {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SctRule::from_bits(val as u8)
    }
    #[doc = "SCTimer"]
    #[inline(always)]
    pub const fn set_sct_rule(&mut self, val: super::vals::SctRule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Flexcomm interface 0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm0_rule(&self) -> super::vals::Flexcomm0Rule {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Flexcomm0Rule::from_bits(val as u8)
    }
    #[doc = "Flexcomm interface 0"]
    #[inline(always)]
    pub const fn set_flexcomm0_rule(&mut self, val: super::vals::Flexcomm0Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Flexcomm interface 1"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm1_rule(&self) -> super::vals::Flexcomm1Rule {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Flexcomm1Rule::from_bits(val as u8)
    }
    #[doc = "Flexcomm interface 1"]
    #[inline(always)]
    pub const fn set_flexcomm1_rule(&mut self, val: super::vals::Flexcomm1Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlAhbPort8Slave0Rule {
    #[inline(always)]
    fn default() -> SecCtrlAhbPort8Slave0Rule {
        SecCtrlAhbPort8Slave0Rule(0)
    }
}
impl core::fmt::Debug for SecCtrlAhbPort8Slave0Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlAhbPort8Slave0Rule")
            .field("dma0_rule", &self.dma0_rule())
            .field("fs_usb_dev_rule", &self.fs_usb_dev_rule())
            .field("sct_rule", &self.sct_rule())
            .field("flexcomm0_rule", &self.flexcomm0_rule())
            .field("flexcomm1_rule", &self.flexcomm1_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlAhbPort8Slave0Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlAhbPort8Slave0Rule {{ dma0_rule: {:?}, fs_usb_dev_rule: {:?}, sct_rule: {:?}, flexcomm0_rule: {:?}, flexcomm1_rule: {:?} }}",
            self.dma0_rule(),
            self.fs_usb_dev_rule(),
            self.sct_rule(),
            self.flexcomm0_rule(),
            self.flexcomm1_rule()
        )
    }
}
#[doc = "Security access rules for AHB peripherals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlAhbPort8Slave1Rule(pub u32);
impl SecCtrlAhbPort8Slave1Rule {
    #[doc = "Flexcomm interface 2"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm2_rule(&self) -> super::vals::Flexcomm2Rule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexcomm2Rule::from_bits(val as u8)
    }
    #[doc = "Flexcomm interface 2"]
    #[inline(always)]
    pub const fn set_flexcomm2_rule(&mut self, val: super::vals::Flexcomm2Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Flexcomm interface 3"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm3_rule(&self) -> super::vals::Flexcomm3Rule {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Flexcomm3Rule::from_bits(val as u8)
    }
    #[doc = "Flexcomm interface 3"]
    #[inline(always)]
    pub const fn set_flexcomm3_rule(&mut self, val: super::vals::Flexcomm3Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Flexcomm interface 4"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm4_rule(&self) -> super::vals::Flexcomm4Rule {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Flexcomm4Rule::from_bits(val as u8)
    }
    #[doc = "Flexcomm interface 4"]
    #[inline(always)]
    pub const fn set_flexcomm4_rule(&mut self, val: super::vals::Flexcomm4Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Inter CPU communication Mailbox"]
    #[must_use]
    #[inline(always)]
    pub const fn mailbox_rule(&self) -> super::vals::MailboxRule {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::MailboxRule::from_bits(val as u8)
    }
    #[doc = "Inter CPU communication Mailbox"]
    #[inline(always)]
    pub const fn set_mailbox_rule(&mut self, val: super::vals::MailboxRule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "High Speed GPIO"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0_rule(&self) -> super::vals::Gpio0Rule {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Gpio0Rule::from_bits(val as u8)
    }
    #[doc = "High Speed GPIO"]
    #[inline(always)]
    pub const fn set_gpio0_rule(&mut self, val: super::vals::Gpio0Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for SecCtrlAhbPort8Slave1Rule {
    #[inline(always)]
    fn default() -> SecCtrlAhbPort8Slave1Rule {
        SecCtrlAhbPort8Slave1Rule(0)
    }
}
impl core::fmt::Debug for SecCtrlAhbPort8Slave1Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlAhbPort8Slave1Rule")
            .field("flexcomm2_rule", &self.flexcomm2_rule())
            .field("flexcomm3_rule", &self.flexcomm3_rule())
            .field("flexcomm4_rule", &self.flexcomm4_rule())
            .field("mailbox_rule", &self.mailbox_rule())
            .field("gpio0_rule", &self.gpio0_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlAhbPort8Slave1Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlAhbPort8Slave1Rule {{ flexcomm2_rule: {:?}, flexcomm3_rule: {:?}, flexcomm4_rule: {:?}, mailbox_rule: {:?}, gpio0_rule: {:?} }}",
            self.flexcomm2_rule(),
            self.flexcomm3_rule(),
            self.flexcomm4_rule(),
            self.mailbox_rule(),
            self.gpio0_rule()
        )
    }
}
#[doc = "Security access rules for AHB peripherals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlAhbPort9Slave0Rule(pub u32);
impl SecCtrlAhbPort9Slave0Rule {
    #[doc = "USB high Speed device registers"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs_dev_rule(&self) -> super::vals::UsbHsDevRule {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::UsbHsDevRule::from_bits(val as u8)
    }
    #[doc = "USB high Speed device registers"]
    #[inline(always)]
    pub const fn set_usb_hs_dev_rule(&mut self, val: super::vals::UsbHsDevRule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CRC engine"]
    #[must_use]
    #[inline(always)]
    pub const fn crc_rule(&self) -> super::vals::CrcRule {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::CrcRule::from_bits(val as u8)
    }
    #[doc = "CRC engine"]
    #[inline(always)]
    pub const fn set_crc_rule(&mut self, val: super::vals::CrcRule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Flexcomm interface 5"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm5_rule(&self) -> super::vals::Flexcomm5Rule {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Flexcomm5Rule::from_bits(val as u8)
    }
    #[doc = "Flexcomm interface 5"]
    #[inline(always)]
    pub const fn set_flexcomm5_rule(&mut self, val: super::vals::Flexcomm5Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Flexcomm interface 6"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm6_rule(&self) -> super::vals::Flexcomm6Rule {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Flexcomm6Rule::from_bits(val as u8)
    }
    #[doc = "Flexcomm interface 6"]
    #[inline(always)]
    pub const fn set_flexcomm6_rule(&mut self, val: super::vals::Flexcomm6Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlAhbPort9Slave0Rule {
    #[inline(always)]
    fn default() -> SecCtrlAhbPort9Slave0Rule {
        SecCtrlAhbPort9Slave0Rule(0)
    }
}
impl core::fmt::Debug for SecCtrlAhbPort9Slave0Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlAhbPort9Slave0Rule")
            .field("usb_hs_dev_rule", &self.usb_hs_dev_rule())
            .field("crc_rule", &self.crc_rule())
            .field("flexcomm5_rule", &self.flexcomm5_rule())
            .field("flexcomm6_rule", &self.flexcomm6_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlAhbPort9Slave0Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlAhbPort9Slave0Rule {{ usb_hs_dev_rule: {:?}, crc_rule: {:?}, flexcomm5_rule: {:?}, flexcomm6_rule: {:?} }}",
            self.usb_hs_dev_rule(),
            self.crc_rule(),
            self.flexcomm5_rule(),
            self.flexcomm6_rule()
        )
    }
}
#[doc = "Security access rules for AHB peripherals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlAhbPort9Slave1Rule(pub u32);
impl SecCtrlAhbPort9Slave1Rule {
    #[doc = "Flexcomm interface 7"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm7_rule(&self) -> super::vals::Flexcomm7Rule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexcomm7Rule::from_bits(val as u8)
    }
    #[doc = "Flexcomm interface 7"]
    #[inline(always)]
    pub const fn set_flexcomm7_rule(&mut self, val: super::vals::Flexcomm7Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SDMMC card interface"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio_rule(&self) -> super::vals::SdioRule {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SdioRule::from_bits(val as u8)
    }
    #[doc = "SDMMC card interface"]
    #[inline(always)]
    pub const fn set_sdio_rule(&mut self, val: super::vals::SdioRule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Debug mailbox (aka ISP-AP)"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_mailbox_rule(&self) -> super::vals::DbgMailboxRule {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::DbgMailboxRule::from_bits(val as u8)
    }
    #[doc = "Debug mailbox (aka ISP-AP)"]
    #[inline(always)]
    pub const fn set_dbg_mailbox_rule(&mut self, val: super::vals::DbgMailboxRule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "High Speed SPI"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_lspi_rule(&self) -> super::vals::HsLspiRule {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::HsLspiRule::from_bits(val as u8)
    }
    #[doc = "High Speed SPI"]
    #[inline(always)]
    pub const fn set_hs_lspi_rule(&mut self, val: super::vals::HsLspiRule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlAhbPort9Slave1Rule {
    #[inline(always)]
    fn default() -> SecCtrlAhbPort9Slave1Rule {
        SecCtrlAhbPort9Slave1Rule(0)
    }
}
impl core::fmt::Debug for SecCtrlAhbPort9Slave1Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlAhbPort9Slave1Rule")
            .field("flexcomm7_rule", &self.flexcomm7_rule())
            .field("sdio_rule", &self.sdio_rule())
            .field("dbg_mailbox_rule", &self.dbg_mailbox_rule())
            .field("hs_lspi_rule", &self.hs_lspi_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlAhbPort9Slave1Rule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlAhbPort9Slave1Rule {{ flexcomm7_rule: {:?}, sdio_rule: {:?}, dbg_mailbox_rule: {:?}, hs_lspi_rule: {:?} }}",
            self.flexcomm7_rule(),
            self.sdio_rule(),
            self.dbg_mailbox_rule(),
            self.hs_lspi_rule()
        )
    }
}
#[doc = "Security access rules for AHB_SEC_CTRL_AHB."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlAhbSecCtrlMemRule(pub u32);
impl SecCtrlAhbSecCtrlMemRule {
    #[doc = "Address space: 0x400A_0000 - 0x400A_CFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn ahb_sec_ctrl_sect_0_rule(&self) -> super::vals::AhbSecCtrlSect0Rule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::AhbSecCtrlSect0Rule::from_bits(val as u8)
    }
    #[doc = "Address space: 0x400A_0000 - 0x400A_CFFF"]
    #[inline(always)]
    pub const fn set_ahb_sec_ctrl_sect_0_rule(&mut self, val: super::vals::AhbSecCtrlSect0Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Address space: 0x400A_D000 - 0x400A_DFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn ahb_sec_ctrl_sect_1_rule(&self) -> super::vals::AhbSecCtrlSect1Rule {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::AhbSecCtrlSect1Rule::from_bits(val as u8)
    }
    #[doc = "Address space: 0x400A_D000 - 0x400A_DFFF"]
    #[inline(always)]
    pub const fn set_ahb_sec_ctrl_sect_1_rule(&mut self, val: super::vals::AhbSecCtrlSect1Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Address space: 0x400A_E000 - 0x400A_EFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn ahb_sec_ctrl_sect_2_rule(&self) -> super::vals::AhbSecCtrlSect2Rule {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::AhbSecCtrlSect2Rule::from_bits(val as u8)
    }
    #[doc = "Address space: 0x400A_E000 - 0x400A_EFFF"]
    #[inline(always)]
    pub const fn set_ahb_sec_ctrl_sect_2_rule(&mut self, val: super::vals::AhbSecCtrlSect2Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Address space: 0x400A_F000 - 0x400A_FFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn ahb_sec_ctrl_sect_3_rule(&self) -> super::vals::AhbSecCtrlSect3Rule {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::AhbSecCtrlSect3Rule::from_bits(val as u8)
    }
    #[doc = "Address space: 0x400A_F000 - 0x400A_FFFF"]
    #[inline(always)]
    pub const fn set_ahb_sec_ctrl_sect_3_rule(&mut self, val: super::vals::AhbSecCtrlSect3Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for SecCtrlAhbSecCtrlMemRule {
    #[inline(always)]
    fn default() -> SecCtrlAhbSecCtrlMemRule {
        SecCtrlAhbSecCtrlMemRule(0)
    }
}
impl core::fmt::Debug for SecCtrlAhbSecCtrlMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlAhbSecCtrlMemRule")
            .field("ahb_sec_ctrl_sect_0_rule", &self.ahb_sec_ctrl_sect_0_rule())
            .field("ahb_sec_ctrl_sect_1_rule", &self.ahb_sec_ctrl_sect_1_rule())
            .field("ahb_sec_ctrl_sect_2_rule", &self.ahb_sec_ctrl_sect_2_rule())
            .field("ahb_sec_ctrl_sect_3_rule", &self.ahb_sec_ctrl_sect_3_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlAhbSecCtrlMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlAhbSecCtrlMemRule {{ ahb_sec_ctrl_sect_0_rule: {:?}, ahb_sec_ctrl_sect_1_rule: {:?}, ahb_sec_ctrl_sect_2_rule: {:?}, ahb_sec_ctrl_sect_3_rule: {:?} }}",
            self.ahb_sec_ctrl_sect_0_rule(),
            self.ahb_sec_ctrl_sect_1_rule(),
            self.ahb_sec_ctrl_sect_2_rule(),
            self.ahb_sec_ctrl_sect_3_rule()
        )
    }
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlApbBridge0MemCtrl0(pub u32);
impl SecCtrlApbBridge0MemCtrl0 {
    #[doc = "System Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn syscon_rule(&self) -> super::vals::SysconRule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SysconRule::from_bits(val as u8)
    }
    #[doc = "System Configuration"]
    #[inline(always)]
    pub const fn set_syscon_rule(&mut self, val: super::vals::SysconRule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "I/O Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn iocon_rule(&self) -> super::vals::IoconRule {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::IoconRule::from_bits(val as u8)
    }
    #[doc = "I/O Configuration"]
    #[inline(always)]
    pub const fn set_iocon_rule(&mut self, val: super::vals::IoconRule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "GPIO input Interrupt 0"]
    #[must_use]
    #[inline(always)]
    pub const fn gint0_rule(&self) -> super::vals::Gint0Rule {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Gint0Rule::from_bits(val as u8)
    }
    #[doc = "GPIO input Interrupt 0"]
    #[inline(always)]
    pub const fn set_gint0_rule(&mut self, val: super::vals::Gint0Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "GPIO input Interrupt 1"]
    #[must_use]
    #[inline(always)]
    pub const fn gint1_rule(&self) -> super::vals::Gint1Rule {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Gint1Rule::from_bits(val as u8)
    }
    #[doc = "GPIO input Interrupt 1"]
    #[inline(always)]
    pub const fn set_gint1_rule(&mut self, val: super::vals::Gint1Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Pin Interrupt and Pattern match"]
    #[must_use]
    #[inline(always)]
    pub const fn pint_rule(&self) -> super::vals::PintRule {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::PintRule::from_bits(val as u8)
    }
    #[doc = "Pin Interrupt and Pattern match"]
    #[inline(always)]
    pub const fn set_pint_rule(&mut self, val: super::vals::PintRule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Secure Pin Interrupt and Pattern match"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_pint_rule(&self) -> super::vals::SecPintRule {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SecPintRule::from_bits(val as u8)
    }
    #[doc = "Secure Pin Interrupt and Pattern match"]
    #[inline(always)]
    pub const fn set_sec_pint_rule(&mut self, val: super::vals::SecPintRule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Peripheral input multiplexing"]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux_rule(&self) -> super::vals::InputmuxRule {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::InputmuxRule::from_bits(val as u8)
    }
    #[doc = "Peripheral input multiplexing"]
    #[inline(always)]
    pub const fn set_inputmux_rule(&mut self, val: super::vals::InputmuxRule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for SecCtrlApbBridge0MemCtrl0 {
    #[inline(always)]
    fn default() -> SecCtrlApbBridge0MemCtrl0 {
        SecCtrlApbBridge0MemCtrl0(0)
    }
}
impl core::fmt::Debug for SecCtrlApbBridge0MemCtrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlApbBridge0MemCtrl0")
            .field("syscon_rule", &self.syscon_rule())
            .field("iocon_rule", &self.iocon_rule())
            .field("gint0_rule", &self.gint0_rule())
            .field("gint1_rule", &self.gint1_rule())
            .field("pint_rule", &self.pint_rule())
            .field("sec_pint_rule", &self.sec_pint_rule())
            .field("inputmux_rule", &self.inputmux_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlApbBridge0MemCtrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlApbBridge0MemCtrl0 {{ syscon_rule: {:?}, iocon_rule: {:?}, gint0_rule: {:?}, gint1_rule: {:?}, pint_rule: {:?}, sec_pint_rule: {:?}, inputmux_rule: {:?} }}",
            self.syscon_rule(),
            self.iocon_rule(),
            self.gint0_rule(),
            self.gint1_rule(),
            self.pint_rule(),
            self.sec_pint_rule(),
            self.inputmux_rule()
        )
    }
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlApbBridge0MemCtrl1(pub u32);
impl SecCtrlApbBridge0MemCtrl1 {
    #[doc = "Standard counter/Timer 0"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0_rule(&self) -> super::vals::Ctimer0Rule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ctimer0Rule::from_bits(val as u8)
    }
    #[doc = "Standard counter/Timer 0"]
    #[inline(always)]
    pub const fn set_ctimer0_rule(&mut self, val: super::vals::Ctimer0Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Standard counter/Timer 1"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1_rule(&self) -> super::vals::Ctimer1Rule {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ctimer1Rule::from_bits(val as u8)
    }
    #[doc = "Standard counter/Timer 1"]
    #[inline(always)]
    pub const fn set_ctimer1_rule(&mut self, val: super::vals::Ctimer1Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Windiwed wtachdog Timer"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt_rule(&self) -> super::vals::WwdtRule {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::WwdtRule::from_bits(val as u8)
    }
    #[doc = "Windiwed wtachdog Timer"]
    #[inline(always)]
    pub const fn set_wwdt_rule(&mut self, val: super::vals::WwdtRule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Multi-rate Timer"]
    #[must_use]
    #[inline(always)]
    pub const fn mrt_rule(&self) -> super::vals::MrtRule {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::MrtRule::from_bits(val as u8)
    }
    #[doc = "Multi-rate Timer"]
    #[inline(always)]
    pub const fn set_mrt_rule(&mut self, val: super::vals::MrtRule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Micro-Timer"]
    #[must_use]
    #[inline(always)]
    pub const fn utick_rule(&self) -> super::vals::UtickRule {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::UtickRule::from_bits(val as u8)
    }
    #[doc = "Micro-Timer"]
    #[inline(always)]
    pub const fn set_utick_rule(&mut self, val: super::vals::UtickRule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for SecCtrlApbBridge0MemCtrl1 {
    #[inline(always)]
    fn default() -> SecCtrlApbBridge0MemCtrl1 {
        SecCtrlApbBridge0MemCtrl1(0)
    }
}
impl core::fmt::Debug for SecCtrlApbBridge0MemCtrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlApbBridge0MemCtrl1")
            .field("ctimer0_rule", &self.ctimer0_rule())
            .field("ctimer1_rule", &self.ctimer1_rule())
            .field("wwdt_rule", &self.wwdt_rule())
            .field("mrt_rule", &self.mrt_rule())
            .field("utick_rule", &self.utick_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlApbBridge0MemCtrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlApbBridge0MemCtrl1 {{ ctimer0_rule: {:?}, ctimer1_rule: {:?}, wwdt_rule: {:?}, mrt_rule: {:?}, utick_rule: {:?} }}",
            self.ctimer0_rule(),
            self.ctimer1_rule(),
            self.wwdt_rule(),
            self.mrt_rule(),
            self.utick_rule()
        )
    }
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlApbBridge0MemCtrl2(pub u32);
impl SecCtrlApbBridge0MemCtrl2 {
    #[doc = "Analog Modules controller"]
    #[must_use]
    #[inline(always)]
    pub const fn anactrl_rule(&self) -> super::vals::AnactrlRule {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::AnactrlRule::from_bits(val as u8)
    }
    #[doc = "Analog Modules controller"]
    #[inline(always)]
    pub const fn set_anactrl_rule(&mut self, val: super::vals::AnactrlRule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for SecCtrlApbBridge0MemCtrl2 {
    #[inline(always)]
    fn default() -> SecCtrlApbBridge0MemCtrl2 {
        SecCtrlApbBridge0MemCtrl2(0)
    }
}
impl core::fmt::Debug for SecCtrlApbBridge0MemCtrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlApbBridge0MemCtrl2")
            .field("anactrl_rule", &self.anactrl_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlApbBridge0MemCtrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlApbBridge0MemCtrl2 {{ anactrl_rule: {:?} }}",
            self.anactrl_rule()
        )
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlApbBridge1MemCtrl0(pub u32);
impl SecCtrlApbBridge1MemCtrl0 {
    #[doc = "Power Management Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn pmc_rule(&self) -> super::vals::PmcRule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::PmcRule::from_bits(val as u8)
    }
    #[doc = "Power Management Controller"]
    #[inline(always)]
    pub const fn set_pmc_rule(&mut self, val: super::vals::PmcRule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "System Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn sysctrl_rule(&self) -> super::vals::SysctrlRule {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SysctrlRule::from_bits(val as u8)
    }
    #[doc = "System Controller"]
    #[inline(always)]
    pub const fn set_sysctrl_rule(&mut self, val: super::vals::SysctrlRule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for SecCtrlApbBridge1MemCtrl0 {
    #[inline(always)]
    fn default() -> SecCtrlApbBridge1MemCtrl0 {
        SecCtrlApbBridge1MemCtrl0(0)
    }
}
impl core::fmt::Debug for SecCtrlApbBridge1MemCtrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlApbBridge1MemCtrl0")
            .field("pmc_rule", &self.pmc_rule())
            .field("sysctrl_rule", &self.sysctrl_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlApbBridge1MemCtrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlApbBridge1MemCtrl0 {{ pmc_rule: {:?}, sysctrl_rule: {:?} }}",
            self.pmc_rule(),
            self.sysctrl_rule()
        )
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlApbBridge1MemCtrl1(pub u32);
impl SecCtrlApbBridge1MemCtrl1 {
    #[doc = "Standard counter/Timer 2"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2_rule(&self) -> super::vals::Ctimer2Rule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ctimer2Rule::from_bits(val as u8)
    }
    #[doc = "Standard counter/Timer 2"]
    #[inline(always)]
    pub const fn set_ctimer2_rule(&mut self, val: super::vals::Ctimer2Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Standard counter/Timer 3"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3_rule(&self) -> super::vals::Ctimer3Rule {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ctimer3Rule::from_bits(val as u8)
    }
    #[doc = "Standard counter/Timer 3"]
    #[inline(always)]
    pub const fn set_ctimer3_rule(&mut self, val: super::vals::Ctimer3Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Standard counter/Timer 4"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4_rule(&self) -> super::vals::Ctimer4Rule {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ctimer4Rule::from_bits(val as u8)
    }
    #[doc = "Standard counter/Timer 4"]
    #[inline(always)]
    pub const fn set_ctimer4_rule(&mut self, val: super::vals::Ctimer4Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Real Time Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_rule(&self) -> super::vals::RtcRule {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::RtcRule::from_bits(val as u8)
    }
    #[doc = "Real Time Counter"]
    #[inline(always)]
    pub const fn set_rtc_rule(&mut self, val: super::vals::RtcRule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "OS Event Timer"]
    #[must_use]
    #[inline(always)]
    pub const fn osevent_rule(&self) -> super::vals::OseventRule {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::OseventRule::from_bits(val as u8)
    }
    #[doc = "OS Event Timer"]
    #[inline(always)]
    pub const fn set_osevent_rule(&mut self, val: super::vals::OseventRule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for SecCtrlApbBridge1MemCtrl1 {
    #[inline(always)]
    fn default() -> SecCtrlApbBridge1MemCtrl1 {
        SecCtrlApbBridge1MemCtrl1(0)
    }
}
impl core::fmt::Debug for SecCtrlApbBridge1MemCtrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlApbBridge1MemCtrl1")
            .field("ctimer2_rule", &self.ctimer2_rule())
            .field("ctimer3_rule", &self.ctimer3_rule())
            .field("ctimer4_rule", &self.ctimer4_rule())
            .field("rtc_rule", &self.rtc_rule())
            .field("osevent_rule", &self.osevent_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlApbBridge1MemCtrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlApbBridge1MemCtrl1 {{ ctimer2_rule: {:?}, ctimer3_rule: {:?}, ctimer4_rule: {:?}, rtc_rule: {:?}, osevent_rule: {:?} }}",
            self.ctimer2_rule(),
            self.ctimer3_rule(),
            self.ctimer4_rule(),
            self.rtc_rule(),
            self.osevent_rule()
        )
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlApbBridge1MemCtrl2(pub u32);
impl SecCtrlApbBridge1MemCtrl2 {
    #[doc = "Flash Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_ctrl_rule(&self) -> super::vals::FlashCtrlRule {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::FlashCtrlRule::from_bits(val as u8)
    }
    #[doc = "Flash Controller"]
    #[inline(always)]
    pub const fn set_flash_ctrl_rule(&mut self, val: super::vals::FlashCtrlRule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Prince"]
    #[must_use]
    #[inline(always)]
    pub const fn prince_rule(&self) -> super::vals::PrinceRule {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::PrinceRule::from_bits(val as u8)
    }
    #[doc = "Prince"]
    #[inline(always)]
    pub const fn set_prince_rule(&mut self, val: super::vals::PrinceRule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for SecCtrlApbBridge1MemCtrl2 {
    #[inline(always)]
    fn default() -> SecCtrlApbBridge1MemCtrl2 {
        SecCtrlApbBridge1MemCtrl2(0)
    }
}
impl core::fmt::Debug for SecCtrlApbBridge1MemCtrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlApbBridge1MemCtrl2")
            .field("flash_ctrl_rule", &self.flash_ctrl_rule())
            .field("prince_rule", &self.prince_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlApbBridge1MemCtrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlApbBridge1MemCtrl2 {{ flash_ctrl_rule: {:?}, prince_rule: {:?} }}",
            self.flash_ctrl_rule(),
            self.prince_rule()
        )
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlApbBridge1MemCtrl3(pub u32);
impl SecCtrlApbBridge1MemCtrl3 {
    #[doc = "USB High Speed Phy controller"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhphy_rule(&self) -> super::vals::UsbhphyRule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::UsbhphyRule::from_bits(val as u8)
    }
    #[doc = "USB High Speed Phy controller"]
    #[inline(always)]
    pub const fn set_usbhphy_rule(&mut self, val: super::vals::UsbhphyRule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "True Random Number Generator"]
    #[must_use]
    #[inline(always)]
    pub const fn rng_rule(&self) -> super::vals::RngRule {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RngRule::from_bits(val as u8)
    }
    #[doc = "True Random Number Generator"]
    #[inline(always)]
    pub const fn set_rng_rule(&mut self, val: super::vals::RngRule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "PUF"]
    #[must_use]
    #[inline(always)]
    pub const fn puf_rule(&self) -> super::vals::PufRule {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::PufRule::from_bits(val as u8)
    }
    #[doc = "PUF"]
    #[inline(always)]
    pub const fn set_puf_rule(&mut self, val: super::vals::PufRule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Programmable Look-Up logic"]
    #[must_use]
    #[inline(always)]
    pub const fn plu_rule(&self) -> super::vals::PluRule {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::PluRule::from_bits(val as u8)
    }
    #[doc = "Programmable Look-Up logic"]
    #[inline(always)]
    pub const fn set_plu_rule(&mut self, val: super::vals::PluRule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for SecCtrlApbBridge1MemCtrl3 {
    #[inline(always)]
    fn default() -> SecCtrlApbBridge1MemCtrl3 {
        SecCtrlApbBridge1MemCtrl3(0)
    }
}
impl core::fmt::Debug for SecCtrlApbBridge1MemCtrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlApbBridge1MemCtrl3")
            .field("usbhphy_rule", &self.usbhphy_rule())
            .field("rng_rule", &self.rng_rule())
            .field("puf_rule", &self.puf_rule())
            .field("plu_rule", &self.plu_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlApbBridge1MemCtrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlApbBridge1MemCtrl3 {{ usbhphy_rule: {:?}, rng_rule: {:?}, puf_rule: {:?}, plu_rule: {:?} }}",
            self.usbhphy_rule(),
            self.rng_rule(),
            self.puf_rule(),
            self.plu_rule()
        )
    }
}
#[doc = "Security access rules for both APB Bridges slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlApbBridgeSlaveRule(pub u32);
impl SecCtrlApbBridgeSlaveRule {
    #[doc = "Security access rules for the whole APB Bridge 0"]
    #[must_use]
    #[inline(always)]
    pub const fn apbbridge0_rule(&self) -> super::vals::Apbbridge0Rule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Apbbridge0Rule::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole APB Bridge 0"]
    #[inline(always)]
    pub const fn set_apbbridge0_rule(&mut self, val: super::vals::Apbbridge0Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Security access rules for the whole APB Bridge 1"]
    #[must_use]
    #[inline(always)]
    pub const fn apbbridge1_rule(&self) -> super::vals::Apbbridge1Rule {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Apbbridge1Rule::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole APB Bridge 1"]
    #[inline(always)]
    pub const fn set_apbbridge1_rule(&mut self, val: super::vals::Apbbridge1Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for SecCtrlApbBridgeSlaveRule {
    #[inline(always)]
    fn default() -> SecCtrlApbBridgeSlaveRule {
        SecCtrlApbBridgeSlaveRule(0)
    }
}
impl core::fmt::Debug for SecCtrlApbBridgeSlaveRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlApbBridgeSlaveRule")
            .field("apbbridge0_rule", &self.apbbridge0_rule())
            .field("apbbridge1_rule", &self.apbbridge1_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlApbBridgeSlaveRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlApbBridgeSlaveRule {{ apbbridge0_rule: {:?}, apbbridge1_rule: {:?} }}",
            self.apbbridge0_rule(),
            self.apbbridge1_rule()
        )
    }
}
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlFlashMemRule0(pub u32);
impl SecCtrlFlashMemRule0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::SecCtrlFlashMemRule0Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecCtrlFlashMemRule0Rule0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::SecCtrlFlashMemRule0Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::SecCtrlFlashMemRule0Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecCtrlFlashMemRule0Rule1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::SecCtrlFlashMemRule0Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::SecCtrlFlashMemRule0Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCtrlFlashMemRule0Rule2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::SecCtrlFlashMemRule0Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::SecCtrlFlashMemRule0Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecCtrlFlashMemRule0Rule3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::SecCtrlFlashMemRule0Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::SecCtrlFlashMemRule0Rule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SecCtrlFlashMemRule0Rule4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::SecCtrlFlashMemRule0Rule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::SecCtrlFlashMemRule0Rule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SecCtrlFlashMemRule0Rule5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::SecCtrlFlashMemRule0Rule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::SecCtrlFlashMemRule0Rule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SecCtrlFlashMemRule0Rule6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::SecCtrlFlashMemRule0Rule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::SecCtrlFlashMemRule0Rule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SecCtrlFlashMemRule0Rule7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::SecCtrlFlashMemRule0Rule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlFlashMemRule0 {
    #[inline(always)]
    fn default() -> SecCtrlFlashMemRule0 {
        SecCtrlFlashMemRule0(0)
    }
}
impl core::fmt::Debug for SecCtrlFlashMemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlFlashMemRule0")
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
impl defmt::Format for SecCtrlFlashMemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlFlashMemRule0 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
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
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlFlashMemRule1(pub u32);
impl SecCtrlFlashMemRule1 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::SecCtrlFlashMemRule1Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecCtrlFlashMemRule1Rule0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::SecCtrlFlashMemRule1Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::SecCtrlFlashMemRule1Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecCtrlFlashMemRule1Rule1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::SecCtrlFlashMemRule1Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::SecCtrlFlashMemRule1Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCtrlFlashMemRule1Rule2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::SecCtrlFlashMemRule1Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::SecCtrlFlashMemRule1Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecCtrlFlashMemRule1Rule3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::SecCtrlFlashMemRule1Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::SecCtrlFlashMemRule1Rule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SecCtrlFlashMemRule1Rule4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::SecCtrlFlashMemRule1Rule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::SecCtrlFlashMemRule1Rule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SecCtrlFlashMemRule1Rule5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::SecCtrlFlashMemRule1Rule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::SecCtrlFlashMemRule1Rule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SecCtrlFlashMemRule1Rule6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::SecCtrlFlashMemRule1Rule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::SecCtrlFlashMemRule1Rule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SecCtrlFlashMemRule1Rule7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::SecCtrlFlashMemRule1Rule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlFlashMemRule1 {
    #[inline(always)]
    fn default() -> SecCtrlFlashMemRule1 {
        SecCtrlFlashMemRule1(0)
    }
}
impl core::fmt::Debug for SecCtrlFlashMemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlFlashMemRule1")
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
impl defmt::Format for SecCtrlFlashMemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlFlashMemRule1 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
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
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlFlashMemRule2(pub u32);
impl SecCtrlFlashMemRule2 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::SecCtrlFlashMemRule2Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecCtrlFlashMemRule2Rule0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::SecCtrlFlashMemRule2Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::SecCtrlFlashMemRule2Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecCtrlFlashMemRule2Rule1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::SecCtrlFlashMemRule2Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::SecCtrlFlashMemRule2Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCtrlFlashMemRule2Rule2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::SecCtrlFlashMemRule2Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::SecCtrlFlashMemRule2Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecCtrlFlashMemRule2Rule3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::SecCtrlFlashMemRule2Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::SecCtrlFlashMemRule2Rule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SecCtrlFlashMemRule2Rule4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::SecCtrlFlashMemRule2Rule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::SecCtrlFlashMemRule2Rule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SecCtrlFlashMemRule2Rule5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::SecCtrlFlashMemRule2Rule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::SecCtrlFlashMemRule2Rule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SecCtrlFlashMemRule2Rule6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::SecCtrlFlashMemRule2Rule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::SecCtrlFlashMemRule2Rule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SecCtrlFlashMemRule2Rule7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::SecCtrlFlashMemRule2Rule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlFlashMemRule2 {
    #[inline(always)]
    fn default() -> SecCtrlFlashMemRule2 {
        SecCtrlFlashMemRule2(0)
    }
}
impl core::fmt::Debug for SecCtrlFlashMemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlFlashMemRule2")
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
impl defmt::Format for SecCtrlFlashMemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlFlashMemRule2 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
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
#[doc = "Security access rules for Flash and ROM slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlFlashRomSlaveRule(pub u32);
impl SecCtrlFlashRomSlaveRule {
    #[doc = "Security access rules for the whole FLASH : 0x0000_0000 - 0x0009_FFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_rule(&self) -> super::vals::FlashRule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::FlashRule::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole FLASH : 0x0000_0000 - 0x0009_FFFF"]
    #[inline(always)]
    pub const fn set_flash_rule(&mut self, val: super::vals::FlashRule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Security access rules for the whole ROM : 0x0300_0000 - 0x0301_FFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn rom_rule(&self) -> super::vals::RomRule {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::RomRule::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole ROM : 0x0300_0000 - 0x0301_FFFF"]
    #[inline(always)]
    pub const fn set_rom_rule(&mut self, val: super::vals::RomRule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for SecCtrlFlashRomSlaveRule {
    #[inline(always)]
    fn default() -> SecCtrlFlashRomSlaveRule {
        SecCtrlFlashRomSlaveRule(0)
    }
}
impl core::fmt::Debug for SecCtrlFlashRomSlaveRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlFlashRomSlaveRule")
            .field("flash_rule", &self.flash_rule())
            .field("rom_rule", &self.rom_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlFlashRomSlaveRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlFlashRomSlaveRule {{ flash_rule: {:?}, rom_rule: {:?} }}",
            self.flash_rule(),
            self.rom_rule()
        )
    }
}
#[doc = "Security access rules for RAM0 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRam0MemRule0(pub u32);
impl SecCtrlRam0MemRule0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::SecCtrlRam0MemRule0Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecCtrlRam0MemRule0Rule0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::SecCtrlRam0MemRule0Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::SecCtrlRam0MemRule0Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecCtrlRam0MemRule0Rule1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::SecCtrlRam0MemRule0Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::SecCtrlRam0MemRule0Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCtrlRam0MemRule0Rule2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::SecCtrlRam0MemRule0Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::SecCtrlRam0MemRule0Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecCtrlRam0MemRule0Rule3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::SecCtrlRam0MemRule0Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::SecCtrlRam0MemRule0Rule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SecCtrlRam0MemRule0Rule4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::SecCtrlRam0MemRule0Rule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::SecCtrlRam0MemRule0Rule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SecCtrlRam0MemRule0Rule5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::SecCtrlRam0MemRule0Rule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::SecCtrlRam0MemRule0Rule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SecCtrlRam0MemRule0Rule6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::SecCtrlRam0MemRule0Rule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::SecCtrlRam0MemRule0Rule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SecCtrlRam0MemRule0Rule7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::SecCtrlRam0MemRule0Rule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlRam0MemRule0 {
    #[inline(always)]
    fn default() -> SecCtrlRam0MemRule0 {
        SecCtrlRam0MemRule0(0)
    }
}
impl core::fmt::Debug for SecCtrlRam0MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRam0MemRule0")
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
impl defmt::Format for SecCtrlRam0MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRam0MemRule0 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
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
#[doc = "Security access rules for RAM0 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRam0MemRule1(pub u32);
impl SecCtrlRam0MemRule1 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::SecCtrlRam0MemRule1Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecCtrlRam0MemRule1Rule0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::SecCtrlRam0MemRule1Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::SecCtrlRam0MemRule1Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecCtrlRam0MemRule1Rule1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::SecCtrlRam0MemRule1Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::SecCtrlRam0MemRule1Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCtrlRam0MemRule1Rule2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::SecCtrlRam0MemRule1Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::SecCtrlRam0MemRule1Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecCtrlRam0MemRule1Rule3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::SecCtrlRam0MemRule1Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::SecCtrlRam0MemRule1Rule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SecCtrlRam0MemRule1Rule4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::SecCtrlRam0MemRule1Rule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::SecCtrlRam0MemRule1Rule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SecCtrlRam0MemRule1Rule5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::SecCtrlRam0MemRule1Rule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::SecCtrlRam0MemRule1Rule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SecCtrlRam0MemRule1Rule6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::SecCtrlRam0MemRule1Rule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::SecCtrlRam0MemRule1Rule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SecCtrlRam0MemRule1Rule7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::SecCtrlRam0MemRule1Rule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlRam0MemRule1 {
    #[inline(always)]
    fn default() -> SecCtrlRam0MemRule1 {
        SecCtrlRam0MemRule1(0)
    }
}
impl core::fmt::Debug for SecCtrlRam0MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRam0MemRule1")
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
impl defmt::Format for SecCtrlRam0MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRam0MemRule1 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
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
#[doc = "Security access rules for RAM0 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRam0SlaveRule(pub u32);
impl SecCtrlRam0SlaveRule {
    #[doc = "Security access rules for the whole RAM0 : 0x2000_0000 - 0x2000_FFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn ram0_rule(&self) -> super::vals::Ram0Rule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram0Rule::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole RAM0 : 0x2000_0000 - 0x2000_FFFF"]
    #[inline(always)]
    pub const fn set_ram0_rule(&mut self, val: super::vals::Ram0Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SecCtrlRam0SlaveRule {
    #[inline(always)]
    fn default() -> SecCtrlRam0SlaveRule {
        SecCtrlRam0SlaveRule(0)
    }
}
impl core::fmt::Debug for SecCtrlRam0SlaveRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRam0SlaveRule")
            .field("ram0_rule", &self.ram0_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlRam0SlaveRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRam0SlaveRule {{ ram0_rule: {:?} }}",
            self.ram0_rule()
        )
    }
}
#[doc = "Security access rules for RAM1 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRam1MemRule0(pub u32);
impl SecCtrlRam1MemRule0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::SecCtrlRam1MemRule0Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecCtrlRam1MemRule0Rule0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::SecCtrlRam1MemRule0Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::SecCtrlRam1MemRule0Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecCtrlRam1MemRule0Rule1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::SecCtrlRam1MemRule0Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::SecCtrlRam1MemRule0Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCtrlRam1MemRule0Rule2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::SecCtrlRam1MemRule0Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::SecCtrlRam1MemRule0Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecCtrlRam1MemRule0Rule3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::SecCtrlRam1MemRule0Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::SecCtrlRam1MemRule0Rule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SecCtrlRam1MemRule0Rule4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::SecCtrlRam1MemRule0Rule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::SecCtrlRam1MemRule0Rule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SecCtrlRam1MemRule0Rule5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::SecCtrlRam1MemRule0Rule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::SecCtrlRam1MemRule0Rule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SecCtrlRam1MemRule0Rule6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::SecCtrlRam1MemRule0Rule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::SecCtrlRam1MemRule0Rule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SecCtrlRam1MemRule0Rule7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::SecCtrlRam1MemRule0Rule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlRam1MemRule0 {
    #[inline(always)]
    fn default() -> SecCtrlRam1MemRule0 {
        SecCtrlRam1MemRule0(0)
    }
}
impl core::fmt::Debug for SecCtrlRam1MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRam1MemRule0")
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
impl defmt::Format for SecCtrlRam1MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRam1MemRule0 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
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
#[doc = "Security access rules for RAM1 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRam1MemRule1(pub u32);
impl SecCtrlRam1MemRule1 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::SecCtrlRam1MemRule1Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecCtrlRam1MemRule1Rule0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::SecCtrlRam1MemRule1Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::SecCtrlRam1MemRule1Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecCtrlRam1MemRule1Rule1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::SecCtrlRam1MemRule1Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::SecCtrlRam1MemRule1Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCtrlRam1MemRule1Rule2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::SecCtrlRam1MemRule1Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::SecCtrlRam1MemRule1Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecCtrlRam1MemRule1Rule3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::SecCtrlRam1MemRule1Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::SecCtrlRam1MemRule1Rule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SecCtrlRam1MemRule1Rule4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::SecCtrlRam1MemRule1Rule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::SecCtrlRam1MemRule1Rule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SecCtrlRam1MemRule1Rule5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::SecCtrlRam1MemRule1Rule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::SecCtrlRam1MemRule1Rule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SecCtrlRam1MemRule1Rule6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::SecCtrlRam1MemRule1Rule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::SecCtrlRam1MemRule1Rule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SecCtrlRam1MemRule1Rule7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::SecCtrlRam1MemRule1Rule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlRam1MemRule1 {
    #[inline(always)]
    fn default() -> SecCtrlRam1MemRule1 {
        SecCtrlRam1MemRule1(0)
    }
}
impl core::fmt::Debug for SecCtrlRam1MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRam1MemRule1")
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
impl defmt::Format for SecCtrlRam1MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRam1MemRule1 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
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
#[doc = "Security access rules for RAM1 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRam1SlaveRule(pub u32);
impl SecCtrlRam1SlaveRule {
    #[doc = "Security access rules for the whole RAM1 : 0x2001_0000 - 0x2001_FFFF\" name=\"0"]
    #[must_use]
    #[inline(always)]
    pub const fn ram1_rule(&self) -> super::vals::Ram1Rule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram1Rule::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole RAM1 : 0x2001_0000 - 0x2001_FFFF\" name=\"0"]
    #[inline(always)]
    pub const fn set_ram1_rule(&mut self, val: super::vals::Ram1Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SecCtrlRam1SlaveRule {
    #[inline(always)]
    fn default() -> SecCtrlRam1SlaveRule {
        SecCtrlRam1SlaveRule(0)
    }
}
impl core::fmt::Debug for SecCtrlRam1SlaveRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRam1SlaveRule")
            .field("ram1_rule", &self.ram1_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlRam1SlaveRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRam1SlaveRule {{ ram1_rule: {:?} }}",
            self.ram1_rule()
        )
    }
}
#[doc = "Security access rules for RAM2 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRam2MemRule0(pub u32);
impl SecCtrlRam2MemRule0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::SecCtrlRam2MemRule0Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecCtrlRam2MemRule0Rule0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::SecCtrlRam2MemRule0Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::SecCtrlRam2MemRule0Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecCtrlRam2MemRule0Rule1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::SecCtrlRam2MemRule0Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::SecCtrlRam2MemRule0Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCtrlRam2MemRule0Rule2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::SecCtrlRam2MemRule0Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::SecCtrlRam2MemRule0Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecCtrlRam2MemRule0Rule3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::SecCtrlRam2MemRule0Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::SecCtrlRam2MemRule0Rule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SecCtrlRam2MemRule0Rule4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::SecCtrlRam2MemRule0Rule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::SecCtrlRam2MemRule0Rule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SecCtrlRam2MemRule0Rule5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::SecCtrlRam2MemRule0Rule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::SecCtrlRam2MemRule0Rule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SecCtrlRam2MemRule0Rule6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::SecCtrlRam2MemRule0Rule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::SecCtrlRam2MemRule0Rule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SecCtrlRam2MemRule0Rule7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::SecCtrlRam2MemRule0Rule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlRam2MemRule0 {
    #[inline(always)]
    fn default() -> SecCtrlRam2MemRule0 {
        SecCtrlRam2MemRule0(0)
    }
}
impl core::fmt::Debug for SecCtrlRam2MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRam2MemRule0")
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
impl defmt::Format for SecCtrlRam2MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRam2MemRule0 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
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
#[doc = "Security access rules for RAM2 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRam2MemRule1(pub u32);
impl SecCtrlRam2MemRule1 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::SecCtrlRam2MemRule1Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecCtrlRam2MemRule1Rule0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::SecCtrlRam2MemRule1Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::SecCtrlRam2MemRule1Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecCtrlRam2MemRule1Rule1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::SecCtrlRam2MemRule1Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::SecCtrlRam2MemRule1Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCtrlRam2MemRule1Rule2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::SecCtrlRam2MemRule1Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::SecCtrlRam2MemRule1Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecCtrlRam2MemRule1Rule3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::SecCtrlRam2MemRule1Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::SecCtrlRam2MemRule1Rule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SecCtrlRam2MemRule1Rule4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::SecCtrlRam2MemRule1Rule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::SecCtrlRam2MemRule1Rule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SecCtrlRam2MemRule1Rule5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::SecCtrlRam2MemRule1Rule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::SecCtrlRam2MemRule1Rule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SecCtrlRam2MemRule1Rule6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::SecCtrlRam2MemRule1Rule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::SecCtrlRam2MemRule1Rule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SecCtrlRam2MemRule1Rule7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::SecCtrlRam2MemRule1Rule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlRam2MemRule1 {
    #[inline(always)]
    fn default() -> SecCtrlRam2MemRule1 {
        SecCtrlRam2MemRule1(0)
    }
}
impl core::fmt::Debug for SecCtrlRam2MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRam2MemRule1")
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
impl defmt::Format for SecCtrlRam2MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRam2MemRule1 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
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
#[doc = "Security access rules for RAM2 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRam2SlaveRule(pub u32);
impl SecCtrlRam2SlaveRule {
    #[doc = "Security access rules for the whole RAM2 : 0x2002_0000 - 0x2002_FFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn ram2_rule(&self) -> super::vals::Ram2Rule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram2Rule::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole RAM2 : 0x2002_0000 - 0x2002_FFFF"]
    #[inline(always)]
    pub const fn set_ram2_rule(&mut self, val: super::vals::Ram2Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SecCtrlRam2SlaveRule {
    #[inline(always)]
    fn default() -> SecCtrlRam2SlaveRule {
        SecCtrlRam2SlaveRule(0)
    }
}
impl core::fmt::Debug for SecCtrlRam2SlaveRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRam2SlaveRule")
            .field("ram2_rule", &self.ram2_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlRam2SlaveRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRam2SlaveRule {{ ram2_rule: {:?} }}",
            self.ram2_rule()
        )
    }
}
#[doc = "Security access rules for RAM3 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRam3MemRule0(pub u32);
impl SecCtrlRam3MemRule0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::SecCtrlRam3MemRule0Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecCtrlRam3MemRule0Rule0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::SecCtrlRam3MemRule0Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::SecCtrlRam3MemRule0Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecCtrlRam3MemRule0Rule1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::SecCtrlRam3MemRule0Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::SecCtrlRam3MemRule0Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCtrlRam3MemRule0Rule2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::SecCtrlRam3MemRule0Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::SecCtrlRam3MemRule0Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecCtrlRam3MemRule0Rule3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::SecCtrlRam3MemRule0Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::SecCtrlRam3MemRule0Rule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SecCtrlRam3MemRule0Rule4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::SecCtrlRam3MemRule0Rule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::SecCtrlRam3MemRule0Rule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SecCtrlRam3MemRule0Rule5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::SecCtrlRam3MemRule0Rule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::SecCtrlRam3MemRule0Rule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SecCtrlRam3MemRule0Rule6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::SecCtrlRam3MemRule0Rule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::SecCtrlRam3MemRule0Rule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SecCtrlRam3MemRule0Rule7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::SecCtrlRam3MemRule0Rule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlRam3MemRule0 {
    #[inline(always)]
    fn default() -> SecCtrlRam3MemRule0 {
        SecCtrlRam3MemRule0(0)
    }
}
impl core::fmt::Debug for SecCtrlRam3MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRam3MemRule0")
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
impl defmt::Format for SecCtrlRam3MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRam3MemRule0 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
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
#[doc = "Security access rules for RAM3 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRam3MemRule1(pub u32);
impl SecCtrlRam3MemRule1 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::SecCtrlRam3MemRule1Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecCtrlRam3MemRule1Rule0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::SecCtrlRam3MemRule1Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::SecCtrlRam3MemRule1Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecCtrlRam3MemRule1Rule1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::SecCtrlRam3MemRule1Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::SecCtrlRam3MemRule1Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCtrlRam3MemRule1Rule2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::SecCtrlRam3MemRule1Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::SecCtrlRam3MemRule1Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecCtrlRam3MemRule1Rule3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::SecCtrlRam3MemRule1Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::SecCtrlRam3MemRule1Rule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SecCtrlRam3MemRule1Rule4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::SecCtrlRam3MemRule1Rule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::SecCtrlRam3MemRule1Rule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SecCtrlRam3MemRule1Rule5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::SecCtrlRam3MemRule1Rule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::SecCtrlRam3MemRule1Rule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SecCtrlRam3MemRule1Rule6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::SecCtrlRam3MemRule1Rule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::SecCtrlRam3MemRule1Rule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SecCtrlRam3MemRule1Rule7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::SecCtrlRam3MemRule1Rule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlRam3MemRule1 {
    #[inline(always)]
    fn default() -> SecCtrlRam3MemRule1 {
        SecCtrlRam3MemRule1(0)
    }
}
impl core::fmt::Debug for SecCtrlRam3MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRam3MemRule1")
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
impl defmt::Format for SecCtrlRam3MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRam3MemRule1 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
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
#[doc = "Security access rules for RAM3 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRam3SlaveRule(pub u32);
impl SecCtrlRam3SlaveRule {
    #[doc = "Security access rules for the whole RAM3: 0x2003_0000 - 0x2003_FFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn ram3_rule(&self) -> super::vals::Ram3Rule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram3Rule::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole RAM3: 0x2003_0000 - 0x2003_FFFF"]
    #[inline(always)]
    pub const fn set_ram3_rule(&mut self, val: super::vals::Ram3Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SecCtrlRam3SlaveRule {
    #[inline(always)]
    fn default() -> SecCtrlRam3SlaveRule {
        SecCtrlRam3SlaveRule(0)
    }
}
impl core::fmt::Debug for SecCtrlRam3SlaveRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRam3SlaveRule")
            .field("ram3_rule", &self.ram3_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlRam3SlaveRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRam3SlaveRule {{ ram3_rule: {:?} }}",
            self.ram3_rule()
        )
    }
}
#[doc = "Security access rules for RAM4 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRam4MemRule0(pub u32);
impl SecCtrlRam4MemRule0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::SecCtrlRam4MemRule0Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecCtrlRam4MemRule0Rule0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::SecCtrlRam4MemRule0Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::SecCtrlRam4MemRule0Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecCtrlRam4MemRule0Rule1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::SecCtrlRam4MemRule0Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::SecCtrlRam4MemRule0Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCtrlRam4MemRule0Rule2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::SecCtrlRam4MemRule0Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::SecCtrlRam4MemRule0Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecCtrlRam4MemRule0Rule3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::SecCtrlRam4MemRule0Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for SecCtrlRam4MemRule0 {
    #[inline(always)]
    fn default() -> SecCtrlRam4MemRule0 {
        SecCtrlRam4MemRule0(0)
    }
}
impl core::fmt::Debug for SecCtrlRam4MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRam4MemRule0")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlRam4MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRam4MemRule0 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3()
        )
    }
}
#[doc = "Security access rules for RAM4 slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRam4SlaveRule(pub u32);
impl SecCtrlRam4SlaveRule {
    #[doc = "Security access rules for the whole RAM4 : 0x2004_0000 - 0x2004_3FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn ram4_rule(&self) -> super::vals::Ram4Rule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ram4Rule::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole RAM4 : 0x2004_0000 - 0x2004_3FFF"]
    #[inline(always)]
    pub const fn set_ram4_rule(&mut self, val: super::vals::Ram4Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SecCtrlRam4SlaveRule {
    #[inline(always)]
    fn default() -> SecCtrlRam4SlaveRule {
        SecCtrlRam4SlaveRule(0)
    }
}
impl core::fmt::Debug for SecCtrlRam4SlaveRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRam4SlaveRule")
            .field("ram4_rule", &self.ram4_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlRam4SlaveRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRam4SlaveRule {{ ram4_rule: {:?} }}",
            self.ram4_rule()
        )
    }
}
#[doc = "Security access rules for RAMX slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRamxMemRule0(pub u32);
impl SecCtrlRamxMemRule0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::SecCtrlRamxMemRule0Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecCtrlRamxMemRule0Rule0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::SecCtrlRamxMemRule0Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::SecCtrlRamxMemRule0Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecCtrlRamxMemRule0Rule1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::SecCtrlRamxMemRule0Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::SecCtrlRamxMemRule0Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCtrlRamxMemRule0Rule2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::SecCtrlRamxMemRule0Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::SecCtrlRamxMemRule0Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecCtrlRamxMemRule0Rule3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::SecCtrlRamxMemRule0Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::SecCtrlRamxMemRule0Rule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SecCtrlRamxMemRule0Rule4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::SecCtrlRamxMemRule0Rule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::SecCtrlRamxMemRule0Rule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SecCtrlRamxMemRule0Rule5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::SecCtrlRamxMemRule0Rule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::SecCtrlRamxMemRule0Rule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SecCtrlRamxMemRule0Rule6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::SecCtrlRamxMemRule0Rule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::SecCtrlRamxMemRule0Rule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SecCtrlRamxMemRule0Rule7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::SecCtrlRamxMemRule0Rule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlRamxMemRule0 {
    #[inline(always)]
    fn default() -> SecCtrlRamxMemRule0 {
        SecCtrlRamxMemRule0(0)
    }
}
impl core::fmt::Debug for SecCtrlRamxMemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRamxMemRule0")
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
impl defmt::Format for SecCtrlRamxMemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRamxMemRule0 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
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
#[doc = "Security access rules for RAMX slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRamxSlaveRule(pub u32);
impl SecCtrlRamxSlaveRule {
    #[doc = "Security access rules for the whole RAMX : 0x0400_0000 - 0x0400_7FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn ramx_rule(&self) -> super::vals::RamxRule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RamxRule::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole RAMX : 0x0400_0000 - 0x0400_7FFF"]
    #[inline(always)]
    pub const fn set_ramx_rule(&mut self, val: super::vals::RamxRule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SecCtrlRamxSlaveRule {
    #[inline(always)]
    fn default() -> SecCtrlRamxSlaveRule {
        SecCtrlRamxSlaveRule(0)
    }
}
impl core::fmt::Debug for SecCtrlRamxSlaveRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRamxSlaveRule")
            .field("ramx_rule", &self.ramx_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlRamxSlaveRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRamxSlaveRule {{ ramx_rule: {:?} }}",
            self.ramx_rule()
        )
    }
}
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRomMemRule0(pub u32);
impl SecCtrlRomMemRule0 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::SecCtrlRomMemRule0Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecCtrlRomMemRule0Rule0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::SecCtrlRomMemRule0Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::SecCtrlRomMemRule0Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecCtrlRomMemRule0Rule1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::SecCtrlRomMemRule0Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::SecCtrlRomMemRule0Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCtrlRomMemRule0Rule2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::SecCtrlRomMemRule0Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::SecCtrlRomMemRule0Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecCtrlRomMemRule0Rule3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::SecCtrlRomMemRule0Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::SecCtrlRomMemRule0Rule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SecCtrlRomMemRule0Rule4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::SecCtrlRomMemRule0Rule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::SecCtrlRomMemRule0Rule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SecCtrlRomMemRule0Rule5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::SecCtrlRomMemRule0Rule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::SecCtrlRomMemRule0Rule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SecCtrlRomMemRule0Rule6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::SecCtrlRomMemRule0Rule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::SecCtrlRomMemRule0Rule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SecCtrlRomMemRule0Rule7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::SecCtrlRomMemRule0Rule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlRomMemRule0 {
    #[inline(always)]
    fn default() -> SecCtrlRomMemRule0 {
        SecCtrlRomMemRule0(0)
    }
}
impl core::fmt::Debug for SecCtrlRomMemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRomMemRule0")
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
impl defmt::Format for SecCtrlRomMemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRomMemRule0 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
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
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRomMemRule1(pub u32);
impl SecCtrlRomMemRule1 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::SecCtrlRomMemRule1Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecCtrlRomMemRule1Rule0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::SecCtrlRomMemRule1Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::SecCtrlRomMemRule1Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecCtrlRomMemRule1Rule1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::SecCtrlRomMemRule1Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::SecCtrlRomMemRule1Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCtrlRomMemRule1Rule2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::SecCtrlRomMemRule1Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::SecCtrlRomMemRule1Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecCtrlRomMemRule1Rule3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::SecCtrlRomMemRule1Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::SecCtrlRomMemRule1Rule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SecCtrlRomMemRule1Rule4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::SecCtrlRomMemRule1Rule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::SecCtrlRomMemRule1Rule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SecCtrlRomMemRule1Rule5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::SecCtrlRomMemRule1Rule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::SecCtrlRomMemRule1Rule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SecCtrlRomMemRule1Rule6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::SecCtrlRomMemRule1Rule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::SecCtrlRomMemRule1Rule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SecCtrlRomMemRule1Rule7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::SecCtrlRomMemRule1Rule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlRomMemRule1 {
    #[inline(always)]
    fn default() -> SecCtrlRomMemRule1 {
        SecCtrlRomMemRule1(0)
    }
}
impl core::fmt::Debug for SecCtrlRomMemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRomMemRule1")
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
impl defmt::Format for SecCtrlRomMemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRomMemRule1 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
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
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRomMemRule2(pub u32);
impl SecCtrlRomMemRule2 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::SecCtrlRomMemRule2Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecCtrlRomMemRule2Rule0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::SecCtrlRomMemRule2Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::SecCtrlRomMemRule2Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecCtrlRomMemRule2Rule1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::SecCtrlRomMemRule2Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::SecCtrlRomMemRule2Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCtrlRomMemRule2Rule2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::SecCtrlRomMemRule2Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::SecCtrlRomMemRule2Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecCtrlRomMemRule2Rule3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::SecCtrlRomMemRule2Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::SecCtrlRomMemRule2Rule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SecCtrlRomMemRule2Rule4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::SecCtrlRomMemRule2Rule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::SecCtrlRomMemRule2Rule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SecCtrlRomMemRule2Rule5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::SecCtrlRomMemRule2Rule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::SecCtrlRomMemRule2Rule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SecCtrlRomMemRule2Rule6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::SecCtrlRomMemRule2Rule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::SecCtrlRomMemRule2Rule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SecCtrlRomMemRule2Rule7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::SecCtrlRomMemRule2Rule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlRomMemRule2 {
    #[inline(always)]
    fn default() -> SecCtrlRomMemRule2 {
        SecCtrlRomMemRule2(0)
    }
}
impl core::fmt::Debug for SecCtrlRomMemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRomMemRule2")
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
impl defmt::Format for SecCtrlRomMemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRomMemRule2 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
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
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlRomMemRule3(pub u32);
impl SecCtrlRomMemRule3 {
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::SecCtrlRomMemRule3Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecCtrlRomMemRule3Rule0::from_bits(val as u8)
    }
    #[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::SecCtrlRomMemRule3Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::SecCtrlRomMemRule3Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SecCtrlRomMemRule3Rule1::from_bits(val as u8)
    }
    #[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::SecCtrlRomMemRule3Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::SecCtrlRomMemRule3Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCtrlRomMemRule3Rule2::from_bits(val as u8)
    }
    #[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::SecCtrlRomMemRule3Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::SecCtrlRomMemRule3Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SecCtrlRomMemRule3Rule3::from_bits(val as u8)
    }
    #[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::SecCtrlRomMemRule3Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::SecCtrlRomMemRule3Rule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SecCtrlRomMemRule3Rule4::from_bits(val as u8)
    }
    #[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::SecCtrlRomMemRule3Rule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::SecCtrlRomMemRule3Rule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SecCtrlRomMemRule3Rule5::from_bits(val as u8)
    }
    #[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::SecCtrlRomMemRule3Rule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::SecCtrlRomMemRule3Rule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SecCtrlRomMemRule3Rule6::from_bits(val as u8)
    }
    #[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::SecCtrlRomMemRule3Rule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::SecCtrlRomMemRule3Rule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SecCtrlRomMemRule3Rule7::from_bits(val as u8)
    }
    #[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::SecCtrlRomMemRule3Rule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for SecCtrlRomMemRule3 {
    #[inline(always)]
    fn default() -> SecCtrlRomMemRule3 {
        SecCtrlRomMemRule3(0)
    }
}
impl core::fmt::Debug for SecCtrlRomMemRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlRomMemRule3")
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
impl defmt::Format for SecCtrlRomMemRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlRomMemRule3 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
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
#[doc = "Security access rules for RAM_USB_HS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlUsbHsMemRule(pub u32);
impl SecCtrlUsbHsMemRule {
    #[doc = "Address space: 0x4010_0000 - 0x4010_0FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_sect_0_rule(&self) -> super::vals::SramSect0Rule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SramSect0Rule::from_bits(val as u8)
    }
    #[doc = "Address space: 0x4010_0000 - 0x4010_0FFF"]
    #[inline(always)]
    pub const fn set_sram_sect_0_rule(&mut self, val: super::vals::SramSect0Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Address space: 0x4010_1000 - 0x4010_1FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_sect_1_rule(&self) -> super::vals::SramSect1Rule {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SramSect1Rule::from_bits(val as u8)
    }
    #[doc = "Address space: 0x4010_1000 - 0x4010_1FFF"]
    #[inline(always)]
    pub const fn set_sram_sect_1_rule(&mut self, val: super::vals::SramSect1Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Address space: 0x4010_2000 - 0x4010_2FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_sect_2_rule(&self) -> super::vals::SramSect2Rule {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SramSect2Rule::from_bits(val as u8)
    }
    #[doc = "Address space: 0x4010_2000 - 0x4010_2FFF"]
    #[inline(always)]
    pub const fn set_sram_sect_2_rule(&mut self, val: super::vals::SramSect2Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Address space: 0x4010_3000 - 0x4010_3FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_sect_3_rule(&self) -> super::vals::SramSect3Rule {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SramSect3Rule::from_bits(val as u8)
    }
    #[doc = "Address space: 0x4010_3000 - 0x4010_3FFF"]
    #[inline(always)]
    pub const fn set_sram_sect_3_rule(&mut self, val: super::vals::SramSect3Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for SecCtrlUsbHsMemRule {
    #[inline(always)]
    fn default() -> SecCtrlUsbHsMemRule {
        SecCtrlUsbHsMemRule(0)
    }
}
impl core::fmt::Debug for SecCtrlUsbHsMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlUsbHsMemRule")
            .field("sram_sect_0_rule", &self.sram_sect_0_rule())
            .field("sram_sect_1_rule", &self.sram_sect_1_rule())
            .field("sram_sect_2_rule", &self.sram_sect_2_rule())
            .field("sram_sect_3_rule", &self.sram_sect_3_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlUsbHsMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlUsbHsMemRule {{ sram_sect_0_rule: {:?}, sram_sect_1_rule: {:?}, sram_sect_2_rule: {:?}, sram_sect_3_rule: {:?} }}",
            self.sram_sect_0_rule(),
            self.sram_sect_1_rule(),
            self.sram_sect_2_rule(),
            self.sram_sect_3_rule()
        )
    }
}
#[doc = "Security access rules for USB High speed RAM slaves."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCtrlUsbHsSlaveRule(pub u32);
impl SecCtrlUsbHsSlaveRule {
    #[doc = "Security access rules for the whole USB High Speed RAM : 0x4010_0000 - 0x4010_3FFF"]
    #[must_use]
    #[inline(always)]
    pub const fn ram_usb_hs_rule(&self) -> super::vals::RamUsbHsRule {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RamUsbHsRule::from_bits(val as u8)
    }
    #[doc = "Security access rules for the whole USB High Speed RAM : 0x4010_0000 - 0x4010_3FFF"]
    #[inline(always)]
    pub const fn set_ram_usb_hs_rule(&mut self, val: super::vals::RamUsbHsRule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SecCtrlUsbHsSlaveRule {
    #[inline(always)]
    fn default() -> SecCtrlUsbHsSlaveRule {
        SecCtrlUsbHsSlaveRule(0)
    }
}
impl core::fmt::Debug for SecCtrlUsbHsSlaveRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCtrlUsbHsSlaveRule")
            .field("ram_usb_hs_rule", &self.ram_usb_hs_rule())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCtrlUsbHsSlaveRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCtrlUsbHsSlaveRule {{ ram_usb_hs_rule: {:?} }}",
            self.ram_usb_hs_rule()
        )
    }
}
#[doc = "Secure GPIO mask for port 0 pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpioMask0(pub u32);
impl SecGpioMask0 {
    #[doc = "Secure mask for pin P0_0"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin0_sec_mask(&self) -> super::vals::Pio0Pin0SecMask {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pio0Pin0SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_0"]
    #[inline(always)]
    pub const fn set_pio0_pin0_sec_mask(&mut self, val: super::vals::Pio0Pin0SecMask) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure mask for pin P0_1"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin1_sec_mask(&self) -> super::vals::Pio0Pin1SecMask {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pio0Pin1SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_1"]
    #[inline(always)]
    pub const fn set_pio0_pin1_sec_mask(&mut self, val: super::vals::Pio0Pin1SecMask) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Secure mask for pin P0_2"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin2_sec_mask(&self) -> super::vals::Pio0Pin2SecMask {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pio0Pin2SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_2"]
    #[inline(always)]
    pub const fn set_pio0_pin2_sec_mask(&mut self, val: super::vals::Pio0Pin2SecMask) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Secure mask for pin P0_3"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin3_sec_mask(&self) -> super::vals::Pio0Pin3SecMask {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pio0Pin3SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_3"]
    #[inline(always)]
    pub const fn set_pio0_pin3_sec_mask(&mut self, val: super::vals::Pio0Pin3SecMask) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Secure mask for pin P0_4"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin4_sec_mask(&self) -> super::vals::Pio0Pin4SecMask {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pio0Pin4SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_4"]
    #[inline(always)]
    pub const fn set_pio0_pin4_sec_mask(&mut self, val: super::vals::Pio0Pin4SecMask) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Secure mask for pin P0_5"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin5_sec_mask(&self) -> super::vals::Pio0Pin5SecMask {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pio0Pin5SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_5"]
    #[inline(always)]
    pub const fn set_pio0_pin5_sec_mask(&mut self, val: super::vals::Pio0Pin5SecMask) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Secure mask for pin P0_6"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin6_sec_mask(&self) -> super::vals::Pio0Pin6SecMask {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio0Pin6SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_6"]
    #[inline(always)]
    pub const fn set_pio0_pin6_sec_mask(&mut self, val: super::vals::Pio0Pin6SecMask) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Secure mask for pin P0_7"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin7_sec_mask(&self) -> super::vals::Pio0Pin7SecMask {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pio0Pin7SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_7"]
    #[inline(always)]
    pub const fn set_pio0_pin7_sec_mask(&mut self, val: super::vals::Pio0Pin7SecMask) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Secure mask for pin P0_8"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin8_sec_mask(&self) -> super::vals::Pio0Pin8SecMask {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio0Pin8SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_8"]
    #[inline(always)]
    pub const fn set_pio0_pin8_sec_mask(&mut self, val: super::vals::Pio0Pin8SecMask) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Secure mask for pin P0_9"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin9_sec_mask(&self) -> super::vals::Pio0Pin9SecMask {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio0Pin9SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_9"]
    #[inline(always)]
    pub const fn set_pio0_pin9_sec_mask(&mut self, val: super::vals::Pio0Pin9SecMask) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Secure mask for pin P0_10"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin10_sec_mask(&self) -> super::vals::Pio0Pin10SecMask {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio0Pin10SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_10"]
    #[inline(always)]
    pub const fn set_pio0_pin10_sec_mask(&mut self, val: super::vals::Pio0Pin10SecMask) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Secure mask for pin P0_11"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin11_sec_mask(&self) -> super::vals::Pio0Pin11SecMask {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pio0Pin11SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_11"]
    #[inline(always)]
    pub const fn set_pio0_pin11_sec_mask(&mut self, val: super::vals::Pio0Pin11SecMask) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Secure mask for pin P0_12"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin12_sec_mask(&self) -> super::vals::Pio0Pin12SecMask {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pio0Pin12SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_12"]
    #[inline(always)]
    pub const fn set_pio0_pin12_sec_mask(&mut self, val: super::vals::Pio0Pin12SecMask) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Secure mask for pin P0_13"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin13_sec_mask(&self) -> super::vals::Pio0Pin13SecMask {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pio0Pin13SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_13"]
    #[inline(always)]
    pub const fn set_pio0_pin13_sec_mask(&mut self, val: super::vals::Pio0Pin13SecMask) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Secure mask for pin P0_14"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin14_sec_mask(&self) -> super::vals::Pio0Pin14SecMask {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pio0Pin14SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_14"]
    #[inline(always)]
    pub const fn set_pio0_pin14_sec_mask(&mut self, val: super::vals::Pio0Pin14SecMask) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Secure mask for pin P0_15"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin15_sec_mask(&self) -> super::vals::Pio0Pin15SecMask {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pio0Pin15SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_15"]
    #[inline(always)]
    pub const fn set_pio0_pin15_sec_mask(&mut self, val: super::vals::Pio0Pin15SecMask) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Secure mask for pin P0_16"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin16_sec_mask(&self) -> super::vals::Pio0Pin16SecMask {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pio0Pin16SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_16"]
    #[inline(always)]
    pub const fn set_pio0_pin16_sec_mask(&mut self, val: super::vals::Pio0Pin16SecMask) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Secure mask for pin P0_17"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin17_sec_mask(&self) -> super::vals::Pio0Pin17SecMask {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pio0Pin17SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_17"]
    #[inline(always)]
    pub const fn set_pio0_pin17_sec_mask(&mut self, val: super::vals::Pio0Pin17SecMask) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Secure mask for pin P0_18"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin18_sec_mask(&self) -> super::vals::Pio0Pin18SecMask {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pio0Pin18SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_18"]
    #[inline(always)]
    pub const fn set_pio0_pin18_sec_mask(&mut self, val: super::vals::Pio0Pin18SecMask) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Secure mask for pin P0_19"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin19_sec_mask(&self) -> super::vals::Pio0Pin19SecMask {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pio0Pin19SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_19"]
    #[inline(always)]
    pub const fn set_pio0_pin19_sec_mask(&mut self, val: super::vals::Pio0Pin19SecMask) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Secure mask for pin P0_20"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin20_sec_mask(&self) -> super::vals::Pio0Pin20SecMask {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pio0Pin20SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_20"]
    #[inline(always)]
    pub const fn set_pio0_pin20_sec_mask(&mut self, val: super::vals::Pio0Pin20SecMask) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Secure mask for pin P0_21"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin21_sec_mask(&self) -> super::vals::Pio0Pin21SecMask {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pio0Pin21SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_21"]
    #[inline(always)]
    pub const fn set_pio0_pin21_sec_mask(&mut self, val: super::vals::Pio0Pin21SecMask) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Secure mask for pin P0_22"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin22_sec_mask(&self) -> super::vals::Pio0Pin22SecMask {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pio0Pin22SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_22"]
    #[inline(always)]
    pub const fn set_pio0_pin22_sec_mask(&mut self, val: super::vals::Pio0Pin22SecMask) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Secure mask for pin P0_23"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin23_sec_mask(&self) -> super::vals::Pio0Pin23SecMask {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pio0Pin23SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_23"]
    #[inline(always)]
    pub const fn set_pio0_pin23_sec_mask(&mut self, val: super::vals::Pio0Pin23SecMask) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Secure mask for pin P0_24"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin24_sec_mask(&self) -> super::vals::Pio0Pin24SecMask {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pio0Pin24SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_24"]
    #[inline(always)]
    pub const fn set_pio0_pin24_sec_mask(&mut self, val: super::vals::Pio0Pin24SecMask) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Secure mask for pin P0_25"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin25_sec_mask(&self) -> super::vals::Pio0Pin25SecMask {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pio0Pin25SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_25"]
    #[inline(always)]
    pub const fn set_pio0_pin25_sec_mask(&mut self, val: super::vals::Pio0Pin25SecMask) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Secure mask for pin P0_26"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin26_sec_mask(&self) -> super::vals::Pio0Pin26SecMask {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pio0Pin26SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_26"]
    #[inline(always)]
    pub const fn set_pio0_pin26_sec_mask(&mut self, val: super::vals::Pio0Pin26SecMask) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Secure mask for pin P0_27"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin27_sec_mask(&self) -> super::vals::Pio0Pin27SecMask {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pio0Pin27SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_27"]
    #[inline(always)]
    pub const fn set_pio0_pin27_sec_mask(&mut self, val: super::vals::Pio0Pin27SecMask) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Secure mask for pin P0_28"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin28_sec_mask(&self) -> super::vals::Pio0Pin28SecMask {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pio0Pin28SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_28"]
    #[inline(always)]
    pub const fn set_pio0_pin28_sec_mask(&mut self, val: super::vals::Pio0Pin28SecMask) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Secure mask for pin P0_29"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin29_sec_mask(&self) -> super::vals::Pio0Pin29SecMask {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pio0Pin29SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_29"]
    #[inline(always)]
    pub const fn set_pio0_pin29_sec_mask(&mut self, val: super::vals::Pio0Pin29SecMask) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Secure mask for pin P0_30"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin30_sec_mask(&self) -> super::vals::Pio0Pin30SecMask {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Pio0Pin30SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_30"]
    #[inline(always)]
    pub const fn set_pio0_pin30_sec_mask(&mut self, val: super::vals::Pio0Pin30SecMask) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Secure mask for pin P0_31"]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin31_sec_mask(&self) -> super::vals::Pio0Pin31SecMask {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pio0Pin31SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P0_31"]
    #[inline(always)]
    pub const fn set_pio0_pin31_sec_mask(&mut self, val: super::vals::Pio0Pin31SecMask) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            "SecGpioMask0 {{ pio0_pin0_sec_mask: {:?}, pio0_pin1_sec_mask: {:?}, pio0_pin2_sec_mask: {:?}, pio0_pin3_sec_mask: {:?}, pio0_pin4_sec_mask: {:?}, pio0_pin5_sec_mask: {:?}, pio0_pin6_sec_mask: {:?}, pio0_pin7_sec_mask: {:?}, pio0_pin8_sec_mask: {:?}, pio0_pin9_sec_mask: {:?}, pio0_pin10_sec_mask: {:?}, pio0_pin11_sec_mask: {:?}, pio0_pin12_sec_mask: {:?}, pio0_pin13_sec_mask: {:?}, pio0_pin14_sec_mask: {:?}, pio0_pin15_sec_mask: {:?}, pio0_pin16_sec_mask: {:?}, pio0_pin17_sec_mask: {:?}, pio0_pin18_sec_mask: {:?}, pio0_pin19_sec_mask: {:?}, pio0_pin20_sec_mask: {:?}, pio0_pin21_sec_mask: {:?}, pio0_pin22_sec_mask: {:?}, pio0_pin23_sec_mask: {:?}, pio0_pin24_sec_mask: {:?}, pio0_pin25_sec_mask: {:?}, pio0_pin26_sec_mask: {:?}, pio0_pin27_sec_mask: {:?}, pio0_pin28_sec_mask: {:?}, pio0_pin29_sec_mask: {:?}, pio0_pin30_sec_mask: {:?}, pio0_pin31_sec_mask: {:?} }}",
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
    #[doc = "Secure mask for pin P1_0"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin0_sec_mask(&self) -> super::vals::Pio1Pin0SecMask {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pio1Pin0SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_0"]
    #[inline(always)]
    pub const fn set_pio1_pin0_sec_mask(&mut self, val: super::vals::Pio1Pin0SecMask) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure mask for pin P1_1"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin1_sec_mask(&self) -> super::vals::Pio1Pin1SecMask {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pio1Pin1SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_1"]
    #[inline(always)]
    pub const fn set_pio1_pin1_sec_mask(&mut self, val: super::vals::Pio1Pin1SecMask) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Secure mask for pin P1_2"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin2_sec_mask(&self) -> super::vals::Pio1Pin2SecMask {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pio1Pin2SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_2"]
    #[inline(always)]
    pub const fn set_pio1_pin2_sec_mask(&mut self, val: super::vals::Pio1Pin2SecMask) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Secure mask for pin P1_3"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin3_sec_mask(&self) -> super::vals::Pio1Pin3SecMask {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pio1Pin3SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_3"]
    #[inline(always)]
    pub const fn set_pio1_pin3_sec_mask(&mut self, val: super::vals::Pio1Pin3SecMask) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Secure mask for pin P1_4"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin4_sec_mask(&self) -> super::vals::Pio1Pin4SecMask {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pio1Pin4SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_4"]
    #[inline(always)]
    pub const fn set_pio1_pin4_sec_mask(&mut self, val: super::vals::Pio1Pin4SecMask) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Secure mask for pin P1_5"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin5_sec_mask(&self) -> super::vals::Pio1Pin5SecMask {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pio1Pin5SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_5"]
    #[inline(always)]
    pub const fn set_pio1_pin5_sec_mask(&mut self, val: super::vals::Pio1Pin5SecMask) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Secure mask for pin P1_6"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin6_sec_mask(&self) -> super::vals::Pio1Pin6SecMask {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio1Pin6SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_6"]
    #[inline(always)]
    pub const fn set_pio1_pin6_sec_mask(&mut self, val: super::vals::Pio1Pin6SecMask) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Secure mask for pin P1_7"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin7_sec_mask(&self) -> super::vals::Pio1Pin7SecMask {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pio1Pin7SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_7"]
    #[inline(always)]
    pub const fn set_pio1_pin7_sec_mask(&mut self, val: super::vals::Pio1Pin7SecMask) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Secure mask for pin P1_8"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin8_sec_mask(&self) -> super::vals::Pio1Pin8SecMask {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio1Pin8SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_8"]
    #[inline(always)]
    pub const fn set_pio1_pin8_sec_mask(&mut self, val: super::vals::Pio1Pin8SecMask) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Secure mask for pin P1_9"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin9_sec_mask(&self) -> super::vals::Pio1Pin9SecMask {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio1Pin9SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_9"]
    #[inline(always)]
    pub const fn set_pio1_pin9_sec_mask(&mut self, val: super::vals::Pio1Pin9SecMask) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Secure mask for pin P1_10"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin10_sec_mask(&self) -> super::vals::Pio1Pin10SecMask {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio1Pin10SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_10"]
    #[inline(always)]
    pub const fn set_pio1_pin10_sec_mask(&mut self, val: super::vals::Pio1Pin10SecMask) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Secure mask for pin P1_11"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin11_sec_mask(&self) -> super::vals::Pio1Pin11SecMask {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pio1Pin11SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_11"]
    #[inline(always)]
    pub const fn set_pio1_pin11_sec_mask(&mut self, val: super::vals::Pio1Pin11SecMask) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Secure mask for pin P1_12"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin12_sec_mask(&self) -> super::vals::Pio1Pin12SecMask {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pio1Pin12SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_12"]
    #[inline(always)]
    pub const fn set_pio1_pin12_sec_mask(&mut self, val: super::vals::Pio1Pin12SecMask) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Secure mask for pin P1_13"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin13_sec_mask(&self) -> super::vals::Pio1Pin13SecMask {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pio1Pin13SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_13"]
    #[inline(always)]
    pub const fn set_pio1_pin13_sec_mask(&mut self, val: super::vals::Pio1Pin13SecMask) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Secure mask for pin P1_14"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin14_sec_mask(&self) -> super::vals::Pio1Pin14SecMask {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pio1Pin14SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_14"]
    #[inline(always)]
    pub const fn set_pio1_pin14_sec_mask(&mut self, val: super::vals::Pio1Pin14SecMask) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Secure mask for pin P1_15"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin15_sec_mask(&self) -> super::vals::Pio1Pin15SecMask {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pio1Pin15SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_15"]
    #[inline(always)]
    pub const fn set_pio1_pin15_sec_mask(&mut self, val: super::vals::Pio1Pin15SecMask) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Secure mask for pin P1_16"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin16_sec_mask(&self) -> super::vals::Pio1Pin16SecMask {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pio1Pin16SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_16"]
    #[inline(always)]
    pub const fn set_pio1_pin16_sec_mask(&mut self, val: super::vals::Pio1Pin16SecMask) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Secure mask for pin P1_17"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin17_sec_mask(&self) -> super::vals::Pio1Pin17SecMask {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pio1Pin17SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_17"]
    #[inline(always)]
    pub const fn set_pio1_pin17_sec_mask(&mut self, val: super::vals::Pio1Pin17SecMask) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Secure mask for pin P1_18"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin18_sec_mask(&self) -> super::vals::Pio1Pin18SecMask {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pio1Pin18SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_18"]
    #[inline(always)]
    pub const fn set_pio1_pin18_sec_mask(&mut self, val: super::vals::Pio1Pin18SecMask) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Secure mask for pin P1_19"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin19_sec_mask(&self) -> super::vals::Pio1Pin19SecMask {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pio1Pin19SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_19"]
    #[inline(always)]
    pub const fn set_pio1_pin19_sec_mask(&mut self, val: super::vals::Pio1Pin19SecMask) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Secure mask for pin P1_20"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin20_sec_mask(&self) -> super::vals::Pio1Pin20SecMask {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pio1Pin20SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_20"]
    #[inline(always)]
    pub const fn set_pio1_pin20_sec_mask(&mut self, val: super::vals::Pio1Pin20SecMask) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Secure mask for pin P1_21"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin21_sec_mask(&self) -> super::vals::Pio1Pin21SecMask {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pio1Pin21SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_21"]
    #[inline(always)]
    pub const fn set_pio1_pin21_sec_mask(&mut self, val: super::vals::Pio1Pin21SecMask) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Secure mask for pin P1_22"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin22_sec_mask(&self) -> super::vals::Pio1Pin22SecMask {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pio1Pin22SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_22"]
    #[inline(always)]
    pub const fn set_pio1_pin22_sec_mask(&mut self, val: super::vals::Pio1Pin22SecMask) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Secure mask for pin P1_23"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin23_sec_mask(&self) -> super::vals::Pio1Pin23SecMask {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pio1Pin23SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_23"]
    #[inline(always)]
    pub const fn set_pio1_pin23_sec_mask(&mut self, val: super::vals::Pio1Pin23SecMask) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Secure mask for pin P1_24"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin24_sec_mask(&self) -> super::vals::Pio1Pin24SecMask {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pio1Pin24SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_24"]
    #[inline(always)]
    pub const fn set_pio1_pin24_sec_mask(&mut self, val: super::vals::Pio1Pin24SecMask) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Secure mask for pin P1_25"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin25_sec_mask(&self) -> super::vals::Pio1Pin25SecMask {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pio1Pin25SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_25"]
    #[inline(always)]
    pub const fn set_pio1_pin25_sec_mask(&mut self, val: super::vals::Pio1Pin25SecMask) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Secure mask for pin P1_26"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin26_sec_mask(&self) -> super::vals::Pio1Pin26SecMask {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pio1Pin26SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_26"]
    #[inline(always)]
    pub const fn set_pio1_pin26_sec_mask(&mut self, val: super::vals::Pio1Pin26SecMask) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Secure mask for pin P1_27"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin27_sec_mask(&self) -> super::vals::Pio1Pin27SecMask {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pio1Pin27SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_27"]
    #[inline(always)]
    pub const fn set_pio1_pin27_sec_mask(&mut self, val: super::vals::Pio1Pin27SecMask) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Secure mask for pin P1_28"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin28_sec_mask(&self) -> super::vals::Pio1Pin28SecMask {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pio1Pin28SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_28"]
    #[inline(always)]
    pub const fn set_pio1_pin28_sec_mask(&mut self, val: super::vals::Pio1Pin28SecMask) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Secure mask for pin P1_29"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin29_sec_mask(&self) -> super::vals::Pio1Pin29SecMask {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pio1Pin29SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_29"]
    #[inline(always)]
    pub const fn set_pio1_pin29_sec_mask(&mut self, val: super::vals::Pio1Pin29SecMask) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Secure mask for pin P1_30"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin30_sec_mask(&self) -> super::vals::Pio1Pin30SecMask {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Pio1Pin30SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_30"]
    #[inline(always)]
    pub const fn set_pio1_pin30_sec_mask(&mut self, val: super::vals::Pio1Pin30SecMask) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Secure mask for pin P1_31"]
    #[must_use]
    #[inline(always)]
    pub const fn pio1_pin31_sec_mask(&self) -> super::vals::Pio1Pin31SecMask {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pio1Pin31SecMask::from_bits(val as u8)
    }
    #[doc = "Secure mask for pin P1_31"]
    #[inline(always)]
    pub const fn set_pio1_pin31_sec_mask(&mut self, val: super::vals::Pio1Pin31SecMask) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            "SecGpioMask1 {{ pio1_pin0_sec_mask: {:?}, pio1_pin1_sec_mask: {:?}, pio1_pin2_sec_mask: {:?}, pio1_pin3_sec_mask: {:?}, pio1_pin4_sec_mask: {:?}, pio1_pin5_sec_mask: {:?}, pio1_pin6_sec_mask: {:?}, pio1_pin7_sec_mask: {:?}, pio1_pin8_sec_mask: {:?}, pio1_pin9_sec_mask: {:?}, pio1_pin10_sec_mask: {:?}, pio1_pin11_sec_mask: {:?}, pio1_pin12_sec_mask: {:?}, pio1_pin13_sec_mask: {:?}, pio1_pin14_sec_mask: {:?}, pio1_pin15_sec_mask: {:?}, pio1_pin16_sec_mask: {:?}, pio1_pin17_sec_mask: {:?}, pio1_pin18_sec_mask: {:?}, pio1_pin19_sec_mask: {:?}, pio1_pin20_sec_mask: {:?}, pio1_pin21_sec_mask: {:?}, pio1_pin22_sec_mask: {:?}, pio1_pin23_sec_mask: {:?}, pio1_pin24_sec_mask: {:?}, pio1_pin25_sec_mask: {:?}, pio1_pin26_sec_mask: {:?}, pio1_pin27_sec_mask: {:?}, pio1_pin28_sec_mask: {:?}, pio1_pin29_sec_mask: {:?}, pio1_pin30_sec_mask: {:?}, pio1_pin31_sec_mask: {:?} }}",
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
#[doc = "Security General Purpose register access control."]
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
    #[doc = "SEC_CPU_INT_MASK0 register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_cpu1_int_mask0_lock(&self) -> super::vals::SecCpu1IntMask0Lock {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SecCpu1IntMask0Lock::from_bits(val as u8)
    }
    #[doc = "SEC_CPU_INT_MASK0 register write-lock."]
    #[inline(always)]
    pub const fn set_sec_cpu1_int_mask0_lock(&mut self, val: super::vals::SecCpu1IntMask0Lock) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "SEC_CPU_INT_MASK1 register write-lock."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_cpu1_int_mask1_lock(&self) -> super::vals::SecCpu1IntMask1Lock {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::SecCpu1IntMask1Lock::from_bits(val as u8)
    }
    #[doc = "SEC_CPU_INT_MASK1 register write-lock."]
    #[inline(always)]
    pub const fn set_sec_cpu1_int_mask1_lock(&mut self, val: super::vals::SecCpu1IntMask1Lock) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
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
            .field("sec_cpu1_int_mask0_lock", &self.sec_cpu1_int_mask0_lock())
            .field("sec_cpu1_int_mask1_lock", &self.sec_cpu1_int_mask1_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecMaskLock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecMaskLock {{ sec_gpio_mask0_lock: {:?}, sec_gpio_mask1_lock: {:?}, sec_cpu1_int_mask0_lock: {:?}, sec_cpu1_int_mask1_lock: {:?} }}",
            self.sec_gpio_mask0_lock(),
            self.sec_gpio_mask1_lock(),
            self.sec_cpu1_int_mask0_lock(),
            self.sec_cpu1_int_mask1_lock()
        )
    }
}
#[doc = "most recent security violation address for AHB port n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioAddr(pub u32);
impl SecVioAddr {
    #[doc = "security violation address for AHB port"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "security violation address for AHB port"]
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
    #[doc = "violation information valid flag for AHB port 0. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 0. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "violation information valid flag for AHB port 1. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 1. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "violation information valid flag for AHB port 2. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 2. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "violation information valid flag for AHB port 3. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 3. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "violation information valid flag for AHB port 4. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 4. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "violation information valid flag for AHB port 5. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 5. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "violation information valid flag for AHB port 6. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 6. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "violation information valid flag for AHB port 7. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 7. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "violation information valid flag for AHB port 8. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 8. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "violation information valid flag for AHB port 9. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 9. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "violation information valid flag for AHB port 10. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 10. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "violation information valid flag for AHB port 11. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "violation information valid flag for AHB port 11. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_vio_info_valid11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioInfoValid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioInfoValid {{ vio_info_valid0: {=bool:?}, vio_info_valid1: {=bool:?}, vio_info_valid2: {=bool:?}, vio_info_valid3: {=bool:?}, vio_info_valid4: {=bool:?}, vio_info_valid5: {=bool:?}, vio_info_valid6: {=bool:?}, vio_info_valid7: {=bool:?}, vio_info_valid8: {=bool:?}, vio_info_valid9: {=bool:?}, vio_info_valid10: {=bool:?}, vio_info_valid11: {=bool:?} }}",
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
            self.vio_info_valid11()
        )
    }
}
#[doc = "most recent security violation miscellaneous information for AHB port n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioMiscInfo(pub u32);
impl SecVioMiscInfo {
    #[doc = "security violation access read/write indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_write(&self) -> super::vals::SecVioInfoWrite {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SecVioInfoWrite::from_bits(val as u8)
    }
    #[doc = "security violation access read/write indicator."]
    #[inline(always)]
    pub const fn set_sec_vio_info_write(&mut self, val: super::vals::SecVioInfoWrite) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "security violation access data/code indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_data_access(&self) -> super::vals::SecVioInfoDataAccess {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SecVioInfoDataAccess::from_bits(val as u8)
    }
    #[doc = "security violation access data/code indicator."]
    #[inline(always)]
    pub const fn set_sec_vio_info_data_access(&mut self, val: super::vals::SecVioInfoDataAccess) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
    pub const fn sec_vio_info_master(&self) -> super::vals::SecVioInfoMaster {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::SecVioInfoMaster::from_bits(val as u8)
    }
    #[doc = "security violation master number"]
    #[inline(always)]
    pub const fn set_sec_vio_info_master(&mut self, val: super::vals::SecVioInfoMaster) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
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
            "SecVioMiscInfo {{ sec_vio_info_write: {:?}, sec_vio_info_data_access: {:?}, sec_vio_info_master_sec_level: {=u8:?}, sec_vio_info_master: {:?} }}",
            self.sec_vio_info_write(),
            self.sec_vio_info_data_access(),
            self.sec_vio_info_master_sec_level(),
            self.sec_vio_info_master()
        )
    }
}
