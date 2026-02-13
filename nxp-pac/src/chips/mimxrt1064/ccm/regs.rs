#[doc = "CCM Arm Clock Root Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cacrr(pub u32);
impl Cacrr {
    #[doc = "Divider for Arm clock root"]
    #[must_use]
    #[inline(always)]
    pub const fn arm_podf(&self) -> super::vals::ArmPodf {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::ArmPodf::from_bits(val as u8)
    }
    #[doc = "Divider for Arm clock root"]
    #[inline(always)]
    pub const fn set_arm_podf(&mut self, val: super::vals::ArmPodf) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Cacrr {
    #[inline(always)]
    fn default() -> Cacrr {
        Cacrr(0)
    }
}
impl core::fmt::Debug for Cacrr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cacrr")
            .field("arm_podf", &self.arm_podf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cacrr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cacrr {{ arm_podf: {:?} }}", self.arm_podf())
    }
}
#[doc = "CCM Bus Clock Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbcdr(pub u32);
impl Cbcdr {
    #[doc = "SEMC clock source select"]
    #[must_use]
    #[inline(always)]
    pub const fn semc_clk_sel(&self) -> super::vals::SemcClkSel {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SemcClkSel::from_bits(val as u8)
    }
    #[doc = "SEMC clock source select"]
    #[inline(always)]
    pub const fn set_semc_clk_sel(&mut self, val: super::vals::SemcClkSel) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "SEMC alternative clock select"]
    #[must_use]
    #[inline(always)]
    pub const fn semc_alt_clk_sel(&self) -> super::vals::SemcAltClkSel {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SemcAltClkSel::from_bits(val as u8)
    }
    #[doc = "SEMC alternative clock select"]
    #[inline(always)]
    pub const fn set_semc_alt_clk_sel(&mut self, val: super::vals::SemcAltClkSel) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Divider for ipg podf."]
    #[must_use]
    #[inline(always)]
    pub const fn ipg_podf(&self) -> super::vals::IpgPodf {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IpgPodf::from_bits(val as u8)
    }
    #[doc = "Divider for ipg podf."]
    #[inline(always)]
    pub const fn set_ipg_podf(&mut self, val: super::vals::IpgPodf) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Divider for AHB PODF"]
    #[must_use]
    #[inline(always)]
    pub const fn ahb_podf(&self) -> super::vals::AhbPodf {
        let val = (self.0 >> 10usize) & 0x07;
        super::vals::AhbPodf::from_bits(val as u8)
    }
    #[doc = "Divider for AHB PODF"]
    #[inline(always)]
    pub const fn set_ahb_podf(&mut self, val: super::vals::AhbPodf) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
    }
    #[doc = "Post divider for SEMC clock"]
    #[must_use]
    #[inline(always)]
    pub const fn semc_podf(&self) -> super::vals::SemcPodf {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::SemcPodf::from_bits(val as u8)
    }
    #[doc = "Post divider for SEMC clock"]
    #[inline(always)]
    pub const fn set_semc_podf(&mut self, val: super::vals::SemcPodf) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Selector for peripheral main clock"]
    #[must_use]
    #[inline(always)]
    pub const fn periph_clk_sel(&self) -> super::vals::PeriphClkSel {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::PeriphClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for peripheral main clock"]
    #[inline(always)]
    pub const fn set_periph_clk_sel(&mut self, val: super::vals::PeriphClkSel) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Divider for periph_clk2_podf."]
    #[must_use]
    #[inline(always)]
    pub const fn periph_clk2_podf(&self) -> super::vals::PeriphClk2Podf {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::PeriphClk2Podf::from_bits(val as u8)
    }
    #[doc = "Divider for periph_clk2_podf."]
    #[inline(always)]
    pub const fn set_periph_clk2_podf(&mut self, val: super::vals::PeriphClk2Podf) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
}
impl Default for Cbcdr {
    #[inline(always)]
    fn default() -> Cbcdr {
        Cbcdr(0)
    }
}
impl core::fmt::Debug for Cbcdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cbcdr")
            .field("semc_clk_sel", &self.semc_clk_sel())
            .field("semc_alt_clk_sel", &self.semc_alt_clk_sel())
            .field("ipg_podf", &self.ipg_podf())
            .field("ahb_podf", &self.ahb_podf())
            .field("semc_podf", &self.semc_podf())
            .field("periph_clk_sel", &self.periph_clk_sel())
            .field("periph_clk2_podf", &self.periph_clk2_podf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cbcdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cbcdr {{ semc_clk_sel: {:?}, semc_alt_clk_sel: {:?}, ipg_podf: {:?}, ahb_podf: {:?}, semc_podf: {:?}, periph_clk_sel: {:?}, periph_clk2_podf: {:?} }}",
            self.semc_clk_sel(),
            self.semc_alt_clk_sel(),
            self.ipg_podf(),
            self.ahb_podf(),
            self.semc_podf(),
            self.periph_clk_sel(),
            self.periph_clk2_podf()
        )
    }
}
#[doc = "CCM Bus Clock Multiplexer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbcmr(pub u32);
impl Cbcmr {
    #[doc = "Selector for lpspi clock multiplexer"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi_clk_sel(&self) -> super::vals::LpspiClkSel {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::LpspiClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for lpspi clock multiplexer"]
    #[inline(always)]
    pub const fn set_lpspi_clk_sel(&mut self, val: super::vals::LpspiClkSel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Selector for flexspi2 clock multiplexer"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi2_clk_sel(&self) -> super::vals::Flexspi2ClkSel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Flexspi2ClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for flexspi2 clock multiplexer"]
    #[inline(always)]
    pub const fn set_flexspi2_clk_sel(&mut self, val: super::vals::Flexspi2ClkSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Selector for peripheral clk2 clock multiplexer"]
    #[must_use]
    #[inline(always)]
    pub const fn periph_clk2_sel(&self) -> super::vals::PeriphClk2Sel {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::PeriphClk2Sel::from_bits(val as u8)
    }
    #[doc = "Selector for peripheral clk2 clock multiplexer"]
    #[inline(always)]
    pub const fn set_periph_clk2_sel(&mut self, val: super::vals::PeriphClk2Sel) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Selector for Trace clock multiplexer"]
    #[must_use]
    #[inline(always)]
    pub const fn trace_clk_sel(&self) -> super::vals::TraceClkSel {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::TraceClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for Trace clock multiplexer"]
    #[inline(always)]
    pub const fn set_trace_clk_sel(&mut self, val: super::vals::TraceClkSel) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Selector for pre_periph clock multiplexer"]
    #[must_use]
    #[inline(always)]
    pub const fn pre_periph_clk_sel(&self) -> super::vals::PrePeriphClkSel {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::PrePeriphClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for pre_periph clock multiplexer"]
    #[inline(always)]
    pub const fn set_pre_periph_clk_sel(&mut self, val: super::vals::PrePeriphClkSel) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Post-divider for LCDIF clock."]
    #[must_use]
    #[inline(always)]
    pub const fn lcdif_podf(&self) -> super::vals::LcdifPodf {
        let val = (self.0 >> 23usize) & 0x07;
        super::vals::LcdifPodf::from_bits(val as u8)
    }
    #[doc = "Post-divider for LCDIF clock."]
    #[inline(always)]
    pub const fn set_lcdif_podf(&mut self, val: super::vals::LcdifPodf) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val.to_bits() as u32) & 0x07) << 23usize);
    }
    #[doc = "Divider for LPSPI. Divider should be updated when output clock is gated."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi_podf(&self) -> super::vals::LpspiPodf {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::LpspiPodf::from_bits(val as u8)
    }
    #[doc = "Divider for LPSPI. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn set_lpspi_podf(&mut self, val: super::vals::LpspiPodf) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "Divider for flexspi2 clock root."]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi2_podf(&self) -> super::vals::Flexspi2Podf {
        let val = (self.0 >> 29usize) & 0x07;
        super::vals::Flexspi2Podf::from_bits(val as u8)
    }
    #[doc = "Divider for flexspi2 clock root."]
    #[inline(always)]
    pub const fn set_flexspi2_podf(&mut self, val: super::vals::Flexspi2Podf) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
    }
}
impl Default for Cbcmr {
    #[inline(always)]
    fn default() -> Cbcmr {
        Cbcmr(0)
    }
}
impl core::fmt::Debug for Cbcmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cbcmr")
            .field("lpspi_clk_sel", &self.lpspi_clk_sel())
            .field("flexspi2_clk_sel", &self.flexspi2_clk_sel())
            .field("periph_clk2_sel", &self.periph_clk2_sel())
            .field("trace_clk_sel", &self.trace_clk_sel())
            .field("pre_periph_clk_sel", &self.pre_periph_clk_sel())
            .field("lcdif_podf", &self.lcdif_podf())
            .field("lpspi_podf", &self.lpspi_podf())
            .field("flexspi2_podf", &self.flexspi2_podf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cbcmr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cbcmr {{ lpspi_clk_sel: {:?}, flexspi2_clk_sel: {:?}, periph_clk2_sel: {:?}, trace_clk_sel: {:?}, pre_periph_clk_sel: {:?}, lcdif_podf: {:?}, lpspi_podf: {:?}, flexspi2_podf: {:?} }}",
            self.lpspi_clk_sel(),
            self.flexspi2_clk_sel(),
            self.periph_clk2_sel(),
            self.trace_clk_sel(),
            self.pre_periph_clk_sel(),
            self.lcdif_podf(),
            self.lpspi_podf(),
            self.flexspi2_podf()
        )
    }
}
#[doc = "CCM Clock Gating Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccgr0(pub u32);
impl Ccgr0 {
    #[doc = "aips_tz1 clocks (aips_tz1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "aips_tz1 clocks (aips_tz1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "aips_tz2 clocks (aips_tz2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "aips_tz2 clocks (aips_tz2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "mqs clock ( mqs_hmclk_clock_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "mqs clock ( mqs_hmclk_clock_enable)"]
    #[inline(always)]
    pub const fn set_cg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn cg3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "sim_m or sim_main register access clock (sim_m_mainclk_r_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "sim_m or sim_main register access clock (sim_m_mainclk_r_enable)"]
    #[inline(always)]
    pub const fn set_cg4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "dcp clock (dcp_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "dcp clock (dcp_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "lpuart3 clock (lpuart3_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "lpuart3 clock (lpuart3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "can1 clock (can1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "can1 clock (can1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "can1_serial clock (can1_serial_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "can1_serial clock (can1_serial_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "can2 clock (can2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "can2 clock (can2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "can2_serial clock (can2_serial_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "can2_serial clock (can2_serial_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "trace clock (trace_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "trace clock (trace_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "gpt2 bus clocks (gpt2_bus_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "gpt2 bus clocks (gpt2_bus_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "gpt2 serial clocks (gpt2_serial_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "gpt2 serial clocks (gpt2_serial_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "lpuart2 clock (lpuart2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "lpuart2 clock (lpuart2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "gpio2_clocks (gpio2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "gpio2_clocks (gpio2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ccgr0 {
    #[inline(always)]
    fn default() -> Ccgr0 {
        Ccgr0(0)
    }
}
impl core::fmt::Debug for Ccgr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccgr0")
            .field("cg0", &self.cg0())
            .field("cg1", &self.cg1())
            .field("cg2", &self.cg2())
            .field("cg3", &self.cg3())
            .field("cg4", &self.cg4())
            .field("cg5", &self.cg5())
            .field("cg6", &self.cg6())
            .field("cg7", &self.cg7())
            .field("cg8", &self.cg8())
            .field("cg9", &self.cg9())
            .field("cg10", &self.cg10())
            .field("cg11", &self.cg11())
            .field("cg12", &self.cg12())
            .field("cg13", &self.cg13())
            .field("cg14", &self.cg14())
            .field("cg15", &self.cg15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccgr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccgr0 {{ cg0: {=u8:?}, cg1: {=u8:?}, cg2: {=u8:?}, cg3: {=u8:?}, cg4: {=u8:?}, cg5: {=u8:?}, cg6: {=u8:?}, cg7: {=u8:?}, cg8: {=u8:?}, cg9: {=u8:?}, cg10: {=u8:?}, cg11: {=u8:?}, cg12: {=u8:?}, cg13: {=u8:?}, cg14: {=u8:?}, cg15: {=u8:?} }}",
            self.cg0(),
            self.cg1(),
            self.cg2(),
            self.cg3(),
            self.cg4(),
            self.cg5(),
            self.cg6(),
            self.cg7(),
            self.cg8(),
            self.cg9(),
            self.cg10(),
            self.cg11(),
            self.cg12(),
            self.cg13(),
            self.cg14(),
            self.cg15()
        )
    }
}
#[doc = "CCM Clock Gating Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccgr1(pub u32);
impl Ccgr1 {
    #[doc = "lpspi1 clocks (lpspi1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "lpspi1 clocks (lpspi1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "lpspi2 clocks (lpspi2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "lpspi2 clocks (lpspi2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "lpspi3 clocks (lpspi3_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "lpspi3 clocks (lpspi3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "lpspi4 clocks (lpspi4_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "lpspi4 clocks (lpspi4_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "adc2 clock (adc2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "adc2 clock (adc2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "enet clock (enet_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "enet clock (enet_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "pit clocks (pit_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "pit clocks (pit_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "aoi2 clocks (aoi2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "aoi2 clocks (aoi2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "adc1 clock (adc1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "adc1 clock (adc1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "semc_exsc clock (semc_exsc_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "semc_exsc clock (semc_exsc_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "gpt1 bus clock (gpt_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "gpt1 bus clock (gpt_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "gpt1 serial clock (gpt_serial_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "gpt1 serial clock (gpt_serial_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "lpuart4 clock (lpuart4_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "lpuart4 clock (lpuart4_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "gpio1 clock (gpio1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "gpio1 clock (gpio1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "csu clock (csu_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "csu clock (csu_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "gpio5 clock (gpio5_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "gpio5 clock (gpio5_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ccgr1 {
    #[inline(always)]
    fn default() -> Ccgr1 {
        Ccgr1(0)
    }
}
impl core::fmt::Debug for Ccgr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccgr1")
            .field("cg0", &self.cg0())
            .field("cg1", &self.cg1())
            .field("cg2", &self.cg2())
            .field("cg3", &self.cg3())
            .field("cg4", &self.cg4())
            .field("cg5", &self.cg5())
            .field("cg6", &self.cg6())
            .field("cg7", &self.cg7())
            .field("cg8", &self.cg8())
            .field("cg9", &self.cg9())
            .field("cg10", &self.cg10())
            .field("cg11", &self.cg11())
            .field("cg12", &self.cg12())
            .field("cg13", &self.cg13())
            .field("cg14", &self.cg14())
            .field("cg15", &self.cg15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccgr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccgr1 {{ cg0: {=u8:?}, cg1: {=u8:?}, cg2: {=u8:?}, cg3: {=u8:?}, cg4: {=u8:?}, cg5: {=u8:?}, cg6: {=u8:?}, cg7: {=u8:?}, cg8: {=u8:?}, cg9: {=u8:?}, cg10: {=u8:?}, cg11: {=u8:?}, cg12: {=u8:?}, cg13: {=u8:?}, cg14: {=u8:?}, cg15: {=u8:?} }}",
            self.cg0(),
            self.cg1(),
            self.cg2(),
            self.cg3(),
            self.cg4(),
            self.cg5(),
            self.cg6(),
            self.cg7(),
            self.cg8(),
            self.cg9(),
            self.cg10(),
            self.cg11(),
            self.cg12(),
            self.cg13(),
            self.cg14(),
            self.cg15()
        )
    }
}
#[doc = "CCM Clock Gating Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccgr2(pub u32);
impl Ccgr2 {
    #[doc = "ocram_exsc clock (ocram_exsc_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "ocram_exsc clock (ocram_exsc_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "csi clock (csi_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "csi clock (csi_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "iomuxc_snvs clock (iomuxc_snvs_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "iomuxc_snvs clock (iomuxc_snvs_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "lpi2c1 clock (lpi2c1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "lpi2c1 clock (lpi2c1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "lpi2c2 clock (lpi2c2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "lpi2c2 clock (lpi2c2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "lpi2c3 clock (lpi2c3_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "lpi2c3 clock (lpi2c3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "OCOTP_CTRL clock (ocotp_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "OCOTP_CTRL clock (ocotp_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "xbar3 clock (xbar3_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "xbar3 clock (xbar3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "ipmux1 clock (ipmux1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "ipmux1 clock (ipmux1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "ipmux2 clock (ipmux2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "ipmux2 clock (ipmux2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "ipmux3 clock (ipmux3_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "ipmux3 clock (ipmux3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "xbar1 clock (xbar1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "xbar1 clock (xbar1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "xbar2 clock (xbar2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "xbar2 clock (xbar2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "gpio3 clock (gpio3_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "gpio3 clock (gpio3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "lcd clocks (lcd_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "lcd clocks (lcd_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "pxp clocks (pxp_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "pxp clocks (pxp_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ccgr2 {
    #[inline(always)]
    fn default() -> Ccgr2 {
        Ccgr2(0)
    }
}
impl core::fmt::Debug for Ccgr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccgr2")
            .field("cg0", &self.cg0())
            .field("cg1", &self.cg1())
            .field("cg2", &self.cg2())
            .field("cg3", &self.cg3())
            .field("cg4", &self.cg4())
            .field("cg5", &self.cg5())
            .field("cg6", &self.cg6())
            .field("cg7", &self.cg7())
            .field("cg8", &self.cg8())
            .field("cg9", &self.cg9())
            .field("cg10", &self.cg10())
            .field("cg11", &self.cg11())
            .field("cg12", &self.cg12())
            .field("cg13", &self.cg13())
            .field("cg14", &self.cg14())
            .field("cg15", &self.cg15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccgr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccgr2 {{ cg0: {=u8:?}, cg1: {=u8:?}, cg2: {=u8:?}, cg3: {=u8:?}, cg4: {=u8:?}, cg5: {=u8:?}, cg6: {=u8:?}, cg7: {=u8:?}, cg8: {=u8:?}, cg9: {=u8:?}, cg10: {=u8:?}, cg11: {=u8:?}, cg12: {=u8:?}, cg13: {=u8:?}, cg14: {=u8:?}, cg15: {=u8:?} }}",
            self.cg0(),
            self.cg1(),
            self.cg2(),
            self.cg3(),
            self.cg4(),
            self.cg5(),
            self.cg6(),
            self.cg7(),
            self.cg8(),
            self.cg9(),
            self.cg10(),
            self.cg11(),
            self.cg12(),
            self.cg13(),
            self.cg14(),
            self.cg15()
        )
    }
}
#[doc = "CCM Clock Gating Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccgr3(pub u32);
impl Ccgr3 {
    #[doc = "flexio2 clocks (flexio2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "flexio2 clocks (flexio2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "lpuart5 clock (lpuart5_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "lpuart5 clock (lpuart5_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "semc clocks (semc_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "semc clocks (semc_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "lpuart6 clock (lpuart6_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "lpuart6 clock (lpuart6_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "aoi1 clock (aoi1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "aoi1 clock (aoi1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "lcdif pix clock (lcdif_pix_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "lcdif pix clock (lcdif_pix_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "gpio4 clock (gpio4_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "gpio4 clock (gpio4_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "ewm clocks (ewm_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "ewm clocks (ewm_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "wdog1 clock (wdog1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "wdog1 clock (wdog1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "flexram clock (flexram_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "flexram clock (flexram_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "acmp1 clocks (acmp1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "acmp1 clocks (acmp1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "acmp2 clocks (acmp2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "acmp2 clocks (acmp2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "acmp3 clocks (acmp3_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "acmp3 clocks (acmp3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "acmp4 clocks (acmp4_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "acmp4 clocks (acmp4_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "The OCRAM clock cannot be turned off when the CM cache is running on this device."]
    #[must_use]
    #[inline(always)]
    pub const fn cg14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "The OCRAM clock cannot be turned off when the CM cache is running on this device."]
    #[inline(always)]
    pub const fn set_cg14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "iomuxc_snvs_gpr clock (iomuxc_snvs_gpr_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "iomuxc_snvs_gpr clock (iomuxc_snvs_gpr_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ccgr3 {
    #[inline(always)]
    fn default() -> Ccgr3 {
        Ccgr3(0)
    }
}
impl core::fmt::Debug for Ccgr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccgr3")
            .field("cg0", &self.cg0())
            .field("cg1", &self.cg1())
            .field("cg2", &self.cg2())
            .field("cg3", &self.cg3())
            .field("cg4", &self.cg4())
            .field("cg5", &self.cg5())
            .field("cg6", &self.cg6())
            .field("cg7", &self.cg7())
            .field("cg8", &self.cg8())
            .field("cg9", &self.cg9())
            .field("cg10", &self.cg10())
            .field("cg11", &self.cg11())
            .field("cg12", &self.cg12())
            .field("cg13", &self.cg13())
            .field("cg14", &self.cg14())
            .field("cg15", &self.cg15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccgr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccgr3 {{ cg0: {=u8:?}, cg1: {=u8:?}, cg2: {=u8:?}, cg3: {=u8:?}, cg4: {=u8:?}, cg5: {=u8:?}, cg6: {=u8:?}, cg7: {=u8:?}, cg8: {=u8:?}, cg9: {=u8:?}, cg10: {=u8:?}, cg11: {=u8:?}, cg12: {=u8:?}, cg13: {=u8:?}, cg14: {=u8:?}, cg15: {=u8:?} }}",
            self.cg0(),
            self.cg1(),
            self.cg2(),
            self.cg3(),
            self.cg4(),
            self.cg5(),
            self.cg6(),
            self.cg7(),
            self.cg8(),
            self.cg9(),
            self.cg10(),
            self.cg11(),
            self.cg12(),
            self.cg13(),
            self.cg14(),
            self.cg15()
        )
    }
}
#[doc = "CCM Clock Gating Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccgr4(pub u32);
impl Ccgr4 {
    #[doc = "sim_m7 register access clock (sim_m7_mainclk_r_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "sim_m7 register access clock (sim_m7_mainclk_r_enable)"]
    #[inline(always)]
    pub const fn set_cg0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "iomuxc clock (iomuxc_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "iomuxc clock (iomuxc_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "iomuxc gpr clock (iomuxc_gpr_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "iomuxc gpr clock (iomuxc_gpr_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "bee clock(bee_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "bee clock(bee_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "sim_m7 clock (sim_m7_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "sim_m7 clock (sim_m7_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "tsc_dig clock (tsc_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "tsc_dig clock (tsc_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "sim_m clocks (sim_m_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "sim_m clocks (sim_m_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "sim_ems clocks (sim_ems_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "sim_ems clocks (sim_ems_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "pwm1 clocks (pwm1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "pwm1 clocks (pwm1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "pwm2 clocks (pwm2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "pwm2 clocks (pwm2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "pwm3 clocks (pwm3_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "pwm3 clocks (pwm3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "pwm4 clocks (pwm4_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "pwm4 clocks (pwm4_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "qdc1 clocks (qdc1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "qdc1 clocks (qdc1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "qdc2 clocks (qdc2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "qdc2 clocks (qdc2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "qdc3 clocks (qdc3_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "qdc3 clocks (qdc3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "qdc4 clocks (qdc4_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "qdc4 clocks (qdc4_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ccgr4 {
    #[inline(always)]
    fn default() -> Ccgr4 {
        Ccgr4(0)
    }
}
impl core::fmt::Debug for Ccgr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccgr4")
            .field("cg0", &self.cg0())
            .field("cg1", &self.cg1())
            .field("cg2", &self.cg2())
            .field("cg3", &self.cg3())
            .field("cg4", &self.cg4())
            .field("cg5", &self.cg5())
            .field("cg6", &self.cg6())
            .field("cg7", &self.cg7())
            .field("cg8", &self.cg8())
            .field("cg9", &self.cg9())
            .field("cg10", &self.cg10())
            .field("cg11", &self.cg11())
            .field("cg12", &self.cg12())
            .field("cg13", &self.cg13())
            .field("cg14", &self.cg14())
            .field("cg15", &self.cg15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccgr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccgr4 {{ cg0: {=u8:?}, cg1: {=u8:?}, cg2: {=u8:?}, cg3: {=u8:?}, cg4: {=u8:?}, cg5: {=u8:?}, cg6: {=u8:?}, cg7: {=u8:?}, cg8: {=u8:?}, cg9: {=u8:?}, cg10: {=u8:?}, cg11: {=u8:?}, cg12: {=u8:?}, cg13: {=u8:?}, cg14: {=u8:?}, cg15: {=u8:?} }}",
            self.cg0(),
            self.cg1(),
            self.cg2(),
            self.cg3(),
            self.cg4(),
            self.cg5(),
            self.cg6(),
            self.cg7(),
            self.cg8(),
            self.cg9(),
            self.cg10(),
            self.cg11(),
            self.cg12(),
            self.cg13(),
            self.cg14(),
            self.cg15()
        )
    }
}
#[doc = "CCM Clock Gating Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccgr5(pub u32);
impl Ccgr5 {
    #[doc = "rom clock (rom_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "rom clock (rom_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "flexio1 clock (flexio1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "flexio1 clock (flexio1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "wdog3 clock (wdog3_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "wdog3 clock (wdog3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "dma clock (dma_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "dma clock (dma_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "kpp clock (kpp_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "kpp clock (kpp_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "wdog2 clock (wdog2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "wdog2 clock (wdog2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "aipstz4 clocks (aips_tz4_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "aipstz4 clocks (aips_tz4_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "spdif clock (spdif_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "spdif clock (spdif_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "sim_main clock (sim_main_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "sim_main clock (sim_main_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "sai1 clock (sai1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "sai1 clock (sai1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "sai2 clock (sai2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "sai2 clock (sai2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "sai3 clock (sai3_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "sai3 clock (sai3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "lpuart1 clock (lpuart1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "lpuart1 clock (lpuart1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "lpuart7 clock (lpuart7_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "lpuart7 clock (lpuart7_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "snvs_hp clock (snvs_hp_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "snvs_hp clock (snvs_hp_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "snvs_lp clock (snvs_lp_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "snvs_lp clock (snvs_lp_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ccgr5 {
    #[inline(always)]
    fn default() -> Ccgr5 {
        Ccgr5(0)
    }
}
impl core::fmt::Debug for Ccgr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccgr5")
            .field("cg0", &self.cg0())
            .field("cg1", &self.cg1())
            .field("cg2", &self.cg2())
            .field("cg3", &self.cg3())
            .field("cg4", &self.cg4())
            .field("cg5", &self.cg5())
            .field("cg6", &self.cg6())
            .field("cg7", &self.cg7())
            .field("cg8", &self.cg8())
            .field("cg9", &self.cg9())
            .field("cg10", &self.cg10())
            .field("cg11", &self.cg11())
            .field("cg12", &self.cg12())
            .field("cg13", &self.cg13())
            .field("cg14", &self.cg14())
            .field("cg15", &self.cg15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccgr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccgr5 {{ cg0: {=u8:?}, cg1: {=u8:?}, cg2: {=u8:?}, cg3: {=u8:?}, cg4: {=u8:?}, cg5: {=u8:?}, cg6: {=u8:?}, cg7: {=u8:?}, cg8: {=u8:?}, cg9: {=u8:?}, cg10: {=u8:?}, cg11: {=u8:?}, cg12: {=u8:?}, cg13: {=u8:?}, cg14: {=u8:?}, cg15: {=u8:?} }}",
            self.cg0(),
            self.cg1(),
            self.cg2(),
            self.cg3(),
            self.cg4(),
            self.cg5(),
            self.cg6(),
            self.cg7(),
            self.cg8(),
            self.cg9(),
            self.cg10(),
            self.cg11(),
            self.cg12(),
            self.cg13(),
            self.cg14(),
            self.cg15()
        )
    }
}
#[doc = "CCM Clock Gating Register 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccgr6(pub u32);
impl Ccgr6 {
    #[doc = "usboh3 clock (usboh3_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "usboh3 clock (usboh3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "usdhc1 clocks (usdhc1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "usdhc1 clocks (usdhc1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "usdhc2 clocks (usdhc2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "usdhc2 clocks (usdhc2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "dcdc clocks (dcdc_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "dcdc clocks (dcdc_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "ipmux4 clock (ipmux4_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "ipmux4 clock (ipmux4_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "flexspi clocks (flexspi_clk_enable) sim_ems_clk_enable must also be cleared, when flexspi_clk_enable is cleared"]
    #[must_use]
    #[inline(always)]
    pub const fn cg5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "flexspi clocks (flexspi_clk_enable) sim_ems_clk_enable must also be cleared, when flexspi_clk_enable is cleared"]
    #[inline(always)]
    pub const fn set_cg5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "trng clock (trng_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "trng clock (trng_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "lpuart8 clocks (lpuart8_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "lpuart8 clocks (lpuart8_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "timer4 clocks (timer4_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "timer4 clocks (timer4_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "aips_tz3 clock (aips_tz3_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "aips_tz3 clock (aips_tz3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "sim_axbs_p_clk_enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cg10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "sim_axbs_p_clk_enable"]
    #[inline(always)]
    pub const fn set_cg10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "anadig clocks (anadig_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "anadig clocks (anadig_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "lpi2c4 serial clock (lpi2c4_serial_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "lpi2c4 serial clock (lpi2c4_serial_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "timer1 clocks (timer1_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "timer1 clocks (timer1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "timer2 clocks (timer2_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "timer2 clocks (timer2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "timer3 clocks (timer3_clk_enable)"]
    #[must_use]
    #[inline(always)]
    pub const fn cg15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "timer3 clocks (timer3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ccgr6 {
    #[inline(always)]
    fn default() -> Ccgr6 {
        Ccgr6(0)
    }
}
impl core::fmt::Debug for Ccgr6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccgr6")
            .field("cg0", &self.cg0())
            .field("cg1", &self.cg1())
            .field("cg2", &self.cg2())
            .field("cg3", &self.cg3())
            .field("cg4", &self.cg4())
            .field("cg5", &self.cg5())
            .field("cg6", &self.cg6())
            .field("cg7", &self.cg7())
            .field("cg8", &self.cg8())
            .field("cg9", &self.cg9())
            .field("cg10", &self.cg10())
            .field("cg11", &self.cg11())
            .field("cg12", &self.cg12())
            .field("cg13", &self.cg13())
            .field("cg14", &self.cg14())
            .field("cg15", &self.cg15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccgr6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccgr6 {{ cg0: {=u8:?}, cg1: {=u8:?}, cg2: {=u8:?}, cg3: {=u8:?}, cg4: {=u8:?}, cg5: {=u8:?}, cg6: {=u8:?}, cg7: {=u8:?}, cg8: {=u8:?}, cg9: {=u8:?}, cg10: {=u8:?}, cg11: {=u8:?}, cg12: {=u8:?}, cg13: {=u8:?}, cg14: {=u8:?}, cg15: {=u8:?} }}",
            self.cg0(),
            self.cg1(),
            self.cg2(),
            self.cg3(),
            self.cg4(),
            self.cg5(),
            self.cg6(),
            self.cg7(),
            self.cg8(),
            self.cg9(),
            self.cg10(),
            self.cg11(),
            self.cg12(),
            self.cg13(),
            self.cg14(),
            self.cg15()
        )
    }
}
#[doc = "CCM Clock Gating Register 7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccgr7(pub u32);
impl Ccgr7 {
    #[doc = "enet2_clk_enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cg0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "enet2_clk_enable"]
    #[inline(always)]
    pub const fn set_cg0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "flexspi2_clk_enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cg1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "flexspi2_clk_enable"]
    #[inline(always)]
    pub const fn set_cg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "axbs_l_clk_enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "axbs_l_clk_enable"]
    #[inline(always)]
    pub const fn set_cg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "can3_clk_enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cg3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "can3_clk_enable"]
    #[inline(always)]
    pub const fn set_cg3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "can3_serial_clk_enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cg4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "can3_serial_clk_enable"]
    #[inline(always)]
    pub const fn set_cg4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "aips_lite_clk_enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cg5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "aips_lite_clk_enable"]
    #[inline(always)]
    pub const fn set_cg5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "flexio3_clk_enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cg6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "flexio3_clk_enable"]
    #[inline(always)]
    pub const fn set_cg6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for Ccgr7 {
    #[inline(always)]
    fn default() -> Ccgr7 {
        Ccgr7(0)
    }
}
impl core::fmt::Debug for Ccgr7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccgr7")
            .field("cg0", &self.cg0())
            .field("cg1", &self.cg1())
            .field("cg2", &self.cg2())
            .field("cg3", &self.cg3())
            .field("cg4", &self.cg4())
            .field("cg5", &self.cg5())
            .field("cg6", &self.cg6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccgr7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccgr7 {{ cg0: {=u8:?}, cg1: {=u8:?}, cg2: {=u8:?}, cg3: {=u8:?}, cg4: {=u8:?}, cg5: {=u8:?}, cg6: {=u8:?} }}",
            self.cg0(),
            self.cg1(),
            self.cg2(),
            self.cg3(),
            self.cg4(),
            self.cg5(),
            self.cg6()
        )
    }
}
#[doc = "CCM Clock Output Source Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccosr(pub u32);
impl Ccosr {
    #[doc = "Selection of the clock to be generated on CCM_CLKO1"]
    #[must_use]
    #[inline(always)]
    pub const fn clko1_sel(&self) -> super::vals::Clko1Sel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Clko1Sel::from_bits(val as u8)
    }
    #[doc = "Selection of the clock to be generated on CCM_CLKO1"]
    #[inline(always)]
    pub const fn set_clko1_sel(&mut self, val: super::vals::Clko1Sel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Setting the divider of CCM_CLKO1"]
    #[must_use]
    #[inline(always)]
    pub const fn clko1_div(&self) -> super::vals::Clko1Div {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Clko1Div::from_bits(val as u8)
    }
    #[doc = "Setting the divider of CCM_CLKO1"]
    #[inline(always)]
    pub const fn set_clko1_div(&mut self, val: super::vals::Clko1Div) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Enable of CCM_CLKO1 clock"]
    #[must_use]
    #[inline(always)]
    pub const fn clko1_en(&self) -> super::vals::Clko1En {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Clko1En::from_bits(val as u8)
    }
    #[doc = "Enable of CCM_CLKO1 clock"]
    #[inline(always)]
    pub const fn set_clko1_en(&mut self, val: super::vals::Clko1En) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "CCM_CLKO1 output to reflect CCM_CLKO1 or CCM_CLKO2 clocks"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_out_sel(&self) -> super::vals::ClkOutSel {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::ClkOutSel::from_bits(val as u8)
    }
    #[doc = "CCM_CLKO1 output to reflect CCM_CLKO1 or CCM_CLKO2 clocks"]
    #[inline(always)]
    pub const fn set_clk_out_sel(&mut self, val: super::vals::ClkOutSel) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Selection of the clock to be generated on CCM_CLKO2"]
    #[must_use]
    #[inline(always)]
    pub const fn clko2_sel(&self) -> super::vals::Clko2Sel {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Clko2Sel::from_bits(val as u8)
    }
    #[doc = "Selection of the clock to be generated on CCM_CLKO2"]
    #[inline(always)]
    pub const fn set_clko2_sel(&mut self, val: super::vals::Clko2Sel) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Setting the divider of CCM_CLKO2"]
    #[must_use]
    #[inline(always)]
    pub const fn clko2_div(&self) -> super::vals::Clko2Div {
        let val = (self.0 >> 21usize) & 0x07;
        super::vals::Clko2Div::from_bits(val as u8)
    }
    #[doc = "Setting the divider of CCM_CLKO2"]
    #[inline(always)]
    pub const fn set_clko2_div(&mut self, val: super::vals::Clko2Div) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
    }
    #[doc = "Enable of CCM_CLKO2 clock"]
    #[must_use]
    #[inline(always)]
    pub const fn clko2_en(&self) -> super::vals::Clko2En {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Clko2En::from_bits(val as u8)
    }
    #[doc = "Enable of CCM_CLKO2 clock"]
    #[inline(always)]
    pub const fn set_clko2_en(&mut self, val: super::vals::Clko2En) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Ccosr {
    #[inline(always)]
    fn default() -> Ccosr {
        Ccosr(0)
    }
}
impl core::fmt::Debug for Ccosr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccosr")
            .field("clko1_sel", &self.clko1_sel())
            .field("clko1_div", &self.clko1_div())
            .field("clko1_en", &self.clko1_en())
            .field("clk_out_sel", &self.clk_out_sel())
            .field("clko2_sel", &self.clko2_sel())
            .field("clko2_div", &self.clko2_div())
            .field("clko2_en", &self.clko2_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccosr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccosr {{ clko1_sel: {:?}, clko1_div: {:?}, clko1_en: {:?}, clk_out_sel: {:?}, clko2_sel: {:?}, clko2_div: {:?}, clko2_en: {:?} }}",
            self.clko1_sel(),
            self.clko1_div(),
            self.clko1_en(),
            self.clk_out_sel(),
            self.clko2_sel(),
            self.clko2_div(),
            self.clko2_en()
        )
    }
}
#[doc = "CCM Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "Oscillator ready counter value. These bits define value of 32KHz counter, that serve as counter for oscillator lock time (count to n+1 ckil's). This is used for oscillator lock time. Current estimation is ~5ms. This counter will be used in ignition sequence and in wake from stop sequence if sbyos bit was defined, to notify that on chip oscillator output is ready for the dpll_ip to use and only then the gate in dpll_ip can be opened."]
    #[must_use]
    #[inline(always)]
    pub const fn oscnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Oscillator ready counter value. These bits define value of 32KHz counter, that serve as counter for oscillator lock time (count to n+1 ckil's). This is used for oscillator lock time. Current estimation is ~5ms. This counter will be used in ignition sequence and in wake from stop sequence if sbyos bit was defined, to notify that on chip oscillator output is ready for the dpll_ip to use and only then the gate in dpll_ip can be opened."]
    #[inline(always)]
    pub const fn set_oscnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "On chip oscillator enable bit - this bit value is reflected on the output cosc_en"]
    #[must_use]
    #[inline(always)]
    pub const fn cosc_en(&self) -> super::vals::CoscEn {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::CoscEn::from_bits(val as u8)
    }
    #[doc = "On chip oscillator enable bit - this bit value is reflected on the output cosc_en"]
    #[inline(always)]
    pub const fn set_cosc_en(&mut self, val: super::vals::CoscEn) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter for analog_reg_bypass signal assertion after standby voltage request by PMIC_STBY_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn reg_bypass_count(&self) -> super::vals::RegBypassCount {
        let val = (self.0 >> 21usize) & 0x3f;
        super::vals::RegBypassCount::from_bits(val as u8)
    }
    #[doc = "Counter for analog_reg_bypass signal assertion after standby voltage request by PMIC_STBY_REQ"]
    #[inline(always)]
    pub const fn set_reg_bypass_count(&mut self, val: super::vals::RegBypassCount) {
        self.0 = (self.0 & !(0x3f << 21usize)) | (((val.to_bits() as u32) & 0x3f) << 21usize);
    }
    #[doc = "Enable for REG_BYPASS_COUNTER"]
    #[must_use]
    #[inline(always)]
    pub const fn rbc_en(&self) -> super::vals::RbcEn {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::RbcEn::from_bits(val as u8)
    }
    #[doc = "Enable for REG_BYPASS_COUNTER"]
    #[inline(always)]
    pub const fn set_rbc_en(&mut self, val: super::vals::RbcEn) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Ccr {
    #[inline(always)]
    fn default() -> Ccr {
        Ccr(0)
    }
}
impl core::fmt::Debug for Ccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr")
            .field("oscnt", &self.oscnt())
            .field("cosc_en", &self.cosc_en())
            .field("reg_bypass_count", &self.reg_bypass_count())
            .field("rbc_en", &self.rbc_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr {{ oscnt: {=u8:?}, cosc_en: {:?}, reg_bypass_count: {:?}, rbc_en: {:?} }}",
            self.oscnt(),
            self.cosc_en(),
            self.reg_bypass_count(),
            self.rbc_en()
        )
    }
}
#[doc = "CCM Clock Switcher Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccsr(pub u32);
impl Ccsr {
    #[doc = "Selects source to generate pll3_sw_clk. This bit should only be used for testing purposes."]
    #[must_use]
    #[inline(always)]
    pub const fn pll3_sw_clk_sel(&self) -> super::vals::Pll3SwClkSel {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pll3SwClkSel::from_bits(val as u8)
    }
    #[doc = "Selects source to generate pll3_sw_clk. This bit should only be used for testing purposes."]
    #[inline(always)]
    pub const fn set_pll3_sw_clk_sel(&mut self, val: super::vals::Pll3SwClkSel) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Ccsr {
    #[inline(always)]
    fn default() -> Ccsr {
        Ccsr(0)
    }
}
impl core::fmt::Debug for Ccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccsr")
            .field("pll3_sw_clk_sel", &self.pll3_sw_clk_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccsr {{ pll3_sw_clk_sel: {:?} }}",
            self.pll3_sw_clk_sel()
        )
    }
}
#[doc = "CCM D1 Clock Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdcdr(pub u32);
impl Cdcdr {
    #[doc = "Selector for flexio1 clock multiplexer"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio1_clk_sel(&self) -> super::vals::Flexio1ClkSel {
        let val = (self.0 >> 7usize) & 0x03;
        super::vals::Flexio1ClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for flexio1 clock multiplexer"]
    #[inline(always)]
    pub const fn set_flexio1_clk_sel(&mut self, val: super::vals::Flexio1ClkSel) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u32) & 0x03) << 7usize);
    }
    #[doc = "Divider for flexio1 clock podf. Divider should be updated when output clock is gated."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio1_clk_podf(&self) -> super::vals::Flexio1ClkPodf {
        let val = (self.0 >> 9usize) & 0x07;
        super::vals::Flexio1ClkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for flexio1 clock podf. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn set_flexio1_clk_podf(&mut self, val: super::vals::Flexio1ClkPodf) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
    }
    #[doc = "Divider for flexio1 clock pred. Divider should be updated when output clock is gated."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio1_clk_pred(&self) -> super::vals::Flexio1ClkPred {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Flexio1ClkPred::from_bits(val as u8)
    }
    #[doc = "Divider for flexio1 clock pred. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn set_flexio1_clk_pred(&mut self, val: super::vals::Flexio1ClkPred) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Selector for spdif0 clock multiplexer"]
    #[must_use]
    #[inline(always)]
    pub const fn spdif0_clk_sel(&self) -> super::vals::Spdif0ClkSel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Spdif0ClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for spdif0 clock multiplexer"]
    #[inline(always)]
    pub const fn set_spdif0_clk_sel(&mut self, val: super::vals::Spdif0ClkSel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Divider for spdif0 clock podf. Divider should be updated when output clock is gated."]
    #[must_use]
    #[inline(always)]
    pub const fn spdif0_clk_podf(&self) -> super::vals::Spdif0ClkPodf {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::Spdif0ClkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for spdif0 clock podf. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn set_spdif0_clk_podf(&mut self, val: super::vals::Spdif0ClkPodf) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "Divider for spdif0 clock pred. Divider should be updated when output clock is gated."]
    #[must_use]
    #[inline(always)]
    pub const fn spdif0_clk_pred(&self) -> super::vals::Spdif0ClkPred {
        let val = (self.0 >> 25usize) & 0x07;
        super::vals::Spdif0ClkPred::from_bits(val as u8)
    }
    #[doc = "Divider for spdif0 clock pred. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn set_spdif0_clk_pred(&mut self, val: super::vals::Spdif0ClkPred) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val.to_bits() as u32) & 0x07) << 25usize);
    }
}
impl Default for Cdcdr {
    #[inline(always)]
    fn default() -> Cdcdr {
        Cdcdr(0)
    }
}
impl core::fmt::Debug for Cdcdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cdcdr")
            .field("flexio1_clk_sel", &self.flexio1_clk_sel())
            .field("flexio1_clk_podf", &self.flexio1_clk_podf())
            .field("flexio1_clk_pred", &self.flexio1_clk_pred())
            .field("spdif0_clk_sel", &self.spdif0_clk_sel())
            .field("spdif0_clk_podf", &self.spdif0_clk_podf())
            .field("spdif0_clk_pred", &self.spdif0_clk_pred())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cdcdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cdcdr {{ flexio1_clk_sel: {:?}, flexio1_clk_podf: {:?}, flexio1_clk_pred: {:?}, spdif0_clk_sel: {:?}, spdif0_clk_podf: {:?}, spdif0_clk_pred: {:?} }}",
            self.flexio1_clk_sel(),
            self.flexio1_clk_podf(),
            self.flexio1_clk_pred(),
            self.spdif0_clk_sel(),
            self.spdif0_clk_podf(),
            self.spdif0_clk_pred()
        )
    }
}
#[doc = "CCM Divider Handshake In-Process Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdhipr(pub u32);
impl Cdhipr {
    #[doc = "Busy indicator for semc_podf."]
    #[must_use]
    #[inline(always)]
    pub const fn semc_podf_busy(&self) -> super::vals::SemcPodfBusy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SemcPodfBusy::from_bits(val as u8)
    }
    #[doc = "Busy indicator for semc_podf."]
    #[inline(always)]
    pub const fn set_semc_podf_busy(&mut self, val: super::vals::SemcPodfBusy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Busy indicator for ahb_podf."]
    #[must_use]
    #[inline(always)]
    pub const fn ahb_podf_busy(&self) -> super::vals::AhbPodfBusy {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::AhbPodfBusy::from_bits(val as u8)
    }
    #[doc = "Busy indicator for ahb_podf."]
    #[inline(always)]
    pub const fn set_ahb_podf_busy(&mut self, val: super::vals::AhbPodfBusy) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Busy indicator for periph2_clk_sel mux control."]
    #[must_use]
    #[inline(always)]
    pub const fn periph2_clk_sel_busy(&self) -> super::vals::Periph2ClkSelBusy {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Periph2ClkSelBusy::from_bits(val as u8)
    }
    #[doc = "Busy indicator for periph2_clk_sel mux control."]
    #[inline(always)]
    pub const fn set_periph2_clk_sel_busy(&mut self, val: super::vals::Periph2ClkSelBusy) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Busy indicator for periph_clk_sel mux control."]
    #[must_use]
    #[inline(always)]
    pub const fn periph_clk_sel_busy(&self) -> super::vals::PeriphClkSelBusy {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::PeriphClkSelBusy::from_bits(val as u8)
    }
    #[doc = "Busy indicator for periph_clk_sel mux control."]
    #[inline(always)]
    pub const fn set_periph_clk_sel_busy(&mut self, val: super::vals::PeriphClkSelBusy) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Busy indicator for arm_podf."]
    #[must_use]
    #[inline(always)]
    pub const fn arm_podf_busy(&self) -> super::vals::ArmPodfBusy {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::ArmPodfBusy::from_bits(val as u8)
    }
    #[doc = "Busy indicator for arm_podf."]
    #[inline(always)]
    pub const fn set_arm_podf_busy(&mut self, val: super::vals::ArmPodfBusy) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Cdhipr {
    #[inline(always)]
    fn default() -> Cdhipr {
        Cdhipr(0)
    }
}
impl core::fmt::Debug for Cdhipr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cdhipr")
            .field("semc_podf_busy", &self.semc_podf_busy())
            .field("ahb_podf_busy", &self.ahb_podf_busy())
            .field("periph2_clk_sel_busy", &self.periph2_clk_sel_busy())
            .field("periph_clk_sel_busy", &self.periph_clk_sel_busy())
            .field("arm_podf_busy", &self.arm_podf_busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cdhipr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cdhipr {{ semc_podf_busy: {:?}, ahb_podf_busy: {:?}, periph2_clk_sel_busy: {:?}, periph_clk_sel_busy: {:?}, arm_podf_busy: {:?} }}",
            self.semc_podf_busy(),
            self.ahb_podf_busy(),
            self.periph2_clk_sel_busy(),
            self.periph_clk_sel_busy(),
            self.arm_podf_busy()
        )
    }
}
#[doc = "CCM General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cgpr(pub u32);
impl Cgpr {
    #[doc = "Defines clock dividion of clock for stby_count (pmic delay counter)"]
    #[must_use]
    #[inline(always)]
    pub const fn pmic_delay_scaler(&self) -> super::vals::PmicDelayScaler {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PmicDelayScaler::from_bits(val as u8)
    }
    #[doc = "Defines clock dividion of clock for stby_count (pmic delay counter)"]
    #[inline(always)]
    pub const fn set_pmic_delay_scaler(&mut self, val: super::vals::PmicDelayScaler) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Defines the value of the output signal cgpr_dout\\[4\\]. Gate of program supply for efuse programing"]
    #[must_use]
    #[inline(always)]
    pub const fn efuse_prog_supply_gate(&self) -> super::vals::EfuseProgSupplyGate {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::EfuseProgSupplyGate::from_bits(val as u8)
    }
    #[doc = "Defines the value of the output signal cgpr_dout\\[4\\]. Gate of program supply for efuse programing"]
    #[inline(always)]
    pub const fn set_efuse_prog_supply_gate(&mut self, val: super::vals::EfuseProgSupplyGate) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "System memory DS control"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_mem_ds_ctrl(&self) -> super::vals::SysMemDsCtrl {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::SysMemDsCtrl::from_bits(val as u8)
    }
    #[doc = "System memory DS control"]
    #[inline(always)]
    pub const fn set_sys_mem_ds_ctrl(&mut self, val: super::vals::SysMemDsCtrl) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Fast PLL enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fpl(&self) -> super::vals::Fpl {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Fpl::from_bits(val as u8)
    }
    #[doc = "Fast PLL enable."]
    #[inline(always)]
    pub const fn set_fpl(&mut self, val: super::vals::Fpl) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Control for the Deep Sleep signal to the Arm Platform memories with additional control logic based on the Arm WFI signal"]
    #[must_use]
    #[inline(always)]
    pub const fn int_mem_clk_lpm(&self) -> super::vals::IntMemClkLpm {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::IntMemClkLpm::from_bits(val as u8)
    }
    #[doc = "Control for the Deep Sleep signal to the Arm Platform memories with additional control logic based on the Arm WFI signal"]
    #[inline(always)]
    pub const fn set_int_mem_clk_lpm(&mut self, val: super::vals::IntMemClkLpm) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
}
impl Default for Cgpr {
    #[inline(always)]
    fn default() -> Cgpr {
        Cgpr(0)
    }
}
impl core::fmt::Debug for Cgpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cgpr")
            .field("pmic_delay_scaler", &self.pmic_delay_scaler())
            .field("efuse_prog_supply_gate", &self.efuse_prog_supply_gate())
            .field("sys_mem_ds_ctrl", &self.sys_mem_ds_ctrl())
            .field("fpl", &self.fpl())
            .field("int_mem_clk_lpm", &self.int_mem_clk_lpm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cgpr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cgpr {{ pmic_delay_scaler: {:?}, efuse_prog_supply_gate: {:?}, sys_mem_ds_ctrl: {:?}, fpl: {:?}, int_mem_clk_lpm: {:?} }}",
            self.pmic_delay_scaler(),
            self.efuse_prog_supply_gate(),
            self.sys_mem_ds_ctrl(),
            self.fpl(),
            self.int_mem_clk_lpm()
        )
    }
}
#[doc = "CCM Interrupt Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cimr(pub u32);
impl Cimr {
    #[doc = "mask interrupt generation due to lrf of PLLs"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_lrf_pll(&self) -> super::vals::MaskLrfPll {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MaskLrfPll::from_bits(val as u8)
    }
    #[doc = "mask interrupt generation due to lrf of PLLs"]
    #[inline(always)]
    pub const fn set_mask_lrf_pll(&mut self, val: super::vals::MaskLrfPll) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "mask interrupt generation due to on board oscillator ready"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cosc_ready(&self) -> super::vals::MaskCoscReady {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::MaskCoscReady::from_bits(val as u8)
    }
    #[doc = "mask interrupt generation due to on board oscillator ready"]
    #[inline(always)]
    pub const fn set_mask_cosc_ready(&mut self, val: super::vals::MaskCoscReady) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "mask interrupt generation due to frequency change of semc_podf"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_semc_podf_loaded(&self) -> super::vals::MaskSemcPodfLoaded {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::MaskSemcPodfLoaded::from_bits(val as u8)
    }
    #[doc = "mask interrupt generation due to frequency change of semc_podf"]
    #[inline(always)]
    pub const fn set_mask_semc_podf_loaded(&mut self, val: super::vals::MaskSemcPodfLoaded) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "mask interrupt generation due to update of periph2_clk_sel."]
    #[must_use]
    #[inline(always)]
    pub const fn mask_periph2_clk_sel_loaded(&self) -> super::vals::MaskPeriph2ClkSelLoaded {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::MaskPeriph2ClkSelLoaded::from_bits(val as u8)
    }
    #[doc = "mask interrupt generation due to update of periph2_clk_sel."]
    #[inline(always)]
    pub const fn set_mask_periph2_clk_sel_loaded(
        &mut self,
        val: super::vals::MaskPeriph2ClkSelLoaded,
    ) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "mask interrupt generation due to frequency change of ahb_podf"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_ahb_podf_loaded(&self) -> super::vals::MaskAhbPodfLoaded {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::MaskAhbPodfLoaded::from_bits(val as u8)
    }
    #[doc = "mask interrupt generation due to frequency change of ahb_podf"]
    #[inline(always)]
    pub const fn set_mask_ahb_podf_loaded(&mut self, val: super::vals::MaskAhbPodfLoaded) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "mask interrupt generation due to update of periph_clk_sel."]
    #[must_use]
    #[inline(always)]
    pub const fn mask_periph_clk_sel_loaded(&self) -> super::vals::MaskPeriphClkSelLoaded {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::MaskPeriphClkSelLoaded::from_bits(val as u8)
    }
    #[doc = "mask interrupt generation due to update of periph_clk_sel."]
    #[inline(always)]
    pub const fn set_mask_periph_clk_sel_loaded(
        &mut self,
        val: super::vals::MaskPeriphClkSelLoaded,
    ) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "mask interrupt generation due to frequency change of arm_podf"]
    #[must_use]
    #[inline(always)]
    pub const fn arm_podf_loaded(&self) -> super::vals::CimrArmPodfLoaded {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::CimrArmPodfLoaded::from_bits(val as u8)
    }
    #[doc = "mask interrupt generation due to frequency change of arm_podf"]
    #[inline(always)]
    pub const fn set_arm_podf_loaded(&mut self, val: super::vals::CimrArmPodfLoaded) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for Cimr {
    #[inline(always)]
    fn default() -> Cimr {
        Cimr(0)
    }
}
impl core::fmt::Debug for Cimr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cimr")
            .field("mask_lrf_pll", &self.mask_lrf_pll())
            .field("mask_cosc_ready", &self.mask_cosc_ready())
            .field("mask_semc_podf_loaded", &self.mask_semc_podf_loaded())
            .field(
                "mask_periph2_clk_sel_loaded",
                &self.mask_periph2_clk_sel_loaded(),
            )
            .field("mask_ahb_podf_loaded", &self.mask_ahb_podf_loaded())
            .field(
                "mask_periph_clk_sel_loaded",
                &self.mask_periph_clk_sel_loaded(),
            )
            .field("arm_podf_loaded", &self.arm_podf_loaded())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cimr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cimr {{ mask_lrf_pll: {:?}, mask_cosc_ready: {:?}, mask_semc_podf_loaded: {:?}, mask_periph2_clk_sel_loaded: {:?}, mask_ahb_podf_loaded: {:?}, mask_periph_clk_sel_loaded: {:?}, arm_podf_loaded: {:?} }}",
            self.mask_lrf_pll(),
            self.mask_cosc_ready(),
            self.mask_semc_podf_loaded(),
            self.mask_periph2_clk_sel_loaded(),
            self.mask_ahb_podf_loaded(),
            self.mask_periph_clk_sel_loaded(),
            self.arm_podf_loaded()
        )
    }
}
#[doc = "CCM Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cisr(pub u32);
impl Cisr {
    #[doc = "CCM interrupt request 2 generated due to lock of all enabled and not bypaseed PLLs"]
    #[must_use]
    #[inline(always)]
    pub const fn lrf_pll(&self) -> super::vals::LrfPll {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LrfPll::from_bits(val as u8)
    }
    #[doc = "CCM interrupt request 2 generated due to lock of all enabled and not bypaseed PLLs"]
    #[inline(always)]
    pub const fn set_lrf_pll(&mut self, val: super::vals::LrfPll) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CCM interrupt request 2 generated due to on board oscillator ready, i"]
    #[must_use]
    #[inline(always)]
    pub const fn cosc_ready(&self) -> super::vals::CisrCoscReady {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::CisrCoscReady::from_bits(val as u8)
    }
    #[doc = "CCM interrupt request 2 generated due to on board oscillator ready, i"]
    #[inline(always)]
    pub const fn set_cosc_ready(&mut self, val: super::vals::CisrCoscReady) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of semc_podf"]
    #[must_use]
    #[inline(always)]
    pub const fn semc_podf_loaded(&self) -> super::vals::SemcPodfLoaded {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::SemcPodfLoaded::from_bits(val as u8)
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of semc_podf"]
    #[inline(always)]
    pub const fn set_semc_podf_loaded(&mut self, val: super::vals::SemcPodfLoaded) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of periph2_clk_sel"]
    #[must_use]
    #[inline(always)]
    pub const fn periph2_clk_sel_loaded(&self) -> super::vals::Periph2ClkSelLoaded {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Periph2ClkSelLoaded::from_bits(val as u8)
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of periph2_clk_sel"]
    #[inline(always)]
    pub const fn set_periph2_clk_sel_loaded(&mut self, val: super::vals::Periph2ClkSelLoaded) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of ahb_podf"]
    #[must_use]
    #[inline(always)]
    pub const fn ahb_podf_loaded(&self) -> super::vals::AhbPodfLoaded {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::AhbPodfLoaded::from_bits(val as u8)
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of ahb_podf"]
    #[inline(always)]
    pub const fn set_ahb_podf_loaded(&mut self, val: super::vals::AhbPodfLoaded) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "CCM interrupt request 1 generated due to update of periph_clk_sel."]
    #[must_use]
    #[inline(always)]
    pub const fn periph_clk_sel_loaded(&self) -> super::vals::PeriphClkSelLoaded {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::PeriphClkSelLoaded::from_bits(val as u8)
    }
    #[doc = "CCM interrupt request 1 generated due to update of periph_clk_sel."]
    #[inline(always)]
    pub const fn set_periph_clk_sel_loaded(&mut self, val: super::vals::PeriphClkSelLoaded) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of arm_podf"]
    #[must_use]
    #[inline(always)]
    pub const fn arm_podf_loaded(&self) -> super::vals::CisrArmPodfLoaded {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::CisrArmPodfLoaded::from_bits(val as u8)
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of arm_podf"]
    #[inline(always)]
    pub const fn set_arm_podf_loaded(&mut self, val: super::vals::CisrArmPodfLoaded) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for Cisr {
    #[inline(always)]
    fn default() -> Cisr {
        Cisr(0)
    }
}
impl core::fmt::Debug for Cisr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cisr")
            .field("lrf_pll", &self.lrf_pll())
            .field("cosc_ready", &self.cosc_ready())
            .field("semc_podf_loaded", &self.semc_podf_loaded())
            .field("periph2_clk_sel_loaded", &self.periph2_clk_sel_loaded())
            .field("ahb_podf_loaded", &self.ahb_podf_loaded())
            .field("periph_clk_sel_loaded", &self.periph_clk_sel_loaded())
            .field("arm_podf_loaded", &self.arm_podf_loaded())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cisr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cisr {{ lrf_pll: {:?}, cosc_ready: {:?}, semc_podf_loaded: {:?}, periph2_clk_sel_loaded: {:?}, ahb_podf_loaded: {:?}, periph_clk_sel_loaded: {:?}, arm_podf_loaded: {:?} }}",
            self.lrf_pll(),
            self.cosc_ready(),
            self.semc_podf_loaded(),
            self.periph2_clk_sel_loaded(),
            self.ahb_podf_loaded(),
            self.periph_clk_sel_loaded(),
            self.arm_podf_loaded()
        )
    }
}
#[doc = "CCM Low Power Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clpcr(pub u32);
impl Clpcr {
    #[doc = "Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[must_use]
    #[inline(always)]
    pub const fn lpm(&self) -> super::vals::Lpm {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpm::from_bits(val as u8)
    }
    #[doc = "Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline(always)]
    pub const fn set_lpm(&mut self, val: super::vals::Lpm) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Define if Arm clocks (arm_clk, soc_mxclk, soc_pclk, soc_dbg_pclk, vl_wrck) will be disabled on wait mode"]
    #[must_use]
    #[inline(always)]
    pub const fn arm_clk_dis_on_lpm(&self) -> super::vals::ArmClkDisOnLpm {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::ArmClkDisOnLpm::from_bits(val as u8)
    }
    #[doc = "Define if Arm clocks (arm_clk, soc_mxclk, soc_pclk, soc_dbg_pclk, vl_wrck) will be disabled on wait mode"]
    #[inline(always)]
    pub const fn set_arm_clk_dis_on_lpm(&mut self, val: super::vals::ArmClkDisOnLpm) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Standby clock oscillator bit"]
    #[must_use]
    #[inline(always)]
    pub const fn sbyos(&self) -> super::vals::Sbyos {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sbyos::from_bits(val as u8)
    }
    #[doc = "Standby clock oscillator bit"]
    #[inline(always)]
    pub const fn set_sbyos(&mut self, val: super::vals::Sbyos) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "dis_ref_osc - in run mode, software can manually control closing of external reference oscillator clock, i"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_ref_osc(&self) -> super::vals::DisRefOsc {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::DisRefOsc::from_bits(val as u8)
    }
    #[doc = "dis_ref_osc - in run mode, software can manually control closing of external reference oscillator clock, i"]
    #[inline(always)]
    pub const fn set_dis_ref_osc(&mut self, val: super::vals::DisRefOsc) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Voltage standby request bit"]
    #[must_use]
    #[inline(always)]
    pub const fn vstby(&self) -> super::vals::Vstby {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Vstby::from_bits(val as u8)
    }
    #[doc = "Voltage standby request bit"]
    #[inline(always)]
    pub const fn set_vstby(&mut self, val: super::vals::Vstby) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Standby counter definition"]
    #[must_use]
    #[inline(always)]
    pub const fn stby_count(&self) -> super::vals::StbyCount {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::StbyCount::from_bits(val as u8)
    }
    #[doc = "Standby counter definition"]
    #[inline(always)]
    pub const fn set_stby_count(&mut self, val: super::vals::StbyCount) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "In run mode, software can manually control powering down of on chip oscillator, i"]
    #[must_use]
    #[inline(always)]
    pub const fn cosc_pwrdown(&self) -> super::vals::CoscPwrdown {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::CoscPwrdown::from_bits(val as u8)
    }
    #[doc = "In run mode, software can manually control powering down of on chip oscillator, i"]
    #[inline(always)]
    pub const fn set_cosc_pwrdown(&mut self, val: super::vals::CoscPwrdown) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_lpm_hs1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    #[inline(always)]
    pub const fn set_bypass_lpm_hs1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_lpm_hs0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    #[inline(always)]
    pub const fn set_bypass_lpm_hs0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Mask WFI of core0 for entering low power mode Assertion of all bits\\[27:22\\] will generate low power mode request"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_core0_wfi(&self) -> super::vals::MaskCore0Wfi {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::MaskCore0Wfi::from_bits(val as u8)
    }
    #[doc = "Mask WFI of core0 for entering low power mode Assertion of all bits\\[27:22\\] will generate low power mode request"]
    #[inline(always)]
    pub const fn set_mask_core0_wfi(&mut self, val: super::vals::MaskCore0Wfi) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Mask SCU IDLE for entering low power mode Assertion of all bits\\[27:22\\] will generate low power mode request"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_scu_idle(&self) -> super::vals::MaskScuIdle {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::MaskScuIdle::from_bits(val as u8)
    }
    #[doc = "Mask SCU IDLE for entering low power mode Assertion of all bits\\[27:22\\] will generate low power mode request"]
    #[inline(always)]
    pub const fn set_mask_scu_idle(&mut self, val: super::vals::MaskScuIdle) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Mask L2CC IDLE for entering low power mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_l2cc_idle(&self) -> super::vals::MaskL2ccIdle {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::MaskL2ccIdle::from_bits(val as u8)
    }
    #[doc = "Mask L2CC IDLE for entering low power mode"]
    #[inline(always)]
    pub const fn set_mask_l2cc_idle(&mut self, val: super::vals::MaskL2ccIdle) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Clpcr {
    #[inline(always)]
    fn default() -> Clpcr {
        Clpcr(0)
    }
}
impl core::fmt::Debug for Clpcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clpcr")
            .field("lpm", &self.lpm())
            .field("arm_clk_dis_on_lpm", &self.arm_clk_dis_on_lpm())
            .field("sbyos", &self.sbyos())
            .field("dis_ref_osc", &self.dis_ref_osc())
            .field("vstby", &self.vstby())
            .field("stby_count", &self.stby_count())
            .field("cosc_pwrdown", &self.cosc_pwrdown())
            .field("bypass_lpm_hs1", &self.bypass_lpm_hs1())
            .field("bypass_lpm_hs0", &self.bypass_lpm_hs0())
            .field("mask_core0_wfi", &self.mask_core0_wfi())
            .field("mask_scu_idle", &self.mask_scu_idle())
            .field("mask_l2cc_idle", &self.mask_l2cc_idle())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clpcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clpcr {{ lpm: {:?}, arm_clk_dis_on_lpm: {:?}, sbyos: {:?}, dis_ref_osc: {:?}, vstby: {:?}, stby_count: {:?}, cosc_pwrdown: {:?}, bypass_lpm_hs1: {=bool:?}, bypass_lpm_hs0: {=bool:?}, mask_core0_wfi: {:?}, mask_scu_idle: {:?}, mask_l2cc_idle: {:?} }}",
            self.lpm(),
            self.arm_clk_dis_on_lpm(),
            self.sbyos(),
            self.dis_ref_osc(),
            self.vstby(),
            self.stby_count(),
            self.cosc_pwrdown(),
            self.bypass_lpm_hs1(),
            self.bypass_lpm_hs0(),
            self.mask_core0_wfi(),
            self.mask_scu_idle(),
            self.mask_l2cc_idle()
        )
    }
}
#[doc = "CCM Module Enable Overide Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmeor(pub u32);
impl Cmeor {
    #[doc = "Overide clock enable signal from GPT - clock will not be gated based on GPT's signal 'ipg_enable_clk'"]
    #[must_use]
    #[inline(always)]
    pub const fn mod_en_ov_gpt(&self) -> super::vals::ModEnOvGpt {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::ModEnOvGpt::from_bits(val as u8)
    }
    #[doc = "Overide clock enable signal from GPT - clock will not be gated based on GPT's signal 'ipg_enable_clk'"]
    #[inline(always)]
    pub const fn set_mod_en_ov_gpt(&mut self, val: super::vals::ModEnOvGpt) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Overide clock enable signal from PIT - clock will not be gated based on PIT's signal 'ipg_enable_clk'"]
    #[must_use]
    #[inline(always)]
    pub const fn mod_en_ov_pit(&self) -> super::vals::ModEnOvPit {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::ModEnOvPit::from_bits(val as u8)
    }
    #[doc = "Overide clock enable signal from PIT - clock will not be gated based on PIT's signal 'ipg_enable_clk'"]
    #[inline(always)]
    pub const fn set_mod_en_ov_pit(&mut self, val: super::vals::ModEnOvPit) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "overide clock enable signal from USDHC."]
    #[must_use]
    #[inline(always)]
    pub const fn mod_en_usdhc(&self) -> super::vals::ModEnUsdhc {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::ModEnUsdhc::from_bits(val as u8)
    }
    #[doc = "overide clock enable signal from USDHC."]
    #[inline(always)]
    pub const fn set_mod_en_usdhc(&mut self, val: super::vals::ModEnUsdhc) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Overide clock enable signal from TRNG"]
    #[must_use]
    #[inline(always)]
    pub const fn mod_en_ov_trng(&self) -> super::vals::ModEnOvTrng {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::ModEnOvTrng::from_bits(val as u8)
    }
    #[doc = "Overide clock enable signal from TRNG"]
    #[inline(always)]
    pub const fn set_mod_en_ov_trng(&mut self, val: super::vals::ModEnOvTrng) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Overide clock enable signal from FlexCAN3(CANFD) - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[must_use]
    #[inline(always)]
    pub const fn mod_en_ov_canfd_cpi(&self) -> super::vals::ModEnOvCanfdCpi {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::ModEnOvCanfdCpi::from_bits(val as u8)
    }
    #[doc = "Overide clock enable signal from FlexCAN3(CANFD) - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[inline(always)]
    pub const fn set_mod_en_ov_canfd_cpi(&mut self, val: super::vals::ModEnOvCanfdCpi) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Overide clock enable signal from CAN2 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[must_use]
    #[inline(always)]
    pub const fn mod_en_ov_can2_cpi(&self) -> super::vals::ModEnOvCan2Cpi {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::ModEnOvCan2Cpi::from_bits(val as u8)
    }
    #[doc = "Overide clock enable signal from CAN2 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[inline(always)]
    pub const fn set_mod_en_ov_can2_cpi(&mut self, val: super::vals::ModEnOvCan2Cpi) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Overide clock enable signal from CAN1 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[must_use]
    #[inline(always)]
    pub const fn mod_en_ov_can1_cpi(&self) -> super::vals::ModEnOvCan1Cpi {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ModEnOvCan1Cpi::from_bits(val as u8)
    }
    #[doc = "Overide clock enable signal from CAN1 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[inline(always)]
    pub const fn set_mod_en_ov_can1_cpi(&mut self, val: super::vals::ModEnOvCan1Cpi) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Cmeor {
    #[inline(always)]
    fn default() -> Cmeor {
        Cmeor(0)
    }
}
impl core::fmt::Debug for Cmeor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmeor")
            .field("mod_en_ov_gpt", &self.mod_en_ov_gpt())
            .field("mod_en_ov_pit", &self.mod_en_ov_pit())
            .field("mod_en_usdhc", &self.mod_en_usdhc())
            .field("mod_en_ov_trng", &self.mod_en_ov_trng())
            .field("mod_en_ov_canfd_cpi", &self.mod_en_ov_canfd_cpi())
            .field("mod_en_ov_can2_cpi", &self.mod_en_ov_can2_cpi())
            .field("mod_en_ov_can1_cpi", &self.mod_en_ov_can1_cpi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmeor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmeor {{ mod_en_ov_gpt: {:?}, mod_en_ov_pit: {:?}, mod_en_usdhc: {:?}, mod_en_ov_trng: {:?}, mod_en_ov_canfd_cpi: {:?}, mod_en_ov_can2_cpi: {:?}, mod_en_ov_can1_cpi: {:?} }}",
            self.mod_en_ov_gpt(),
            self.mod_en_ov_pit(),
            self.mod_en_usdhc(),
            self.mod_en_ov_trng(),
            self.mod_en_ov_canfd_cpi(),
            self.mod_en_ov_can2_cpi(),
            self.mod_en_ov_can1_cpi()
        )
    }
}
#[doc = "CCM Clock Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs1cdr(pub u32);
impl Cs1cdr {
    #[doc = "Divider for sai1 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    #[must_use]
    #[inline(always)]
    pub const fn sai1_clk_podf(&self) -> super::vals::Sai1ClkPodf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sai1ClkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for sai1 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    #[inline(always)]
    pub const fn set_sai1_clk_podf(&mut self, val: super::vals::Sai1ClkPodf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Divider for sai1 clock pred."]
    #[must_use]
    #[inline(always)]
    pub const fn sai1_clk_pred(&self) -> super::vals::Sai1ClkPred {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Sai1ClkPred::from_bits(val as u8)
    }
    #[doc = "Divider for sai1 clock pred."]
    #[inline(always)]
    pub const fn set_sai1_clk_pred(&mut self, val: super::vals::Sai1ClkPred) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Divider for flexio2/flexio3 clock."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio2_clk_pred(&self) -> super::vals::Flexio2ClkPred {
        let val = (self.0 >> 9usize) & 0x07;
        super::vals::Flexio2ClkPred::from_bits(val as u8)
    }
    #[doc = "Divider for flexio2/flexio3 clock."]
    #[inline(always)]
    pub const fn set_flexio2_clk_pred(&mut self, val: super::vals::Flexio2ClkPred) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
    }
    #[doc = "Divider for sai3 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    #[must_use]
    #[inline(always)]
    pub const fn sai3_clk_podf(&self) -> super::vals::Sai3ClkPodf {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Sai3ClkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for sai3 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    #[inline(always)]
    pub const fn set_sai3_clk_podf(&mut self, val: super::vals::Sai3ClkPodf) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "Divider for sai3/adc1/adc2 clock pred."]
    #[must_use]
    #[inline(always)]
    pub const fn sai3_clk_pred(&self) -> super::vals::Sai3ClkPred {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::Sai3ClkPred::from_bits(val as u8)
    }
    #[doc = "Divider for sai3/adc1/adc2 clock pred."]
    #[inline(always)]
    pub const fn set_sai3_clk_pred(&mut self, val: super::vals::Sai3ClkPred) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "Divider for flexio2/flexio3 clock. Divider should be updated when output clock is gated."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio2_clk_podf(&self) -> super::vals::Flexio2ClkPodf {
        let val = (self.0 >> 25usize) & 0x07;
        super::vals::Flexio2ClkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for flexio2/flexio3 clock. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn set_flexio2_clk_podf(&mut self, val: super::vals::Flexio2ClkPodf) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val.to_bits() as u32) & 0x07) << 25usize);
    }
}
impl Default for Cs1cdr {
    #[inline(always)]
    fn default() -> Cs1cdr {
        Cs1cdr(0)
    }
}
impl core::fmt::Debug for Cs1cdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cs1cdr")
            .field("sai1_clk_podf", &self.sai1_clk_podf())
            .field("sai1_clk_pred", &self.sai1_clk_pred())
            .field("flexio2_clk_pred", &self.flexio2_clk_pred())
            .field("sai3_clk_podf", &self.sai3_clk_podf())
            .field("sai3_clk_pred", &self.sai3_clk_pred())
            .field("flexio2_clk_podf", &self.flexio2_clk_podf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cs1cdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cs1cdr {{ sai1_clk_podf: {:?}, sai1_clk_pred: {:?}, flexio2_clk_pred: {:?}, sai3_clk_podf: {:?}, sai3_clk_pred: {:?}, flexio2_clk_podf: {:?} }}",
            self.sai1_clk_podf(),
            self.sai1_clk_pred(),
            self.flexio2_clk_pred(),
            self.sai3_clk_podf(),
            self.sai3_clk_pred(),
            self.flexio2_clk_podf()
        )
    }
}
#[doc = "CCM Clock Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs2cdr(pub u32);
impl Cs2cdr {
    #[doc = "Divider for sai2 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    #[must_use]
    #[inline(always)]
    pub const fn sai2_clk_podf(&self) -> super::vals::Sai2ClkPodf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sai2ClkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for sai2 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    #[inline(always)]
    pub const fn set_sai2_clk_podf(&mut self, val: super::vals::Sai2ClkPodf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Divider for sai2 clock pred.Divider should be updated when output clock is gated."]
    #[must_use]
    #[inline(always)]
    pub const fn sai2_clk_pred(&self) -> super::vals::Sai2ClkPred {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Sai2ClkPred::from_bits(val as u8)
    }
    #[doc = "Divider for sai2 clock pred.Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn set_sai2_clk_pred(&mut self, val: super::vals::Sai2ClkPred) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
}
impl Default for Cs2cdr {
    #[inline(always)]
    fn default() -> Cs2cdr {
        Cs2cdr(0)
    }
}
impl core::fmt::Debug for Cs2cdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cs2cdr")
            .field("sai2_clk_podf", &self.sai2_clk_podf())
            .field("sai2_clk_pred", &self.sai2_clk_pred())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cs2cdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cs2cdr {{ sai2_clk_podf: {:?}, sai2_clk_pred: {:?} }}",
            self.sai2_clk_podf(),
            self.sai2_clk_pred()
        )
    }
}
#[doc = "CCM Serial Clock Divider Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cscdr1(pub u32);
impl Cscdr1 {
    #[doc = "Divider for uart clock podf."]
    #[must_use]
    #[inline(always)]
    pub const fn uart_clk_podf(&self) -> super::vals::UartClkPodf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::UartClkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for uart clock podf."]
    #[inline(always)]
    pub const fn set_uart_clk_podf(&mut self, val: super::vals::UartClkPodf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Selector for the UART clock multiplexor"]
    #[must_use]
    #[inline(always)]
    pub const fn uart_clk_sel(&self) -> super::vals::UartClkSel {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::UartClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for the UART clock multiplexor"]
    #[inline(always)]
    pub const fn set_uart_clk_sel(&mut self, val: super::vals::UartClkSel) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Divider for usdhc1 clock podf. Divider should be updated when output clock is gated."]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc1_podf(&self) -> super::vals::Usdhc1Podf {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Usdhc1Podf::from_bits(val as u8)
    }
    #[doc = "Divider for usdhc1 clock podf. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn set_usdhc1_podf(&mut self, val: super::vals::Usdhc1Podf) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Divider for usdhc2 clock. Divider should be updated when output clock is gated."]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc2_podf(&self) -> super::vals::Usdhc2Podf {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Usdhc2Podf::from_bits(val as u8)
    }
    #[doc = "Divider for usdhc2 clock. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn set_usdhc2_podf(&mut self, val: super::vals::Usdhc2Podf) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Divider for trace clock. Divider should be updated when output clock is gated."]
    #[must_use]
    #[inline(always)]
    pub const fn trace_podf(&self) -> super::vals::TracePodf {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::TracePodf::from_bits(val as u8)
    }
    #[doc = "Divider for trace clock. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn set_trace_podf(&mut self, val: super::vals::TracePodf) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
}
impl Default for Cscdr1 {
    #[inline(always)]
    fn default() -> Cscdr1 {
        Cscdr1(0)
    }
}
impl core::fmt::Debug for Cscdr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cscdr1")
            .field("uart_clk_podf", &self.uart_clk_podf())
            .field("uart_clk_sel", &self.uart_clk_sel())
            .field("usdhc1_podf", &self.usdhc1_podf())
            .field("usdhc2_podf", &self.usdhc2_podf())
            .field("trace_podf", &self.trace_podf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cscdr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cscdr1 {{ uart_clk_podf: {:?}, uart_clk_sel: {:?}, usdhc1_podf: {:?}, usdhc2_podf: {:?}, trace_podf: {:?} }}",
            self.uart_clk_podf(),
            self.uart_clk_sel(),
            self.usdhc1_podf(),
            self.usdhc2_podf(),
            self.trace_podf()
        )
    }
}
#[doc = "CCM Serial Clock Divider Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cscdr2(pub u32);
impl Cscdr2 {
    #[doc = "Pre-divider for lcdif clock. Divider should be updated when output clock is gated."]
    #[must_use]
    #[inline(always)]
    pub const fn lcdif_pred(&self) -> super::vals::LcdifPred {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::LcdifPred::from_bits(val as u8)
    }
    #[doc = "Pre-divider for lcdif clock. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn set_lcdif_pred(&mut self, val: super::vals::LcdifPred) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Selector for lcdif root clock pre-multiplexer"]
    #[must_use]
    #[inline(always)]
    pub const fn lcdif_pre_clk_sel(&self) -> super::vals::LcdifPreClkSel {
        let val = (self.0 >> 15usize) & 0x07;
        super::vals::LcdifPreClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for lcdif root clock pre-multiplexer"]
    #[inline(always)]
    pub const fn set_lcdif_pre_clk_sel(&mut self, val: super::vals::LcdifPreClkSel) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val.to_bits() as u32) & 0x07) << 15usize);
    }
    #[doc = "Selector for the LPI2C clock multiplexor"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c_clk_sel(&self) -> super::vals::Lpi2cClkSel {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Lpi2cClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for the LPI2C clock multiplexor"]
    #[inline(always)]
    pub const fn set_lpi2c_clk_sel(&mut self, val: super::vals::Lpi2cClkSel) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Divider for lpi2c clock podf. Divider should be updated when output clock is gated. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c_clk_podf(&self) -> super::vals::Lpi2cClkPodf {
        let val = (self.0 >> 19usize) & 0x3f;
        super::vals::Lpi2cClkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for lpi2c clock podf. Divider should be updated when output clock is gated. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    #[inline(always)]
    pub const fn set_lpi2c_clk_podf(&mut self, val: super::vals::Lpi2cClkPodf) {
        self.0 = (self.0 & !(0x3f << 19usize)) | (((val.to_bits() as u32) & 0x3f) << 19usize);
    }
}
impl Default for Cscdr2 {
    #[inline(always)]
    fn default() -> Cscdr2 {
        Cscdr2(0)
    }
}
impl core::fmt::Debug for Cscdr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cscdr2")
            .field("lcdif_pred", &self.lcdif_pred())
            .field("lcdif_pre_clk_sel", &self.lcdif_pre_clk_sel())
            .field("lpi2c_clk_sel", &self.lpi2c_clk_sel())
            .field("lpi2c_clk_podf", &self.lpi2c_clk_podf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cscdr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cscdr2 {{ lcdif_pred: {:?}, lcdif_pre_clk_sel: {:?}, lpi2c_clk_sel: {:?}, lpi2c_clk_podf: {:?} }}",
            self.lcdif_pred(),
            self.lcdif_pre_clk_sel(),
            self.lpi2c_clk_sel(),
            self.lpi2c_clk_podf()
        )
    }
}
#[doc = "CCM Serial Clock Divider Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cscdr3(pub u32);
impl Cscdr3 {
    #[doc = "Selector for csi_mclk multiplexer"]
    #[must_use]
    #[inline(always)]
    pub const fn csi_clk_sel(&self) -> super::vals::CsiClkSel {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::CsiClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for csi_mclk multiplexer"]
    #[inline(always)]
    pub const fn set_csi_clk_sel(&mut self, val: super::vals::CsiClkSel) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Post divider for csi_mclk. Divider should be updated when output clock is gated."]
    #[must_use]
    #[inline(always)]
    pub const fn csi_podf(&self) -> super::vals::CsiPodf {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::CsiPodf::from_bits(val as u8)
    }
    #[doc = "Post divider for csi_mclk. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn set_csi_podf(&mut self, val: super::vals::CsiPodf) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
}
impl Default for Cscdr3 {
    #[inline(always)]
    fn default() -> Cscdr3 {
        Cscdr3(0)
    }
}
impl core::fmt::Debug for Cscdr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cscdr3")
            .field("csi_clk_sel", &self.csi_clk_sel())
            .field("csi_podf", &self.csi_podf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cscdr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cscdr3 {{ csi_clk_sel: {:?}, csi_podf: {:?} }}",
            self.csi_clk_sel(),
            self.csi_podf()
        )
    }
}
#[doc = "CCM Serial Clock Multiplexer Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cscmr1(pub u32);
impl Cscmr1 {
    #[doc = "Divider for perclk podf."]
    #[must_use]
    #[inline(always)]
    pub const fn perclk_podf(&self) -> super::vals::PerclkPodf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::PerclkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for perclk podf."]
    #[inline(always)]
    pub const fn set_perclk_podf(&mut self, val: super::vals::PerclkPodf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Selector for the perclk clock multiplexor"]
    #[must_use]
    #[inline(always)]
    pub const fn perclk_clk_sel(&self) -> super::vals::PerclkClkSel {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PerclkClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for the perclk clock multiplexor"]
    #[inline(always)]
    pub const fn set_perclk_clk_sel(&mut self, val: super::vals::PerclkClkSel) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Selector for sai1 clock multiplexer"]
    #[must_use]
    #[inline(always)]
    pub const fn sai1_clk_sel(&self) -> super::vals::Sai1ClkSel {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Sai1ClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for sai1 clock multiplexer"]
    #[inline(always)]
    pub const fn set_sai1_clk_sel(&mut self, val: super::vals::Sai1ClkSel) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Selector for sai2 clock multiplexer"]
    #[must_use]
    #[inline(always)]
    pub const fn sai2_clk_sel(&self) -> super::vals::Sai2ClkSel {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Sai2ClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for sai2 clock multiplexer"]
    #[inline(always)]
    pub const fn set_sai2_clk_sel(&mut self, val: super::vals::Sai2ClkSel) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Selector for sai3/adc1/adc2 clock multiplexer"]
    #[must_use]
    #[inline(always)]
    pub const fn sai3_clk_sel(&self) -> super::vals::Sai3ClkSel {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Sai3ClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for sai3/adc1/adc2 clock multiplexer"]
    #[inline(always)]
    pub const fn set_sai3_clk_sel(&mut self, val: super::vals::Sai3ClkSel) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Selector for usdhc1 clock multiplexer"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc1_clk_sel(&self) -> super::vals::Usdhc1ClkSel {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Usdhc1ClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for usdhc1 clock multiplexer"]
    #[inline(always)]
    pub const fn set_usdhc1_clk_sel(&mut self, val: super::vals::Usdhc1ClkSel) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Selector for usdhc2 clock multiplexer"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc2_clk_sel(&self) -> super::vals::Usdhc2ClkSel {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Usdhc2ClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for usdhc2 clock multiplexer"]
    #[inline(always)]
    pub const fn set_usdhc2_clk_sel(&mut self, val: super::vals::Usdhc2ClkSel) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Divider for flexspi clock root."]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_podf(&self) -> super::vals::FlexspiPodf {
        let val = (self.0 >> 23usize) & 0x07;
        super::vals::FlexspiPodf::from_bits(val as u8)
    }
    #[doc = "Divider for flexspi clock root."]
    #[inline(always)]
    pub const fn set_flexspi_podf(&mut self, val: super::vals::FlexspiPodf) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val.to_bits() as u32) & 0x07) << 23usize);
    }
    #[doc = "Selector for flexspi clock multiplexer"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_clk_sel(&self) -> super::vals::FlexspiClkSel {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::FlexspiClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for flexspi clock multiplexer"]
    #[inline(always)]
    pub const fn set_flexspi_clk_sel(&mut self, val: super::vals::FlexspiClkSel) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Cscmr1 {
    #[inline(always)]
    fn default() -> Cscmr1 {
        Cscmr1(0)
    }
}
impl core::fmt::Debug for Cscmr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cscmr1")
            .field("perclk_podf", &self.perclk_podf())
            .field("perclk_clk_sel", &self.perclk_clk_sel())
            .field("sai1_clk_sel", &self.sai1_clk_sel())
            .field("sai2_clk_sel", &self.sai2_clk_sel())
            .field("sai3_clk_sel", &self.sai3_clk_sel())
            .field("usdhc1_clk_sel", &self.usdhc1_clk_sel())
            .field("usdhc2_clk_sel", &self.usdhc2_clk_sel())
            .field("flexspi_podf", &self.flexspi_podf())
            .field("flexspi_clk_sel", &self.flexspi_clk_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cscmr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cscmr1 {{ perclk_podf: {:?}, perclk_clk_sel: {:?}, sai1_clk_sel: {:?}, sai2_clk_sel: {:?}, sai3_clk_sel: {:?}, usdhc1_clk_sel: {:?}, usdhc2_clk_sel: {:?}, flexspi_podf: {:?}, flexspi_clk_sel: {:?} }}",
            self.perclk_podf(),
            self.perclk_clk_sel(),
            self.sai1_clk_sel(),
            self.sai2_clk_sel(),
            self.sai3_clk_sel(),
            self.usdhc1_clk_sel(),
            self.usdhc2_clk_sel(),
            self.flexspi_podf(),
            self.flexspi_clk_sel()
        )
    }
}
#[doc = "CCM Serial Clock Multiplexer Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cscmr2(pub u32);
impl Cscmr2 {
    #[doc = "Divider for CAN/CANFD clock podf."]
    #[must_use]
    #[inline(always)]
    pub const fn can_clk_podf(&self) -> super::vals::CanClkPodf {
        let val = (self.0 >> 2usize) & 0x3f;
        super::vals::CanClkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for CAN/CANFD clock podf."]
    #[inline(always)]
    pub const fn set_can_clk_podf(&mut self, val: super::vals::CanClkPodf) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val.to_bits() as u32) & 0x3f) << 2usize);
    }
    #[doc = "Selector for CAN/CANFD clock multiplexer"]
    #[must_use]
    #[inline(always)]
    pub const fn can_clk_sel(&self) -> super::vals::CanClkSel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::CanClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for CAN/CANFD clock multiplexer"]
    #[inline(always)]
    pub const fn set_can_clk_sel(&mut self, val: super::vals::CanClkSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Selector for flexio2/flexio3 clock multiplexer"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio2_clk_sel(&self) -> super::vals::Flexio2ClkSel {
        let val = (self.0 >> 19usize) & 0x03;
        super::vals::Flexio2ClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for flexio2/flexio3 clock multiplexer"]
    #[inline(always)]
    pub const fn set_flexio2_clk_sel(&mut self, val: super::vals::Flexio2ClkSel) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
    }
}
impl Default for Cscmr2 {
    #[inline(always)]
    fn default() -> Cscmr2 {
        Cscmr2(0)
    }
}
impl core::fmt::Debug for Cscmr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cscmr2")
            .field("can_clk_podf", &self.can_clk_podf())
            .field("can_clk_sel", &self.can_clk_sel())
            .field("flexio2_clk_sel", &self.flexio2_clk_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cscmr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cscmr2 {{ can_clk_podf: {:?}, can_clk_sel: {:?}, flexio2_clk_sel: {:?} }}",
            self.can_clk_podf(),
            self.can_clk_sel(),
            self.flexio2_clk_sel()
        )
    }
}
#[doc = "CCM Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "Status of the value of CCM_REF_EN_B output of ccm"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_en_b(&self) -> super::vals::RefEnB {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RefEnB::from_bits(val as u8)
    }
    #[doc = "Status of the value of CCM_REF_EN_B output of ccm"]
    #[inline(always)]
    pub const fn set_ref_en_b(&mut self, val: super::vals::RefEnB) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Status indication of CAMP2."]
    #[must_use]
    #[inline(always)]
    pub const fn camp2_ready(&self) -> super::vals::Camp2Ready {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Camp2Ready::from_bits(val as u8)
    }
    #[doc = "Status indication of CAMP2."]
    #[inline(always)]
    pub const fn set_camp2_ready(&mut self, val: super::vals::Camp2Ready) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Status indication of on board oscillator"]
    #[must_use]
    #[inline(always)]
    pub const fn cosc_ready(&self) -> super::vals::CsrCoscReady {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::CsrCoscReady::from_bits(val as u8)
    }
    #[doc = "Status indication of on board oscillator"]
    #[inline(always)]
    pub const fn set_cosc_ready(&mut self, val: super::vals::CsrCoscReady) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        Csr(0)
    }
}
impl core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csr")
            .field("ref_en_b", &self.ref_en_b())
            .field("camp2_ready", &self.camp2_ready())
            .field("cosc_ready", &self.cosc_ready())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csr {{ ref_en_b: {:?}, camp2_ready: {:?}, cosc_ready: {:?} }}",
            self.ref_en_b(),
            self.camp2_ready(),
            self.cosc_ready()
        )
    }
}
