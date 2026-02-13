#[doc = "System Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkdiv(pub u32);
impl Ahbclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::AhbclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::AhbclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::AhbclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbclkdiv {
    #[inline(always)]
    fn default() -> Ahbclkdiv {
        Ahbclkdiv(0)
    }
}
impl core::fmt::Debug for Ahbclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkdiv")
            .field("div", &self.div())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkdiv {{ div: {=u8:?}, unstab: {:?} }}",
            self.div(),
            self.unstab()
        )
    }
}
#[doc = "AHB Matrix Priority Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbmatprio(pub u32);
impl Ahbmatprio {
    #[doc = "CPU0 C-AHB bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_cbus(&self) -> super::vals::Cpu0Cbus {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cpu0Cbus::from_bits(val as u8)
    }
    #[doc = "CPU0 C-AHB bus master priority level"]
    #[inline(always)]
    pub const fn set_cpu0_cbus(&mut self, val: super::vals::Cpu0Cbus) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 S-AHB bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_sbus(&self) -> super::vals::AhbmatprioCpu0Sbus {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::AhbmatprioCpu0Sbus::from_bits(val as u8)
    }
    #[doc = "CPU0 S-AHB bus master priority level"]
    #[inline(always)]
    pub const fn set_cpu0_sbus(&mut self, val: super::vals::AhbmatprioCpu0Sbus) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "SmartDMA-I bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_cbus_smart_dma_i(&self) -> super::vals::Cpu1CbusSmartDmaI {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Cpu1CbusSmartDmaI::from_bits(val as u8)
    }
    #[doc = "SmartDMA-I bus master priority level"]
    #[inline(always)]
    pub const fn set_cpu1_cbus_smart_dma_i(&mut self, val: super::vals::Cpu1CbusSmartDmaI) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SmartDMA-D bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_sbus_smart_dma_d(&self) -> super::vals::Cpu1SbusSmartDmaD {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Cpu1SbusSmartDmaD::from_bits(val as u8)
    }
    #[doc = "SmartDMA-D bus master priority level"]
    #[inline(always)]
    pub const fn set_cpu1_sbus_smart_dma_d(&mut self, val: super::vals::Cpu1SbusSmartDmaD) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "DMA0 controller bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> super::vals::AhbmatprioDma0 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::AhbmatprioDma0::from_bits(val as u8)
    }
    #[doc = "DMA0 controller bus master priority level"]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: super::vals::AhbmatprioDma0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "PKC and ELS bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pkc_els(&self) -> super::vals::PkcEls {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::PkcEls::from_bits(val as u8)
    }
    #[doc = "PKC and ELS bus master priority level"]
    #[inline(always)]
    pub const fn set_pkc_els(&mut self, val: super::vals::PkcEls) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "USB-FS bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_fs_enet(&self) -> super::vals::UsbFsEnet {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::UsbFsEnet::from_bits(val as u8)
    }
    #[doc = "USB-FS bus master priority level"]
    #[inline(always)]
    pub const fn set_usb_fs_enet(&mut self, val: super::vals::UsbFsEnet) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for Ahbmatprio {
    #[inline(always)]
    fn default() -> Ahbmatprio {
        Ahbmatprio(0)
    }
}
impl core::fmt::Debug for Ahbmatprio {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbmatprio")
            .field("cpu0_cbus", &self.cpu0_cbus())
            .field("cpu0_sbus", &self.cpu0_sbus())
            .field("cpu1_cbus_smart_dma_i", &self.cpu1_cbus_smart_dma_i())
            .field("cpu1_sbus_smart_dma_d", &self.cpu1_sbus_smart_dma_d())
            .field("dma0", &self.dma0())
            .field("pkc_els", &self.pkc_els())
            .field("usb_fs_enet", &self.usb_fs_enet())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbmatprio {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbmatprio {{ cpu0_cbus: {:?}, cpu0_sbus: {:?}, cpu1_cbus_smart_dma_i: {:?}, cpu1_sbus_smart_dma_d: {:?}, dma0: {:?}, pkc_els: {:?}, usb_fs_enet: {:?} }}",
            self.cpu0_cbus(),
            self.cpu0_sbus(),
            self.cpu1_cbus_smart_dma_i(),
            self.cpu1_sbus_smart_dma_d(),
            self.dma0(),
            self.pkc_els(),
            self.usb_fs_enet()
        )
    }
}
#[doc = "Gray to Binary Converter Binary Code \\[31:0\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BinaryCodeLsb(pub u32);
impl BinaryCodeLsb {
    #[doc = "Binary code \\[31:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn code_bin_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Binary code \\[31:0\\]"]
    #[inline(always)]
    pub const fn set_code_bin_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BinaryCodeLsb {
    #[inline(always)]
    fn default() -> BinaryCodeLsb {
        BinaryCodeLsb(0)
    }
}
impl core::fmt::Debug for BinaryCodeLsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BinaryCodeLsb")
            .field("code_bin_31_0", &self.code_bin_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BinaryCodeLsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BinaryCodeLsb {{ code_bin_31_0: {=u32:?} }}",
            self.code_bin_31_0()
        )
    }
}
#[doc = "Gray to Binary Converter Binary Code \\[41:32\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BinaryCodeMsb(pub u32);
impl BinaryCodeMsb {
    #[doc = "Binary code \\[41:32\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn code_bin_41_32(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Binary code \\[41:32\\]"]
    #[inline(always)]
    pub const fn set_code_bin_41_32(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for BinaryCodeMsb {
    #[inline(always)]
    fn default() -> BinaryCodeMsb {
        BinaryCodeMsb(0)
    }
}
impl core::fmt::Debug for BinaryCodeMsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BinaryCodeMsb")
            .field("code_bin_41_32", &self.code_bin_41_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BinaryCodeMsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BinaryCodeMsb {{ code_bin_41_32: {=u16:?} }}",
            self.code_bin_41_32()
        )
    }
}
#[doc = "BUS_CLK Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busclkdiv(pub u32);
impl Busclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::BusclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::BusclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::BusclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::BusclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::BusclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::BusclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::BusclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::BusclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::BusclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Busclkdiv {
    #[inline(always)]
    fn default() -> Busclkdiv {
        Busclkdiv(0)
    }
}
impl core::fmt::Debug for Busclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Busclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Busclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Clock Configuration Unlock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkunlock(pub u32);
impl Clkunlock {
    #[doc = "Controls clock configuration registers access (for example, SLOWCLKDIV, BUSCLKDIV, AHBCLKDIV, FROHFDIV, FROLFDIV, PLLxCLKDIV, MRCC_xxx_CLKDIV, MRCC_xxx_CLKSEL, MRCC_GLB_xxx)"]
    #[must_use]
    #[inline(always)]
    pub const fn unlock(&self) -> super::vals::Unlock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Unlock::from_bits(val as u8)
    }
    #[doc = "Controls clock configuration registers access (for example, SLOWCLKDIV, BUSCLKDIV, AHBCLKDIV, FROHFDIV, FROLFDIV, PLLxCLKDIV, MRCC_xxx_CLKDIV, MRCC_xxx_CLKSEL, MRCC_GLB_xxx)"]
    #[inline(always)]
    pub const fn set_unlock(&mut self, val: super::vals::Unlock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Clkunlock {
    #[inline(always)]
    fn default() -> Clkunlock {
        Clkunlock(0)
    }
}
impl core::fmt::Debug for Clkunlock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkunlock")
            .field("unlock", &self.unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkunlock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkunlock {{ unlock: {:?} }}", self.unlock())
    }
}
#[doc = "Non-Secure CPU0 System Tick Calibration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0nstckcal(pub u32);
impl Cpu0nstckcal {
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[must_use]
    #[inline(always)]
    pub const fn tenms(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub const fn set_tenms(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Indicates whether the TENMS value is exact."]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> super::vals::Skew {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Skew::from_bits(val as u8)
    }
    #[doc = "Indicates whether the TENMS value is exact."]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: super::vals::Skew) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor."]
    #[must_use]
    #[inline(always)]
    pub const fn noref(&self) -> super::vals::Noref {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Noref::from_bits(val as u8)
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor."]
    #[inline(always)]
    pub const fn set_noref(&mut self, val: super::vals::Noref) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Cpu0nstckcal {
    #[inline(always)]
    fn default() -> Cpu0nstckcal {
        Cpu0nstckcal(0)
    }
}
impl core::fmt::Debug for Cpu0nstckcal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu0nstckcal")
            .field("tenms", &self.tenms())
            .field("skew", &self.skew())
            .field("noref", &self.noref())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu0nstckcal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu0nstckcal {{ tenms: {=u32:?}, skew: {:?}, noref: {:?} }}",
            self.tenms(),
            self.skew(),
            self.noref()
        )
    }
}
#[doc = "CPU Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpustat(pub u32);
impl Cpustat {
    #[doc = "CPU0 sleeping state"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0sleeping(&self) -> super::vals::Cpu0sleeping {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cpu0sleeping::from_bits(val as u8)
    }
    #[doc = "CPU0 sleeping state"]
    #[inline(always)]
    pub const fn set_cpu0sleeping(&mut self, val: super::vals::Cpu0sleeping) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CPU0 lockup state"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0lockup(&self) -> super::vals::Cpu0lockup {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Cpu0lockup::from_bits(val as u8)
    }
    #[doc = "CPU0 lockup state"]
    #[inline(always)]
    pub const fn set_cpu0lockup(&mut self, val: super::vals::Cpu0lockup) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Cpustat {
    #[inline(always)]
    fn default() -> Cpustat {
        Cpustat(0)
    }
}
impl core::fmt::Debug for Cpustat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpustat")
            .field("cpu0sleeping", &self.cpu0sleeping())
            .field("cpu0lockup", &self.cpu0lockup())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpustat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpustat {{ cpu0sleeping: {:?}, cpu0lockup: {:?} }}",
            self.cpu0sleeping(),
            self.cpu0lockup()
        )
    }
}
#[doc = "CTIMER Global Start Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerglobalstarten(pub u32);
impl Ctimerglobalstarten {
    #[doc = "Enables the CTIMER0 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0_clk_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER0 function clock"]
    #[inline(always)]
    pub const fn set_ctimer0_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the CTIMER1 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1_clk_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER1 function clock"]
    #[inline(always)]
    pub const fn set_ctimer1_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the CTIMER2 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2_clk_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER2 function clock"]
    #[inline(always)]
    pub const fn set_ctimer2_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the CTIMER3 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3_clk_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER3 function clock"]
    #[inline(always)]
    pub const fn set_ctimer3_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the CTIMER4 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4_clk_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER4 function clock"]
    #[inline(always)]
    pub const fn set_ctimer4_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Ctimerglobalstarten {
    #[inline(always)]
    fn default() -> Ctimerglobalstarten {
        Ctimerglobalstarten(0)
    }
}
impl core::fmt::Debug for Ctimerglobalstarten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerglobalstarten")
            .field("ctimer0_clk_en", &self.ctimer0_clk_en())
            .field("ctimer1_clk_en", &self.ctimer1_clk_en())
            .field("ctimer2_clk_en", &self.ctimer2_clk_en())
            .field("ctimer3_clk_en", &self.ctimer3_clk_en())
            .field("ctimer4_clk_en", &self.ctimer4_clk_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerglobalstarten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctimerglobalstarten {{ ctimer0_clk_en: {=bool:?}, ctimer1_clk_en: {=bool:?}, ctimer2_clk_en: {=bool:?}, ctimer3_clk_en: {=bool:?}, ctimer4_clk_en: {=bool:?} }}",
            self.ctimer0_clk_en(),
            self.ctimer1_clk_en(),
            self.ctimer2_clk_en(),
            self.ctimer3_clk_en(),
            self.ctimer4_clk_en()
        )
    }
}
#[doc = "Debug Authentication BEACON"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugAuthBeacon(pub u32);
impl DebugAuthBeacon {
    #[doc = "Sets by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to the application code."]
    #[must_use]
    #[inline(always)]
    pub const fn beacon(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Sets by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to the application code."]
    #[inline(always)]
    pub const fn set_beacon(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DebugAuthBeacon {
    #[inline(always)]
    fn default() -> DebugAuthBeacon {
        DebugAuthBeacon(0)
    }
}
impl core::fmt::Debug for DebugAuthBeacon {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugAuthBeacon")
            .field("beacon", &self.beacon())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugAuthBeacon {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DebugAuthBeacon {{ beacon: {=u32:?} }}", self.beacon())
    }
}
#[doc = "Cortex Debug Features Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugFeatures(pub u32);
impl DebugFeatures {
    #[doc = "CPU0 invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_dbgen(&self) -> super::vals::DebugFeaturesCpu0Dbgen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DebugFeaturesCpu0Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU0 invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_dbgen(&mut self, val: super::vals::DebugFeaturesCpu0Dbgen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 non-invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_niden(&self) -> super::vals::DebugFeaturesCpu0Niden {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DebugFeaturesCpu0Niden::from_bits(val as u8)
    }
    #[doc = "CPU0 non-invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_niden(&mut self, val: super::vals::DebugFeaturesCpu0Niden) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for DebugFeatures {
    #[inline(always)]
    fn default() -> DebugFeatures {
        DebugFeatures(0)
    }
}
impl core::fmt::Debug for DebugFeatures {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugFeatures")
            .field("cpu0_dbgen", &self.cpu0_dbgen())
            .field("cpu0_niden", &self.cpu0_niden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugFeatures {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DebugFeatures {{ cpu0_dbgen: {:?}, cpu0_niden: {:?} }}",
            self.cpu0_dbgen(),
            self.cpu0_niden()
        )
    }
}
#[doc = "Cortex Debug Features Control (Duplicate)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugFeaturesDp(pub u32);
impl DebugFeaturesDp {
    #[doc = "CPU0 invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_dbgen(&self) -> super::vals::DebugFeaturesDpCpu0Dbgen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DebugFeaturesDpCpu0Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU0 invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_dbgen(&mut self, val: super::vals::DebugFeaturesDpCpu0Dbgen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 non-invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_niden(&self) -> super::vals::DebugFeaturesDpCpu0Niden {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DebugFeaturesDpCpu0Niden::from_bits(val as u8)
    }
    #[doc = "CPU0 non-invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_niden(&mut self, val: super::vals::DebugFeaturesDpCpu0Niden) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for DebugFeaturesDp {
    #[inline(always)]
    fn default() -> DebugFeaturesDp {
        DebugFeaturesDp(0)
    }
}
impl core::fmt::Debug for DebugFeaturesDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugFeaturesDp")
            .field("cpu0_dbgen", &self.cpu0_dbgen())
            .field("cpu0_niden", &self.cpu0_niden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugFeaturesDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DebugFeaturesDp {{ cpu0_dbgen: {:?}, cpu0_niden: {:?} }}",
            self.cpu0_dbgen(),
            self.cpu0_niden()
        )
    }
}
#[doc = "Control Write Access to Security"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugLockEn(pub u32);
impl DebugLockEn {
    #[doc = "Controls write access to the security registers"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_all(&self) -> super::vals::LockAll {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::LockAll::from_bits(val as u8)
    }
    #[doc = "Controls write access to the security registers"]
    #[inline(always)]
    pub const fn set_lock_all(&mut self, val: super::vals::LockAll) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for DebugLockEn {
    #[inline(always)]
    fn default() -> DebugLockEn {
        DebugLockEn(0)
    }
}
impl core::fmt::Debug for DebugLockEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugLockEn")
            .field("lock_all", &self.lock_all())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugLockEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DebugLockEn {{ lock_all: {:?} }}", self.lock_all())
    }
}
#[doc = "Device ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DeviceId0(pub u32);
impl DeviceId0 {
    #[doc = "Indicates the device's ram size"]
    #[must_use]
    #[inline(always)]
    pub const fn ram_size(&self) -> super::vals::RamSize {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::RamSize::from_bits(val as u8)
    }
    #[doc = "Indicates the device's ram size"]
    #[inline(always)]
    pub const fn set_ram_size(&mut self, val: super::vals::RamSize) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Indicates the device's flash size"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_size(&self) -> super::vals::FlashSize {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::FlashSize::from_bits(val as u8)
    }
    #[doc = "Indicates the device's flash size"]
    #[inline(always)]
    pub const fn set_flash_size(&mut self, val: super::vals::FlashSize) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Indicates the device's ROM revision"]
    #[must_use]
    #[inline(always)]
    pub const fn rom_rev_minor(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the device's ROM revision"]
    #[inline(always)]
    pub const fn set_rom_rev_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn security(&self) -> super::vals::Security {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Security::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_security(&mut self, val: super::vals::Security) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for DeviceId0 {
    #[inline(always)]
    fn default() -> DeviceId0 {
        DeviceId0(0)
    }
}
impl core::fmt::Debug for DeviceId0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DeviceId0")
            .field("ram_size", &self.ram_size())
            .field("flash_size", &self.flash_size())
            .field("rom_rev_minor", &self.rom_rev_minor())
            .field("security", &self.security())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DeviceId0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DeviceId0 {{ ram_size: {:?}, flash_size: {:?}, rom_rev_minor: {=u8:?}, security: {:?} }}",
            self.ram_size(),
            self.flash_size(),
            self.rom_rev_minor(),
            self.security()
        )
    }
}
#[doc = "Device Type"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DeviceType(pub u32);
impl DeviceType {
    #[doc = "Indicates the device part number"]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_num(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Indicates the device part number"]
    #[inline(always)]
    pub const fn set_device_type_num(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates the device type"]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_sec(&self) -> super::vals::DeviceTypeSec {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::DeviceTypeSec::from_bits(val as u8)
    }
    #[doc = "Indicates the device type"]
    #[inline(always)]
    pub const fn set_device_type_sec(&mut self, val: super::vals::DeviceTypeSec) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates the device's package type"]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_pkg(&self) -> super::vals::DeviceTypePkg {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::DeviceTypePkg::from_bits(val as u8)
    }
    #[doc = "Indicates the device's package type"]
    #[inline(always)]
    pub const fn set_device_type_pkg(&mut self, val: super::vals::DeviceTypePkg) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Indicates the device's pin number"]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_pin(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the device's pin number"]
    #[inline(always)]
    pub const fn set_device_type_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for DeviceType {
    #[inline(always)]
    fn default() -> DeviceType {
        DeviceType(0)
    }
}
impl core::fmt::Debug for DeviceType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DeviceType")
            .field("device_type_num", &self.device_type_num())
            .field("device_type_sec", &self.device_type_sec())
            .field("device_type_pkg", &self.device_type_pkg())
            .field("device_type_pin", &self.device_type_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DeviceType {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DeviceType {{ device_type_num: {=u16:?}, device_type_sec: {:?}, device_type_pkg: {:?}, device_type_pin: {=u8:?} }}",
            self.device_type_num(),
            self.device_type_sec(),
            self.device_type_pkg(),
            self.device_type_pin()
        )
    }
}
#[doc = "Chip Revision ID and Number"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieid(pub u32);
impl Dieid {
    #[doc = "Chip minor revision"]
    #[must_use]
    #[inline(always)]
    pub const fn minor_revision(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Chip minor revision"]
    #[inline(always)]
    pub const fn set_minor_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Chip major revision"]
    #[must_use]
    #[inline(always)]
    pub const fn major_revision(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Chip major revision"]
    #[inline(always)]
    pub const fn set_major_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Chip number"]
    #[must_use]
    #[inline(always)]
    pub const fn mco_num_in_die_id(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Chip number"]
    #[inline(always)]
    pub const fn set_mco_num_in_die_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 8usize)) | (((val as u32) & 0x000f_ffff) << 8usize);
    }
}
impl Default for Dieid {
    #[inline(always)]
    fn default() -> Dieid {
        Dieid(0)
    }
}
impl core::fmt::Debug for Dieid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dieid")
            .field("minor_revision", &self.minor_revision())
            .field("major_revision", &self.major_revision())
            .field("mco_num_in_die_id", &self.mco_num_in_die_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dieid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dieid {{ minor_revision: {=u8:?}, major_revision: {=u8:?}, mco_num_in_die_id: {=u32:?} }}",
            self.minor_revision(),
            self.major_revision(),
            self.mco_num_in_die_id()
        )
    }
}
#[doc = "Life Cycle State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsOtpLcState(pub u32);
impl ElsOtpLcState {
    #[doc = "OTP life cycle state"]
    #[must_use]
    #[inline(always)]
    pub const fn otp_lc_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "OTP life cycle state"]
    #[inline(always)]
    pub const fn set_otp_lc_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ElsOtpLcState {
    #[inline(always)]
    fn default() -> ElsOtpLcState {
        ElsOtpLcState(0)
    }
}
impl core::fmt::Debug for ElsOtpLcState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsOtpLcState")
            .field("otp_lc_state", &self.otp_lc_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsOtpLcState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsOtpLcState {{ otp_lc_state: {=u8:?} }}",
            self.otp_lc_state()
        )
    }
}
#[doc = "Life Cycle State Register (Duplicate)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsOtpLcStateDp(pub u32);
impl ElsOtpLcStateDp {
    #[doc = "OTP life cycle state"]
    #[must_use]
    #[inline(always)]
    pub const fn otp_lc_state_dp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "OTP life cycle state"]
    #[inline(always)]
    pub const fn set_otp_lc_state_dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ElsOtpLcStateDp {
    #[inline(always)]
    fn default() -> ElsOtpLcStateDp {
        ElsOtpLcStateDp(0)
    }
}
impl core::fmt::Debug for ElsOtpLcStateDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsOtpLcStateDp")
            .field("otp_lc_state_dp", &self.otp_lc_state_dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsOtpLcStateDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsOtpLcStateDp {{ otp_lc_state_dp: {=u8:?} }}",
            self.otp_lc_state_dp()
        )
    }
}
#[doc = "UDF Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsUdf(pub u32);
impl ElsUdf {
    #[doc = "UDF KEY Select"]
    #[must_use]
    #[inline(always)]
    pub const fn key_sel(&self) -> super::vals::KeySel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::KeySel::from_bits(val as u8)
    }
    #[doc = "UDF KEY Select"]
    #[inline(always)]
    pub const fn set_key_sel(&mut self, val: super::vals::KeySel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "UID register hidden control. Write values other than 1010b, locked the write of UID_HIDDEN until a system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn uid_hidden(&self) -> super::vals::UidHidden {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::UidHidden::from_bits(val as u8)
    }
    #[doc = "UID register hidden control. Write values other than 1010b, locked the write of UID_HIDDEN until a system reset."]
    #[inline(always)]
    pub const fn set_uid_hidden(&mut self, val: super::vals::UidHidden) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "UDF register hidden control. Write values other than 1010b, locked the write of UDF_HIDDEN until a system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn udf_hidden(&self) -> super::vals::UdfHidden {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::UdfHidden::from_bits(val as u8)
    }
    #[doc = "UDF register hidden control. Write values other than 1010b, locked the write of UDF_HIDDEN until a system reset."]
    #[inline(always)]
    pub const fn set_udf_hidden(&mut self, val: super::vals::UdfHidden) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for ElsUdf {
    #[inline(always)]
    fn default() -> ElsUdf {
        ElsUdf(0)
    }
}
impl core::fmt::Debug for ElsUdf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsUdf")
            .field("key_sel", &self.key_sel())
            .field("uid_hidden", &self.uid_hidden())
            .field("udf_hidden", &self.udf_hidden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsUdf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsUdf {{ key_sel: {:?}, uid_hidden: {:?}, udf_hidden: {:?} }}",
            self.key_sel(),
            self.uid_hidden(),
            self.udf_hidden()
        )
    }
}
#[doc = "Device UID n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsUid(pub u32);
impl ElsUid {
    #[doc = "UID"]
    #[must_use]
    #[inline(always)]
    pub const fn uid0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "UID"]
    #[inline(always)]
    pub const fn set_uid0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ElsUid {
    #[inline(always)]
    fn default() -> ElsUid {
        ElsUid(0)
    }
}
impl core::fmt::Debug for ElsUid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsUid")
            .field("uid0", &self.uid0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsUid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ElsUid {{ uid0: {=u32:?} }}", self.uid0())
    }
}
#[doc = "FRO_HF_DIV Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frohfdiv(pub u32);
impl Frohfdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::FrohfdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::FrohfdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::FrohfdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::FrohfdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::FrohfdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::FrohfdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::FrohfdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FrohfdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::FrohfdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Frohfdiv {
    #[inline(always)]
    fn default() -> Frohfdiv {
        Frohfdiv(0)
    }
}
impl core::fmt::Debug for Frohfdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frohfdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frohfdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frohfdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FRO_LF_DIV Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frolfdiv(pub u32);
impl Frolfdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::FrolfdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::FrolfdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::FrolfdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::FrolfdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::FrolfdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::FrolfdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::FrolfdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FrolfdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::FrolfdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Frolfdiv {
    #[inline(always)]
    fn default() -> Frolfdiv {
        Frolfdiv(0)
    }
}
impl core::fmt::Debug for Frolfdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frolfdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frolfdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frolfdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Gray to Binary Converter Gray Code \\[31:0\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrayCodeLsb(pub u32);
impl GrayCodeLsb {
    #[doc = "Gray code \\[31:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn code_gray_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Gray code \\[31:0\\]"]
    #[inline(always)]
    pub const fn set_code_gray_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GrayCodeLsb {
    #[inline(always)]
    fn default() -> GrayCodeLsb {
        GrayCodeLsb(0)
    }
}
impl core::fmt::Debug for GrayCodeLsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GrayCodeLsb")
            .field("code_gray_31_0", &self.code_gray_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GrayCodeLsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GrayCodeLsb {{ code_gray_31_0: {=u32:?} }}",
            self.code_gray_31_0()
        )
    }
}
#[doc = "Gray to Binary Converter Gray Code \\[41:32\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrayCodeMsb(pub u32);
impl GrayCodeMsb {
    #[doc = "Gray code \\[41:32\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn code_gray_41_32(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Gray code \\[41:32\\]"]
    #[inline(always)]
    pub const fn set_code_gray_41_32(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for GrayCodeMsb {
    #[inline(always)]
    fn default() -> GrayCodeMsb {
        GrayCodeMsb(0)
    }
}
impl core::fmt::Debug for GrayCodeMsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GrayCodeMsb")
            .field("code_gray_41_32", &self.code_gray_41_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GrayCodeMsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GrayCodeMsb {{ code_gray_41_32: {=u16:?} }}",
            self.code_gray_41_32()
        )
    }
}
#[doc = "JTAG Chip ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JtagId(pub u32);
impl JtagId {
    #[doc = "Indicates the device ID"]
    #[must_use]
    #[inline(always)]
    pub const fn jtag_id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Indicates the device ID"]
    #[inline(always)]
    pub const fn set_jtag_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for JtagId {
    #[inline(always)]
    fn default() -> JtagId {
        JtagId(0)
    }
}
impl core::fmt::Debug for JtagId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JtagId")
            .field("jtag_id", &self.jtag_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JtagId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "JtagId {{ jtag_id: {=u32:?} }}", self.jtag_id())
    }
}
#[doc = "LPCAC Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpcacCtrl(pub u32);
impl LpcacCtrl {
    #[doc = "Disables/enables the cache function."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_lpcac(&self) -> super::vals::DisLpcac {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DisLpcac::from_bits(val as u8)
    }
    #[doc = "Disables/enables the cache function."]
    #[inline(always)]
    pub const fn set_dis_lpcac(&mut self, val: super::vals::DisLpcac) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Clears the cache function."]
    #[must_use]
    #[inline(always)]
    pub const fn clr_lpcac(&self) -> super::vals::ClrLpcac {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ClrLpcac::from_bits(val as u8)
    }
    #[doc = "Clears the cache function."]
    #[inline(always)]
    pub const fn set_clr_lpcac(&mut self, val: super::vals::ClrLpcac) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Forces no allocation."]
    #[must_use]
    #[inline(always)]
    pub const fn frc_no_alloc(&self) -> super::vals::FrcNoAlloc {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::FrcNoAlloc::from_bits(val as u8)
    }
    #[doc = "Forces no allocation."]
    #[inline(always)]
    pub const fn set_frc_no_alloc(&mut self, val: super::vals::FrcNoAlloc) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Disable LPCAC Write Through Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_lpcac_wtbf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Disable LPCAC Write Through Buffer."]
    #[inline(always)]
    pub const fn set_dis_lpcac_wtbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Limit LPCAC Write Through Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn lim_lpcac_wtbf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Limit LPCAC Write Through Buffer."]
    #[inline(always)]
    pub const fn set_lim_lpcac_wtbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LPCAC XOM(eXecute-Only-Memory) attribute control"]
    #[must_use]
    #[inline(always)]
    pub const fn lpcac_xom(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LPCAC XOM(eXecute-Only-Memory) attribute control"]
    #[inline(always)]
    pub const fn set_lpcac_xom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Request LPCAC memories."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcac_mem_req(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Request LPCAC memories."]
    #[inline(always)]
    pub const fn set_lpcac_mem_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for LpcacCtrl {
    #[inline(always)]
    fn default() -> LpcacCtrl {
        LpcacCtrl(0)
    }
}
impl core::fmt::Debug for LpcacCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpcacCtrl")
            .field("dis_lpcac", &self.dis_lpcac())
            .field("clr_lpcac", &self.clr_lpcac())
            .field("frc_no_alloc", &self.frc_no_alloc())
            .field("dis_lpcac_wtbf", &self.dis_lpcac_wtbf())
            .field("lim_lpcac_wtbf", &self.lim_lpcac_wtbf())
            .field("lpcac_xom", &self.lpcac_xom())
            .field("lpcac_mem_req", &self.lpcac_mem_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpcacCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpcacCtrl {{ dis_lpcac: {:?}, clr_lpcac: {:?}, frc_no_alloc: {:?}, dis_lpcac_wtbf: {=bool:?}, lim_lpcac_wtbf: {=bool:?}, lpcac_xom: {=bool:?}, lpcac_mem_req: {=bool:?} }}",
            self.dis_lpcac(),
            self.clr_lpcac(),
            self.frc_no_alloc(),
            self.dis_lpcac_wtbf(),
            self.lim_lpcac_wtbf(),
            self.lpcac_xom(),
            self.lpcac_mem_req()
        )
    }
}
#[doc = "MSF Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msfcfg(pub u32);
impl Msfcfg {
    #[doc = "user IFR sector 0 erase control"]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_erase_dis0(&self) -> super::vals::IfrEraseDis0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::IfrEraseDis0::from_bits(val as u8)
    }
    #[doc = "user IFR sector 0 erase control"]
    #[inline(always)]
    pub const fn set_ifr_erase_dis0(&mut self, val: super::vals::IfrEraseDis0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "user IFR sector 1 erase control"]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_erase_dis1(&self) -> super::vals::IfrEraseDis1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IfrEraseDis1::from_bits(val as u8)
    }
    #[doc = "user IFR sector 1 erase control"]
    #[inline(always)]
    pub const fn set_ifr_erase_dis1(&mut self, val: super::vals::IfrEraseDis1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "user IFR sector 2 erase control"]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_erase_dis2(&self) -> super::vals::IfrEraseDis2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::IfrEraseDis2::from_bits(val as u8)
    }
    #[doc = "user IFR sector 2 erase control"]
    #[inline(always)]
    pub const fn set_ifr_erase_dis2(&mut self, val: super::vals::IfrEraseDis2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "user IFR sector 3 erase control"]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_erase_dis3(&self) -> super::vals::IfrEraseDis3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::IfrEraseDis3::from_bits(val as u8)
    }
    #[doc = "user IFR sector 3 erase control"]
    #[inline(always)]
    pub const fn set_ifr_erase_dis3(&mut self, val: super::vals::IfrEraseDis3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Mass erase control"]
    #[must_use]
    #[inline(always)]
    pub const fn mass_erase_dis(&self) -> super::vals::MassEraseDis {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::MassEraseDis::from_bits(val as u8)
    }
    #[doc = "Mass erase control"]
    #[inline(always)]
    pub const fn set_mass_erase_dis(&mut self, val: super::vals::MassEraseDis) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Msfcfg {
    #[inline(always)]
    fn default() -> Msfcfg {
        Msfcfg(0)
    }
}
impl core::fmt::Debug for Msfcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Msfcfg")
            .field("ifr_erase_dis0", &self.ifr_erase_dis0())
            .field("ifr_erase_dis1", &self.ifr_erase_dis1())
            .field("ifr_erase_dis2", &self.ifr_erase_dis2())
            .field("ifr_erase_dis3", &self.ifr_erase_dis3())
            .field("mass_erase_dis", &self.mass_erase_dis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Msfcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Msfcfg {{ ifr_erase_dis0: {:?}, ifr_erase_dis1: {:?}, ifr_erase_dis2: {:?}, ifr_erase_dis3: {:?}, mass_erase_dis: {:?} }}",
            self.ifr_erase_dis0(),
            self.ifr_erase_dis1(),
            self.ifr_erase_dis2(),
            self.ifr_erase_dis3(),
            self.mass_erase_dis()
        )
    }
}
#[doc = "NMI Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmisrc(pub u32);
impl Nmisrc {
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU0, if enabled by NMIENCPU0."]
    #[must_use]
    #[inline(always)]
    pub const fn irqcpu0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU0, if enabled by NMIENCPU0."]
    #[inline(always)]
    pub const fn set_irqcpu0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[must_use]
    #[inline(always)]
    pub const fn nmiencpu0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[inline(always)]
    pub const fn set_nmiencpu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Nmisrc {
    #[inline(always)]
    fn default() -> Nmisrc {
        Nmisrc(0)
    }
}
impl core::fmt::Debug for Nmisrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nmisrc")
            .field("irqcpu0", &self.irqcpu0())
            .field("nmiencpu0", &self.nmiencpu0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nmisrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nmisrc {{ irqcpu0: {=u8:?}, nmiencpu0: {=bool:?} }}",
            self.irqcpu0(),
            self.nmiencpu0()
        )
    }
}
#[doc = "NVM Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NvmCtrl(pub u32);
impl NvmCtrl {
    #[doc = "Flash speculation control"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_flash_spec(&self) -> super::vals::DisFlashSpec {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DisFlashSpec::from_bits(val as u8)
    }
    #[doc = "Flash speculation control"]
    #[inline(always)]
    pub const fn set_dis_flash_spec(&mut self, val: super::vals::DisFlashSpec) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Flash data speculation control"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_data_spec(&self) -> super::vals::DisDataSpec {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DisDataSpec::from_bits(val as u8)
    }
    #[doc = "Flash data speculation control"]
    #[inline(always)]
    pub const fn set_dis_data_spec(&mut self, val: super::vals::DisDataSpec) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FLASH stall on busy control"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_stall_en(&self) -> super::vals::FlashStallEn {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::FlashStallEn::from_bits(val as u8)
    }
    #[doc = "FLASH stall on busy control"]
    #[inline(always)]
    pub const fn set_flash_stall_en(&mut self, val: super::vals::FlashStallEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Bus error on data multi-bit ECC error control Set this field to 0 if you want to enable flash speculative"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_mbecc_err_inst(&self) -> super::vals::DisMbeccErrInst {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::DisMbeccErrInst::from_bits(val as u8)
    }
    #[doc = "Bus error on data multi-bit ECC error control Set this field to 0 if you want to enable flash speculative"]
    #[inline(always)]
    pub const fn set_dis_mbecc_err_inst(&mut self, val: super::vals::DisMbeccErrInst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Bus error on data multi-bit ECC error control Set this field to 0 if you want to enable flash speculative"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_mbecc_err_data(&self) -> super::vals::DisMbeccErrData {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::DisMbeccErrData::from_bits(val as u8)
    }
    #[doc = "Bus error on data multi-bit ECC error control Set this field to 0 if you want to enable flash speculative"]
    #[inline(always)]
    pub const fn set_dis_mbecc_err_data(&mut self, val: super::vals::DisMbeccErrData) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
}
impl Default for NvmCtrl {
    #[inline(always)]
    fn default() -> NvmCtrl {
        NvmCtrl(0)
    }
}
impl core::fmt::Debug for NvmCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NvmCtrl")
            .field("dis_flash_spec", &self.dis_flash_spec())
            .field("dis_data_spec", &self.dis_data_spec())
            .field("flash_stall_en", &self.flash_stall_en())
            .field("dis_mbecc_err_inst", &self.dis_mbecc_err_inst())
            .field("dis_mbecc_err_data", &self.dis_mbecc_err_data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NvmCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NvmCtrl {{ dis_flash_spec: {:?}, dis_data_spec: {:?}, flash_stall_en: {:?}, dis_mbecc_err_inst: {:?}, dis_mbecc_err_data: {:?} }}",
            self.dis_flash_spec(),
            self.dis_data_spec(),
            self.flash_stall_en(),
            self.dis_mbecc_err_inst(),
            self.dis_mbecc_err_data()
        )
    }
}
#[doc = "PLL1_CLK_DIV Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll1clkdiv(pub u32);
impl Pll1clkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Pll1clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pll1clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Pll1clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Pll1clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Pll1clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Pll1clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Pll1clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pll1clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Pll1clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pll1clkdiv {
    #[inline(always)]
    fn default() -> Pll1clkdiv {
        Pll1clkdiv(0)
    }
}
impl core::fmt::Debug for Pll1clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll1clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll1clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll1clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Protect Level Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Protlvl(pub u32);
impl Protlvl {
    #[doc = "Control privileged access of EIM, ERM, Flexcan, MBC, SCG."]
    #[must_use]
    #[inline(always)]
    pub const fn priv_(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control privileged access of EIM, ERM, Flexcan, MBC, SCG."]
    #[inline(always)]
    pub const fn set_priv_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control write access to Nonsecure MPU memory regions."]
    #[must_use]
    #[inline(always)]
    pub const fn locknsmpu(&self) -> super::vals::Locknsmpu {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Locknsmpu::from_bits(val as u8)
    }
    #[doc = "Control write access to Nonsecure MPU memory regions."]
    #[inline(always)]
    pub const fn set_locknsmpu(&mut self, val: super::vals::Locknsmpu) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to the this register to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::ProtlvlLock {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ProtlvlLock::from_bits(val as u8)
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to the this register to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::ProtlvlLock) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Protlvl {
    #[inline(always)]
    fn default() -> Protlvl {
        Protlvl(0)
    }
}
impl core::fmt::Debug for Protlvl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Protlvl")
            .field("priv_", &self.priv_())
            .field("locknsmpu", &self.locknsmpu())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Protlvl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Protlvl {{ priv_: {=bool:?}, locknsmpu: {:?}, lock: {:?} }}",
            self.priv_(),
            self.locknsmpu(),
            self.lock()
        )
    }
}
#[doc = "PWM0 Submodule Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm0subctl(pub u32);
impl Pwm0subctl {
    #[doc = "Enables PWM0 SUB Clock0"]
    #[must_use]
    #[inline(always)]
    pub const fn clk0_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM0 SUB Clock0"]
    #[inline(always)]
    pub const fn set_clk0_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables PWM0 SUB Clock1"]
    #[must_use]
    #[inline(always)]
    pub const fn clk1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM0 SUB Clock1"]
    #[inline(always)]
    pub const fn set_clk1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables PWM0 SUB Clock2"]
    #[must_use]
    #[inline(always)]
    pub const fn clk2_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM0 SUB Clock2"]
    #[inline(always)]
    pub const fn set_clk2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables PWM0 SUB Clock3"]
    #[must_use]
    #[inline(always)]
    pub const fn clk3_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM0 SUB Clock3"]
    #[inline(always)]
    pub const fn set_clk3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Pwm0subctl {
    #[inline(always)]
    fn default() -> Pwm0subctl {
        Pwm0subctl(0)
    }
}
impl core::fmt::Debug for Pwm0subctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwm0subctl")
            .field("clk0_en", &self.clk0_en())
            .field("clk1_en", &self.clk1_en())
            .field("clk2_en", &self.clk2_en())
            .field("clk3_en", &self.clk3_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwm0subctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwm0subctl {{ clk0_en: {=bool:?}, clk1_en: {=bool:?}, clk2_en: {=bool:?}, clk3_en: {=bool:?} }}",
            self.clk0_en(),
            self.clk1_en(),
            self.clk2_en(),
            self.clk3_en()
        )
    }
}
#[doc = "PWM1 Submodule Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm1subctl(pub u32);
impl Pwm1subctl {
    #[doc = "Enables PWM1 SUB Clock0"]
    #[must_use]
    #[inline(always)]
    pub const fn clk0_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM1 SUB Clock0"]
    #[inline(always)]
    pub const fn set_clk0_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables PWM1 SUB Clock1"]
    #[must_use]
    #[inline(always)]
    pub const fn clk1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM1 SUB Clock1"]
    #[inline(always)]
    pub const fn set_clk1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables PWM1 SUB Clock2"]
    #[must_use]
    #[inline(always)]
    pub const fn clk2_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM1 SUB Clock2"]
    #[inline(always)]
    pub const fn set_clk2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables PWM1 SUB Clock3"]
    #[must_use]
    #[inline(always)]
    pub const fn clk3_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM1 SUB Clock3"]
    #[inline(always)]
    pub const fn set_clk3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Pwm1subctl {
    #[inline(always)]
    fn default() -> Pwm1subctl {
        Pwm1subctl(0)
    }
}
impl core::fmt::Debug for Pwm1subctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwm1subctl")
            .field("clk0_en", &self.clk0_en())
            .field("clk1_en", &self.clk1_en())
            .field("clk2_en", &self.clk2_en())
            .field("clk3_en", &self.clk3_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwm1subctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwm1subctl {{ clk0_en: {=bool:?}, clk1_en: {=bool:?}, clk2_en: {=bool:?}, clk3_en: {=bool:?} }}",
            self.clk0_en(),
            self.clk1_en(),
            self.clk2_en(),
            self.clk3_en()
        )
    }
}
#[doc = "RAM Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamCtrl(pub u32);
impl RamCtrl {
    #[doc = "RAMA ECC enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rama_ecc_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA ECC enable"]
    #[inline(always)]
    pub const fn set_rama_ecc_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RAMA bank clock gating control, only avaiable when RAMA_ECC_ENABLE = 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rama_cg_override(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA bank clock gating control, only avaiable when RAMA_ECC_ENABLE = 0."]
    #[inline(always)]
    pub const fn set_rama_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "RAMX bank clock gating control"]
    #[must_use]
    #[inline(always)]
    pub const fn ramx_cg_override(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "RAMX bank clock gating control"]
    #[inline(always)]
    pub const fn set_ramx_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "RAMB bank clock gating control"]
    #[must_use]
    #[inline(always)]
    pub const fn ramb_cg_override(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB bank clock gating control"]
    #[inline(always)]
    pub const fn set_ramb_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "RAMC bank clock gating control"]
    #[must_use]
    #[inline(always)]
    pub const fn ramc_cg_override(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "RAMC bank clock gating control"]
    #[inline(always)]
    pub const fn set_ramc_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for RamCtrl {
    #[inline(always)]
    fn default() -> RamCtrl {
        RamCtrl(0)
    }
}
impl core::fmt::Debug for RamCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamCtrl")
            .field("rama_ecc_enable", &self.rama_ecc_enable())
            .field("rama_cg_override", &self.rama_cg_override())
            .field("ramx_cg_override", &self.ramx_cg_override())
            .field("ramb_cg_override", &self.ramb_cg_override())
            .field("ramc_cg_override", &self.ramc_cg_override())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamCtrl {{ rama_ecc_enable: {=bool:?}, rama_cg_override: {=bool:?}, ramx_cg_override: {=bool:?}, ramb_cg_override: {=bool:?}, ramc_cg_override: {=bool:?} }}",
            self.rama_ecc_enable(),
            self.rama_cg_override(),
            self.ramx_cg_override(),
            self.ramb_cg_override(),
            self.ramc_cg_override()
        )
    }
}
#[doc = "Controls RAM Interleave Integration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamInterleave(pub u32);
impl RamInterleave {
    #[doc = "Controls RAM access for RAMA1 and RAMA2"]
    #[must_use]
    #[inline(always)]
    pub const fn interleave(&self) -> super::vals::Interleave {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Interleave::from_bits(val as u8)
    }
    #[doc = "Controls RAM access for RAMA1 and RAMA2"]
    #[inline(always)]
    pub const fn set_interleave(&mut self, val: super::vals::Interleave) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for RamInterleave {
    #[inline(always)]
    fn default() -> RamInterleave {
        RamInterleave(0)
    }
}
impl core::fmt::Debug for RamInterleave {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamInterleave")
            .field("interleave", &self.interleave())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamInterleave {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RamInterleave {{ interleave: {:?} }}", self.interleave())
    }
}
#[doc = "AHB Matrix Remap Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Remap(pub u32);
impl Remap {
    #[doc = "RAMX0 address remap for CPU System bus"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_sbus(&self) -> super::vals::RemapCpu0Sbus {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::RemapCpu0Sbus::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for CPU System bus"]
    #[inline(always)]
    pub const fn set_cpu0_sbus(&mut self, val: super::vals::RemapCpu0Sbus) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RAMX0 address remap for SmartDMA D-BUS"]
    #[must_use]
    #[inline(always)]
    pub const fn smart_dma_d(&self) -> super::vals::SmartDmaD {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SmartDmaD::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for SmartDMA D-BUS"]
    #[inline(always)]
    pub const fn set_smart_dma_d(&mut self, val: super::vals::SmartDmaD) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "RAMX0 address remap for SmartDMA I-BUS"]
    #[must_use]
    #[inline(always)]
    pub const fn smart_dma_i(&self) -> super::vals::SmartDmaI {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::SmartDmaI::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for SmartDMA I-BUS"]
    #[inline(always)]
    pub const fn set_smart_dma_i(&mut self, val: super::vals::SmartDmaI) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "RAMX0 address remap for DMA0"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> super::vals::RemapDma0 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RemapDma0::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for DMA0"]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: super::vals::RemapDma0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "RAMX0 address remap for PKC"]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> super::vals::Pkc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Pkc::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for PKC"]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: super::vals::Pkc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "RAMX0 address remap for USB0"]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> super::vals::Usb0 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Usb0::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for USB0"]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: super::vals::Usb0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to the this register to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::RemapLock {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::RemapLock::from_bits(val as u8)
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to the this register to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::RemapLock) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Remap {
    #[inline(always)]
    fn default() -> Remap {
        Remap(0)
    }
}
impl core::fmt::Debug for Remap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Remap")
            .field("cpu0_sbus", &self.cpu0_sbus())
            .field("smart_dma_d", &self.smart_dma_d())
            .field("smart_dma_i", &self.smart_dma_i())
            .field("dma0", &self.dma0())
            .field("pkc", &self.pkc())
            .field("usb0", &self.usb0())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Remap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Remap {{ cpu0_sbus: {:?}, smart_dma_d: {:?}, smart_dma_i: {:?}, dma0: {:?}, pkc: {:?}, usb0: {:?}, lock: {:?} }}",
            self.cpu0_sbus(),
            self.smart_dma_d(),
            self.smart_dma_i(),
            self.dma0(),
            self.pkc(),
            self.usb0(),
            self.lock()
        )
    }
}
#[doc = "ROP State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RopState(pub u32);
impl RopState {
    #[doc = "ROP state"]
    #[must_use]
    #[inline(always)]
    pub const fn rop_state(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ROP state"]
    #[inline(always)]
    pub const fn set_rop_state(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RopState {
    #[inline(always)]
    fn default() -> RopState {
        RopState(0)
    }
}
impl core::fmt::Debug for RopState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RopState")
            .field("rop_state", &self.rop_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RopState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RopState {{ rop_state: {=u32:?} }}", self.rop_state())
    }
}
#[doc = "SLOW_CLK Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slowclkdiv(pub u32);
impl Slowclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::SlowclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SlowclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::SlowclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::SlowclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::SlowclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::SlowclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::SlowclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SlowclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::SlowclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Slowclkdiv {
    #[inline(always)]
    fn default() -> Slowclkdiv {
        Slowclkdiv(0)
    }
}
impl core::fmt::Debug for Slowclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slowclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slowclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Slowclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "SmartDMA Interrupt Hijack"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmartDmaint(pub u32);
impl SmartDmaint {
    #[doc = "SmartDMA hijack NVIC IRQ1"]
    #[must_use]
    #[inline(always)]
    pub const fn int0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ1"]
    #[inline(always)]
    pub const fn set_int0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ17"]
    #[must_use]
    #[inline(always)]
    pub const fn int1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ17"]
    #[inline(always)]
    pub const fn set_int1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ18"]
    #[must_use]
    #[inline(always)]
    pub const fn int2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ18"]
    #[inline(always)]
    pub const fn set_int2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ29"]
    #[must_use]
    #[inline(always)]
    pub const fn int3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ29"]
    #[inline(always)]
    pub const fn set_int3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ30"]
    #[must_use]
    #[inline(always)]
    pub const fn int4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ30"]
    #[inline(always)]
    pub const fn set_int4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ31"]
    #[must_use]
    #[inline(always)]
    pub const fn int5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ31"]
    #[inline(always)]
    pub const fn set_int5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ32"]
    #[must_use]
    #[inline(always)]
    pub const fn int6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ32"]
    #[inline(always)]
    pub const fn set_int6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ33"]
    #[must_use]
    #[inline(always)]
    pub const fn int7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ33"]
    #[inline(always)]
    pub const fn set_int7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ34"]
    #[must_use]
    #[inline(always)]
    pub const fn int8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ34"]
    #[inline(always)]
    pub const fn set_int8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ35"]
    #[must_use]
    #[inline(always)]
    pub const fn int9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ35"]
    #[inline(always)]
    pub const fn set_int9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ36"]
    #[must_use]
    #[inline(always)]
    pub const fn int10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ36"]
    #[inline(always)]
    pub const fn set_int10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ37"]
    #[must_use]
    #[inline(always)]
    pub const fn int11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ37"]
    #[inline(always)]
    pub const fn set_int11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ38"]
    #[must_use]
    #[inline(always)]
    pub const fn int12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ38"]
    #[inline(always)]
    pub const fn set_int12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ39"]
    #[must_use]
    #[inline(always)]
    pub const fn int13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ39"]
    #[inline(always)]
    pub const fn set_int13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ40"]
    #[must_use]
    #[inline(always)]
    pub const fn int14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ40"]
    #[inline(always)]
    pub const fn set_int14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ41"]
    #[must_use]
    #[inline(always)]
    pub const fn int15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ41"]
    #[inline(always)]
    pub const fn set_int15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ42"]
    #[must_use]
    #[inline(always)]
    pub const fn int16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ42"]
    #[inline(always)]
    pub const fn set_int16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ45"]
    #[must_use]
    #[inline(always)]
    pub const fn int17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ45"]
    #[inline(always)]
    pub const fn set_int17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ47"]
    #[must_use]
    #[inline(always)]
    pub const fn int18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ47"]
    #[inline(always)]
    pub const fn set_int18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ50"]
    #[must_use]
    #[inline(always)]
    pub const fn int19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ50"]
    #[inline(always)]
    pub const fn set_int19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ51"]
    #[must_use]
    #[inline(always)]
    pub const fn int20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ51"]
    #[inline(always)]
    pub const fn set_int20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ66"]
    #[must_use]
    #[inline(always)]
    pub const fn int21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ66"]
    #[inline(always)]
    pub const fn set_int21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ67"]
    #[must_use]
    #[inline(always)]
    pub const fn int22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ67"]
    #[inline(always)]
    pub const fn set_int22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ77"]
    #[must_use]
    #[inline(always)]
    pub const fn int23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ77"]
    #[inline(always)]
    pub const fn set_int23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for SmartDmaint {
    #[inline(always)]
    fn default() -> SmartDmaint {
        SmartDmaint(0)
    }
}
impl core::fmt::Debug for SmartDmaint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmartDmaint")
            .field("int0", &self.int0())
            .field("int1", &self.int1())
            .field("int2", &self.int2())
            .field("int3", &self.int3())
            .field("int4", &self.int4())
            .field("int5", &self.int5())
            .field("int6", &self.int6())
            .field("int7", &self.int7())
            .field("int8", &self.int8())
            .field("int9", &self.int9())
            .field("int10", &self.int10())
            .field("int11", &self.int11())
            .field("int12", &self.int12())
            .field("int13", &self.int13())
            .field("int14", &self.int14())
            .field("int15", &self.int15())
            .field("int16", &self.int16())
            .field("int17", &self.int17())
            .field("int18", &self.int18())
            .field("int19", &self.int19())
            .field("int20", &self.int20())
            .field("int21", &self.int21())
            .field("int22", &self.int22())
            .field("int23", &self.int23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmartDmaint {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmartDmaint {{ int0: {=bool:?}, int1: {=bool:?}, int2: {=bool:?}, int3: {=bool:?}, int4: {=bool:?}, int5: {=bool:?}, int6: {=bool:?}, int7: {=bool:?}, int8: {=bool:?}, int9: {=bool:?}, int10: {=bool:?}, int11: {=bool:?}, int12: {=bool:?}, int13: {=bool:?}, int14: {=bool:?}, int15: {=bool:?}, int16: {=bool:?}, int17: {=bool:?}, int18: {=bool:?}, int19: {=bool:?}, int20: {=bool:?}, int21: {=bool:?}, int22: {=bool:?}, int23: {=bool:?} }}",
            self.int0(),
            self.int1(),
            self.int2(),
            self.int3(),
            self.int4(),
            self.int5(),
            self.int6(),
            self.int7(),
            self.int8(),
            self.int9(),
            self.int10(),
            self.int11(),
            self.int12(),
            self.int13(),
            self.int14(),
            self.int15(),
            self.int16(),
            self.int17(),
            self.int18(),
            self.int19(),
            self.int20(),
            self.int21(),
            self.int22(),
            self.int23()
        )
    }
}
#[doc = "RAM XEN Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SramXen(pub u32);
impl SramXen {
    #[doc = "RAMX0 Execute permission control."]
    #[must_use]
    #[inline(always)]
    pub const fn ramx0_xen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RAMX0 Execute permission control."]
    #[inline(always)]
    pub const fn set_ramx0_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RAMX1 Execute permission control."]
    #[must_use]
    #[inline(always)]
    pub const fn ramx1_xen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RAMX1 Execute permission control."]
    #[inline(always)]
    pub const fn set_ramx1_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RAMA0 Execute permission control."]
    #[must_use]
    #[inline(always)]
    pub const fn rama0_xen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA0 Execute permission control."]
    #[inline(always)]
    pub const fn set_rama0_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RAMAx (excepts RAMA0) Execute permission control."]
    #[must_use]
    #[inline(always)]
    pub const fn rama1_xen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RAMAx (excepts RAMA0) Execute permission control."]
    #[inline(always)]
    pub const fn set_rama1_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RAMBx Execute permission control."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb_xen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RAMBx Execute permission control."]
    #[inline(always)]
    pub const fn set_ramb_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RAMCx Execute permission control."]
    #[must_use]
    #[inline(always)]
    pub const fn ramc_xen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RAMCx Execute permission control."]
    #[inline(always)]
    pub const fn set_ramc_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to the this register (and SRAM_XEN_DP) to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::SramXenLock {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SramXenLock::from_bits(val as u8)
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to the this register (and SRAM_XEN_DP) to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::SramXenLock) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SramXen {
    #[inline(always)]
    fn default() -> SramXen {
        SramXen(0)
    }
}
impl core::fmt::Debug for SramXen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SramXen")
            .field("ramx0_xen", &self.ramx0_xen())
            .field("ramx1_xen", &self.ramx1_xen())
            .field("rama0_xen", &self.rama0_xen())
            .field("rama1_xen", &self.rama1_xen())
            .field("ramb_xen", &self.ramb_xen())
            .field("ramc_xen", &self.ramc_xen())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SramXen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SramXen {{ ramx0_xen: {=bool:?}, ramx1_xen: {=bool:?}, rama0_xen: {=bool:?}, rama1_xen: {=bool:?}, ramb_xen: {=bool:?}, ramc_xen: {=bool:?}, lock: {:?} }}",
            self.ramx0_xen(),
            self.ramx1_xen(),
            self.rama0_xen(),
            self.rama1_xen(),
            self.ramb_xen(),
            self.ramc_xen(),
            self.lock()
        )
    }
}
#[doc = "RAM XEN Control (Duplicate)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SramXenDp(pub u32);
impl SramXenDp {
    #[doc = "Refer to SRAM_XEN for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn ramx0_xen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[inline(always)]
    pub const fn set_ramx0_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn ramx1_xen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[inline(always)]
    pub const fn set_ramx1_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn rama0_xen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[inline(always)]
    pub const fn set_rama0_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn rama1_xen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[inline(always)]
    pub const fn set_rama1_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb_xen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[inline(always)]
    pub const fn set_ramb_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn ramc_xen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[inline(always)]
    pub const fn set_ramc_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for SramXenDp {
    #[inline(always)]
    fn default() -> SramXenDp {
        SramXenDp(0)
    }
}
impl core::fmt::Debug for SramXenDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SramXenDp")
            .field("ramx0_xen", &self.ramx0_xen())
            .field("ramx1_xen", &self.ramx1_xen())
            .field("rama0_xen", &self.rama0_xen())
            .field("rama1_xen", &self.rama1_xen())
            .field("ramb_xen", &self.ramb_xen())
            .field("ramc_xen", &self.ramc_xen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SramXenDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SramXenDp {{ ramx0_xen: {=bool:?}, ramx1_xen: {=bool:?}, rama0_xen: {=bool:?}, rama1_xen: {=bool:?}, ramb_xen: {=bool:?}, ramc_xen: {=bool:?} }}",
            self.ramx0_xen(),
            self.ramx1_xen(),
            self.rama0_xen(),
            self.rama1_xen(),
            self.ramb_xen(),
            self.ramc_xen()
        )
    }
}
#[doc = "CPU0 Software Debug Access"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwdAccessCpu0(pub u32);
impl SwdAccessCpu0 {
    #[doc = "CPU0 SWD-AP: 0x12345678"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_code(&self) -> super::vals::SecCode {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::SecCode::from_bits(val as u32)
    }
    #[doc = "CPU0 SWD-AP: 0x12345678"]
    #[inline(always)]
    pub const fn set_sec_code(&mut self, val: super::vals::SecCode) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SwdAccessCpu0 {
    #[inline(always)]
    fn default() -> SwdAccessCpu0 {
        SwdAccessCpu0(0)
    }
}
impl core::fmt::Debug for SwdAccessCpu0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwdAccessCpu0")
            .field("sec_code", &self.sec_code())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessCpu0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SwdAccessCpu0 {{ sec_code: {:?} }}", self.sec_code())
    }
}
