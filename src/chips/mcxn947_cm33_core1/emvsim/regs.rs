#[doc = "Block Guard Time Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgtVal(pub u32);
impl BgtVal {
    #[doc = "Block Guard Time Value"]
    #[must_use]
    #[inline(always)]
    pub const fn bgt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Block Guard Time Value"]
    #[inline(always)]
    pub const fn set_bgt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for BgtVal {
    #[inline(always)]
    fn default() -> BgtVal {
        BgtVal(0)
    }
}
impl core::fmt::Debug for BgtVal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BgtVal").field("bgt", &self.bgt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BgtVal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BgtVal {{ bgt: {=u16:?} }}", self.bgt())
    }
}
#[doc = "Block Wait Time Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BwtVal(pub u32);
impl BwtVal {
    #[doc = "Block Wait Time Value"]
    #[must_use]
    #[inline(always)]
    pub const fn bwt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Block Wait Time Value"]
    #[inline(always)]
    pub const fn set_bwt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BwtVal {
    #[inline(always)]
    fn default() -> BwtVal {
        BwtVal(0)
    }
}
impl core::fmt::Debug for BwtVal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BwtVal").field("bwt", &self.bwt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BwtVal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BwtVal {{ bwt: {=u32:?} }}", self.bwt())
    }
}
#[doc = "Clock Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkcfg(pub u32);
impl Clkcfg {
    #[doc = "Clock Prescaler Value"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_prsc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Prescaler Value"]
    #[inline(always)]
    pub const fn set_clk_prsc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "General Purpose Counter 1 Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn gpcnt1_clk_sel(&self) -> super::vals::Gpcnt1ClkSel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Gpcnt1ClkSel::from_bits(val as u8)
    }
    #[doc = "General Purpose Counter 1 Clock Select"]
    #[inline(always)]
    pub const fn set_gpcnt1_clk_sel(&mut self, val: super::vals::Gpcnt1ClkSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "General Purpose Counter 0 Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn gpcnt0_clk_sel(&self) -> super::vals::Gpcnt0ClkSel {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Gpcnt0ClkSel::from_bits(val as u8)
    }
    #[doc = "General Purpose Counter 0 Clock Select"]
    #[inline(always)]
    pub const fn set_gpcnt0_clk_sel(&mut self, val: super::vals::Gpcnt0ClkSel) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
}
impl Default for Clkcfg {
    #[inline(always)]
    fn default() -> Clkcfg {
        Clkcfg(0)
    }
}
impl core::fmt::Debug for Clkcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkcfg")
            .field("clk_prsc", &self.clk_prsc())
            .field("gpcnt1_clk_sel", &self.gpcnt1_clk_sel())
            .field("gpcnt0_clk_sel", &self.gpcnt0_clk_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clkcfg {{ clk_prsc: {=u8:?}, gpcnt1_clk_sel: {:?}, gpcnt0_clk_sel: {:?} }}",
            self.clk_prsc(),
            self.gpcnt1_clk_sel(),
            self.gpcnt0_clk_sel()
        )
    }
}
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Inverse Convention"]
    #[must_use]
    #[inline(always)]
    pub const fn ic(&self) -> super::vals::Ic {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ic::from_bits(val as u8)
    }
    #[doc = "Inverse Convention"]
    #[inline(always)]
    pub const fn set_ic(&mut self, val: super::vals::Ic) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Initial Character Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn icm(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Initial Character Mode"]
    #[inline(always)]
    pub const fn set_icm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Auto NACK Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn anack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Auto NACK Enable"]
    #[inline(always)]
    pub const fn set_anack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Overrun NACK Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn onack(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun NACK Enable"]
    #[inline(always)]
    pub const fn set_onack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Flush Receiver"]
    #[must_use]
    #[inline(always)]
    pub const fn flsh_rx(&self) -> super::vals::FlshRx {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::FlshRx::from_bits(val as u8)
    }
    #[doc = "Flush Receiver"]
    #[inline(always)]
    pub const fn set_flsh_rx(&mut self, val: super::vals::FlshRx) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Flush Transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn flsh_tx(&self) -> super::vals::FlshTx {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::FlshTx::from_bits(val as u8)
    }
    #[doc = "Flush Transmitter"]
    #[inline(always)]
    pub const fn set_flsh_tx(&mut self, val: super::vals::FlshTx) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_rst(&self) -> super::vals::SwRst {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::SwRst::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_sw_rst(&mut self, val: super::vals::SwRst) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Kill Internal Clocks"]
    #[must_use]
    #[inline(always)]
    pub const fn kill_clocks(&self) -> super::vals::KillClocks {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::KillClocks::from_bits(val as u8)
    }
    #[doc = "Kill Internal Clocks"]
    #[inline(always)]
    pub const fn set_kill_clocks(&mut self, val: super::vals::KillClocks) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Doze Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn doze_en(&self) -> super::vals::DozeEn {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::DozeEn::from_bits(val as u8)
    }
    #[doc = "Doze Enable"]
    #[inline(always)]
    pub const fn set_doze_en(&mut self, val: super::vals::DozeEn) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "STOP Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn stop_en(&self) -> super::vals::StopEn {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::StopEn::from_bits(val as u8)
    }
    #[doc = "STOP Enable"]
    #[inline(always)]
    pub const fn set_stop_en(&mut self, val: super::vals::StopEn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Receiver Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rcv_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Enable"]
    #[inline(always)]
    pub const fn set_rcv_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Transmitter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn xmt_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Enable"]
    #[inline(always)]
    pub const fn set_xmt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Receiver 11 ETU Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rcvr_11(&self) -> super::vals::Rcvr11 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Rcvr11::from_bits(val as u8)
    }
    #[doc = "Receiver 11 ETU Mode Enable"]
    #[inline(always)]
    pub const fn set_rcvr_11(&mut self, val: super::vals::Rcvr11) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Receive DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_dma_en(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Receive DMA Enable"]
    #[inline(always)]
    pub const fn set_rx_dma_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Transmit DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_dma_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit DMA Enable"]
    #[inline(always)]
    pub const fn set_tx_dma_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Invert CRC Output Value Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn inv_crc_val(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Invert CRC Output Value Bits"]
    #[inline(always)]
    pub const fn set_inv_crc_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "CRC Output Value Bit Reversal Or Flip Control"]
    #[must_use]
    #[inline(always)]
    pub const fn crc_out_flip(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Output Value Bit Reversal Or Flip Control"]
    #[inline(always)]
    pub const fn set_crc_out_flip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "CRC Input Byte's Bit Reversal Or Flip Control"]
    #[must_use]
    #[inline(always)]
    pub const fn crc_in_flip(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Input Byte's Bit Reversal Or Flip Control"]
    #[inline(always)]
    pub const fn set_crc_in_flip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "CWT Counter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cwt_en(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "CWT Counter Enable"]
    #[inline(always)]
    pub const fn set_cwt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "LRC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lrc_en(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "LRC Enable"]
    #[inline(always)]
    pub const fn set_lrc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "CRC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn crc_en(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Enable"]
    #[inline(always)]
    pub const fn set_crc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Transmit CRC or LRC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn xmt_crc_lrc(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit CRC or LRC Enable"]
    #[inline(always)]
    pub const fn set_xmt_crc_lrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Block Wait Time Counter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bwt_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Block Wait Time Counter Enable"]
    #[inline(always)]
    pub const fn set_bwt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("ic", &self.ic())
            .field("icm", &self.icm())
            .field("anack", &self.anack())
            .field("onack", &self.onack())
            .field("flsh_rx", &self.flsh_rx())
            .field("flsh_tx", &self.flsh_tx())
            .field("sw_rst", &self.sw_rst())
            .field("kill_clocks", &self.kill_clocks())
            .field("doze_en", &self.doze_en())
            .field("stop_en", &self.stop_en())
            .field("rcv_en", &self.rcv_en())
            .field("xmt_en", &self.xmt_en())
            .field("rcvr_11", &self.rcvr_11())
            .field("rx_dma_en", &self.rx_dma_en())
            .field("tx_dma_en", &self.tx_dma_en())
            .field("inv_crc_val", &self.inv_crc_val())
            .field("crc_out_flip", &self.crc_out_flip())
            .field("crc_in_flip", &self.crc_in_flip())
            .field("cwt_en", &self.cwt_en())
            .field("lrc_en", &self.lrc_en())
            .field("crc_en", &self.crc_en())
            .field("xmt_crc_lrc", &self.xmt_crc_lrc())
            .field("bwt_en", &self.bwt_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ ic: {:?}, icm: {=bool:?}, anack: {=bool:?}, onack: {=bool:?}, flsh_rx: {:?}, flsh_tx: {:?}, sw_rst: {:?}, kill_clocks: {:?}, doze_en: {:?}, stop_en: {:?}, rcv_en: {=bool:?}, xmt_en: {=bool:?}, rcvr_11: {:?}, rx_dma_en: {=bool:?}, tx_dma_en: {=bool:?}, inv_crc_val: {=bool:?}, crc_out_flip: {=bool:?}, crc_in_flip: {=bool:?}, cwt_en: {=bool:?}, lrc_en: {=bool:?}, crc_en: {=bool:?}, xmt_crc_lrc: {=bool:?}, bwt_en: {=bool:?} }}",
            self.ic(),
            self.icm(),
            self.anack(),
            self.onack(),
            self.flsh_rx(),
            self.flsh_tx(),
            self.sw_rst(),
            self.kill_clocks(),
            self.doze_en(),
            self.stop_en(),
            self.rcv_en(),
            self.xmt_en(),
            self.rcvr_11(),
            self.rx_dma_en(),
            self.tx_dma_en(),
            self.inv_crc_val(),
            self.crc_out_flip(),
            self.crc_in_flip(),
            self.cwt_en(),
            self.lrc_en(),
            self.crc_en(),
            self.xmt_crc_lrc(),
            self.bwt_en()
        )
    }
}
#[doc = "Character Wait Time Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CwtVal(pub u32);
impl CwtVal {
    #[doc = "Character Wait Time Value"]
    #[must_use]
    #[inline(always)]
    pub const fn cwt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Character Wait Time Value"]
    #[inline(always)]
    pub const fn set_cwt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for CwtVal {
    #[inline(always)]
    fn default() -> CwtVal {
        CwtVal(0)
    }
}
impl core::fmt::Debug for CwtVal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CwtVal").field("cwt", &self.cwt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CwtVal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CwtVal {{ cwt: {=u16:?} }}", self.cwt())
    }
}
#[doc = "Baud Rate Divisor"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Divisor(pub u32);
impl Divisor {
    #[doc = "Divisor (F/D) Value"]
    #[must_use]
    #[inline(always)]
    pub const fn divisor_value(&self) -> super::vals::DivisorValue {
        let val = (self.0 >> 0usize) & 0x01ff;
        super::vals::DivisorValue::from_bits(val as u16)
    }
    #[doc = "Divisor (F/D) Value"]
    #[inline(always)]
    pub const fn set_divisor_value(&mut self, val: super::vals::DivisorValue) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val.to_bits() as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Divisor {
    #[inline(always)]
    fn default() -> Divisor {
        Divisor(0)
    }
}
impl core::fmt::Debug for Divisor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Divisor")
            .field("divisor_value", &self.divisor_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Divisor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Divisor {{ divisor_value: {:?} }}", self.divisor_value())
    }
}
#[doc = "General Purpose Counter 0 Timeout Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpcnt0Val(pub u32);
impl Gpcnt0Val {
    #[doc = "General Purpose Counter 0 Timeout Value"]
    #[must_use]
    #[inline(always)]
    pub const fn gpcnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "General Purpose Counter 0 Timeout Value"]
    #[inline(always)]
    pub const fn set_gpcnt0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Gpcnt0Val {
    #[inline(always)]
    fn default() -> Gpcnt0Val {
        Gpcnt0Val(0)
    }
}
impl core::fmt::Debug for Gpcnt0Val {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpcnt0Val")
            .field("gpcnt0", &self.gpcnt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpcnt0Val {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpcnt0Val {{ gpcnt0: {=u16:?} }}", self.gpcnt0())
    }
}
#[doc = "General Purpose Counter 1 Timeout Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpcnt1Val(pub u32);
impl Gpcnt1Val {
    #[doc = "General Purpose Counter 1 Timeout Value"]
    #[must_use]
    #[inline(always)]
    pub const fn gpcnt1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "General Purpose Counter 1 Timeout Value"]
    #[inline(always)]
    pub const fn set_gpcnt1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Gpcnt1Val {
    #[inline(always)]
    fn default() -> Gpcnt1Val {
        Gpcnt1Val(0)
    }
}
impl core::fmt::Debug for Gpcnt1Val {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpcnt1Val")
            .field("gpcnt1", &self.gpcnt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpcnt1Val {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpcnt1Val {{ gpcnt1: {=u16:?} }}", self.gpcnt1())
    }
}
#[doc = "Interrupt Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntMask(pub u32);
impl IntMask {
    #[doc = "Receive Data Threshold Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rdt_im(&self) -> super::vals::RdtIm {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RdtIm::from_bits(val as u8)
    }
    #[doc = "Receive Data Threshold Interrupt Mask"]
    #[inline(always)]
    pub const fn set_rdt_im(&mut self, val: super::vals::RdtIm) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Complete Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn tc_im(&self) -> super::vals::TcIm {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::TcIm::from_bits(val as u8)
    }
    #[doc = "Transmit Complete Interrupt Mask"]
    #[inline(always)]
    pub const fn set_tc_im(&mut self, val: super::vals::TcIm) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive FIFO Overflow Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rfo_im(&self) -> super::vals::RfoIm {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RfoIm::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Overflow Interrupt Mask"]
    #[inline(always)]
    pub const fn set_rfo_im(&mut self, val: super::vals::RfoIm) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Early Transmit Complete Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn etc_im(&self) -> super::vals::EtcIm {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::EtcIm::from_bits(val as u8)
    }
    #[doc = "Early Transmit Complete Interrupt Mask"]
    #[inline(always)]
    pub const fn set_etc_im(&mut self, val: super::vals::EtcIm) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit FIFO Empty Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn tfe_im(&self) -> super::vals::TfeIm {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::TfeIm::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub const fn set_tfe_im(&mut self, val: super::vals::TfeIm) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit NACK Threshold Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn tnack_im(&self) -> super::vals::TnackIm {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::TnackIm::from_bits(val as u8)
    }
    #[doc = "Transmit NACK Threshold Interrupt Mask"]
    #[inline(always)]
    pub const fn set_tnack_im(&mut self, val: super::vals::TnackIm) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmit FIFO Full Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn tff_im(&self) -> super::vals::TffIm {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::TffIm::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub const fn set_tff_im(&mut self, val: super::vals::TffIm) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit Data Threshold Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn tdt_im(&self) -> super::vals::TdtIm {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::TdtIm::from_bits(val as u8)
    }
    #[doc = "Transmit Data Threshold Interrupt Mask"]
    #[inline(always)]
    pub const fn set_tdt_im(&mut self, val: super::vals::TdtIm) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "General Purpose Timer 0 Timeout Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn gpcnt0_im(&self) -> super::vals::Gpcnt0Im {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Gpcnt0Im::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer 0 Timeout Interrupt Mask"]
    #[inline(always)]
    pub const fn set_gpcnt0_im(&mut self, val: super::vals::Gpcnt0Im) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Character Wait Time Error Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn cwt_err_im(&self) -> super::vals::CwtErrIm {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::CwtErrIm::from_bits(val as u8)
    }
    #[doc = "Character Wait Time Error Interrupt Mask"]
    #[inline(always)]
    pub const fn set_cwt_err_im(&mut self, val: super::vals::CwtErrIm) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Receiver NACK Threshold Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rnack_im(&self) -> super::vals::RnackIm {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::RnackIm::from_bits(val as u8)
    }
    #[doc = "Receiver NACK Threshold Interrupt Mask"]
    #[inline(always)]
    pub const fn set_rnack_im(&mut self, val: super::vals::RnackIm) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Block Wait Time Error Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn bwt_err_im(&self) -> super::vals::BwtErrIm {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::BwtErrIm::from_bits(val as u8)
    }
    #[doc = "Block Wait Time Error Interrupt Mask"]
    #[inline(always)]
    pub const fn set_bwt_err_im(&mut self, val: super::vals::BwtErrIm) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Block Guard Time Error Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn bgt_err_im(&self) -> super::vals::BgtErrIm {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::BgtErrIm::from_bits(val as u8)
    }
    #[doc = "Block Guard Time Error Interrupt"]
    #[inline(always)]
    pub const fn set_bgt_err_im(&mut self, val: super::vals::BgtErrIm) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "General Purpose Counter 1 Timeout Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn gpcnt1_im(&self) -> super::vals::Gpcnt1Im {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Gpcnt1Im::from_bits(val as u8)
    }
    #[doc = "General Purpose Counter 1 Timeout Interrupt Mask"]
    #[inline(always)]
    pub const fn set_gpcnt1_im(&mut self, val: super::vals::Gpcnt1Im) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Receive Data Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_data_im(&self) -> super::vals::RxDataIm {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::RxDataIm::from_bits(val as u8)
    }
    #[doc = "Receive Data Interrupt Mask"]
    #[inline(always)]
    pub const fn set_rx_data_im(&mut self, val: super::vals::RxDataIm) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Parity Error Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn pef_im(&self) -> super::vals::PefIm {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::PefIm::from_bits(val as u8)
    }
    #[doc = "Parity Error Interrupt Mask"]
    #[inline(always)]
    pub const fn set_pef_im(&mut self, val: super::vals::PefIm) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for IntMask {
    #[inline(always)]
    fn default() -> IntMask {
        IntMask(0)
    }
}
impl core::fmt::Debug for IntMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntMask")
            .field("rdt_im", &self.rdt_im())
            .field("tc_im", &self.tc_im())
            .field("rfo_im", &self.rfo_im())
            .field("etc_im", &self.etc_im())
            .field("tfe_im", &self.tfe_im())
            .field("tnack_im", &self.tnack_im())
            .field("tff_im", &self.tff_im())
            .field("tdt_im", &self.tdt_im())
            .field("gpcnt0_im", &self.gpcnt0_im())
            .field("cwt_err_im", &self.cwt_err_im())
            .field("rnack_im", &self.rnack_im())
            .field("bwt_err_im", &self.bwt_err_im())
            .field("bgt_err_im", &self.bgt_err_im())
            .field("gpcnt1_im", &self.gpcnt1_im())
            .field("rx_data_im", &self.rx_data_im())
            .field("pef_im", &self.pef_im())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntMask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntMask {{ rdt_im: {:?}, tc_im: {:?}, rfo_im: {:?}, etc_im: {:?}, tfe_im: {:?}, tnack_im: {:?}, tff_im: {:?}, tdt_im: {:?}, gpcnt0_im: {:?}, cwt_err_im: {:?}, rnack_im: {:?}, bwt_err_im: {:?}, bgt_err_im: {:?}, gpcnt1_im: {:?}, rx_data_im: {:?}, pef_im: {:?} }}",
            self.rdt_im(),
            self.tc_im(),
            self.rfo_im(),
            self.etc_im(),
            self.tfe_im(),
            self.tnack_im(),
            self.tff_im(),
            self.tdt_im(),
            self.gpcnt0_im(),
            self.cwt_err_im(),
            self.rnack_im(),
            self.bwt_err_im(),
            self.bgt_err_im(),
            self.gpcnt1_im(),
            self.rx_data_im(),
            self.pef_im()
        )
    }
}
#[doc = "Parameters"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Receive FIFO Depth"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_fifo_depth(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive FIFO Depth"]
    #[inline(always)]
    pub const fn set_rx_fifo_depth(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Transmit FIFO Depth"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_fifo_depth(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit FIFO Depth"]
    #[inline(always)]
    pub const fn set_tx_fifo_depth(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
impl core::fmt::Debug for Param {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Param")
            .field("rx_fifo_depth", &self.rx_fifo_depth())
            .field("tx_fifo_depth", &self.tx_fifo_depth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ rx_fifo_depth: {=u8:?}, tx_fifo_depth: {=u8:?} }}",
            self.rx_fifo_depth(),
            self.tx_fifo_depth()
        )
    }
}
#[doc = "Port Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcsr(pub u32);
impl Pcsr {
    #[doc = "Auto Power Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sapd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Power Down Enable"]
    #[inline(always)]
    pub const fn set_sapd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Vcc Enable for Smart Card"]
    #[must_use]
    #[inline(always)]
    pub const fn svcc_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Vcc Enable for Smart Card"]
    #[inline(always)]
    pub const fn set_svcc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "VCC Enable Polarity Control"]
    #[must_use]
    #[inline(always)]
    pub const fn vccenp(&self) -> super::vals::Vccenp {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Vccenp::from_bits(val as u8)
    }
    #[doc = "VCC Enable Polarity Control"]
    #[inline(always)]
    pub const fn set_vccenp(&mut self, val: super::vals::Vccenp) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Reset Smart Card"]
    #[must_use]
    #[inline(always)]
    pub const fn srst(&self) -> super::vals::Srst {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Srst::from_bits(val as u8)
    }
    #[doc = "Reset Smart Card"]
    #[inline(always)]
    pub const fn set_srst(&mut self, val: super::vals::Srst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Clock Enable for Smart Card"]
    #[must_use]
    #[inline(always)]
    pub const fn scen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Enable for Smart Card"]
    #[inline(always)]
    pub const fn set_scen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Smart Card Clock Stop Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn scsp(&self) -> super::vals::Scsp {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Scsp::from_bits(val as u8)
    }
    #[doc = "Smart Card Clock Stop Polarity"]
    #[inline(always)]
    pub const fn set_scsp(&mut self, val: super::vals::Scsp) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Auto Power-Down Control"]
    #[must_use]
    #[inline(always)]
    pub const fn spd(&self) -> super::vals::Spd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Spd::from_bits(val as u8)
    }
    #[doc = "Auto Power-Down Control"]
    #[inline(always)]
    pub const fn set_spd(&mut self, val: super::vals::Spd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Smart Card Presence Detect Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn spdim(&self) -> super::vals::Spdim {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Spdim::from_bits(val as u8)
    }
    #[doc = "Smart Card Presence Detect Interrupt Mask"]
    #[inline(always)]
    pub const fn set_spdim(&mut self, val: super::vals::Spdim) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Smart Card Presence Detect Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn spdif(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Smart Card Presence Detect Interrupt Flag"]
    #[inline(always)]
    pub const fn set_spdif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Smart Card Presence Detect Pin Status"]
    #[must_use]
    #[inline(always)]
    pub const fn spdp(&self) -> super::vals::Spdp {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Spdp::from_bits(val as u8)
    }
    #[doc = "Smart Card Presence Detect Pin Status"]
    #[inline(always)]
    pub const fn set_spdp(&mut self, val: super::vals::Spdp) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SIM Presence Detect Edge Select"]
    #[must_use]
    #[inline(always)]
    pub const fn spdes(&self) -> super::vals::Spdes {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Spdes::from_bits(val as u8)
    }
    #[doc = "SIM Presence Detect Edge Select"]
    #[inline(always)]
    pub const fn set_spdes(&mut self, val: super::vals::Spdes) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Pcsr {
    #[inline(always)]
    fn default() -> Pcsr {
        Pcsr(0)
    }
}
impl core::fmt::Debug for Pcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcsr")
            .field("sapd", &self.sapd())
            .field("svcc_en", &self.svcc_en())
            .field("vccenp", &self.vccenp())
            .field("srst", &self.srst())
            .field("scen", &self.scen())
            .field("scsp", &self.scsp())
            .field("spd", &self.spd())
            .field("spdim", &self.spdim())
            .field("spdif", &self.spdif())
            .field("spdp", &self.spdp())
            .field("spdes", &self.spdes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcsr {{ sapd: {=bool:?}, svcc_en: {=bool:?}, vccenp: {:?}, srst: {:?}, scen: {=bool:?}, scsp: {:?}, spd: {:?}, spdim: {:?}, spdif: {=bool:?}, spdp: {:?}, spdes: {:?} }}",
            self.sapd(),
            self.svcc_en(),
            self.vccenp(),
            self.srst(),
            self.scen(),
            self.scsp(),
            self.spd(),
            self.spdim(),
            self.spdif(),
            self.spdp(),
            self.spdes()
        )
    }
}
#[doc = "Receive Data Read Buffer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxBuf(pub u32);
impl RxBuf {
    #[doc = "Receive Data Byte Read"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_byte(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Data Byte Read"]
    #[inline(always)]
    pub const fn set_rx_byte(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for RxBuf {
    #[inline(always)]
    fn default() -> RxBuf {
        RxBuf(0)
    }
}
impl core::fmt::Debug for RxBuf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxBuf")
            .field("rx_byte", &self.rx_byte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxBuf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RxBuf {{ rx_byte: {=u8:?} }}", self.rx_byte())
    }
}
#[doc = "Receive Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxStatus(pub u32);
impl RxStatus {
    #[doc = "Receive FIFO Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rfo(&self) -> super::vals::Rfo {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rfo::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Overflow Flag"]
    #[inline(always)]
    pub const fn set_rfo(&mut self, val: super::vals::Rfo) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_data(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Interrupt Flag"]
    #[inline(always)]
    pub const fn set_rx_data(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receive Data Threshold Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rdtf(&self) -> super::vals::Rdtf {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Rdtf::from_bits(val as u8)
    }
    #[doc = "Receive Data Threshold Interrupt Flag"]
    #[inline(always)]
    pub const fn set_rdtf(&mut self, val: super::vals::Rdtf) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "LRC Check OK Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn lrc_ok(&self) -> super::vals::LrcOk {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::LrcOk::from_bits(val as u8)
    }
    #[doc = "LRC Check OK Flag"]
    #[inline(always)]
    pub const fn set_lrc_ok(&mut self, val: super::vals::LrcOk) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "CRC Check OK Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn crc_ok(&self) -> super::vals::CrcOk {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CrcOk::from_bits(val as u8)
    }
    #[doc = "CRC Check OK Flag"]
    #[inline(always)]
    pub const fn set_crc_ok(&mut self, val: super::vals::CrcOk) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Character Wait Time Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cwt_err(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Character Wait Time Error Flag"]
    #[inline(always)]
    pub const fn set_cwt_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Received NACK Threshold Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rte(&self) -> super::vals::Rte {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Rte::from_bits(val as u8)
    }
    #[doc = "Received NACK Threshold Error Flag"]
    #[inline(always)]
    pub const fn set_rte(&mut self, val: super::vals::Rte) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Block Wait Time Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bwt_err(&self) -> super::vals::BwtErr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::BwtErr::from_bits(val as u8)
    }
    #[doc = "Block Wait Time Error Flag"]
    #[inline(always)]
    pub const fn set_bwt_err(&mut self, val: super::vals::BwtErr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Block Guard Time Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bgt_err(&self) -> super::vals::BgtErr {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::BgtErr::from_bits(val as u8)
    }
    #[doc = "Block Guard Time Error Flag"]
    #[inline(always)]
    pub const fn set_bgt_err(&mut self, val: super::vals::BgtErr) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Parity Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn pef(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub const fn set_pef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Frame Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fef(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Error Flag"]
    #[inline(always)]
    pub const fn set_fef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Receive FIFO Write Pointer Value"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_wptr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Receive FIFO Write Pointer Value"]
    #[inline(always)]
    pub const fn set_rx_wptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Receive FIFO Byte Count"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_cnt(&self) -> super::vals::RxCnt {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::RxCnt::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Byte Count"]
    #[inline(always)]
    pub const fn set_rx_cnt(&mut self, val: super::vals::RxCnt) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for RxStatus {
    #[inline(always)]
    fn default() -> RxStatus {
        RxStatus(0)
    }
}
impl core::fmt::Debug for RxStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxStatus")
            .field("rfo", &self.rfo())
            .field("rx_data", &self.rx_data())
            .field("rdtf", &self.rdtf())
            .field("lrc_ok", &self.lrc_ok())
            .field("crc_ok", &self.crc_ok())
            .field("cwt_err", &self.cwt_err())
            .field("rte", &self.rte())
            .field("bwt_err", &self.bwt_err())
            .field("bgt_err", &self.bgt_err())
            .field("pef", &self.pef())
            .field("fef", &self.fef())
            .field("rx_wptr", &self.rx_wptr())
            .field("rx_cnt", &self.rx_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxStatus {{ rfo: {:?}, rx_data: {=bool:?}, rdtf: {:?}, lrc_ok: {:?}, crc_ok: {:?}, cwt_err: {=bool:?}, rte: {:?}, bwt_err: {:?}, bgt_err: {:?}, pef: {=bool:?}, fef: {=bool:?}, rx_wptr: {=u8:?}, rx_cnt: {:?} }}",
            self.rfo(),
            self.rx_data(),
            self.rdtf(),
            self.lrc_ok(),
            self.crc_ok(),
            self.cwt_err(),
            self.rte(),
            self.bwt_err(),
            self.bgt_err(),
            self.pef(),
            self.fef(),
            self.rx_wptr(),
            self.rx_cnt()
        )
    }
}
#[doc = "Receiver Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxThd(pub u32);
impl RxThd {
    #[doc = "Receiver Data Threshold Value"]
    #[must_use]
    #[inline(always)]
    pub const fn rdt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Receiver Data Threshold Value"]
    #[inline(always)]
    pub const fn set_rdt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Receiver NACK Threshold Value"]
    #[must_use]
    #[inline(always)]
    pub const fn rnck_thd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Receiver NACK Threshold Value"]
    #[inline(always)]
    pub const fn set_rnck_thd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for RxThd {
    #[inline(always)]
    fn default() -> RxThd {
        RxThd(0)
    }
}
impl core::fmt::Debug for RxThd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxThd")
            .field("rdt", &self.rdt())
            .field("rnck_thd", &self.rnck_thd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxThd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxThd {{ rdt: {=u8:?}, rnck_thd: {=u8:?} }}",
            self.rdt(),
            self.rnck_thd()
        )
    }
}
#[doc = "Transmit Data Buffer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxBuf(pub u32);
impl TxBuf {
    #[doc = "Transmit Data Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_byte(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Data Byte"]
    #[inline(always)]
    pub const fn set_tx_byte(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for TxBuf {
    #[inline(always)]
    fn default() -> TxBuf {
        TxBuf(0)
    }
}
impl core::fmt::Debug for TxBuf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxBuf")
            .field("tx_byte", &self.tx_byte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxBuf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TxBuf {{ tx_byte: {=u8:?} }}", self.tx_byte())
    }
}
#[doc = "Transmitter Guard ETU Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxGetu(pub u32);
impl TxGetu {
    #[doc = "Transmitter Guard Time Value in ETU"]
    #[must_use]
    #[inline(always)]
    pub const fn getu(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmitter Guard Time Value in ETU"]
    #[inline(always)]
    pub const fn set_getu(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for TxGetu {
    #[inline(always)]
    fn default() -> TxGetu {
        TxGetu(0)
    }
}
impl core::fmt::Debug for TxGetu {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxGetu")
            .field("getu", &self.getu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxGetu {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TxGetu {{ getu: {=u8:?} }}", self.getu())
    }
}
#[doc = "Transmitter Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxStatus(pub u32);
impl TxStatus {
    #[doc = "Transmit NACK Threshold Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tnte(&self) -> super::vals::Tnte {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tnte::from_bits(val as u8)
    }
    #[doc = "Transmit NACK Threshold Error Flag"]
    #[inline(always)]
    pub const fn set_tnte(&mut self, val: super::vals::Tnte) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO Empty Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tfe(&self) -> super::vals::Tfe {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Tfe::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Empty Flag"]
    #[inline(always)]
    pub const fn set_tfe(&mut self, val: super::vals::Tfe) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Early Transmit Complete Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn etcf(&self) -> super::vals::Etcf {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Etcf::from_bits(val as u8)
    }
    #[doc = "Early Transmit Complete Flag"]
    #[inline(always)]
    pub const fn set_etcf(&mut self, val: super::vals::Etcf) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit Complete Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf(&self) -> super::vals::Tcf {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Tcf::from_bits(val as u8)
    }
    #[doc = "Transmit Complete Flag"]
    #[inline(always)]
    pub const fn set_tcf(&mut self, val: super::vals::Tcf) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmit FIFO Full Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tff(&self) -> super::vals::Tff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Tff::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Full Flag"]
    #[inline(always)]
    pub const fn set_tff(&mut self, val: super::vals::Tff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit Data Threshold Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tdtf(&self) -> super::vals::Tdtf {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Tdtf::from_bits(val as u8)
    }
    #[doc = "Transmit Data Threshold Flag"]
    #[inline(always)]
    pub const fn set_tdtf(&mut self, val: super::vals::Tdtf) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "General Purpose Counter 0 Timeout Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn gpcnt0_to(&self) -> super::vals::Gpcnt0To {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Gpcnt0To::from_bits(val as u8)
    }
    #[doc = "General Purpose Counter 0 Timeout Flag"]
    #[inline(always)]
    pub const fn set_gpcnt0_to(&mut self, val: super::vals::Gpcnt0To) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "General Purpose Counter 1 Timeout Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn gpcnt1_to(&self) -> super::vals::Gpcnt1To {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Gpcnt1To::from_bits(val as u8)
    }
    #[doc = "General Purpose Counter 1 Timeout Flag"]
    #[inline(always)]
    pub const fn set_gpcnt1_to(&mut self, val: super::vals::Gpcnt1To) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmit FIFO Read Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_rptr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmit FIFO Read Pointer"]
    #[inline(always)]
    pub const fn set_tx_rptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Transmit FIFO Byte Count"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cnt(&self) -> super::vals::TxCnt {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::TxCnt::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Byte Count"]
    #[inline(always)]
    pub const fn set_tx_cnt(&mut self, val: super::vals::TxCnt) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for TxStatus {
    #[inline(always)]
    fn default() -> TxStatus {
        TxStatus(0)
    }
}
impl core::fmt::Debug for TxStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxStatus")
            .field("tnte", &self.tnte())
            .field("tfe", &self.tfe())
            .field("etcf", &self.etcf())
            .field("tcf", &self.tcf())
            .field("tff", &self.tff())
            .field("tdtf", &self.tdtf())
            .field("gpcnt0_to", &self.gpcnt0_to())
            .field("gpcnt1_to", &self.gpcnt1_to())
            .field("tx_rptr", &self.tx_rptr())
            .field("tx_cnt", &self.tx_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxStatus {{ tnte: {:?}, tfe: {:?}, etcf: {:?}, tcf: {:?}, tff: {:?}, tdtf: {:?}, gpcnt0_to: {:?}, gpcnt1_to: {:?}, tx_rptr: {=u8:?}, tx_cnt: {:?} }}",
            self.tnte(),
            self.tfe(),
            self.etcf(),
            self.tcf(),
            self.tff(),
            self.tdtf(),
            self.gpcnt0_to(),
            self.gpcnt1_to(),
            self.tx_rptr(),
            self.tx_cnt()
        )
    }
}
#[doc = "Transmitter Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxThd(pub u32);
impl TxThd {
    #[doc = "Transmitter Data Threshold Value"]
    #[must_use]
    #[inline(always)]
    pub const fn tdt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmitter Data Threshold Value"]
    #[inline(always)]
    pub const fn set_tdt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Transmitter NACK Threshold Value"]
    #[must_use]
    #[inline(always)]
    pub const fn tnck_thd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmitter NACK Threshold Value"]
    #[inline(always)]
    pub const fn set_tnck_thd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for TxThd {
    #[inline(always)]
    fn default() -> TxThd {
        TxThd(0)
    }
}
impl core::fmt::Debug for TxThd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxThd")
            .field("tdt", &self.tdt())
            .field("tnck_thd", &self.tnck_thd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxThd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxThd {{ tdt: {=u8:?}, tnck_thd: {=u8:?} }}",
            self.tdt(),
            self.tnck_thd()
        )
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VerId(pub u32);
impl VerId {
    #[doc = "Version ID"]
    #[must_use]
    #[inline(always)]
    pub const fn ver(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Version ID"]
    #[inline(always)]
    pub const fn set_ver(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for VerId {
    #[inline(always)]
    fn default() -> VerId {
        VerId(0)
    }
}
impl core::fmt::Debug for VerId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VerId").field("ver", &self.ver()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VerId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "VerId {{ ver: {=u32:?} }}", self.ver())
    }
}
