#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "EMVSIM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emvsim {
    ptr: *mut u8,
}
unsafe impl Send for Emvsim {}
unsafe impl Sync for Emvsim {}
impl Emvsim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID."]
    #[inline(always)]
    pub const fn ver_id(self) -> crate::pac::common::Reg<VerId, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameters."]
    #[inline(always)]
    pub const fn param(self) -> crate::pac::common::Reg<Param, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Clock Configuration."]
    #[inline(always)]
    pub const fn clkcfg(self) -> crate::pac::common::Reg<Clkcfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Baud Rate Divisor."]
    #[inline(always)]
    pub const fn divisor(self) -> crate::pac::common::Reg<Divisor, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::pac::common::Reg<Ctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt Mask."]
    #[inline(always)]
    pub const fn int_mask(self) -> crate::pac::common::Reg<IntMask, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Receiver Threshold."]
    #[inline(always)]
    pub const fn rx_thd(self) -> crate::pac::common::Reg<RxThd, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Transmitter Threshold."]
    #[inline(always)]
    pub const fn tx_thd(self) -> crate::pac::common::Reg<TxThd, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Receive Status."]
    #[inline(always)]
    pub const fn rx_status(self) -> crate::pac::common::Reg<RxStatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Transmitter Status."]
    #[inline(always)]
    pub const fn tx_status(self) -> crate::pac::common::Reg<TxStatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Port Control and Status."]
    #[inline(always)]
    pub const fn pcsr(self) -> crate::pac::common::Reg<Pcsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Receive Data Read Buffer."]
    #[inline(always)]
    pub const fn rx_buf(self) -> crate::pac::common::Reg<RxBuf, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Transmit Data Buffer."]
    #[inline(always)]
    pub const fn tx_buf(self) -> crate::pac::common::Reg<TxBuf, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Transmitter Guard ETU Value."]
    #[inline(always)]
    pub const fn tx_getu(self) -> crate::pac::common::Reg<TxGetu, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Character Wait Time Value."]
    #[inline(always)]
    pub const fn cwt_val(self) -> crate::pac::common::Reg<CwtVal, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Block Wait Time Value."]
    #[inline(always)]
    pub const fn bwt_val(self) -> crate::pac::common::Reg<BwtVal, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Block Guard Time Value."]
    #[inline(always)]
    pub const fn bgt_val(self) -> crate::pac::common::Reg<BgtVal, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "General Purpose Counter 0 Timeout Value."]
    #[inline(always)]
    pub const fn gpcnt0_val(self) -> crate::pac::common::Reg<Gpcnt0Val, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "General Purpose Counter 1 Timeout Value."]
    #[inline(always)]
    pub const fn gpcnt1_val(self) -> crate::pac::common::Reg<Gpcnt1Val, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
}
#[doc = "Block Guard Time Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgtVal(pub u32);
impl BgtVal {
    #[doc = "Block Guard Time Value."]
    #[must_use]
    #[inline(always)]
    pub const fn bgt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Block Guard Time Value."]
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
#[doc = "Block Wait Time Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BwtVal(pub u32);
impl BwtVal {
    #[doc = "Block Wait Time Value."]
    #[must_use]
    #[inline(always)]
    pub const fn bwt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Block Wait Time Value."]
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
#[doc = "Clock Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkcfg(pub u32);
impl Clkcfg {
    #[doc = "Clock Prescaler Value."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_prsc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Prescaler Value."]
    #[inline(always)]
    pub const fn set_clk_prsc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "General Purpose Counter 1 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn gpcnt1_clk_sel(&self) -> Gpcnt1ClkSel {
        let val = (self.0 >> 8usize) & 0x03;
        Gpcnt1ClkSel::from_bits(val as u8)
    }
    #[doc = "General Purpose Counter 1 Clock Select."]
    #[inline(always)]
    pub const fn set_gpcnt1_clk_sel(&mut self, val: Gpcnt1ClkSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "General Purpose Counter 0 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn gpcnt0_clk_sel(&self) -> Gpcnt0ClkSel {
        let val = (self.0 >> 10usize) & 0x03;
        Gpcnt0ClkSel::from_bits(val as u8)
    }
    #[doc = "General Purpose Counter 0 Clock Select."]
    #[inline(always)]
    pub const fn set_gpcnt0_clk_sel(&mut self, val: Gpcnt0ClkSel) {
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
#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Inverse Convention."]
    #[must_use]
    #[inline(always)]
    pub const fn ic(&self) -> Ic {
        let val = (self.0 >> 0usize) & 0x01;
        Ic::from_bits(val as u8)
    }
    #[doc = "Inverse Convention."]
    #[inline(always)]
    pub const fn set_ic(&mut self, val: Ic) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Initial Character Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn icm(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Initial Character Mode."]
    #[inline(always)]
    pub const fn set_icm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Auto NACK Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn anack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Auto NACK Enable."]
    #[inline(always)]
    pub const fn set_anack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Overrun NACK Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn onack(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun NACK Enable."]
    #[inline(always)]
    pub const fn set_onack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Flush Receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn flsh_rx(&self) -> FlshRx {
        let val = (self.0 >> 8usize) & 0x01;
        FlshRx::from_bits(val as u8)
    }
    #[doc = "Flush Receiver."]
    #[inline(always)]
    pub const fn set_flsh_rx(&mut self, val: FlshRx) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Flush Transmitter."]
    #[must_use]
    #[inline(always)]
    pub const fn flsh_tx(&self) -> FlshTx {
        let val = (self.0 >> 9usize) & 0x01;
        FlshTx::from_bits(val as u8)
    }
    #[doc = "Flush Transmitter."]
    #[inline(always)]
    pub const fn set_flsh_tx(&mut self, val: FlshTx) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_rst(&self) -> SwRst {
        let val = (self.0 >> 10usize) & 0x01;
        SwRst::from_bits(val as u8)
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_sw_rst(&mut self, val: SwRst) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Kill Internal Clocks."]
    #[must_use]
    #[inline(always)]
    pub const fn kill_clocks(&self) -> KillClocks {
        let val = (self.0 >> 11usize) & 0x01;
        KillClocks::from_bits(val as u8)
    }
    #[doc = "Kill Internal Clocks."]
    #[inline(always)]
    pub const fn set_kill_clocks(&mut self, val: KillClocks) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Doze Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn doze_en(&self) -> DozeEn {
        let val = (self.0 >> 12usize) & 0x01;
        DozeEn::from_bits(val as u8)
    }
    #[doc = "Doze Enable."]
    #[inline(always)]
    pub const fn set_doze_en(&mut self, val: DozeEn) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "STOP Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn stop_en(&self) -> StopEn {
        let val = (self.0 >> 13usize) & 0x01;
        StopEn::from_bits(val as u8)
    }
    #[doc = "STOP Enable."]
    #[inline(always)]
    pub const fn set_stop_en(&mut self, val: StopEn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Receiver Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rcv_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Enable."]
    #[inline(always)]
    pub const fn set_rcv_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Transmitter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn xmt_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Enable."]
    #[inline(always)]
    pub const fn set_xmt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Receiver 11 ETU Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rcvr_11(&self) -> Rcvr11 {
        let val = (self.0 >> 18usize) & 0x01;
        Rcvr11::from_bits(val as u8)
    }
    #[doc = "Receiver 11 ETU Mode Enable."]
    #[inline(always)]
    pub const fn set_rcvr_11(&mut self, val: Rcvr11) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Receive DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rx_dma_en(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Receive DMA Enable."]
    #[inline(always)]
    pub const fn set_rx_dma_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Transmit DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_dma_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit DMA Enable."]
    #[inline(always)]
    pub const fn set_tx_dma_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Invert CRC Output Value Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn inv_crc_val(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Invert CRC Output Value Bits."]
    #[inline(always)]
    pub const fn set_inv_crc_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "CRC Output Value Bit Reversal Or Flip Control."]
    #[must_use]
    #[inline(always)]
    pub const fn crc_out_flip(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Output Value Bit Reversal Or Flip Control."]
    #[inline(always)]
    pub const fn set_crc_out_flip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "CRC Input Byte's Bit Reversal Or Flip Control."]
    #[must_use]
    #[inline(always)]
    pub const fn crc_in_flip(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Input Byte's Bit Reversal Or Flip Control."]
    #[inline(always)]
    pub const fn set_crc_in_flip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "CWT Counter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cwt_en(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "CWT Counter Enable."]
    #[inline(always)]
    pub const fn set_cwt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "LRC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lrc_en(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "LRC Enable."]
    #[inline(always)]
    pub const fn set_lrc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "CRC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn crc_en(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Enable."]
    #[inline(always)]
    pub const fn set_crc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Transmit CRC or LRC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn xmt_crc_lrc(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit CRC or LRC Enable."]
    #[inline(always)]
    pub const fn set_xmt_crc_lrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Block Wait Time Counter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bwt_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Block Wait Time Counter Enable."]
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
#[doc = "Character Wait Time Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CwtVal(pub u32);
impl CwtVal {
    #[doc = "Character Wait Time Value."]
    #[must_use]
    #[inline(always)]
    pub const fn cwt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Character Wait Time Value."]
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
#[doc = "Baud Rate Divisor."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Divisor(pub u32);
impl Divisor {
    #[doc = "Divisor (F/D) Value."]
    #[must_use]
    #[inline(always)]
    pub const fn divisor_value(&self) -> DivisorValue {
        let val = (self.0 >> 0usize) & 0x01ff;
        DivisorValue::from_bits(val as u16)
    }
    #[doc = "Divisor (F/D) Value."]
    #[inline(always)]
    pub const fn set_divisor_value(&mut self, val: DivisorValue) {
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
#[doc = "General Purpose Counter 0 Timeout Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpcnt0Val(pub u32);
impl Gpcnt0Val {
    #[doc = "General Purpose Counter 0 Timeout Value."]
    #[must_use]
    #[inline(always)]
    pub const fn gpcnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "General Purpose Counter 0 Timeout Value."]
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
#[doc = "General Purpose Counter 1 Timeout Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpcnt1Val(pub u32);
impl Gpcnt1Val {
    #[doc = "General Purpose Counter 1 Timeout Value."]
    #[must_use]
    #[inline(always)]
    pub const fn gpcnt1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "General Purpose Counter 1 Timeout Value."]
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
#[doc = "Interrupt Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntMask(pub u32);
impl IntMask {
    #[doc = "Receive Data Threshold Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rdt_im(&self) -> RdtIm {
        let val = (self.0 >> 0usize) & 0x01;
        RdtIm::from_bits(val as u8)
    }
    #[doc = "Receive Data Threshold Interrupt Mask."]
    #[inline(always)]
    pub const fn set_rdt_im(&mut self, val: RdtIm) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Complete Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn tc_im(&self) -> TcIm {
        let val = (self.0 >> 1usize) & 0x01;
        TcIm::from_bits(val as u8)
    }
    #[doc = "Transmit Complete Interrupt Mask."]
    #[inline(always)]
    pub const fn set_tc_im(&mut self, val: TcIm) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive FIFO Overflow Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rfo_im(&self) -> RfoIm {
        let val = (self.0 >> 2usize) & 0x01;
        RfoIm::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Overflow Interrupt Mask."]
    #[inline(always)]
    pub const fn set_rfo_im(&mut self, val: RfoIm) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Early Transmit Complete Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn etc_im(&self) -> EtcIm {
        let val = (self.0 >> 3usize) & 0x01;
        EtcIm::from_bits(val as u8)
    }
    #[doc = "Early Transmit Complete Interrupt Mask."]
    #[inline(always)]
    pub const fn set_etc_im(&mut self, val: EtcIm) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit FIFO Empty Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn tfe_im(&self) -> TfeIm {
        let val = (self.0 >> 4usize) & 0x01;
        TfeIm::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Empty Interrupt Mask."]
    #[inline(always)]
    pub const fn set_tfe_im(&mut self, val: TfeIm) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit NACK Threshold Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn tnack_im(&self) -> TnackIm {
        let val = (self.0 >> 5usize) & 0x01;
        TnackIm::from_bits(val as u8)
    }
    #[doc = "Transmit NACK Threshold Interrupt Mask."]
    #[inline(always)]
    pub const fn set_tnack_im(&mut self, val: TnackIm) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmit FIFO Full Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn tff_im(&self) -> TffIm {
        let val = (self.0 >> 6usize) & 0x01;
        TffIm::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Full Interrupt Mask."]
    #[inline(always)]
    pub const fn set_tff_im(&mut self, val: TffIm) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit Data Threshold Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn tdt_im(&self) -> TdtIm {
        let val = (self.0 >> 7usize) & 0x01;
        TdtIm::from_bits(val as u8)
    }
    #[doc = "Transmit Data Threshold Interrupt Mask."]
    #[inline(always)]
    pub const fn set_tdt_im(&mut self, val: TdtIm) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "General Purpose Timer 0 Timeout Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn gpcnt0_im(&self) -> Gpcnt0Im {
        let val = (self.0 >> 8usize) & 0x01;
        Gpcnt0Im::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer 0 Timeout Interrupt Mask."]
    #[inline(always)]
    pub const fn set_gpcnt0_im(&mut self, val: Gpcnt0Im) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Character Wait Time Error Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn cwt_err_im(&self) -> CwtErrIm {
        let val = (self.0 >> 9usize) & 0x01;
        CwtErrIm::from_bits(val as u8)
    }
    #[doc = "Character Wait Time Error Interrupt Mask."]
    #[inline(always)]
    pub const fn set_cwt_err_im(&mut self, val: CwtErrIm) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Receiver NACK Threshold Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rnack_im(&self) -> RnackIm {
        let val = (self.0 >> 10usize) & 0x01;
        RnackIm::from_bits(val as u8)
    }
    #[doc = "Receiver NACK Threshold Interrupt Mask."]
    #[inline(always)]
    pub const fn set_rnack_im(&mut self, val: RnackIm) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Block Wait Time Error Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn bwt_err_im(&self) -> BwtErrIm {
        let val = (self.0 >> 11usize) & 0x01;
        BwtErrIm::from_bits(val as u8)
    }
    #[doc = "Block Wait Time Error Interrupt Mask."]
    #[inline(always)]
    pub const fn set_bwt_err_im(&mut self, val: BwtErrIm) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Block Guard Time Error Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn bgt_err_im(&self) -> BgtErrIm {
        let val = (self.0 >> 12usize) & 0x01;
        BgtErrIm::from_bits(val as u8)
    }
    #[doc = "Block Guard Time Error Interrupt."]
    #[inline(always)]
    pub const fn set_bgt_err_im(&mut self, val: BgtErrIm) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "General Purpose Counter 1 Timeout Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn gpcnt1_im(&self) -> Gpcnt1Im {
        let val = (self.0 >> 13usize) & 0x01;
        Gpcnt1Im::from_bits(val as u8)
    }
    #[doc = "General Purpose Counter 1 Timeout Interrupt Mask."]
    #[inline(always)]
    pub const fn set_gpcnt1_im(&mut self, val: Gpcnt1Im) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Receive Data Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rx_data_im(&self) -> RxDataIm {
        let val = (self.0 >> 14usize) & 0x01;
        RxDataIm::from_bits(val as u8)
    }
    #[doc = "Receive Data Interrupt Mask."]
    #[inline(always)]
    pub const fn set_rx_data_im(&mut self, val: RxDataIm) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Parity Error Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn pef_im(&self) -> PefIm {
        let val = (self.0 >> 15usize) & 0x01;
        PefIm::from_bits(val as u8)
    }
    #[doc = "Parity Error Interrupt Mask."]
    #[inline(always)]
    pub const fn set_pef_im(&mut self, val: PefIm) {
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
#[doc = "Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Receive FIFO Depth."]
    #[must_use]
    #[inline(always)]
    pub const fn rx_fifo_depth(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive FIFO Depth."]
    #[inline(always)]
    pub const fn set_rx_fifo_depth(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Transmit FIFO Depth."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_fifo_depth(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit FIFO Depth."]
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
#[doc = "Port Control and Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcsr(pub u32);
impl Pcsr {
    #[doc = "Auto Power Down Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sapd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Power Down Enable."]
    #[inline(always)]
    pub const fn set_sapd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Vcc Enable for Smart Card."]
    #[must_use]
    #[inline(always)]
    pub const fn svcc_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Vcc Enable for Smart Card."]
    #[inline(always)]
    pub const fn set_svcc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "VCC Enable Polarity Control."]
    #[must_use]
    #[inline(always)]
    pub const fn vccenp(&self) -> Vccenp {
        let val = (self.0 >> 2usize) & 0x01;
        Vccenp::from_bits(val as u8)
    }
    #[doc = "VCC Enable Polarity Control."]
    #[inline(always)]
    pub const fn set_vccenp(&mut self, val: Vccenp) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Reset Smart Card."]
    #[must_use]
    #[inline(always)]
    pub const fn srst(&self) -> Srst {
        let val = (self.0 >> 3usize) & 0x01;
        Srst::from_bits(val as u8)
    }
    #[doc = "Reset Smart Card."]
    #[inline(always)]
    pub const fn set_srst(&mut self, val: Srst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Clock Enable for Smart Card."]
    #[must_use]
    #[inline(always)]
    pub const fn scen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Enable for Smart Card."]
    #[inline(always)]
    pub const fn set_scen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Smart Card Clock Stop Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn scsp(&self) -> Scsp {
        let val = (self.0 >> 5usize) & 0x01;
        Scsp::from_bits(val as u8)
    }
    #[doc = "Smart Card Clock Stop Polarity."]
    #[inline(always)]
    pub const fn set_scsp(&mut self, val: Scsp) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Auto Power-Down Control."]
    #[must_use]
    #[inline(always)]
    pub const fn spd(&self) -> Spd {
        let val = (self.0 >> 7usize) & 0x01;
        Spd::from_bits(val as u8)
    }
    #[doc = "Auto Power-Down Control."]
    #[inline(always)]
    pub const fn set_spd(&mut self, val: Spd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Smart Card Presence Detect Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn spdim(&self) -> Spdim {
        let val = (self.0 >> 24usize) & 0x01;
        Spdim::from_bits(val as u8)
    }
    #[doc = "Smart Card Presence Detect Interrupt Mask."]
    #[inline(always)]
    pub const fn set_spdim(&mut self, val: Spdim) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Smart Card Presence Detect Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn spdif(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Smart Card Presence Detect Interrupt Flag."]
    #[inline(always)]
    pub const fn set_spdif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Smart Card Presence Detect Pin Status."]
    #[must_use]
    #[inline(always)]
    pub const fn spdp(&self) -> Spdp {
        let val = (self.0 >> 26usize) & 0x01;
        Spdp::from_bits(val as u8)
    }
    #[doc = "Smart Card Presence Detect Pin Status."]
    #[inline(always)]
    pub const fn set_spdp(&mut self, val: Spdp) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SIM Presence Detect Edge Select."]
    #[must_use]
    #[inline(always)]
    pub const fn spdes(&self) -> Spdes {
        let val = (self.0 >> 27usize) & 0x01;
        Spdes::from_bits(val as u8)
    }
    #[doc = "SIM Presence Detect Edge Select."]
    #[inline(always)]
    pub const fn set_spdes(&mut self, val: Spdes) {
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
#[doc = "Receive Data Read Buffer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxBuf(pub u32);
impl RxBuf {
    #[doc = "Receive Data Byte Read."]
    #[must_use]
    #[inline(always)]
    pub const fn rx_byte(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Data Byte Read."]
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
#[doc = "Receive Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxStatus(pub u32);
impl RxStatus {
    #[doc = "Receive FIFO Overflow Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rfo(&self) -> Rfo {
        let val = (self.0 >> 0usize) & 0x01;
        Rfo::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Overflow Flag."]
    #[inline(always)]
    pub const fn set_rfo(&mut self, val: Rfo) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rx_data(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rx_data(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receive Data Threshold Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rdtf(&self) -> Rdtf {
        let val = (self.0 >> 5usize) & 0x01;
        Rdtf::from_bits(val as u8)
    }
    #[doc = "Receive Data Threshold Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rdtf(&mut self, val: Rdtf) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "LRC Check OK Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lrc_ok(&self) -> LrcOk {
        let val = (self.0 >> 6usize) & 0x01;
        LrcOk::from_bits(val as u8)
    }
    #[doc = "LRC Check OK Flag."]
    #[inline(always)]
    pub const fn set_lrc_ok(&mut self, val: LrcOk) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "CRC Check OK Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn crc_ok(&self) -> CrcOk {
        let val = (self.0 >> 7usize) & 0x01;
        CrcOk::from_bits(val as u8)
    }
    #[doc = "CRC Check OK Flag."]
    #[inline(always)]
    pub const fn set_crc_ok(&mut self, val: CrcOk) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Character Wait Time Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cwt_err(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Character Wait Time Error Flag."]
    #[inline(always)]
    pub const fn set_cwt_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Received NACK Threshold Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rte(&self) -> Rte {
        let val = (self.0 >> 9usize) & 0x01;
        Rte::from_bits(val as u8)
    }
    #[doc = "Received NACK Threshold Error Flag."]
    #[inline(always)]
    pub const fn set_rte(&mut self, val: Rte) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Block Wait Time Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bwt_err(&self) -> BwtErr {
        let val = (self.0 >> 10usize) & 0x01;
        BwtErr::from_bits(val as u8)
    }
    #[doc = "Block Wait Time Error Flag."]
    #[inline(always)]
    pub const fn set_bwt_err(&mut self, val: BwtErr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Block Guard Time Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bgt_err(&self) -> BgtErr {
        let val = (self.0 >> 11usize) & 0x01;
        BgtErr::from_bits(val as u8)
    }
    #[doc = "Block Guard Time Error Flag."]
    #[inline(always)]
    pub const fn set_bgt_err(&mut self, val: BgtErr) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Parity Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pef(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error Flag."]
    #[inline(always)]
    pub const fn set_pef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Frame Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fef(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Error Flag."]
    #[inline(always)]
    pub const fn set_fef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Receive FIFO Write Pointer Value."]
    #[must_use]
    #[inline(always)]
    pub const fn rx_wptr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Receive FIFO Write Pointer Value."]
    #[inline(always)]
    pub const fn set_rx_wptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Receive FIFO Byte Count."]
    #[must_use]
    #[inline(always)]
    pub const fn rx_cnt(&self) -> RxCnt {
        let val = (self.0 >> 24usize) & 0x0f;
        RxCnt::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Byte Count."]
    #[inline(always)]
    pub const fn set_rx_cnt(&mut self, val: RxCnt) {
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
#[doc = "Receiver Threshold."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxThd(pub u32);
impl RxThd {
    #[doc = "Receiver Data Threshold Value."]
    #[must_use]
    #[inline(always)]
    pub const fn rdt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Receiver Data Threshold Value."]
    #[inline(always)]
    pub const fn set_rdt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Receiver NACK Threshold Value."]
    #[must_use]
    #[inline(always)]
    pub const fn rnck_thd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Receiver NACK Threshold Value."]
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
#[doc = "Transmit Data Buffer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxBuf(pub u32);
impl TxBuf {
    #[doc = "Transmit Data Byte."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_byte(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Data Byte."]
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
#[doc = "Transmitter Guard ETU Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxGetu(pub u32);
impl TxGetu {
    #[doc = "Transmitter Guard Time Value in ETU."]
    #[must_use]
    #[inline(always)]
    pub const fn getu(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmitter Guard Time Value in ETU."]
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
#[doc = "Transmitter Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxStatus(pub u32);
impl TxStatus {
    #[doc = "Transmit NACK Threshold Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tnte(&self) -> Tnte {
        let val = (self.0 >> 0usize) & 0x01;
        Tnte::from_bits(val as u8)
    }
    #[doc = "Transmit NACK Threshold Error Flag."]
    #[inline(always)]
    pub const fn set_tnte(&mut self, val: Tnte) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO Empty Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tfe(&self) -> Tfe {
        let val = (self.0 >> 3usize) & 0x01;
        Tfe::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Empty Flag."]
    #[inline(always)]
    pub const fn set_tfe(&mut self, val: Tfe) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Early Transmit Complete Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn etcf(&self) -> Etcf {
        let val = (self.0 >> 4usize) & 0x01;
        Etcf::from_bits(val as u8)
    }
    #[doc = "Early Transmit Complete Flag."]
    #[inline(always)]
    pub const fn set_etcf(&mut self, val: Etcf) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit Complete Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tcf(&self) -> Tcf {
        let val = (self.0 >> 5usize) & 0x01;
        Tcf::from_bits(val as u8)
    }
    #[doc = "Transmit Complete Flag."]
    #[inline(always)]
    pub const fn set_tcf(&mut self, val: Tcf) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmit FIFO Full Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tff(&self) -> Tff {
        let val = (self.0 >> 6usize) & 0x01;
        Tff::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Full Flag."]
    #[inline(always)]
    pub const fn set_tff(&mut self, val: Tff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit Data Threshold Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tdtf(&self) -> Tdtf {
        let val = (self.0 >> 7usize) & 0x01;
        Tdtf::from_bits(val as u8)
    }
    #[doc = "Transmit Data Threshold Flag."]
    #[inline(always)]
    pub const fn set_tdtf(&mut self, val: Tdtf) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "General Purpose Counter 0 Timeout Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn gpcnt0_to(&self) -> Gpcnt0To {
        let val = (self.0 >> 8usize) & 0x01;
        Gpcnt0To::from_bits(val as u8)
    }
    #[doc = "General Purpose Counter 0 Timeout Flag."]
    #[inline(always)]
    pub const fn set_gpcnt0_to(&mut self, val: Gpcnt0To) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "General Purpose Counter 1 Timeout Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn gpcnt1_to(&self) -> Gpcnt1To {
        let val = (self.0 >> 9usize) & 0x01;
        Gpcnt1To::from_bits(val as u8)
    }
    #[doc = "General Purpose Counter 1 Timeout Flag."]
    #[inline(always)]
    pub const fn set_gpcnt1_to(&mut self, val: Gpcnt1To) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmit FIFO Read Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_rptr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmit FIFO Read Pointer."]
    #[inline(always)]
    pub const fn set_tx_rptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Transmit FIFO Byte Count."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cnt(&self) -> TxCnt {
        let val = (self.0 >> 24usize) & 0x0f;
        TxCnt::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Byte Count."]
    #[inline(always)]
    pub const fn set_tx_cnt(&mut self, val: TxCnt) {
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
#[doc = "Transmitter Threshold."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxThd(pub u32);
impl TxThd {
    #[doc = "Transmitter Data Threshold Value."]
    #[must_use]
    #[inline(always)]
    pub const fn tdt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmitter Data Threshold Value."]
    #[inline(always)]
    pub const fn set_tdt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Transmitter NACK Threshold Value."]
    #[must_use]
    #[inline(always)]
    pub const fn tnck_thd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmitter NACK Threshold Value."]
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
#[doc = "Version ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VerId(pub u32);
impl VerId {
    #[doc = "Version ID."]
    #[must_use]
    #[inline(always)]
    pub const fn ver(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Version ID."]
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BgtErr {
    #[doc = "Sufficient."]
    BgtErrSufficient = 0x0,
    #[doc = "Too small."]
    BgtErrToosmall = 0x01,
}
impl BgtErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BgtErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BgtErr {
    #[inline(always)]
    fn from(val: u8) -> BgtErr {
        BgtErr::from_bits(val)
    }
}
impl From<BgtErr> for u8 {
    #[inline(always)]
    fn from(val: BgtErr) -> u8 {
        BgtErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BgtErrIm {
    #[doc = "Enable."]
    IntEnabled = 0x0,
    #[doc = "Masked."]
    IntMasked = 0x01,
}
impl BgtErrIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BgtErrIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BgtErrIm {
    #[inline(always)]
    fn from(val: u8) -> BgtErrIm {
        BgtErrIm::from_bits(val)
    }
}
impl From<BgtErrIm> for u8 {
    #[inline(always)]
    fn from(val: BgtErrIm) -> u8 {
        BgtErrIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BwtErr {
    #[doc = "Not exceeded."]
    BwtErrNo = 0x0,
    #[doc = "Exceeded."]
    BwtErrYes = 0x01,
}
impl BwtErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BwtErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BwtErr {
    #[inline(always)]
    fn from(val: u8) -> BwtErr {
        BwtErr::from_bits(val)
    }
}
impl From<BwtErr> for u8 {
    #[inline(always)]
    fn from(val: BwtErr) -> u8 {
        BwtErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BwtErrIm {
    #[doc = "Enable."]
    IntEnabled = 0x0,
    #[doc = "Masked."]
    IntMasked = 0x01,
}
impl BwtErrIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BwtErrIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BwtErrIm {
    #[inline(always)]
    fn from(val: u8) -> BwtErrIm {
        BwtErrIm::from_bits(val)
    }
}
impl From<BwtErrIm> for u8 {
    #[inline(always)]
    fn from(val: BwtErrIm) -> u8 {
        BwtErrIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcOk {
    #[doc = "Current CRC value does not match remainder."]
    CrcNotok = 0x0,
    #[doc = "Current calculated CRC value matches the expected result."]
    CrcOk = 0x01,
}
impl CrcOk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcOk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcOk {
    #[inline(always)]
    fn from(val: u8) -> CrcOk {
        CrcOk::from_bits(val)
    }
}
impl From<CrcOk> for u8 {
    #[inline(always)]
    fn from(val: CrcOk) -> u8 {
        CrcOk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CwtErrIm {
    #[doc = "Enable."]
    IntEnabled = 0x0,
    #[doc = "Masked."]
    IntDisabled = 0x01,
}
impl CwtErrIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CwtErrIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CwtErrIm {
    #[inline(always)]
    fn from(val: u8) -> CwtErrIm {
        CwtErrIm::from_bits(val)
    }
}
impl From<CwtErrIm> for u8 {
    #[inline(always)]
    fn from(val: CwtErrIm) -> u8 {
        CwtErrIm::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DivisorValue(u16);
impl DivisorValue {
    #[doc = "Invalid. As per ISO 7816 specification, the minimum value of F/D is 5."]
    pub const Invalid0: Self = Self(0x0);
    #[doc = "Invalid. As per ISO 7816 specification, the minimum value of F/D is 5."]
    pub const Invalid1: Self = Self(0x01);
    #[doc = "Invalid. As per ISO 7816 specification, the minimum value of F/D is 5."]
    pub const Invalid2: Self = Self(0x02);
    #[doc = "Invalid. As per ISO 7816 specification, the minimum value of F/D is 5."]
    pub const Invalid3: Self = Self(0x03);
    #[doc = "Invalid. As per ISO 7816 specification, the minimum value of F/D is 5."]
    pub const Invalid4: Self = Self(0x04);
    #[doc = "Divisor value F/D."]
    pub const Valid5: Self = Self(0x05);
    #[doc = "Divisor value F/D."]
    pub const Valid6: Self = Self(0x06);
    #[doc = "Divisor value F/D."]
    pub const Valid7: Self = Self(0x07);
    #[doc = "Divisor value F/D."]
    pub const Valid8: Self = Self(0x08);
    #[doc = "Divisor value F/D."]
    pub const Valid9: Self = Self(0x09);
}
impl DivisorValue {
    pub const fn from_bits(val: u16) -> DivisorValue {
        Self(val & 0x01ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for DivisorValue {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Invalid0"),
            0x01 => f.write_str("Invalid1"),
            0x02 => f.write_str("Invalid2"),
            0x03 => f.write_str("Invalid3"),
            0x04 => f.write_str("Invalid4"),
            0x05 => f.write_str("Valid5"),
            0x06 => f.write_str("Valid6"),
            0x07 => f.write_str("Valid7"),
            0x08 => f.write_str("Valid8"),
            0x09 => f.write_str("Valid9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DivisorValue {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Invalid0"),
            0x01 => defmt::write!(f, "Invalid1"),
            0x02 => defmt::write!(f, "Invalid2"),
            0x03 => defmt::write!(f, "Invalid3"),
            0x04 => defmt::write!(f, "Invalid4"),
            0x05 => defmt::write!(f, "Valid5"),
            0x06 => defmt::write!(f, "Valid6"),
            0x07 => defmt::write!(f, "Valid7"),
            0x08 => defmt::write!(f, "Valid8"),
            0x09 => defmt::write!(f, "Valid9"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for DivisorValue {
    #[inline(always)]
    fn from(val: u16) -> DivisorValue {
        DivisorValue::from_bits(val)
    }
}
impl From<DivisorValue> for u16 {
    #[inline(always)]
    fn from(val: DivisorValue) -> u16 {
        DivisorValue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DozeEn {
    #[doc = "Disable."]
    DozeGate = 0x0,
    #[doc = "Enable."]
    DozeNogate = 0x01,
}
impl DozeEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DozeEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DozeEn {
    #[inline(always)]
    fn from(val: u8) -> DozeEn {
        DozeEn::from_bits(val)
    }
}
impl From<DozeEn> for u8 {
    #[inline(always)]
    fn from(val: DozeEn) -> u8 {
        DozeEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EtcIm {
    #[doc = "Enable."]
    IntEnabled = 0x0,
    #[doc = "Masked."]
    IntMasked = 0x01,
}
impl EtcIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EtcIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EtcIm {
    #[inline(always)]
    fn from(val: u8) -> EtcIm {
        EtcIm::from_bits(val)
    }
}
impl From<EtcIm> for u8 {
    #[inline(always)]
    fn from(val: EtcIm) -> u8 {
        EtcIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Etcf {
    #[doc = "Pending or incomplete."]
    EtxPending = 0x0,
    #[doc = "Complete."]
    EtxComplete = 0x01,
}
impl Etcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Etcf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Etcf {
    #[inline(always)]
    fn from(val: u8) -> Etcf {
        Etcf::from_bits(val)
    }
}
impl From<Etcf> for u8 {
    #[inline(always)]
    fn from(val: Etcf) -> u8 {
        Etcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlshRx {
    #[doc = "Normal."]
    Normalop = 0x0,
    #[doc = "Reset."]
    Resethold = 0x01,
}
impl FlshRx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlshRx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlshRx {
    #[inline(always)]
    fn from(val: u8) -> FlshRx {
        FlshRx::from_bits(val)
    }
}
impl From<FlshRx> for u8 {
    #[inline(always)]
    fn from(val: FlshRx) -> u8 {
        FlshRx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlshTx {
    #[doc = "Normal."]
    Normalop = 0x0,
    #[doc = "Reset."]
    Resethold = 0x01,
}
impl FlshTx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlshTx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlshTx {
    #[inline(always)]
    fn from(val: u8) -> FlshTx {
        FlshTx::from_bits(val)
    }
}
impl From<FlshTx> for u8 {
    #[inline(always)]
    fn from(val: FlshTx) -> u8 {
        FlshTx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpcnt0ClkSel {
    #[doc = "Disable/reset."]
    Disabled = 0x0,
    #[doc = "Card clock."]
    Cardclk = 0x01,
    #[doc = "Receive clock."]
    Rxclk = 0x02,
    #[doc = "ETU clock (transmit clock)."]
    Txclk = 0x03,
}
impl Gpcnt0ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpcnt0ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpcnt0ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Gpcnt0ClkSel {
        Gpcnt0ClkSel::from_bits(val)
    }
}
impl From<Gpcnt0ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Gpcnt0ClkSel) -> u8 {
        Gpcnt0ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpcnt0Im {
    #[doc = "Enable."]
    IntEnabled = 0x0,
    #[doc = "Masked."]
    IntMasked = 0x01,
}
impl Gpcnt0Im {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpcnt0Im {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpcnt0Im {
    #[inline(always)]
    fn from(val: u8) -> Gpcnt0Im {
        Gpcnt0Im::from_bits(val)
    }
}
impl From<Gpcnt0Im> for u8 {
    #[inline(always)]
    fn from(val: Gpcnt0Im) -> u8 {
        Gpcnt0Im::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpcnt0To {
    #[doc = "GPCNT0 not reached, or flag cleared."]
    Gpcnt0ToNotreached = 0x0,
    #[doc = "GPCNT0 reached."]
    Gpcnt0ToReached = 0x01,
}
impl Gpcnt0To {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpcnt0To {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpcnt0To {
    #[inline(always)]
    fn from(val: u8) -> Gpcnt0To {
        Gpcnt0To::from_bits(val)
    }
}
impl From<Gpcnt0To> for u8 {
    #[inline(always)]
    fn from(val: Gpcnt0To) -> u8 {
        Gpcnt0To::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpcnt1ClkSel {
    #[doc = "Disable/reset."]
    Disabled = 0x0,
    #[doc = "Card clock."]
    Cardclk = 0x01,
    #[doc = "Receive clock."]
    Rxclk = 0x02,
    #[doc = "ETU clock (transmit clock)."]
    Txclk = 0x03,
}
impl Gpcnt1ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpcnt1ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpcnt1ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Gpcnt1ClkSel {
        Gpcnt1ClkSel::from_bits(val)
    }
}
impl From<Gpcnt1ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Gpcnt1ClkSel) -> u8 {
        Gpcnt1ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpcnt1Im {
    #[doc = "Enable."]
    IntEnabled = 0x0,
    #[doc = "Masked."]
    IntMasked = 0x01,
}
impl Gpcnt1Im {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpcnt1Im {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpcnt1Im {
    #[inline(always)]
    fn from(val: u8) -> Gpcnt1Im {
        Gpcnt1Im::from_bits(val)
    }
}
impl From<Gpcnt1Im> for u8 {
    #[inline(always)]
    fn from(val: Gpcnt1Im) -> u8 {
        Gpcnt1Im::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpcnt1To {
    #[doc = "GPCNT1 not reached, or flag cleared."]
    Gpcnt1ToNotreached = 0x0,
    #[doc = "GPCNT1 reached."]
    Gpcnt1ToReached = 0x01,
}
impl Gpcnt1To {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpcnt1To {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpcnt1To {
    #[inline(always)]
    fn from(val: u8) -> Gpcnt1To {
        Gpcnt1To::from_bits(val)
    }
}
impl From<Gpcnt1To> for u8 {
    #[inline(always)]
    fn from(val: Gpcnt1To) -> u8 {
        Gpcnt1To::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ic {
    #[doc = "Direct."]
    DirConvention = 0x0,
    #[doc = "Inverse."]
    InvConvention = 0x01,
}
impl Ic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ic {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ic {
    #[inline(always)]
    fn from(val: u8) -> Ic {
        Ic::from_bits(val)
    }
}
impl From<Ic> for u8 {
    #[inline(always)]
    fn from(val: Ic) -> u8 {
        Ic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KillClocks {
    #[doc = "Enable."]
    InclkEnabled = 0x0,
    #[doc = "Disable."]
    InclkDisabled = 0x01,
}
impl KillClocks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KillClocks {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KillClocks {
    #[inline(always)]
    fn from(val: u8) -> KillClocks {
        KillClocks::from_bits(val)
    }
}
impl From<KillClocks> for u8 {
    #[inline(always)]
    fn from(val: KillClocks) -> u8 {
        KillClocks::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LrcOk {
    #[doc = "No match."]
    LrcNotok = 0x0,
    #[doc = "Match."]
    LrcOk = 0x01,
}
impl LrcOk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LrcOk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LrcOk {
    #[inline(always)]
    fn from(val: u8) -> LrcOk {
        LrcOk::from_bits(val)
    }
}
impl From<LrcOk> for u8 {
    #[inline(always)]
    fn from(val: LrcOk) -> u8 {
        LrcOk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PefIm {
    #[doc = "Enable."]
    IntEnabled = 0x0,
    #[doc = "Masked."]
    IntMasked = 0x01,
}
impl PefIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PefIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PefIm {
    #[inline(always)]
    fn from(val: u8) -> PefIm {
        PefIm::from_bits(val)
    }
}
impl From<PefIm> for u8 {
    #[inline(always)]
    fn from(val: PefIm) -> u8 {
        PefIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcvr11 {
    #[doc = "12 ETU operation."]
    Rcvr12 = 0x0,
    #[doc = "11 ETU operation."]
    Rcvr11 = 0x01,
}
impl Rcvr11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcvr11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcvr11 {
    #[inline(always)]
    fn from(val: u8) -> Rcvr11 {
        Rcvr11::from_bits(val)
    }
}
impl From<Rcvr11> for u8 {
    #[inline(always)]
    fn from(val: Rcvr11) -> u8 {
        Rcvr11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RdtIm {
    #[doc = "Enable."]
    IntEnabled = 0x0,
    #[doc = "Masked."]
    IntMasked = 0x01,
}
impl RdtIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RdtIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RdtIm {
    #[inline(always)]
    fn from(val: u8) -> RdtIm {
        RdtIm::from_bits(val)
    }
}
impl From<RdtIm> for u8 {
    #[inline(always)]
    fn from(val: RdtIm) -> u8 {
        RdtIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdtf {
    #[doc = "Less than threshold."]
    LessthanRxthresh = 0x0,
    #[doc = "Greater than or equal to threshold."]
    GreaterEqRxthresh = 0x01,
}
impl Rdtf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdtf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdtf {
    #[inline(always)]
    fn from(val: u8) -> Rdtf {
        Rdtf::from_bits(val)
    }
}
impl From<Rdtf> for u8 {
    #[inline(always)]
    fn from(val: Rdtf) -> u8 {
        Rdtf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfo {
    #[doc = "No overrun error."]
    NoOverrun = 0x0,
    #[doc = "Overrun error."]
    Overflow = 0x01,
}
impl Rfo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfo {
    #[inline(always)]
    fn from(val: u8) -> Rfo {
        Rfo::from_bits(val)
    }
}
impl From<Rfo> for u8 {
    #[inline(always)]
    fn from(val: Rfo) -> u8 {
        Rfo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RfoIm {
    #[doc = "Enable."]
    IntEnabled = 0x0,
    #[doc = "Masked."]
    IntMasked = 0x01,
}
impl RfoIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RfoIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RfoIm {
    #[inline(always)]
    fn from(val: u8) -> RfoIm {
        RfoIm::from_bits(val)
    }
}
impl From<RfoIm> for u8 {
    #[inline(always)]
    fn from(val: RfoIm) -> u8 {
        RfoIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RnackIm {
    #[doc = "Enable."]
    IntEnabled = 0x0,
    #[doc = "Masked."]
    IntMasked = 0x01,
}
impl RnackIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RnackIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RnackIm {
    #[inline(always)]
    fn from(val: u8) -> RnackIm {
        RnackIm::from_bits(val)
    }
}
impl From<RnackIm> for u8 {
    #[inline(always)]
    fn from(val: RnackIm) -> u8 {
        RnackIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rte {
    #[doc = "Less than."]
    LessthanNackthresh = 0x0,
    #[doc = "Equal to."]
    GreaterEqNackthresh = 0x01,
}
impl Rte {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rte {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rte {
    #[inline(always)]
    fn from(val: u8) -> Rte {
        Rte::from_bits(val)
    }
}
impl From<Rte> for u8 {
    #[inline(always)]
    fn from(val: Rte) -> u8 {
        Rte::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxCnt {
    #[doc = "FIFO empty."]
    FifoEmpty = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl RxCnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxCnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxCnt {
    #[inline(always)]
    fn from(val: u8) -> RxCnt {
        RxCnt::from_bits(val)
    }
}
impl From<RxCnt> for u8 {
    #[inline(always)]
    fn from(val: RxCnt) -> u8 {
        RxCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxDataIm {
    #[doc = "Enable."]
    IntEnabled = 0x0,
    #[doc = "Masked."]
    IntMasked = 0x01,
}
impl RxDataIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxDataIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxDataIm {
    #[inline(always)]
    fn from(val: u8) -> RxDataIm {
        RxDataIm::from_bits(val)
    }
}
impl From<RxDataIm> for u8 {
    #[inline(always)]
    fn from(val: RxDataIm) -> u8 {
        RxDataIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scsp {
    #[doc = "Logic 0."]
    ScspLogic0 = 0x0,
    #[doc = "Logic 1."]
    ScspLogic1 = 0x01,
}
impl Scsp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scsp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scsp {
    #[inline(always)]
    fn from(val: u8) -> Scsp {
        Scsp::from_bits(val)
    }
}
impl From<Scsp> for u8 {
    #[inline(always)]
    fn from(val: Scsp) -> u8 {
        Scsp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spd {
    #[doc = "No."]
    NoEffect = 0x0,
    #[doc = "Yes."]
    Powerdown = 0x01,
}
impl Spd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spd {
    #[inline(always)]
    fn from(val: u8) -> Spd {
        Spd::from_bits(val)
    }
}
impl From<Spd> for u8 {
    #[inline(always)]
    fn from(val: Spd) -> u8 {
        Spd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spdes {
    #[doc = "Falling edge."]
    FallingEdge = 0x0,
    #[doc = "Rising edge."]
    RisingEdge = 0x01,
}
impl Spdes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spdes {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spdes {
    #[inline(always)]
    fn from(val: u8) -> Spdes {
        Spdes::from_bits(val)
    }
}
impl From<Spdes> for u8 {
    #[inline(always)]
    fn from(val: Spdes) -> u8 {
        Spdes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spdim {
    #[doc = "Enable."]
    IntEnabled = 0x0,
    #[doc = "Mask."]
    IntMasked = 0x01,
}
impl Spdim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spdim {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spdim {
    #[inline(always)]
    fn from(val: u8) -> Spdim {
        Spdim::from_bits(val)
    }
}
impl From<Spdim> for u8 {
    #[inline(always)]
    fn from(val: Spdim) -> u8 {
        Spdim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spdp {
    #[doc = "Logic low."]
    LogicLow = 0x0,
    #[doc = "Logic high."]
    LogicHigh = 0x01,
}
impl Spdp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spdp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spdp {
    #[inline(always)]
    fn from(val: u8) -> Spdp {
        Spdp::from_bits(val)
    }
}
impl From<Spdp> for u8 {
    #[inline(always)]
    fn from(val: Spdp) -> u8 {
        Spdp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srst {
    #[doc = "Assert."]
    Asserted = 0x0,
    #[doc = "Deassert."]
    DeAsserted = 0x01,
}
impl Srst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srst {
    #[inline(always)]
    fn from(val: u8) -> Srst {
        Srst::from_bits(val)
    }
}
impl From<Srst> for u8 {
    #[inline(always)]
    fn from(val: Srst) -> u8 {
        Srst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StopEn {
    #[doc = "Disable."]
    StopAllClks = 0x0,
    #[doc = "Enable."]
    OnlySckOn = 0x01,
}
impl StopEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StopEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StopEn {
    #[inline(always)]
    fn from(val: u8) -> StopEn {
        StopEn::from_bits(val)
    }
}
impl From<StopEn> for u8 {
    #[inline(always)]
    fn from(val: StopEn) -> u8 {
        StopEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwRst {
    #[doc = "Normal."]
    Normalop = 0x0,
    #[doc = "Reset."]
    Resethold = 0x01,
}
impl SwRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwRst {
    #[inline(always)]
    fn from(val: u8) -> SwRst {
        SwRst::from_bits(val)
    }
}
impl From<SwRst> for u8 {
    #[inline(always)]
    fn from(val: SwRst) -> u8 {
        SwRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcIm {
    #[doc = "Enable."]
    IntEnabled = 0x0,
    #[doc = "Masked."]
    IntMasked = 0x01,
}
impl TcIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcIm {
    #[inline(always)]
    fn from(val: u8) -> TcIm {
        TcIm::from_bits(val)
    }
}
impl From<TcIm> for u8 {
    #[inline(always)]
    fn from(val: TcIm) -> u8 {
        TcIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcf {
    #[doc = "Pending or incomplete."]
    TxPending = 0x0,
    #[doc = "Complete."]
    TxComplete = 0x01,
}
impl Tcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcf {
    #[inline(always)]
    fn from(val: u8) -> Tcf {
        Tcf::from_bits(val)
    }
}
impl From<Tcf> for u8 {
    #[inline(always)]
    fn from(val: Tcf) -> u8 {
        Tcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TdtIm {
    #[doc = "Enable."]
    IntEnabled = 0x0,
    #[doc = "Masked."]
    IntMasked = 0x01,
}
impl TdtIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TdtIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TdtIm {
    #[inline(always)]
    fn from(val: u8) -> TdtIm {
        TdtIm::from_bits(val)
    }
}
impl From<TdtIm> for u8 {
    #[inline(always)]
    fn from(val: TdtIm) -> u8 {
        TdtIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdtf {
    #[doc = "Threshold exceeded or this field written to 0."]
    LessthanTxthresh = 0x0,
    #[doc = "Threshold not exceeded."]
    GreaterEqTxthresh = 0x01,
}
impl Tdtf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdtf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdtf {
    #[inline(always)]
    fn from(val: u8) -> Tdtf {
        Tdtf::from_bits(val)
    }
}
impl From<Tdtf> for u8 {
    #[inline(always)]
    fn from(val: Tdtf) -> u8 {
        Tdtf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfe {
    #[doc = "Not empty."]
    FifoEmpty = 0x0,
    #[doc = "Empty."]
    FifoNotempty = 0x01,
}
impl Tfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfe {
    #[inline(always)]
    fn from(val: u8) -> Tfe {
        Tfe::from_bits(val)
    }
}
impl From<Tfe> for u8 {
    #[inline(always)]
    fn from(val: Tfe) -> u8 {
        Tfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TfeIm {
    #[doc = "Enable."]
    IntEnabled = 0x0,
    #[doc = "Masked."]
    IntMasked = 0x01,
}
impl TfeIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TfeIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TfeIm {
    #[inline(always)]
    fn from(val: u8) -> TfeIm {
        TfeIm::from_bits(val)
    }
}
impl From<TfeIm> for u8 {
    #[inline(always)]
    fn from(val: TfeIm) -> u8 {
        TfeIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tff {
    #[doc = "Not full."]
    TxFifoNotfull = 0x0,
    #[doc = "Full."]
    TxFifoFull = 0x01,
}
impl Tff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tff {
    #[inline(always)]
    fn from(val: u8) -> Tff {
        Tff::from_bits(val)
    }
}
impl From<Tff> for u8 {
    #[inline(always)]
    fn from(val: Tff) -> u8 {
        Tff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TffIm {
    #[doc = "Enable."]
    IntEnabled = 0x0,
    #[doc = "Masked."]
    IntMasked = 0x01,
}
impl TffIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TffIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TffIm {
    #[inline(always)]
    fn from(val: u8) -> TffIm {
        TffIm::from_bits(val)
    }
}
impl From<TffIm> for u8 {
    #[inline(always)]
    fn from(val: TffIm) -> u8 {
        TffIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TnackIm {
    #[doc = "Enable."]
    IntEnabled = 0x0,
    #[doc = "Masked."]
    IntMasked = 0x01,
}
impl TnackIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TnackIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TnackIm {
    #[inline(always)]
    fn from(val: u8) -> TnackIm {
        TnackIm::from_bits(val)
    }
}
impl From<TnackIm> for u8 {
    #[inline(always)]
    fn from(val: TnackIm) -> u8 {
        TnackIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tnte {
    #[doc = "Threshold not reached."]
    LessthanNackthresh = 0x0,
    #[doc = "Threshold reached."]
    GreaterEqNackthresh = 0x01,
}
impl Tnte {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tnte {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tnte {
    #[inline(always)]
    fn from(val: u8) -> Tnte {
        Tnte::from_bits(val)
    }
}
impl From<Tnte> for u8 {
    #[inline(always)]
    fn from(val: Tnte) -> u8 {
        Tnte::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxCnt {
    #[doc = "FIFO empty."]
    FifoEmpty = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl TxCnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxCnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxCnt {
    #[inline(always)]
    fn from(val: u8) -> TxCnt {
        TxCnt::from_bits(val)
    }
}
impl From<TxCnt> for u8 {
    #[inline(always)]
    fn from(val: TxCnt) -> u8 {
        TxCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vccenp {
    #[doc = "Active high."]
    ActiveHigh = 0x0,
    #[doc = "Active low."]
    ActiveLow = 0x01,
}
impl Vccenp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vccenp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vccenp {
    #[inline(always)]
    fn from(val: u8) -> Vccenp {
        Vccenp::from_bits(val)
    }
}
impl From<Vccenp> for u8 {
    #[inline(always)]
    fn from(val: Vccenp) -> u8 {
        Vccenp::to_bits(val)
    }
}
