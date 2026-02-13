#[doc = "Address register for automatic address matching."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addr(pub u32);
impl Addr {
    #[doc = "8-bit address used with automatic address matching. Used when address detection is enabled (ADDRDET in CTL = 1) and automatic address matching is enabled (AUTOADDR in CFG = 1)."]
    #[must_use]
    #[inline(always)]
    pub const fn address(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit address used with automatic address matching. Used when address detection is enabled (ADDRDET in CTL = 1) and automatic address matching is enabled (AUTOADDR in CFG = 1)."]
    #[inline(always)]
    pub const fn set_address(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Addr {
    #[inline(always)]
    fn default() -> Addr {
        Addr(0)
    }
}
impl core::fmt::Debug for Addr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Addr")
            .field("address", &self.address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Addr {{ address: {=u8:?} }}", self.address())
    }
}
#[doc = "Baud Rate Generator register. 16-bit integer baud rate divisor value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brg(pub u32);
impl Brg {
    #[doc = "This value is used to divide the USART input clock to determine the baud rate, based on the input clock from the FRG. 0 = FCLK is used directly by the USART function. 1 = FCLK is divided by 2 before use by the USART function. 2 = FCLK is divided by 3 before use by the USART function. 0xFFFF = FCLK is divided by 65,536 before use by the USART function."]
    #[must_use]
    #[inline(always)]
    pub const fn brgval(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This value is used to divide the USART input clock to determine the baud rate, based on the input clock from the FRG. 0 = FCLK is used directly by the USART function. 1 = FCLK is divided by 2 before use by the USART function. 2 = FCLK is divided by 3 before use by the USART function. 0xFFFF = FCLK is divided by 65,536 before use by the USART function."]
    #[inline(always)]
    pub const fn set_brgval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Brg {
    #[inline(always)]
    fn default() -> Brg {
        Brg(0)
    }
}
impl core::fmt::Debug for Brg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Brg")
            .field("brgval", &self.brgval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Brg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Brg {{ brgval: {=u16:?} }}", self.brgval())
    }
}
#[doc = "USART Configuration register. Basic USART configuration settings that typically are not changed during operation."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "USART Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USART Enable."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Selects the data size for the USART."]
    #[must_use]
    #[inline(always)]
    pub const fn datalen(&self) -> super::vals::Datalen {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Datalen::from_bits(val as u8)
    }
    #[doc = "Selects the data size for the USART."]
    #[inline(always)]
    pub const fn set_datalen(&mut self, val: super::vals::Datalen) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Selects what type of parity is used by the USART."]
    #[must_use]
    #[inline(always)]
    pub const fn paritysel(&self) -> super::vals::Paritysel {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Paritysel::from_bits(val as u8)
    }
    #[doc = "Selects what type of parity is used by the USART."]
    #[inline(always)]
    pub const fn set_paritysel(&mut self, val: super::vals::Paritysel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
    #[must_use]
    #[inline(always)]
    pub const fn stoplen(&self) -> super::vals::Stoplen {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Stoplen::from_bits(val as u8)
    }
    #[doc = "Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
    #[inline(always)]
    pub const fn set_stoplen(&mut self, val: super::vals::Stoplen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Selects standard or 32 kHz clocking mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode32k(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Selects standard or 32 kHz clocking mode."]
    #[inline(always)]
    pub const fn set_mode32k(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LIN break mode enable."]
    #[must_use]
    #[inline(always)]
    pub const fn linmode(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LIN break mode enable."]
    #[inline(always)]
    pub const fn set_linmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled."]
    #[inline(always)]
    pub const fn set_ctsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Selects synchronous or asynchronous operation."]
    #[must_use]
    #[inline(always)]
    pub const fn syncen(&self) -> super::vals::Syncen {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Syncen::from_bits(val as u8)
    }
    #[doc = "Selects synchronous or asynchronous operation."]
    #[inline(always)]
    pub const fn set_syncen(&mut self, val: super::vals::Syncen) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Selects the clock polarity and sampling edge of received data in synchronous mode."]
    #[must_use]
    #[inline(always)]
    pub const fn clkpol(&self) -> super::vals::Clkpol {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Clkpol::from_bits(val as u8)
    }
    #[doc = "Selects the clock polarity and sampling edge of received data in synchronous mode."]
    #[inline(always)]
    pub const fn set_clkpol(&mut self, val: super::vals::Clkpol) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Synchronous mode Master select."]
    #[must_use]
    #[inline(always)]
    pub const fn syncmst(&self) -> super::vals::Syncmst {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Syncmst::from_bits(val as u8)
    }
    #[doc = "Synchronous mode Master select."]
    #[inline(always)]
    pub const fn set_syncmst(&mut self, val: super::vals::Syncmst) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Selects data loopback mode."]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Loop {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Loop::from_bits(val as u8)
    }
    #[doc = "Selects data loopback mode."]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::Loop) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Output Enable Turnaround time enable for RS-485 operation."]
    #[must_use]
    #[inline(always)]
    pub const fn oeta(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Output Enable Turnaround time enable for RS-485 operation."]
    #[inline(always)]
    pub const fn set_oeta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Automatic Address matching enable."]
    #[must_use]
    #[inline(always)]
    pub const fn autoaddr(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic Address matching enable."]
    #[inline(always)]
    pub const fn set_autoaddr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Output Enable Select."]
    #[must_use]
    #[inline(always)]
    pub const fn oesel(&self) -> super::vals::Oesel {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Oesel::from_bits(val as u8)
    }
    #[doc = "Output Enable Select."]
    #[inline(always)]
    pub const fn set_oesel(&mut self, val: super::vals::Oesel) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Output Enable Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn oepol(&self) -> super::vals::Oepol {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Oepol::from_bits(val as u8)
    }
    #[doc = "Output Enable Polarity."]
    #[inline(always)]
    pub const fn set_oepol(&mut self, val: super::vals::Oepol) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Receive data polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpol(&self) -> super::vals::Rxpol {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Rxpol::from_bits(val as u8)
    }
    #[doc = "Receive data polarity."]
    #[inline(always)]
    pub const fn set_rxpol(&mut self, val: super::vals::Rxpol) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Transmit data polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn txpol(&self) -> super::vals::Txpol {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Txpol::from_bits(val as u8)
    }
    #[doc = "Transmit data polarity."]
    #[inline(always)]
    pub const fn set_txpol(&mut self, val: super::vals::Txpol) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
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
            .field("datalen", &self.datalen())
            .field("paritysel", &self.paritysel())
            .field("stoplen", &self.stoplen())
            .field("mode32k", &self.mode32k())
            .field("linmode", &self.linmode())
            .field("ctsen", &self.ctsen())
            .field("syncen", &self.syncen())
            .field("clkpol", &self.clkpol())
            .field("syncmst", &self.syncmst())
            .field("loop_", &self.loop_())
            .field("oeta", &self.oeta())
            .field("autoaddr", &self.autoaddr())
            .field("oesel", &self.oesel())
            .field("oepol", &self.oepol())
            .field("rxpol", &self.rxpol())
            .field("txpol", &self.txpol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg {{ enable: {=bool:?}, datalen: {:?}, paritysel: {:?}, stoplen: {:?}, mode32k: {=bool:?}, linmode: {=bool:?}, ctsen: {=bool:?}, syncen: {:?}, clkpol: {:?}, syncmst: {:?}, loop_: {:?}, oeta: {=bool:?}, autoaddr: {=bool:?}, oesel: {:?}, oepol: {:?}, rxpol: {:?}, txpol: {:?} }}",
            self.enable(),
            self.datalen(),
            self.paritysel(),
            self.stoplen(),
            self.mode32k(),
            self.linmode(),
            self.ctsen(),
            self.syncen(),
            self.clkpol(),
            self.syncmst(),
            self.loop_(),
            self.oeta(),
            self.autoaddr(),
            self.oesel(),
            self.oepol(),
            self.rxpol(),
            self.txpol()
        )
    }
}
#[doc = "USART Control register. USART control settings that are more likely to change during operation."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc = "Break Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txbrken(&self) -> super::vals::Txbrken {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Txbrken::from_bits(val as u8)
    }
    #[doc = "Break Enable."]
    #[inline(always)]
    pub const fn set_txbrken(&mut self, val: super::vals::Txbrken) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable address detect mode."]
    #[must_use]
    #[inline(always)]
    pub const fn addrdet(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable address detect mode."]
    #[inline(always)]
    pub const fn set_addrdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn txdis(&self) -> super::vals::Txdis {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Txdis::from_bits(val as u8)
    }
    #[doc = "Transmit Disable."]
    #[inline(always)]
    pub const fn set_txdis(&mut self, val: super::vals::Txdis) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
    #[must_use]
    #[inline(always)]
    pub const fn cc(&self) -> super::vals::Cc {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cc::from_bits(val as u8)
    }
    #[doc = "Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
    #[inline(always)]
    pub const fn set_cc(&mut self, val: super::vals::Cc) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Clear Continuous Clock."]
    #[must_use]
    #[inline(always)]
    pub const fn clrcconrx(&self) -> super::vals::Clrcconrx {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Clrcconrx::from_bits(val as u8)
    }
    #[doc = "Clear Continuous Clock."]
    #[inline(always)]
    pub const fn set_clrcconrx(&mut self, val: super::vals::Clrcconrx) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Autobaud enable."]
    #[must_use]
    #[inline(always)]
    pub const fn autobaud(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Autobaud enable."]
    #[inline(always)]
    pub const fn set_autobaud(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Ctl {
    #[inline(always)]
    fn default() -> Ctl {
        Ctl(0)
    }
}
impl core::fmt::Debug for Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctl")
            .field("txbrken", &self.txbrken())
            .field("addrdet", &self.addrdet())
            .field("txdis", &self.txdis())
            .field("cc", &self.cc())
            .field("clrcconrx", &self.clrcconrx())
            .field("autobaud", &self.autobaud())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctl {{ txbrken: {:?}, addrdet: {=bool:?}, txdis: {:?}, cc: {:?}, clrcconrx: {:?}, autobaud: {=bool:?} }}",
            self.txbrken(),
            self.addrdet(),
            self.txdis(),
            self.cc(),
            self.clrcconrx(),
            self.autobaud()
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
    #[doc = "Received data from the FIFO. The number of bits used depends on the DATALEN and PARITYSEL settings."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Received data from the FIFO. The number of bits used depends on the DATALEN and PARITYSEL settings."]
    #[inline(always)]
    pub const fn set_rxdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Framing Error status flag. This bit reflects the status for the data it is read along with from the FIFO, and indicates that the character was received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[must_use]
    #[inline(always)]
    pub const fn framerr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error status flag. This bit reflects the status for the data it is read along with from the FIFO, and indicates that the character was received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[inline(always)]
    pub const fn set_framerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Parity Error status flag. This bit reflects the status for the data it is read along with from the FIFO. This bit will be set when a parity error is detected in a received character."]
    #[must_use]
    #[inline(always)]
    pub const fn parityerr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error status flag. This bit reflects the status for the data it is read along with from the FIFO. This bit will be set when a parity error is detected in a received character."]
    #[inline(always)]
    pub const fn set_parityerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Received Noise flag. See description of the RxNoiseInt bit in Table 354."]
    #[must_use]
    #[inline(always)]
    pub const fn rxnoise(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Received Noise flag. See description of the RxNoiseInt bit in Table 354."]
    #[inline(always)]
    pub const fn set_rxnoise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
            .field("framerr", &self.framerr())
            .field("parityerr", &self.parityerr())
            .field("rxnoise", &self.rxnoise())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fiford {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fiford {{ rxdata: {=u16:?}, framerr: {=bool:?}, parityerr: {=bool:?}, rxnoise: {=bool:?} }}",
            self.rxdata(),
            self.framerr(),
            self.parityerr(),
            self.rxnoise()
        )
    }
}
#[doc = "FIFO data read with no FIFO pop."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifordnopop(pub u32);
impl Fifordnopop {
    #[doc = "Received data from the FIFO. The number of bits used depends on the DATALEN and PARITYSEL settings."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Received data from the FIFO. The number of bits used depends on the DATALEN and PARITYSEL settings."]
    #[inline(always)]
    pub const fn set_rxdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Framing Error status flag. This bit reflects the status for the data it is read along with from the FIFO, and indicates that the character was received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[must_use]
    #[inline(always)]
    pub const fn framerr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error status flag. This bit reflects the status for the data it is read along with from the FIFO, and indicates that the character was received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[inline(always)]
    pub const fn set_framerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Parity Error status flag. This bit reflects the status for the data it is read along with from the FIFO. This bit will be set when a parity error is detected in a received character."]
    #[must_use]
    #[inline(always)]
    pub const fn parityerr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error status flag. This bit reflects the status for the data it is read along with from the FIFO. This bit will be set when a parity error is detected in a received character."]
    #[inline(always)]
    pub const fn set_parityerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Received Noise flag. See description of the RxNoiseInt bit in Table 354."]
    #[must_use]
    #[inline(always)]
    pub const fn rxnoise(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Received Noise flag. See description of the RxNoiseInt bit in Table 354."]
    #[inline(always)]
    pub const fn set_rxnoise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
            .field("framerr", &self.framerr())
            .field("parityerr", &self.parityerr())
            .field("rxnoise", &self.rxnoise())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifordnopop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifordnopop {{ rxdata: {=u16:?}, framerr: {=bool:?}, parityerr: {=bool:?}, rxnoise: {=bool:?} }}",
            self.rxdata(),
            self.framerr(),
            self.parityerr(),
            self.rxnoise()
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
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Transmit data to the FIFO."]
    #[inline(always)]
    pub const fn set_txdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifowr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fifowr {{ txdata: {=u16:?} }}", self.txdata())
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
#[doc = "Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenclr(pub u32);
impl Intenclr {
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn txidleclr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_txidleclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn deltactsclr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_deltactsclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn txdisclr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_txdisclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn deltarxbrkclr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_deltarxbrkclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn startclr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_startclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn framerrclr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_framerrclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn parityerrclr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_parityerrclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn rxnoiseclr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_rxnoiseclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[must_use]
    #[inline(always)]
    pub const fn aberrclr(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn set_aberrclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            .field("txidleclr", &self.txidleclr())
            .field("deltactsclr", &self.deltactsclr())
            .field("txdisclr", &self.txdisclr())
            .field("deltarxbrkclr", &self.deltarxbrkclr())
            .field("startclr", &self.startclr())
            .field("framerrclr", &self.framerrclr())
            .field("parityerrclr", &self.parityerrclr())
            .field("rxnoiseclr", &self.rxnoiseclr())
            .field("aberrclr", &self.aberrclr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intenclr {{ txidleclr: {=bool:?}, deltactsclr: {=bool:?}, txdisclr: {=bool:?}, deltarxbrkclr: {=bool:?}, startclr: {=bool:?}, framerrclr: {=bool:?}, parityerrclr: {=bool:?}, rxnoiseclr: {=bool:?}, aberrclr: {=bool:?} }}",
            self.txidleclr(),
            self.deltactsclr(),
            self.txdisclr(),
            self.deltarxbrkclr(),
            self.startclr(),
            self.framerrclr(),
            self.parityerrclr(),
            self.rxnoiseclr(),
            self.aberrclr()
        )
    }
}
#[doc = "Interrupt Enable read and Set register for USART (not FIFO) status. Contains individual interrupt enable bits for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenset(pub u32);
impl Intenset {
    #[doc = "When 1, enables an interrupt when the transmitter becomes idle (TXIDLE = 1)."]
    #[must_use]
    #[inline(always)]
    pub const fn txidleen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when the transmitter becomes idle (TXIDLE = 1)."]
    #[inline(always)]
    pub const fn set_txidleen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "When 1, enables an interrupt when there is a change in the state of the CTS input."]
    #[must_use]
    #[inline(always)]
    pub const fn deltactsen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when there is a change in the state of the CTS input."]
    #[inline(always)]
    pub const fn set_deltactsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details."]
    #[must_use]
    #[inline(always)]
    pub const fn txdisen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details."]
    #[inline(always)]
    pub const fn set_txdisen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted)."]
    #[must_use]
    #[inline(always)]
    pub const fn deltarxbrken(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted)."]
    #[inline(always)]
    pub const fn set_deltarxbrken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "When 1, enables an interrupt when a received start bit has been detected."]
    #[must_use]
    #[inline(always)]
    pub const fn starten(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when a received start bit has been detected."]
    #[inline(always)]
    pub const fn set_starten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "When 1, enables an interrupt when a framing error has been detected."]
    #[must_use]
    #[inline(always)]
    pub const fn framerren(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when a framing error has been detected."]
    #[inline(always)]
    pub const fn set_framerren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "When 1, enables an interrupt when a parity error has been detected."]
    #[must_use]
    #[inline(always)]
    pub const fn parityerren(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when a parity error has been detected."]
    #[inline(always)]
    pub const fn set_parityerren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "When 1, enables an interrupt when noise is detected. See description of the RXNOISEINT bit in Table 354."]
    #[must_use]
    #[inline(always)]
    pub const fn rxnoiseen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when noise is detected. See description of the RXNOISEINT bit in Table 354."]
    #[inline(always)]
    pub const fn set_rxnoiseen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "When 1, enables an interrupt when an auto baud error occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn aberren(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enables an interrupt when an auto baud error occurs."]
    #[inline(always)]
    pub const fn set_aberren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            .field("txidleen", &self.txidleen())
            .field("deltactsen", &self.deltactsen())
            .field("txdisen", &self.txdisen())
            .field("deltarxbrken", &self.deltarxbrken())
            .field("starten", &self.starten())
            .field("framerren", &self.framerren())
            .field("parityerren", &self.parityerren())
            .field("rxnoiseen", &self.rxnoiseen())
            .field("aberren", &self.aberren())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intenset {{ txidleen: {=bool:?}, deltactsen: {=bool:?}, txdisen: {=bool:?}, deltarxbrken: {=bool:?}, starten: {=bool:?}, framerren: {=bool:?}, parityerren: {=bool:?}, rxnoiseen: {=bool:?}, aberren: {=bool:?} }}",
            self.txidleen(),
            self.deltactsen(),
            self.txdisen(),
            self.deltarxbrken(),
            self.starten(),
            self.framerren(),
            self.parityerren(),
            self.rxnoiseen(),
            self.aberren()
        )
    }
}
#[doc = "Interrupt status register. Reflects interrupts that are currently enabled."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Transmitter Idle status."]
    #[must_use]
    #[inline(always)]
    pub const fn txidle(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Idle status."]
    #[inline(always)]
    pub const fn set_txidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit is set when a change in the state of the CTS input is detected."]
    #[must_use]
    #[inline(always)]
    pub const fn deltacts(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set when a change in the state of the CTS input is detected."]
    #[inline(always)]
    pub const fn set_deltacts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmitter Disabled Interrupt flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txdisint(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Disabled Interrupt flag."]
    #[inline(always)]
    pub const fn set_txdisint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit is set when a change in the state of receiver break detection occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn deltarxbrk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set when a change in the state of receiver break detection occurs."]
    #[inline(always)]
    pub const fn set_deltarxbrk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit is set when a start is detected on the receiver input."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set when a start is detected on the receiver input."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Framing Error interrupt flag."]
    #[must_use]
    #[inline(always)]
    pub const fn framerrint(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error interrupt flag."]
    #[inline(always)]
    pub const fn set_framerrint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Parity Error interrupt flag."]
    #[must_use]
    #[inline(always)]
    pub const fn parityerrint(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error interrupt flag."]
    #[inline(always)]
    pub const fn set_parityerrint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Received Noise interrupt flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxnoiseint(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Received Noise interrupt flag."]
    #[inline(always)]
    pub const fn set_rxnoiseint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Auto baud Error Interrupt flag."]
    #[must_use]
    #[inline(always)]
    pub const fn aberrint(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Auto baud Error Interrupt flag."]
    #[inline(always)]
    pub const fn set_aberrint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            .field("txidle", &self.txidle())
            .field("deltacts", &self.deltacts())
            .field("txdisint", &self.txdisint())
            .field("deltarxbrk", &self.deltarxbrk())
            .field("start", &self.start())
            .field("framerrint", &self.framerrint())
            .field("parityerrint", &self.parityerrint())
            .field("rxnoiseint", &self.rxnoiseint())
            .field("aberrint", &self.aberrint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intstat {{ txidle: {=bool:?}, deltacts: {=bool:?}, txdisint: {=bool:?}, deltarxbrk: {=bool:?}, start: {=bool:?}, framerrint: {=bool:?}, parityerrint: {=bool:?}, rxnoiseint: {=bool:?}, aberrint: {=bool:?} }}",
            self.txidle(),
            self.deltacts(),
            self.txdisint(),
            self.deltarxbrk(),
            self.start(),
            self.framerrint(),
            self.parityerrint(),
            self.rxnoiseint(),
            self.aberrint()
        )
    }
}
#[doc = "Oversample selection register for asynchronous communication."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osr(pub u32);
impl Osr {
    #[doc = "Oversample Selection Value. 0 to 3 = not supported 0x4 = 5 function clocks are used to transmit and receive each data bit. 0x5 = 6 function clocks are used to transmit and receive each data bit. 0xF= 16 function clocks are used to transmit and receive each data bit."]
    #[must_use]
    #[inline(always)]
    pub const fn osrval(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Oversample Selection Value. 0 to 3 = not supported 0x4 = 5 function clocks are used to transmit and receive each data bit. 0x5 = 6 function clocks are used to transmit and receive each data bit. 0xF= 16 function clocks are used to transmit and receive each data bit."]
    #[inline(always)]
    pub const fn set_osrval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Osr {
    #[inline(always)]
    fn default() -> Osr {
        Osr(0)
    }
}
impl core::fmt::Debug for Osr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osr")
            .field("osrval", &self.osrval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Osr {{ osrval: {=u8:?} }}", self.osrval())
    }
}
#[doc = "USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Receiver Idle. When 0, indicates that the receiver is currently in the process of receiving data. When 1, indicates that the receiver is not currently in the process of receiving data."]
    #[must_use]
    #[inline(always)]
    pub const fn rxidle(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Idle. When 0, indicates that the receiver is currently in the process of receiving data. When 1, indicates that the receiver is not currently in the process of receiving data."]
    #[inline(always)]
    pub const fn set_rxidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmitter Idle. When 0, indicates that the transmitter is currently in the process of sending data.When 1, indicate that the transmitter is not currently in the process of sending data."]
    #[must_use]
    #[inline(always)]
    pub const fn txidle(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Idle. When 0, indicates that the transmitter is currently in the process of sending data.When 1, indicate that the transmitter is not currently in the process of sending data."]
    #[inline(always)]
    pub const fn set_txidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit reflects the current state of the CTS signal, regardless of the setting of the CTSEN bit in the CFG register. This will be the value of the CTS input pin unless loopback mode is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn cts(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit reflects the current state of the CTS signal, regardless of the setting of the CTSEN bit in the CFG register. This will be the value of the CTS input pin unless loopback mode is enabled."]
    #[inline(always)]
    pub const fn set_cts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit is set when a change in the state is detected for the CTS flag above. This bit is cleared by software."]
    #[must_use]
    #[inline(always)]
    pub const fn deltacts(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set when a change in the state is detected for the CTS flag above. This bit is cleared by software."]
    #[inline(always)]
    pub const fn set_deltacts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmitter Disabled Status flag. When 1, this bit indicates that the USART transmitter is fully idle after being disabled via the TXDIS bit in the CFG register (TXDIS = 1)."]
    #[must_use]
    #[inline(always)]
    pub const fn txdisstat(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Disabled Status flag. When 1, this bit indicates that the USART transmitter is fully idle after being disabled via the TXDIS bit in the CFG register (TXDIS = 1)."]
    #[inline(always)]
    pub const fn set_txdisstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Received Break. This bit reflects the current state of the receiver break detection logic. It is set when the Un_RXD pin remains low for 16 bit times. Note that FRAMERRINT will also be set when this condition occurs because the stop bit(s) for the character would be missing. RXBRK is cleared when the Un_RXD pin goes high."]
    #[must_use]
    #[inline(always)]
    pub const fn rxbrk(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Received Break. This bit reflects the current state of the receiver break detection logic. It is set when the Un_RXD pin remains low for 16 bit times. Note that FRAMERRINT will also be set when this condition occurs because the stop bit(s) for the character would be missing. RXBRK is cleared when the Un_RXD pin goes high."]
    #[inline(always)]
    pub const fn set_rxbrk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This bit is set when a change in the state of receiver break detection occurs. Cleared by software."]
    #[must_use]
    #[inline(always)]
    pub const fn deltarxbrk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set when a change in the state of receiver break detection occurs. Cleared by software."]
    #[inline(always)]
    pub const fn set_deltarxbrk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit is set when a start is detected on the receiver input. Its purpose is primarily to allow wake-up from Deep-sleep or Power-down mode immediately when a start is detected. Cleared by software."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set when a start is detected on the receiver input. Its purpose is primarily to allow wake-up from Deep-sleep or Power-down mode immediately when a start is detected. Cleared by software."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Framing Error interrupt flag. This flag is set when a character is received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[must_use]
    #[inline(always)]
    pub const fn framerrint(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error interrupt flag. This flag is set when a character is received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[inline(always)]
    pub const fn set_framerrint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Parity Error interrupt flag. This flag is set when a parity error is detected in a received character."]
    #[must_use]
    #[inline(always)]
    pub const fn parityerrint(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error interrupt flag. This flag is set when a parity error is detected in a received character."]
    #[inline(always)]
    pub const fn set_parityerrint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Received Noise interrupt flag. Three samples of received data are taken in order to determine the value of each received data bit, except in synchronous mode. This acts as a noise filter if one sample disagrees. This flag is set when a received data bit contains one disagreeing sample. This could indicate line noise, a baud rate or character format mismatch, or loss of synchronization during data reception."]
    #[must_use]
    #[inline(always)]
    pub const fn rxnoiseint(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Received Noise interrupt flag. Three samples of received data are taken in order to determine the value of each received data bit, except in synchronous mode. This acts as a noise filter if one sample disagrees. This flag is set when a received data bit contains one disagreeing sample. This could indicate line noise, a baud rate or character format mismatch, or loss of synchronization during data reception."]
    #[inline(always)]
    pub const fn set_rxnoiseint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Auto baud Error. An auto baud error can occur if the BRG counts to its limit before the end of the start bit that is being measured, essentially an auto baud time-out."]
    #[must_use]
    #[inline(always)]
    pub const fn aberr(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Auto baud Error. An auto baud error can occur if the BRG counts to its limit before the end of the start bit that is being measured, essentially an auto baud time-out."]
    #[inline(always)]
    pub const fn set_aberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            .field("rxidle", &self.rxidle())
            .field("txidle", &self.txidle())
            .field("cts", &self.cts())
            .field("deltacts", &self.deltacts())
            .field("txdisstat", &self.txdisstat())
            .field("rxbrk", &self.rxbrk())
            .field("deltarxbrk", &self.deltarxbrk())
            .field("start", &self.start())
            .field("framerrint", &self.framerrint())
            .field("parityerrint", &self.parityerrint())
            .field("rxnoiseint", &self.rxnoiseint())
            .field("aberr", &self.aberr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stat {{ rxidle: {=bool:?}, txidle: {=bool:?}, cts: {=bool:?}, deltacts: {=bool:?}, txdisstat: {=bool:?}, rxbrk: {=bool:?}, deltarxbrk: {=bool:?}, start: {=bool:?}, framerrint: {=bool:?}, parityerrint: {=bool:?}, rxnoiseint: {=bool:?}, aberr: {=bool:?} }}",
            self.rxidle(),
            self.txidle(),
            self.cts(),
            self.deltacts(),
            self.txdisstat(),
            self.rxbrk(),
            self.deltarxbrk(),
            self.start(),
            self.framerrint(),
            self.parityerrint(),
            self.rxnoiseint(),
            self.aberr()
        )
    }
}
