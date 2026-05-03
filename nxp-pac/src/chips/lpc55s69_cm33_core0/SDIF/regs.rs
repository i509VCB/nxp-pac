#[doc = "Power control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BACKENDPWR(pub u32);
impl BACKENDPWR {
    #[doc = "Back-end Power control for card application."]
    #[must_use]
    #[inline(always)]
    pub const fn BACKENDPWR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Back-end Power control for card application."]
    #[inline(always)]
    pub const fn set_BACKENDPWR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for BACKENDPWR {
    #[inline(always)]
    fn default() -> BACKENDPWR {
        BACKENDPWR(0)
    }
}
impl core::fmt::Debug for BACKENDPWR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKENDPWR")
            .field("BACKENDPWR", &self.BACKENDPWR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BACKENDPWR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BACKENDPWR {{ BACKENDPWR: {=bool:?} }}",
            self.BACKENDPWR()
        )
    }
}
#[doc = "Block Size register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BLKSIZ(pub u32);
impl BLKSIZ {
    #[doc = "Block size."]
    #[must_use]
    #[inline(always)]
    pub const fn BLOCK_SIZE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Block size."]
    #[inline(always)]
    pub const fn set_BLOCK_SIZE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for BLKSIZ {
    #[inline(always)]
    fn default() -> BLKSIZ {
        BLKSIZ(0)
    }
}
impl core::fmt::Debug for BLKSIZ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLKSIZ")
            .field("BLOCK_SIZE", &self.BLOCK_SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BLKSIZ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BLKSIZ {{ BLOCK_SIZE: {=u16:?} }}", self.BLOCK_SIZE())
    }
}
#[doc = "Bus Mode register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BMOD(pub u32);
impl BMOD {
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn SWR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_SWR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Fixed Burst."]
    #[must_use]
    #[inline(always)]
    pub const fn FB(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Fixed Burst."]
    #[inline(always)]
    pub const fn set_FB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Descriptor Skip Length."]
    #[must_use]
    #[inline(always)]
    pub const fn DSL(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x1f;
        val as u8
    }
    #[doc = "Descriptor Skip Length."]
    #[inline(always)]
    pub const fn set_DSL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
    }
    #[doc = "SD/MMC DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SD/MMC DMA Enable."]
    #[inline(always)]
    pub const fn set_DE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Programmable Burst Length."]
    #[must_use]
    #[inline(always)]
    pub const fn PBL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Programmable Burst Length."]
    #[inline(always)]
    pub const fn set_PBL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for BMOD {
    #[inline(always)]
    fn default() -> BMOD {
        BMOD(0)
    }
}
impl core::fmt::Debug for BMOD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BMOD")
            .field("SWR", &self.SWR())
            .field("FB", &self.FB())
            .field("DSL", &self.DSL())
            .field("DE", &self.DE())
            .field("PBL", &self.PBL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BMOD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BMOD {{ SWR: {=bool:?}, FB: {=bool:?}, DSL: {=u8:?}, DE: {=bool:?}, PBL: {=u8:?} }}",
            self.SWR(),
            self.FB(),
            self.DSL(),
            self.DE(),
            self.PBL()
        )
    }
}
#[doc = "Current Buffer Descriptor Address register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BUFADDR(pub u32);
impl BUFADDR {
    #[doc = "Host Buffer Address Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn HBA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Host Buffer Address Pointer."]
    #[inline(always)]
    pub const fn set_HBA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BUFADDR {
    #[inline(always)]
    fn default() -> BUFADDR {
        BUFADDR(0)
    }
}
impl core::fmt::Debug for BUFADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUFADDR").field("HBA", &self.HBA()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BUFADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BUFADDR {{ HBA: {=u32:?} }}", self.HBA())
    }
}
#[doc = "Byte Count register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BYTCNT(pub u32);
impl BYTCNT {
    #[doc = "Number of bytes to be transferred; should be integer multiple of Block Size for block transfers."]
    #[must_use]
    #[inline(always)]
    pub const fn BYTE_COUNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Number of bytes to be transferred; should be integer multiple of Block Size for block transfers."]
    #[inline(always)]
    pub const fn set_BYTE_COUNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BYTCNT {
    #[inline(always)]
    fn default() -> BYTCNT {
        BYTCNT(0)
    }
}
impl core::fmt::Debug for BYTCNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BYTCNT")
            .field("BYTE_COUNT", &self.BYTE_COUNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BYTCNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BYTCNT {{ BYTE_COUNT: {=u32:?} }}", self.BYTE_COUNT())
    }
}
#[doc = "Card Threshold Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CARDTHRCTL(pub u32);
impl CARDTHRCTL {
    #[doc = "Card Read Threshold Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CARDRDTHREN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Card Read Threshold Enable."]
    #[inline(always)]
    pub const fn set_CARDRDTHREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Busy Clear Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn BSYCLRINTEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Busy Clear Interrupt Enable."]
    #[inline(always)]
    pub const fn set_BSYCLRINTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Card Threshold size."]
    #[must_use]
    #[inline(always)]
    pub const fn CARDTHRESHOLD(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Card Threshold size."]
    #[inline(always)]
    pub const fn set_CARDTHRESHOLD(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for CARDTHRCTL {
    #[inline(always)]
    fn default() -> CARDTHRCTL {
        CARDTHRCTL(0)
    }
}
impl core::fmt::Debug for CARDTHRCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CARDTHRCTL")
            .field("CARDRDTHREN", &self.CARDRDTHREN())
            .field("BSYCLRINTEN", &self.BSYCLRINTEN())
            .field("CARDTHRESHOLD", &self.CARDTHRESHOLD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CARDTHRCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CARDTHRCTL {{ CARDRDTHREN: {=bool:?}, BSYCLRINTEN: {=bool:?}, CARDTHRESHOLD: {=u8:?} }}",
            self.CARDRDTHREN(),
            self.BSYCLRINTEN(),
            self.CARDTHRESHOLD()
        )
    }
}
#[doc = "Card Detect register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CDETECT(pub u32);
impl CDETECT {
    #[doc = "Card 0 detect."]
    #[must_use]
    #[inline(always)]
    pub const fn CARD0_DETECT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Card 0 detect."]
    #[inline(always)]
    pub const fn set_CARD0_DETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Card 1 detect."]
    #[must_use]
    #[inline(always)]
    pub const fn CARD1_DETECT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Card 1 detect."]
    #[inline(always)]
    pub const fn set_CARD1_DETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for CDETECT {
    #[inline(always)]
    fn default() -> CDETECT {
        CDETECT(0)
    }
}
impl core::fmt::Debug for CDETECT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDETECT")
            .field("CARD0_DETECT", &self.CARD0_DETECT())
            .field("CARD1_DETECT", &self.CARD1_DETECT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CDETECT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CDETECT {{ CARD0_DETECT: {=bool:?}, CARD1_DETECT: {=bool:?} }}",
            self.CARD0_DETECT(),
            self.CARD1_DETECT()
        )
    }
}
#[doc = "Clock Divider register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLKDIV(pub u32);
impl CLKDIV {
    #[doc = "Clock divider-0 value."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_DIVIDER0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider-0 value."]
    #[inline(always)]
    pub const fn set_CLK_DIVIDER0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for CLKDIV {
    #[inline(always)]
    fn default() -> CLKDIV {
        CLKDIV(0)
    }
}
impl core::fmt::Debug for CLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKDIV")
            .field("CLK_DIVIDER0", &self.CLK_DIVIDER0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CLKDIV {{ CLK_DIVIDER0: {=u8:?} }}", self.CLK_DIVIDER0())
    }
}
#[doc = "Clock Enable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLKENA(pub u32);
impl CLKENA {
    #[doc = "Clock-enable control for SD card 0 clock."]
    #[must_use]
    #[inline(always)]
    pub const fn CCLK0_ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clock-enable control for SD card 0 clock."]
    #[inline(always)]
    pub const fn set_CCLK0_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clock-enable control for SD card 1 clock."]
    #[must_use]
    #[inline(always)]
    pub const fn CCLK1_ENABLE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clock-enable control for SD card 1 clock."]
    #[inline(always)]
    pub const fn set_CCLK1_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Low-power control for SD card 0 clock."]
    #[must_use]
    #[inline(always)]
    pub const fn CCLK0_LOW_POWER(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Low-power control for SD card 0 clock."]
    #[inline(always)]
    pub const fn set_CCLK0_LOW_POWER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Low-power control for SD card 1 clock."]
    #[must_use]
    #[inline(always)]
    pub const fn CCLK1_LOW_POWER(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Low-power control for SD card 1 clock."]
    #[inline(always)]
    pub const fn set_CCLK1_LOW_POWER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for CLKENA {
    #[inline(always)]
    fn default() -> CLKENA {
        CLKENA(0)
    }
}
impl core::fmt::Debug for CLKENA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKENA")
            .field("CCLK0_ENABLE", &self.CCLK0_ENABLE())
            .field("CCLK1_ENABLE", &self.CCLK1_ENABLE())
            .field("CCLK0_LOW_POWER", &self.CCLK0_LOW_POWER())
            .field("CCLK1_LOW_POWER", &self.CCLK1_LOW_POWER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLKENA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CLKENA {{ CCLK0_ENABLE: {=bool:?}, CCLK1_ENABLE: {=bool:?}, CCLK0_LOW_POWER: {=bool:?}, CCLK1_LOW_POWER: {=bool:?} }}",
            self.CCLK0_ENABLE(),
            self.CCLK1_ENABLE(),
            self.CCLK0_LOW_POWER(),
            self.CCLK1_LOW_POWER()
        )
    }
}
#[doc = "Command register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMD(pub u32);
impl CMD {
    #[doc = "Command index."]
    #[must_use]
    #[inline(always)]
    pub const fn CMD_INDEX(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Command index."]
    #[inline(always)]
    pub const fn set_CMD_INDEX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Response expect."]
    #[must_use]
    #[inline(always)]
    pub const fn RESPONSE_EXPECT(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Response expect."]
    #[inline(always)]
    pub const fn set_RESPONSE_EXPECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Response length."]
    #[must_use]
    #[inline(always)]
    pub const fn RESPONSE_LENGTH(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Response length."]
    #[inline(always)]
    pub const fn set_RESPONSE_LENGTH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Check response CRC."]
    #[must_use]
    #[inline(always)]
    pub const fn CHECK_RESPONSE_CRC(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Check response CRC."]
    #[inline(always)]
    pub const fn set_CHECK_RESPONSE_CRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Data expected."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA_EXPECTED(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Data expected."]
    #[inline(always)]
    pub const fn set_DATA_EXPECTED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "read/write."]
    #[must_use]
    #[inline(always)]
    pub const fn READ_WRITE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "read/write."]
    #[inline(always)]
    pub const fn set_READ_WRITE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Transfer mode."]
    #[must_use]
    #[inline(always)]
    pub const fn TRANSFER_MODE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer mode."]
    #[inline(always)]
    pub const fn set_TRANSFER_MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Send auto stop."]
    #[must_use]
    #[inline(always)]
    pub const fn SEND_AUTO_STOP(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Send auto stop."]
    #[inline(always)]
    pub const fn set_SEND_AUTO_STOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Wait prvdata complete."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_PRVDATA_COMPLETE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Wait prvdata complete."]
    #[inline(always)]
    pub const fn set_WAIT_PRVDATA_COMPLETE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Stop abort command."]
    #[must_use]
    #[inline(always)]
    pub const fn STOP_ABORT_CMD(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Stop abort command."]
    #[inline(always)]
    pub const fn set_STOP_ABORT_CMD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Send initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn SEND_INITIALIZATION(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Send initialization."]
    #[inline(always)]
    pub const fn set_SEND_INITIALIZATION(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Specifies the card number of SDCARD for which the current Command is being executed."]
    #[must_use]
    #[inline(always)]
    pub const fn CARD_NUMBER(&self) -> super::vals::CARD_NUMBER {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::CARD_NUMBER::from_bits(val as u8)
    }
    #[doc = "Specifies the card number of SDCARD for which the current Command is being executed."]
    #[inline(always)]
    pub const fn set_CARD_NUMBER(&mut self, val: super::vals::CARD_NUMBER) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Update clock registers only."]
    #[must_use]
    #[inline(always)]
    pub const fn UPDATE_CLOCK_REGISTERS_ONLY(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Update clock registers only."]
    #[inline(always)]
    pub const fn set_UPDATE_CLOCK_REGISTERS_ONLY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Read ceata device."]
    #[must_use]
    #[inline(always)]
    pub const fn READ_CEATA_DEVICE(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Read ceata device."]
    #[inline(always)]
    pub const fn set_READ_CEATA_DEVICE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "CCS expected."]
    #[must_use]
    #[inline(always)]
    pub const fn CCS_EXPECTED(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "CCS expected."]
    #[inline(always)]
    pub const fn set_CCS_EXPECTED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable Boot - this bit should be set only for mandatory boot mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE_BOOT(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Boot - this bit should be set only for mandatory boot mode."]
    #[inline(always)]
    pub const fn set_ENABLE_BOOT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Expect Boot Acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn EXPECT_BOOT_ACK(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Expect Boot Acknowledge."]
    #[inline(always)]
    pub const fn set_EXPECT_BOOT_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Disable Boot."]
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE_BOOT(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Boot."]
    #[inline(always)]
    pub const fn set_DISABLE_BOOT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Boot Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_MODE(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Boot Mode."]
    #[inline(always)]
    pub const fn set_BOOT_MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Voltage switch bit."]
    #[must_use]
    #[inline(always)]
    pub const fn VOLT_SWITCH(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage switch bit."]
    #[inline(always)]
    pub const fn set_VOLT_SWITCH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Use Hold Register."]
    #[must_use]
    #[inline(always)]
    pub const fn USE_HOLD_REG(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Use Hold Register."]
    #[inline(always)]
    pub const fn set_USE_HOLD_REG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Start command."]
    #[must_use]
    #[inline(always)]
    pub const fn START_CMD(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Start command."]
    #[inline(always)]
    pub const fn set_START_CMD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CMD {
    #[inline(always)]
    fn default() -> CMD {
        CMD(0)
    }
}
impl core::fmt::Debug for CMD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("CMD_INDEX", &self.CMD_INDEX())
            .field("RESPONSE_EXPECT", &self.RESPONSE_EXPECT())
            .field("RESPONSE_LENGTH", &self.RESPONSE_LENGTH())
            .field("CHECK_RESPONSE_CRC", &self.CHECK_RESPONSE_CRC())
            .field("DATA_EXPECTED", &self.DATA_EXPECTED())
            .field("READ_WRITE", &self.READ_WRITE())
            .field("TRANSFER_MODE", &self.TRANSFER_MODE())
            .field("SEND_AUTO_STOP", &self.SEND_AUTO_STOP())
            .field("WAIT_PRVDATA_COMPLETE", &self.WAIT_PRVDATA_COMPLETE())
            .field("STOP_ABORT_CMD", &self.STOP_ABORT_CMD())
            .field("SEND_INITIALIZATION", &self.SEND_INITIALIZATION())
            .field("CARD_NUMBER", &self.CARD_NUMBER())
            .field(
                "UPDATE_CLOCK_REGISTERS_ONLY",
                &self.UPDATE_CLOCK_REGISTERS_ONLY(),
            )
            .field("READ_CEATA_DEVICE", &self.READ_CEATA_DEVICE())
            .field("CCS_EXPECTED", &self.CCS_EXPECTED())
            .field("ENABLE_BOOT", &self.ENABLE_BOOT())
            .field("EXPECT_BOOT_ACK", &self.EXPECT_BOOT_ACK())
            .field("DISABLE_BOOT", &self.DISABLE_BOOT())
            .field("BOOT_MODE", &self.BOOT_MODE())
            .field("VOLT_SWITCH", &self.VOLT_SWITCH())
            .field("USE_HOLD_REG", &self.USE_HOLD_REG())
            .field("START_CMD", &self.START_CMD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMD {{ CMD_INDEX: {=u8:?}, RESPONSE_EXPECT: {=bool:?}, RESPONSE_LENGTH: {=bool:?}, CHECK_RESPONSE_CRC: {=bool:?}, DATA_EXPECTED: {=bool:?}, READ_WRITE: {=bool:?}, TRANSFER_MODE: {=bool:?}, SEND_AUTO_STOP: {=bool:?}, WAIT_PRVDATA_COMPLETE: {=bool:?}, STOP_ABORT_CMD: {=bool:?}, SEND_INITIALIZATION: {=bool:?}, CARD_NUMBER: {:?}, UPDATE_CLOCK_REGISTERS_ONLY: {=bool:?}, READ_CEATA_DEVICE: {=bool:?}, CCS_EXPECTED: {=bool:?}, ENABLE_BOOT: {=bool:?}, EXPECT_BOOT_ACK: {=bool:?}, DISABLE_BOOT: {=bool:?}, BOOT_MODE: {=bool:?}, VOLT_SWITCH: {=bool:?}, USE_HOLD_REG: {=bool:?}, START_CMD: {=bool:?} }}",
            self.CMD_INDEX(),
            self.RESPONSE_EXPECT(),
            self.RESPONSE_LENGTH(),
            self.CHECK_RESPONSE_CRC(),
            self.DATA_EXPECTED(),
            self.READ_WRITE(),
            self.TRANSFER_MODE(),
            self.SEND_AUTO_STOP(),
            self.WAIT_PRVDATA_COMPLETE(),
            self.STOP_ABORT_CMD(),
            self.SEND_INITIALIZATION(),
            self.CARD_NUMBER(),
            self.UPDATE_CLOCK_REGISTERS_ONLY(),
            self.READ_CEATA_DEVICE(),
            self.CCS_EXPECTED(),
            self.ENABLE_BOOT(),
            self.EXPECT_BOOT_ACK(),
            self.DISABLE_BOOT(),
            self.BOOT_MODE(),
            self.VOLT_SWITCH(),
            self.USE_HOLD_REG(),
            self.START_CMD()
        )
    }
}
#[doc = "Command Argument register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDARG(pub u32);
impl CMDARG {
    #[doc = "Value indicates command argument to be passed to card."]
    #[must_use]
    #[inline(always)]
    pub const fn CMD_ARG(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Value indicates command argument to be passed to card."]
    #[inline(always)]
    pub const fn set_CMD_ARG(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CMDARG {
    #[inline(always)]
    fn default() -> CMDARG {
        CMDARG(0)
    }
}
impl core::fmt::Debug for CMDARG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDARG")
            .field("CMD_ARG", &self.CMD_ARG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDARG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CMDARG {{ CMD_ARG: {=u32:?} }}", self.CMD_ARG())
    }
}
#[doc = "Control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "Controller reset."]
    #[must_use]
    #[inline(always)]
    pub const fn CONTROLLER_RESET(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Controller reset."]
    #[inline(always)]
    pub const fn set_CONTROLLER_RESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Fifo reset."]
    #[must_use]
    #[inline(always)]
    pub const fn FIFO_RESET(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Fifo reset."]
    #[inline(always)]
    pub const fn set_FIFO_RESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA reset."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA_RESET(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA reset."]
    #[inline(always)]
    pub const fn set_DMA_RESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Global interrupt enable/disable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn INT_ENABLE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Global interrupt enable/disable bit."]
    #[inline(always)]
    pub const fn set_INT_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Read/wait."]
    #[must_use]
    #[inline(always)]
    pub const fn READ_WAIT(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Read/wait."]
    #[inline(always)]
    pub const fn set_READ_WAIT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Send irq response."]
    #[must_use]
    #[inline(always)]
    pub const fn SEND_IRQ_RESPONSE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Send irq response."]
    #[inline(always)]
    pub const fn set_SEND_IRQ_RESPONSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Abort read data."]
    #[must_use]
    #[inline(always)]
    pub const fn ABORT_READ_DATA(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Abort read data."]
    #[inline(always)]
    pub const fn set_ABORT_READ_DATA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Send ccsd."]
    #[must_use]
    #[inline(always)]
    pub const fn SEND_CCSD(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Send ccsd."]
    #[inline(always)]
    pub const fn set_SEND_CCSD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Send auto stop ccsd."]
    #[must_use]
    #[inline(always)]
    pub const fn SEND_AUTO_STOP_CCSD(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Send auto stop ccsd."]
    #[inline(always)]
    pub const fn set_SEND_AUTO_STOP_CCSD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "CEATA device interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn CEATA_DEVICE_INTERRUPT_STATUS(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CEATA device interrupt status."]
    #[inline(always)]
    pub const fn set_CEATA_DEVICE_INTERRUPT_STATUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Controls the state of the SD_VOLT0 pin."]
    #[must_use]
    #[inline(always)]
    pub const fn CARD_VOLTAGE_A0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Controls the state of the SD_VOLT0 pin."]
    #[inline(always)]
    pub const fn set_CARD_VOLTAGE_A0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Controls the state of the SD_VOLT1 pin."]
    #[must_use]
    #[inline(always)]
    pub const fn CARD_VOLTAGE_A1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Controls the state of the SD_VOLT1 pin."]
    #[inline(always)]
    pub const fn set_CARD_VOLTAGE_A1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Controls the state of the SD_VOLT2 pin."]
    #[must_use]
    #[inline(always)]
    pub const fn CARD_VOLTAGE_A2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Controls the state of the SD_VOLT2 pin."]
    #[inline(always)]
    pub const fn set_CARD_VOLTAGE_A2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SD/MMC DMA use."]
    #[must_use]
    #[inline(always)]
    pub const fn USE_INTERNAL_DMAC(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SD/MMC DMA use."]
    #[inline(always)]
    pub const fn set_USE_INTERNAL_DMAC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for CTRL {
    #[inline(always)]
    fn default() -> CTRL {
        CTRL(0)
    }
}
impl core::fmt::Debug for CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("CONTROLLER_RESET", &self.CONTROLLER_RESET())
            .field("FIFO_RESET", &self.FIFO_RESET())
            .field("DMA_RESET", &self.DMA_RESET())
            .field("INT_ENABLE", &self.INT_ENABLE())
            .field("READ_WAIT", &self.READ_WAIT())
            .field("SEND_IRQ_RESPONSE", &self.SEND_IRQ_RESPONSE())
            .field("ABORT_READ_DATA", &self.ABORT_READ_DATA())
            .field("SEND_CCSD", &self.SEND_CCSD())
            .field("SEND_AUTO_STOP_CCSD", &self.SEND_AUTO_STOP_CCSD())
            .field(
                "CEATA_DEVICE_INTERRUPT_STATUS",
                &self.CEATA_DEVICE_INTERRUPT_STATUS(),
            )
            .field("CARD_VOLTAGE_A0", &self.CARD_VOLTAGE_A0())
            .field("CARD_VOLTAGE_A1", &self.CARD_VOLTAGE_A1())
            .field("CARD_VOLTAGE_A2", &self.CARD_VOLTAGE_A2())
            .field("USE_INTERNAL_DMAC", &self.USE_INTERNAL_DMAC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL {{ CONTROLLER_RESET: {=bool:?}, FIFO_RESET: {=bool:?}, DMA_RESET: {=bool:?}, INT_ENABLE: {=bool:?}, READ_WAIT: {=bool:?}, SEND_IRQ_RESPONSE: {=bool:?}, ABORT_READ_DATA: {=bool:?}, SEND_CCSD: {=bool:?}, SEND_AUTO_STOP_CCSD: {=bool:?}, CEATA_DEVICE_INTERRUPT_STATUS: {=bool:?}, CARD_VOLTAGE_A0: {=bool:?}, CARD_VOLTAGE_A1: {=bool:?}, CARD_VOLTAGE_A2: {=bool:?}, USE_INTERNAL_DMAC: {=bool:?} }}",
            self.CONTROLLER_RESET(),
            self.FIFO_RESET(),
            self.DMA_RESET(),
            self.INT_ENABLE(),
            self.READ_WAIT(),
            self.SEND_IRQ_RESPONSE(),
            self.ABORT_READ_DATA(),
            self.SEND_CCSD(),
            self.SEND_AUTO_STOP_CCSD(),
            self.CEATA_DEVICE_INTERRUPT_STATUS(),
            self.CARD_VOLTAGE_A0(),
            self.CARD_VOLTAGE_A1(),
            self.CARD_VOLTAGE_A2(),
            self.USE_INTERNAL_DMAC()
        )
    }
}
#[doc = "Card Type register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTYPE(pub u32);
impl CTYPE {
    #[doc = "Indicates if card 0 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD0_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[must_use]
    #[inline(always)]
    pub const fn CARD0_WIDTH0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if card 0 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD0_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline(always)]
    pub const fn set_CARD0_WIDTH0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates if card 1 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD1_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[must_use]
    #[inline(always)]
    pub const fn CARD1_WIDTH0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if card 1 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD1_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline(always)]
    pub const fn set_CARD1_WIDTH0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates if card 0 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[must_use]
    #[inline(always)]
    pub const fn CARD0_WIDTH1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if card 0 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline(always)]
    pub const fn set_CARD0_WIDTH1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates if card 1 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[must_use]
    #[inline(always)]
    pub const fn CARD1_WIDTH1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if card 1 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline(always)]
    pub const fn set_CARD1_WIDTH1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for CTYPE {
    #[inline(always)]
    fn default() -> CTYPE {
        CTYPE(0)
    }
}
impl core::fmt::Debug for CTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTYPE")
            .field("CARD0_WIDTH0", &self.CARD0_WIDTH0())
            .field("CARD1_WIDTH0", &self.CARD1_WIDTH0())
            .field("CARD0_WIDTH1", &self.CARD0_WIDTH1())
            .field("CARD1_WIDTH1", &self.CARD1_WIDTH1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTYPE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTYPE {{ CARD0_WIDTH0: {=bool:?}, CARD1_WIDTH0: {=bool:?}, CARD0_WIDTH1: {=bool:?}, CARD1_WIDTH1: {=bool:?} }}",
            self.CARD0_WIDTH0(),
            self.CARD1_WIDTH0(),
            self.CARD0_WIDTH1(),
            self.CARD1_WIDTH1()
        )
    }
}
#[doc = "Descriptor List Base Address register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DBADDR(pub u32);
impl DBADDR {
    #[doc = "Start of Descriptor List."]
    #[must_use]
    #[inline(always)]
    pub const fn SDL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Start of Descriptor List."]
    #[inline(always)]
    pub const fn set_SDL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DBADDR {
    #[inline(always)]
    fn default() -> DBADDR {
        DBADDR(0)
    }
}
impl core::fmt::Debug for DBADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBADDR").field("SDL", &self.SDL()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DBADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DBADDR {{ SDL: {=u32:?} }}", self.SDL())
    }
}
#[doc = "Debounce Count register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DEBNCE(pub u32);
impl DEBNCE {
    #[doc = "Number of host clocks (SD_CLK) used by debounce filter logic for card detect; typical debounce time is 5-25 ms."]
    #[must_use]
    #[inline(always)]
    pub const fn DEBOUNCE_COUNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Number of host clocks (SD_CLK) used by debounce filter logic for card detect; typical debounce time is 5-25 ms."]
    #[inline(always)]
    pub const fn set_DEBOUNCE_COUNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DEBNCE {
    #[inline(always)]
    fn default() -> DEBNCE {
        DEBNCE(0)
    }
}
impl core::fmt::Debug for DEBNCE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBNCE")
            .field("DEBOUNCE_COUNT", &self.DEBOUNCE_COUNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DEBNCE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DEBNCE {{ DEBOUNCE_COUNT: {=u32:?} }}",
            self.DEBOUNCE_COUNT()
        )
    }
}
#[doc = "Current Host Descriptor Address register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DSCADDR(pub u32);
impl DSCADDR {
    #[doc = "Host Descriptor Address Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn HDA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Host Descriptor Address Pointer."]
    #[inline(always)]
    pub const fn set_HDA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DSCADDR {
    #[inline(always)]
    fn default() -> DSCADDR {
        DSCADDR(0)
    }
}
impl core::fmt::Debug for DSCADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSCADDR").field("HDA", &self.HDA()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DSCADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DSCADDR {{ HDA: {=u32:?} }}", self.HDA())
    }
}
#[doc = "SDIF FIFO."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIFO(pub u32);
impl FIFO {
    #[doc = "SDIF FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SDIF FIFO."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FIFO {
    #[inline(always)]
    fn default() -> FIFO {
        FIFO(0)
    }
}
impl core::fmt::Debug for FIFO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO").field("DATA", &self.DATA()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FIFO {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "FIFO Threshold Watermark register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIFOTH(pub u32);
impl FIFOTH {
    #[doc = "FIFO threshold watermark level when transmitting data to card."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_WMARK(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "FIFO threshold watermark level when transmitting data to card."]
    #[inline(always)]
    pub const fn set_TX_WMARK(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "FIFO threshold watermark level when receiving data to card."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_WMARK(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "FIFO threshold watermark level when receiving data to card."]
    #[inline(always)]
    pub const fn set_RX_WMARK(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA_MTS(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
    #[inline(always)]
    pub const fn set_DMA_MTS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for FIFOTH {
    #[inline(always)]
    fn default() -> FIFOTH {
        FIFOTH(0)
    }
}
impl core::fmt::Debug for FIFOTH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOTH")
            .field("TX_WMARK", &self.TX_WMARK())
            .field("RX_WMARK", &self.RX_WMARK())
            .field("DMA_MTS", &self.DMA_MTS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFOTH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FIFOTH {{ TX_WMARK: {=u16:?}, RX_WMARK: {=u16:?}, DMA_MTS: {=u8:?} }}",
            self.TX_WMARK(),
            self.RX_WMARK(),
            self.DMA_MTS()
        )
    }
}
#[doc = "Internal DMAC Interrupt Enable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IDINTEN(pub u32);
impl IDINTEN {
    #[doc = "Transmit Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TI(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt Enable."]
    #[inline(always)]
    pub const fn set_TI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RI(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Interrupt Enable."]
    #[inline(always)]
    pub const fn set_RI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Fatal Bus Error Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn FBE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fatal Bus Error Enable."]
    #[inline(always)]
    pub const fn set_FBE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Descriptor Unavailable Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn DU(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub const fn set_DU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Card Error summary Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CES(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Card Error summary Interrupt Enable."]
    #[inline(always)]
    pub const fn set_CES(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Normal Interrupt Summary Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn NIS(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Normal Interrupt Summary Enable."]
    #[inline(always)]
    pub const fn set_NIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Abnormal Interrupt Summary Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn AIS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Abnormal Interrupt Summary Enable."]
    #[inline(always)]
    pub const fn set_AIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for IDINTEN {
    #[inline(always)]
    fn default() -> IDINTEN {
        IDINTEN(0)
    }
}
impl core::fmt::Debug for IDINTEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDINTEN")
            .field("TI", &self.TI())
            .field("RI", &self.RI())
            .field("FBE", &self.FBE())
            .field("DU", &self.DU())
            .field("CES", &self.CES())
            .field("NIS", &self.NIS())
            .field("AIS", &self.AIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IDINTEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IDINTEN {{ TI: {=bool:?}, RI: {=bool:?}, FBE: {=bool:?}, DU: {=bool:?}, CES: {=bool:?}, NIS: {=bool:?}, AIS: {=bool:?} }}",
            self.TI(),
            self.RI(),
            self.FBE(),
            self.DU(),
            self.CES(),
            self.NIS(),
            self.AIS()
        )
    }
}
#[doc = "Internal DMAC Status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IDSTS(pub u32);
impl IDSTS {
    #[doc = "Transmit Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn TI(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt."]
    #[inline(always)]
    pub const fn set_TI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn RI(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Interrupt."]
    #[inline(always)]
    pub const fn set_RI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Fatal Bus Error Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn FBE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fatal Bus Error Interrupt."]
    #[inline(always)]
    pub const fn set_FBE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Descriptor Unavailable Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn DU(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub const fn set_DU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Card Error Summary."]
    #[must_use]
    #[inline(always)]
    pub const fn CES(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Card Error Summary."]
    #[inline(always)]
    pub const fn set_CES(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Normal Interrupt Summary."]
    #[must_use]
    #[inline(always)]
    pub const fn NIS(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Normal Interrupt Summary."]
    #[inline(always)]
    pub const fn set_NIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Abnormal Interrupt Summary."]
    #[must_use]
    #[inline(always)]
    pub const fn AIS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Abnormal Interrupt Summary."]
    #[inline(always)]
    pub const fn set_AIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Error Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn EB(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Error Bits."]
    #[inline(always)]
    pub const fn set_EB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "DMAC state machine present state."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x0f;
        val as u8
    }
    #[doc = "DMAC state machine present state."]
    #[inline(always)]
    pub const fn set_FSM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
    }
}
impl Default for IDSTS {
    #[inline(always)]
    fn default() -> IDSTS {
        IDSTS(0)
    }
}
impl core::fmt::Debug for IDSTS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDSTS")
            .field("TI", &self.TI())
            .field("RI", &self.RI())
            .field("FBE", &self.FBE())
            .field("DU", &self.DU())
            .field("CES", &self.CES())
            .field("NIS", &self.NIS())
            .field("AIS", &self.AIS())
            .field("EB", &self.EB())
            .field("FSM", &self.FSM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IDSTS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IDSTS {{ TI: {=bool:?}, RI: {=bool:?}, FBE: {=bool:?}, DU: {=bool:?}, CES: {=bool:?}, NIS: {=bool:?}, AIS: {=bool:?}, EB: {=u8:?}, FSM: {=u8:?} }}",
            self.TI(),
            self.RI(),
            self.FBE(),
            self.DU(),
            self.CES(),
            self.NIS(),
            self.AIS(),
            self.EB(),
            self.FSM()
        )
    }
}
#[doc = "Interrupt Mask register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTMASK(pub u32);
impl INTMASK {
    #[doc = "Card detect."]
    #[must_use]
    #[inline(always)]
    pub const fn CDET(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Card detect."]
    #[inline(always)]
    pub const fn set_CDET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Response error."]
    #[must_use]
    #[inline(always)]
    pub const fn RE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Response error."]
    #[inline(always)]
    pub const fn set_RE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Command done."]
    #[must_use]
    #[inline(always)]
    pub const fn CDONE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Command done."]
    #[inline(always)]
    pub const fn set_CDONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Data transfer over."]
    #[must_use]
    #[inline(always)]
    pub const fn DTO(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Data transfer over."]
    #[inline(always)]
    pub const fn set_DTO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit FIFO data request."]
    #[must_use]
    #[inline(always)]
    pub const fn TXDR(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO data request."]
    #[inline(always)]
    pub const fn set_TXDR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receive FIFO data request."]
    #[must_use]
    #[inline(always)]
    pub const fn RXDR(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO data request."]
    #[inline(always)]
    pub const fn set_RXDR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Response CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn RCRC(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Response CRC error."]
    #[inline(always)]
    pub const fn set_RCRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Data CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn DCRC(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Data CRC error."]
    #[inline(always)]
    pub const fn set_DCRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Response time-out."]
    #[must_use]
    #[inline(always)]
    pub const fn RTO(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Response time-out."]
    #[inline(always)]
    pub const fn set_RTO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Data read time-out."]
    #[must_use]
    #[inline(always)]
    pub const fn DRTO(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Data read time-out."]
    #[inline(always)]
    pub const fn set_DRTO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Data starvation-by-host time-out (HTO)."]
    #[must_use]
    #[inline(always)]
    pub const fn HTO(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Data starvation-by-host time-out (HTO)."]
    #[inline(always)]
    pub const fn set_HTO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "FIFO underrun/overrun error."]
    #[must_use]
    #[inline(always)]
    pub const fn FRUN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO underrun/overrun error."]
    #[inline(always)]
    pub const fn set_FRUN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Hardware locked write error."]
    #[must_use]
    #[inline(always)]
    pub const fn HLE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware locked write error."]
    #[inline(always)]
    pub const fn set_HLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Start-bit error."]
    #[must_use]
    #[inline(always)]
    pub const fn SBE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Start-bit error."]
    #[inline(always)]
    pub const fn set_SBE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Auto command done."]
    #[must_use]
    #[inline(always)]
    pub const fn ACD(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Auto command done."]
    #[inline(always)]
    pub const fn set_ACD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "End-bit error (read)/Write no CRC."]
    #[must_use]
    #[inline(always)]
    pub const fn EBE(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "End-bit error (read)/Write no CRC."]
    #[inline(always)]
    pub const fn set_EBE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Mask SDIO interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn SDIO_INT_MASK(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Mask SDIO interrupt."]
    #[inline(always)]
    pub const fn set_SDIO_INT_MASK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for INTMASK {
    #[inline(always)]
    fn default() -> INTMASK {
        INTMASK(0)
    }
}
impl core::fmt::Debug for INTMASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTMASK")
            .field("CDET", &self.CDET())
            .field("RE", &self.RE())
            .field("CDONE", &self.CDONE())
            .field("DTO", &self.DTO())
            .field("TXDR", &self.TXDR())
            .field("RXDR", &self.RXDR())
            .field("RCRC", &self.RCRC())
            .field("DCRC", &self.DCRC())
            .field("RTO", &self.RTO())
            .field("DRTO", &self.DRTO())
            .field("HTO", &self.HTO())
            .field("FRUN", &self.FRUN())
            .field("HLE", &self.HLE())
            .field("SBE", &self.SBE())
            .field("ACD", &self.ACD())
            .field("EBE", &self.EBE())
            .field("SDIO_INT_MASK", &self.SDIO_INT_MASK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTMASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTMASK {{ CDET: {=bool:?}, RE: {=bool:?}, CDONE: {=bool:?}, DTO: {=bool:?}, TXDR: {=bool:?}, RXDR: {=bool:?}, RCRC: {=bool:?}, DCRC: {=bool:?}, RTO: {=bool:?}, DRTO: {=bool:?}, HTO: {=bool:?}, FRUN: {=bool:?}, HLE: {=bool:?}, SBE: {=bool:?}, ACD: {=bool:?}, EBE: {=bool:?}, SDIO_INT_MASK: {=bool:?} }}",
            self.CDET(),
            self.RE(),
            self.CDONE(),
            self.DTO(),
            self.TXDR(),
            self.RXDR(),
            self.RCRC(),
            self.DCRC(),
            self.RTO(),
            self.DRTO(),
            self.HTO(),
            self.FRUN(),
            self.HLE(),
            self.SBE(),
            self.ACD(),
            self.EBE(),
            self.SDIO_INT_MASK()
        )
    }
}
#[doc = "Masked Interrupt Status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MINTSTS(pub u32);
impl MINTSTS {
    #[doc = "Card detect."]
    #[must_use]
    #[inline(always)]
    pub const fn CDET(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Card detect."]
    #[inline(always)]
    pub const fn set_CDET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Response error."]
    #[must_use]
    #[inline(always)]
    pub const fn RE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Response error."]
    #[inline(always)]
    pub const fn set_RE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Command done."]
    #[must_use]
    #[inline(always)]
    pub const fn CDONE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Command done."]
    #[inline(always)]
    pub const fn set_CDONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Data transfer over."]
    #[must_use]
    #[inline(always)]
    pub const fn DTO(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Data transfer over."]
    #[inline(always)]
    pub const fn set_DTO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit FIFO data request."]
    #[must_use]
    #[inline(always)]
    pub const fn TXDR(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO data request."]
    #[inline(always)]
    pub const fn set_TXDR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receive FIFO data request."]
    #[must_use]
    #[inline(always)]
    pub const fn RXDR(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO data request."]
    #[inline(always)]
    pub const fn set_RXDR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Response CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn RCRC(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Response CRC error."]
    #[inline(always)]
    pub const fn set_RCRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Data CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn DCRC(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Data CRC error."]
    #[inline(always)]
    pub const fn set_DCRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Response time-out."]
    #[must_use]
    #[inline(always)]
    pub const fn RTO(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Response time-out."]
    #[inline(always)]
    pub const fn set_RTO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Data read time-out."]
    #[must_use]
    #[inline(always)]
    pub const fn DRTO(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Data read time-out."]
    #[inline(always)]
    pub const fn set_DRTO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Data starvation-by-host time-out (HTO)."]
    #[must_use]
    #[inline(always)]
    pub const fn HTO(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Data starvation-by-host time-out (HTO)."]
    #[inline(always)]
    pub const fn set_HTO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "FIFO underrun/overrun error."]
    #[must_use]
    #[inline(always)]
    pub const fn FRUN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO underrun/overrun error."]
    #[inline(always)]
    pub const fn set_FRUN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Hardware locked write error."]
    #[must_use]
    #[inline(always)]
    pub const fn HLE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware locked write error."]
    #[inline(always)]
    pub const fn set_HLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Start-bit error."]
    #[must_use]
    #[inline(always)]
    pub const fn SBE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Start-bit error."]
    #[inline(always)]
    pub const fn set_SBE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Auto command done."]
    #[must_use]
    #[inline(always)]
    pub const fn ACD(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Auto command done."]
    #[inline(always)]
    pub const fn set_ACD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "End-bit error (read)/write no CRC."]
    #[must_use]
    #[inline(always)]
    pub const fn EBE(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "End-bit error (read)/write no CRC."]
    #[inline(always)]
    pub const fn set_EBE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt from SDIO card."]
    #[must_use]
    #[inline(always)]
    pub const fn SDIO_INTERRUPT(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt from SDIO card."]
    #[inline(always)]
    pub const fn set_SDIO_INTERRUPT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for MINTSTS {
    #[inline(always)]
    fn default() -> MINTSTS {
        MINTSTS(0)
    }
}
impl core::fmt::Debug for MINTSTS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MINTSTS")
            .field("CDET", &self.CDET())
            .field("RE", &self.RE())
            .field("CDONE", &self.CDONE())
            .field("DTO", &self.DTO())
            .field("TXDR", &self.TXDR())
            .field("RXDR", &self.RXDR())
            .field("RCRC", &self.RCRC())
            .field("DCRC", &self.DCRC())
            .field("RTO", &self.RTO())
            .field("DRTO", &self.DRTO())
            .field("HTO", &self.HTO())
            .field("FRUN", &self.FRUN())
            .field("HLE", &self.HLE())
            .field("SBE", &self.SBE())
            .field("ACD", &self.ACD())
            .field("EBE", &self.EBE())
            .field("SDIO_INTERRUPT", &self.SDIO_INTERRUPT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MINTSTS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MINTSTS {{ CDET: {=bool:?}, RE: {=bool:?}, CDONE: {=bool:?}, DTO: {=bool:?}, TXDR: {=bool:?}, RXDR: {=bool:?}, RCRC: {=bool:?}, DCRC: {=bool:?}, RTO: {=bool:?}, DRTO: {=bool:?}, HTO: {=bool:?}, FRUN: {=bool:?}, HLE: {=bool:?}, SBE: {=bool:?}, ACD: {=bool:?}, EBE: {=bool:?}, SDIO_INTERRUPT: {=bool:?} }}",
            self.CDET(),
            self.RE(),
            self.CDONE(),
            self.DTO(),
            self.TXDR(),
            self.RXDR(),
            self.RCRC(),
            self.DCRC(),
            self.RTO(),
            self.DRTO(),
            self.HTO(),
            self.FRUN(),
            self.HLE(),
            self.SBE(),
            self.ACD(),
            self.EBE(),
            self.SDIO_INTERRUPT()
        )
    }
}
#[doc = "Poll Demand register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLDMND(pub u32);
impl PLDMND {
    #[doc = "Poll Demand."]
    #[must_use]
    #[inline(always)]
    pub const fn PD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Poll Demand."]
    #[inline(always)]
    pub const fn set_PD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PLDMND {
    #[inline(always)]
    fn default() -> PLDMND {
        PLDMND(0)
    }
}
impl core::fmt::Debug for PLDMND {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLDMND").field("PD", &self.PD()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLDMND {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PLDMND {{ PD: {=u32:?} }}", self.PD())
    }
}
#[doc = "Power Enable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWREN(pub u32);
impl PWREN {
    #[doc = "Power on/off switch for card 0; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 0."]
    #[must_use]
    #[inline(always)]
    pub const fn POWER_ENABLE0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power on/off switch for card 0; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 0."]
    #[inline(always)]
    pub const fn set_POWER_ENABLE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Power on/off switch for card 1; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 1."]
    #[must_use]
    #[inline(always)]
    pub const fn POWER_ENABLE1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Power on/off switch for card 1; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 1."]
    #[inline(always)]
    pub const fn set_POWER_ENABLE1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for PWREN {
    #[inline(always)]
    fn default() -> PWREN {
        PWREN(0)
    }
}
impl core::fmt::Debug for PWREN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWREN")
            .field("POWER_ENABLE0", &self.POWER_ENABLE0())
            .field("POWER_ENABLE1", &self.POWER_ENABLE1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWREN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWREN {{ POWER_ENABLE0: {=bool:?}, POWER_ENABLE1: {=bool:?} }}",
            self.POWER_ENABLE0(),
            self.POWER_ENABLE1()
        )
    }
}
#[doc = "Response register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RESP(pub u32);
impl RESP {
    #[doc = "Bits of response."]
    #[must_use]
    #[inline(always)]
    pub const fn RESPONSE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Bits of response."]
    #[inline(always)]
    pub const fn set_RESPONSE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RESP {
    #[inline(always)]
    fn default() -> RESP {
        RESP(0)
    }
}
impl core::fmt::Debug for RESP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESP")
            .field("RESPONSE", &self.RESPONSE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RESP {{ RESPONSE: {=u32:?} }}", self.RESPONSE())
    }
}
#[doc = "Raw Interrupt Status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RINTSTS(pub u32);
impl RINTSTS {
    #[doc = "Card detect."]
    #[must_use]
    #[inline(always)]
    pub const fn CDET(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Card detect."]
    #[inline(always)]
    pub const fn set_CDET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Response error."]
    #[must_use]
    #[inline(always)]
    pub const fn RE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Response error."]
    #[inline(always)]
    pub const fn set_RE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Command done."]
    #[must_use]
    #[inline(always)]
    pub const fn CDONE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Command done."]
    #[inline(always)]
    pub const fn set_CDONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Data transfer over."]
    #[must_use]
    #[inline(always)]
    pub const fn DTO(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Data transfer over."]
    #[inline(always)]
    pub const fn set_DTO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit FIFO data request."]
    #[must_use]
    #[inline(always)]
    pub const fn TXDR(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO data request."]
    #[inline(always)]
    pub const fn set_TXDR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receive FIFO data request."]
    #[must_use]
    #[inline(always)]
    pub const fn RXDR(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO data request."]
    #[inline(always)]
    pub const fn set_RXDR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Response CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn RCRC(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Response CRC error."]
    #[inline(always)]
    pub const fn set_RCRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Data CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn DCRC(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Data CRC error."]
    #[inline(always)]
    pub const fn set_DCRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Response time-out (RTO)/Boot Ack Received (BAR)."]
    #[must_use]
    #[inline(always)]
    pub const fn RTO_BAR(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Response time-out (RTO)/Boot Ack Received (BAR)."]
    #[inline(always)]
    pub const fn set_RTO_BAR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Data read time-out (DRTO)/Boot Data Start (BDS)."]
    #[must_use]
    #[inline(always)]
    pub const fn DRTO_BDS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Data read time-out (DRTO)/Boot Data Start (BDS)."]
    #[inline(always)]
    pub const fn set_DRTO_BDS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Data starvation-by-host time-out (HTO)."]
    #[must_use]
    #[inline(always)]
    pub const fn HTO(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Data starvation-by-host time-out (HTO)."]
    #[inline(always)]
    pub const fn set_HTO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "FIFO underrun/overrun error."]
    #[must_use]
    #[inline(always)]
    pub const fn FRUN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO underrun/overrun error."]
    #[inline(always)]
    pub const fn set_FRUN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Hardware locked write error."]
    #[must_use]
    #[inline(always)]
    pub const fn HLE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware locked write error."]
    #[inline(always)]
    pub const fn set_HLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Start-bit error."]
    #[must_use]
    #[inline(always)]
    pub const fn SBE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Start-bit error."]
    #[inline(always)]
    pub const fn set_SBE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Auto command done."]
    #[must_use]
    #[inline(always)]
    pub const fn ACD(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Auto command done."]
    #[inline(always)]
    pub const fn set_ACD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "End-bit error (read)/write no CRC."]
    #[must_use]
    #[inline(always)]
    pub const fn EBE(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "End-bit error (read)/write no CRC."]
    #[inline(always)]
    pub const fn set_EBE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt from SDIO card."]
    #[must_use]
    #[inline(always)]
    pub const fn SDIO_INTERRUPT(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt from SDIO card."]
    #[inline(always)]
    pub const fn set_SDIO_INTERRUPT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for RINTSTS {
    #[inline(always)]
    fn default() -> RINTSTS {
        RINTSTS(0)
    }
}
impl core::fmt::Debug for RINTSTS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RINTSTS")
            .field("CDET", &self.CDET())
            .field("RE", &self.RE())
            .field("CDONE", &self.CDONE())
            .field("DTO", &self.DTO())
            .field("TXDR", &self.TXDR())
            .field("RXDR", &self.RXDR())
            .field("RCRC", &self.RCRC())
            .field("DCRC", &self.DCRC())
            .field("RTO_BAR", &self.RTO_BAR())
            .field("DRTO_BDS", &self.DRTO_BDS())
            .field("HTO", &self.HTO())
            .field("FRUN", &self.FRUN())
            .field("HLE", &self.HLE())
            .field("SBE", &self.SBE())
            .field("ACD", &self.ACD())
            .field("EBE", &self.EBE())
            .field("SDIO_INTERRUPT", &self.SDIO_INTERRUPT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RINTSTS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RINTSTS {{ CDET: {=bool:?}, RE: {=bool:?}, CDONE: {=bool:?}, DTO: {=bool:?}, TXDR: {=bool:?}, RXDR: {=bool:?}, RCRC: {=bool:?}, DCRC: {=bool:?}, RTO_BAR: {=bool:?}, DRTO_BDS: {=bool:?}, HTO: {=bool:?}, FRUN: {=bool:?}, HLE: {=bool:?}, SBE: {=bool:?}, ACD: {=bool:?}, EBE: {=bool:?}, SDIO_INTERRUPT: {=bool:?} }}",
            self.CDET(),
            self.RE(),
            self.CDONE(),
            self.DTO(),
            self.TXDR(),
            self.RXDR(),
            self.RCRC(),
            self.DCRC(),
            self.RTO_BAR(),
            self.DRTO_BDS(),
            self.HTO(),
            self.FRUN(),
            self.HLE(),
            self.SBE(),
            self.ACD(),
            self.EBE(),
            self.SDIO_INTERRUPT()
        )
    }
}
#[doc = "Hardware Reset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RST_N(pub u32);
impl RST_N {
    #[doc = "Hardware reset."]
    #[must_use]
    #[inline(always)]
    pub const fn CARD_RESET(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware reset."]
    #[inline(always)]
    pub const fn set_CARD_RESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RST_N {
    #[inline(always)]
    fn default() -> RST_N {
        RST_N(0)
    }
}
impl core::fmt::Debug for RST_N {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RST_N")
            .field("CARD_RESET", &self.CARD_RESET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RST_N {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RST_N {{ CARD_RESET: {=bool:?} }}", self.CARD_RESET())
    }
}
#[doc = "Status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STATUS(pub u32);
impl STATUS {
    #[doc = "FIFO reached Receive watermark level; not qualified with data transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn FIFO_RX_WATERMARK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO reached Receive watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub const fn set_FIFO_RX_WATERMARK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO reached Transmit watermark level; not qualified with data transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn FIFO_TX_WATERMARK(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO reached Transmit watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub const fn set_FIFO_TX_WATERMARK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO is empty status."]
    #[must_use]
    #[inline(always)]
    pub const fn FIFO_EMPTY(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO is empty status."]
    #[inline(always)]
    pub const fn set_FIFO_EMPTY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIFO is full status."]
    #[must_use]
    #[inline(always)]
    pub const fn FIFO_FULL(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO is full status."]
    #[inline(always)]
    pub const fn set_FIFO_FULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CMDFSMSTATES(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
    #[inline(always)]
    pub const fn set_CMDFSMSTATES(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA_3_STATUS(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
    #[inline(always)]
    pub const fn set_DATA_3_STATUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Inverted version of raw selected card_data\\[0\\] 0 - card data not busy 1 - card data busy."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA_BUSY(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Inverted version of raw selected card_data\\[0\\] 0 - card data not busy 1 - card data busy."]
    #[inline(always)]
    pub const fn set_DATA_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Data transmit or receive state-machine is busy."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA_STATE_MC_BUSY(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Data transmit or receive state-machine is busy."]
    #[inline(always)]
    pub const fn set_DATA_STATE_MC_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Index of previous response, including any auto-stop sent by core."]
    #[must_use]
    #[inline(always)]
    pub const fn RESPONSE_INDEX(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x3f;
        val as u8
    }
    #[doc = "Index of previous response, including any auto-stop sent by core."]
    #[inline(always)]
    pub const fn set_RESPONSE_INDEX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 11usize)) | (((val as u32) & 0x3f) << 11usize);
    }
    #[doc = "FIFO count - Number of filled locations in FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn FIFO_COUNT(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x1fff;
        val as u16
    }
    #[doc = "FIFO count - Number of filled locations in FIFO."]
    #[inline(always)]
    pub const fn set_FIFO_COUNT(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 17usize)) | (((val as u32) & 0x1fff) << 17usize);
    }
    #[doc = "DMA acknowledge signal state."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA_ACK(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA acknowledge signal state."]
    #[inline(always)]
    pub const fn set_DMA_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA request signal state."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA_REQ(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA request signal state."]
    #[inline(always)]
    pub const fn set_DMA_REQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for STATUS {
    #[inline(always)]
    fn default() -> STATUS {
        STATUS(0)
    }
}
impl core::fmt::Debug for STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("FIFO_RX_WATERMARK", &self.FIFO_RX_WATERMARK())
            .field("FIFO_TX_WATERMARK", &self.FIFO_TX_WATERMARK())
            .field("FIFO_EMPTY", &self.FIFO_EMPTY())
            .field("FIFO_FULL", &self.FIFO_FULL())
            .field("CMDFSMSTATES", &self.CMDFSMSTATES())
            .field("DATA_3_STATUS", &self.DATA_3_STATUS())
            .field("DATA_BUSY", &self.DATA_BUSY())
            .field("DATA_STATE_MC_BUSY", &self.DATA_STATE_MC_BUSY())
            .field("RESPONSE_INDEX", &self.RESPONSE_INDEX())
            .field("FIFO_COUNT", &self.FIFO_COUNT())
            .field("DMA_ACK", &self.DMA_ACK())
            .field("DMA_REQ", &self.DMA_REQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STATUS {{ FIFO_RX_WATERMARK: {=bool:?}, FIFO_TX_WATERMARK: {=bool:?}, FIFO_EMPTY: {=bool:?}, FIFO_FULL: {=bool:?}, CMDFSMSTATES: {=u8:?}, DATA_3_STATUS: {=bool:?}, DATA_BUSY: {=bool:?}, DATA_STATE_MC_BUSY: {=bool:?}, RESPONSE_INDEX: {=u8:?}, FIFO_COUNT: {=u16:?}, DMA_ACK: {=bool:?}, DMA_REQ: {=bool:?} }}",
            self.FIFO_RX_WATERMARK(),
            self.FIFO_TX_WATERMARK(),
            self.FIFO_EMPTY(),
            self.FIFO_FULL(),
            self.CMDFSMSTATES(),
            self.DATA_3_STATUS(),
            self.DATA_BUSY(),
            self.DATA_STATE_MC_BUSY(),
            self.RESPONSE_INDEX(),
            self.FIFO_COUNT(),
            self.DMA_ACK(),
            self.DMA_REQ()
        )
    }
}
#[doc = "Transferred Host to BIU-FIFO Byte Count register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TBBCNT(pub u32);
impl TBBCNT {
    #[doc = "Number of bytes transferred between Host/DMA memory and BIU FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn TRANS_FIFO_BYTE_COUNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Number of bytes transferred between Host/DMA memory and BIU FIFO."]
    #[inline(always)]
    pub const fn set_TRANS_FIFO_BYTE_COUNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TBBCNT {
    #[inline(always)]
    fn default() -> TBBCNT {
        TBBCNT(0)
    }
}
impl core::fmt::Debug for TBBCNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBBCNT")
            .field("TRANS_FIFO_BYTE_COUNT", &self.TRANS_FIFO_BYTE_COUNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TBBCNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TBBCNT {{ TRANS_FIFO_BYTE_COUNT: {=u32:?} }}",
            self.TRANS_FIFO_BYTE_COUNT()
        )
    }
}
#[doc = "Transferred CIU Card Byte Count register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TCBCNT(pub u32);
impl TCBCNT {
    #[doc = "Number of bytes transferred by CIU unit to card."]
    #[must_use]
    #[inline(always)]
    pub const fn TRANS_CARD_BYTE_COUNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Number of bytes transferred by CIU unit to card."]
    #[inline(always)]
    pub const fn set_TRANS_CARD_BYTE_COUNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TCBCNT {
    #[inline(always)]
    fn default() -> TCBCNT {
        TCBCNT(0)
    }
}
impl core::fmt::Debug for TCBCNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCBCNT")
            .field("TRANS_CARD_BYTE_COUNT", &self.TRANS_CARD_BYTE_COUNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TCBCNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TCBCNT {{ TRANS_CARD_BYTE_COUNT: {=u32:?} }}",
            self.TRANS_CARD_BYTE_COUNT()
        )
    }
}
#[doc = "Time-out register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TMOUT(pub u32);
impl TMOUT {
    #[doc = "Response time-out value."]
    #[must_use]
    #[inline(always)]
    pub const fn RESPONSE_TIMEOUT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Response time-out value."]
    #[inline(always)]
    pub const fn set_RESPONSE_TIMEOUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Value for card Data Read time-out; same value also used for Data Starvation by Host time-out."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA_TIMEOUT(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Value for card Data Read time-out; same value also used for Data Starvation by Host time-out."]
    #[inline(always)]
    pub const fn set_DATA_TIMEOUT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for TMOUT {
    #[inline(always)]
    fn default() -> TMOUT {
        TMOUT(0)
    }
}
impl core::fmt::Debug for TMOUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMOUT")
            .field("RESPONSE_TIMEOUT", &self.RESPONSE_TIMEOUT())
            .field("DATA_TIMEOUT", &self.DATA_TIMEOUT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TMOUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TMOUT {{ RESPONSE_TIMEOUT: {=u8:?}, DATA_TIMEOUT: {=u32:?} }}",
            self.RESPONSE_TIMEOUT(),
            self.DATA_TIMEOUT()
        )
    }
}
#[doc = "Write Protect register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WRTPRT(pub u32);
impl WRTPRT {
    #[doc = "Write protect."]
    #[must_use]
    #[inline(always)]
    pub const fn WRITE_PROTECT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write protect."]
    #[inline(always)]
    pub const fn set_WRITE_PROTECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for WRTPRT {
    #[inline(always)]
    fn default() -> WRTPRT {
        WRTPRT(0)
    }
}
impl core::fmt::Debug for WRTPRT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRTPRT")
            .field("WRITE_PROTECT", &self.WRITE_PROTECT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WRTPRT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WRTPRT {{ WRITE_PROTECT: {=bool:?} }}",
            self.WRITE_PROTECT()
        )
    }
}
