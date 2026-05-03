#[doc = "Aliases to allow writing words in a burst."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ALIAS(pub u32);
impl ALIAS {
    #[doc = "Write next word in little-endian form. The hash requires big endian word data, but this block swaps the bytes automatically. That is, SHA assumes the data coming in is treated as bytes (e.g. \"abcd\") and since the ARM core will treat \"abcd\" as a word as 0x64636261, the block will swap the word to restore into big endian."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Write next word in little-endian form. The hash requires big endian word data, but this block swaps the bytes automatically. That is, SHA assumes the data coming in is treated as bytes (e.g. \"abcd\") and since the ARM core will treat \"abcd\" as a word as 0x64636261, the block will swap the word to restore into big endian."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ALIAS {
    #[inline(always)]
    fn default() -> ALIAS {
        ALIAS(0)
    }
}
impl core::fmt::Debug for ALIAS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALIAS").field("DATA", &self.DATA()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ALIAS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ALIAS {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Returns the configuration of this block in this chip - indicates what services are available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG(pub u32);
impl CONFIG {
    #[doc = "1 if 2 x 512 bit buffers, 0 if only 1 x 512 bit."]
    #[must_use]
    #[inline(always)]
    pub const fn DUAL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1 if 2 x 512 bit buffers, 0 if only 1 x 512 bit."]
    #[inline(always)]
    pub const fn set_DUAL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1 if DMA is connected."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1 if DMA is connected."]
    #[inline(always)]
    pub const fn set_DMA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "1 if AHB Master is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn AHB(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1 if AHB Master is enabled."]
    #[inline(always)]
    pub const fn set_AHB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "1 if AES 128 included."]
    #[must_use]
    #[inline(always)]
    pub const fn AES(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "1 if AES 128 included."]
    #[inline(always)]
    pub const fn set_AES(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "1 if AES 192 and 256 also included."]
    #[must_use]
    #[inline(always)]
    pub const fn AESKEY(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "1 if AES 192 and 256 also included."]
    #[inline(always)]
    pub const fn set_AESKEY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "1 if AES Secret key available."]
    #[must_use]
    #[inline(always)]
    pub const fn SECRET(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1 if AES Secret key available."]
    #[inline(always)]
    pub const fn set_SECRET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for CONFIG {
    #[inline(always)]
    fn default() -> CONFIG {
        CONFIG(0)
    }
}
impl core::fmt::Debug for CONFIG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG")
            .field("DUAL", &self.DUAL())
            .field("DMA", &self.DMA())
            .field("AHB", &self.AHB())
            .field("AES", &self.AES())
            .field("AESKEY", &self.AESKEY())
            .field("SECRET", &self.SECRET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG {{ DUAL: {=bool:?}, DMA: {=bool:?}, AHB: {=bool:?}, AES: {=bool:?}, AESKEY: {=bool:?}, SECRET: {=bool:?} }}",
            self.DUAL(),
            self.DMA(),
            self.AHB(),
            self.AES(),
            self.AESKEY(),
            self.SECRET()
        )
    }
}
#[doc = "Crypto settings for AES and Salsa and ChaCha."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CRYPTCFG(pub u32);
impl CRYPTCFG {
    #[doc = "If 1, OUTDATA0 will be read Most significant word 1st for AES. Else it will be read in normal little endian - Least significant word 1st. Note: only if allowed by configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn MSW1ST_OUT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, OUTDATA0 will be read Most significant word 1st for AES. Else it will be read in normal little endian - Least significant word 1st. Note: only if allowed by configuration."]
    #[inline(always)]
    pub const fn set_MSW1ST_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, will Swap the key input (bytes in each word)."]
    #[must_use]
    #[inline(always)]
    pub const fn SWAPKEY(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, will Swap the key input (bytes in each word)."]
    #[inline(always)]
    pub const fn set_SWAPKEY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, will SWAP the data and IV inputs (bytes in each word)."]
    #[must_use]
    #[inline(always)]
    pub const fn SWAPDAT(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, will SWAP the data and IV inputs (bytes in each word)."]
    #[inline(always)]
    pub const fn set_SWAPDAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, load of key, IV, and data is MSW 1st for AES. Else, the words are little endian. Note: only if allowed by configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn MSW1ST(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, load of key, IV, and data is MSW 1st for AES. Else, the words are little endian. Note: only if allowed by configuration."]
    #[inline(always)]
    pub const fn set_MSW1ST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "AES Cipher mode to use if plain AES."]
    #[must_use]
    #[inline(always)]
    pub const fn AESMODE(&self) -> super::vals::AESMODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::AESMODE::from_bits(val as u8)
    }
    #[doc = "AES Cipher mode to use if plain AES."]
    #[inline(always)]
    pub const fn set_AESMODE(&mut self, val: super::vals::AESMODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "AES ECB direction. Only encryption used if CTR mode or manual modes such as CFB."]
    #[must_use]
    #[inline(always)]
    pub const fn AESDECRYPT(&self) -> super::vals::AESDECRYPT {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::AESDECRYPT::from_bits(val as u8)
    }
    #[doc = "AES ECB direction. Only encryption used if CTR mode or manual modes such as CFB."]
    #[inline(always)]
    pub const fn set_AESDECRYPT(&mut self, val: super::vals::AESDECRYPT) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Selects the Hidden Secret key vs. User key, if provided. If security levels are used, only the highest level is permitted to select this."]
    #[must_use]
    #[inline(always)]
    pub const fn AESSECRET(&self) -> super::vals::AESSECRET {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::AESSECRET::from_bits(val as u8)
    }
    #[doc = "Selects the Hidden Secret key vs. User key, if provided. If security levels are used, only the highest level is permitted to select this."]
    #[inline(always)]
    pub const fn set_AESSECRET(&mut self, val: super::vals::AESSECRET) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sets the AES key size."]
    #[must_use]
    #[inline(always)]
    pub const fn AESKEYSZ(&self) -> super::vals::AESKEYSZ {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::AESKEYSZ::from_bits(val as u8)
    }
    #[doc = "Sets the AES key size."]
    #[inline(always)]
    pub const fn set_AESKEYSZ(&mut self, val: super::vals::AESKEYSZ) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Halfword position of 16b counter in IV if AESMODE is CTR (position is fixed for Salsa and ChaCha). Only supports 16b counter, so application must control any additional bytes if using more. The 16-bit counter is read from the IV and incremented by 1 each time. Any other use CTR should use ECB directly and do its own XOR and so on."]
    #[must_use]
    #[inline(always)]
    pub const fn AESCTRPOS(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Halfword position of 16b counter in IV if AESMODE is CTR (position is fixed for Salsa and ChaCha). Only supports 16b counter, so application must control any additional bytes if using more. The 16-bit counter is read from the IV and incremented by 1 each time. Any other use CTR should use ECB directly and do its own XOR and so on."]
    #[inline(always)]
    pub const fn set_AESCTRPOS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "Is 1 if last stream block. If not 1, then the engine will compute the next \"hash\"."]
    #[must_use]
    #[inline(always)]
    pub const fn STREAMLAST(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Is 1 if last stream block. If not 1, then the engine will compute the next \"hash\"."]
    #[inline(always)]
    pub const fn set_STREAMLAST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for CRYPTCFG {
    #[inline(always)]
    fn default() -> CRYPTCFG {
        CRYPTCFG(0)
    }
}
impl core::fmt::Debug for CRYPTCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRYPTCFG")
            .field("MSW1ST_OUT", &self.MSW1ST_OUT())
            .field("SWAPKEY", &self.SWAPKEY())
            .field("SWAPDAT", &self.SWAPDAT())
            .field("MSW1ST", &self.MSW1ST())
            .field("AESMODE", &self.AESMODE())
            .field("AESDECRYPT", &self.AESDECRYPT())
            .field("AESSECRET", &self.AESSECRET())
            .field("AESKEYSZ", &self.AESKEYSZ())
            .field("AESCTRPOS", &self.AESCTRPOS())
            .field("STREAMLAST", &self.STREAMLAST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CRYPTCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CRYPTCFG {{ MSW1ST_OUT: {=bool:?}, SWAPKEY: {=bool:?}, SWAPDAT: {=bool:?}, MSW1ST: {=bool:?}, AESMODE: {:?}, AESDECRYPT: {:?}, AESSECRET: {:?}, AESKEYSZ: {:?}, AESCTRPOS: {=u8:?}, STREAMLAST: {=bool:?} }}",
            self.MSW1ST_OUT(),
            self.SWAPKEY(),
            self.SWAPDAT(),
            self.MSW1ST(),
            self.AESMODE(),
            self.AESDECRYPT(),
            self.AESSECRET(),
            self.AESKEYSZ(),
            self.AESCTRPOS(),
            self.STREAMLAST()
        )
    }
}
#[doc = "Control register to enable and operate Hash and Crypto."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available."]
    #[must_use]
    #[inline(always)]
    pub const fn Mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available."]
    #[inline(always)]
    pub const fn set_Mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Written with 1 when starting a new Hash/Crypto. It self clears. Note that the WAITING Status bit will clear for a cycle during the initialization from New=1."]
    #[must_use]
    #[inline(always)]
    pub const fn New_Hash(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Written with 1 when starting a new Hash/Crypto. It self clears. Note that the WAITING Status bit will clear for a cycle during the initialization from New=1."]
    #[inline(always)]
    pub const fn set_New_Hash(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, allows the SHA RELOAD registers to be used. This is used to save a partial Hash Digest (e.g. when need to run AES) and then reload it later for continuation."]
    #[must_use]
    #[inline(always)]
    pub const fn Reload(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, allows the SHA RELOAD registers to be used. This is used to save a partial Hash Digest (e.g. when need to run AES) and then reload it later for continuation."]
    #[inline(always)]
    pub const fn set_Reload(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed)."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA_I(&self) -> super::vals::DMA_I {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::DMA_I::from_bits(val as u8)
    }
    #[doc = "Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed)."]
    #[inline(always)]
    pub const fn set_DMA_I(&mut self, val: super::vals::DMA_I) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA_O(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses."]
    #[inline(always)]
    pub const fn set_DMA_O(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "If 1, will swap bytes in the word for SHA hashing. The default is byte order (so LSB is 1st byte) but this allows swapping to MSB is 1st such as is shown in SHS spec. For cryptographic swapping, see the CRYPTCFG register."]
    #[must_use]
    #[inline(always)]
    pub const fn HASHSWPB(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, will swap bytes in the word for SHA hashing. The default is byte order (so LSB is 1st byte) but this allows swapping to MSB is 1st such as is shown in SHS spec. For cryptographic swapping, see the CRYPTCFG register."]
    #[inline(always)]
    pub const fn set_HASHSWPB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Flushes the AES engine registers. This bit self clears."]
    #[must_use]
    #[inline(always)]
    pub const fn AESFLUSH(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Flushes the AES engine registers. This bit self clears."]
    #[inline(always)]
    pub const fn set_AESFLUSH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
            .field("Mode", &self.Mode())
            .field("New_Hash", &self.New_Hash())
            .field("Reload", &self.Reload())
            .field("DMA_I", &self.DMA_I())
            .field("DMA_O", &self.DMA_O())
            .field("HASHSWPB", &self.HASHSWPB())
            .field("AESFLUSH", &self.AESFLUSH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL {{ Mode: {:?}, New_Hash: {=bool:?}, Reload: {=bool:?}, DMA_I: {:?}, DMA_O: {=bool:?}, HASHSWPB: {=bool:?}, AESFLUSH: {=bool:?} }}",
            self.Mode(),
            self.New_Hash(),
            self.Reload(),
            self.DMA_I(),
            self.DMA_O(),
            self.HASHSWPB(),
            self.AESFLUSH()
        )
    }
}
#[doc = "Result digest (when status says so): Is 1st 5 words if SHA1 used Is all 8 words if SHA2 used Is all 8 words if crypto or SHA512."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DIGEST0(pub u32);
impl DIGEST0 {
    #[doc = "One word of the Digest or output. Note that only 1st 4 are populated for AES and 1st 5 are populated for SHA1."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGEST(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "One word of the Digest or output. Note that only 1st 4 are populated for AES and 1st 5 are populated for SHA1."]
    #[inline(always)]
    pub const fn set_DIGEST(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DIGEST0 {
    #[inline(always)]
    fn default() -> DIGEST0 {
        DIGEST0(0)
    }
}
impl core::fmt::Debug for DIGEST0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIGEST0")
            .field("DIGEST", &self.DIGEST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DIGEST0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DIGEST0 {{ DIGEST: {=u32:?} }}", self.DIGEST())
    }
}
#[doc = "Input of 16 words at a time to load up buffer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INDATA(pub u32);
impl INDATA {
    #[doc = "Write next word in little-endian form. The hash requires big endian word data, but this block swaps the bytes automatically. That is, SHA assumes the data coming in is treated as bytes (e.g. \"abcd\") and since the ARM core will treat \"abcd\" as a word as 0x64636261, the block will swap the word to restore into big endian."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Write next word in little-endian form. The hash requires big endian word data, but this block swaps the bytes automatically. That is, SHA assumes the data coming in is treated as bytes (e.g. \"abcd\") and since the ARM core will treat \"abcd\" as a word as 0x64636261, the block will swap the word to restore into big endian."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for INDATA {
    #[inline(always)]
    fn default() -> INDATA {
        INDATA(0)
    }
}
impl core::fmt::Debug for INDATA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INDATA")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INDATA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INDATA {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Write 1 to clear interrupts."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTENCLR(pub u32);
impl INTENCLR {
    #[doc = "Write 1 to clear mask."]
    #[must_use]
    #[inline(always)]
    pub const fn WAITING(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear mask."]
    #[inline(always)]
    pub const fn set_WAITING(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write 1 to clear mask."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGEST(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear mask."]
    #[inline(always)]
    pub const fn set_DIGEST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write 1 to clear mask."]
    #[must_use]
    #[inline(always)]
    pub const fn ERROR(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear mask."]
    #[inline(always)]
    pub const fn set_ERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write 1 to clear mask."]
    #[must_use]
    #[inline(always)]
    pub const fn FAULT(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear mask."]
    #[inline(always)]
    pub const fn set_FAULT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
            .field("WAITING", &self.WAITING())
            .field("DIGEST", &self.DIGEST())
            .field("ERROR", &self.ERROR())
            .field("FAULT", &self.FAULT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTENCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTENCLR {{ WAITING: {=bool:?}, DIGEST: {=bool:?}, ERROR: {=bool:?}, FAULT: {=bool:?} }}",
            self.WAITING(),
            self.DIGEST(),
            self.ERROR(),
            self.FAULT()
        )
    }
}
#[doc = "Write 1 to enable interrupts; reads back with which are set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTENSET(pub u32);
impl INTENSET {
    #[doc = "Indicates if should interrupt when waiting for data input."]
    #[must_use]
    #[inline(always)]
    pub const fn WAITING(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if should interrupt when waiting for data input."]
    #[inline(always)]
    pub const fn set_WAITING(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates if should interrupt when Digest (or Outdata) is ready (completed a hash/crypto or completed a full sequence)."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGEST(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if should interrupt when Digest (or Outdata) is ready (completed a hash/crypto or completed a full sequence)."]
    #[inline(always)]
    pub const fn set_DIGEST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates if should interrupt on an ERROR (as defined in Status)."]
    #[must_use]
    #[inline(always)]
    pub const fn ERROR(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if should interrupt on an ERROR (as defined in Status)."]
    #[inline(always)]
    pub const fn set_ERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates if should interrupt on an AES or PRNG fault as indicated in the STATUS register."]
    #[must_use]
    #[inline(always)]
    pub const fn FAULT(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if should interrupt on an AES or PRNG fault as indicated in the STATUS register."]
    #[inline(always)]
    pub const fn set_FAULT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
            .field("WAITING", &self.WAITING())
            .field("DIGEST", &self.DIGEST())
            .field("ERROR", &self.ERROR())
            .field("FAULT", &self.FAULT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTENSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTENSET {{ WAITING: {=bool:?}, DIGEST: {=bool:?}, ERROR: {=bool:?}, FAULT: {=bool:?} }}",
            self.WAITING(),
            self.DIGEST(),
            self.ERROR(),
            self.FAULT()
        )
    }
}
#[doc = "Lock register allows locking to the current security level or unlocking by the lock holding level."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LOCK(pub u32);
impl LOCK {
    #[doc = "Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level."]
    #[must_use]
    #[inline(always)]
    pub const fn SECLOCK(&self) -> super::vals::SECLOCK {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SECLOCK::from_bits(val as u8)
    }
    #[doc = "Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level."]
    #[inline(always)]
    pub const fn set_SECLOCK(&mut self, val: super::vals::SECLOCK) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Must write 0xA75 to change lock state. A75:Pattern needed to change bits 1:0."]
    #[must_use]
    #[inline(always)]
    pub const fn PATTERN(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "Must write 0xA75 to change lock state. A75:Pattern needed to change bits 1:0."]
    #[inline(always)]
    pub const fn set_PATTERN(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
}
impl Default for LOCK {
    #[inline(always)]
    fn default() -> LOCK {
        LOCK(0)
    }
}
impl core::fmt::Debug for LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCK")
            .field("SECLOCK", &self.SECLOCK())
            .field("PATTERN", &self.PATTERN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LOCK {{ SECLOCK: {:?}, PATTERN: {=u16:?} }}",
            self.SECLOCK(),
            self.PATTERN()
        )
    }
}
#[doc = "Allows Application to write a random mask for ICB use. Normally only a new one on each system reset (including power up)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MASK(pub u32);
impl MASK {
    #[doc = "A random word."]
    #[must_use]
    #[inline(always)]
    pub const fn MASK(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "A random word."]
    #[inline(always)]
    pub const fn set_MASK(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MASK {
    #[inline(always)]
    fn default() -> MASK {
        MASK(0)
    }
}
impl core::fmt::Debug for MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK").field("MASK", &self.MASK()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MASK {{ MASK: {=u32:?} }}", self.MASK())
    }
}
#[doc = "Address to start memory access from (if available)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MEMADDR(pub u32);
impl MEMADDR {
    #[doc = "Address base to start copying from, word aligned (so bits 1:0 must be 0). This field will advance as it processes the words. If it fails with a bus error, the register will contain the failing word. N:Address in Flash or RAM space; RAM only as mapped in this part. May also be able to address SPIFI."]
    #[must_use]
    #[inline(always)]
    pub const fn BASE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address base to start copying from, word aligned (so bits 1:0 must be 0). This field will advance as it processes the words. If it fails with a bus error, the register will contain the failing word. N:Address in Flash or RAM space; RAM only as mapped in this part. May also be able to address SPIFI."]
    #[inline(always)]
    pub const fn set_BASE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MEMADDR {
    #[inline(always)]
    fn default() -> MEMADDR {
        MEMADDR(0)
    }
}
impl core::fmt::Debug for MEMADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMADDR")
            .field("BASE", &self.BASE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MEMADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MEMADDR {{ BASE: {=u32:?} }}", self.BASE())
    }
}
#[doc = "Setup Master to access memory (if available)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MEMCTRL(pub u32);
impl MEMCTRL {
    #[doc = "Enables mastering."]
    #[must_use]
    #[inline(always)]
    pub const fn MASTER(&self) -> super::vals::MASTER {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MASTER::from_bits(val as u8)
    }
    #[doc = "Enables mastering."]
    #[inline(always)]
    pub const fn set_MASTER(&mut self, val: super::vals::MASTER) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Number of 512-bit (128-bit if AES, except 1st block which may include key and IV) blocks to copy starting at MEMADDR. This register will decrement after each block is copied, ending in 0. For Hash, the DIGEST interrupt will occur when it reaches 0. Fro AES, the DIGEST/OUTDATA interrupt will occur on ever block. If a bus error occurs, it will stop with this field set to the block that failed. 0:Done - nothing to process. 1 to 2K: Number of 512-bit (or 128bit) blocks to hash."]
    #[must_use]
    #[inline(always)]
    pub const fn COUNT(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Number of 512-bit (128-bit if AES, except 1st block which may include key and IV) blocks to copy starting at MEMADDR. This register will decrement after each block is copied, ending in 0. For Hash, the DIGEST interrupt will occur when it reaches 0. Fro AES, the DIGEST/OUTDATA interrupt will occur on ever block. If a bus error occurs, it will stop with this field set to the block that failed. 0:Done - nothing to process. 1 to 2K: Number of 512-bit (or 128bit) blocks to hash."]
    #[inline(always)]
    pub const fn set_COUNT(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for MEMCTRL {
    #[inline(always)]
    fn default() -> MEMCTRL {
        MEMCTRL(0)
    }
}
impl core::fmt::Debug for MEMCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMCTRL")
            .field("MASTER", &self.MASTER())
            .field("COUNT", &self.COUNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MEMCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MEMCTRL {{ MASTER: {:?}, COUNT: {=u16:?} }}",
            self.MASTER(),
            self.COUNT()
        )
    }
}
#[doc = "Provide random number."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRNG_OUT(pub u32);
impl PRNG_OUT {
    #[doc = "Provide random number."]
    #[must_use]
    #[inline(always)]
    pub const fn PRNG_OUT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Provide random number."]
    #[inline(always)]
    pub const fn set_PRNG_OUT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRNG_OUT {
    #[inline(always)]
    fn default() -> PRNG_OUT {
        PRNG_OUT(0)
    }
}
impl core::fmt::Debug for PRNG_OUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRNG_OUT")
            .field("PRNG_OUT", &self.PRNG_OUT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRNG_OUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PRNG_OUT {{ PRNG_OUT: {=u32:?} }}", self.PRNG_OUT())
    }
}
#[doc = "PRNG random input value used as an entropy source."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRNG_SEED(pub u32);
impl PRNG_SEED {
    #[doc = "Random input value used as an entropy source."]
    #[must_use]
    #[inline(always)]
    pub const fn PRNG_SEED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Random input value used as an entropy source."]
    #[inline(always)]
    pub const fn set_PRNG_SEED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRNG_SEED {
    #[inline(always)]
    fn default() -> PRNG_SEED {
        PRNG_SEED(0)
    }
}
impl core::fmt::Debug for PRNG_SEED {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRNG_SEED")
            .field("PRNG_SEED", &self.PRNG_SEED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRNG_SEED {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PRNG_SEED {{ PRNG_SEED: {=u32:?} }}", self.PRNG_SEED())
    }
}
#[doc = "The WO digest-reload registers may be written with a saved Hash digest, to allow continuation from where left off. These registers may only be written if the Reload field in CTRL is 1. If SHA1, only the 1st 5 are used."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RELOAD(pub u32);
impl RELOAD {
    #[doc = "SHA Digest word to reload."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGEST(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SHA Digest word to reload."]
    #[inline(always)]
    pub const fn set_DIGEST(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RELOAD {
    #[inline(always)]
    fn default() -> RELOAD {
        RELOAD(0)
    }
}
impl core::fmt::Debug for RELOAD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RELOAD")
            .field("DIGEST", &self.DIGEST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RELOAD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RELOAD {{ DIGEST: {=u32:?} }}", self.DIGEST())
    }
}
#[doc = "Indicates status of Hash peripheral."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STATUS(pub u32);
impl STATUS {
    #[doc = "If 1, the block is waiting for more data to process."]
    #[must_use]
    #[inline(always)]
    pub const fn WAITING(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the block is waiting for more data to process."]
    #[inline(always)]
    pub const fn set_WAITING(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "For Hash, if 1 then a DIGEST is ready and waiting and there is no active next block already started. For Cryptographic uses, this will be set for each block processed, indicating OUTDATA (and OUTDATA2 if larger output) contains the next value to read out. This is cleared when any data is written, when New is written, for Cryptographic uses when the last word is read out, or when the block is disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGEST(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "For Hash, if 1 then a DIGEST is ready and waiting and there is no active next block already started. For Cryptographic uses, this will be set for each block processed, indicating OUTDATA (and OUTDATA2 if larger output) contains the next value to read out. This is cleared when any data is written, when New is written, for Cryptographic uses when the last word is read out, or when the block is disabled."]
    #[inline(always)]
    pub const fn set_DIGEST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, an error occurred. For normal uses, this is due to an attempted overrun: INDATA was written when it was not appropriate. For Master cases, this is an AHB bus error; the COUNT field will indicate which block it was on."]
    #[must_use]
    #[inline(always)]
    pub const fn ERROR(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, an error occurred. For normal uses, this is due to an attempted overrun: INDATA was written when it was not appropriate. For Master cases, this is an AHB bus error; the COUNT field will indicate which block it was on."]
    #[inline(always)]
    pub const fn set_ERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates if an AES or PRNG fault has occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn FAULT(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if an AES or PRNG fault has occurred."]
    #[inline(always)]
    pub const fn set_FAULT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Indicates the block wants the key to be written in (set along with WAITING)."]
    #[must_use]
    #[inline(always)]
    pub const fn NEEDKEY(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the block wants the key to be written in (set along with WAITING)."]
    #[inline(always)]
    pub const fn set_NEEDKEY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates the block wants an IV/NONE to be written in (set along with WAITING)."]
    #[must_use]
    #[inline(always)]
    pub const fn NEEDIV(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the block wants an IV/NONE to be written in (set along with WAITING)."]
    #[inline(always)]
    pub const fn set_NEEDIV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "AES fault status."]
    #[must_use]
    #[inline(always)]
    pub const fn AESFAULT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "AES fault status."]
    #[inline(always)]
    pub const fn set_AESFAULT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PRNG fault status."]
    #[must_use]
    #[inline(always)]
    pub const fn PRNGFAULT(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PRNG fault status."]
    #[inline(always)]
    pub const fn set_PRNGFAULT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("WAITING", &self.WAITING())
            .field("DIGEST", &self.DIGEST())
            .field("ERROR", &self.ERROR())
            .field("FAULT", &self.FAULT())
            .field("NEEDKEY", &self.NEEDKEY())
            .field("NEEDIV", &self.NEEDIV())
            .field("AESFAULT", &self.AESFAULT())
            .field("PRNGFAULT", &self.PRNGFAULT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STATUS {{ WAITING: {=bool:?}, DIGEST: {=bool:?}, ERROR: {=bool:?}, FAULT: {=bool:?}, NEEDKEY: {=bool:?}, NEEDIV: {=bool:?}, AESFAULT: {=bool:?}, PRNGFAULT: {=bool:?} }}",
            self.WAITING(),
            self.DIGEST(),
            self.ERROR(),
            self.FAULT(),
            self.NEEDKEY(),
            self.NEEDIV(),
            self.AESFAULT(),
            self.PRNGFAULT()
        )
    }
}
