#[doc = "TSI Baseline"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Baseline(pub u32);
impl Baseline {
    #[doc = "Baseline"]
    #[must_use]
    #[inline(always)]
    pub const fn baseline(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Baseline"]
    #[inline(always)]
    pub const fn set_baseline(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Base Trace Debounce"]
    #[must_use]
    #[inline(always)]
    pub const fn base_trace_debounce(&self) -> super::vals::BaseTraceDebounce {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::BaseTraceDebounce::from_bits(val as u8)
    }
    #[doc = "Base Trace Debounce"]
    #[inline(always)]
    pub const fn set_base_trace_debounce(&mut self, val: super::vals::BaseTraceDebounce) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Baseline Trace Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn base_trace_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Baseline Trace Enable"]
    #[inline(always)]
    pub const fn set_base_trace_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Threshold Ratio"]
    #[must_use]
    #[inline(always)]
    pub const fn theshold_ratio(&self) -> super::vals::ThesholdRatio {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::ThesholdRatio::from_bits(val as u8)
    }
    #[doc = "Threshold Ratio"]
    #[inline(always)]
    pub const fn set_theshold_ratio(&mut self, val: super::vals::ThesholdRatio) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "Threshold Trace Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn threshold_trace_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Threshold Trace Enable"]
    #[inline(always)]
    pub const fn set_threshold_trace_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Baseline {
    #[inline(always)]
    fn default() -> Baseline {
        Baseline(0)
    }
}
impl core::fmt::Debug for Baseline {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Baseline")
            .field("baseline", &self.baseline())
            .field("base_trace_debounce", &self.base_trace_debounce())
            .field("base_trace_en", &self.base_trace_en())
            .field("theshold_ratio", &self.theshold_ratio())
            .field("threshold_trace_en", &self.threshold_trace_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Baseline {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Baseline {{ baseline: {=u16:?}, base_trace_debounce: {:?}, base_trace_en: {=bool:?}, theshold_ratio: {:?}, threshold_trace_en: {=bool:?} }}",
            self.baseline(),
            self.base_trace_debounce(),
            self.base_trace_en(),
            self.theshold_ratio(),
            self.threshold_trace_en()
        )
    }
}
#[doc = "TSI Channel Merge"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chmerge(pub u32);
impl Chmerge {
    #[doc = "Channel Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn channel_enable(&self) -> super::vals::ChannelEnable {
        let val = (self.0 >> 0usize) & 0x01ff_ffff;
        super::vals::ChannelEnable::from_bits(val as u32)
    }
    #[doc = "Channel Enable"]
    #[inline(always)]
    pub const fn set_channel_enable(&mut self, val: super::vals::ChannelEnable) {
        self.0 = (self.0 & !(0x01ff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0x01ff_ffff) << 0usize);
    }
}
impl Default for Chmerge {
    #[inline(always)]
    fn default() -> Chmerge {
        Chmerge(0)
    }
}
impl core::fmt::Debug for Chmerge {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Chmerge")
            .field("channel_enable", &self.channel_enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chmerge {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Chmerge {{ channel_enable: {:?} }}",
            self.channel_enable()
        )
    }
}
#[doc = "TSI CONFIG (TSI_CONFIG) for Self-Capacitor"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::ConfigMode {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ConfigMode::from_bits(val as u8)
    }
    #[doc = "Mode"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::ConfigMode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "TSI Channel"]
    #[must_use]
    #[inline(always)]
    pub const fn tsich(&self) -> super::vals::Tsich {
        let val = (self.0 >> 1usize) & 0x1f;
        super::vals::Tsich::from_bits(val as u8)
    }
    #[doc = "TSI Channel"]
    #[inline(always)]
    pub const fn set_tsich(&mut self, val: super::vals::Tsich) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val.to_bits() as u32) & 0x1f) << 1usize);
    }
    #[doc = "Self-Capacitance Noise Cancelation"]
    #[must_use]
    #[inline(always)]
    pub const fn s_noise(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Self-Capacitance Noise Cancelation"]
    #[inline(always)]
    pub const fn set_s_noise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Self-Capacitance Charge Current Multiple"]
    #[must_use]
    #[inline(always)]
    pub const fn s_xch(&self) -> super::vals::SXch {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::SXch::from_bits(val as u8)
    }
    #[doc = "Self-Capacitance Charge Current Multiple"]
    #[inline(always)]
    pub const fn set_s_xch(&mut self, val: super::vals::SXch) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Self-Capacitance Input Current Multiple"]
    #[must_use]
    #[inline(always)]
    pub const fn s_xin(&self) -> super::vals::SXin {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::SXin::from_bits(val as u8)
    }
    #[doc = "Self-Capacitance Input Current Multiple"]
    #[inline(always)]
    pub const fn set_s_xin(&mut self, val: super::vals::SXin) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Capacitor Trim Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn s_ctrim(&self) -> super::vals::SCtrim {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::SCtrim::from_bits(val as u8)
    }
    #[doc = "Capacitor Trim Setting"]
    #[inline(always)]
    pub const fn set_s_ctrim(&mut self, val: super::vals::SCtrim) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "Self-Capacitance Sensitivity Boost"]
    #[must_use]
    #[inline(always)]
    pub const fn s_sen(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Self-Capacitance Sensitivity Boost"]
    #[inline(always)]
    pub const fn set_s_sen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Self-Capacitance Discharge Current Multiple"]
    #[must_use]
    #[inline(always)]
    pub const fn s_xdn(&self) -> super::vals::SXdn {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::SXdn::from_bits(val as u8)
    }
    #[doc = "Self-Capacitance Discharge Current Multiple"]
    #[inline(always)]
    pub const fn set_s_xdn(&mut self, val: super::vals::SXdn) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "S_XIN Adjust Ratio"]
    #[must_use]
    #[inline(always)]
    pub const fn s_xin_add(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "S_XIN Adjust Ratio"]
    #[inline(always)]
    pub const fn set_s_xin_add(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
impl core::fmt::Debug for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Config")
            .field("mode", &self.mode())
            .field("tsich", &self.tsich())
            .field("s_noise", &self.s_noise())
            .field("s_xch", &self.s_xch())
            .field("s_xin", &self.s_xin())
            .field("s_ctrim", &self.s_ctrim())
            .field("s_sen", &self.s_sen())
            .field("s_xdn", &self.s_xdn())
            .field("s_xin_add", &self.s_xin_add())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Config {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Config {{ mode: {:?}, tsich: {:?}, s_noise: {=bool:?}, s_xch: {:?}, s_xin: {:?}, s_ctrim: {:?}, s_sen: {=bool:?}, s_xdn: {:?}, s_xin_add: {=bool:?} }}",
            self.mode(),
            self.tsich(),
            self.s_noise(),
            self.s_xch(),
            self.s_xin(),
            self.s_ctrim(),
            self.s_sen(),
            self.s_xdn(),
            self.s_xin_add()
        )
    }
}
#[doc = "TSI CONFIG (TSI_CONFIG) for Mutual-Capacitor"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConfigMutual(pub u32);
impl ConfigMutual {
    #[doc = "Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::ConfigMutualMode {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ConfigMutualMode::from_bits(val as u8)
    }
    #[doc = "Mode"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::ConfigMutualMode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "NMOS Current Mirror"]
    #[must_use]
    #[inline(always)]
    pub const fn m_nmirror(&self) -> super::vals::MNmirror {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::MNmirror::from_bits(val as u8)
    }
    #[doc = "NMOS Current Mirror"]
    #[inline(always)]
    pub const fn set_m_nmirror(&mut self, val: super::vals::MNmirror) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "PMOS Current Mirror on Right Side"]
    #[must_use]
    #[inline(always)]
    pub const fn m_pmirrorr(&self) -> super::vals::MPmirrorr {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::MPmirrorr::from_bits(val as u8)
    }
    #[doc = "PMOS Current Mirror on Right Side"]
    #[inline(always)]
    pub const fn set_m_pmirrorr(&mut self, val: super::vals::MPmirrorr) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "PMOS Current Mirror on Left Side"]
    #[must_use]
    #[inline(always)]
    pub const fn m_pmirrorl(&self) -> super::vals::MPmirrorl {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::MPmirrorl::from_bits(val as u8)
    }
    #[doc = "PMOS Current Mirror on Left Side"]
    #[inline(always)]
    pub const fn set_m_pmirrorl(&mut self, val: super::vals::MPmirrorl) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
    #[doc = "Mutual-Capacitance RX Channel Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn m_sel_rx(&self) -> super::vals::MSelRx {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::MSelRx::from_bits(val as u8)
    }
    #[doc = "Mutual-Capacitance RX Channel Selection"]
    #[inline(always)]
    pub const fn set_m_sel_rx(&mut self, val: super::vals::MSelRx) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Mutual-Capacitance TX Channel Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn m_sel_tx(&self) -> super::vals::MSelTx {
        let val = (self.0 >> 13usize) & 0x07;
        super::vals::MSelTx::from_bits(val as u8)
    }
    #[doc = "Mutual-Capacitance TX Channel Selection"]
    #[inline(always)]
    pub const fn set_m_sel_tx(&mut self, val: super::vals::MSelTx) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
    }
    #[doc = "Mutual-Capacitance Counter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn m_cnt_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Mutual-Capacitance Counter Enable"]
    #[inline(always)]
    pub const fn set_m_cnt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Mutual-Capacitance TX Pulldown Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn m_tx_pd_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Mutual-Capacitance TX Pulldown Enable"]
    #[inline(always)]
    pub const fn set_m_tx_pd_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Mutual-Capacitance Sensitivity Boost"]
    #[must_use]
    #[inline(always)]
    pub const fn m_sen_boost(&self) -> super::vals::MSenBoost {
        let val = (self.0 >> 18usize) & 0x1f;
        super::vals::MSenBoost::from_bits(val as u8)
    }
    #[doc = "Mutual-Capacitance Sensitivity Boost"]
    #[inline(always)]
    pub const fn set_m_sen_boost(&mut self, val: super::vals::MSenBoost) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val.to_bits() as u32) & 0x1f) << 18usize);
    }
    #[doc = "Mutual-Capacitance Precharge Resistor"]
    #[must_use]
    #[inline(always)]
    pub const fn m_pre_res(&self) -> super::vals::MPreRes {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::MPreRes::from_bits(val as u8)
    }
    #[doc = "Mutual-Capacitance Precharge Resistor"]
    #[inline(always)]
    pub const fn set_m_pre_res(&mut self, val: super::vals::MPreRes) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "Mutual-Capacitance Precharge Current"]
    #[must_use]
    #[inline(always)]
    pub const fn m_pre_current(&self) -> super::vals::MPreCurrent {
        let val = (self.0 >> 29usize) & 0x07;
        super::vals::MPreCurrent::from_bits(val as u8)
    }
    #[doc = "Mutual-Capacitance Precharge Current"]
    #[inline(always)]
    pub const fn set_m_pre_current(&mut self, val: super::vals::MPreCurrent) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
    }
}
impl Default for ConfigMutual {
    #[inline(always)]
    fn default() -> ConfigMutual {
        ConfigMutual(0)
    }
}
impl core::fmt::Debug for ConfigMutual {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ConfigMutual")
            .field("mode", &self.mode())
            .field("m_nmirror", &self.m_nmirror())
            .field("m_pmirrorr", &self.m_pmirrorr())
            .field("m_pmirrorl", &self.m_pmirrorl())
            .field("m_sel_rx", &self.m_sel_rx())
            .field("m_sel_tx", &self.m_sel_tx())
            .field("m_cnt_en", &self.m_cnt_en())
            .field("m_tx_pd_en", &self.m_tx_pd_en())
            .field("m_sen_boost", &self.m_sen_boost())
            .field("m_pre_res", &self.m_pre_res())
            .field("m_pre_current", &self.m_pre_current())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ConfigMutual {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ConfigMutual {{ mode: {:?}, m_nmirror: {:?}, m_pmirrorr: {:?}, m_pmirrorl: {:?}, m_sel_rx: {:?}, m_sel_tx: {:?}, m_cnt_en: {=bool:?}, m_tx_pd_en: {=bool:?}, m_sen_boost: {:?}, m_pre_res: {:?}, m_pre_current: {:?} }}",
            self.mode(),
            self.m_nmirror(),
            self.m_pmirrorr(),
            self.m_pmirrorl(),
            self.m_sel_rx(),
            self.m_sel_tx(),
            self.m_cnt_en(),
            self.m_tx_pd_en(),
            self.m_sen_boost(),
            self.m_pre_res(),
            self.m_pre_current()
        )
    }
}
#[doc = "TSI Data and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data(pub u32);
impl Data {
    #[doc = "TSI Conversion Counter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn tsicnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TSI Conversion Counter Value"]
    #[inline(always)]
    pub const fn set_tsicnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "End-of-Scan Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn eosf(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "End-of-Scan Flag"]
    #[inline(always)]
    pub const fn set_eosf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Overrun Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrunf(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun Flag"]
    #[inline(always)]
    pub const fn set_overrunf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Out-of-Range Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn outrgf(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Out-of-Range Flag"]
    #[inline(always)]
    pub const fn set_outrgf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Data {
    #[inline(always)]
    fn default() -> Data {
        Data(0)
    }
}
impl core::fmt::Debug for Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Data")
            .field("tsicnt", &self.tsicnt())
            .field("eosf", &self.eosf())
            .field("overrunf", &self.overrunf())
            .field("outrgf", &self.outrgf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Data {{ tsicnt: {=u16:?}, eosf: {=bool:?}, overrunf: {=bool:?}, outrgf: {=bool:?} }}",
            self.tsicnt(),
            self.eosf(),
            self.overrunf(),
            self.outrgf()
        )
    }
}
#[doc = "TSI General Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gencs(pub u32);
impl Gencs {
    #[doc = "In-Progress DMA Transfer Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen_eos(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "In-Progress DMA Transfer Request Enable"]
    #[inline(always)]
    pub const fn set_dmaen_eos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Out-of-Range DMA Transfer Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen_outrg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Out-of-Range DMA Transfer Request Enable"]
    #[inline(always)]
    pub const fn set_dmaen_outrg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Scan Trigger Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn stm(&self) -> super::vals::Stm {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Stm::from_bits(val as u8)
    }
    #[doc = "Scan Trigger Mode"]
    #[inline(always)]
    pub const fn set_stm(&mut self, val: super::vals::Stm) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "TSI Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn stpe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TSI Stop Enable"]
    #[inline(always)]
    pub const fn set_stpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TSI Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tsien(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TSI Enable"]
    #[inline(always)]
    pub const fn set_tsien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Software Trigger Start"]
    #[must_use]
    #[inline(always)]
    pub const fn swts(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Software Trigger Start"]
    #[inline(always)]
    pub const fn set_swts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Capacitor Fine Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrim_fine(&self) -> super::vals::CtrimFine {
        let val = (self.0 >> 9usize) & 0x07;
        super::vals::CtrimFine::from_bits(val as u8)
    }
    #[doc = "Capacitor Fine Trim"]
    #[inline(always)]
    pub const fn set_ctrim_fine(&mut self, val: super::vals::CtrimFine) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
    }
    #[doc = "Delta Voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn dvolt(&self) -> super::vals::Dvolt {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Dvolt::from_bits(val as u8)
    }
    #[doc = "Delta Voltage"]
    #[inline(always)]
    pub const fn set_dvolt(&mut self, val: super::vals::Dvolt) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Debounce"]
    #[must_use]
    #[inline(always)]
    pub const fn debounce(&self) -> super::vals::Debounce {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Debounce::from_bits(val as u8)
    }
    #[doc = "Debounce"]
    #[inline(always)]
    pub const fn set_debounce(&mut self, val: super::vals::Debounce) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Proximity Enable Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn s_prox_en(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Proximity Enable Signal"]
    #[inline(always)]
    pub const fn set_s_prox_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Set Clock"]
    #[must_use]
    #[inline(always)]
    pub const fn setclk(&self) -> super::vals::Setclk {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Setclk::from_bits(val as u8)
    }
    #[doc = "Set Clock"]
    #[inline(always)]
    pub const fn set_setclk(&mut self, val: super::vals::Setclk) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "End-of-Scan Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn esor(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "End-of-Scan Interrupt Enable"]
    #[inline(always)]
    pub const fn set_esor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Out-of-Range Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn outrg_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Out-of-Range Interrupt Enable"]
    #[inline(always)]
    pub const fn set_outrg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Gencs {
    #[inline(always)]
    fn default() -> Gencs {
        Gencs(0)
    }
}
impl core::fmt::Debug for Gencs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gencs")
            .field("dmaen_eos", &self.dmaen_eos())
            .field("dmaen_outrg", &self.dmaen_outrg())
            .field("stm", &self.stm())
            .field("stpe", &self.stpe())
            .field("tsien", &self.tsien())
            .field("swts", &self.swts())
            .field("ctrim_fine", &self.ctrim_fine())
            .field("dvolt", &self.dvolt())
            .field("debounce", &self.debounce())
            .field("s_prox_en", &self.s_prox_en())
            .field("setclk", &self.setclk())
            .field("esor", &self.esor())
            .field("outrg_en", &self.outrg_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gencs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gencs {{ dmaen_eos: {=bool:?}, dmaen_outrg: {=bool:?}, stm: {:?}, stpe: {=bool:?}, tsien: {=bool:?}, swts: {=bool:?}, ctrim_fine: {:?}, dvolt: {:?}, debounce: {:?}, s_prox_en: {=bool:?}, setclk: {:?}, esor: {=bool:?}, outrg_en: {=bool:?} }}",
            self.dmaen_eos(),
            self.dmaen_outrg(),
            self.stm(),
            self.stpe(),
            self.tsien(),
            self.swts(),
            self.ctrim_fine(),
            self.dvolt(),
            self.debounce(),
            self.s_prox_en(),
            self.setclk(),
            self.esor(),
            self.outrg_en()
        )
    }
}
#[doc = "TSI Miscellaneous"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc(pub u32);
impl Misc {
    #[doc = "Oscillator Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_clk_sel(&self) -> super::vals::OscClkSel {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::OscClkSel::from_bits(val as u8)
    }
    #[doc = "Oscillator Clock Select"]
    #[inline(always)]
    pub const fn set_osc_clk_sel(&mut self, val: super::vals::OscClkSel) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Test Finger"]
    #[must_use]
    #[inline(always)]
    pub const fn test_finger(&self) -> super::vals::TestFinger {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::TestFinger::from_bits(val as u8)
    }
    #[doc = "Test Finger"]
    #[inline(always)]
    pub const fn set_test_finger(&mut self, val: super::vals::TestFinger) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Test Finger Function Enable Signals"]
    #[must_use]
    #[inline(always)]
    pub const fn test_finger_en(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Test Finger Function Enable Signals"]
    #[inline(always)]
    pub const fn set_test_finger_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TSI Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn clkdivider(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "TSI Clock Divider"]
    #[inline(always)]
    pub const fn set_clkdivider(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Misc {
    #[inline(always)]
    fn default() -> Misc {
        Misc(0)
    }
}
impl core::fmt::Debug for Misc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misc")
            .field("osc_clk_sel", &self.osc_clk_sel())
            .field("test_finger", &self.test_finger())
            .field("test_finger_en", &self.test_finger_en())
            .field("clkdivider", &self.clkdivider())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc {{ osc_clk_sel: {:?}, test_finger: {:?}, test_finger_en: {=bool:?}, clkdivider: {=u8:?} }}",
            self.osc_clk_sel(),
            self.test_finger(),
            self.test_finger_en(),
            self.clkdivider()
        )
    }
}
#[doc = "TSI Mutual-Capacitance"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mul(pub u32);
impl Mul {
    #[doc = "Mutual-Capacitance Prevoltage"]
    #[must_use]
    #[inline(always)]
    pub const fn m_vpre_choose(&self) -> super::vals::MVpreChoose {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::MVpreChoose::from_bits(val as u8)
    }
    #[doc = "Mutual-Capacitance Prevoltage"]
    #[inline(always)]
    pub const fn set_m_vpre_choose(&mut self, val: super::vals::MVpreChoose) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Mutual-Capacitance Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn m_mode(&self) -> super::vals::MMode {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::MMode::from_bits(val as u8)
    }
    #[doc = "Mutual-Capacitance Mode"]
    #[inline(always)]
    pub const fn set_m_mode(&mut self, val: super::vals::MMode) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Mutual-Capacitance Trim Cap"]
    #[must_use]
    #[inline(always)]
    pub const fn m_trim_cap(&self) -> super::vals::MTrimCap {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::MTrimCap::from_bits(val as u8)
    }
    #[doc = "Mutual-Capacitance Trim Cap"]
    #[inline(always)]
    pub const fn set_m_trim_cap(&mut self, val: super::vals::MTrimCap) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Mutual-Capacitance TX Used"]
    #[must_use]
    #[inline(always)]
    pub const fn m_tx_used(&self) -> super::vals::MTxUsed {
        let val = (self.0 >> 5usize) & 0xff;
        super::vals::MTxUsed::from_bits(val as u8)
    }
    #[doc = "Mutual-Capacitance TX Used"]
    #[inline(always)]
    pub const fn set_m_tx_used(&mut self, val: super::vals::MTxUsed) {
        self.0 = (self.0 & !(0xff << 5usize)) | (((val.to_bits() as u32) & 0xff) << 5usize);
    }
    #[doc = "Mutual-Capacitance Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn m_trim(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Mutual-Capacitance Trim"]
    #[inline(always)]
    pub const fn set_m_trim(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Mul {
    #[inline(always)]
    fn default() -> Mul {
        Mul(0)
    }
}
impl core::fmt::Debug for Mul {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mul")
            .field("m_vpre_choose", &self.m_vpre_choose())
            .field("m_mode", &self.m_mode())
            .field("m_trim_cap", &self.m_trim_cap())
            .field("m_tx_used", &self.m_tx_used())
            .field("m_trim", &self.m_trim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mul {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mul {{ m_vpre_choose: {:?}, m_mode: {:?}, m_trim_cap: {:?}, m_tx_used: {:?}, m_trim: {=u16:?} }}",
            self.m_vpre_choose(),
            self.m_mode(),
            self.m_trim_cap(),
            self.m_tx_used(),
            self.m_trim()
        )
    }
}
#[doc = "TSI Shield"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shield(pub u32);
impl Shield {
    #[doc = "Shield Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn shield_enable(&self) -> super::vals::ShieldEnable {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::ShieldEnable::from_bits(val as u8)
    }
    #[doc = "Shield Enable"]
    #[inline(always)]
    pub const fn set_shield_enable(&mut self, val: super::vals::ShieldEnable) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Mutual-Capacitance Sensitivity Resistor"]
    #[must_use]
    #[inline(always)]
    pub const fn m_sen_res(&self) -> super::vals::MSenRes {
        let val = (self.0 >> 25usize) & 0x3f;
        super::vals::MSenRes::from_bits(val as u8)
    }
    #[doc = "Mutual-Capacitance Sensitivity Resistor"]
    #[inline(always)]
    pub const fn set_m_sen_res(&mut self, val: super::vals::MSenRes) {
        self.0 = (self.0 & !(0x3f << 25usize)) | (((val.to_bits() as u32) & 0x3f) << 25usize);
    }
}
impl Default for Shield {
    #[inline(always)]
    fn default() -> Shield {
        Shield(0)
    }
}
impl core::fmt::Debug for Shield {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shield")
            .field("shield_enable", &self.shield_enable())
            .field("m_sen_res", &self.m_sen_res())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shield {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shield {{ shield_enable: {:?}, m_sen_res: {:?} }}",
            self.shield_enable(),
            self.m_sen_res()
        )
    }
}
#[doc = "TSI SINC Filter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sinc(pub u32);
impl Sinc {
    #[doc = "SSC Output Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ssc_control_out(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SSC Output Control"]
    #[inline(always)]
    pub const fn set_ssc_control_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SINC Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn sinc_valid(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SINC Valid"]
    #[inline(always)]
    pub const fn set_sinc_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SINC Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sinc_overflow_flag(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SINC Overflow Flag"]
    #[inline(always)]
    pub const fn set_sinc_overflow_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Switch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn switch_enable(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Switch Enable"]
    #[inline(always)]
    pub const fn set_switch_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Decimation"]
    #[must_use]
    #[inline(always)]
    pub const fn decimation(&self) -> super::vals::Decimation {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Decimation::from_bits(val as u8)
    }
    #[doc = "Decimation"]
    #[inline(always)]
    pub const fn set_decimation(&mut self, val: super::vals::Decimation) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Order"]
    #[must_use]
    #[inline(always)]
    pub const fn order(&self) -> super::vals::Order {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Order::from_bits(val as u8)
    }
    #[doc = "Order"]
    #[inline(always)]
    pub const fn set_order(&mut self, val: super::vals::Order) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Cutoff"]
    #[must_use]
    #[inline(always)]
    pub const fn cutoff(&self) -> super::vals::Cutoff {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cutoff::from_bits(val as u8)
    }
    #[doc = "Cutoff"]
    #[inline(always)]
    pub const fn set_cutoff(&mut self, val: super::vals::Cutoff) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Sinc {
    #[inline(always)]
    fn default() -> Sinc {
        Sinc(0)
    }
}
impl core::fmt::Debug for Sinc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sinc")
            .field("ssc_control_out", &self.ssc_control_out())
            .field("sinc_valid", &self.sinc_valid())
            .field("sinc_overflow_flag", &self.sinc_overflow_flag())
            .field("switch_enable", &self.switch_enable())
            .field("decimation", &self.decimation())
            .field("order", &self.order())
            .field("cutoff", &self.cutoff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sinc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sinc {{ ssc_control_out: {=bool:?}, sinc_valid: {=bool:?}, sinc_overflow_flag: {=bool:?}, switch_enable: {=bool:?}, decimation: {:?}, order: {:?}, cutoff: {:?} }}",
            self.ssc_control_out(),
            self.sinc_valid(),
            self.sinc_overflow_flag(),
            self.switch_enable(),
            self.decimation(),
            self.order(),
            self.cutoff()
        )
    }
}
#[doc = "TSI SSC 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssc0(pub u32);
impl Ssc0 {
    #[doc = "SSC Prescale Number"]
    #[must_use]
    #[inline(always)]
    pub const fn ssc_prescale_num(&self) -> super::vals::SscPrescaleNum {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::SscPrescaleNum::from_bits(val as u8)
    }
    #[doc = "SSC Prescale Number"]
    #[inline(always)]
    pub const fn set_ssc_prescale_num(&mut self, val: super::vals::SscPrescaleNum) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Base Nocharge Number"]
    #[must_use]
    #[inline(always)]
    pub const fn base_nocharge_num(&self) -> super::vals::BaseNochargeNum {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::BaseNochargeNum::from_bits(val as u8)
    }
    #[doc = "Base Nocharge Number"]
    #[inline(always)]
    pub const fn set_base_nocharge_num(&mut self, val: super::vals::BaseNochargeNum) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Charge Number"]
    #[must_use]
    #[inline(always)]
    pub const fn charge_num(&self) -> super::vals::ChargeNum {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::ChargeNum::from_bits(val as u8)
    }
    #[doc = "Charge Number"]
    #[inline(always)]
    pub const fn set_charge_num(&mut self, val: super::vals::ChargeNum) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "SSC Control Reverse"]
    #[must_use]
    #[inline(always)]
    pub const fn ssc_control_reverse(&self) -> super::vals::SscControlReverse {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::SscControlReverse::from_bits(val as u8)
    }
    #[doc = "SSC Control Reverse"]
    #[inline(always)]
    pub const fn set_ssc_control_reverse(&mut self, val: super::vals::SscControlReverse) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SSC Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ssc_mode(&self) -> super::vals::SscMode {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::SscMode::from_bits(val as u8)
    }
    #[doc = "SSC Mode"]
    #[inline(always)]
    pub const fn set_ssc_mode(&mut self, val: super::vals::SscMode) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "PRBS Output Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn prbs_outsel(&self) -> super::vals::PrbsOutsel {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::PrbsOutsel::from_bits(val as u8)
    }
    #[doc = "PRBS Output Selection"]
    #[inline(always)]
    pub const fn set_prbs_outsel(&mut self, val: super::vals::PrbsOutsel) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ssc0 {
    #[inline(always)]
    fn default() -> Ssc0 {
        Ssc0(0)
    }
}
impl core::fmt::Debug for Ssc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssc0")
            .field("ssc_prescale_num", &self.ssc_prescale_num())
            .field("base_nocharge_num", &self.base_nocharge_num())
            .field("charge_num", &self.charge_num())
            .field("ssc_control_reverse", &self.ssc_control_reverse())
            .field("ssc_mode", &self.ssc_mode())
            .field("prbs_outsel", &self.prbs_outsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssc0 {{ ssc_prescale_num: {:?}, base_nocharge_num: {:?}, charge_num: {:?}, ssc_control_reverse: {:?}, ssc_mode: {:?}, prbs_outsel: {:?} }}",
            self.ssc_prescale_num(),
            self.base_nocharge_num(),
            self.charge_num(),
            self.ssc_control_reverse(),
            self.ssc_mode(),
            self.prbs_outsel()
        )
    }
}
#[doc = "TSI SSC 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssc1(pub u32);
impl Ssc1 {
    #[doc = "PRBS Low Seed"]
    #[must_use]
    #[inline(always)]
    pub const fn prbs_seed_lo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "PRBS Low Seed"]
    #[inline(always)]
    pub const fn set_prbs_seed_lo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "PRBS High Seed"]
    #[must_use]
    #[inline(always)]
    pub const fn prbs_seed_hi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "PRBS High Seed"]
    #[inline(always)]
    pub const fn set_prbs_seed_hi(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "PRBS Low Weight"]
    #[must_use]
    #[inline(always)]
    pub const fn prbs_weight_lo(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "PRBS Low Weight"]
    #[inline(always)]
    pub const fn set_prbs_weight_lo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "PRBS High Weight"]
    #[must_use]
    #[inline(always)]
    pub const fn prbs_weight_hi(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "PRBS High Weight"]
    #[inline(always)]
    pub const fn set_prbs_weight_hi(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ssc1 {
    #[inline(always)]
    fn default() -> Ssc1 {
        Ssc1(0)
    }
}
impl core::fmt::Debug for Ssc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssc1")
            .field("prbs_seed_lo", &self.prbs_seed_lo())
            .field("prbs_seed_hi", &self.prbs_seed_hi())
            .field("prbs_weight_lo", &self.prbs_weight_lo())
            .field("prbs_weight_hi", &self.prbs_weight_hi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssc1 {{ prbs_seed_lo: {=u8:?}, prbs_seed_hi: {=u8:?}, prbs_weight_lo: {=u8:?}, prbs_weight_hi: {=u8:?} }}",
            self.prbs_seed_lo(),
            self.prbs_seed_hi(),
            self.prbs_weight_lo(),
            self.prbs_weight_hi()
        )
    }
}
#[doc = "TSI SSC 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssc2(pub u32);
impl Ssc2 {
    #[doc = "Move Repeat Number"]
    #[must_use]
    #[inline(always)]
    pub const fn move_repeat_num(&self) -> super::vals::MoveRepeatNum {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::MoveRepeatNum::from_bits(val as u8)
    }
    #[doc = "Move Repeat Number"]
    #[inline(always)]
    pub const fn set_move_repeat_num(&mut self, val: super::vals::MoveRepeatNum) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Move Steps Number"]
    #[must_use]
    #[inline(always)]
    pub const fn move_steps_num(&self) -> super::vals::MoveStepsNum {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::MoveStepsNum::from_bits(val as u8)
    }
    #[doc = "Move Steps Number"]
    #[inline(always)]
    pub const fn set_move_steps_num(&mut self, val: super::vals::MoveStepsNum) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Move Nocharge Maximum"]
    #[must_use]
    #[inline(always)]
    pub const fn move_nocharge_max(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Move Nocharge Maximum"]
    #[inline(always)]
    pub const fn set_move_nocharge_max(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Move Nocharge Minimum"]
    #[must_use]
    #[inline(always)]
    pub const fn move_nocharge_min(&self) -> super::vals::MoveNochargeMin {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::MoveNochargeMin::from_bits(val as u8)
    }
    #[doc = "Move Nocharge Minimum"]
    #[inline(always)]
    pub const fn set_move_nocharge_min(&mut self, val: super::vals::MoveNochargeMin) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ssc2 {
    #[inline(always)]
    fn default() -> Ssc2 {
        Ssc2(0)
    }
}
impl core::fmt::Debug for Ssc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssc2")
            .field("move_repeat_num", &self.move_repeat_num())
            .field("move_steps_num", &self.move_steps_num())
            .field("move_nocharge_max", &self.move_nocharge_max())
            .field("move_nocharge_min", &self.move_nocharge_min())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssc2 {{ move_repeat_num: {:?}, move_steps_num: {:?}, move_nocharge_max: {=u8:?}, move_nocharge_min: {:?} }}",
            self.move_repeat_num(),
            self.move_steps_num(),
            self.move_nocharge_max(),
            self.move_nocharge_min()
        )
    }
}
#[doc = "TSI AUTO TRIG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig(pub u32);
impl Trig {
    #[doc = "Trigger Period Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_period_counter(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Trigger Period Counter"]
    #[inline(always)]
    pub const fn set_trig_period_counter(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
    #[doc = "Trigger Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_clk_divider(&self) -> super::vals::TrigClkDivider {
        let val = (self.0 >> 24usize) & 0x1f;
        super::vals::TrigClkDivider::from_bits(val as u8)
    }
    #[doc = "Trigger Clock Divider"]
    #[inline(always)]
    pub const fn set_trig_clk_divider(&mut self, val: super::vals::TrigClkDivider) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val.to_bits() as u32) & 0x1f) << 24usize);
    }
    #[doc = "Trigger Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Enable"]
    #[inline(always)]
    pub const fn set_trig_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Trigger Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_clk_sel(&self) -> super::vals::TrigClkSel {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::TrigClkSel::from_bits(val as u8)
    }
    #[doc = "Trigger Clock Select"]
    #[inline(always)]
    pub const fn set_trig_clk_sel(&mut self, val: super::vals::TrigClkSel) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig {
    #[inline(always)]
    fn default() -> Trig {
        Trig(0)
    }
}
impl core::fmt::Debug for Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig")
            .field("trig_period_counter", &self.trig_period_counter())
            .field("trig_clk_divider", &self.trig_clk_divider())
            .field("trig_en", &self.trig_en())
            .field("trig_clk_sel", &self.trig_clk_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig {{ trig_period_counter: {=u32:?}, trig_clk_divider: {:?}, trig_en: {=bool:?}, trig_clk_sel: {:?} }}",
            self.trig_period_counter(),
            self.trig_clk_divider(),
            self.trig_en(),
            self.trig_clk_sel()
        )
    }
}
#[doc = "TSI Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tshd(pub u32);
impl Tshd {
    #[doc = "TSI Wakeup Channel Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn thresl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TSI Wakeup Channel Low Threshold"]
    #[inline(always)]
    pub const fn set_thresl(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TSI Wakeup Channel High Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn thresh(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TSI Wakeup Channel High Threshold"]
    #[inline(always)]
    pub const fn set_thresh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Tshd {
    #[inline(always)]
    fn default() -> Tshd {
        Tshd(0)
    }
}
impl core::fmt::Debug for Tshd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tshd")
            .field("thresl", &self.thresl())
            .field("thresh", &self.thresh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tshd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tshd {{ thresl: {=u16:?}, thresh: {=u16:?} }}",
            self.thresl(),
            self.thresh()
        )
    }
}
