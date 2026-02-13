#[doc = "SPDIF Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn usrc_sel(&self) -> super::vals::UsrcSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::UsrcSel::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_usrc_sel(&mut self, val: super::vals::UsrcSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_sel(&self) -> super::vals::TxSel {
        let val = (self.0 >> 2usize) & 0x07;
        super::vals::TxSel::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_tx_sel(&mut self, val: super::vals::TxSel) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn val_ctrl(&self) -> super::vals::ValCtrl {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::ValCtrl::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_val_ctrl(&mut self, val: super::vals::ValCtrl) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA Transmit Request Enable (Tx FIFO empty)"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_tx_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Transmit Request Enable (Tx FIFO empty)"]
    #[inline(always)]
    pub const fn set_dma_tx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA Receive Request Enable (RX FIFO full)"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_rx_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Receive Request Enable (RX FIFO full)"]
    #[inline(always)]
    pub const fn set_dma_rx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_fifo_ctrl(&self) -> super::vals::TxFifoCtrl {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::TxFifoCtrl::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_tx_fifo_ctrl(&mut self, val: super::vals::TxFifoCtrl) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "When write 1 to this bit, it will cause SPDIF software reset"]
    #[must_use]
    #[inline(always)]
    pub const fn soft_reset(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "When write 1 to this bit, it will cause SPDIF software reset"]
    #[inline(always)]
    pub const fn set_soft_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "When write 1 to this bit, it will cause SPDIF enter low-power mode"]
    #[must_use]
    #[inline(always)]
    pub const fn low_power(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "When write 1 to this bit, it will cause SPDIF enter low-power mode"]
    #[inline(always)]
    pub const fn set_low_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_fifoempty_sel(&self) -> super::vals::TxFifoemptySel {
        let val = (self.0 >> 15usize) & 0x03;
        super::vals::TxFifoemptySel::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_tx_fifoempty_sel(&mut self, val: super::vals::TxFifoemptySel) {
        self.0 = (self.0 & !(0x03 << 15usize)) | (((val.to_bits() as u32) & 0x03) << 15usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_auto_sync(&self) -> super::vals::TxAutoSync {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::TxAutoSync::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_tx_auto_sync(&mut self, val: super::vals::TxAutoSync) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_auto_sync(&self) -> super::vals::RxAutoSync {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::RxAutoSync::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_rx_auto_sync(&mut self, val: super::vals::RxAutoSync) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_fifofull_sel(&self) -> super::vals::RxFifofullSel {
        let val = (self.0 >> 19usize) & 0x03;
        super::vals::RxFifofullSel::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_rx_fifofull_sel(&mut self, val: super::vals::RxFifofullSel) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_fifo_rst(&self) -> super::vals::RxFifoRst {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::RxFifoRst::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_rx_fifo_rst(&mut self, val: super::vals::RxFifoRst) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_fifo_off_on(&self) -> super::vals::RxFifoOffOn {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::RxFifoOffOn::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_rx_fifo_off_on(&mut self, val: super::vals::RxFifoOffOn) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_fifo_ctrl(&self) -> super::vals::RxFifoCtrl {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::RxFifoCtrl::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_rx_fifo_ctrl(&mut self, val: super::vals::RxFifoCtrl) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        Scr(0)
    }
}
impl core::fmt::Debug for Scr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr")
            .field("usrc_sel", &self.usrc_sel())
            .field("tx_sel", &self.tx_sel())
            .field("val_ctrl", &self.val_ctrl())
            .field("dma_tx_en", &self.dma_tx_en())
            .field("dma_rx_en", &self.dma_rx_en())
            .field("tx_fifo_ctrl", &self.tx_fifo_ctrl())
            .field("soft_reset", &self.soft_reset())
            .field("low_power", &self.low_power())
            .field("tx_fifoempty_sel", &self.tx_fifoempty_sel())
            .field("tx_auto_sync", &self.tx_auto_sync())
            .field("rx_auto_sync", &self.rx_auto_sync())
            .field("rx_fifofull_sel", &self.rx_fifofull_sel())
            .field("rx_fifo_rst", &self.rx_fifo_rst())
            .field("rx_fifo_off_on", &self.rx_fifo_off_on())
            .field("rx_fifo_ctrl", &self.rx_fifo_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr {{ usrc_sel: {:?}, tx_sel: {:?}, val_ctrl: {:?}, dma_tx_en: {=bool:?}, dma_rx_en: {=bool:?}, tx_fifo_ctrl: {:?}, soft_reset: {=bool:?}, low_power: {=bool:?}, tx_fifoempty_sel: {:?}, tx_auto_sync: {:?}, rx_auto_sync: {:?}, rx_fifofull_sel: {:?}, rx_fifo_rst: {:?}, rx_fifo_off_on: {:?}, rx_fifo_ctrl: {:?} }}",
            self.usrc_sel(),
            self.tx_sel(),
            self.val_ctrl(),
            self.dma_tx_en(),
            self.dma_rx_en(),
            self.tx_fifo_ctrl(),
            self.soft_reset(),
            self.low_power(),
            self.tx_fifoempty_sel(),
            self.tx_auto_sync(),
            self.rx_auto_sync(),
            self.rx_fifofull_sel(),
            self.rx_fifo_rst(),
            self.rx_fifo_off_on(),
            self.rx_fifo_ctrl()
        )
    }
}
#[doc = "InterruptClear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sic(pub u32);
impl Sic {
    #[doc = "SPDIF receiver loss of lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_loss(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF receiver loss of lock"]
    #[inline(always)]
    pub const fn set_lock_loss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rx FIFO resync"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_fiforesyn(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO resync"]
    #[inline(always)]
    pub const fn set_rx_fiforesyn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rx FIFO underrun/overrun"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_fifoun_ov(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO underrun/overrun"]
    #[inline(always)]
    pub const fn set_rx_fifoun_ov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "U/Q Channel framing error"]
    #[must_use]
    #[inline(always)]
    pub const fn uqerr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "U/Q Channel framing error"]
    #[inline(always)]
    pub const fn set_uqerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "U/Q Channel sync found"]
    #[must_use]
    #[inline(always)]
    pub const fn uqsync(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "U/Q Channel sync found"]
    #[inline(always)]
    pub const fn set_uqsync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Q Channel receive register overrun"]
    #[must_use]
    #[inline(always)]
    pub const fn qrx_ov(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Q Channel receive register overrun"]
    #[inline(always)]
    pub const fn set_qrx_ov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "U Channel receive register overrun"]
    #[must_use]
    #[inline(always)]
    pub const fn urx_ov(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "U Channel receive register overrun"]
    #[inline(always)]
    pub const fn set_urx_ov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SPDIF receiver found parity bit error"]
    #[must_use]
    #[inline(always)]
    pub const fn bit_err(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF receiver found parity bit error"]
    #[inline(always)]
    pub const fn set_bit_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SPDIF receiver found illegal symbol"]
    #[must_use]
    #[inline(always)]
    pub const fn sym_err(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF receiver found illegal symbol"]
    #[inline(always)]
    pub const fn set_sym_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "SPDIF validity flag no good"]
    #[must_use]
    #[inline(always)]
    pub const fn val_no_good(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF validity flag no good"]
    #[inline(always)]
    pub const fn set_val_no_good(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SPDIF receive change in value of control channel"]
    #[must_use]
    #[inline(always)]
    pub const fn cnew(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF receive change in value of control channel"]
    #[inline(always)]
    pub const fn set_cnew(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "SPDIF Tx FIFO resync"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_resyn(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF Tx FIFO resync"]
    #[inline(always)]
    pub const fn set_tx_resyn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SPDIF Tx FIFO under/overrun"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_un_ov(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF Tx FIFO under/overrun"]
    #[inline(always)]
    pub const fn set_tx_un_ov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SPDIF receiver's DPLL is locked"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF receiver's DPLL is locked"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Sic {
    #[inline(always)]
    fn default() -> Sic {
        Sic(0)
    }
}
impl core::fmt::Debug for Sic {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sic")
            .field("lock_loss", &self.lock_loss())
            .field("rx_fiforesyn", &self.rx_fiforesyn())
            .field("rx_fifoun_ov", &self.rx_fifoun_ov())
            .field("uqerr", &self.uqerr())
            .field("uqsync", &self.uqsync())
            .field("qrx_ov", &self.qrx_ov())
            .field("urx_ov", &self.urx_ov())
            .field("bit_err", &self.bit_err())
            .field("sym_err", &self.sym_err())
            .field("val_no_good", &self.val_no_good())
            .field("cnew", &self.cnew())
            .field("tx_resyn", &self.tx_resyn())
            .field("tx_un_ov", &self.tx_un_ov())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sic {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sic {{ lock_loss: {=bool:?}, rx_fiforesyn: {=bool:?}, rx_fifoun_ov: {=bool:?}, uqerr: {=bool:?}, uqsync: {=bool:?}, qrx_ov: {=bool:?}, urx_ov: {=bool:?}, bit_err: {=bool:?}, sym_err: {=bool:?}, val_no_good: {=bool:?}, cnew: {=bool:?}, tx_resyn: {=bool:?}, tx_un_ov: {=bool:?}, lock: {=bool:?} }}",
            self.lock_loss(),
            self.rx_fiforesyn(),
            self.rx_fifoun_ov(),
            self.uqerr(),
            self.uqsync(),
            self.qrx_ov(),
            self.urx_ov(),
            self.bit_err(),
            self.sym_err(),
            self.val_no_good(),
            self.cnew(),
            self.tx_resyn(),
            self.tx_un_ov(),
            self.lock()
        )
    }
}
#[doc = "InterruptEn Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sie(pub u32);
impl Sie {
    #[doc = "SPDIF Rx FIFO full, can't be cleared with reg. IntClear. To clear it, read from Rx FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn rx_fifoful(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF Rx FIFO full, can't be cleared with reg. IntClear. To clear it, read from Rx FIFO."]
    #[inline(always)]
    pub const fn set_rx_fifoful(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SPDIF Tx FIFO empty, can't be cleared with reg. IntClear. To clear it, write toTx FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_em(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF Tx FIFO empty, can't be cleared with reg. IntClear. To clear it, write toTx FIFO."]
    #[inline(always)]
    pub const fn set_tx_em(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SPDIF receiver loss of lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_loss(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF receiver loss of lock"]
    #[inline(always)]
    pub const fn set_lock_loss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rx FIFO resync"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_fiforesyn(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO resync"]
    #[inline(always)]
    pub const fn set_rx_fiforesyn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rx FIFO underrun/overrun"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_fifoun_ov(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO underrun/overrun"]
    #[inline(always)]
    pub const fn set_rx_fifoun_ov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "U/Q Channel framing error"]
    #[must_use]
    #[inline(always)]
    pub const fn uqerr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "U/Q Channel framing error"]
    #[inline(always)]
    pub const fn set_uqerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "U/Q Channel sync found"]
    #[must_use]
    #[inline(always)]
    pub const fn uqsync(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "U/Q Channel sync found"]
    #[inline(always)]
    pub const fn set_uqsync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Q Channel receive register overrun"]
    #[must_use]
    #[inline(always)]
    pub const fn qrx_ov(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Q Channel receive register overrun"]
    #[inline(always)]
    pub const fn set_qrx_ov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Q Channel receive register full, can't be cleared with reg"]
    #[must_use]
    #[inline(always)]
    pub const fn qrx_ful(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Q Channel receive register full, can't be cleared with reg"]
    #[inline(always)]
    pub const fn set_qrx_ful(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "U Channel receive register overrun"]
    #[must_use]
    #[inline(always)]
    pub const fn urx_ov(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "U Channel receive register overrun"]
    #[inline(always)]
    pub const fn set_urx_ov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "U Channel receive register full, can't be cleared with reg"]
    #[must_use]
    #[inline(always)]
    pub const fn urx_ful(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "U Channel receive register full, can't be cleared with reg"]
    #[inline(always)]
    pub const fn set_urx_ful(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SPDIF receiver found parity bit error"]
    #[must_use]
    #[inline(always)]
    pub const fn bit_err(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF receiver found parity bit error"]
    #[inline(always)]
    pub const fn set_bit_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SPDIF receiver found illegal symbol"]
    #[must_use]
    #[inline(always)]
    pub const fn sym_err(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF receiver found illegal symbol"]
    #[inline(always)]
    pub const fn set_sym_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "SPDIF validity flag no good"]
    #[must_use]
    #[inline(always)]
    pub const fn val_no_good(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF validity flag no good"]
    #[inline(always)]
    pub const fn set_val_no_good(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SPDIF receive change in value of control channel"]
    #[must_use]
    #[inline(always)]
    pub const fn cnew(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF receive change in value of control channel"]
    #[inline(always)]
    pub const fn set_cnew(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "SPDIF Tx FIFO resync"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_resyn(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF Tx FIFO resync"]
    #[inline(always)]
    pub const fn set_tx_resyn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SPDIF Tx FIFO under/overrun"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_un_ov(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF Tx FIFO under/overrun"]
    #[inline(always)]
    pub const fn set_tx_un_ov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SPDIF receiver's DPLL is locked"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF receiver's DPLL is locked"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Sie {
    #[inline(always)]
    fn default() -> Sie {
        Sie(0)
    }
}
impl core::fmt::Debug for Sie {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sie")
            .field("rx_fifoful", &self.rx_fifoful())
            .field("tx_em", &self.tx_em())
            .field("lock_loss", &self.lock_loss())
            .field("rx_fiforesyn", &self.rx_fiforesyn())
            .field("rx_fifoun_ov", &self.rx_fifoun_ov())
            .field("uqerr", &self.uqerr())
            .field("uqsync", &self.uqsync())
            .field("qrx_ov", &self.qrx_ov())
            .field("qrx_ful", &self.qrx_ful())
            .field("urx_ov", &self.urx_ov())
            .field("urx_ful", &self.urx_ful())
            .field("bit_err", &self.bit_err())
            .field("sym_err", &self.sym_err())
            .field("val_no_good", &self.val_no_good())
            .field("cnew", &self.cnew())
            .field("tx_resyn", &self.tx_resyn())
            .field("tx_un_ov", &self.tx_un_ov())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sie {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sie {{ rx_fifoful: {=bool:?}, tx_em: {=bool:?}, lock_loss: {=bool:?}, rx_fiforesyn: {=bool:?}, rx_fifoun_ov: {=bool:?}, uqerr: {=bool:?}, uqsync: {=bool:?}, qrx_ov: {=bool:?}, qrx_ful: {=bool:?}, urx_ov: {=bool:?}, urx_ful: {=bool:?}, bit_err: {=bool:?}, sym_err: {=bool:?}, val_no_good: {=bool:?}, cnew: {=bool:?}, tx_resyn: {=bool:?}, tx_un_ov: {=bool:?}, lock: {=bool:?} }}",
            self.rx_fifoful(),
            self.tx_em(),
            self.lock_loss(),
            self.rx_fiforesyn(),
            self.rx_fifoun_ov(),
            self.uqerr(),
            self.uqsync(),
            self.qrx_ov(),
            self.qrx_ful(),
            self.urx_ov(),
            self.urx_ful(),
            self.bit_err(),
            self.sym_err(),
            self.val_no_good(),
            self.cnew(),
            self.tx_resyn(),
            self.tx_un_ov(),
            self.lock()
        )
    }
}
#[doc = "InterruptStat Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sis(pub u32);
impl Sis {
    #[doc = "SPDIF Rx FIFO full, can't be cleared with reg. IntClear. To clear it, read from Rx FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn rx_fifoful(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF Rx FIFO full, can't be cleared with reg. IntClear. To clear it, read from Rx FIFO."]
    #[inline(always)]
    pub const fn set_rx_fifoful(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SPDIF Tx FIFO empty, can't be cleared with reg. IntClear. To clear it, write toTx FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_em(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF Tx FIFO empty, can't be cleared with reg. IntClear. To clear it, write toTx FIFO."]
    #[inline(always)]
    pub const fn set_tx_em(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SPDIF receiver loss of lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_loss(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF receiver loss of lock"]
    #[inline(always)]
    pub const fn set_lock_loss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rx FIFO resync"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_fiforesyn(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO resync"]
    #[inline(always)]
    pub const fn set_rx_fiforesyn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rx FIFO underrun/overrun"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_fifoun_ov(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO underrun/overrun"]
    #[inline(always)]
    pub const fn set_rx_fifoun_ov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "U/Q Channel framing error"]
    #[must_use]
    #[inline(always)]
    pub const fn uqerr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "U/Q Channel framing error"]
    #[inline(always)]
    pub const fn set_uqerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "U/Q Channel sync found"]
    #[must_use]
    #[inline(always)]
    pub const fn uqsync(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "U/Q Channel sync found"]
    #[inline(always)]
    pub const fn set_uqsync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Q Channel receive register overrun"]
    #[must_use]
    #[inline(always)]
    pub const fn qrx_ov(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Q Channel receive register overrun"]
    #[inline(always)]
    pub const fn set_qrx_ov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Q Channel receive register full, can't be cleared with reg"]
    #[must_use]
    #[inline(always)]
    pub const fn qrx_ful(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Q Channel receive register full, can't be cleared with reg"]
    #[inline(always)]
    pub const fn set_qrx_ful(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "U Channel receive register overrun"]
    #[must_use]
    #[inline(always)]
    pub const fn urx_ov(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "U Channel receive register overrun"]
    #[inline(always)]
    pub const fn set_urx_ov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "U Channel receive register full, can't be cleared with reg"]
    #[must_use]
    #[inline(always)]
    pub const fn urx_ful(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "U Channel receive register full, can't be cleared with reg"]
    #[inline(always)]
    pub const fn set_urx_ful(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SPDIF receiver found parity bit error"]
    #[must_use]
    #[inline(always)]
    pub const fn bit_err(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF receiver found parity bit error"]
    #[inline(always)]
    pub const fn set_bit_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SPDIF receiver found illegal symbol"]
    #[must_use]
    #[inline(always)]
    pub const fn sym_err(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF receiver found illegal symbol"]
    #[inline(always)]
    pub const fn set_sym_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "SPDIF validity flag no good"]
    #[must_use]
    #[inline(always)]
    pub const fn val_no_good(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF validity flag no good"]
    #[inline(always)]
    pub const fn set_val_no_good(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SPDIF receive change in value of control channel"]
    #[must_use]
    #[inline(always)]
    pub const fn cnew(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF receive change in value of control channel"]
    #[inline(always)]
    pub const fn set_cnew(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "SPDIF Tx FIFO resync"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_resyn(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF Tx FIFO resync"]
    #[inline(always)]
    pub const fn set_tx_resyn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SPDIF Tx FIFO under/overrun"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_un_ov(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF Tx FIFO under/overrun"]
    #[inline(always)]
    pub const fn set_tx_un_ov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SPDIF receiver's DPLL is locked"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIF receiver's DPLL is locked"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Sis {
    #[inline(always)]
    fn default() -> Sis {
        Sis(0)
    }
}
impl core::fmt::Debug for Sis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sis")
            .field("rx_fifoful", &self.rx_fifoful())
            .field("tx_em", &self.tx_em())
            .field("lock_loss", &self.lock_loss())
            .field("rx_fiforesyn", &self.rx_fiforesyn())
            .field("rx_fifoun_ov", &self.rx_fifoun_ov())
            .field("uqerr", &self.uqerr())
            .field("uqsync", &self.uqsync())
            .field("qrx_ov", &self.qrx_ov())
            .field("qrx_ful", &self.qrx_ful())
            .field("urx_ov", &self.urx_ov())
            .field("urx_ful", &self.urx_ful())
            .field("bit_err", &self.bit_err())
            .field("sym_err", &self.sym_err())
            .field("val_no_good", &self.val_no_good())
            .field("cnew", &self.cnew())
            .field("tx_resyn", &self.tx_resyn())
            .field("tx_un_ov", &self.tx_un_ov())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sis {{ rx_fifoful: {=bool:?}, tx_em: {=bool:?}, lock_loss: {=bool:?}, rx_fiforesyn: {=bool:?}, rx_fifoun_ov: {=bool:?}, uqerr: {=bool:?}, uqsync: {=bool:?}, qrx_ov: {=bool:?}, qrx_ful: {=bool:?}, urx_ov: {=bool:?}, urx_ful: {=bool:?}, bit_err: {=bool:?}, sym_err: {=bool:?}, val_no_good: {=bool:?}, cnew: {=bool:?}, tx_resyn: {=bool:?}, tx_un_ov: {=bool:?}, lock: {=bool:?} }}",
            self.rx_fifoful(),
            self.tx_em(),
            self.lock_loss(),
            self.rx_fiforesyn(),
            self.rx_fifoun_ov(),
            self.uqerr(),
            self.uqsync(),
            self.qrx_ov(),
            self.qrx_ful(),
            self.urx_ov(),
            self.urx_ful(),
            self.bit_err(),
            self.sym_err(),
            self.val_no_good(),
            self.cnew(),
            self.tx_resyn(),
            self.tx_un_ov(),
            self.lock()
        )
    }
}
#[doc = "CDText Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srcd(pub u32);
impl Srcd {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn usync_mode(&self) -> super::vals::UsyncMode {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::UsyncMode::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_usync_mode(&mut self, val: super::vals::UsyncMode) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Srcd {
    #[inline(always)]
    fn default() -> Srcd {
        Srcd(0)
    }
}
impl core::fmt::Debug for Srcd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srcd")
            .field("usync_mode", &self.usync_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srcd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srcd {{ usync_mode: {:?} }}", self.usync_mode())
    }
}
#[doc = "SPDIFRxCChannel_h Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srcsh(pub u32);
impl Srcsh {
    #[doc = "SPDIF receive C channel register, contains first 24 bits of C channel without interpretation"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_cchannel_h(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "SPDIF receive C channel register, contains first 24 bits of C channel without interpretation"]
    #[inline(always)]
    pub const fn set_rx_cchannel_h(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Srcsh {
    #[inline(always)]
    fn default() -> Srcsh {
        Srcsh(0)
    }
}
impl core::fmt::Debug for Srcsh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srcsh")
            .field("rx_cchannel_h", &self.rx_cchannel_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srcsh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Srcsh {{ rx_cchannel_h: {=u32:?} }}",
            self.rx_cchannel_h()
        )
    }
}
#[doc = "SPDIFRxCChannel_l Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srcsl(pub u32);
impl Srcsl {
    #[doc = "SPDIF receive C channel register, contains next 24 bits of C channel without interpretation"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_cchannel_l(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "SPDIF receive C channel register, contains next 24 bits of C channel without interpretation"]
    #[inline(always)]
    pub const fn set_rx_cchannel_l(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Srcsl {
    #[inline(always)]
    fn default() -> Srcsl {
        Srcsl(0)
    }
}
impl core::fmt::Debug for Srcsl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srcsl")
            .field("rx_cchannel_l", &self.rx_cchannel_l())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srcsl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Srcsl {{ rx_cchannel_l: {=u32:?} }}",
            self.rx_cchannel_l()
        )
    }
}
#[doc = "FreqMeas Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srfm(pub u32);
impl Srfm {
    #[doc = "Frequency measurement data"]
    #[must_use]
    #[inline(always)]
    pub const fn freq_meas(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Frequency measurement data"]
    #[inline(always)]
    pub const fn set_freq_meas(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Srfm {
    #[inline(always)]
    fn default() -> Srfm {
        Srfm(0)
    }
}
impl core::fmt::Debug for Srfm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srfm")
            .field("freq_meas", &self.freq_meas())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srfm {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srfm {{ freq_meas: {=u32:?} }}", self.freq_meas())
    }
}
#[doc = "SPDIFRxLeft Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srl(pub u32);
impl Srl {
    #[doc = "Processor receive SPDIF data left"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_data_left(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Processor receive SPDIF data left"]
    #[inline(always)]
    pub const fn set_rx_data_left(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Srl {
    #[inline(always)]
    fn default() -> Srl {
        Srl(0)
    }
}
impl core::fmt::Debug for Srl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srl")
            .field("rx_data_left", &self.rx_data_left())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srl {{ rx_data_left: {=u32:?} }}", self.rx_data_left())
    }
}
#[doc = "PhaseConfig Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srpc(pub u32);
impl Srpc {
    #[doc = "Gain selection:"]
    #[must_use]
    #[inline(always)]
    pub const fn gain_sel(&self) -> super::vals::GainSel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::GainSel::from_bits(val as u8)
    }
    #[doc = "Gain selection:"]
    #[inline(always)]
    pub const fn set_gain_sel(&mut self, val: super::vals::GainSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "LOCK bit to show that the internal DPLL is locked, read only"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK bit to show that the internal DPLL is locked, read only"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Clock source selection, all other settings not shown are reserved:"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_src_sel(&self) -> super::vals::ClkSrcSel {
        let val = (self.0 >> 7usize) & 0x0f;
        super::vals::ClkSrcSel::from_bits(val as u8)
    }
    #[doc = "Clock source selection, all other settings not shown are reserved:"]
    #[inline(always)]
    pub const fn set_clk_src_sel(&mut self, val: super::vals::ClkSrcSel) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val.to_bits() as u32) & 0x0f) << 7usize);
    }
}
impl Default for Srpc {
    #[inline(always)]
    fn default() -> Srpc {
        Srpc(0)
    }
}
impl core::fmt::Debug for Srpc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srpc")
            .field("gain_sel", &self.gain_sel())
            .field("lock", &self.lock())
            .field("clk_src_sel", &self.clk_src_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srpc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Srpc {{ gain_sel: {:?}, lock: {=bool:?}, clk_src_sel: {:?} }}",
            self.gain_sel(),
            self.lock(),
            self.clk_src_sel()
        )
    }
}
#[doc = "QchannelRx Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srq(pub u32);
impl Srq {
    #[doc = "SPDIF receive Q channel register, contains next 3 Q channel bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_qchannel(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "SPDIF receive Q channel register, contains next 3 Q channel bytes"]
    #[inline(always)]
    pub const fn set_rx_qchannel(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Srq {
    #[inline(always)]
    fn default() -> Srq {
        Srq(0)
    }
}
impl core::fmt::Debug for Srq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srq")
            .field("rx_qchannel", &self.rx_qchannel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srq {{ rx_qchannel: {=u32:?} }}", self.rx_qchannel())
    }
}
#[doc = "SPDIFRxRight Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srr(pub u32);
impl Srr {
    #[doc = "Processor receive SPDIF data right"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_data_right(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Processor receive SPDIF data right"]
    #[inline(always)]
    pub const fn set_rx_data_right(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Srr {
    #[inline(always)]
    fn default() -> Srr {
        Srr(0)
    }
}
impl core::fmt::Debug for Srr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srr")
            .field("rx_data_right", &self.rx_data_right())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srr {{ rx_data_right: {=u32:?} }}", self.rx_data_right())
    }
}
#[doc = "UchannelRx Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sru(pub u32);
impl Sru {
    #[doc = "SPDIF receive U channel register, contains next 3 U channel bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_uchannel(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "SPDIF receive U channel register, contains next 3 U channel bytes"]
    #[inline(always)]
    pub const fn set_rx_uchannel(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Sru {
    #[inline(always)]
    fn default() -> Sru {
        Sru(0)
    }
}
impl core::fmt::Debug for Sru {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sru")
            .field("rx_uchannel", &self.rx_uchannel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sru {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sru {{ rx_uchannel: {=u32:?} }}", self.rx_uchannel())
    }
}
#[doc = "SPDIFTxClk Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stc(pub u32);
impl Stc {
    #[doc = "Divider factor (1-128)"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_clk_df(&self) -> super::vals::TxClkDf {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::TxClkDf::from_bits(val as u8)
    }
    #[doc = "Divider factor (1-128)"]
    #[inline(always)]
    pub const fn set_tx_clk_df(&mut self, val: super::vals::TxClkDf) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "Spdif transfer clock enable. When data is going to be transfered, this bit should be set to1."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_all_clk_en(&self) -> super::vals::TxAllClkEn {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::TxAllClkEn::from_bits(val as u8)
    }
    #[doc = "Spdif transfer clock enable. When data is going to be transfered, this bit should be set to1."]
    #[inline(always)]
    pub const fn set_tx_all_clk_en(&mut self, val: super::vals::TxAllClkEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_clk_source(&self) -> super::vals::TxClkSource {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::TxClkSource::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_tx_clk_source(&mut self, val: super::vals::TxClkSource) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "system clock divider factor, 2~512."]
    #[must_use]
    #[inline(always)]
    pub const fn sysclk_df(&self) -> super::vals::SysclkDf {
        let val = (self.0 >> 11usize) & 0x01ff;
        super::vals::SysclkDf::from_bits(val as u16)
    }
    #[doc = "system clock divider factor, 2~512."]
    #[inline(always)]
    pub const fn set_sysclk_df(&mut self, val: super::vals::SysclkDf) {
        self.0 = (self.0 & !(0x01ff << 11usize)) | (((val.to_bits() as u32) & 0x01ff) << 11usize);
    }
}
impl Default for Stc {
    #[inline(always)]
    fn default() -> Stc {
        Stc(0)
    }
}
impl core::fmt::Debug for Stc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stc")
            .field("tx_clk_df", &self.tx_clk_df())
            .field("tx_all_clk_en", &self.tx_all_clk_en())
            .field("tx_clk_source", &self.tx_clk_source())
            .field("sysclk_df", &self.sysclk_df())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stc {{ tx_clk_df: {:?}, tx_all_clk_en: {:?}, tx_clk_source: {:?}, sysclk_df: {:?} }}",
            self.tx_clk_df(),
            self.tx_all_clk_en(),
            self.tx_clk_source(),
            self.sysclk_df()
        )
    }
}
#[doc = "SPDIFTxCChannelCons_h Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stcsch(pub u32);
impl Stcsch {
    #[doc = "SPDIF transmit Cons"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cchannel_cons_h(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "SPDIF transmit Cons"]
    #[inline(always)]
    pub const fn set_tx_cchannel_cons_h(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Stcsch {
    #[inline(always)]
    fn default() -> Stcsch {
        Stcsch(0)
    }
}
impl core::fmt::Debug for Stcsch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stcsch")
            .field("tx_cchannel_cons_h", &self.tx_cchannel_cons_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stcsch {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stcsch {{ tx_cchannel_cons_h: {=u32:?} }}",
            self.tx_cchannel_cons_h()
        )
    }
}
#[doc = "SPDIFTxCChannelCons_l Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stcscl(pub u32);
impl Stcscl {
    #[doc = "SPDIF transmit Cons"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cchannel_cons_l(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "SPDIF transmit Cons"]
    #[inline(always)]
    pub const fn set_tx_cchannel_cons_l(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Stcscl {
    #[inline(always)]
    fn default() -> Stcscl {
        Stcscl(0)
    }
}
impl core::fmt::Debug for Stcscl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stcscl")
            .field("tx_cchannel_cons_l", &self.tx_cchannel_cons_l())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stcscl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stcscl {{ tx_cchannel_cons_l: {=u32:?} }}",
            self.tx_cchannel_cons_l()
        )
    }
}
#[doc = "SPDIFTxLeft Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stl(pub u32);
impl Stl {
    #[doc = "SPDIF transmit left channel data. It is write-only, and always returns zeros when read"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_data_left(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "SPDIF transmit left channel data. It is write-only, and always returns zeros when read"]
    #[inline(always)]
    pub const fn set_tx_data_left(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Stl {
    #[inline(always)]
    fn default() -> Stl {
        Stl(0)
    }
}
impl core::fmt::Debug for Stl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stl")
            .field("tx_data_left", &self.tx_data_left())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Stl {{ tx_data_left: {=u32:?} }}", self.tx_data_left())
    }
}
#[doc = "SPDIFTxRight Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Str(pub u32);
impl Str {
    #[doc = "SPDIF transmit right channel data. It is write-only, and always returns zeros when read"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_data_right(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "SPDIF transmit right channel data. It is write-only, and always returns zeros when read"]
    #[inline(always)]
    pub const fn set_tx_data_right(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Str {
    #[inline(always)]
    fn default() -> Str {
        Str(0)
    }
}
impl core::fmt::Debug for Str {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Str")
            .field("tx_data_right", &self.tx_data_right())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Str {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Str {{ tx_data_right: {=u32:?} }}", self.tx_data_right())
    }
}
