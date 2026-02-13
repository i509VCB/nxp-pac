#[doc = "SPI Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "SPI enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPI enable."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Master mode select."]
    #[must_use]
    #[inline(always)]
    pub const fn master(&self) -> super::vals::Master {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Master::from_bits(val as u8)
    }
    #[doc = "Master mode select."]
    #[inline(always)]
    pub const fn set_master(&mut self, val: super::vals::Master) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "LSB First mode enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lsbf(&self) -> super::vals::Lsbf {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Lsbf::from_bits(val as u8)
    }
    #[doc = "LSB First mode enable."]
    #[inline(always)]
    pub const fn set_lsbf(&mut self, val: super::vals::Lsbf) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Clock Phase select."]
    #[must_use]
    #[inline(always)]
    pub const fn cpha(&self) -> super::vals::Cpha {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Cpha::from_bits(val as u8)
    }
    #[doc = "Clock Phase select."]
    #[inline(always)]
    pub const fn set_cpha(&mut self, val: super::vals::Cpha) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Clock Polarity select."]
    #[must_use]
    #[inline(always)]
    pub const fn cpol(&self) -> super::vals::Cpol {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cpol::from_bits(val as u8)
    }
    #[doc = "Clock Polarity select."]
    #[inline(always)]
    pub const fn set_cpol(&mut self, val: super::vals::Cpol) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SSEL0 Polarity select."]
    #[must_use]
    #[inline(always)]
    pub const fn spol0(&self) -> super::vals::Spol0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Spol0::from_bits(val as u8)
    }
    #[doc = "SSEL0 Polarity select."]
    #[inline(always)]
    pub const fn set_spol0(&mut self, val: super::vals::Spol0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "SSEL1 Polarity select."]
    #[must_use]
    #[inline(always)]
    pub const fn spol1(&self) -> super::vals::Spol1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Spol1::from_bits(val as u8)
    }
    #[doc = "SSEL1 Polarity select."]
    #[inline(always)]
    pub const fn set_spol1(&mut self, val: super::vals::Spol1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "SSEL2 Polarity select."]
    #[must_use]
    #[inline(always)]
    pub const fn spol2(&self) -> super::vals::Spol2 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Spol2::from_bits(val as u8)
    }
    #[doc = "SSEL2 Polarity select."]
    #[inline(always)]
    pub const fn set_spol2(&mut self, val: super::vals::Spol2) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "SSEL3 Polarity select."]
    #[must_use]
    #[inline(always)]
    pub const fn spol3(&self) -> super::vals::Spol3 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Spol3::from_bits(val as u8)
    }
    #[doc = "SSEL3 Polarity select."]
    #[inline(always)]
    pub const fn set_spol3(&mut self, val: super::vals::Spol3) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
impl core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg")
            .field("enable", &self.enable())
            .field("master", &self.master())
            .field("lsbf", &self.lsbf())
            .field("cpha", &self.cpha())
            .field("cpol", &self.cpol())
            .field("loop_", &self.loop_())
            .field("spol0", &self.spol0())
            .field("spol1", &self.spol1())
            .field("spol2", &self.spol2())
            .field("spol3", &self.spol3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg {{ enable: {=bool:?}, master: {:?}, lsbf: {:?}, cpha: {:?}, cpol: {:?}, loop_: {=bool:?}, spol0: {:?}, spol1: {:?}, spol2: {:?}, spol3: {:?} }}",
            self.enable(),
            self.master(),
            self.lsbf(),
            self.cpha(),
            self.cpol(),
            self.loop_(),
            self.spol0(),
            self.spol1(),
            self.spol2(),
            self.spol3()
        )
    }
}
#[doc = "SPI clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Div(pub u32);
impl Div {
    #[doc = "Rate divider value. Specifies how the Flexcomm clock (FCLK) is divided to produce the SPI clock rate in master mode. DIVVAL is -1 encoded such that the value 0 results in FCLK/1, the value 1 results in FCLK/2, up to the maximum possible divide value of 0xFFFF, which results in FCLK/65536."]
    #[must_use]
    #[inline(always)]
    pub const fn divval(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Rate divider value. Specifies how the Flexcomm clock (FCLK) is divided to produce the SPI clock rate in master mode. DIVVAL is -1 encoded such that the value 0 results in FCLK/1, the value 1 results in FCLK/2, up to the maximum possible divide value of 0xFFFF, which results in FCLK/65536."]
    #[inline(always)]
    pub const fn set_divval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Div {
    #[inline(always)]
    fn default() -> Div {
        Div(0)
    }
}
impl core::fmt::Debug for Div {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Div")
            .field("divval", &self.divval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Div {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Div {{ divval: {=u16:?} }}", self.divval())
    }
}
#[doc = "SPI Delay register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dly(pub u32);
impl Dly {
    #[doc = "Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[must_use]
    #[inline(always)]
    pub const fn pre_delay(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub const fn set_pre_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[must_use]
    #[inline(always)]
    pub const fn post_delay(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub const fn set_post_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[must_use]
    #[inline(always)]
    pub const fn frame_delay(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub const fn set_frame_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
    #[must_use]
    #[inline(always)]
    pub const fn transfer_delay(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
    #[inline(always)]
    pub const fn set_transfer_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Dly {
    #[inline(always)]
    fn default() -> Dly {
        Dly(0)
    }
}
impl core::fmt::Debug for Dly {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dly")
            .field("pre_delay", &self.pre_delay())
            .field("post_delay", &self.post_delay())
            .field("frame_delay", &self.frame_delay())
            .field("transfer_delay", &self.transfer_delay())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dly {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dly {{ pre_delay: {=u8:?}, post_delay: {=u8:?}, frame_delay: {=u8:?}, transfer_delay: {=u8:?} }}",
            self.pre_delay(),
            self.post_delay(),
            self.frame_delay(),
            self.transfer_delay()
        )
    }
}
#[doc = "FIFO configuration and enable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifocfg(pub u32);
impl Fifocfg {
    #[doc = "Enable the transmit FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn enabletx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the transmit FIFO."]
    #[inline(always)]
    pub const fn set_enabletx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the receive FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn enablerx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the receive FIFO."]
    #[inline(always)]
    pub const fn set_enablerx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO size configuration. This is a read-only field. 0x0 = FIFO is configured as 16 entries of 8 bits. 0x1, 0x2, 0x3 = not applicable to USART."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "FIFO size configuration. This is a read-only field. 0x0 = FIFO is configured as 16 entries of 8 bits. 0x1, 0x2, 0x3 = not applicable to USART."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "DMA configuration for transmit."]
    #[must_use]
    #[inline(always)]
    pub const fn dmatx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA configuration for transmit."]
    #[inline(always)]
    pub const fn set_dmatx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA configuration for receive."]
    #[must_use]
    #[inline(always)]
    pub const fn dmarx(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA configuration for receive."]
    #[inline(always)]
    pub const fn set_dmarx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[must_use]
    #[inline(always)]
    pub const fn waketx(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub const fn set_waketx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[must_use]
    #[inline(always)]
    pub const fn wakerx(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub const fn set_wakerx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
    #[must_use]
    #[inline(always)]
    pub const fn emptytx(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
    #[inline(always)]
    pub const fn set_emptytx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
    #[must_use]
    #[inline(always)]
    pub const fn emptyrx(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
    #[inline(always)]
    pub const fn set_emptyrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Fifocfg {
    #[inline(always)]
    fn default() -> Fifocfg {
        Fifocfg(0)
    }
}
impl core::fmt::Debug for Fifocfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifocfg")
            .field("enabletx", &self.enabletx())
            .field("enablerx", &self.enablerx())
            .field("size", &self.size())
            .field("dmatx", &self.dmatx())
            .field("dmarx", &self.dmarx())
            .field("waketx", &self.waketx())
            .field("wakerx", &self.wakerx())
            .field("emptytx", &self.emptytx())
            .field("emptyrx", &self.emptyrx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifocfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifocfg {{ enabletx: {=bool:?}, enablerx: {=bool:?}, size: {=u8:?}, dmatx: {=bool:?}, dmarx: {=bool:?}, waketx: {=bool:?}, wakerx: {=bool:?}, emptytx: {=bool:?}, emptyrx: {=bool:?} }}",
            self.enabletx(),
            self.enablerx(),
            self.size(),
            self.dmatx(),
            self.dmarx(),
            self.waketx(),
            self.wakerx(),
            self.emptytx(),
            self.emptyrx()
        )
    }
}
#[doc = "FIFO interrupt enable clear (disable) and read register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifointenclr(pub u32);
impl Fifointenclr {
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn txerr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub const fn set_txerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn rxerr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub const fn set_rxerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn txlvl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub const fn set_txlvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn rxlvl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub const fn set_rxlvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Fifointenclr {
    #[inline(always)]
    fn default() -> Fifointenclr {
        Fifointenclr(0)
    }
}
impl core::fmt::Debug for Fifointenclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifointenclr")
            .field("txerr", &self.txerr())
            .field("rxerr", &self.rxerr())
            .field("txlvl", &self.txlvl())
            .field("rxlvl", &self.rxlvl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifointenclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifointenclr {{ txerr: {=bool:?}, rxerr: {=bool:?}, txlvl: {=bool:?}, rxlvl: {=bool:?} }}",
            self.txerr(),
            self.rxerr(),
            self.txlvl(),
            self.rxlvl()
        )
    }
}
#[doc = "FIFO interrupt enable set (enable) and read register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifointenset(pub u32);
impl Fifointenset {
    #[doc = "Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register."]
    #[must_use]
    #[inline(always)]
    pub const fn txerr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub const fn set_txerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register."]
    #[must_use]
    #[inline(always)]
    pub const fn rxerr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub const fn set_rxerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[must_use]
    #[inline(always)]
    pub const fn txlvl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub const fn set_txlvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[must_use]
    #[inline(always)]
    pub const fn rxlvl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub const fn set_rxlvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Fifointenset {
    #[inline(always)]
    fn default() -> Fifointenset {
        Fifointenset(0)
    }
}
impl core::fmt::Debug for Fifointenset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifointenset")
            .field("txerr", &self.txerr())
            .field("rxerr", &self.rxerr())
            .field("txlvl", &self.txlvl())
            .field("rxlvl", &self.rxlvl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifointenset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifointenset {{ txerr: {=bool:?}, rxerr: {=bool:?}, txlvl: {=bool:?}, rxlvl: {=bool:?} }}",
            self.txerr(),
            self.rxerr(),
            self.txlvl(),
            self.rxlvl()
        )
    }
}
#[doc = "FIFO interrupt status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifointstat(pub u32);
impl Fifointstat {
    #[doc = "TX FIFO error."]
    #[must_use]
    #[inline(always)]
    pub const fn txerr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TX FIFO error."]
    #[inline(always)]
    pub const fn set_txerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX FIFO error."]
    #[must_use]
    #[inline(always)]
    pub const fn rxerr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO error."]
    #[inline(always)]
    pub const fn set_rxerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit FIFO level interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn txlvl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO level interrupt."]
    #[inline(always)]
    pub const fn set_txlvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive FIFO level interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rxlvl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO level interrupt."]
    #[inline(always)]
    pub const fn set_rxlvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Peripheral interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn perint(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral interrupt."]
    #[inline(always)]
    pub const fn set_perint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Fifointstat {
    #[inline(always)]
    fn default() -> Fifointstat {
        Fifointstat(0)
    }
}
impl core::fmt::Debug for Fifointstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifointstat")
            .field("txerr", &self.txerr())
            .field("rxerr", &self.rxerr())
            .field("txlvl", &self.txlvl())
            .field("rxlvl", &self.rxlvl())
            .field("perint", &self.perint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifointstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifointstat {{ txerr: {=bool:?}, rxerr: {=bool:?}, txlvl: {=bool:?}, rxlvl: {=bool:?}, perint: {=bool:?} }}",
            self.txerr(),
            self.rxerr(),
            self.txlvl(),
            self.rxlvl(),
            self.perint()
        )
    }
}
#[doc = "FIFO read data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fiford(pub u32);
impl Fiford {
    #[doc = "Received data from the FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Received data from the FIFO."]
    #[inline(always)]
    pub const fn set_rxdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL0 pin to be saved along with received data. The value will reflect the SSEL0 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[must_use]
    #[inline(always)]
    pub const fn rxssel0_n(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL0 pin to be saved along with received data. The value will reflect the SSEL0 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub const fn set_rxssel0_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL1 pin to be saved along with received data. The value will reflect the SSEL1 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[must_use]
    #[inline(always)]
    pub const fn rxssel1_n(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL1 pin to be saved along with received data. The value will reflect the SSEL1 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub const fn set_rxssel1_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL2 pin to be saved along with received data. The value will reflect the SSEL2 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[must_use]
    #[inline(always)]
    pub const fn rxssel2_n(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL2 pin to be saved along with received data. The value will reflect the SSEL2 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub const fn set_rxssel2_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL3 pin to be saved along with received data. The value will reflect the SSEL3 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[must_use]
    #[inline(always)]
    pub const fn rxssel3_n(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL3 pin to be saved along with received data. The value will reflect the SSEL3 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub const fn set_rxssel3_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Start of Transfer flag. This flag will be 1 if this is the first data after the SSELs went from deasserted to asserted (i.e., any previous transfer has ended). This information can be used to identify the first piece of data in cases where the transfer length is greater than 16 bits."]
    #[must_use]
    #[inline(always)]
    pub const fn sot(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Transfer flag. This flag will be 1 if this is the first data after the SSELs went from deasserted to asserted (i.e., any previous transfer has ended). This information can be used to identify the first piece of data in cases where the transfer length is greater than 16 bits."]
    #[inline(always)]
    pub const fn set_sot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Fiford {
    #[inline(always)]
    fn default() -> Fiford {
        Fiford(0)
    }
}
impl core::fmt::Debug for Fiford {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fiford")
            .field("rxdata", &self.rxdata())
            .field("rxssel0_n", &self.rxssel0_n())
            .field("rxssel1_n", &self.rxssel1_n())
            .field("rxssel2_n", &self.rxssel2_n())
            .field("rxssel3_n", &self.rxssel3_n())
            .field("sot", &self.sot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fiford {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fiford {{ rxdata: {=u16:?}, rxssel0_n: {=bool:?}, rxssel1_n: {=bool:?}, rxssel2_n: {=bool:?}, rxssel3_n: {=bool:?}, sot: {=bool:?} }}",
            self.rxdata(),
            self.rxssel0_n(),
            self.rxssel1_n(),
            self.rxssel2_n(),
            self.rxssel3_n(),
            self.sot()
        )
    }
}
#[doc = "FIFO data read with no FIFO pop."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifordnopop(pub u32);
impl Fifordnopop {
    #[doc = "Received data from the FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Received data from the FIFO."]
    #[inline(always)]
    pub const fn set_rxdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Slave Select for receive."]
    #[must_use]
    #[inline(always)]
    pub const fn rxssel0_n(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive."]
    #[inline(always)]
    pub const fn set_rxssel0_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Slave Select for receive."]
    #[must_use]
    #[inline(always)]
    pub const fn rxssel1_n(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive."]
    #[inline(always)]
    pub const fn set_rxssel1_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Slave Select for receive."]
    #[must_use]
    #[inline(always)]
    pub const fn rxssel2_n(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive."]
    #[inline(always)]
    pub const fn set_rxssel2_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Slave Select for receive."]
    #[must_use]
    #[inline(always)]
    pub const fn rxssel3_n(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive."]
    #[inline(always)]
    pub const fn set_rxssel3_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Start of transfer flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sot(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Start of transfer flag."]
    #[inline(always)]
    pub const fn set_sot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Fifordnopop {
    #[inline(always)]
    fn default() -> Fifordnopop {
        Fifordnopop(0)
    }
}
impl core::fmt::Debug for Fifordnopop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifordnopop")
            .field("rxdata", &self.rxdata())
            .field("rxssel0_n", &self.rxssel0_n())
            .field("rxssel1_n", &self.rxssel1_n())
            .field("rxssel2_n", &self.rxssel2_n())
            .field("rxssel3_n", &self.rxssel3_n())
            .field("sot", &self.sot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifordnopop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifordnopop {{ rxdata: {=u16:?}, rxssel0_n: {=bool:?}, rxssel1_n: {=bool:?}, rxssel2_n: {=bool:?}, rxssel3_n: {=bool:?}, sot: {=bool:?} }}",
            self.rxdata(),
            self.rxssel0_n(),
            self.rxssel1_n(),
            self.rxssel2_n(),
            self.rxssel3_n(),
            self.sot()
        )
    }
}
#[doc = "FIFO size register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifosize(pub u32);
impl Fifosize {
    #[doc = "Provides the size of the FIFO for software. The size of the SPI FIFO is 8 entries."]
    #[must_use]
    #[inline(always)]
    pub const fn fifosize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Provides the size of the FIFO for software. The size of the SPI FIFO is 8 entries."]
    #[inline(always)]
    pub const fn set_fifosize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Fifosize {
    #[inline(always)]
    fn default() -> Fifosize {
        Fifosize(0)
    }
}
impl core::fmt::Debug for Fifosize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifosize")
            .field("fifosize", &self.fifosize())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifosize {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fifosize {{ fifosize: {=u8:?} }}", self.fifosize())
    }
}
#[doc = "FIFO status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifostat(pub u32);
impl Fifostat {
    #[doc = "TX FIFO error. Will be set if a transmit FIFO error occurs. This could be an overflow caused by pushing data into a full FIFO, or by an underflow if the FIFO is empty when data is needed. Cleared by writing a 1 to this bit."]
    #[must_use]
    #[inline(always)]
    pub const fn txerr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TX FIFO error. Will be set if a transmit FIFO error occurs. This could be an overflow caused by pushing data into a full FIFO, or by an underflow if the FIFO is empty when data is needed. Cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub const fn set_txerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX FIFO error. Will be set if a receive FIFO overflow occurs, caused by software or DMA not emptying the FIFO fast enough. Cleared by writing a 1 to this bit."]
    #[must_use]
    #[inline(always)]
    pub const fn rxerr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO error. Will be set if a receive FIFO overflow occurs, caused by software or DMA not emptying the FIFO fast enough. Cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub const fn set_rxerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Peripheral interrupt. When 1, this indicates that the peripheral function has asserted an interrupt. The details can be found by reading the peripheral's STAT register."]
    #[must_use]
    #[inline(always)]
    pub const fn perint(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral interrupt. When 1, this indicates that the peripheral function has asserted an interrupt. The details can be found by reading the peripheral's STAT register."]
    #[inline(always)]
    pub const fn set_perint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit FIFO empty. When 1, the transmit FIFO is empty. The peripheral may still be processing the last piece of data."]
    #[must_use]
    #[inline(always)]
    pub const fn txempty(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty. When 1, the transmit FIFO is empty. The peripheral may still be processing the last piece of data."]
    #[inline(always)]
    pub const fn set_txempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit FIFO not full. When 1, the transmit FIFO is not full, so more data can be written. When 0, the transmit FIFO is full and another write would cause it to overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn txnotfull(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO not full. When 1, the transmit FIFO is not full, so more data can be written. When 0, the transmit FIFO is full and another write would cause it to overflow."]
    #[inline(always)]
    pub const fn set_txnotfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive FIFO not empty. When 1, the receive FIFO is not empty, so data can be read. When 0, the receive FIFO is empty."]
    #[must_use]
    #[inline(always)]
    pub const fn rxnotempty(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO not empty. When 1, the receive FIFO is not empty, so data can be read. When 0, the receive FIFO is empty."]
    #[inline(always)]
    pub const fn set_rxnotempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Receive FIFO full. When 1, the receive FIFO is full. Data needs to be read out to prevent the peripheral from causing an overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfull(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full. When 1, the receive FIFO is full. Data needs to be read out to prevent the peripheral from causing an overflow."]
    #[inline(always)]
    pub const fn set_rxfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Transmit FIFO current level. A 0 means the TX FIFO is currently empty, and the TXEMPTY and TXNOTFULL flags will be 1. Other values tell how much data is actually in the TX FIFO at the point where the read occurs. If the TX FIFO is full, the TXEMPTY and TXNOTFULL flags will be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn txlvl(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Transmit FIFO current level. A 0 means the TX FIFO is currently empty, and the TXEMPTY and TXNOTFULL flags will be 1. Other values tell how much data is actually in the TX FIFO at the point where the read occurs. If the TX FIFO is full, the TXEMPTY and TXNOTFULL flags will be 0."]
    #[inline(always)]
    pub const fn set_txlvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Receive FIFO current level. A 0 means the RX FIFO is currently empty, and the RXFULL and RXNOTEMPTY flags will be 0. Other values tell how much data is actually in the RX FIFO at the point where the read occurs. If the RX FIFO is full, the RXFULL and RXNOTEMPTY flags will be 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rxlvl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Receive FIFO current level. A 0 means the RX FIFO is currently empty, and the RXFULL and RXNOTEMPTY flags will be 0. Other values tell how much data is actually in the RX FIFO at the point where the read occurs. If the RX FIFO is full, the RXFULL and RXNOTEMPTY flags will be 1."]
    #[inline(always)]
    pub const fn set_rxlvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Fifostat {
    #[inline(always)]
    fn default() -> Fifostat {
        Fifostat(0)
    }
}
impl core::fmt::Debug for Fifostat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifostat")
            .field("txerr", &self.txerr())
            .field("rxerr", &self.rxerr())
            .field("perint", &self.perint())
            .field("txempty", &self.txempty())
            .field("txnotfull", &self.txnotfull())
            .field("rxnotempty", &self.rxnotempty())
            .field("rxfull", &self.rxfull())
            .field("txlvl", &self.txlvl())
            .field("rxlvl", &self.rxlvl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifostat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifostat {{ txerr: {=bool:?}, rxerr: {=bool:?}, perint: {=bool:?}, txempty: {=bool:?}, txnotfull: {=bool:?}, rxnotempty: {=bool:?}, rxfull: {=bool:?}, txlvl: {=u8:?}, rxlvl: {=u8:?} }}",
            self.txerr(),
            self.rxerr(),
            self.perint(),
            self.txempty(),
            self.txnotfull(),
            self.rxnotempty(),
            self.rxfull(),
            self.txlvl(),
            self.rxlvl()
        )
    }
}
#[doc = "FIFO trigger settings for interrupt and DMA request."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifotrig(pub u32);
impl Fifotrig {
    #[doc = "Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
    #[must_use]
    #[inline(always)]
    pub const fn txlvlena(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
    #[inline(always)]
    pub const fn set_txlvlena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
    #[must_use]
    #[inline(always)]
    pub const fn rxlvlena(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
    #[inline(always)]
    pub const fn set_rxlvlena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    #[must_use]
    #[inline(always)]
    pub const fn txlvl(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    #[inline(always)]
    pub const fn set_txlvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
    #[must_use]
    #[inline(always)]
    pub const fn rxlvl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    pub const fn set_rxlvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Fifotrig {
    #[inline(always)]
    fn default() -> Fifotrig {
        Fifotrig(0)
    }
}
impl core::fmt::Debug for Fifotrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifotrig")
            .field("txlvlena", &self.txlvlena())
            .field("rxlvlena", &self.rxlvlena())
            .field("txlvl", &self.txlvl())
            .field("rxlvl", &self.rxlvl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifotrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifotrig {{ txlvlena: {=bool:?}, rxlvlena: {=bool:?}, txlvl: {=u8:?}, rxlvl: {=u8:?} }}",
            self.txlvlena(),
            self.rxlvlena(),
            self.txlvl(),
            self.rxlvl()
        )
    }
}
#[doc = "FIFO write data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifowr(pub u32);
impl Fifowr {
    #[doc = "Transmit data to the FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn txdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit data to the FIFO."]
    #[inline(always)]
    pub const fn set_txdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Transmit slave select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default."]
    #[must_use]
    #[inline(always)]
    pub const fn txssel0_n(&self) -> super::vals::Txssel0N {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Txssel0N::from_bits(val as u8)
    }
    #[doc = "Transmit slave select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub const fn set_txssel0_n(&mut self, val: super::vals::Txssel0N) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Transmit slave select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default."]
    #[must_use]
    #[inline(always)]
    pub const fn txssel1_n(&self) -> super::vals::Txssel1N {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Txssel1N::from_bits(val as u8)
    }
    #[doc = "Transmit slave select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub const fn set_txssel1_n(&mut self, val: super::vals::Txssel1N) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Transmit slave select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default."]
    #[must_use]
    #[inline(always)]
    pub const fn txssel2_n(&self) -> super::vals::Txssel2N {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Txssel2N::from_bits(val as u8)
    }
    #[doc = "Transmit slave select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub const fn set_txssel2_n(&mut self, val: super::vals::Txssel2N) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Transmit slave select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default."]
    #[must_use]
    #[inline(always)]
    pub const fn txssel3_n(&self) -> super::vals::Txssel3N {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Txssel3N::from_bits(val as u8)
    }
    #[doc = "Transmit slave select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub const fn set_txssel3_n(&mut self, val: super::vals::Txssel3N) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "End of transfer. The asserted SSEL will be deasserted at the end of a transfer and remain so far at least the time specified by the Transfer_delay value in the DLY register."]
    #[must_use]
    #[inline(always)]
    pub const fn eot(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "End of transfer. The asserted SSEL will be deasserted at the end of a transfer and remain so far at least the time specified by the Transfer_delay value in the DLY register."]
    #[inline(always)]
    pub const fn set_eot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "End of frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the Frame_delay value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
    #[must_use]
    #[inline(always)]
    pub const fn eof(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "End of frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the Frame_delay value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
    #[inline(always)]
    pub const fn set_eof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn rxignore(&self) -> super::vals::Rxignore {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Rxignore::from_bits(val as u8)
    }
    #[doc = "Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA."]
    #[inline(always)]
    pub const fn set_rxignore(&mut self, val: super::vals::Rxignore) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Data Length. Specifies the data length from 4 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0-2 = Reserved. 0x3 = Data transfer is 4 bits in length. 0x4 = Data transfer is 5 bits in length. 0xF = Data transfer is 16 bits in length."]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Length. Specifies the data length from 4 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0-2 = Reserved. 0x3 = Data transfer is 4 bits in length. 0x4 = Data transfer is 5 bits in length. 0xF = Data transfer is 16 bits in length."]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Fifowr {
    #[inline(always)]
    fn default() -> Fifowr {
        Fifowr(0)
    }
}
impl core::fmt::Debug for Fifowr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifowr")
            .field("txdata", &self.txdata())
            .field("txssel0_n", &self.txssel0_n())
            .field("txssel1_n", &self.txssel1_n())
            .field("txssel2_n", &self.txssel2_n())
            .field("txssel3_n", &self.txssel3_n())
            .field("eot", &self.eot())
            .field("eof", &self.eof())
            .field("rxignore", &self.rxignore())
            .field("len", &self.len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifowr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifowr {{ txdata: {=u16:?}, txssel0_n: {:?}, txssel1_n: {:?}, txssel2_n: {:?}, txssel3_n: {:?}, eot: {=bool:?}, eof: {=bool:?}, rxignore: {:?}, len: {=u8:?} }}",
            self.txdata(),
            self.txssel0_n(),
            self.txssel1_n(),
            self.txssel2_n(),
            self.txssel3_n(),
            self.eot(),
            self.eof(),
            self.rxignore(),
            self.len()
        )
    }
}
#[doc = "Peripheral identification register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id(pub u32);
impl Id {
    #[doc = "Aperture: encoded as (aperture size/4K) -1, so 0x00 means a 4K aperture."]
    #[must_use]
    #[inline(always)]
    pub const fn aperture(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Aperture: encoded as (aperture size/4K) -1, so 0x00 means a 4K aperture."]
    #[inline(always)]
    pub const fn set_aperture(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Minor revision of module implementation."]
    #[must_use]
    #[inline(always)]
    pub const fn minor_rev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision of module implementation."]
    #[inline(always)]
    pub const fn set_minor_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Major revision of module implementation."]
    #[must_use]
    #[inline(always)]
    pub const fn major_rev(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision of module implementation."]
    #[inline(always)]
    pub const fn set_major_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Module identifier for the selected function."]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Module identifier for the selected function."]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Id {
    #[inline(always)]
    fn default() -> Id {
        Id(0)
    }
}
impl core::fmt::Debug for Id {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Id")
            .field("aperture", &self.aperture())
            .field("minor_rev", &self.minor_rev())
            .field("major_rev", &self.major_rev())
            .field("id", &self.id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Id {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Id {{ aperture: {=u8:?}, minor_rev: {=u8:?}, major_rev: {=u8:?}, id: {=u16:?} }}",
            self.aperture(),
            self.minor_rev(),
            self.major_rev(),
            self.id()
        )
    }
}
#[doc = "SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenclr(pub u32);
impl Intenclr {
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn ssaen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_ssaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn ssden(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_ssden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn mstidle(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_mstidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Intenclr {
    #[inline(always)]
    fn default() -> Intenclr {
        Intenclr(0)
    }
}
impl core::fmt::Debug for Intenclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intenclr")
            .field("ssaen", &self.ssaen())
            .field("ssden", &self.ssden())
            .field("mstidle", &self.mstidle())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intenclr {{ ssaen: {=bool:?}, ssden: {=bool:?}, mstidle: {=bool:?} }}",
            self.ssaen(),
            self.ssden(),
            self.mstidle()
        )
    }
}
#[doc = "SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenset(pub u32);
impl Intenset {
    #[doc = "Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn ssaen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline(always)]
    pub const fn set_ssaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[must_use]
    #[inline(always)]
    pub const fn ssden(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline(always)]
    pub const fn set_ssden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Master idle interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mstidleen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Master idle interrupt enable."]
    #[inline(always)]
    pub const fn set_mstidleen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Intenset {
    #[inline(always)]
    fn default() -> Intenset {
        Intenset(0)
    }
}
impl core::fmt::Debug for Intenset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intenset")
            .field("ssaen", &self.ssaen())
            .field("ssden", &self.ssden())
            .field("mstidleen", &self.mstidleen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intenset {{ ssaen: {=bool:?}, ssden: {=bool:?}, mstidleen: {=bool:?} }}",
            self.ssaen(),
            self.ssden(),
            self.mstidleen()
        )
    }
}
#[doc = "SPI Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Slave Select Assert."]
    #[must_use]
    #[inline(always)]
    pub const fn ssa(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select Assert."]
    #[inline(always)]
    pub const fn set_ssa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Slave Select Deassert."]
    #[must_use]
    #[inline(always)]
    pub const fn ssd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select Deassert."]
    #[inline(always)]
    pub const fn set_ssd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Master Idle status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn mstidle(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Master Idle status flag."]
    #[inline(always)]
    pub const fn set_mstidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Intstat {
    #[inline(always)]
    fn default() -> Intstat {
        Intstat(0)
    }
}
impl core::fmt::Debug for Intstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intstat")
            .field("ssa", &self.ssa())
            .field("ssd", &self.ssd())
            .field("mstidle", &self.mstidle())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intstat {{ ssa: {=bool:?}, ssd: {=bool:?}, mstidle: {=bool:?} }}",
            self.ssa(),
            self.ssd(),
            self.mstidle()
        )
    }
}
#[doc = "SPI Status. Some status flags can be cleared by writing a 1 to that bit position."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Slave Select Assert. This flag is set whenever any slave select transitions from deasserted to asserted, in both master and slave modes. This allows determining when the SPI transmit/receive functions become busy, and allows waking up the device from reduced power modes when a slave mode access begins. This flag is cleared by software."]
    #[must_use]
    #[inline(always)]
    pub const fn ssa(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select Assert. This flag is set whenever any slave select transitions from deasserted to asserted, in both master and slave modes. This allows determining when the SPI transmit/receive functions become busy, and allows waking up the device from reduced power modes when a slave mode access begins. This flag is cleared by software."]
    #[inline(always)]
    pub const fn set_ssa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Slave Select Deassert. This flag is set whenever any asserted slave selects transition to deasserted, in both master and slave modes. This allows determining when the SPI transmit/receive functions become idle. This flag is cleared by software."]
    #[must_use]
    #[inline(always)]
    pub const fn ssd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select Deassert. This flag is set whenever any asserted slave selects transition to deasserted, in both master and slave modes. This allows determining when the SPI transmit/receive functions become idle. This flag is cleared by software."]
    #[inline(always)]
    pub const fn set_ssd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Stalled status flag. This indicates whether the SPI is currently in a stall condition."]
    #[must_use]
    #[inline(always)]
    pub const fn stalled(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Stalled status flag. This indicates whether the SPI is currently in a stall condition."]
    #[inline(always)]
    pub const fn set_stalled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "End Transfer control bit. Software can set this bit to force an end to the current transfer when the transmitter finishes any activity already in progress, as if the EOT flag had been set prior to the last transmission. This capability is included to support cases where it is not known when transmit data is written that it will be the end of a transfer. The bit is cleared when the transmitter becomes idle as the transfer comes to an end. Forcing an end of transfer in this manner causes any specified FRAME_DELAY and TRANSFER_DELAY to be inserted."]
    #[must_use]
    #[inline(always)]
    pub const fn endtransfer(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "End Transfer control bit. Software can set this bit to force an end to the current transfer when the transmitter finishes any activity already in progress, as if the EOT flag had been set prior to the last transmission. This capability is included to support cases where it is not known when transmit data is written that it will be the end of a transfer. The bit is cleared when the transmitter becomes idle as the transfer comes to an end. Forcing an end of transfer in this manner causes any specified FRAME_DELAY and TRANSFER_DELAY to be inserted."]
    #[inline(always)]
    pub const fn set_endtransfer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Master idle status flag. This bit is 1 whenever the SPI master function is fully idle. This means that the transmit holding register is empty and the transmitter is not in the process of sending data."]
    #[must_use]
    #[inline(always)]
    pub const fn mstidle(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Master idle status flag. This bit is 1 whenever the SPI master function is fully idle. This means that the transmit holding register is empty and the transmitter is not in the process of sending data."]
    #[inline(always)]
    pub const fn set_mstidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
impl core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat")
            .field("ssa", &self.ssa())
            .field("ssd", &self.ssd())
            .field("stalled", &self.stalled())
            .field("endtransfer", &self.endtransfer())
            .field("mstidle", &self.mstidle())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stat {{ ssa: {=bool:?}, ssd: {=bool:?}, stalled: {=bool:?}, endtransfer: {=bool:?}, mstidle: {=bool:?} }}",
            self.ssa(),
            self.ssd(),
            self.stalled(),
            self.endtransfer(),
            self.mstidle()
        )
    }
}
