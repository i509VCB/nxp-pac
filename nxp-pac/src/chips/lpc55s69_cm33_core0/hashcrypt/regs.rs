#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Alias(pub u32);
impl Alias {
    #[doc = "Write next word in little-endian form. The hash requires big endian word data, but this block swaps the bytes automatically. That is, SHA assumes the data coming in is treated as bytes (e.g. \"abcd\") and since the ARM core will treat \"abcd\" as a word as 0x64636261, the block will swap the word to restore into big endian."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Write next word in little-endian form. The hash requires big endian word data, but this block swaps the bytes automatically. That is, SHA assumes the data coming in is treated as bytes (e.g. \"abcd\") and since the ARM core will treat \"abcd\" as a word as 0x64636261, the block will swap the word to restore into big endian."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Alias {
    #[inline(always)]
    fn default() -> Alias {
        Alias(0)
    }
}
impl core::fmt::Debug for Alias {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Alias").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Alias {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Alias {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Returns the configuration of this block in this chip - indicates what services are available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "1 if 2 x 512 bit buffers, 0 if only 1 x 512 bit"]
    #[must_use]
    #[inline(always)]
    pub const fn dual(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1 if 2 x 512 bit buffers, 0 if only 1 x 512 bit"]
    #[inline(always)]
    pub const fn set_dual(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1 if DMA is connected"]
    #[must_use]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1 if DMA is connected"]
    #[inline(always)]
    pub const fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "1 if AHB Master is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn ahb(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1 if AHB Master is enabled"]
    #[inline(always)]
    pub const fn set_ahb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "1 if AES 128 included"]
    #[must_use]
    #[inline(always)]
    pub const fn aes(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "1 if AES 128 included"]
    #[inline(always)]
    pub const fn set_aes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "1 if AES 192 and 256 also included"]
    #[must_use]
    #[inline(always)]
    pub const fn aeskey(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "1 if AES 192 and 256 also included"]
    #[inline(always)]
    pub const fn set_aeskey(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "1 if AES Secret key available"]
    #[must_use]
    #[inline(always)]
    pub const fn secret(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1 if AES Secret key available"]
    #[inline(always)]
    pub const fn set_secret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "1 if ICB over AES included"]
    #[must_use]
    #[inline(always)]
    pub const fn icb(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "1 if ICB over AES included"]
    #[inline(always)]
    pub const fn set_icb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
impl core::fmt::Debug for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Config")
            .field("dual", &self.dual())
            .field("dma", &self.dma())
            .field("ahb", &self.ahb())
            .field("aes", &self.aes())
            .field("aeskey", &self.aeskey())
            .field("secret", &self.secret())
            .field("icb", &self.icb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Config {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Config {{ dual: {=bool:?}, dma: {=bool:?}, ahb: {=bool:?}, aes: {=bool:?}, aeskey: {=bool:?}, secret: {=bool:?}, icb: {=bool:?} }}",
            self.dual(),
            self.dma(),
            self.ahb(),
            self.aes(),
            self.aeskey(),
            self.secret(),
            self.icb()
        )
    }
}
#[doc = "Crypto settings for AES and Salsa and ChaCha"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cryptcfg(pub u32);
impl Cryptcfg {
    #[doc = "If 1, OUTDATA0 will be read Most significant word 1st for AES. Else it will be read in normal little endian - Least significant word 1st. Note: only if allowed by configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn msw1st_out(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, OUTDATA0 will be read Most significant word 1st for AES. Else it will be read in normal little endian - Least significant word 1st. Note: only if allowed by configuration."]
    #[inline(always)]
    pub const fn set_msw1st_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, will Swap the key input (bytes in each word)."]
    #[must_use]
    #[inline(always)]
    pub const fn swapkey(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, will Swap the key input (bytes in each word)."]
    #[inline(always)]
    pub const fn set_swapkey(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, will SWAP the data and IV inputs (bytes in each word)."]
    #[must_use]
    #[inline(always)]
    pub const fn swapdat(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, will SWAP the data and IV inputs (bytes in each word)."]
    #[inline(always)]
    pub const fn set_swapdat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, load of key, IV, and data is MSW 1st for AES. Else, the words are little endian. Note: only if allowed by configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn msw1st(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, load of key, IV, and data is MSW 1st for AES. Else, the words are little endian. Note: only if allowed by configuration."]
    #[inline(always)]
    pub const fn set_msw1st(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "AES Cipher mode to use if plain AES"]
    #[must_use]
    #[inline(always)]
    pub const fn aesmode(&self) -> super::vals::Aesmode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Aesmode::from_bits(val as u8)
    }
    #[doc = "AES Cipher mode to use if plain AES"]
    #[inline(always)]
    pub const fn set_aesmode(&mut self, val: super::vals::Aesmode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "AES ECB direction. Only encryption used if CTR mode or manual modes such as CFB"]
    #[must_use]
    #[inline(always)]
    pub const fn aesdecrypt(&self) -> super::vals::Aesdecrypt {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Aesdecrypt::from_bits(val as u8)
    }
    #[doc = "AES ECB direction. Only encryption used if CTR mode or manual modes such as CFB"]
    #[inline(always)]
    pub const fn set_aesdecrypt(&mut self, val: super::vals::Aesdecrypt) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Selects the Hidden Secret key vs. User key, if provided. If security levels are used, only the highest level is permitted to select this."]
    #[must_use]
    #[inline(always)]
    pub const fn aessecret(&self) -> super::vals::Aessecret {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Aessecret::from_bits(val as u8)
    }
    #[doc = "Selects the Hidden Secret key vs. User key, if provided. If security levels are used, only the highest level is permitted to select this."]
    #[inline(always)]
    pub const fn set_aessecret(&mut self, val: super::vals::Aessecret) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sets the AES key size"]
    #[must_use]
    #[inline(always)]
    pub const fn aeskeysz(&self) -> super::vals::Aeskeysz {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Aeskeysz::from_bits(val as u8)
    }
    #[doc = "Sets the AES key size"]
    #[inline(always)]
    pub const fn set_aeskeysz(&mut self, val: super::vals::Aeskeysz) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Halfword position of 16b counter in IV if AESMODE is CTR (position is fixed for Salsa and ChaCha). Only supports 16b counter, so application must control any additional bytes if using more. The 16-bit counter is read from the IV and incremented by 1 each time. Any other use CTR should use ECB directly and do its own XOR and so on."]
    #[must_use]
    #[inline(always)]
    pub const fn aesctrpos(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Halfword position of 16b counter in IV if AESMODE is CTR (position is fixed for Salsa and ChaCha). Only supports 16b counter, so application must control any additional bytes if using more. The 16-bit counter is read from the IV and incremented by 1 each time. Any other use CTR should use ECB directly and do its own XOR and so on."]
    #[inline(always)]
    pub const fn set_aesctrpos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "Is 1 if last stream block. If not 1, then the engine will compute the next \"hash\"."]
    #[must_use]
    #[inline(always)]
    pub const fn streamlast(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Is 1 if last stream block. If not 1, then the engine will compute the next \"hash\"."]
    #[inline(always)]
    pub const fn set_streamlast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This sets the ICB size between 32 and 128 bits, using the following rules. Note that the counter is assumed to occupy the low order bits of the IV."]
    #[must_use]
    #[inline(always)]
    pub const fn icbsz(&self) -> super::vals::Icbsz {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Icbsz::from_bits(val as u8)
    }
    #[doc = "This sets the ICB size between 32 and 128 bits, using the following rules. Note that the counter is assumed to occupy the low order bits of the IV."]
    #[inline(always)]
    pub const fn set_icbsz(&mut self, val: super::vals::Icbsz) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "The size of the ICB-AES stream that can be pushed before needing to compute a new IV/ctr (counter start). This optimizes the performance of the stream of blocks after the 1st."]
    #[must_use]
    #[inline(always)]
    pub const fn icbstrm(&self) -> super::vals::Icbstrm {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Icbstrm::from_bits(val as u8)
    }
    #[doc = "The size of the ICB-AES stream that can be pushed before needing to compute a new IV/ctr (counter start). This optimizes the performance of the stream of blocks after the 1st."]
    #[inline(always)]
    pub const fn set_icbstrm(&mut self, val: super::vals::Icbstrm) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
}
impl Default for Cryptcfg {
    #[inline(always)]
    fn default() -> Cryptcfg {
        Cryptcfg(0)
    }
}
impl core::fmt::Debug for Cryptcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cryptcfg")
            .field("msw1st_out", &self.msw1st_out())
            .field("swapkey", &self.swapkey())
            .field("swapdat", &self.swapdat())
            .field("msw1st", &self.msw1st())
            .field("aesmode", &self.aesmode())
            .field("aesdecrypt", &self.aesdecrypt())
            .field("aessecret", &self.aessecret())
            .field("aeskeysz", &self.aeskeysz())
            .field("aesctrpos", &self.aesctrpos())
            .field("streamlast", &self.streamlast())
            .field("icbsz", &self.icbsz())
            .field("icbstrm", &self.icbstrm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cryptcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cryptcfg {{ msw1st_out: {=bool:?}, swapkey: {=bool:?}, swapdat: {=bool:?}, msw1st: {=bool:?}, aesmode: {:?}, aesdecrypt: {:?}, aessecret: {:?}, aeskeysz: {:?}, aesctrpos: {=u8:?}, streamlast: {=bool:?}, icbsz: {:?}, icbstrm: {:?} }}",
            self.msw1st_out(),
            self.swapkey(),
            self.swapdat(),
            self.msw1st(),
            self.aesmode(),
            self.aesdecrypt(),
            self.aessecret(),
            self.aeskeysz(),
            self.aesctrpos(),
            self.streamlast(),
            self.icbsz(),
            self.icbstrm()
        )
    }
}
#[doc = "Control register to enable and operate Hash and Crypto"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Written with 1 when starting a new Hash/Crypto. It self clears. Note that the WAITING Status bit will clear for a cycle during the initialization from New=1."]
    #[must_use]
    #[inline(always)]
    pub const fn new_hash(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Written with 1 when starting a new Hash/Crypto. It self clears. Note that the WAITING Status bit will clear for a cycle during the initialization from New=1."]
    #[inline(always)]
    pub const fn set_new_hash(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed)."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_i(&self) -> super::vals::DmaI {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::DmaI::from_bits(val as u8)
    }
    #[doc = "Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed)."]
    #[inline(always)]
    pub const fn set_dma_i(&mut self, val: super::vals::DmaI) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_o(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses."]
    #[inline(always)]
    pub const fn set_dma_o(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "If 1, will swap bytes in the word for SHA hashing. The default is byte order (so LSB is 1st byte) but this allows swapping to MSB is 1st such as is shown in SHS spec. For cryptographic swapping, see the CRYPTCFG register."]
    #[must_use]
    #[inline(always)]
    pub const fn hashswpb(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, will swap bytes in the word for SHA hashing. The default is byte order (so LSB is 1st byte) but this allows swapping to MSB is 1st such as is shown in SHS spec. For cryptographic swapping, see the CRYPTCFG register."]
    #[inline(always)]
    pub const fn set_hashswpb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
            .field("mode", &self.mode())
            .field("new_hash", &self.new_hash())
            .field("dma_i", &self.dma_i())
            .field("dma_o", &self.dma_o())
            .field("hashswpb", &self.hashswpb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ mode: {:?}, new_hash: {=bool:?}, dma_i: {:?}, dma_o: {=bool:?}, hashswpb: {=bool:?} }}",
            self.mode(),
            self.new_hash(),
            self.dma_i(),
            self.dma_o(),
            self.hashswpb()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Digest0(pub u32);
impl Digest0 {
    #[doc = "One word of the Digest or output. Note that only 1st 4 are populated for AES and 1st 5 are populated for SHA1."]
    #[must_use]
    #[inline(always)]
    pub const fn digest(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "One word of the Digest or output. Note that only 1st 4 are populated for AES and 1st 5 are populated for SHA1."]
    #[inline(always)]
    pub const fn set_digest(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Digest0 {
    #[inline(always)]
    fn default() -> Digest0 {
        Digest0(0)
    }
}
impl core::fmt::Debug for Digest0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Digest0")
            .field("digest", &self.digest())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Digest0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Digest0 {{ digest: {=u32:?} }}", self.digest())
    }
}
#[doc = "Input of 16 words at a time to load up buffer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Indata(pub u32);
impl Indata {
    #[doc = "Write next word in little-endian form. The hash requires big endian word data, but this block swaps the bytes automatically. That is, SHA assumes the data coming in is treated as bytes (e.g. \"abcd\") and since the ARM core will treat \"abcd\" as a word as 0x64636261, the block will swap the word to restore into big endian."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Write next word in little-endian form. The hash requires big endian word data, but this block swaps the bytes automatically. That is, SHA assumes the data coming in is treated as bytes (e.g. \"abcd\") and since the ARM core will treat \"abcd\" as a word as 0x64636261, the block will swap the word to restore into big endian."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Indata {
    #[inline(always)]
    fn default() -> Indata {
        Indata(0)
    }
}
impl core::fmt::Debug for Indata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Indata")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Indata {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Indata {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Write 1 to clear interrupts."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenclr(pub u32);
impl Intenclr {
    #[doc = "Write 1 to clear mask."]
    #[must_use]
    #[inline(always)]
    pub const fn waiting(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear mask."]
    #[inline(always)]
    pub const fn set_waiting(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write 1 to clear mask."]
    #[must_use]
    #[inline(always)]
    pub const fn digest(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear mask."]
    #[inline(always)]
    pub const fn set_digest(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write 1 to clear mask."]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear mask."]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
            .field("waiting", &self.waiting())
            .field("digest", &self.digest())
            .field("error", &self.error())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intenclr {{ waiting: {=bool:?}, digest: {=bool:?}, error: {=bool:?} }}",
            self.waiting(),
            self.digest(),
            self.error()
        )
    }
}
#[doc = "Write 1 to enable interrupts; reads back with which are set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenset(pub u32);
impl Intenset {
    #[doc = "Indicates if should interrupt when waiting for data input."]
    #[must_use]
    #[inline(always)]
    pub const fn waiting(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if should interrupt when waiting for data input."]
    #[inline(always)]
    pub const fn set_waiting(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates if should interrupt when Digest (or Outdata) is ready (completed a hash/crypto or completed a full sequence)."]
    #[must_use]
    #[inline(always)]
    pub const fn digest(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if should interrupt when Digest (or Outdata) is ready (completed a hash/crypto or completed a full sequence)."]
    #[inline(always)]
    pub const fn set_digest(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates if should interrupt on an ERROR (as defined in Status)"]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if should interrupt on an ERROR (as defined in Status)"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
            .field("waiting", &self.waiting())
            .field("digest", &self.digest())
            .field("error", &self.error())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intenset {{ waiting: {=bool:?}, digest: {=bool:?}, error: {=bool:?} }}",
            self.waiting(),
            self.digest(),
            self.error()
        )
    }
}
#[doc = "Lock register allows locking to the current security level or unlocking by the lock holding level."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc = "Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level."]
    #[must_use]
    #[inline(always)]
    pub const fn seclock(&self) -> super::vals::Seclock {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Seclock::from_bits(val as u8)
    }
    #[doc = "Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level."]
    #[inline(always)]
    pub const fn set_seclock(&mut self, val: super::vals::Seclock) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Must write 0xA75 to change lock state. A75:Pattern needed to change bits 1:0"]
    #[must_use]
    #[inline(always)]
    pub const fn pattern(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "Must write 0xA75 to change lock state. A75:Pattern needed to change bits 1:0"]
    #[inline(always)]
    pub const fn set_pattern(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
}
impl Default for Lock {
    #[inline(always)]
    fn default() -> Lock {
        Lock(0)
    }
}
impl core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lock")
            .field("seclock", &self.seclock())
            .field("pattern", &self.pattern())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lock {{ seclock: {:?}, pattern: {=u16:?} }}",
            self.seclock(),
            self.pattern()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask(pub u32);
impl Mask {
    #[doc = "A random word."]
    #[must_use]
    #[inline(always)]
    pub const fn mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "A random word."]
    #[inline(always)]
    pub const fn set_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mask {
    #[inline(always)]
    fn default() -> Mask {
        Mask(0)
    }
}
impl core::fmt::Debug for Mask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mask").field("mask", &self.mask()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mask {{ mask: {=u32:?} }}", self.mask())
    }
}
#[doc = "Address to start memory access from (if available)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memaddr(pub u32);
impl Memaddr {
    #[doc = "Address base to start copying from, word aligned (so bits 1:0 must be 0). This field will advance as it processes the words. If it fails with a bus error, the register will contain the failing word. N:Address in Flash or RAM space; RAM only as mapped in this part. May also be able to address SPIFI."]
    #[must_use]
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address base to start copying from, word aligned (so bits 1:0 must be 0). This field will advance as it processes the words. If it fails with a bus error, the register will contain the failing word. N:Address in Flash or RAM space; RAM only as mapped in this part. May also be able to address SPIFI."]
    #[inline(always)]
    pub const fn set_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Memaddr {
    #[inline(always)]
    fn default() -> Memaddr {
        Memaddr(0)
    }
}
impl core::fmt::Debug for Memaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Memaddr")
            .field("base", &self.base())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Memaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Memaddr {{ base: {=u32:?} }}", self.base())
    }
}
#[doc = "Setup Master to access memory (if available)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memctrl(pub u32);
impl Memctrl {
    #[doc = "Enables mastering."]
    #[must_use]
    #[inline(always)]
    pub const fn master(&self) -> super::vals::Master {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Master::from_bits(val as u8)
    }
    #[doc = "Enables mastering."]
    #[inline(always)]
    pub const fn set_master(&mut self, val: super::vals::Master) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Number of 512-bit (128-bit if AES, except 1st block which may include key and IV) blocks to copy starting at MEMADDR. This register will decrement after each block is copied, ending in 0. For Hash, the DIGEST interrupt will occur when it reaches 0. Fro AES, the DIGEST/OUTDATA interrupt will occur on ever block. If a bus error occurs, it will stop with this field set to the block that failed. 0:Done - nothing to process. 1 to 2K: Number of 512-bit (or 128bit) blocks to hash."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Number of 512-bit (128-bit if AES, except 1st block which may include key and IV) blocks to copy starting at MEMADDR. This register will decrement after each block is copied, ending in 0. For Hash, the DIGEST interrupt will occur when it reaches 0. Fro AES, the DIGEST/OUTDATA interrupt will occur on ever block. If a bus error occurs, it will stop with this field set to the block that failed. 0:Done - nothing to process. 1 to 2K: Number of 512-bit (or 128bit) blocks to hash."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for Memctrl {
    #[inline(always)]
    fn default() -> Memctrl {
        Memctrl(0)
    }
}
impl core::fmt::Debug for Memctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Memctrl")
            .field("master", &self.master())
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Memctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Memctrl {{ master: {:?}, count: {=u16:?} }}",
            self.master(),
            self.count()
        )
    }
}
#[doc = "Indicates status of Hash peripheral."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "If 1, the block is waiting for more data to process."]
    #[must_use]
    #[inline(always)]
    pub const fn waiting(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the block is waiting for more data to process."]
    #[inline(always)]
    pub const fn set_waiting(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "For Hash, if 1 then a DIGEST is ready and waiting and there is no active next block already started. For Cryptographic uses, this will be set for each block processed, indicating OUTDATA (and OUTDATA2 if larger output) contains the next value to read out. This is cleared when any data is written, when New is written, for Cryptographic uses when the last word is read out, or when the block is disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn digest(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "For Hash, if 1 then a DIGEST is ready and waiting and there is no active next block already started. For Cryptographic uses, this will be set for each block processed, indicating OUTDATA (and OUTDATA2 if larger output) contains the next value to read out. This is cleared when any data is written, when New is written, for Cryptographic uses when the last word is read out, or when the block is disabled."]
    #[inline(always)]
    pub const fn set_digest(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, an error occurred. For normal uses, this is due to an attempted overrun: INDATA was written when it was not appropriate. For Master cases, this is an AHB bus error; the COUNT field will indicate which block it was on."]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, an error occurred. For normal uses, this is due to an attempted overrun: INDATA was written when it was not appropriate. For Master cases, this is an AHB bus error; the COUNT field will indicate which block it was on."]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates the block wants the key to be written in (set along with WAITING)"]
    #[must_use]
    #[inline(always)]
    pub const fn needkey(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the block wants the key to be written in (set along with WAITING)"]
    #[inline(always)]
    pub const fn set_needkey(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates the block wants an IV/NONE to be written in (set along with WAITING)"]
    #[must_use]
    #[inline(always)]
    pub const fn neediv(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the block wants an IV/NONE to be written in (set along with WAITING)"]
    #[inline(always)]
    pub const fn set_neediv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If ICB-AES is selected, then reads as the ICB index count based on ICBSTRM (from CRYPTCFG). That is, if 3 bits of ICBSTRM, then this will count from 0 to 7 and then back to 0. On 0, it has to compute the full ICB, quicker when not 0."]
    #[must_use]
    #[inline(always)]
    pub const fn icbidx(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "If ICB-AES is selected, then reads as the ICB index count based on ICBSTRM (from CRYPTCFG). That is, if 3 bits of ICBSTRM, then this will count from 0 to 7 and then back to 0. On 0, it has to compute the full ICB, quicker when not 0."]
    #[inline(always)]
    pub const fn set_icbidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
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
            .field("waiting", &self.waiting())
            .field("digest", &self.digest())
            .field("error", &self.error())
            .field("needkey", &self.needkey())
            .field("neediv", &self.neediv())
            .field("icbidx", &self.icbidx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ waiting: {=bool:?}, digest: {=bool:?}, error: {=bool:?}, needkey: {=bool:?}, neediv: {=bool:?}, icbidx: {=u8:?} }}",
            self.waiting(),
            self.digest(),
            self.error(),
            self.needkey(),
            self.neediv(),
            self.icbidx()
        )
    }
}
