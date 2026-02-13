#[doc = "Power control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Backendpwr(pub u32);
impl Backendpwr {
    #[doc = "Back-end Power control for card application."]
    #[must_use]
    #[inline(always)]
    pub const fn backendpwr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Back-end Power control for card application."]
    #[inline(always)]
    pub const fn set_backendpwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Backendpwr {
    #[inline(always)]
    fn default() -> Backendpwr {
        Backendpwr(0)
    }
}
impl core::fmt::Debug for Backendpwr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Backendpwr")
            .field("backendpwr", &self.backendpwr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Backendpwr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Backendpwr {{ backendpwr: {=bool:?} }}",
            self.backendpwr()
        )
    }
}
#[doc = "Block Size register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blksiz(pub u32);
impl Blksiz {
    #[doc = "Block size."]
    #[must_use]
    #[inline(always)]
    pub const fn block_size(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Block size."]
    #[inline(always)]
    pub const fn set_block_size(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Blksiz {
    #[inline(always)]
    fn default() -> Blksiz {
        Blksiz(0)
    }
}
impl core::fmt::Debug for Blksiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blksiz")
            .field("block_size", &self.block_size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blksiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Blksiz {{ block_size: {=u16:?} }}", self.block_size())
    }
}
#[doc = "Bus Mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bmod(pub u32);
impl Bmod {
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn swr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_swr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Fixed Burst."]
    #[must_use]
    #[inline(always)]
    pub const fn fb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Fixed Burst."]
    #[inline(always)]
    pub const fn set_fb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Descriptor Skip Length."]
    #[must_use]
    #[inline(always)]
    pub const fn dsl(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x1f;
        val as u8
    }
    #[doc = "Descriptor Skip Length."]
    #[inline(always)]
    pub const fn set_dsl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
    }
    #[doc = "SD/MMC DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn de(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SD/MMC DMA Enable."]
    #[inline(always)]
    pub const fn set_de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Programmable Burst Length."]
    #[must_use]
    #[inline(always)]
    pub const fn pbl(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Programmable Burst Length."]
    #[inline(always)]
    pub const fn set_pbl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for Bmod {
    #[inline(always)]
    fn default() -> Bmod {
        Bmod(0)
    }
}
impl core::fmt::Debug for Bmod {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bmod")
            .field("swr", &self.swr())
            .field("fb", &self.fb())
            .field("dsl", &self.dsl())
            .field("de", &self.de())
            .field("pbl", &self.pbl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bmod {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bmod {{ swr: {=bool:?}, fb: {=bool:?}, dsl: {=u8:?}, de: {=bool:?}, pbl: {=u8:?} }}",
            self.swr(),
            self.fb(),
            self.dsl(),
            self.de(),
            self.pbl()
        )
    }
}
#[doc = "Current Buffer Descriptor Address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bufaddr(pub u32);
impl Bufaddr {
    #[doc = "Host Buffer Address Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn hba(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Host Buffer Address Pointer."]
    #[inline(always)]
    pub const fn set_hba(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bufaddr {
    #[inline(always)]
    fn default() -> Bufaddr {
        Bufaddr(0)
    }
}
impl core::fmt::Debug for Bufaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bufaddr").field("hba", &self.hba()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bufaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bufaddr {{ hba: {=u32:?} }}", self.hba())
    }
}
#[doc = "Byte Count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bytcnt(pub u32);
impl Bytcnt {
    #[doc = "Number of bytes to be transferred; should be integer multiple of Block Size for block transfers."]
    #[must_use]
    #[inline(always)]
    pub const fn byte_count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Number of bytes to be transferred; should be integer multiple of Block Size for block transfers."]
    #[inline(always)]
    pub const fn set_byte_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bytcnt {
    #[inline(always)]
    fn default() -> Bytcnt {
        Bytcnt(0)
    }
}
impl core::fmt::Debug for Bytcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bytcnt")
            .field("byte_count", &self.byte_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bytcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bytcnt {{ byte_count: {=u32:?} }}", self.byte_count())
    }
}
#[doc = "Card Threshold Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cardthrctl(pub u32);
impl Cardthrctl {
    #[doc = "Card Read Threshold Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cardrdthren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Card Read Threshold Enable."]
    #[inline(always)]
    pub const fn set_cardrdthren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Busy Clear Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bsyclrinten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Busy Clear Interrupt Enable."]
    #[inline(always)]
    pub const fn set_bsyclrinten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Card Threshold size."]
    #[must_use]
    #[inline(always)]
    pub const fn cardthreshold(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Card Threshold size."]
    #[inline(always)]
    pub const fn set_cardthreshold(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Cardthrctl {
    #[inline(always)]
    fn default() -> Cardthrctl {
        Cardthrctl(0)
    }
}
impl core::fmt::Debug for Cardthrctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cardthrctl")
            .field("cardrdthren", &self.cardrdthren())
            .field("bsyclrinten", &self.bsyclrinten())
            .field("cardthreshold", &self.cardthreshold())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cardthrctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cardthrctl {{ cardrdthren: {=bool:?}, bsyclrinten: {=bool:?}, cardthreshold: {=u8:?} }}",
            self.cardrdthren(),
            self.bsyclrinten(),
            self.cardthreshold()
        )
    }
}
#[doc = "Card Detect register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdetect(pub u32);
impl Cdetect {
    #[doc = "Card 0 detect"]
    #[must_use]
    #[inline(always)]
    pub const fn card0_detect(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Card 0 detect"]
    #[inline(always)]
    pub const fn set_card0_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Card 1 detect"]
    #[must_use]
    #[inline(always)]
    pub const fn card1_detect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Card 1 detect"]
    #[inline(always)]
    pub const fn set_card1_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Cdetect {
    #[inline(always)]
    fn default() -> Cdetect {
        Cdetect(0)
    }
}
impl core::fmt::Debug for Cdetect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cdetect")
            .field("card0_detect", &self.card0_detect())
            .field("card1_detect", &self.card1_detect())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cdetect {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cdetect {{ card0_detect: {=bool:?}, card1_detect: {=bool:?} }}",
            self.card0_detect(),
            self.card1_detect()
        )
    }
}
#[doc = "Clock Divider register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkdiv(pub u32);
impl Clkdiv {
    #[doc = "Clock divider-0 value."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_divider0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider-0 value."]
    #[inline(always)]
    pub const fn set_clk_divider0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Clkdiv {
    #[inline(always)]
    fn default() -> Clkdiv {
        Clkdiv(0)
    }
}
impl core::fmt::Debug for Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkdiv")
            .field("clk_divider0", &self.clk_divider0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkdiv {{ clk_divider0: {=u8:?} }}", self.clk_divider0())
    }
}
#[doc = "Clock Enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkena(pub u32);
impl Clkena {
    #[doc = "Clock-enable control for SD card 0 clock."]
    #[must_use]
    #[inline(always)]
    pub const fn cclk0_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clock-enable control for SD card 0 clock."]
    #[inline(always)]
    pub const fn set_cclk0_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clock-enable control for SD card 1 clock."]
    #[must_use]
    #[inline(always)]
    pub const fn cclk1_enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clock-enable control for SD card 1 clock."]
    #[inline(always)]
    pub const fn set_cclk1_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Low-power control for SD card 0 clock."]
    #[must_use]
    #[inline(always)]
    pub const fn cclk0_low_power(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Low-power control for SD card 0 clock."]
    #[inline(always)]
    pub const fn set_cclk0_low_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Low-power control for SD card 1 clock."]
    #[must_use]
    #[inline(always)]
    pub const fn cclk1_low_power(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Low-power control for SD card 1 clock."]
    #[inline(always)]
    pub const fn set_cclk1_low_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Clkena {
    #[inline(always)]
    fn default() -> Clkena {
        Clkena(0)
    }
}
impl core::fmt::Debug for Clkena {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkena")
            .field("cclk0_enable", &self.cclk0_enable())
            .field("cclk1_enable", &self.cclk1_enable())
            .field("cclk0_low_power", &self.cclk0_low_power())
            .field("cclk1_low_power", &self.cclk1_low_power())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkena {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clkena {{ cclk0_enable: {=bool:?}, cclk1_enable: {=bool:?}, cclk0_low_power: {=bool:?}, cclk1_low_power: {=bool:?} }}",
            self.cclk0_enable(),
            self.cclk1_enable(),
            self.cclk0_low_power(),
            self.cclk1_low_power()
        )
    }
}
#[doc = "Command register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Command index."]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_index(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Command index."]
    #[inline(always)]
    pub const fn set_cmd_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Response expect."]
    #[must_use]
    #[inline(always)]
    pub const fn response_expect(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Response expect."]
    #[inline(always)]
    pub const fn set_response_expect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Response length."]
    #[must_use]
    #[inline(always)]
    pub const fn response_length(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Response length."]
    #[inline(always)]
    pub const fn set_response_length(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Check response CRC."]
    #[must_use]
    #[inline(always)]
    pub const fn check_response_crc(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Check response CRC."]
    #[inline(always)]
    pub const fn set_check_response_crc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Data expected."]
    #[must_use]
    #[inline(always)]
    pub const fn data_expected(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Data expected."]
    #[inline(always)]
    pub const fn set_data_expected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "read/write."]
    #[must_use]
    #[inline(always)]
    pub const fn read_write(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "read/write."]
    #[inline(always)]
    pub const fn set_read_write(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Transfer mode."]
    #[must_use]
    #[inline(always)]
    pub const fn transfer_mode(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer mode."]
    #[inline(always)]
    pub const fn set_transfer_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Send auto stop."]
    #[must_use]
    #[inline(always)]
    pub const fn send_auto_stop(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Send auto stop."]
    #[inline(always)]
    pub const fn set_send_auto_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Wait prvdata complete."]
    #[must_use]
    #[inline(always)]
    pub const fn wait_prvdata_complete(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Wait prvdata complete."]
    #[inline(always)]
    pub const fn set_wait_prvdata_complete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Stop abort command."]
    #[must_use]
    #[inline(always)]
    pub const fn stop_abort_cmd(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Stop abort command."]
    #[inline(always)]
    pub const fn set_stop_abort_cmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Send initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn send_initialization(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Send initialization."]
    #[inline(always)]
    pub const fn set_send_initialization(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Specifies the card number of SDCARD for which the current Command is being executed"]
    #[must_use]
    #[inline(always)]
    pub const fn card_number(&self) -> super::vals::CardNumber {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::CardNumber::from_bits(val as u8)
    }
    #[doc = "Specifies the card number of SDCARD for which the current Command is being executed"]
    #[inline(always)]
    pub const fn set_card_number(&mut self, val: super::vals::CardNumber) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Update clock registers only."]
    #[must_use]
    #[inline(always)]
    pub const fn update_clock_registers_only(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Update clock registers only."]
    #[inline(always)]
    pub const fn set_update_clock_registers_only(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Read ceata device."]
    #[must_use]
    #[inline(always)]
    pub const fn read_ceata_device(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Read ceata device."]
    #[inline(always)]
    pub const fn set_read_ceata_device(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "CCS expected."]
    #[must_use]
    #[inline(always)]
    pub const fn ccs_expected(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "CCS expected."]
    #[inline(always)]
    pub const fn set_ccs_expected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable Boot - this bit should be set only for mandatory boot mode."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_boot(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Boot - this bit should be set only for mandatory boot mode."]
    #[inline(always)]
    pub const fn set_enable_boot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Expect Boot Acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn expect_boot_ack(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Expect Boot Acknowledge."]
    #[inline(always)]
    pub const fn set_expect_boot_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Disable Boot."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_boot(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Boot."]
    #[inline(always)]
    pub const fn set_disable_boot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Boot Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn boot_mode(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Boot Mode."]
    #[inline(always)]
    pub const fn set_boot_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Voltage switch bit."]
    #[must_use]
    #[inline(always)]
    pub const fn volt_switch(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage switch bit."]
    #[inline(always)]
    pub const fn set_volt_switch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Use Hold Register."]
    #[must_use]
    #[inline(always)]
    pub const fn use_hold_reg(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Use Hold Register."]
    #[inline(always)]
    pub const fn set_use_hold_reg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Start command."]
    #[must_use]
    #[inline(always)]
    pub const fn start_cmd(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Start command."]
    #[inline(always)]
    pub const fn set_start_cmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cmd {
    #[inline(always)]
    fn default() -> Cmd {
        Cmd(0)
    }
}
impl core::fmt::Debug for Cmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmd")
            .field("cmd_index", &self.cmd_index())
            .field("response_expect", &self.response_expect())
            .field("response_length", &self.response_length())
            .field("check_response_crc", &self.check_response_crc())
            .field("data_expected", &self.data_expected())
            .field("read_write", &self.read_write())
            .field("transfer_mode", &self.transfer_mode())
            .field("send_auto_stop", &self.send_auto_stop())
            .field("wait_prvdata_complete", &self.wait_prvdata_complete())
            .field("stop_abort_cmd", &self.stop_abort_cmd())
            .field("send_initialization", &self.send_initialization())
            .field("card_number", &self.card_number())
            .field(
                "update_clock_registers_only",
                &self.update_clock_registers_only(),
            )
            .field("read_ceata_device", &self.read_ceata_device())
            .field("ccs_expected", &self.ccs_expected())
            .field("enable_boot", &self.enable_boot())
            .field("expect_boot_ack", &self.expect_boot_ack())
            .field("disable_boot", &self.disable_boot())
            .field("boot_mode", &self.boot_mode())
            .field("volt_switch", &self.volt_switch())
            .field("use_hold_reg", &self.use_hold_reg())
            .field("start_cmd", &self.start_cmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmd {{ cmd_index: {=u8:?}, response_expect: {=bool:?}, response_length: {=bool:?}, check_response_crc: {=bool:?}, data_expected: {=bool:?}, read_write: {=bool:?}, transfer_mode: {=bool:?}, send_auto_stop: {=bool:?}, wait_prvdata_complete: {=bool:?}, stop_abort_cmd: {=bool:?}, send_initialization: {=bool:?}, card_number: {:?}, update_clock_registers_only: {=bool:?}, read_ceata_device: {=bool:?}, ccs_expected: {=bool:?}, enable_boot: {=bool:?}, expect_boot_ack: {=bool:?}, disable_boot: {=bool:?}, boot_mode: {=bool:?}, volt_switch: {=bool:?}, use_hold_reg: {=bool:?}, start_cmd: {=bool:?} }}",
            self.cmd_index(),
            self.response_expect(),
            self.response_length(),
            self.check_response_crc(),
            self.data_expected(),
            self.read_write(),
            self.transfer_mode(),
            self.send_auto_stop(),
            self.wait_prvdata_complete(),
            self.stop_abort_cmd(),
            self.send_initialization(),
            self.card_number(),
            self.update_clock_registers_only(),
            self.read_ceata_device(),
            self.ccs_expected(),
            self.enable_boot(),
            self.expect_boot_ack(),
            self.disable_boot(),
            self.boot_mode(),
            self.volt_switch(),
            self.use_hold_reg(),
            self.start_cmd()
        )
    }
}
#[doc = "Command Argument register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdarg(pub u32);
impl Cmdarg {
    #[doc = "Value indicates command argument to be passed to card."]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_arg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Value indicates command argument to be passed to card."]
    #[inline(always)]
    pub const fn set_cmd_arg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmdarg {
    #[inline(always)]
    fn default() -> Cmdarg {
        Cmdarg(0)
    }
}
impl core::fmt::Debug for Cmdarg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdarg")
            .field("cmd_arg", &self.cmd_arg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdarg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmdarg {{ cmd_arg: {=u32:?} }}", self.cmd_arg())
    }
}
#[doc = "Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Controller reset."]
    #[must_use]
    #[inline(always)]
    pub const fn controller_reset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Controller reset."]
    #[inline(always)]
    pub const fn set_controller_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Fifo reset."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_reset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Fifo reset."]
    #[inline(always)]
    pub const fn set_fifo_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA reset."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_reset(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA reset."]
    #[inline(always)]
    pub const fn set_dma_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Global interrupt enable/disable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn int_enable(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Global interrupt enable/disable bit."]
    #[inline(always)]
    pub const fn set_int_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Read/wait."]
    #[must_use]
    #[inline(always)]
    pub const fn read_wait(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Read/wait."]
    #[inline(always)]
    pub const fn set_read_wait(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Send irq response."]
    #[must_use]
    #[inline(always)]
    pub const fn send_irq_response(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Send irq response."]
    #[inline(always)]
    pub const fn set_send_irq_response(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Abort read data."]
    #[must_use]
    #[inline(always)]
    pub const fn abort_read_data(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Abort read data."]
    #[inline(always)]
    pub const fn set_abort_read_data(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Send ccsd."]
    #[must_use]
    #[inline(always)]
    pub const fn send_ccsd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Send ccsd."]
    #[inline(always)]
    pub const fn set_send_ccsd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Send auto stop ccsd."]
    #[must_use]
    #[inline(always)]
    pub const fn send_auto_stop_ccsd(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Send auto stop ccsd."]
    #[inline(always)]
    pub const fn set_send_auto_stop_ccsd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "CEATA device interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn ceata_device_interrupt_status(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CEATA device interrupt status."]
    #[inline(always)]
    pub const fn set_ceata_device_interrupt_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Controls the state of the SD_VOLT0 pin."]
    #[must_use]
    #[inline(always)]
    pub const fn card_voltage_a0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Controls the state of the SD_VOLT0 pin."]
    #[inline(always)]
    pub const fn set_card_voltage_a0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Controls the state of the SD_VOLT1 pin."]
    #[must_use]
    #[inline(always)]
    pub const fn card_voltage_a1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Controls the state of the SD_VOLT1 pin."]
    #[inline(always)]
    pub const fn set_card_voltage_a1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Controls the state of the SD_VOLT2 pin."]
    #[must_use]
    #[inline(always)]
    pub const fn card_voltage_a2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Controls the state of the SD_VOLT2 pin."]
    #[inline(always)]
    pub const fn set_card_voltage_a2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SD/MMC DMA use."]
    #[must_use]
    #[inline(always)]
    pub const fn use_internal_dmac(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SD/MMC DMA use."]
    #[inline(always)]
    pub const fn set_use_internal_dmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
            .field("controller_reset", &self.controller_reset())
            .field("fifo_reset", &self.fifo_reset())
            .field("dma_reset", &self.dma_reset())
            .field("int_enable", &self.int_enable())
            .field("read_wait", &self.read_wait())
            .field("send_irq_response", &self.send_irq_response())
            .field("abort_read_data", &self.abort_read_data())
            .field("send_ccsd", &self.send_ccsd())
            .field("send_auto_stop_ccsd", &self.send_auto_stop_ccsd())
            .field(
                "ceata_device_interrupt_status",
                &self.ceata_device_interrupt_status(),
            )
            .field("card_voltage_a0", &self.card_voltage_a0())
            .field("card_voltage_a1", &self.card_voltage_a1())
            .field("card_voltage_a2", &self.card_voltage_a2())
            .field("use_internal_dmac", &self.use_internal_dmac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ controller_reset: {=bool:?}, fifo_reset: {=bool:?}, dma_reset: {=bool:?}, int_enable: {=bool:?}, read_wait: {=bool:?}, send_irq_response: {=bool:?}, abort_read_data: {=bool:?}, send_ccsd: {=bool:?}, send_auto_stop_ccsd: {=bool:?}, ceata_device_interrupt_status: {=bool:?}, card_voltage_a0: {=bool:?}, card_voltage_a1: {=bool:?}, card_voltage_a2: {=bool:?}, use_internal_dmac: {=bool:?} }}",
            self.controller_reset(),
            self.fifo_reset(),
            self.dma_reset(),
            self.int_enable(),
            self.read_wait(),
            self.send_irq_response(),
            self.abort_read_data(),
            self.send_ccsd(),
            self.send_auto_stop_ccsd(),
            self.ceata_device_interrupt_status(),
            self.card_voltage_a0(),
            self.card_voltage_a1(),
            self.card_voltage_a2(),
            self.use_internal_dmac()
        )
    }
}
#[doc = "Card Type register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctype(pub u32);
impl Ctype {
    #[doc = "Indicates if card 0 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD0_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[must_use]
    #[inline(always)]
    pub const fn card0_width0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if card 0 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD0_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline(always)]
    pub const fn set_card0_width0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates if card 1 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD1_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[must_use]
    #[inline(always)]
    pub const fn card1_width0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if card 1 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD1_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline(always)]
    pub const fn set_card1_width0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates if card 0 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[must_use]
    #[inline(always)]
    pub const fn card0_width1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if card 0 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline(always)]
    pub const fn set_card0_width1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates if card 1 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[must_use]
    #[inline(always)]
    pub const fn card1_width1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if card 1 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline(always)]
    pub const fn set_card1_width1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Ctype {
    #[inline(always)]
    fn default() -> Ctype {
        Ctype(0)
    }
}
impl core::fmt::Debug for Ctype {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctype")
            .field("card0_width0", &self.card0_width0())
            .field("card1_width0", &self.card1_width0())
            .field("card0_width1", &self.card0_width1())
            .field("card1_width1", &self.card1_width1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctype {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctype {{ card0_width0: {=bool:?}, card1_width0: {=bool:?}, card0_width1: {=bool:?}, card1_width1: {=bool:?} }}",
            self.card0_width0(),
            self.card1_width0(),
            self.card0_width1(),
            self.card1_width1()
        )
    }
}
#[doc = "Descriptor List Base Address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbaddr(pub u32);
impl Dbaddr {
    #[doc = "Start of Descriptor List."]
    #[must_use]
    #[inline(always)]
    pub const fn sdl(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Start of Descriptor List."]
    #[inline(always)]
    pub const fn set_sdl(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dbaddr {
    #[inline(always)]
    fn default() -> Dbaddr {
        Dbaddr(0)
    }
}
impl core::fmt::Debug for Dbaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbaddr").field("sdl", &self.sdl()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dbaddr {{ sdl: {=u32:?} }}", self.sdl())
    }
}
#[doc = "Debounce Count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debnce(pub u32);
impl Debnce {
    #[doc = "Number of host clocks (SD_CLK) used by debounce filter logic for card detect; typical debounce time is 5-25 ms."]
    #[must_use]
    #[inline(always)]
    pub const fn debounce_count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Number of host clocks (SD_CLK) used by debounce filter logic for card detect; typical debounce time is 5-25 ms."]
    #[inline(always)]
    pub const fn set_debounce_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Debnce {
    #[inline(always)]
    fn default() -> Debnce {
        Debnce(0)
    }
}
impl core::fmt::Debug for Debnce {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debnce")
            .field("debounce_count", &self.debounce_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debnce {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debnce {{ debounce_count: {=u32:?} }}",
            self.debounce_count()
        )
    }
}
#[doc = "Current Host Descriptor Address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dscaddr(pub u32);
impl Dscaddr {
    #[doc = "Host Descriptor Address Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn hda(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Host Descriptor Address Pointer."]
    #[inline(always)]
    pub const fn set_hda(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dscaddr {
    #[inline(always)]
    fn default() -> Dscaddr {
        Dscaddr(0)
    }
}
impl core::fmt::Debug for Dscaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dscaddr").field("hda", &self.hda()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dscaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dscaddr {{ hda: {=u32:?} }}", self.hda())
    }
}
#[doc = "SDIF FIFO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifo(pub u32);
impl Fifo {
    #[doc = "SDIF FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SDIF FIFO."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Fifo {
    #[inline(always)]
    fn default() -> Fifo {
        Fifo(0)
    }
}
impl core::fmt::Debug for Fifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifo").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fifo {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "FIFO Threshold Watermark register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifoth(pub u32);
impl Fifoth {
    #[doc = "FIFO threshold watermark level when transmitting data to card."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_wmark(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "FIFO threshold watermark level when transmitting data to card."]
    #[inline(always)]
    pub const fn set_tx_wmark(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "FIFO threshold watermark level when receiving data to card."]
    #[must_use]
    #[inline(always)]
    pub const fn rx_wmark(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "FIFO threshold watermark level when receiving data to card."]
    #[inline(always)]
    pub const fn set_rx_wmark(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_mts(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
    #[inline(always)]
    pub const fn set_dma_mts(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for Fifoth {
    #[inline(always)]
    fn default() -> Fifoth {
        Fifoth(0)
    }
}
impl core::fmt::Debug for Fifoth {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifoth")
            .field("tx_wmark", &self.tx_wmark())
            .field("rx_wmark", &self.rx_wmark())
            .field("dma_mts", &self.dma_mts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifoth {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifoth {{ tx_wmark: {=u16:?}, rx_wmark: {=u16:?}, dma_mts: {=u8:?} }}",
            self.tx_wmark(),
            self.rx_wmark(),
            self.dma_mts()
        )
    }
}
#[doc = "Internal DMAC Interrupt Enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idinten(pub u32);
impl Idinten {
    #[doc = "Transmit Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ti(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ri(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Fatal Bus Error Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fbe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fatal Bus Error Enable."]
    #[inline(always)]
    pub const fn set_fbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Descriptor Unavailable Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn du(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub const fn set_du(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Card Error summary Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ces(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Card Error summary Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ces(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Normal Interrupt Summary Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nis(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Normal Interrupt Summary Enable."]
    #[inline(always)]
    pub const fn set_nis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Abnormal Interrupt Summary Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ais(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Abnormal Interrupt Summary Enable."]
    #[inline(always)]
    pub const fn set_ais(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Idinten {
    #[inline(always)]
    fn default() -> Idinten {
        Idinten(0)
    }
}
impl core::fmt::Debug for Idinten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Idinten")
            .field("ti", &self.ti())
            .field("ri", &self.ri())
            .field("fbe", &self.fbe())
            .field("du", &self.du())
            .field("ces", &self.ces())
            .field("nis", &self.nis())
            .field("ais", &self.ais())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Idinten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Idinten {{ ti: {=bool:?}, ri: {=bool:?}, fbe: {=bool:?}, du: {=bool:?}, ces: {=bool:?}, nis: {=bool:?}, ais: {=bool:?} }}",
            self.ti(),
            self.ri(),
            self.fbe(),
            self.du(),
            self.ces(),
            self.nis(),
            self.ais()
        )
    }
}
#[doc = "Internal DMAC Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idsts(pub u32);
impl Idsts {
    #[doc = "Transmit Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ti(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt."]
    #[inline(always)]
    pub const fn set_ti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ri(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Interrupt."]
    #[inline(always)]
    pub const fn set_ri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Fatal Bus Error Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fbe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fatal Bus Error Interrupt."]
    #[inline(always)]
    pub const fn set_fbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Descriptor Unavailable Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn du(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub const fn set_du(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Card Error Summary."]
    #[must_use]
    #[inline(always)]
    pub const fn ces(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Card Error Summary."]
    #[inline(always)]
    pub const fn set_ces(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Normal Interrupt Summary."]
    #[must_use]
    #[inline(always)]
    pub const fn nis(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Normal Interrupt Summary."]
    #[inline(always)]
    pub const fn set_nis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Abnormal Interrupt Summary."]
    #[must_use]
    #[inline(always)]
    pub const fn ais(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Abnormal Interrupt Summary."]
    #[inline(always)]
    pub const fn set_ais(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Error Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn eb(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Error Bits."]
    #[inline(always)]
    pub const fn set_eb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "DMAC state machine present state."]
    #[must_use]
    #[inline(always)]
    pub const fn fsm(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x0f;
        val as u8
    }
    #[doc = "DMAC state machine present state."]
    #[inline(always)]
    pub const fn set_fsm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
    }
}
impl Default for Idsts {
    #[inline(always)]
    fn default() -> Idsts {
        Idsts(0)
    }
}
impl core::fmt::Debug for Idsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Idsts")
            .field("ti", &self.ti())
            .field("ri", &self.ri())
            .field("fbe", &self.fbe())
            .field("du", &self.du())
            .field("ces", &self.ces())
            .field("nis", &self.nis())
            .field("ais", &self.ais())
            .field("eb", &self.eb())
            .field("fsm", &self.fsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Idsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Idsts {{ ti: {=bool:?}, ri: {=bool:?}, fbe: {=bool:?}, du: {=bool:?}, ces: {=bool:?}, nis: {=bool:?}, ais: {=bool:?}, eb: {=u8:?}, fsm: {=u8:?} }}",
            self.ti(),
            self.ri(),
            self.fbe(),
            self.du(),
            self.ces(),
            self.nis(),
            self.ais(),
            self.eb(),
            self.fsm()
        )
    }
}
#[doc = "Interrupt Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intmask(pub u32);
impl Intmask {
    #[doc = "Card detect."]
    #[must_use]
    #[inline(always)]
    pub const fn cdet(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Card detect."]
    #[inline(always)]
    pub const fn set_cdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Response error."]
    #[must_use]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Response error."]
    #[inline(always)]
    pub const fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Command done."]
    #[must_use]
    #[inline(always)]
    pub const fn cdone(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Command done."]
    #[inline(always)]
    pub const fn set_cdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Data transfer over."]
    #[must_use]
    #[inline(always)]
    pub const fn dto(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Data transfer over."]
    #[inline(always)]
    pub const fn set_dto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit FIFO data request."]
    #[must_use]
    #[inline(always)]
    pub const fn txdr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO data request."]
    #[inline(always)]
    pub const fn set_txdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receive FIFO data request."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO data request."]
    #[inline(always)]
    pub const fn set_rxdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Response CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn rcrc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Response CRC error."]
    #[inline(always)]
    pub const fn set_rcrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Data CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn dcrc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Data CRC error."]
    #[inline(always)]
    pub const fn set_dcrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Response time-out."]
    #[must_use]
    #[inline(always)]
    pub const fn rto(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Response time-out."]
    #[inline(always)]
    pub const fn set_rto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Data read time-out."]
    #[must_use]
    #[inline(always)]
    pub const fn drto(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Data read time-out."]
    #[inline(always)]
    pub const fn set_drto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Data starvation-by-host time-out (HTO)."]
    #[must_use]
    #[inline(always)]
    pub const fn hto(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Data starvation-by-host time-out (HTO)."]
    #[inline(always)]
    pub const fn set_hto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "FIFO underrun/overrun error."]
    #[must_use]
    #[inline(always)]
    pub const fn frun(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO underrun/overrun error."]
    #[inline(always)]
    pub const fn set_frun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Hardware locked write error."]
    #[must_use]
    #[inline(always)]
    pub const fn hle(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware locked write error."]
    #[inline(always)]
    pub const fn set_hle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Start-bit error."]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Start-bit error."]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Auto command done."]
    #[must_use]
    #[inline(always)]
    pub const fn acd(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Auto command done."]
    #[inline(always)]
    pub const fn set_acd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "End-bit error (read)/Write no CRC."]
    #[must_use]
    #[inline(always)]
    pub const fn ebe(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "End-bit error (read)/Write no CRC."]
    #[inline(always)]
    pub const fn set_ebe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Mask SDIO interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn sdio_int_mask(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Mask SDIO interrupt."]
    #[inline(always)]
    pub const fn set_sdio_int_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Intmask {
    #[inline(always)]
    fn default() -> Intmask {
        Intmask(0)
    }
}
impl core::fmt::Debug for Intmask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intmask")
            .field("cdet", &self.cdet())
            .field("re", &self.re())
            .field("cdone", &self.cdone())
            .field("dto", &self.dto())
            .field("txdr", &self.txdr())
            .field("rxdr", &self.rxdr())
            .field("rcrc", &self.rcrc())
            .field("dcrc", &self.dcrc())
            .field("rto", &self.rto())
            .field("drto", &self.drto())
            .field("hto", &self.hto())
            .field("frun", &self.frun())
            .field("hle", &self.hle())
            .field("sbe", &self.sbe())
            .field("acd", &self.acd())
            .field("ebe", &self.ebe())
            .field("sdio_int_mask", &self.sdio_int_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intmask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intmask {{ cdet: {=bool:?}, re: {=bool:?}, cdone: {=bool:?}, dto: {=bool:?}, txdr: {=bool:?}, rxdr: {=bool:?}, rcrc: {=bool:?}, dcrc: {=bool:?}, rto: {=bool:?}, drto: {=bool:?}, hto: {=bool:?}, frun: {=bool:?}, hle: {=bool:?}, sbe: {=bool:?}, acd: {=bool:?}, ebe: {=bool:?}, sdio_int_mask: {=bool:?} }}",
            self.cdet(),
            self.re(),
            self.cdone(),
            self.dto(),
            self.txdr(),
            self.rxdr(),
            self.rcrc(),
            self.dcrc(),
            self.rto(),
            self.drto(),
            self.hto(),
            self.frun(),
            self.hle(),
            self.sbe(),
            self.acd(),
            self.ebe(),
            self.sdio_int_mask()
        )
    }
}
#[doc = "Masked Interrupt Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mintsts(pub u32);
impl Mintsts {
    #[doc = "Card detect."]
    #[must_use]
    #[inline(always)]
    pub const fn cdet(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Card detect."]
    #[inline(always)]
    pub const fn set_cdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Response error."]
    #[must_use]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Response error."]
    #[inline(always)]
    pub const fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Command done."]
    #[must_use]
    #[inline(always)]
    pub const fn cdone(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Command done."]
    #[inline(always)]
    pub const fn set_cdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Data transfer over."]
    #[must_use]
    #[inline(always)]
    pub const fn dto(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Data transfer over."]
    #[inline(always)]
    pub const fn set_dto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit FIFO data request."]
    #[must_use]
    #[inline(always)]
    pub const fn txdr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO data request."]
    #[inline(always)]
    pub const fn set_txdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receive FIFO data request."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO data request."]
    #[inline(always)]
    pub const fn set_rxdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Response CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn rcrc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Response CRC error."]
    #[inline(always)]
    pub const fn set_rcrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Data CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn dcrc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Data CRC error."]
    #[inline(always)]
    pub const fn set_dcrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Response time-out."]
    #[must_use]
    #[inline(always)]
    pub const fn rto(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Response time-out."]
    #[inline(always)]
    pub const fn set_rto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Data read time-out."]
    #[must_use]
    #[inline(always)]
    pub const fn drto(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Data read time-out."]
    #[inline(always)]
    pub const fn set_drto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Data starvation-by-host time-out (HTO)."]
    #[must_use]
    #[inline(always)]
    pub const fn hto(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Data starvation-by-host time-out (HTO)."]
    #[inline(always)]
    pub const fn set_hto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "FIFO underrun/overrun error."]
    #[must_use]
    #[inline(always)]
    pub const fn frun(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO underrun/overrun error."]
    #[inline(always)]
    pub const fn set_frun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Hardware locked write error."]
    #[must_use]
    #[inline(always)]
    pub const fn hle(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware locked write error."]
    #[inline(always)]
    pub const fn set_hle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Start-bit error."]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Start-bit error."]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Auto command done."]
    #[must_use]
    #[inline(always)]
    pub const fn acd(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Auto command done."]
    #[inline(always)]
    pub const fn set_acd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "End-bit error (read)/write no CRC."]
    #[must_use]
    #[inline(always)]
    pub const fn ebe(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "End-bit error (read)/write no CRC."]
    #[inline(always)]
    pub const fn set_ebe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt from SDIO card."]
    #[must_use]
    #[inline(always)]
    pub const fn sdio_interrupt(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt from SDIO card."]
    #[inline(always)]
    pub const fn set_sdio_interrupt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Mintsts {
    #[inline(always)]
    fn default() -> Mintsts {
        Mintsts(0)
    }
}
impl core::fmt::Debug for Mintsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mintsts")
            .field("cdet", &self.cdet())
            .field("re", &self.re())
            .field("cdone", &self.cdone())
            .field("dto", &self.dto())
            .field("txdr", &self.txdr())
            .field("rxdr", &self.rxdr())
            .field("rcrc", &self.rcrc())
            .field("dcrc", &self.dcrc())
            .field("rto", &self.rto())
            .field("drto", &self.drto())
            .field("hto", &self.hto())
            .field("frun", &self.frun())
            .field("hle", &self.hle())
            .field("sbe", &self.sbe())
            .field("acd", &self.acd())
            .field("ebe", &self.ebe())
            .field("sdio_interrupt", &self.sdio_interrupt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mintsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mintsts {{ cdet: {=bool:?}, re: {=bool:?}, cdone: {=bool:?}, dto: {=bool:?}, txdr: {=bool:?}, rxdr: {=bool:?}, rcrc: {=bool:?}, dcrc: {=bool:?}, rto: {=bool:?}, drto: {=bool:?}, hto: {=bool:?}, frun: {=bool:?}, hle: {=bool:?}, sbe: {=bool:?}, acd: {=bool:?}, ebe: {=bool:?}, sdio_interrupt: {=bool:?} }}",
            self.cdet(),
            self.re(),
            self.cdone(),
            self.dto(),
            self.txdr(),
            self.rxdr(),
            self.rcrc(),
            self.dcrc(),
            self.rto(),
            self.drto(),
            self.hto(),
            self.frun(),
            self.hle(),
            self.sbe(),
            self.acd(),
            self.ebe(),
            self.sdio_interrupt()
        )
    }
}
#[doc = "Poll Demand register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pldmnd(pub u32);
impl Pldmnd {
    #[doc = "Poll Demand."]
    #[must_use]
    #[inline(always)]
    pub const fn pd(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Poll Demand."]
    #[inline(always)]
    pub const fn set_pd(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pldmnd {
    #[inline(always)]
    fn default() -> Pldmnd {
        Pldmnd(0)
    }
}
impl core::fmt::Debug for Pldmnd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pldmnd").field("pd", &self.pd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pldmnd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pldmnd {{ pd: {=u32:?} }}", self.pd())
    }
}
#[doc = "Power Enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwren(pub u32);
impl Pwren {
    #[doc = "Power on/off switch for card 0; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 0."]
    #[must_use]
    #[inline(always)]
    pub const fn power_enable0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power on/off switch for card 0; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 0."]
    #[inline(always)]
    pub const fn set_power_enable0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Power on/off switch for card 1; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 1."]
    #[must_use]
    #[inline(always)]
    pub const fn power_enable1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Power on/off switch for card 1; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 1."]
    #[inline(always)]
    pub const fn set_power_enable1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Pwren {
    #[inline(always)]
    fn default() -> Pwren {
        Pwren(0)
    }
}
impl core::fmt::Debug for Pwren {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwren")
            .field("power_enable0", &self.power_enable0())
            .field("power_enable1", &self.power_enable1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwren {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwren {{ power_enable0: {=bool:?}, power_enable1: {=bool:?} }}",
            self.power_enable0(),
            self.power_enable1()
        )
    }
}
#[doc = "Response register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resp(pub u32);
impl Resp {
    #[doc = "Bits of response."]
    #[must_use]
    #[inline(always)]
    pub const fn response(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Bits of response."]
    #[inline(always)]
    pub const fn set_response(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Resp {
    #[inline(always)]
    fn default() -> Resp {
        Resp(0)
    }
}
impl core::fmt::Debug for Resp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Resp")
            .field("response", &self.response())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Resp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Resp {{ response: {=u32:?} }}", self.response())
    }
}
#[doc = "Raw Interrupt Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rintsts(pub u32);
impl Rintsts {
    #[doc = "Card detect."]
    #[must_use]
    #[inline(always)]
    pub const fn cdet(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Card detect."]
    #[inline(always)]
    pub const fn set_cdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Response error."]
    #[must_use]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Response error."]
    #[inline(always)]
    pub const fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Command done."]
    #[must_use]
    #[inline(always)]
    pub const fn cdone(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Command done."]
    #[inline(always)]
    pub const fn set_cdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Data transfer over."]
    #[must_use]
    #[inline(always)]
    pub const fn dto(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Data transfer over."]
    #[inline(always)]
    pub const fn set_dto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit FIFO data request."]
    #[must_use]
    #[inline(always)]
    pub const fn txdr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO data request."]
    #[inline(always)]
    pub const fn set_txdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receive FIFO data request."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO data request."]
    #[inline(always)]
    pub const fn set_rxdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Response CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn rcrc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Response CRC error."]
    #[inline(always)]
    pub const fn set_rcrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Data CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn dcrc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Data CRC error."]
    #[inline(always)]
    pub const fn set_dcrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Response time-out (RTO)/Boot Ack Received (BAR)."]
    #[must_use]
    #[inline(always)]
    pub const fn rto_bar(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Response time-out (RTO)/Boot Ack Received (BAR)."]
    #[inline(always)]
    pub const fn set_rto_bar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Data read time-out (DRTO)/Boot Data Start (BDS)."]
    #[must_use]
    #[inline(always)]
    pub const fn drto_bds(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Data read time-out (DRTO)/Boot Data Start (BDS)."]
    #[inline(always)]
    pub const fn set_drto_bds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Data starvation-by-host time-out (HTO)."]
    #[must_use]
    #[inline(always)]
    pub const fn hto(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Data starvation-by-host time-out (HTO)."]
    #[inline(always)]
    pub const fn set_hto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "FIFO underrun/overrun error."]
    #[must_use]
    #[inline(always)]
    pub const fn frun(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO underrun/overrun error."]
    #[inline(always)]
    pub const fn set_frun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Hardware locked write error."]
    #[must_use]
    #[inline(always)]
    pub const fn hle(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware locked write error."]
    #[inline(always)]
    pub const fn set_hle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Start-bit error."]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Start-bit error."]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Auto command done."]
    #[must_use]
    #[inline(always)]
    pub const fn acd(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Auto command done."]
    #[inline(always)]
    pub const fn set_acd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "End-bit error (read)/write no CRC."]
    #[must_use]
    #[inline(always)]
    pub const fn ebe(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "End-bit error (read)/write no CRC."]
    #[inline(always)]
    pub const fn set_ebe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt from SDIO card."]
    #[must_use]
    #[inline(always)]
    pub const fn sdio_interrupt(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt from SDIO card."]
    #[inline(always)]
    pub const fn set_sdio_interrupt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Rintsts {
    #[inline(always)]
    fn default() -> Rintsts {
        Rintsts(0)
    }
}
impl core::fmt::Debug for Rintsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rintsts")
            .field("cdet", &self.cdet())
            .field("re", &self.re())
            .field("cdone", &self.cdone())
            .field("dto", &self.dto())
            .field("txdr", &self.txdr())
            .field("rxdr", &self.rxdr())
            .field("rcrc", &self.rcrc())
            .field("dcrc", &self.dcrc())
            .field("rto_bar", &self.rto_bar())
            .field("drto_bds", &self.drto_bds())
            .field("hto", &self.hto())
            .field("frun", &self.frun())
            .field("hle", &self.hle())
            .field("sbe", &self.sbe())
            .field("acd", &self.acd())
            .field("ebe", &self.ebe())
            .field("sdio_interrupt", &self.sdio_interrupt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rintsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rintsts {{ cdet: {=bool:?}, re: {=bool:?}, cdone: {=bool:?}, dto: {=bool:?}, txdr: {=bool:?}, rxdr: {=bool:?}, rcrc: {=bool:?}, dcrc: {=bool:?}, rto_bar: {=bool:?}, drto_bds: {=bool:?}, hto: {=bool:?}, frun: {=bool:?}, hle: {=bool:?}, sbe: {=bool:?}, acd: {=bool:?}, ebe: {=bool:?}, sdio_interrupt: {=bool:?} }}",
            self.cdet(),
            self.re(),
            self.cdone(),
            self.dto(),
            self.txdr(),
            self.rxdr(),
            self.rcrc(),
            self.dcrc(),
            self.rto_bar(),
            self.drto_bds(),
            self.hto(),
            self.frun(),
            self.hle(),
            self.sbe(),
            self.acd(),
            self.ebe(),
            self.sdio_interrupt()
        )
    }
}
#[doc = "Hardware Reset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RstN(pub u32);
impl RstN {
    #[doc = "Hardware reset."]
    #[must_use]
    #[inline(always)]
    pub const fn card_reset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware reset."]
    #[inline(always)]
    pub const fn set_card_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RstN {
    #[inline(always)]
    fn default() -> RstN {
        RstN(0)
    }
}
impl core::fmt::Debug for RstN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RstN")
            .field("card_reset", &self.card_reset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RstN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RstN {{ card_reset: {=bool:?} }}", self.card_reset())
    }
}
#[doc = "Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "FIFO reached Receive watermark level; not qualified with data transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_rx_watermark(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO reached Receive watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub const fn set_fifo_rx_watermark(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO reached Transmit watermark level; not qualified with data transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_tx_watermark(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO reached Transmit watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub const fn set_fifo_tx_watermark(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO is empty status."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_empty(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO is empty status."]
    #[inline(always)]
    pub const fn set_fifo_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIFO is full status."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_full(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO is full status."]
    #[inline(always)]
    pub const fn set_fifo_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdfsmstates(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
    #[inline(always)]
    pub const fn set_cmdfsmstates(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
    #[must_use]
    #[inline(always)]
    pub const fn data_3_status(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
    #[inline(always)]
    pub const fn set_data_3_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Inverted version of raw selected card_data\\[0\\] 0 - card data not busy 1 - card data busy."]
    #[must_use]
    #[inline(always)]
    pub const fn data_busy(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Inverted version of raw selected card_data\\[0\\] 0 - card data not busy 1 - card data busy."]
    #[inline(always)]
    pub const fn set_data_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Data transmit or receive state-machine is busy."]
    #[must_use]
    #[inline(always)]
    pub const fn data_state_mc_busy(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Data transmit or receive state-machine is busy."]
    #[inline(always)]
    pub const fn set_data_state_mc_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Index of previous response, including any auto-stop sent by core."]
    #[must_use]
    #[inline(always)]
    pub const fn response_index(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x3f;
        val as u8
    }
    #[doc = "Index of previous response, including any auto-stop sent by core."]
    #[inline(always)]
    pub const fn set_response_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 11usize)) | (((val as u32) & 0x3f) << 11usize);
    }
    #[doc = "FIFO count - Number of filled locations in FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_count(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x1fff;
        val as u16
    }
    #[doc = "FIFO count - Number of filled locations in FIFO."]
    #[inline(always)]
    pub const fn set_fifo_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 17usize)) | (((val as u32) & 0x1fff) << 17usize);
    }
    #[doc = "DMA acknowledge signal state."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_ack(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA acknowledge signal state."]
    #[inline(always)]
    pub const fn set_dma_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA request signal state."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_req(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA request signal state."]
    #[inline(always)]
    pub const fn set_dma_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("fifo_rx_watermark", &self.fifo_rx_watermark())
            .field("fifo_tx_watermark", &self.fifo_tx_watermark())
            .field("fifo_empty", &self.fifo_empty())
            .field("fifo_full", &self.fifo_full())
            .field("cmdfsmstates", &self.cmdfsmstates())
            .field("data_3_status", &self.data_3_status())
            .field("data_busy", &self.data_busy())
            .field("data_state_mc_busy", &self.data_state_mc_busy())
            .field("response_index", &self.response_index())
            .field("fifo_count", &self.fifo_count())
            .field("dma_ack", &self.dma_ack())
            .field("dma_req", &self.dma_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ fifo_rx_watermark: {=bool:?}, fifo_tx_watermark: {=bool:?}, fifo_empty: {=bool:?}, fifo_full: {=bool:?}, cmdfsmstates: {=u8:?}, data_3_status: {=bool:?}, data_busy: {=bool:?}, data_state_mc_busy: {=bool:?}, response_index: {=u8:?}, fifo_count: {=u16:?}, dma_ack: {=bool:?}, dma_req: {=bool:?} }}",
            self.fifo_rx_watermark(),
            self.fifo_tx_watermark(),
            self.fifo_empty(),
            self.fifo_full(),
            self.cmdfsmstates(),
            self.data_3_status(),
            self.data_busy(),
            self.data_state_mc_busy(),
            self.response_index(),
            self.fifo_count(),
            self.dma_ack(),
            self.dma_req()
        )
    }
}
#[doc = "Transferred Host to BIU-FIFO Byte Count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tbbcnt(pub u32);
impl Tbbcnt {
    #[doc = "Number of bytes transferred between Host/DMA memory and BIU FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn trans_fifo_byte_count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Number of bytes transferred between Host/DMA memory and BIU FIFO."]
    #[inline(always)]
    pub const fn set_trans_fifo_byte_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tbbcnt {
    #[inline(always)]
    fn default() -> Tbbcnt {
        Tbbcnt(0)
    }
}
impl core::fmt::Debug for Tbbcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tbbcnt")
            .field("trans_fifo_byte_count", &self.trans_fifo_byte_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tbbcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tbbcnt {{ trans_fifo_byte_count: {=u32:?} }}",
            self.trans_fifo_byte_count()
        )
    }
}
#[doc = "Transferred CIU Card Byte Count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcbcnt(pub u32);
impl Tcbcnt {
    #[doc = "Number of bytes transferred by CIU unit to card."]
    #[must_use]
    #[inline(always)]
    pub const fn trans_card_byte_count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Number of bytes transferred by CIU unit to card."]
    #[inline(always)]
    pub const fn set_trans_card_byte_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcbcnt {
    #[inline(always)]
    fn default() -> Tcbcnt {
        Tcbcnt(0)
    }
}
impl core::fmt::Debug for Tcbcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcbcnt")
            .field("trans_card_byte_count", &self.trans_card_byte_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcbcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcbcnt {{ trans_card_byte_count: {=u32:?} }}",
            self.trans_card_byte_count()
        )
    }
}
#[doc = "Time-out register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmout(pub u32);
impl Tmout {
    #[doc = "Response time-out value."]
    #[must_use]
    #[inline(always)]
    pub const fn response_timeout(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Response time-out value."]
    #[inline(always)]
    pub const fn set_response_timeout(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Value for card Data Read time-out; same value also used for Data Starvation by Host time-out."]
    #[must_use]
    #[inline(always)]
    pub const fn data_timeout(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Value for card Data Read time-out; same value also used for Data Starvation by Host time-out."]
    #[inline(always)]
    pub const fn set_data_timeout(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Tmout {
    #[inline(always)]
    fn default() -> Tmout {
        Tmout(0)
    }
}
impl core::fmt::Debug for Tmout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tmout")
            .field("response_timeout", &self.response_timeout())
            .field("data_timeout", &self.data_timeout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tmout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tmout {{ response_timeout: {=u8:?}, data_timeout: {=u32:?} }}",
            self.response_timeout(),
            self.data_timeout()
        )
    }
}
#[doc = "Write Protect register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wrtprt(pub u32);
impl Wrtprt {
    #[doc = "Write protect."]
    #[must_use]
    #[inline(always)]
    pub const fn write_protect(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write protect."]
    #[inline(always)]
    pub const fn set_write_protect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Wrtprt {
    #[inline(always)]
    fn default() -> Wrtprt {
        Wrtprt(0)
    }
}
impl core::fmt::Debug for Wrtprt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wrtprt")
            .field("write_protect", &self.write_protect())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wrtprt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wrtprt {{ write_protect: {=bool:?} }}",
            self.write_protect()
        )
    }
}
