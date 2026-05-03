#[doc = "Configuration register 1 for the primary channel pair."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFG1(pub u32);
impl CFG1 {
    #[doc = "Main enable for I 2S function in this Flexcomm."]
    #[must_use]
    #[inline(always)]
    pub const fn MAINENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Main enable for I 2S function in this Flexcomm."]
    #[inline(always)]
    pub const fn set_MAINENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Data flow Pause. Allows pausing data flow between the I2S serializer/deserializer and the FIFO. This could be done in order to change streams, or while restarting after a data underflow or overflow. When paused, FIFO operations can be done without corrupting data that is in the process of being sent or received. Once a data pause has been requested, the interface may need to complete sending data that was in progress before interrupting the flow of data. Software must check that the pause is actually in effect before taking action. This is done by monitoring the DATAPAUSED flag in the STAT register. When DATAPAUSE is cleared, data transfer will resume at the beginning of the next frame."]
    #[must_use]
    #[inline(always)]
    pub const fn DATAPAUSE(&self) -> super::vals::DATAPAUSE {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DATAPAUSE::from_bits(val as u8)
    }
    #[doc = "Data flow Pause. Allows pausing data flow between the I2S serializer/deserializer and the FIFO. This could be done in order to change streams, or while restarting after a data underflow or overflow. When paused, FIFO operations can be done without corrupting data that is in the process of being sent or received. Once a data pause has been requested, the interface may need to complete sending data that was in progress before interrupting the flow of data. Software must check that the pause is actually in effect before taking action. This is done by monitoring the DATAPAUSED flag in the STAT register. When DATAPAUSE is cleared, data transfer will resume at the beginning of the next frame."]
    #[inline(always)]
    pub const fn set_DATAPAUSE(&mut self, val: super::vals::DATAPAUSE) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Provides the number of I2S channel pairs in this Flexcomm This is a read-only field whose value may be different in other Flexcomms. 00 = there is 1 I2S channel pair in this Flexcomm. 01 = there are 2 I2S channel pairs in this Flexcomm. 10 = there are 3 I2S channel pairs in this Flexcomm. 11 = there are 4 I2S channel pairs in this Flexcomm."]
    #[must_use]
    #[inline(always)]
    pub const fn PAIRCOUNT(&self) -> super::vals::PAIRCOUNT {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::PAIRCOUNT::from_bits(val as u8)
    }
    #[doc = "Provides the number of I2S channel pairs in this Flexcomm This is a read-only field whose value may be different in other Flexcomms. 00 = there is 1 I2S channel pair in this Flexcomm. 01 = there are 2 I2S channel pairs in this Flexcomm. 10 = there are 3 I2S channel pairs in this Flexcomm. 11 = there are 4 I2S channel pairs in this Flexcomm."]
    #[inline(always)]
    pub const fn set_PAIRCOUNT(&mut self, val: super::vals::PAIRCOUNT) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Master / slave configuration selection, determining how SCK and WS are used by all channel pairs in this Flexcomm."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTSLVCFG(&self) -> super::vals::MSTSLVCFG {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MSTSLVCFG::from_bits(val as u8)
    }
    #[doc = "Master / slave configuration selection, determining how SCK and WS are used by all channel pairs in this Flexcomm."]
    #[inline(always)]
    pub const fn set_MSTSLVCFG(&mut self, val: super::vals::MSTSLVCFG) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Selects the basic I2S operating mode. Other configurations modify this to obtain all supported cases. See Formats and modes for examples."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::MODE {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MODE::from_bits(val as u8)
    }
    #[doc = "Selects the basic I2S operating mode. Other configurations modify this to obtain all supported cases. See Formats and modes for examples."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::MODE) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Right channel data is in the Low portion of FIFO data. Essentially, this swaps left and right channel data as it is transferred to or from the FIFO. This bit is not used if the data width is greater than 24 bits or if PDMDATA = 1. Note that if the ONECHANNEL field (bit 10 of this register) = 1, the one channel to be used is the nominally the left channel. POSITION can still place that data in the frame where right channel data is normally located. if all enabled channel pairs have ONECHANNEL = 1, then RIGHTLOW = 1 is not allowed."]
    #[must_use]
    #[inline(always)]
    pub const fn RIGHTLOW(&self) -> super::vals::RIGHTLOW {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::RIGHTLOW::from_bits(val as u8)
    }
    #[doc = "Right channel data is in the Low portion of FIFO data. Essentially, this swaps left and right channel data as it is transferred to or from the FIFO. This bit is not used if the data width is greater than 24 bits or if PDMDATA = 1. Note that if the ONECHANNEL field (bit 10 of this register) = 1, the one channel to be used is the nominally the left channel. POSITION can still place that data in the frame where right channel data is normally located. if all enabled channel pairs have ONECHANNEL = 1, then RIGHTLOW = 1 is not allowed."]
    #[inline(always)]
    pub const fn set_RIGHTLOW(&mut self, val: super::vals::RIGHTLOW) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Left Justify data."]
    #[must_use]
    #[inline(always)]
    pub const fn LEFTJUST(&self) -> super::vals::LEFTJUST {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::LEFTJUST::from_bits(val as u8)
    }
    #[doc = "Left Justify data."]
    #[inline(always)]
    pub const fn set_LEFTJUST(&mut self, val: super::vals::LEFTJUST) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Single channel mode. Applies to both transmit and receive. This configuration bit applies only to the first I2S channel pair. Other channel pairs may select this mode independently in their separate CFG1 registers."]
    #[must_use]
    #[inline(always)]
    pub const fn ONECHANNEL(&self) -> super::vals::ONECHANNEL {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::ONECHANNEL::from_bits(val as u8)
    }
    #[doc = "Single channel mode. Applies to both transmit and receive. This configuration bit applies only to the first I2S channel pair. Other channel pairs may select this mode independently in their separate CFG1 registers."]
    #[inline(always)]
    pub const fn set_ONECHANNEL(&mut self, val: super::vals::ONECHANNEL) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "SCK polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn SCK_POL(&self) -> super::vals::SCK_POL {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::SCK_POL::from_bits(val as u8)
    }
    #[doc = "SCK polarity."]
    #[inline(always)]
    pub const fn set_SCK_POL(&mut self, val: super::vals::SCK_POL) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "WS polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn WS_POL(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "WS polarity."]
    #[inline(always)]
    pub const fn set_WS_POL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Data Length, minus 1 encoded, defines the number of data bits to be transmitted or received for all I2S channel pairs in this Flexcomm. Note that data is only driven to or received from SDA for the number of bits defined by DATALEN. DATALEN is also used in these ways by the I2S: Determines the size of data transfers between the FIFO and the I2S serializer/deserializer. See FIFO buffer configurations and usage In mode 1, 2, and 3, determines the location of right data following left data in the frame. In mode 3 (where WS has a one data slot long pulse at the beginning of each data frame) determines the duration of the WS pulse. Values: 0x00 to 0x02 = not supported 0x03 = data is 4 bits in length 0x04 = data is 5 bits in length 0x1F = data is 32 bits in length."]
    #[must_use]
    #[inline(always)]
    pub const fn DATALEN(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Data Length, minus 1 encoded, defines the number of data bits to be transmitted or received for all I2S channel pairs in this Flexcomm. Note that data is only driven to or received from SDA for the number of bits defined by DATALEN. DATALEN is also used in these ways by the I2S: Determines the size of data transfers between the FIFO and the I2S serializer/deserializer. See FIFO buffer configurations and usage In mode 1, 2, and 3, determines the location of right data following left data in the frame. In mode 3 (where WS has a one data slot long pulse at the beginning of each data frame) determines the duration of the WS pulse. Values: 0x00 to 0x02 = not supported 0x03 = data is 4 bits in length 0x04 = data is 5 bits in length 0x1F = data is 32 bits in length."]
    #[inline(always)]
    pub const fn set_DATALEN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for CFG1 {
    #[inline(always)]
    fn default() -> CFG1 {
        CFG1(0)
    }
}
impl core::fmt::Debug for CFG1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG1")
            .field("MAINENABLE", &self.MAINENABLE())
            .field("DATAPAUSE", &self.DATAPAUSE())
            .field("PAIRCOUNT", &self.PAIRCOUNT())
            .field("MSTSLVCFG", &self.MSTSLVCFG())
            .field("MODE", &self.MODE())
            .field("RIGHTLOW", &self.RIGHTLOW())
            .field("LEFTJUST", &self.LEFTJUST())
            .field("ONECHANNEL", &self.ONECHANNEL())
            .field("SCK_POL", &self.SCK_POL())
            .field("WS_POL", &self.WS_POL())
            .field("DATALEN", &self.DATALEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CFG1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CFG1 {{ MAINENABLE: {=bool:?}, DATAPAUSE: {:?}, PAIRCOUNT: {:?}, MSTSLVCFG: {:?}, MODE: {:?}, RIGHTLOW: {:?}, LEFTJUST: {:?}, ONECHANNEL: {:?}, SCK_POL: {:?}, WS_POL: {=bool:?}, DATALEN: {=u8:?} }}",
            self.MAINENABLE(),
            self.DATAPAUSE(),
            self.PAIRCOUNT(),
            self.MSTSLVCFG(),
            self.MODE(),
            self.RIGHTLOW(),
            self.LEFTJUST(),
            self.ONECHANNEL(),
            self.SCK_POL(),
            self.WS_POL(),
            self.DATALEN()
        )
    }
}
#[doc = "Configuration register 2 for the primary channel pair."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFG2(pub u32);
impl CFG2 {
    #[doc = "Frame Length, minus 1 encoded, defines the number of clocks and data bits in the frames that this channel pair participates in. See Frame format. 0x000 to 0x002 = not supported 0x003 = frame is 4 bits in total length 0x004 = frame is 5 bits in total length 0x1FF = frame is 512 bits in total length if FRAMELEN is an defines an odd length frame (e.g. 33 clocks) in mode 0 or 1, the extra clock appears in the right half. When MODE = 3, FRAMELEN must be larger than DATALEN in order for the WS pulse to be generated correctly."]
    #[must_use]
    #[inline(always)]
    pub const fn FRAMELEN(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Frame Length, minus 1 encoded, defines the number of clocks and data bits in the frames that this channel pair participates in. See Frame format. 0x000 to 0x002 = not supported 0x003 = frame is 4 bits in total length 0x004 = frame is 5 bits in total length 0x1FF = frame is 512 bits in total length if FRAMELEN is an defines an odd length frame (e.g. 33 clocks) in mode 0 or 1, the extra clock appears in the right half. When MODE = 3, FRAMELEN must be larger than DATALEN in order for the WS pulse to be generated correctly."]
    #[inline(always)]
    pub const fn set_FRAMELEN(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Data Position. Defines the location within the frame of the data for this channel pair. POSITION + DATALEN must be less than FRAMELEN. See Frame format. When MODE = 0, POSITION defines the location of data in both the left phase and right phase, starting one clock after the WS edge. In other modes, POSITION defines the location of data within the entire frame. ONECHANNEL = 1 while MODE = 0 is a special case, see the description of ONECHANNEL. The combination of DATALEN and the POSITION fields of all channel pairs must be made such that the channels do not overlap within the frame. 0x000 = data begins at bit position 0 (the first bit position) within the frame or WS phase. 0x001 = data begins at bit position 1 within the frame or WS phase. 0x002 = data begins at bit position 2 within the frame or WS phase."]
    #[must_use]
    #[inline(always)]
    pub const fn POSITION(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "Data Position. Defines the location within the frame of the data for this channel pair. POSITION + DATALEN must be less than FRAMELEN. See Frame format. When MODE = 0, POSITION defines the location of data in both the left phase and right phase, starting one clock after the WS edge. In other modes, POSITION defines the location of data within the entire frame. ONECHANNEL = 1 while MODE = 0 is a special case, see the description of ONECHANNEL. The combination of DATALEN and the POSITION fields of all channel pairs must be made such that the channels do not overlap within the frame. 0x000 = data begins at bit position 0 (the first bit position) within the frame or WS phase. 0x001 = data begins at bit position 1 within the frame or WS phase. 0x002 = data begins at bit position 2 within the frame or WS phase."]
    #[inline(always)]
    pub const fn set_POSITION(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
}
impl Default for CFG2 {
    #[inline(always)]
    fn default() -> CFG2 {
        CFG2(0)
    }
}
impl core::fmt::Debug for CFG2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG2")
            .field("FRAMELEN", &self.FRAMELEN())
            .field("POSITION", &self.POSITION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CFG2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CFG2 {{ FRAMELEN: {=u16:?}, POSITION: {=u16:?} }}",
            self.FRAMELEN(),
            self.POSITION()
        )
    }
}
#[doc = "Clock divider, used by all channel pairs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DIV(pub u32);
impl DIV {
    #[doc = "This field controls how this I2S block uses the Flexcomm function clock. 0x000 = The Flexcomm function clock is used directly. 0x001 = The Flexcomm function clock is divided by 2. 0x002 = The Flexcomm function clock is divided by 3. 0xFFF = The Flexcomm function clock is divided by 4,096."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This field controls how this I2S block uses the Flexcomm function clock. 0x000 = The Flexcomm function clock is used directly. 0x001 = The Flexcomm function clock is divided by 2. 0x002 = The Flexcomm function clock is divided by 3. 0xFFF = The Flexcomm function clock is divided by 4,096."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
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
        f.debug_struct("DIV").field("DIV", &self.DIV()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DIV {{ DIV: {=u16:?} }}", self.DIV())
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
    #[doc = "Transmit I2S empty 0. Determines the value sent by the I2S in transmit mode if the TX FIFO becomes empty. This value is sent repeatedly until the I2S is paused, the error is cleared, new data is provided, and the I2S is un-paused."]
    #[must_use]
    #[inline(always)]
    pub const fn TXI2SE0(&self) -> super::vals::TXI2SE0 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::TXI2SE0::from_bits(val as u8)
    }
    #[doc = "Transmit I2S empty 0. Determines the value sent by the I2S in transmit mode if the TX FIFO becomes empty. This value is sent repeatedly until the I2S is paused, the error is cleared, new data is provided, and the I2S is un-paused."]
    #[inline(always)]
    pub const fn set_TXI2SE0(&mut self, val: super::vals::TXI2SE0) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Packing format for 48-bit data. This relates to how data is entered into or taken from the FIFO by software or DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn PACK48(&self) -> super::vals::PACK48 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PACK48::from_bits(val as u8)
    }
    #[doc = "Packing format for 48-bit data. This relates to how data is entered into or taken from the FIFO by software or DMA."]
    #[inline(always)]
    pub const fn set_PACK48(&mut self, val: super::vals::PACK48) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
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
            .field("TXI2SE0", &self.TXI2SE0())
            .field("PACK48", &self.PACK48())
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
            "FIFOCFG {{ ENABLETX: {=bool:?}, ENABLERX: {=bool:?}, TXI2SE0: {:?}, PACK48: {:?}, SIZE: {=u8:?}, DMATX: {=bool:?}, DMARX: {=bool:?}, WAKETX: {=bool:?}, WAKERX: {=bool:?}, EMPTYTX: {=bool:?}, EMPTYRX: {=bool:?} }}",
            self.ENABLETX(),
            self.ENABLERX(),
            self.TXI2SE0(),
            self.PACK48(),
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
    #[doc = "Received data from the FIFO. The number of bits used depends on configuration details."]
    #[must_use]
    #[inline(always)]
    pub const fn RXDATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Received data from the FIFO. The number of bits used depends on configuration details."]
    #[inline(always)]
    pub const fn set_RXDATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFORD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FIFORD {{ RXDATA: {=u32:?} }}", self.RXDATA())
    }
}
#[doc = "FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIFORD48H(pub u32);
impl FIFORD48H {
    #[doc = "Received data from the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[must_use]
    #[inline(always)]
    pub const fn RXDATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Received data from the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[inline(always)]
    pub const fn set_RXDATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for FIFORD48H {
    #[inline(always)]
    fn default() -> FIFORD48H {
        FIFORD48H(0)
    }
}
impl core::fmt::Debug for FIFORD48H {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFORD48H")
            .field("RXDATA", &self.RXDATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFORD48H {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FIFORD48H {{ RXDATA: {=u32:?} }}", self.RXDATA())
    }
}
#[doc = "FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIFORD48HNOPOP(pub u32);
impl FIFORD48HNOPOP {
    #[doc = "Received data from the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[must_use]
    #[inline(always)]
    pub const fn RXDATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Received data from the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[inline(always)]
    pub const fn set_RXDATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for FIFORD48HNOPOP {
    #[inline(always)]
    fn default() -> FIFORD48HNOPOP {
        FIFORD48HNOPOP(0)
    }
}
impl core::fmt::Debug for FIFORD48HNOPOP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFORD48HNOPOP")
            .field("RXDATA", &self.RXDATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFORD48HNOPOP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FIFORD48HNOPOP {{ RXDATA: {=u32:?} }}", self.RXDATA())
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
    pub const fn RXDATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Received data from the FIFO."]
    #[inline(always)]
    pub const fn set_RXDATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFORDNOPOP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FIFORDNOPOP {{ RXDATA: {=u32:?} }}", self.RXDATA())
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
    #[doc = "Transmit data to the FIFO. The number of bits used depends on configuration details."]
    #[must_use]
    #[inline(always)]
    pub const fn TXDATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit data to the FIFO. The number of bits used depends on configuration details."]
    #[inline(always)]
    pub const fn set_TXDATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
        defmt::write!(f, "FIFOWR {{ TXDATA: {=u32:?} }}", self.TXDATA())
    }
}
#[doc = "FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIFOWR48H(pub u32);
impl FIFOWR48H {
    #[doc = "Transmit data to the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[must_use]
    #[inline(always)]
    pub const fn TXDATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Transmit data to the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[inline(always)]
    pub const fn set_TXDATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for FIFOWR48H {
    #[inline(always)]
    fn default() -> FIFOWR48H {
        FIFOWR48H(0)
    }
}
impl core::fmt::Debug for FIFOWR48H {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOWR48H")
            .field("TXDATA", &self.TXDATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFOWR48H {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FIFOWR48H {{ TXDATA: {=u32:?} }}", self.TXDATA())
    }
}
#[doc = "I2S Module identification."]
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
    #[doc = "Minor revision of module implementation, starting at 0."]
    #[must_use]
    #[inline(always)]
    pub const fn MINOR_REV(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision of module implementation, starting at 0."]
    #[inline(always)]
    pub const fn set_MINOR_REV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Major revision of module implementation, starting at 0."]
    #[must_use]
    #[inline(always)]
    pub const fn MAJOR_REV(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision of module implementation, starting at 0."]
    #[inline(always)]
    pub const fn set_MAJOR_REV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Unique module identifier for this IP block."]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Unique module identifier for this IP block."]
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
#[doc = "Status register for the primary channel pair."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STAT(pub u32);
impl STAT {
    #[doc = "Busy status for the primary channel pair. Other BUSY flags may be found in the STAT register for each channel pair."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSY(&self) -> super::vals::BUSY {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::BUSY::from_bits(val as u8)
    }
    #[doc = "Busy status for the primary channel pair. Other BUSY flags may be found in the STAT register for each channel pair."]
    #[inline(always)]
    pub const fn set_BUSY(&mut self, val: super::vals::BUSY) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Slave Frame Error flag. This applies when at least one channel pair is operating as a slave. An error indicates that the incoming WS signal did not transition as expected due to a mismatch between FRAMELEN and the actual incoming I2S stream."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVFRMERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Frame Error flag. This applies when at least one channel pair is operating as a slave. An error indicates that the incoming WS signal did not transition as expected due to a mismatch between FRAMELEN and the actual incoming I2S stream."]
    #[inline(always)]
    pub const fn set_SLVFRMERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Left/Right indication. This flag is considered to be a debugging aid and is not expected to be used by an I2S driver. Valid when one channel pair is busy. Indicates left or right data being processed for the currently busy channel pair."]
    #[must_use]
    #[inline(always)]
    pub const fn LR(&self) -> super::vals::LR {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::LR::from_bits(val as u8)
    }
    #[doc = "Left/Right indication. This flag is considered to be a debugging aid and is not expected to be used by an I2S driver. Valid when one channel pair is busy. Indicates left or right data being processed for the currently busy channel pair."]
    #[inline(always)]
    pub const fn set_LR(&mut self, val: super::vals::LR) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Data Paused status flag. Applies to all I2S channels."]
    #[must_use]
    #[inline(always)]
    pub const fn DATAPAUSED(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Data Paused status flag. Applies to all I2S channels."]
    #[inline(always)]
    pub const fn set_DATAPAUSED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
            .field("BUSY", &self.BUSY())
            .field("SLVFRMERR", &self.SLVFRMERR())
            .field("LR", &self.LR())
            .field("DATAPAUSED", &self.DATAPAUSED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STAT {{ BUSY: {:?}, SLVFRMERR: {=bool:?}, LR: {:?}, DATAPAUSED: {=bool:?} }}",
            self.BUSY(),
            self.SLVFRMERR(),
            self.LR(),
            self.DATAPAUSED()
        )
    }
}
