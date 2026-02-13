#[doc = "Configuration register 1 for the primary channel pair."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg1(pub u32);
impl Cfg1 {
    #[doc = "Main enable for I 2S function in this Flexcomm"]
    #[must_use]
    #[inline(always)]
    pub const fn mainenable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Main enable for I 2S function in this Flexcomm"]
    #[inline(always)]
    pub const fn set_mainenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Data flow Pause. Allows pausing data flow between the I2S serializer/deserializer and the FIFO. This could be done in order to change streams, or while restarting after a data underflow or overflow. When paused, FIFO operations can be done without corrupting data that is in the process of being sent or received. Once a data pause has been requested, the interface may need to complete sending data that was in progress before interrupting the flow of data. Software must check that the pause is actually in effect before taking action. This is done by monitoring the DATAPAUSED flag in the STAT register. When DATAPAUSE is cleared, data transfer will resume at the beginning of the next frame."]
    #[must_use]
    #[inline(always)]
    pub const fn datapause(&self) -> super::vals::Datapause {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Datapause::from_bits(val as u8)
    }
    #[doc = "Data flow Pause. Allows pausing data flow between the I2S serializer/deserializer and the FIFO. This could be done in order to change streams, or while restarting after a data underflow or overflow. When paused, FIFO operations can be done without corrupting data that is in the process of being sent or received. Once a data pause has been requested, the interface may need to complete sending data that was in progress before interrupting the flow of data. Software must check that the pause is actually in effect before taking action. This is done by monitoring the DATAPAUSED flag in the STAT register. When DATAPAUSE is cleared, data transfer will resume at the beginning of the next frame."]
    #[inline(always)]
    pub const fn set_datapause(&mut self, val: super::vals::Datapause) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Provides the number of I2S channel pairs in this Flexcomm This is a read-only field whose value may be different in other Flexcomms. 00 = there is 1 I2S channel pair in this Flexcomm. 01 = there are 2 I2S channel pairs in this Flexcomm. 10 = there are 3 I2S channel pairs in this Flexcomm. 11 = there are 4 I2S channel pairs in this Flexcomm."]
    #[must_use]
    #[inline(always)]
    pub const fn paircount(&self) -> super::vals::Paircount {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Paircount::from_bits(val as u8)
    }
    #[doc = "Provides the number of I2S channel pairs in this Flexcomm This is a read-only field whose value may be different in other Flexcomms. 00 = there is 1 I2S channel pair in this Flexcomm. 01 = there are 2 I2S channel pairs in this Flexcomm. 10 = there are 3 I2S channel pairs in this Flexcomm. 11 = there are 4 I2S channel pairs in this Flexcomm."]
    #[inline(always)]
    pub const fn set_paircount(&mut self, val: super::vals::Paircount) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Master / slave configuration selection, determining how SCK and WS are used by all channel pairs in this Flexcomm."]
    #[must_use]
    #[inline(always)]
    pub const fn mstslvcfg(&self) -> super::vals::Mstslvcfg {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Mstslvcfg::from_bits(val as u8)
    }
    #[doc = "Master / slave configuration selection, determining how SCK and WS are used by all channel pairs in this Flexcomm."]
    #[inline(always)]
    pub const fn set_mstslvcfg(&mut self, val: super::vals::Mstslvcfg) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Selects the basic I2S operating mode. Other configurations modify this to obtain all supported cases. See Formats and modes for examples."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Selects the basic I2S operating mode. Other configurations modify this to obtain all supported cases. See Formats and modes for examples."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Right channel data is in the Low portion of FIFO data. Essentially, this swaps left and right channel data as it is transferred to or from the FIFO. This bit is not used if the data width is greater than 24 bits or if PDMDATA = 1. Note that if the ONECHANNEL field (bit 10 of this register) = 1, the one channel to be used is the nominally the left channel. POSITION can still place that data in the frame where right channel data is normally located. if all enabled channel pairs have ONECHANNEL = 1, then RIGHTLOW = 1 is not allowed."]
    #[must_use]
    #[inline(always)]
    pub const fn rightlow(&self) -> super::vals::Rightlow {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Rightlow::from_bits(val as u8)
    }
    #[doc = "Right channel data is in the Low portion of FIFO data. Essentially, this swaps left and right channel data as it is transferred to or from the FIFO. This bit is not used if the data width is greater than 24 bits or if PDMDATA = 1. Note that if the ONECHANNEL field (bit 10 of this register) = 1, the one channel to be used is the nominally the left channel. POSITION can still place that data in the frame where right channel data is normally located. if all enabled channel pairs have ONECHANNEL = 1, then RIGHTLOW = 1 is not allowed."]
    #[inline(always)]
    pub const fn set_rightlow(&mut self, val: super::vals::Rightlow) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Left Justify data."]
    #[must_use]
    #[inline(always)]
    pub const fn leftjust(&self) -> super::vals::Leftjust {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Leftjust::from_bits(val as u8)
    }
    #[doc = "Left Justify data."]
    #[inline(always)]
    pub const fn set_leftjust(&mut self, val: super::vals::Leftjust) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Single channel mode. Applies to both transmit and receive. This configuration bit applies only to the first I2S channel pair. Other channel pairs may select this mode independently in their separate CFG1 registers."]
    #[must_use]
    #[inline(always)]
    pub const fn onechannel(&self) -> super::vals::Onechannel {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Onechannel::from_bits(val as u8)
    }
    #[doc = "Single channel mode. Applies to both transmit and receive. This configuration bit applies only to the first I2S channel pair. Other channel pairs may select this mode independently in their separate CFG1 registers."]
    #[inline(always)]
    pub const fn set_onechannel(&mut self, val: super::vals::Onechannel) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "SCK polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn sck_pol(&self) -> super::vals::SckPol {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::SckPol::from_bits(val as u8)
    }
    #[doc = "SCK polarity."]
    #[inline(always)]
    pub const fn set_sck_pol(&mut self, val: super::vals::SckPol) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "WS polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn ws_pol(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "WS polarity."]
    #[inline(always)]
    pub const fn set_ws_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Data Length, minus 1 encoded, defines the number of data bits to be transmitted or received for all I2S channel pairs in this Flexcomm. Note that data is only driven to or received from SDA for the number of bits defined by DATALEN. DATALEN is also used in these ways by the I2S: Determines the size of data transfers between the FIFO and the I2S serializer/deserializer. See FIFO buffer configurations and usage In mode 1, 2, and 3, determines the location of right data following left data in the frame. In mode 3 (where WS has a one data slot long pulse at the beginning of each data frame) determines the duration of the WS pulse. Values: 0x00 to 0x02 = not supported 0x03 = data is 4 bits in length 0x04 = data is 5 bits in length 0x1F = data is 32 bits in length"]
    #[must_use]
    #[inline(always)]
    pub const fn datalen(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Data Length, minus 1 encoded, defines the number of data bits to be transmitted or received for all I2S channel pairs in this Flexcomm. Note that data is only driven to or received from SDA for the number of bits defined by DATALEN. DATALEN is also used in these ways by the I2S: Determines the size of data transfers between the FIFO and the I2S serializer/deserializer. See FIFO buffer configurations and usage In mode 1, 2, and 3, determines the location of right data following left data in the frame. In mode 3 (where WS has a one data slot long pulse at the beginning of each data frame) determines the duration of the WS pulse. Values: 0x00 to 0x02 = not supported 0x03 = data is 4 bits in length 0x04 = data is 5 bits in length 0x1F = data is 32 bits in length"]
    #[inline(always)]
    pub const fn set_datalen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Cfg1 {
    #[inline(always)]
    fn default() -> Cfg1 {
        Cfg1(0)
    }
}
impl core::fmt::Debug for Cfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg1")
            .field("mainenable", &self.mainenable())
            .field("datapause", &self.datapause())
            .field("paircount", &self.paircount())
            .field("mstslvcfg", &self.mstslvcfg())
            .field("mode", &self.mode())
            .field("rightlow", &self.rightlow())
            .field("leftjust", &self.leftjust())
            .field("onechannel", &self.onechannel())
            .field("sck_pol", &self.sck_pol())
            .field("ws_pol", &self.ws_pol())
            .field("datalen", &self.datalen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg1 {{ mainenable: {=bool:?}, datapause: {:?}, paircount: {:?}, mstslvcfg: {:?}, mode: {:?}, rightlow: {:?}, leftjust: {:?}, onechannel: {:?}, sck_pol: {:?}, ws_pol: {=bool:?}, datalen: {=u8:?} }}",
            self.mainenable(),
            self.datapause(),
            self.paircount(),
            self.mstslvcfg(),
            self.mode(),
            self.rightlow(),
            self.leftjust(),
            self.onechannel(),
            self.sck_pol(),
            self.ws_pol(),
            self.datalen()
        )
    }
}
#[doc = "Configuration register 2 for the primary channel pair."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg2(pub u32);
impl Cfg2 {
    #[doc = "Frame Length, minus 1 encoded, defines the number of clocks and data bits in the frames that this channel pair participates in. See Frame format. 0x000 to 0x002 = not supported 0x003 = frame is 4 bits in total length 0x004 = frame is 5 bits in total length 0x1FF = frame is 512 bits in total length if FRAMELEN is an defines an odd length frame (e.g. 33 clocks) in mode 0 or 1, the extra clock appears in the right half. When MODE = 3, FRAMELEN must be larger than DATALEN in order for the WS pulse to be generated correctly."]
    #[must_use]
    #[inline(always)]
    pub const fn framelen(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Frame Length, minus 1 encoded, defines the number of clocks and data bits in the frames that this channel pair participates in. See Frame format. 0x000 to 0x002 = not supported 0x003 = frame is 4 bits in total length 0x004 = frame is 5 bits in total length 0x1FF = frame is 512 bits in total length if FRAMELEN is an defines an odd length frame (e.g. 33 clocks) in mode 0 or 1, the extra clock appears in the right half. When MODE = 3, FRAMELEN must be larger than DATALEN in order for the WS pulse to be generated correctly."]
    #[inline(always)]
    pub const fn set_framelen(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Data Position. Defines the location within the frame of the data for this channel pair. POSITION + DATALEN must be less than FRAMELEN. See Frame format. When MODE = 0, POSITION defines the location of data in both the left phase and right phase, starting one clock after the WS edge. In other modes, POSITION defines the location of data within the entire frame. ONECHANNEL = 1 while MODE = 0 is a special case, see the description of ONECHANNEL. The combination of DATALEN and the POSITION fields of all channel pairs must be made such that the channels do not overlap within the frame. 0x000 = data begins at bit position 0 (the first bit position) within the frame or WS phase. 0x001 = data begins at bit position 1 within the frame or WS phase. 0x002 = data begins at bit position 2 within the frame or WS phase."]
    #[must_use]
    #[inline(always)]
    pub const fn position(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "Data Position. Defines the location within the frame of the data for this channel pair. POSITION + DATALEN must be less than FRAMELEN. See Frame format. When MODE = 0, POSITION defines the location of data in both the left phase and right phase, starting one clock after the WS edge. In other modes, POSITION defines the location of data within the entire frame. ONECHANNEL = 1 while MODE = 0 is a special case, see the description of ONECHANNEL. The combination of DATALEN and the POSITION fields of all channel pairs must be made such that the channels do not overlap within the frame. 0x000 = data begins at bit position 0 (the first bit position) within the frame or WS phase. 0x001 = data begins at bit position 1 within the frame or WS phase. 0x002 = data begins at bit position 2 within the frame or WS phase."]
    #[inline(always)]
    pub const fn set_position(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
}
impl Default for Cfg2 {
    #[inline(always)]
    fn default() -> Cfg2 {
        Cfg2(0)
    }
}
impl core::fmt::Debug for Cfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg2")
            .field("framelen", &self.framelen())
            .field("position", &self.position())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg2 {{ framelen: {=u16:?}, position: {=u16:?} }}",
            self.framelen(),
            self.position()
        )
    }
}
#[doc = "Clock divider, used by all channel pairs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Div(pub u32);
impl Div {
    #[doc = "This field controls how this I2S block uses the Flexcomm function clock. 0x000 = The Flexcomm function clock is used directly. 0x001 = The Flexcomm function clock is divided by 2. 0x002 = The Flexcomm function clock is divided by 3. 0xFFF = The Flexcomm function clock is divided by 4,096."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This field controls how this I2S block uses the Flexcomm function clock. 0x000 = The Flexcomm function clock is used directly. 0x001 = The Flexcomm function clock is divided by 2. 0x002 = The Flexcomm function clock is divided by 3. 0xFFF = The Flexcomm function clock is divided by 4,096."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
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
        f.debug_struct("Div").field("div", &self.div()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Div {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Div {{ div: {=u16:?} }}", self.div())
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
    #[doc = "Transmit I2S empty 0. Determines the value sent by the I2S in transmit mode if the TX FIFO becomes empty. This value is sent repeatedly until the I2S is paused, the error is cleared, new data is provided, and the I2S is un-paused."]
    #[must_use]
    #[inline(always)]
    pub const fn txi2se0(&self) -> super::vals::Txi2se0 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Txi2se0::from_bits(val as u8)
    }
    #[doc = "Transmit I2S empty 0. Determines the value sent by the I2S in transmit mode if the TX FIFO becomes empty. This value is sent repeatedly until the I2S is paused, the error is cleared, new data is provided, and the I2S is un-paused."]
    #[inline(always)]
    pub const fn set_txi2se0(&mut self, val: super::vals::Txi2se0) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Packing format for 48-bit data. This relates to how data is entered into or taken from the FIFO by software or DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn pack48(&self) -> super::vals::Pack48 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pack48::from_bits(val as u8)
    }
    #[doc = "Packing format for 48-bit data. This relates to how data is entered into or taken from the FIFO by software or DMA."]
    #[inline(always)]
    pub const fn set_pack48(&mut self, val: super::vals::Pack48) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
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
            .field("txi2se0", &self.txi2se0())
            .field("pack48", &self.pack48())
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
            "Fifocfg {{ enabletx: {=bool:?}, enablerx: {=bool:?}, txi2se0: {:?}, pack48: {:?}, size: {=u8:?}, dmatx: {=bool:?}, dmarx: {=bool:?}, waketx: {=bool:?}, wakerx: {=bool:?}, emptytx: {=bool:?}, emptyrx: {=bool:?} }}",
            self.enabletx(),
            self.enablerx(),
            self.txi2se0(),
            self.pack48(),
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
    #[doc = "Received data from the FIFO. The number of bits used depends on configuration details."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Received data from the FIFO. The number of bits used depends on configuration details."]
    #[inline(always)]
    pub const fn set_rxdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fiford {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fiford {{ rxdata: {=u32:?} }}", self.rxdata())
    }
}
#[doc = "FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fiford48h(pub u32);
impl Fiford48h {
    #[doc = "Received data from the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Received data from the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[inline(always)]
    pub const fn set_rxdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Fiford48h {
    #[inline(always)]
    fn default() -> Fiford48h {
        Fiford48h(0)
    }
}
impl core::fmt::Debug for Fiford48h {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fiford48h")
            .field("rxdata", &self.rxdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fiford48h {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fiford48h {{ rxdata: {=u32:?} }}", self.rxdata())
    }
}
#[doc = "FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fiford48hnopop(pub u32);
impl Fiford48hnopop {
    #[doc = "Received data from the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Received data from the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[inline(always)]
    pub const fn set_rxdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Fiford48hnopop {
    #[inline(always)]
    fn default() -> Fiford48hnopop {
        Fiford48hnopop(0)
    }
}
impl core::fmt::Debug for Fiford48hnopop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fiford48hnopop")
            .field("rxdata", &self.rxdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fiford48hnopop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fiford48hnopop {{ rxdata: {=u32:?} }}", self.rxdata())
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
    pub const fn rxdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Received data from the FIFO."]
    #[inline(always)]
    pub const fn set_rxdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifordnopop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fifordnopop {{ rxdata: {=u32:?} }}", self.rxdata())
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
    #[doc = "Transmit data to the FIFO. The number of bits used depends on configuration details."]
    #[must_use]
    #[inline(always)]
    pub const fn txdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit data to the FIFO. The number of bits used depends on configuration details."]
    #[inline(always)]
    pub const fn set_txdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
        defmt::write!(f, "Fifowr {{ txdata: {=u32:?} }}", self.txdata())
    }
}
#[doc = "FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifowr48h(pub u32);
impl Fifowr48h {
    #[doc = "Transmit data to the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[must_use]
    #[inline(always)]
    pub const fn txdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Transmit data to the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[inline(always)]
    pub const fn set_txdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Fifowr48h {
    #[inline(always)]
    fn default() -> Fifowr48h {
        Fifowr48h(0)
    }
}
impl core::fmt::Debug for Fifowr48h {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifowr48h")
            .field("txdata", &self.txdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifowr48h {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fifowr48h {{ txdata: {=u32:?} }}", self.txdata())
    }
}
#[doc = "I2S Module identification"]
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
    #[doc = "Minor revision of module implementation, starting at 0."]
    #[must_use]
    #[inline(always)]
    pub const fn minor_rev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision of module implementation, starting at 0."]
    #[inline(always)]
    pub const fn set_minor_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Major revision of module implementation, starting at 0."]
    #[must_use]
    #[inline(always)]
    pub const fn major_rev(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision of module implementation, starting at 0."]
    #[inline(always)]
    pub const fn set_major_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Unique module identifier for this IP block."]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Unique module identifier for this IP block."]
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
#[doc = "Status register for the primary channel pair."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Busy status for the primary channel pair. Other BUSY flags may be found in the STAT register for each channel pair."]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> super::vals::Busy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Busy::from_bits(val as u8)
    }
    #[doc = "Busy status for the primary channel pair. Other BUSY flags may be found in the STAT register for each channel pair."]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: super::vals::Busy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Slave Frame Error flag. This applies when at least one channel pair is operating as a slave. An error indicates that the incoming WS signal did not transition as expected due to a mismatch between FRAMELEN and the actual incoming I2S stream."]
    #[must_use]
    #[inline(always)]
    pub const fn slvfrmerr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Frame Error flag. This applies when at least one channel pair is operating as a slave. An error indicates that the incoming WS signal did not transition as expected due to a mismatch between FRAMELEN and the actual incoming I2S stream."]
    #[inline(always)]
    pub const fn set_slvfrmerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Left/Right indication. This flag is considered to be a debugging aid and is not expected to be used by an I2S driver. Valid when one channel pair is busy. Indicates left or right data being processed for the currently busy channel pair."]
    #[must_use]
    #[inline(always)]
    pub const fn lr(&self) -> super::vals::Lr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Lr::from_bits(val as u8)
    }
    #[doc = "Left/Right indication. This flag is considered to be a debugging aid and is not expected to be used by an I2S driver. Valid when one channel pair is busy. Indicates left or right data being processed for the currently busy channel pair."]
    #[inline(always)]
    pub const fn set_lr(&mut self, val: super::vals::Lr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Data Paused status flag. Applies to all I2S channels"]
    #[must_use]
    #[inline(always)]
    pub const fn datapaused(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Data Paused status flag. Applies to all I2S channels"]
    #[inline(always)]
    pub const fn set_datapaused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
            .field("busy", &self.busy())
            .field("slvfrmerr", &self.slvfrmerr())
            .field("lr", &self.lr())
            .field("datapaused", &self.datapaused())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stat {{ busy: {:?}, slvfrmerr: {=bool:?}, lr: {:?}, datapaused: {=bool:?} }}",
            self.busy(),
            self.slvfrmerr(),
            self.lr(),
            self.datapaused()
        )
    }
}
