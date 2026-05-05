#[doc = "SPI Configuration register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFG(pub u32);
impl CFG {
    #[doc = "SPI enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPI enable."]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Master mode select."]
    #[must_use]
    #[inline(always)]
    pub const fn MASTER(&self) -> super::vals::MASTER {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::MASTER::from_bits(val as u8)
    }
    #[doc = "Master mode select."]
    #[inline(always)]
    pub const fn set_MASTER(&mut self, val: super::vals::MASTER) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "LSB First mode enable."]
    #[must_use]
    #[inline(always)]
    pub const fn LSBF(&self) -> super::vals::LSBF {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::LSBF::from_bits(val as u8)
    }
    #[doc = "LSB First mode enable."]
    #[inline(always)]
    pub const fn set_LSBF(&mut self, val: super::vals::LSBF) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Clock Phase select."]
    #[must_use]
    #[inline(always)]
    pub const fn CPHA(&self) -> super::vals::CPHA {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CPHA::from_bits(val as u8)
    }
    #[doc = "Clock Phase select."]
    #[inline(always)]
    pub const fn set_CPHA(&mut self, val: super::vals::CPHA) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Clock Polarity select."]
    #[must_use]
    #[inline(always)]
    pub const fn CPOL(&self) -> super::vals::CPOL {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::CPOL::from_bits(val as u8)
    }
    #[doc = "Clock Polarity select."]
    #[inline(always)]
    pub const fn set_CPOL(&mut self, val: super::vals::CPOL) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOP(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
    #[inline(always)]
    pub const fn set_LOOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SSEL0 Polarity select."]
    #[must_use]
    #[inline(always)]
    pub const fn SPOL0(&self) -> super::vals::SPOL0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::SPOL0::from_bits(val as u8)
    }
    #[doc = "SSEL0 Polarity select."]
    #[inline(always)]
    pub const fn set_SPOL0(&mut self, val: super::vals::SPOL0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "SSEL1 Polarity select."]
    #[must_use]
    #[inline(always)]
    pub const fn SPOL1(&self) -> super::vals::SPOL1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::SPOL1::from_bits(val as u8)
    }
    #[doc = "SSEL1 Polarity select."]
    #[inline(always)]
    pub const fn set_SPOL1(&mut self, val: super::vals::SPOL1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "SSEL2 Polarity select."]
    #[must_use]
    #[inline(always)]
    pub const fn SPOL2(&self) -> super::vals::SPOL2 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::SPOL2::from_bits(val as u8)
    }
    #[doc = "SSEL2 Polarity select."]
    #[inline(always)]
    pub const fn set_SPOL2(&mut self, val: super::vals::SPOL2) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "SSEL3 Polarity select."]
    #[must_use]
    #[inline(always)]
    pub const fn SPOL3(&self) -> super::vals::SPOL3 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SPOL3::from_bits(val as u8)
    }
    #[doc = "SSEL3 Polarity select."]
    #[inline(always)]
    pub const fn set_SPOL3(&mut self, val: super::vals::SPOL3) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
}
impl Default for CFG {
    #[inline(always)]
    fn default() -> CFG {
        CFG(0)
    }
}
impl core::fmt::Debug for CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("ENABLE", &self.ENABLE())
            .field("MASTER", &self.MASTER())
            .field("LSBF", &self.LSBF())
            .field("CPHA", &self.CPHA())
            .field("CPOL", &self.CPOL())
            .field("LOOP", &self.LOOP())
            .field("SPOL0", &self.SPOL0())
            .field("SPOL1", &self.SPOL1())
            .field("SPOL2", &self.SPOL2())
            .field("SPOL3", &self.SPOL3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CFG {{ ENABLE: {=bool:?}, MASTER: {:?}, LSBF: {:?}, CPHA: {:?}, CPOL: {:?}, LOOP: {=bool:?}, SPOL0: {:?}, SPOL1: {:?}, SPOL2: {:?}, SPOL3: {:?} }}",
            self.ENABLE(),
            self.MASTER(),
            self.LSBF(),
            self.CPHA(),
            self.CPOL(),
            self.LOOP(),
            self.SPOL0(),
            self.SPOL1(),
            self.SPOL2(),
            self.SPOL3()
        )
    }
}
#[doc = "SPI clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DIV(pub u32);
impl DIV {
    #[doc = "Rate divider value. Specifies how the Flexcomm clock (FCLK) is divided to produce the SPI clock rate in master mode. DIVVAL is -1 encoded such that the value 0 results in FCLK/1, the value 1 results in FCLK/2, up to the maximum possible divide value of 0xFFFF, which results in FCLK/65536."]
    #[must_use]
    #[inline(always)]
    pub const fn DIVVAL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Rate divider value. Specifies how the Flexcomm clock (FCLK) is divided to produce the SPI clock rate in master mode. DIVVAL is -1 encoded such that the value 0 results in FCLK/1, the value 1 results in FCLK/2, up to the maximum possible divide value of 0xFFFF, which results in FCLK/65536."]
    #[inline(always)]
    pub const fn set_DIVVAL(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for DIV {
    #[inline(always)]
    fn default() -> DIV {
        DIV(0)
    }
}
impl core::fmt::Debug for DIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIV")
            .field("DIVVAL", &self.DIVVAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DIV {{ DIVVAL: {=u16:?} }}", self.DIVVAL())
    }
}
#[doc = "SPI Delay register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DLY(pub u32);
impl DLY {
    #[doc = "Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[must_use]
    #[inline(always)]
    pub const fn PRE_DELAY(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub const fn set_PRE_DELAY(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[must_use]
    #[inline(always)]
    pub const fn POST_DELAY(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub const fn set_POST_DELAY(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[must_use]
    #[inline(always)]
    pub const fn FRAME_DELAY(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub const fn set_FRAME_DELAY(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
    #[must_use]
    #[inline(always)]
    pub const fn TRANSFER_DELAY(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
    #[inline(always)]
    pub const fn set_TRANSFER_DELAY(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for DLY {
    #[inline(always)]
    fn default() -> DLY {
        DLY(0)
    }
}
impl core::fmt::Debug for DLY {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLY")
            .field("PRE_DELAY", &self.PRE_DELAY())
            .field("POST_DELAY", &self.POST_DELAY())
            .field("FRAME_DELAY", &self.FRAME_DELAY())
            .field("TRANSFER_DELAY", &self.TRANSFER_DELAY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DLY {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DLY {{ PRE_DELAY: {=u8:?}, POST_DELAY: {=u8:?}, FRAME_DELAY: {=u8:?}, TRANSFER_DELAY: {=u8:?} }}",
            self.PRE_DELAY(),
            self.POST_DELAY(),
            self.FRAME_DELAY(),
            self.TRANSFER_DELAY()
        )
    }
}
#[doc = "FIFO configuration and enable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIFOCFG(pub u32);
impl FIFOCFG {
    #[doc = "Enable the transmit FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLETX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the transmit FIFO."]
    #[inline(always)]
    pub const fn set_ENABLETX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the receive FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLERX(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the receive FIFO."]
    #[inline(always)]
    pub const fn set_ENABLERX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO size configuration. This is a read-only field. 0x0 = FIFO is configured as 16 entries of 8 bits. 0x1, 0x2, 0x3 = not applicable to USART."]
    #[must_use]
    #[inline(always)]
    pub const fn SIZE(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "FIFO size configuration. This is a read-only field. 0x0 = FIFO is configured as 16 entries of 8 bits. 0x1, 0x2, 0x3 = not applicable to USART."]
    #[inline(always)]
    pub const fn set_SIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "DMA configuration for transmit."]
    #[must_use]
    #[inline(always)]
    pub const fn DMATX(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA configuration for transmit."]
    #[inline(always)]
    pub const fn set_DMATX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA configuration for receive."]
    #[must_use]
    #[inline(always)]
    pub const fn DMARX(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA configuration for receive."]
    #[inline(always)]
    pub const fn set_DMARX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKETX(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub const fn set_WAKETX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKERX(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub const fn set_WAKERX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
    #[must_use]
    #[inline(always)]
    pub const fn EMPTYTX(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
    #[inline(always)]
    pub const fn set_EMPTYTX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
    #[must_use]
    #[inline(always)]
    pub const fn EMPTYRX(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
    #[inline(always)]
    pub const fn set_EMPTYRX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for FIFOCFG {
    #[inline(always)]
    fn default() -> FIFOCFG {
        FIFOCFG(0)
    }
}
impl core::fmt::Debug for FIFOCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOCFG")
            .field("ENABLETX", &self.ENABLETX())
            .field("ENABLERX", &self.ENABLERX())
            .field("SIZE", &self.SIZE())
            .field("DMATX", &self.DMATX())
            .field("DMARX", &self.DMARX())
            .field("WAKETX", &self.WAKETX())
            .field("WAKERX", &self.WAKERX())
            .field("EMPTYTX", &self.EMPTYTX())
            .field("EMPTYRX", &self.EMPTYRX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFOCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FIFOCFG {{ ENABLETX: {=bool:?}, ENABLERX: {=bool:?}, SIZE: {=u8:?}, DMATX: {=bool:?}, DMARX: {=bool:?}, WAKETX: {=bool:?}, WAKERX: {=bool:?}, EMPTYTX: {=bool:?}, EMPTYRX: {=bool:?} }}",
            self.ENABLETX(),
            self.ENABLERX(),
            self.SIZE(),
            self.DMATX(),
            self.DMARX(),
            self.WAKETX(),
            self.WAKERX(),
            self.EMPTYTX(),
            self.EMPTYRX()
        )
    }
}
#[doc = "FIFO interrupt enable clear (disable) and read register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIFOINTENCLR(pub u32);
impl FIFOINTENCLR {
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn TXERR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub const fn set_TXERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn RXERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub const fn set_RXERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn TXLVL(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub const fn set_TXLVL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn RXLVL(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub const fn set_RXLVL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for FIFOINTENCLR {
    #[inline(always)]
    fn default() -> FIFOINTENCLR {
        FIFOINTENCLR(0)
    }
}
impl core::fmt::Debug for FIFOINTENCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOINTENCLR")
            .field("TXERR", &self.TXERR())
            .field("RXERR", &self.RXERR())
            .field("TXLVL", &self.TXLVL())
            .field("RXLVL", &self.RXLVL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFOINTENCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FIFOINTENCLR {{ TXERR: {=bool:?}, RXERR: {=bool:?}, TXLVL: {=bool:?}, RXLVL: {=bool:?} }}",
            self.TXERR(),
            self.RXERR(),
            self.TXLVL(),
            self.RXLVL()
        )
    }
}
#[doc = "FIFO interrupt enable set (enable) and read register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIFOINTENSET(pub u32);
impl FIFOINTENSET {
    #[doc = "Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register."]
    #[must_use]
    #[inline(always)]
    pub const fn TXERR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub const fn set_TXERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register."]
    #[must_use]
    #[inline(always)]
    pub const fn RXERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub const fn set_RXERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[must_use]
    #[inline(always)]
    pub const fn TXLVL(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub const fn set_TXLVL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[must_use]
    #[inline(always)]
    pub const fn RXLVL(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub const fn set_RXLVL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for FIFOINTENSET {
    #[inline(always)]
    fn default() -> FIFOINTENSET {
        FIFOINTENSET(0)
    }
}
impl core::fmt::Debug for FIFOINTENSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOINTENSET")
            .field("TXERR", &self.TXERR())
            .field("RXERR", &self.RXERR())
            .field("TXLVL", &self.TXLVL())
            .field("RXLVL", &self.RXLVL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFOINTENSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FIFOINTENSET {{ TXERR: {=bool:?}, RXERR: {=bool:?}, TXLVL: {=bool:?}, RXLVL: {=bool:?} }}",
            self.TXERR(),
            self.RXERR(),
            self.TXLVL(),
            self.RXLVL()
        )
    }
}
#[doc = "FIFO interrupt status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIFOINTSTAT(pub u32);
impl FIFOINTSTAT {
    #[doc = "TX FIFO error."]
    #[must_use]
    #[inline(always)]
    pub const fn TXERR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TX FIFO error."]
    #[inline(always)]
    pub const fn set_TXERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX FIFO error."]
    #[must_use]
    #[inline(always)]
    pub const fn RXERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO error."]
    #[inline(always)]
    pub const fn set_RXERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit FIFO level interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn TXLVL(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO level interrupt."]
    #[inline(always)]
    pub const fn set_TXLVL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive FIFO level interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn RXLVL(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO level interrupt."]
    #[inline(always)]
    pub const fn set_RXLVL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Peripheral interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn PERINT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral interrupt."]
    #[inline(always)]
    pub const fn set_PERINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for FIFOINTSTAT {
    #[inline(always)]
    fn default() -> FIFOINTSTAT {
        FIFOINTSTAT(0)
    }
}
impl core::fmt::Debug for FIFOINTSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOINTSTAT")
            .field("TXERR", &self.TXERR())
            .field("RXERR", &self.RXERR())
            .field("TXLVL", &self.TXLVL())
            .field("RXLVL", &self.RXLVL())
            .field("PERINT", &self.PERINT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFOINTSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FIFOINTSTAT {{ TXERR: {=bool:?}, RXERR: {=bool:?}, TXLVL: {=bool:?}, RXLVL: {=bool:?}, PERINT: {=bool:?} }}",
            self.TXERR(),
            self.RXERR(),
            self.TXLVL(),
            self.RXLVL(),
            self.PERINT()
        )
    }
}
#[doc = "FIFO read data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIFORD(pub u32);
impl FIFORD {
    #[doc = "Received data from the FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn RXDATA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Received data from the FIFO."]
    #[inline(always)]
    pub const fn set_RXDATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL0 pin to be saved along with received data. The value will reflect the SSEL0 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[must_use]
    #[inline(always)]
    pub const fn RXSSEL0_N(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL0 pin to be saved along with received data. The value will reflect the SSEL0 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub const fn set_RXSSEL0_N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL1 pin to be saved along with received data. The value will reflect the SSEL1 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[must_use]
    #[inline(always)]
    pub const fn RXSSEL1_N(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL1 pin to be saved along with received data. The value will reflect the SSEL1 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub const fn set_RXSSEL1_N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL2 pin to be saved along with received data. The value will reflect the SSEL2 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[must_use]
    #[inline(always)]
    pub const fn RXSSEL2_N(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL2 pin to be saved along with received data. The value will reflect the SSEL2 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub const fn set_RXSSEL2_N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL3 pin to be saved along with received data. The value will reflect the SSEL3 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[must_use]
    #[inline(always)]
    pub const fn RXSSEL3_N(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL3 pin to be saved along with received data. The value will reflect the SSEL3 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub const fn set_RXSSEL3_N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Start of Transfer flag. This flag will be 1 if this is the first data after the SSELs went from deasserted to asserted (i.e., any previous transfer has ended). This information can be used to identify the first piece of data in cases where the transfer length is greater than 16 bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SOT(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Transfer flag. This flag will be 1 if this is the first data after the SSELs went from deasserted to asserted (i.e., any previous transfer has ended). This information can be used to identify the first piece of data in cases where the transfer length is greater than 16 bits."]
    #[inline(always)]
    pub const fn set_SOT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for FIFORD {
    #[inline(always)]
    fn default() -> FIFORD {
        FIFORD(0)
    }
}
impl core::fmt::Debug for FIFORD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFORD")
            .field("RXDATA", &self.RXDATA())
            .field("RXSSEL0_N", &self.RXSSEL0_N())
            .field("RXSSEL1_N", &self.RXSSEL1_N())
            .field("RXSSEL2_N", &self.RXSSEL2_N())
            .field("RXSSEL3_N", &self.RXSSEL3_N())
            .field("SOT", &self.SOT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFORD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FIFORD {{ RXDATA: {=u16:?}, RXSSEL0_N: {=bool:?}, RXSSEL1_N: {=bool:?}, RXSSEL2_N: {=bool:?}, RXSSEL3_N: {=bool:?}, SOT: {=bool:?} }}",
            self.RXDATA(),
            self.RXSSEL0_N(),
            self.RXSSEL1_N(),
            self.RXSSEL2_N(),
            self.RXSSEL3_N(),
            self.SOT()
        )
    }
}
#[doc = "FIFO data read with no FIFO pop."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIFORDNOPOP(pub u32);
impl FIFORDNOPOP {
    #[doc = "Received data from the FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn RXDATA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Received data from the FIFO."]
    #[inline(always)]
    pub const fn set_RXDATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Slave Select for receive."]
    #[must_use]
    #[inline(always)]
    pub const fn RXSSEL0_N(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive."]
    #[inline(always)]
    pub const fn set_RXSSEL0_N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Slave Select for receive."]
    #[must_use]
    #[inline(always)]
    pub const fn RXSSEL1_N(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive."]
    #[inline(always)]
    pub const fn set_RXSSEL1_N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Slave Select for receive."]
    #[must_use]
    #[inline(always)]
    pub const fn RXSSEL2_N(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive."]
    #[inline(always)]
    pub const fn set_RXSSEL2_N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Slave Select for receive."]
    #[must_use]
    #[inline(always)]
    pub const fn RXSSEL3_N(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive."]
    #[inline(always)]
    pub const fn set_RXSSEL3_N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Start of transfer flag."]
    #[must_use]
    #[inline(always)]
    pub const fn SOT(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Start of transfer flag."]
    #[inline(always)]
    pub const fn set_SOT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for FIFORDNOPOP {
    #[inline(always)]
    fn default() -> FIFORDNOPOP {
        FIFORDNOPOP(0)
    }
}
impl core::fmt::Debug for FIFORDNOPOP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFORDNOPOP")
            .field("RXDATA", &self.RXDATA())
            .field("RXSSEL0_N", &self.RXSSEL0_N())
            .field("RXSSEL1_N", &self.RXSSEL1_N())
            .field("RXSSEL2_N", &self.RXSSEL2_N())
            .field("RXSSEL3_N", &self.RXSSEL3_N())
            .field("SOT", &self.SOT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFORDNOPOP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FIFORDNOPOP {{ RXDATA: {=u16:?}, RXSSEL0_N: {=bool:?}, RXSSEL1_N: {=bool:?}, RXSSEL2_N: {=bool:?}, RXSSEL3_N: {=bool:?}, SOT: {=bool:?} }}",
            self.RXDATA(),
            self.RXSSEL0_N(),
            self.RXSSEL1_N(),
            self.RXSSEL2_N(),
            self.RXSSEL3_N(),
            self.SOT()
        )
    }
}
#[doc = "FIFO size register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIFOSIZE(pub u32);
impl FIFOSIZE {
    #[doc = "Provides the size of the FIFO for software. The size of the SPI FIFO is 8 entries."]
    #[must_use]
    #[inline(always)]
    pub const fn FIFOSIZE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Provides the size of the FIFO for software. The size of the SPI FIFO is 8 entries."]
    #[inline(always)]
    pub const fn set_FIFOSIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for FIFOSIZE {
    #[inline(always)]
    fn default() -> FIFOSIZE {
        FIFOSIZE(0)
    }
}
impl core::fmt::Debug for FIFOSIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOSIZE")
            .field("FIFOSIZE", &self.FIFOSIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFOSIZE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FIFOSIZE {{ FIFOSIZE: {=u8:?} }}", self.FIFOSIZE())
    }
}
#[doc = "FIFO status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIFOSTAT(pub u32);
impl FIFOSTAT {
    #[doc = "TX FIFO error. Will be set if a transmit FIFO error occurs. This could be an overflow caused by pushing data into a full FIFO, or by an underflow if the FIFO is empty when data is needed. Cleared by writing a 1 to this bit."]
    #[must_use]
    #[inline(always)]
    pub const fn TXERR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TX FIFO error. Will be set if a transmit FIFO error occurs. This could be an overflow caused by pushing data into a full FIFO, or by an underflow if the FIFO is empty when data is needed. Cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub const fn set_TXERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX FIFO error. Will be set if a receive FIFO overflow occurs, caused by software or DMA not emptying the FIFO fast enough. Cleared by writing a 1 to this bit."]
    #[must_use]
    #[inline(always)]
    pub const fn RXERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO error. Will be set if a receive FIFO overflow occurs, caused by software or DMA not emptying the FIFO fast enough. Cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub const fn set_RXERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Peripheral interrupt. When 1, this indicates that the peripheral function has asserted an interrupt. The details can be found by reading the peripheral's STAT register."]
    #[must_use]
    #[inline(always)]
    pub const fn PERINT(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral interrupt. When 1, this indicates that the peripheral function has asserted an interrupt. The details can be found by reading the peripheral's STAT register."]
    #[inline(always)]
    pub const fn set_PERINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit FIFO empty. When 1, the transmit FIFO is empty. The peripheral may still be processing the last piece of data."]
    #[must_use]
    #[inline(always)]
    pub const fn TXEMPTY(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty. When 1, the transmit FIFO is empty. The peripheral may still be processing the last piece of data."]
    #[inline(always)]
    pub const fn set_TXEMPTY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit FIFO not full. When 1, the transmit FIFO is not full, so more data can be written. When 0, the transmit FIFO is full and another write would cause it to overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn TXNOTFULL(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO not full. When 1, the transmit FIFO is not full, so more data can be written. When 0, the transmit FIFO is full and another write would cause it to overflow."]
    #[inline(always)]
    pub const fn set_TXNOTFULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive FIFO not empty. When 1, the receive FIFO is not empty, so data can be read. When 0, the receive FIFO is empty."]
    #[must_use]
    #[inline(always)]
    pub const fn RXNOTEMPTY(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO not empty. When 1, the receive FIFO is not empty, so data can be read. When 0, the receive FIFO is empty."]
    #[inline(always)]
    pub const fn set_RXNOTEMPTY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Receive FIFO full. When 1, the receive FIFO is full. Data needs to be read out to prevent the peripheral from causing an overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn RXFULL(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full. When 1, the receive FIFO is full. Data needs to be read out to prevent the peripheral from causing an overflow."]
    #[inline(always)]
    pub const fn set_RXFULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Transmit FIFO current level. A 0 means the TX FIFO is currently empty, and the TXEMPTY and TXNOTFULL flags will be 1. Other values tell how much data is actually in the TX FIFO at the point where the read occurs. If the TX FIFO is full, the TXEMPTY and TXNOTFULL flags will be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn TXLVL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Transmit FIFO current level. A 0 means the TX FIFO is currently empty, and the TXEMPTY and TXNOTFULL flags will be 1. Other values tell how much data is actually in the TX FIFO at the point where the read occurs. If the TX FIFO is full, the TXEMPTY and TXNOTFULL flags will be 0."]
    #[inline(always)]
    pub const fn set_TXLVL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Receive FIFO current level. A 0 means the RX FIFO is currently empty, and the RXFULL and RXNOTEMPTY flags will be 0. Other values tell how much data is actually in the RX FIFO at the point where the read occurs. If the RX FIFO is full, the RXFULL and RXNOTEMPTY flags will be 1."]
    #[must_use]
    #[inline(always)]
    pub const fn RXLVL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Receive FIFO current level. A 0 means the RX FIFO is currently empty, and the RXFULL and RXNOTEMPTY flags will be 0. Other values tell how much data is actually in the RX FIFO at the point where the read occurs. If the RX FIFO is full, the RXFULL and RXNOTEMPTY flags will be 1."]
    #[inline(always)]
    pub const fn set_RXLVL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for FIFOSTAT {
    #[inline(always)]
    fn default() -> FIFOSTAT {
        FIFOSTAT(0)
    }
}
impl core::fmt::Debug for FIFOSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOSTAT")
            .field("TXERR", &self.TXERR())
            .field("RXERR", &self.RXERR())
            .field("PERINT", &self.PERINT())
            .field("TXEMPTY", &self.TXEMPTY())
            .field("TXNOTFULL", &self.TXNOTFULL())
            .field("RXNOTEMPTY", &self.RXNOTEMPTY())
            .field("RXFULL", &self.RXFULL())
            .field("TXLVL", &self.TXLVL())
            .field("RXLVL", &self.RXLVL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFOSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FIFOSTAT {{ TXERR: {=bool:?}, RXERR: {=bool:?}, PERINT: {=bool:?}, TXEMPTY: {=bool:?}, TXNOTFULL: {=bool:?}, RXNOTEMPTY: {=bool:?}, RXFULL: {=bool:?}, TXLVL: {=u8:?}, RXLVL: {=u8:?} }}",
            self.TXERR(),
            self.RXERR(),
            self.PERINT(),
            self.TXEMPTY(),
            self.TXNOTFULL(),
            self.RXNOTEMPTY(),
            self.RXFULL(),
            self.TXLVL(),
            self.RXLVL()
        )
    }
}
#[doc = "FIFO trigger settings for interrupt and DMA request."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIFOTRIG(pub u32);
impl FIFOTRIG {
    #[doc = "Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
    #[must_use]
    #[inline(always)]
    pub const fn TXLVLENA(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
    #[inline(always)]
    pub const fn set_TXLVLENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
    #[must_use]
    #[inline(always)]
    pub const fn RXLVLENA(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
    #[inline(always)]
    pub const fn set_RXLVLENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    #[must_use]
    #[inline(always)]
    pub const fn TXLVL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    #[inline(always)]
    pub const fn set_TXLVL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
    #[must_use]
    #[inline(always)]
    pub const fn RXLVL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    pub const fn set_RXLVL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for FIFOTRIG {
    #[inline(always)]
    fn default() -> FIFOTRIG {
        FIFOTRIG(0)
    }
}
impl core::fmt::Debug for FIFOTRIG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOTRIG")
            .field("TXLVLENA", &self.TXLVLENA())
            .field("RXLVLENA", &self.RXLVLENA())
            .field("TXLVL", &self.TXLVL())
            .field("RXLVL", &self.RXLVL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFOTRIG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FIFOTRIG {{ TXLVLENA: {=bool:?}, RXLVLENA: {=bool:?}, TXLVL: {=u8:?}, RXLVL: {=u8:?} }}",
            self.TXLVLENA(),
            self.RXLVLENA(),
            self.TXLVL(),
            self.RXLVL()
        )
    }
}
#[doc = "FIFO write data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIFOWR(pub u32);
impl FIFOWR {
    #[doc = "Transmit data to the FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn TXDATA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit data to the FIFO."]
    #[inline(always)]
    pub const fn set_TXDATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Transmit slave select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default."]
    #[must_use]
    #[inline(always)]
    pub const fn TXSSEL0_N(&self) -> super::vals::TXSSEL0_N {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::TXSSEL0_N::from_bits(val as u8)
    }
    #[doc = "Transmit slave select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub const fn set_TXSSEL0_N(&mut self, val: super::vals::TXSSEL0_N) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Transmit slave select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default."]
    #[must_use]
    #[inline(always)]
    pub const fn TXSSEL1_N(&self) -> super::vals::TXSSEL1_N {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::TXSSEL1_N::from_bits(val as u8)
    }
    #[doc = "Transmit slave select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub const fn set_TXSSEL1_N(&mut self, val: super::vals::TXSSEL1_N) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Transmit slave select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default."]
    #[must_use]
    #[inline(always)]
    pub const fn TXSSEL2_N(&self) -> super::vals::TXSSEL2_N {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::TXSSEL2_N::from_bits(val as u8)
    }
    #[doc = "Transmit slave select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub const fn set_TXSSEL2_N(&mut self, val: super::vals::TXSSEL2_N) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Transmit slave select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default."]
    #[must_use]
    #[inline(always)]
    pub const fn TXSSEL3_N(&self) -> super::vals::TXSSEL3_N {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::TXSSEL3_N::from_bits(val as u8)
    }
    #[doc = "Transmit slave select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub const fn set_TXSSEL3_N(&mut self, val: super::vals::TXSSEL3_N) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "End of transfer. The asserted SSEL will be deasserted at the end of a transfer and remain so far at least the time specified by the Transfer_delay value in the DLY register."]
    #[must_use]
    #[inline(always)]
    pub const fn EOT(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "End of transfer. The asserted SSEL will be deasserted at the end of a transfer and remain so far at least the time specified by the Transfer_delay value in the DLY register."]
    #[inline(always)]
    pub const fn set_EOT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "End of frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the Frame_delay value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
    #[must_use]
    #[inline(always)]
    pub const fn EOF(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "End of frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the Frame_delay value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
    #[inline(always)]
    pub const fn set_EOF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn RXIGNORE(&self) -> super::vals::RXIGNORE {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::RXIGNORE::from_bits(val as u8)
    }
    #[doc = "Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA."]
    #[inline(always)]
    pub const fn set_RXIGNORE(&mut self, val: super::vals::RXIGNORE) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Transmit Ignore. This allows data to be received using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA.This bit can only be set by writing to the upper 16 bits only of FIFOWR, i.e., a half-word write to offset 0xE22."]
    #[must_use]
    #[inline(always)]
    pub const fn TXIGNORE(&self) -> super::vals::TXIGNORE {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::TXIGNORE::from_bits(val as u8)
    }
    #[doc = "Transmit Ignore. This allows data to be received using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA.This bit can only be set by writing to the upper 16 bits only of FIFOWR, i.e., a half-word write to offset 0xE22."]
    #[inline(always)]
    pub const fn set_TXIGNORE(&mut self, val: super::vals::TXIGNORE) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Data Length. Specifies the data length from 4 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0-2 = Reserved. 0x3 = Data transfer is 4 bits in length. 0x4 = Data transfer is 5 bits in length. 0xF = Data transfer is 16 bits in length."]
    #[must_use]
    #[inline(always)]
    pub const fn LEN(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Length. Specifies the data length from 4 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0-2 = Reserved. 0x3 = Data transfer is 4 bits in length. 0x4 = Data transfer is 5 bits in length. 0xF = Data transfer is 16 bits in length."]
    #[inline(always)]
    pub const fn set_LEN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for FIFOWR {
    #[inline(always)]
    fn default() -> FIFOWR {
        FIFOWR(0)
    }
}
impl core::fmt::Debug for FIFOWR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOWR")
            .field("TXDATA", &self.TXDATA())
            .field("TXSSEL0_N", &self.TXSSEL0_N())
            .field("TXSSEL1_N", &self.TXSSEL1_N())
            .field("TXSSEL2_N", &self.TXSSEL2_N())
            .field("TXSSEL3_N", &self.TXSSEL3_N())
            .field("EOT", &self.EOT())
            .field("EOF", &self.EOF())
            .field("RXIGNORE", &self.RXIGNORE())
            .field("TXIGNORE", &self.TXIGNORE())
            .field("LEN", &self.LEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFOWR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FIFOWR {{ TXDATA: {=u16:?}, TXSSEL0_N: {:?}, TXSSEL1_N: {:?}, TXSSEL2_N: {:?}, TXSSEL3_N: {:?}, EOT: {=bool:?}, EOF: {=bool:?}, RXIGNORE: {:?}, TXIGNORE: {:?}, LEN: {=u8:?} }}",
            self.TXDATA(),
            self.TXSSEL0_N(),
            self.TXSSEL1_N(),
            self.TXSSEL2_N(),
            self.TXSSEL3_N(),
            self.EOT(),
            self.EOF(),
            self.RXIGNORE(),
            self.TXIGNORE(),
            self.LEN()
        )
    }
}
#[doc = "Peripheral identification register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ID(pub u32);
impl ID {
    #[doc = "Aperture: encoded as (aperture size/4K) -1, so 0x00 means a 4K aperture."]
    #[must_use]
    #[inline(always)]
    pub const fn APERTURE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Aperture: encoded as (aperture size/4K) -1, so 0x00 means a 4K aperture."]
    #[inline(always)]
    pub const fn set_APERTURE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Minor revision of module implementation."]
    #[must_use]
    #[inline(always)]
    pub const fn MINOR_REV(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision of module implementation."]
    #[inline(always)]
    pub const fn set_MINOR_REV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Major revision of module implementation."]
    #[must_use]
    #[inline(always)]
    pub const fn MAJOR_REV(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision of module implementation."]
    #[inline(always)]
    pub const fn set_MAJOR_REV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Module identifier for the selected function."]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Module identifier for the selected function."]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for ID {
    #[inline(always)]
    fn default() -> ID {
        ID(0)
    }
}
impl core::fmt::Debug for ID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID")
            .field("APERTURE", &self.APERTURE())
            .field("MINOR_REV", &self.MINOR_REV())
            .field("MAJOR_REV", &self.MAJOR_REV())
            .field("ID", &self.ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ID {{ APERTURE: {=u8:?}, MINOR_REV: {=u8:?}, MAJOR_REV: {=u8:?}, ID: {=u16:?} }}",
            self.APERTURE(),
            self.MINOR_REV(),
            self.MAJOR_REV(),
            self.ID()
        )
    }
}
#[doc = "SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTENCLR(pub u32);
impl INTENCLR {
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn SSAEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_SSAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn SSDEN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_SSDEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTIDLE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_MSTIDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for INTENCLR {
    #[inline(always)]
    fn default() -> INTENCLR {
        INTENCLR(0)
    }
}
impl core::fmt::Debug for INTENCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENCLR")
            .field("SSAEN", &self.SSAEN())
            .field("SSDEN", &self.SSDEN())
            .field("MSTIDLE", &self.MSTIDLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTENCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTENCLR {{ SSAEN: {=bool:?}, SSDEN: {=bool:?}, MSTIDLE: {=bool:?} }}",
            self.SSAEN(),
            self.SSDEN(),
            self.MSTIDLE()
        )
    }
}
#[doc = "SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTENSET(pub u32);
impl INTENSET {
    #[doc = "Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn SSAEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline(always)]
    pub const fn set_SSAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[must_use]
    #[inline(always)]
    pub const fn SSDEN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline(always)]
    pub const fn set_SSDEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Master idle interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTIDLEEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Master idle interrupt enable."]
    #[inline(always)]
    pub const fn set_MSTIDLEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for INTENSET {
    #[inline(always)]
    fn default() -> INTENSET {
        INTENSET(0)
    }
}
impl core::fmt::Debug for INTENSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENSET")
            .field("SSAEN", &self.SSAEN())
            .field("SSDEN", &self.SSDEN())
            .field("MSTIDLEEN", &self.MSTIDLEEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTENSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTENSET {{ SSAEN: {=bool:?}, SSDEN: {=bool:?}, MSTIDLEEN: {=bool:?} }}",
            self.SSAEN(),
            self.SSDEN(),
            self.MSTIDLEEN()
        )
    }
}
#[doc = "SPI Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTSTAT(pub u32);
impl INTSTAT {
    #[doc = "Slave Select Assert."]
    #[must_use]
    #[inline(always)]
    pub const fn SSA(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select Assert."]
    #[inline(always)]
    pub const fn set_SSA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Slave Select Deassert."]
    #[must_use]
    #[inline(always)]
    pub const fn SSD(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select Deassert."]
    #[inline(always)]
    pub const fn set_SSD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Master Idle status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTIDLE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Master Idle status flag."]
    #[inline(always)]
    pub const fn set_MSTIDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for INTSTAT {
    #[inline(always)]
    fn default() -> INTSTAT {
        INTSTAT(0)
    }
}
impl core::fmt::Debug for INTSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTAT")
            .field("SSA", &self.SSA())
            .field("SSD", &self.SSD())
            .field("MSTIDLE", &self.MSTIDLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTSTAT {{ SSA: {=bool:?}, SSD: {=bool:?}, MSTIDLE: {=bool:?} }}",
            self.SSA(),
            self.SSD(),
            self.MSTIDLE()
        )
    }
}
#[doc = "SPI Status. Some status flags can be cleared by writing a 1 to that bit position."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STAT(pub u32);
impl STAT {
    #[doc = "Slave Select Assert. This flag is set whenever any slave select transitions from deasserted to asserted, in both master and slave modes. This allows determining when the SPI transmit/receive functions become busy, and allows waking up the device from reduced power modes when a slave mode access begins. This flag is cleared by software."]
    #[must_use]
    #[inline(always)]
    pub const fn SSA(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select Assert. This flag is set whenever any slave select transitions from deasserted to asserted, in both master and slave modes. This allows determining when the SPI transmit/receive functions become busy, and allows waking up the device from reduced power modes when a slave mode access begins. This flag is cleared by software."]
    #[inline(always)]
    pub const fn set_SSA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Slave Select Deassert. This flag is set whenever any asserted slave selects transition to deasserted, in both master and slave modes. This allows determining when the SPI transmit/receive functions become idle. This flag is cleared by software."]
    #[must_use]
    #[inline(always)]
    pub const fn SSD(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select Deassert. This flag is set whenever any asserted slave selects transition to deasserted, in both master and slave modes. This allows determining when the SPI transmit/receive functions become idle. This flag is cleared by software."]
    #[inline(always)]
    pub const fn set_SSD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Stalled status flag. This indicates whether the SPI is currently in a stall condition."]
    #[must_use]
    #[inline(always)]
    pub const fn STALLED(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Stalled status flag. This indicates whether the SPI is currently in a stall condition."]
    #[inline(always)]
    pub const fn set_STALLED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "End Transfer control bit. Software can set this bit to force an end to the current transfer when the transmitter finishes any activity already in progress, as if the EOT flag had been set prior to the last transmission. This capability is included to support cases where it is not known when transmit data is written that it will be the end of a transfer. The bit is cleared when the transmitter becomes idle as the transfer comes to an end. Forcing an end of transfer in this manner causes any specified FRAME_DELAY and TRANSFER_DELAY to be inserted."]
    #[must_use]
    #[inline(always)]
    pub const fn ENDTRANSFER(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "End Transfer control bit. Software can set this bit to force an end to the current transfer when the transmitter finishes any activity already in progress, as if the EOT flag had been set prior to the last transmission. This capability is included to support cases where it is not known when transmit data is written that it will be the end of a transfer. The bit is cleared when the transmitter becomes idle as the transfer comes to an end. Forcing an end of transfer in this manner causes any specified FRAME_DELAY and TRANSFER_DELAY to be inserted."]
    #[inline(always)]
    pub const fn set_ENDTRANSFER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Master idle status flag. This bit is 1 whenever the SPI master function is fully idle. This means that the transmit holding register is empty and the transmitter is not in the process of sending data."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTIDLE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Master idle status flag. This bit is 1 whenever the SPI master function is fully idle. This means that the transmit holding register is empty and the transmitter is not in the process of sending data."]
    #[inline(always)]
    pub const fn set_MSTIDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for STAT {
    #[inline(always)]
    fn default() -> STAT {
        STAT(0)
    }
}
impl core::fmt::Debug for STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("SSA", &self.SSA())
            .field("SSD", &self.SSD())
            .field("STALLED", &self.STALLED())
            .field("ENDTRANSFER", &self.ENDTRANSFER())
            .field("MSTIDLE", &self.MSTIDLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STAT {{ SSA: {=bool:?}, SSD: {=bool:?}, STALLED: {=bool:?}, ENDTRANSFER: {=bool:?}, MSTIDLE: {=bool:?} }}",
            self.SSA(),
            self.SSD(),
            self.STALLED(),
            self.ENDTRANSFER(),
            self.MSTIDLE()
        )
    }
}
