#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CC {
    #[doc = "Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    CLOCK_ON_CHARACTER = 0x0,
    #[doc = "Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    CONTINOUS_CLOCK = 0x01,
}
impl CC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CC {
    #[inline(always)]
    fn from(val: u8) -> CC {
        CC::from_bits(val)
    }
}
impl From<CC> for u8 {
    #[inline(always)]
    fn from(val: CC) -> u8 {
        CC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CLKPOL {
    #[doc = "Falling edge. Un_RXD is sampled on the falling edge of SCLK."]
    FALLING_EDGE = 0x0,
    #[doc = "Rising edge. Un_RXD is sampled on the rising edge of SCLK."]
    RISING_EDGE = 0x01,
}
impl CLKPOL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CLKPOL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CLKPOL {
    #[inline(always)]
    fn from(val: u8) -> CLKPOL {
        CLKPOL::from_bits(val)
    }
}
impl From<CLKPOL> for u8 {
    #[inline(always)]
    fn from(val: CLKPOL) -> u8 {
        CLKPOL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CLRCCONRX {
    #[doc = "No effect. No effect on the CC bit."]
    NO_EFFECT = 0x0,
    #[doc = "Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    AUTO_CLEAR = 0x01,
}
impl CLRCCONRX {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CLRCCONRX {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CLRCCONRX {
    #[inline(always)]
    fn from(val: u8) -> CLRCCONRX {
        CLRCCONRX::from_bits(val)
    }
}
impl From<CLRCCONRX> for u8 {
    #[inline(always)]
    fn from(val: CLRCCONRX) -> u8 {
        CLRCCONRX::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DATALEN {
    #[doc = "7 bit Data length."]
    BIT_7 = 0x0,
    #[doc = "8 bit Data length."]
    BIT_8 = 0x01,
    #[doc = "9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTL register."]
    BIT_9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl DATALEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DATALEN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DATALEN {
    #[inline(always)]
    fn from(val: u8) -> DATALEN {
        DATALEN::from_bits(val)
    }
}
impl From<DATALEN> for u8 {
    #[inline(always)]
    fn from(val: DATALEN) -> u8 {
        DATALEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LOOP {
    #[doc = "Normal operation."]
    NORMAL = 0x0,
    #[doc = "Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN."]
    LOOPBACK = 0x01,
}
impl LOOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LOOP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LOOP {
    #[inline(always)]
    fn from(val: u8) -> LOOP {
        LOOP::from_bits(val)
    }
}
impl From<LOOP> for u8 {
    #[inline(always)]
    fn from(val: LOOP) -> u8 {
        LOOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OEPOL {
    #[doc = "Low. If selected by OESEL, the output enable is active low."]
    LOW = 0x0,
    #[doc = "High. If selected by OESEL, the output enable is active high."]
    HIGH = 0x01,
}
impl OEPOL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OEPOL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OEPOL {
    #[inline(always)]
    fn from(val: u8) -> OEPOL {
        OEPOL::from_bits(val)
    }
}
impl From<OEPOL> for u8 {
    #[inline(always)]
    fn from(val: OEPOL) -> u8 {
        OEPOL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OESEL {
    #[doc = "Standard. The RTS signal is used as the standard flow control function."]
    STANDARD = 0x0,
    #[doc = "RS-485. The RTS signal configured to provide an output enable signal to control an RS-485 transceiver."]
    RS_485 = 0x01,
}
impl OESEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OESEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OESEL {
    #[inline(always)]
    fn from(val: u8) -> OESEL {
        OESEL::from_bits(val)
    }
}
impl From<OESEL> for u8 {
    #[inline(always)]
    fn from(val: OESEL) -> u8 {
        OESEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PARITYSEL {
    #[doc = "No parity."]
    NO_PARITY = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even."]
    EVEN_PARITY = 0x02,
    #[doc = "Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd."]
    ODD_PARITY = 0x03,
}
impl PARITYSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PARITYSEL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PARITYSEL {
    #[inline(always)]
    fn from(val: u8) -> PARITYSEL {
        PARITYSEL::from_bits(val)
    }
}
impl From<PARITYSEL> for u8 {
    #[inline(always)]
    fn from(val: PARITYSEL) -> u8 {
        PARITYSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RXPOL {
    #[doc = "Standard. The RX signal is used as it arrives from the pin. This means that the RX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    STANDARD = 0x0,
    #[doc = "Inverted. The RX signal is inverted before being used by the USART. This means that the RX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    INVERTED = 0x01,
}
impl RXPOL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RXPOL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RXPOL {
    #[inline(always)]
    fn from(val: u8) -> RXPOL {
        RXPOL::from_bits(val)
    }
}
impl From<RXPOL> for u8 {
    #[inline(always)]
    fn from(val: RXPOL) -> u8 {
        RXPOL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STOPLEN {
    #[doc = "1 stop bit."]
    BIT_1 = 0x0,
    #[doc = "2 stop bits. This setting should only be used for asynchronous communication."]
    BITS_2 = 0x01,
}
impl STOPLEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STOPLEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STOPLEN {
    #[inline(always)]
    fn from(val: u8) -> STOPLEN {
        STOPLEN::from_bits(val)
    }
}
impl From<STOPLEN> for u8 {
    #[inline(always)]
    fn from(val: STOPLEN) -> u8 {
        STOPLEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYNCEN {
    #[doc = "Asynchronous mode."]
    ASYNCHRONOUS_MODE = 0x0,
    #[doc = "Synchronous mode."]
    SYNCHRONOUS_MODE = 0x01,
}
impl SYNCEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYNCEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYNCEN {
    #[inline(always)]
    fn from(val: u8) -> SYNCEN {
        SYNCEN::from_bits(val)
    }
}
impl From<SYNCEN> for u8 {
    #[inline(always)]
    fn from(val: SYNCEN) -> u8 {
        SYNCEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYNCMST {
    #[doc = "Slave. When synchronous mode is enabled, the USART is a slave."]
    SLAVE = 0x0,
    #[doc = "Master. When synchronous mode is enabled, the USART is a master."]
    MASTER = 0x01,
}
impl SYNCMST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYNCMST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYNCMST {
    #[inline(always)]
    fn from(val: u8) -> SYNCMST {
        SYNCMST::from_bits(val)
    }
}
impl From<SYNCMST> for u8 {
    #[inline(always)]
    fn from(val: SYNCMST) -> u8 {
        SYNCMST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TXBRKEN {
    #[doc = "Normal operation."]
    NORMAL = 0x0,
    #[doc = "Continuous break. Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    CONTINOUS = 0x01,
}
impl TXBRKEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TXBRKEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TXBRKEN {
    #[inline(always)]
    fn from(val: u8) -> TXBRKEN {
        TXBRKEN::from_bits(val)
    }
}
impl From<TXBRKEN> for u8 {
    #[inline(always)]
    fn from(val: TXBRKEN) -> u8 {
        TXBRKEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TXDIS {
    #[doc = "Not disabled. USART transmitter is not disabled."]
    ENABLED = 0x0,
    #[doc = "Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    DISABLED = 0x01,
}
impl TXDIS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TXDIS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TXDIS {
    #[inline(always)]
    fn from(val: u8) -> TXDIS {
        TXDIS::from_bits(val)
    }
}
impl From<TXDIS> for u8 {
    #[inline(always)]
    fn from(val: TXDIS) -> u8 {
        TXDIS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TXPOL {
    #[doc = "Standard. The TX signal is sent out without change. This means that the TX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    STANDARD = 0x0,
    #[doc = "Inverted. The TX signal is inverted by the USART before being sent out. This means that the TX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    INVERTED = 0x01,
}
impl TXPOL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TXPOL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TXPOL {
    #[inline(always)]
    fn from(val: u8) -> TXPOL {
        TXPOL::from_bits(val)
    }
}
impl From<TXPOL> for u8 {
    #[inline(always)]
    fn from(val: TXPOL) -> u8 {
        TXPOL::to_bits(val)
    }
}
