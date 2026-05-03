#[doc = "Address register for automatic address matching."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADDR(pub u32);
impl ADDR {
    #[doc = "8-bit address used with automatic address matching. Used when address detection is enabled (ADDRDET in CTL = 1) and automatic address matching is enabled (AUTOADDR in CFG = 1)."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDRESS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit address used with automatic address matching. Used when address detection is enabled (ADDRDET in CTL = 1) and automatic address matching is enabled (AUTOADDR in CFG = 1)."]
    #[inline(always)]
    pub const fn set_ADDRESS(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ADDR {
    #[inline(always)]
    fn default() -> ADDR {
        ADDR(0)
    }
}
impl core::fmt::Debug for ADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR")
            .field("ADDRESS", &self.ADDRESS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADDR {{ ADDRESS: {=u8:?} }}", self.ADDRESS())
    }
}
#[doc = "Baud Rate Generator register. 16-bit integer baud rate divisor value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BRG(pub u32);
impl BRG {
    #[doc = "This value is used to divide the USART input clock to determine the baud rate, based on the input clock from the FRG. 0 = FCLK is used directly by the USART function. 1 = FCLK is divided by 2 before use by the USART function. 2 = FCLK is divided by 3 before use by the USART function. 0xFFFF = FCLK is divided by 65,536 before use by the USART function."]
    #[must_use]
    #[inline(always)]
    pub const fn BRGVAL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This value is used to divide the USART input clock to determine the baud rate, based on the input clock from the FRG. 0 = FCLK is used directly by the USART function. 1 = FCLK is divided by 2 before use by the USART function. 2 = FCLK is divided by 3 before use by the USART function. 0xFFFF = FCLK is divided by 65,536 before use by the USART function."]
    #[inline(always)]
    pub const fn set_BRGVAL(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for BRG {
    #[inline(always)]
    fn default() -> BRG {
        BRG(0)
    }
}
impl core::fmt::Debug for BRG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRG")
            .field("BRGVAL", &self.BRGVAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BRG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BRG {{ BRGVAL: {=u16:?} }}", self.BRGVAL())
    }
}
#[doc = "USART Configuration register. Basic USART configuration settings that typically are not changed during operation."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFG(pub u32);
impl CFG {
    #[doc = "USART Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USART Enable."]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Selects the data size for the USART."]
    #[must_use]
    #[inline(always)]
    pub const fn DATALEN(&self) -> super::vals::DATALEN {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DATALEN::from_bits(val as u8)
    }
    #[doc = "Selects the data size for the USART."]
    #[inline(always)]
    pub const fn set_DATALEN(&mut self, val: super::vals::DATALEN) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Selects what type of parity is used by the USART."]
    #[must_use]
    #[inline(always)]
    pub const fn PARITYSEL(&self) -> super::vals::PARITYSEL {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PARITYSEL::from_bits(val as u8)
    }
    #[doc = "Selects what type of parity is used by the USART."]
    #[inline(always)]
    pub const fn set_PARITYSEL(&mut self, val: super::vals::PARITYSEL) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
    #[must_use]
    #[inline(always)]
    pub const fn STOPLEN(&self) -> super::vals::STOPLEN {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::STOPLEN::from_bits(val as u8)
    }
    #[doc = "Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
    #[inline(always)]
    pub const fn set_STOPLEN(&mut self, val: super::vals::STOPLEN) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Selects standard or 32 kHz clocking mode."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE32K(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Selects standard or 32 kHz clocking mode."]
    #[inline(always)]
    pub const fn set_MODE32K(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LIN break mode enable."]
    #[must_use]
    #[inline(always)]
    pub const fn LINMODE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LIN break mode enable."]
    #[inline(always)]
    pub const fn set_LINMODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CTSEN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled."]
    #[inline(always)]
    pub const fn set_CTSEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Selects synchronous or asynchronous operation."]
    #[must_use]
    #[inline(always)]
    pub const fn SYNCEN(&self) -> super::vals::SYNCEN {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SYNCEN::from_bits(val as u8)
    }
    #[doc = "Selects synchronous or asynchronous operation."]
    #[inline(always)]
    pub const fn set_SYNCEN(&mut self, val: super::vals::SYNCEN) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Selects the clock polarity and sampling edge of received data in synchronous mode."]
    #[must_use]
    #[inline(always)]
    pub const fn CLKPOL(&self) -> super::vals::CLKPOL {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::CLKPOL::from_bits(val as u8)
    }
    #[doc = "Selects the clock polarity and sampling edge of received data in synchronous mode."]
    #[inline(always)]
    pub const fn set_CLKPOL(&mut self, val: super::vals::CLKPOL) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Synchronous mode Master select."]
    #[must_use]
    #[inline(always)]
    pub const fn SYNCMST(&self) -> super::vals::SYNCMST {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::SYNCMST::from_bits(val as u8)
    }
    #[doc = "Synchronous mode Master select."]
    #[inline(always)]
    pub const fn set_SYNCMST(&mut self, val: super::vals::SYNCMST) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Selects data loopback mode."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOP(&self) -> super::vals::LOOP {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::LOOP::from_bits(val as u8)
    }
    #[doc = "Selects data loopback mode."]
    #[inline(always)]
    pub const fn set_LOOP(&mut self, val: super::vals::LOOP) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Output Enable Turnaround time enable for RS-485 operation."]
    #[must_use]
    #[inline(always)]
    pub const fn OETA(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Output Enable Turnaround time enable for RS-485 operation."]
    #[inline(always)]
    pub const fn set_OETA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Automatic Address matching enable."]
    #[must_use]
    #[inline(always)]
    pub const fn AUTOADDR(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic Address matching enable."]
    #[inline(always)]
    pub const fn set_AUTOADDR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Output Enable Select."]
    #[must_use]
    #[inline(always)]
    pub const fn OESEL(&self) -> super::vals::OESEL {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::OESEL::from_bits(val as u8)
    }
    #[doc = "Output Enable Select."]
    #[inline(always)]
    pub const fn set_OESEL(&mut self, val: super::vals::OESEL) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Output Enable Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn OEPOL(&self) -> super::vals::OEPOL {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::OEPOL::from_bits(val as u8)
    }
    #[doc = "Output Enable Polarity."]
    #[inline(always)]
    pub const fn set_OEPOL(&mut self, val: super::vals::OEPOL) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Receive data polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPOL(&self) -> super::vals::RXPOL {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::RXPOL::from_bits(val as u8)
    }
    #[doc = "Receive data polarity."]
    #[inline(always)]
    pub const fn set_RXPOL(&mut self, val: super::vals::RXPOL) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Transmit data polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPOL(&self) -> super::vals::TXPOL {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::TXPOL::from_bits(val as u8)
    }
    #[doc = "Transmit data polarity."]
    #[inline(always)]
    pub const fn set_TXPOL(&mut self, val: super::vals::TXPOL) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
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
            .field("DATALEN", &self.DATALEN())
            .field("PARITYSEL", &self.PARITYSEL())
            .field("STOPLEN", &self.STOPLEN())
            .field("MODE32K", &self.MODE32K())
            .field("LINMODE", &self.LINMODE())
            .field("CTSEN", &self.CTSEN())
            .field("SYNCEN", &self.SYNCEN())
            .field("CLKPOL", &self.CLKPOL())
            .field("SYNCMST", &self.SYNCMST())
            .field("LOOP", &self.LOOP())
            .field("OETA", &self.OETA())
            .field("AUTOADDR", &self.AUTOADDR())
            .field("OESEL", &self.OESEL())
            .field("OEPOL", &self.OEPOL())
            .field("RXPOL", &self.RXPOL())
            .field("TXPOL", &self.TXPOL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CFG {{ ENABLE: {=bool:?}, DATALEN: {:?}, PARITYSEL: {:?}, STOPLEN: {:?}, MODE32K: {=bool:?}, LINMODE: {=bool:?}, CTSEN: {=bool:?}, SYNCEN: {:?}, CLKPOL: {:?}, SYNCMST: {:?}, LOOP: {:?}, OETA: {=bool:?}, AUTOADDR: {=bool:?}, OESEL: {:?}, OEPOL: {:?}, RXPOL: {:?}, TXPOL: {:?} }}",
            self.ENABLE(),
            self.DATALEN(),
            self.PARITYSEL(),
            self.STOPLEN(),
            self.MODE32K(),
            self.LINMODE(),
            self.CTSEN(),
            self.SYNCEN(),
            self.CLKPOL(),
            self.SYNCMST(),
            self.LOOP(),
            self.OETA(),
            self.AUTOADDR(),
            self.OESEL(),
            self.OEPOL(),
            self.RXPOL(),
            self.TXPOL()
        )
    }
}
#[doc = "USART Control register. USART control settings that are more likely to change during operation."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTL(pub u32);
impl CTL {
    #[doc = "Break Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TXBRKEN(&self) -> super::vals::TXBRKEN {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::TXBRKEN::from_bits(val as u8)
    }
    #[doc = "Break Enable."]
    #[inline(always)]
    pub const fn set_TXBRKEN(&mut self, val: super::vals::TXBRKEN) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable address detect mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDRDET(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable address detect mode."]
    #[inline(always)]
    pub const fn set_ADDRDET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn TXDIS(&self) -> super::vals::TXDIS {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::TXDIS::from_bits(val as u8)
    }
    #[doc = "Transmit Disable."]
    #[inline(always)]
    pub const fn set_TXDIS(&mut self, val: super::vals::TXDIS) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
    #[must_use]
    #[inline(always)]
    pub const fn CC(&self) -> super::vals::CC {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::CC::from_bits(val as u8)
    }
    #[doc = "Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
    #[inline(always)]
    pub const fn set_CC(&mut self, val: super::vals::CC) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Clear Continuous Clock."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRCCONRX(&self) -> super::vals::CLRCCONRX {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::CLRCCONRX::from_bits(val as u8)
    }
    #[doc = "Clear Continuous Clock."]
    #[inline(always)]
    pub const fn set_CLRCCONRX(&mut self, val: super::vals::CLRCCONRX) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Autobaud enable."]
    #[must_use]
    #[inline(always)]
    pub const fn AUTOBAUD(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Autobaud enable."]
    #[inline(always)]
    pub const fn set_AUTOBAUD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for CTL {
    #[inline(always)]
    fn default() -> CTL {
        CTL(0)
    }
}
impl core::fmt::Debug for CTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL")
            .field("TXBRKEN", &self.TXBRKEN())
            .field("ADDRDET", &self.ADDRDET())
            .field("TXDIS", &self.TXDIS())
            .field("CC", &self.CC())
            .field("CLRCCONRX", &self.CLRCCONRX())
            .field("AUTOBAUD", &self.AUTOBAUD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTL {{ TXBRKEN: {:?}, ADDRDET: {=bool:?}, TXDIS: {:?}, CC: {:?}, CLRCCONRX: {:?}, AUTOBAUD: {=bool:?} }}",
            self.TXBRKEN(),
            self.ADDRDET(),
            self.TXDIS(),
            self.CC(),
            self.CLRCCONRX(),
            self.AUTOBAUD()
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
    #[doc = "Received data from the FIFO. The number of bits used depends on the DATALEN and PARITYSEL settings."]
    #[must_use]
    #[inline(always)]
    pub const fn RXDATA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Received data from the FIFO. The number of bits used depends on the DATALEN and PARITYSEL settings."]
    #[inline(always)]
    pub const fn set_RXDATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Framing Error status flag. This bit reflects the status for the data it is read along with from the FIFO, and indicates that the character was received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[must_use]
    #[inline(always)]
    pub const fn FRAMERR(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error status flag. This bit reflects the status for the data it is read along with from the FIFO, and indicates that the character was received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[inline(always)]
    pub const fn set_FRAMERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Parity Error status flag. This bit reflects the status for the data it is read along with from the FIFO. This bit will be set when a parity error is detected in a received character."]
    #[must_use]
    #[inline(always)]
    pub const fn PARITYERR(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error status flag. This bit reflects the status for the data it is read along with from the FIFO. This bit will be set when a parity error is detected in a received character."]
    #[inline(always)]
    pub const fn set_PARITYERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Received Noise flag. See description of the RxNoiseInt bit in Table 354."]
    #[must_use]
    #[inline(always)]
    pub const fn RXNOISE(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Received Noise flag. See description of the RxNoiseInt bit in Table 354."]
    #[inline(always)]
    pub const fn set_RXNOISE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
            .field("FRAMERR", &self.FRAMERR())
            .field("PARITYERR", &self.PARITYERR())
            .field("RXNOISE", &self.RXNOISE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFORD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FIFORD {{ RXDATA: {=u16:?}, FRAMERR: {=bool:?}, PARITYERR: {=bool:?}, RXNOISE: {=bool:?} }}",
            self.RXDATA(),
            self.FRAMERR(),
            self.PARITYERR(),
            self.RXNOISE()
        )
    }
}
#[doc = "FIFO data read with no FIFO pop."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIFORDNOPOP(pub u32);
impl FIFORDNOPOP {
    #[doc = "Received data from the FIFO. The number of bits used depends on the DATALEN and PARITYSEL settings."]
    #[must_use]
    #[inline(always)]
    pub const fn RXDATA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Received data from the FIFO. The number of bits used depends on the DATALEN and PARITYSEL settings."]
    #[inline(always)]
    pub const fn set_RXDATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Framing Error status flag. This bit reflects the status for the data it is read along with from the FIFO, and indicates that the character was received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[must_use]
    #[inline(always)]
    pub const fn FRAMERR(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error status flag. This bit reflects the status for the data it is read along with from the FIFO, and indicates that the character was received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[inline(always)]
    pub const fn set_FRAMERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Parity Error status flag. This bit reflects the status for the data it is read along with from the FIFO. This bit will be set when a parity error is detected in a received character."]
    #[must_use]
    #[inline(always)]
    pub const fn PARITYERR(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error status flag. This bit reflects the status for the data it is read along with from the FIFO. This bit will be set when a parity error is detected in a received character."]
    #[inline(always)]
    pub const fn set_PARITYERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Received Noise flag. See description of the RxNoiseInt bit in Table 354."]
    #[must_use]
    #[inline(always)]
    pub const fn RXNOISE(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Received Noise flag. See description of the RxNoiseInt bit in Table 354."]
    #[inline(always)]
    pub const fn set_RXNOISE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
            .field("FRAMERR", &self.FRAMERR())
            .field("PARITYERR", &self.PARITYERR())
            .field("RXNOISE", &self.RXNOISE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFORDNOPOP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FIFORDNOPOP {{ RXDATA: {=u16:?}, FRAMERR: {=bool:?}, PARITYERR: {=bool:?}, RXNOISE: {=bool:?} }}",
            self.RXDATA(),
            self.FRAMERR(),
            self.PARITYERR(),
            self.RXNOISE()
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
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Transmit data to the FIFO."]
    #[inline(always)]
    pub const fn set_TXDATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFOWR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FIFOWR {{ TXDATA: {=u16:?} }}", self.TXDATA())
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
#[doc = "Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTENCLR(pub u32);
impl INTENCLR {
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn TXIDLECLR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_TXIDLECLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTACTSCLR(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_DELTACTSCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn TXDISCLR(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_TXDISCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTARXBRKCLR(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_DELTARXBRKCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn STARTCLR(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_STARTCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn FRAMERRCLR(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_FRAMERRCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn PARITYERRCLR(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_PARITYERRCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn RXNOISECLR(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_RXNOISECLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn ABERRCLR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_ABERRCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            .field("TXIDLECLR", &self.TXIDLECLR())
            .field("DELTACTSCLR", &self.DELTACTSCLR())
            .field("TXDISCLR", &self.TXDISCLR())
            .field("DELTARXBRKCLR", &self.DELTARXBRKCLR())
            .field("STARTCLR", &self.STARTCLR())
            .field("FRAMERRCLR", &self.FRAMERRCLR())
            .field("PARITYERRCLR", &self.PARITYERRCLR())
            .field("RXNOISECLR", &self.RXNOISECLR())
            .field("ABERRCLR", &self.ABERRCLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTENCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTENCLR {{ TXIDLECLR: {=bool:?}, DELTACTSCLR: {=bool:?}, TXDISCLR: {=bool:?}, DELTARXBRKCLR: {=bool:?}, STARTCLR: {=bool:?}, FRAMERRCLR: {=bool:?}, PARITYERRCLR: {=bool:?}, RXNOISECLR: {=bool:?}, ABERRCLR: {=bool:?} }}",
            self.TXIDLECLR(),
            self.DELTACTSCLR(),
            self.TXDISCLR(),
            self.DELTARXBRKCLR(),
            self.STARTCLR(),
            self.FRAMERRCLR(),
            self.PARITYERRCLR(),
            self.RXNOISECLR(),
            self.ABERRCLR()
        )
    }
}
#[doc = "Interrupt Enable read and Set register for USART (not FIFO) status. Contains individual interrupt enable bits for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTENSET(pub u32);
impl INTENSET {
    #[doc = "When 1, enables an interrupt when the transmitter becomes idle (TXIDLE = 1)."]
    #[must_use]
    #[inline(always)]
    pub const fn TXIDLEEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when the transmitter becomes idle (TXIDLE = 1)."]
    #[inline(always)]
    pub const fn set_TXIDLEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "When 1, enables an interrupt when there is a change in the state of the CTS input."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTACTSEN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when there is a change in the state of the CTS input."]
    #[inline(always)]
    pub const fn set_DELTACTSEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details."]
    #[must_use]
    #[inline(always)]
    pub const fn TXDISEN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details."]
    #[inline(always)]
    pub const fn set_TXDISEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted)."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTARXBRKEN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted)."]
    #[inline(always)]
    pub const fn set_DELTARXBRKEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "When 1, enables an interrupt when a received start bit has been detected."]
    #[must_use]
    #[inline(always)]
    pub const fn STARTEN(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when a received start bit has been detected."]
    #[inline(always)]
    pub const fn set_STARTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "When 1, enables an interrupt when a framing error has been detected."]
    #[must_use]
    #[inline(always)]
    pub const fn FRAMERREN(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when a framing error has been detected."]
    #[inline(always)]
    pub const fn set_FRAMERREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "When 1, enables an interrupt when a parity error has been detected."]
    #[must_use]
    #[inline(always)]
    pub const fn PARITYERREN(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when a parity error has been detected."]
    #[inline(always)]
    pub const fn set_PARITYERREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "When 1, enables an interrupt when noise is detected. See description of the RXNOISEINT bit in Table 354."]
    #[must_use]
    #[inline(always)]
    pub const fn RXNOISEEN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when noise is detected. See description of the RXNOISEINT bit in Table 354."]
    #[inline(always)]
    pub const fn set_RXNOISEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "When 1, enables an interrupt when an auto baud error occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn ABERREN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when an auto baud error occurs."]
    #[inline(always)]
    pub const fn set_ABERREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            .field("TXIDLEEN", &self.TXIDLEEN())
            .field("DELTACTSEN", &self.DELTACTSEN())
            .field("TXDISEN", &self.TXDISEN())
            .field("DELTARXBRKEN", &self.DELTARXBRKEN())
            .field("STARTEN", &self.STARTEN())
            .field("FRAMERREN", &self.FRAMERREN())
            .field("PARITYERREN", &self.PARITYERREN())
            .field("RXNOISEEN", &self.RXNOISEEN())
            .field("ABERREN", &self.ABERREN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTENSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTENSET {{ TXIDLEEN: {=bool:?}, DELTACTSEN: {=bool:?}, TXDISEN: {=bool:?}, DELTARXBRKEN: {=bool:?}, STARTEN: {=bool:?}, FRAMERREN: {=bool:?}, PARITYERREN: {=bool:?}, RXNOISEEN: {=bool:?}, ABERREN: {=bool:?} }}",
            self.TXIDLEEN(),
            self.DELTACTSEN(),
            self.TXDISEN(),
            self.DELTARXBRKEN(),
            self.STARTEN(),
            self.FRAMERREN(),
            self.PARITYERREN(),
            self.RXNOISEEN(),
            self.ABERREN()
        )
    }
}
#[doc = "Interrupt status register. Reflects interrupts that are currently enabled."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTSTAT(pub u32);
impl INTSTAT {
    #[doc = "Transmitter Idle status."]
    #[must_use]
    #[inline(always)]
    pub const fn TXIDLE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Idle status."]
    #[inline(always)]
    pub const fn set_TXIDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit is set when a change in the state of the CTS input is detected."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTACTS(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set when a change in the state of the CTS input is detected."]
    #[inline(always)]
    pub const fn set_DELTACTS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmitter Disabled Interrupt flag."]
    #[must_use]
    #[inline(always)]
    pub const fn TXDISINT(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Disabled Interrupt flag."]
    #[inline(always)]
    pub const fn set_TXDISINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit is set when a change in the state of receiver break detection occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTARXBRK(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set when a change in the state of receiver break detection occurs."]
    #[inline(always)]
    pub const fn set_DELTARXBRK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit is set when a start is detected on the receiver input."]
    #[must_use]
    #[inline(always)]
    pub const fn START(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set when a start is detected on the receiver input."]
    #[inline(always)]
    pub const fn set_START(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Framing Error interrupt flag."]
    #[must_use]
    #[inline(always)]
    pub const fn FRAMERRINT(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error interrupt flag."]
    #[inline(always)]
    pub const fn set_FRAMERRINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Parity Error interrupt flag."]
    #[must_use]
    #[inline(always)]
    pub const fn PARITYERRINT(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error interrupt flag."]
    #[inline(always)]
    pub const fn set_PARITYERRINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Received Noise interrupt flag."]
    #[must_use]
    #[inline(always)]
    pub const fn RXNOISEINT(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Received Noise interrupt flag."]
    #[inline(always)]
    pub const fn set_RXNOISEINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Auto baud Error Interrupt flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ABERRINT(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Auto baud Error Interrupt flag."]
    #[inline(always)]
    pub const fn set_ABERRINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            .field("TXIDLE", &self.TXIDLE())
            .field("DELTACTS", &self.DELTACTS())
            .field("TXDISINT", &self.TXDISINT())
            .field("DELTARXBRK", &self.DELTARXBRK())
            .field("START", &self.START())
            .field("FRAMERRINT", &self.FRAMERRINT())
            .field("PARITYERRINT", &self.PARITYERRINT())
            .field("RXNOISEINT", &self.RXNOISEINT())
            .field("ABERRINT", &self.ABERRINT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTSTAT {{ TXIDLE: {=bool:?}, DELTACTS: {=bool:?}, TXDISINT: {=bool:?}, DELTARXBRK: {=bool:?}, START: {=bool:?}, FRAMERRINT: {=bool:?}, PARITYERRINT: {=bool:?}, RXNOISEINT: {=bool:?}, ABERRINT: {=bool:?} }}",
            self.TXIDLE(),
            self.DELTACTS(),
            self.TXDISINT(),
            self.DELTARXBRK(),
            self.START(),
            self.FRAMERRINT(),
            self.PARITYERRINT(),
            self.RXNOISEINT(),
            self.ABERRINT()
        )
    }
}
#[doc = "Oversample selection register for asynchronous communication."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OSR(pub u32);
impl OSR {
    #[doc = "Oversample Selection Value. 0 to 3 = not supported 0x4 = 5 function clocks are used to transmit and receive each data bit. 0x5 = 6 function clocks are used to transmit and receive each data bit. 0xF= 16 function clocks are used to transmit and receive each data bit."]
    #[must_use]
    #[inline(always)]
    pub const fn OSRVAL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Oversample Selection Value. 0 to 3 = not supported 0x4 = 5 function clocks are used to transmit and receive each data bit. 0x5 = 6 function clocks are used to transmit and receive each data bit. 0xF= 16 function clocks are used to transmit and receive each data bit."]
    #[inline(always)]
    pub const fn set_OSRVAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for OSR {
    #[inline(always)]
    fn default() -> OSR {
        OSR(0)
    }
}
impl core::fmt::Debug for OSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSR")
            .field("OSRVAL", &self.OSRVAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OSR {{ OSRVAL: {=u8:?} }}", self.OSRVAL())
    }
}
#[doc = "USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STAT(pub u32);
impl STAT {
    #[doc = "Receiver Idle. When 0, indicates that the receiver is currently in the process of receiving data. When 1, indicates that the receiver is not currently in the process of receiving data."]
    #[must_use]
    #[inline(always)]
    pub const fn RXIDLE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Idle. When 0, indicates that the receiver is currently in the process of receiving data. When 1, indicates that the receiver is not currently in the process of receiving data."]
    #[inline(always)]
    pub const fn set_RXIDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmitter Idle. When 0, indicates that the transmitter is currently in the process of sending data.When 1, indicate that the transmitter is not currently in the process of sending data."]
    #[must_use]
    #[inline(always)]
    pub const fn TXIDLE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Idle. When 0, indicates that the transmitter is currently in the process of sending data.When 1, indicate that the transmitter is not currently in the process of sending data."]
    #[inline(always)]
    pub const fn set_TXIDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit reflects the current state of the CTS signal, regardless of the setting of the CTSEN bit in the CFG register. This will be the value of the CTS input pin unless loopback mode is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CTS(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit reflects the current state of the CTS signal, regardless of the setting of the CTSEN bit in the CFG register. This will be the value of the CTS input pin unless loopback mode is enabled."]
    #[inline(always)]
    pub const fn set_CTS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit is set when a change in the state is detected for the CTS flag above. This bit is cleared by software."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTACTS(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set when a change in the state is detected for the CTS flag above. This bit is cleared by software."]
    #[inline(always)]
    pub const fn set_DELTACTS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmitter Disabled Status flag. When 1, this bit indicates that the USART transmitter is fully idle after being disabled via the TXDIS bit in the CFG register (TXDIS = 1)."]
    #[must_use]
    #[inline(always)]
    pub const fn TXDISSTAT(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Disabled Status flag. When 1, this bit indicates that the USART transmitter is fully idle after being disabled via the TXDIS bit in the CFG register (TXDIS = 1)."]
    #[inline(always)]
    pub const fn set_TXDISSTAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Received Break. This bit reflects the current state of the receiver break detection logic. It is set when the Un_RXD pin remains low for 16 bit times. Note that FRAMERRINT will also be set when this condition occurs because the stop bit(s) for the character would be missing. RXBRK is cleared when the Un_RXD pin goes high."]
    #[must_use]
    #[inline(always)]
    pub const fn RXBRK(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Received Break. This bit reflects the current state of the receiver break detection logic. It is set when the Un_RXD pin remains low for 16 bit times. Note that FRAMERRINT will also be set when this condition occurs because the stop bit(s) for the character would be missing. RXBRK is cleared when the Un_RXD pin goes high."]
    #[inline(always)]
    pub const fn set_RXBRK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This bit is set when a change in the state of receiver break detection occurs. Cleared by software."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTARXBRK(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set when a change in the state of receiver break detection occurs. Cleared by software."]
    #[inline(always)]
    pub const fn set_DELTARXBRK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit is set when a start is detected on the receiver input. Its purpose is primarily to allow wake-up from Deep-sleep or Power-down mode immediately when a start is detected. Cleared by software."]
    #[must_use]
    #[inline(always)]
    pub const fn START(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set when a start is detected on the receiver input. Its purpose is primarily to allow wake-up from Deep-sleep or Power-down mode immediately when a start is detected. Cleared by software."]
    #[inline(always)]
    pub const fn set_START(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Framing Error interrupt flag. This flag is set when a character is received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[must_use]
    #[inline(always)]
    pub const fn FRAMERRINT(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error interrupt flag. This flag is set when a character is received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[inline(always)]
    pub const fn set_FRAMERRINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Parity Error interrupt flag. This flag is set when a parity error is detected in a received character."]
    #[must_use]
    #[inline(always)]
    pub const fn PARITYERRINT(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error interrupt flag. This flag is set when a parity error is detected in a received character."]
    #[inline(always)]
    pub const fn set_PARITYERRINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Received Noise interrupt flag. Three samples of received data are taken in order to determine the value of each received data bit, except in synchronous mode. This acts as a noise filter if one sample disagrees. This flag is set when a received data bit contains one disagreeing sample. This could indicate line noise, a baud rate or character format mismatch, or loss of synchronization during data reception."]
    #[must_use]
    #[inline(always)]
    pub const fn RXNOISEINT(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Received Noise interrupt flag. Three samples of received data are taken in order to determine the value of each received data bit, except in synchronous mode. This acts as a noise filter if one sample disagrees. This flag is set when a received data bit contains one disagreeing sample. This could indicate line noise, a baud rate or character format mismatch, or loss of synchronization during data reception."]
    #[inline(always)]
    pub const fn set_RXNOISEINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Auto baud Error. An auto baud error can occur if the BRG counts to its limit before the end of the start bit that is being measured, essentially an auto baud time-out."]
    #[must_use]
    #[inline(always)]
    pub const fn ABERR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Auto baud Error. An auto baud error can occur if the BRG counts to its limit before the end of the start bit that is being measured, essentially an auto baud time-out."]
    #[inline(always)]
    pub const fn set_ABERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            .field("RXIDLE", &self.RXIDLE())
            .field("TXIDLE", &self.TXIDLE())
            .field("CTS", &self.CTS())
            .field("DELTACTS", &self.DELTACTS())
            .field("TXDISSTAT", &self.TXDISSTAT())
            .field("RXBRK", &self.RXBRK())
            .field("DELTARXBRK", &self.DELTARXBRK())
            .field("START", &self.START())
            .field("FRAMERRINT", &self.FRAMERRINT())
            .field("PARITYERRINT", &self.PARITYERRINT())
            .field("RXNOISEINT", &self.RXNOISEINT())
            .field("ABERR", &self.ABERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STAT {{ RXIDLE: {=bool:?}, TXIDLE: {=bool:?}, CTS: {=bool:?}, DELTACTS: {=bool:?}, TXDISSTAT: {=bool:?}, RXBRK: {=bool:?}, DELTARXBRK: {=bool:?}, START: {=bool:?}, FRAMERRINT: {=bool:?}, PARITYERRINT: {=bool:?}, RXNOISEINT: {=bool:?}, ABERR: {=bool:?} }}",
            self.RXIDLE(),
            self.TXIDLE(),
            self.CTS(),
            self.DELTACTS(),
            self.TXDISSTAT(),
            self.RXBRK(),
            self.DELTARXBRK(),
            self.START(),
            self.FRAMERRINT(),
            self.PARITYERRINT(),
            self.RXNOISEINT(),
            self.ABERR()
        )
    }
}
