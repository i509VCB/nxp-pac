#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sgi {
    ptr: *mut u8,
}
unsafe impl Send for Sgi {}
unsafe impl Sync for Sgi {}
impl Sgi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Input Data register 0 - Word-3"]
    #[inline(always)]
    pub const fn sgi_datin0a(self) -> crate::common::Reg<regs::SgiDatin0a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Input Data register 0 - Word-2"]
    #[inline(always)]
    pub const fn sgi_datin0b(self) -> crate::common::Reg<regs::SgiDatin0b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Input Data register 0 - Word-1"]
    #[inline(always)]
    pub const fn sgi_datin0c(self) -> crate::common::Reg<regs::SgiDatin0c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Input Data register 0 - Word-0"]
    #[inline(always)]
    pub const fn sgi_datin0d(self) -> crate::common::Reg<regs::SgiDatin0d, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Input Data register 1 - Word-3"]
    #[inline(always)]
    pub const fn sgi_datin1a(self) -> crate::common::Reg<regs::SgiDatin1a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Input Data register 1 - Word-2"]
    #[inline(always)]
    pub const fn sgi_datin1b(self) -> crate::common::Reg<regs::SgiDatin1b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Input Data register 1 - Word-1"]
    #[inline(always)]
    pub const fn sgi_datin1c(self) -> crate::common::Reg<regs::SgiDatin1c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Input Data register 1 - Word-0"]
    #[inline(always)]
    pub const fn sgi_datin1d(self) -> crate::common::Reg<regs::SgiDatin1d, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "Input Data register 2 - Word-3"]
    #[inline(always)]
    pub const fn sgi_datin2a(self) -> crate::common::Reg<regs::SgiDatin2a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Input Data register 2 - Word-2"]
    #[inline(always)]
    pub const fn sgi_datin2b(self) -> crate::common::Reg<regs::SgiDatin2b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Input Data register 2 - Word-1"]
    #[inline(always)]
    pub const fn sgi_datin2c(self) -> crate::common::Reg<regs::SgiDatin2c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Input Data register 2 - Word-0"]
    #[inline(always)]
    pub const fn sgi_datin2d(self) -> crate::common::Reg<regs::SgiDatin2d, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "Input Data register 3 - Word-3"]
    #[inline(always)]
    pub const fn sgi_datin3a(self) -> crate::common::Reg<regs::SgiDatin3a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "Input Data register 3 - Word-2"]
    #[inline(always)]
    pub const fn sgi_datin3b(self) -> crate::common::Reg<regs::SgiDatin3b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "Input Data register 3 - Word-1"]
    #[inline(always)]
    pub const fn sgi_datin3c(self) -> crate::common::Reg<regs::SgiDatin3c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Input Data register 3 - Word-0"]
    #[inline(always)]
    pub const fn sgi_datin3d(self) -> crate::common::Reg<regs::SgiDatin3d, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "Input Key register 0 - Word-3"]
    #[inline(always)]
    pub const fn sgi_key0a(self) -> crate::common::Reg<regs::SgiKey0a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "Input Key register 0 - Word-2"]
    #[inline(always)]
    pub const fn sgi_key0b(self) -> crate::common::Reg<regs::SgiKey0b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "Input Key register 0 - Word-1"]
    #[inline(always)]
    pub const fn sgi_key0c(self) -> crate::common::Reg<regs::SgiKey0c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "Input Key register 0 - Word-0"]
    #[inline(always)]
    pub const fn sgi_key0d(self) -> crate::common::Reg<regs::SgiKey0d, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "Input Key register 1 - Word-3"]
    #[inline(always)]
    pub const fn sgi_key1a(self) -> crate::common::Reg<regs::SgiKey1a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "Input Key register 1 - Word-2"]
    #[inline(always)]
    pub const fn sgi_key1b(self) -> crate::common::Reg<regs::SgiKey1b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "Input Key register 1 - Word-1"]
    #[inline(always)]
    pub const fn sgi_key1c(self) -> crate::common::Reg<regs::SgiKey1c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "Input Key register 1 - Word-0"]
    #[inline(always)]
    pub const fn sgi_key1d(self) -> crate::common::Reg<regs::SgiKey1d, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "Input Key register 2 - Word-3"]
    #[inline(always)]
    pub const fn sgi_key2a(self) -> crate::common::Reg<regs::SgiKey2a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "Input Key register 2 - Word-2"]
    #[inline(always)]
    pub const fn sgi_key2b(self) -> crate::common::Reg<regs::SgiKey2b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Input Key register 2 - Word-1"]
    #[inline(always)]
    pub const fn sgi_key2c(self) -> crate::common::Reg<regs::SgiKey2c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "Input Key register 2 - Word-0"]
    #[inline(always)]
    pub const fn sgi_key2d(self) -> crate::common::Reg<regs::SgiKey2d, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "Input Key register 3 - Word-3"]
    #[inline(always)]
    pub const fn sgi_key3a(self) -> crate::common::Reg<regs::SgiKey3a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "Input Key register 3 - Word-2"]
    #[inline(always)]
    pub const fn sgi_key3b(self) -> crate::common::Reg<regs::SgiKey3b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "Input Key register 3 - Word-1"]
    #[inline(always)]
    pub const fn sgi_key3c(self) -> crate::common::Reg<regs::SgiKey3c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "Input Key register 3 - Word-0"]
    #[inline(always)]
    pub const fn sgi_key3d(self) -> crate::common::Reg<regs::SgiKey3d, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize) as _) }
    }
    #[doc = "Input Key register 4 - Word-3"]
    #[inline(always)]
    pub const fn sgi_key4a(self) -> crate::common::Reg<regs::SgiKey4a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "Input Key register 4 - Word-2"]
    #[inline(always)]
    pub const fn sgi_key4b(self) -> crate::common::Reg<regs::SgiKey4b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "Input Key register 4 - Word-1"]
    #[inline(always)]
    pub const fn sgi_key4c(self) -> crate::common::Reg<regs::SgiKey4c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "Input Key register 4 - Word-0"]
    #[inline(always)]
    pub const fn sgi_key4d(self) -> crate::common::Reg<regs::SgiKey4d, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[doc = "Input Key register 5 - Word-3"]
    #[inline(always)]
    pub const fn sgi_key5a(self) -> crate::common::Reg<regs::SgiKey5a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "Input Key register 5 - Word-2"]
    #[inline(always)]
    pub const fn sgi_key5b(self) -> crate::common::Reg<regs::SgiKey5b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "Input Key register 5 - Word-1"]
    #[inline(always)]
    pub const fn sgi_key5c(self) -> crate::common::Reg<regs::SgiKey5c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "Input Key register 5 - Word-0"]
    #[inline(always)]
    pub const fn sgi_key5d(self) -> crate::common::Reg<regs::SgiKey5d, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x029cusize) as _) }
    }
    #[doc = "Input Key register 6 - Word-3"]
    #[inline(always)]
    pub const fn sgi_key6a(self) -> crate::common::Reg<regs::SgiKey6a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "Input Key register 6 - Word-2"]
    #[inline(always)]
    pub const fn sgi_key6b(self) -> crate::common::Reg<regs::SgiKey6b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "Input Key register 6 - Word-1"]
    #[inline(always)]
    pub const fn sgi_key6c(self) -> crate::common::Reg<regs::SgiKey6c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "Input Key register 6 - Word-0"]
    #[inline(always)]
    pub const fn sgi_key6d(self) -> crate::common::Reg<regs::SgiKey6d, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
    #[doc = "Input Key register 7 - Word-3"]
    #[inline(always)]
    pub const fn sgi_key7a(self) -> crate::common::Reg<regs::SgiKey7a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "Input Key register 7 - Word-2"]
    #[inline(always)]
    pub const fn sgi_key7b(self) -> crate::common::Reg<regs::SgiKey7b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "Input Key register 7 - Word-1"]
    #[inline(always)]
    pub const fn sgi_key7c(self) -> crate::common::Reg<regs::SgiKey7c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "Input Key register 7 - Word-0"]
    #[inline(always)]
    pub const fn sgi_key7d(self) -> crate::common::Reg<regs::SgiKey7d, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "Output Data register - Word-3"]
    #[inline(always)]
    pub const fn sgi_datouta(self) -> crate::common::Reg<regs::SgiDatouta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "Output Data register - Word-2"]
    #[inline(always)]
    pub const fn sgi_datoutb(self) -> crate::common::Reg<regs::SgiDatoutb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "Output Data register - Word-1"]
    #[inline(always)]
    pub const fn sgi_datoutc(self) -> crate::common::Reg<regs::SgiDatoutc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[doc = "Output Data register - Word-0"]
    #[inline(always)]
    pub const fn sgi_datoutd(self) -> crate::common::Reg<regs::SgiDatoutd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sgi_status(self) -> crate::common::Reg<regs::SgiStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[doc = "Calculation counter"]
    #[inline(always)]
    pub const fn sgi_count(self) -> crate::common::Reg<regs::SgiCount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c04usize) as _) }
    }
    #[doc = "Key checksum register"]
    #[inline(always)]
    pub const fn sgi_keychk(self) -> crate::common::Reg<regs::SgiKeychk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c08usize) as _) }
    }
    #[doc = "SGI Control register"]
    #[inline(always)]
    pub const fn sgi_ctrl(self) -> crate::common::Reg<regs::SgiCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d00usize) as _) }
    }
    #[doc = "SGI Control register 2"]
    #[inline(always)]
    pub const fn sgi_ctrl2(self) -> crate::common::Reg<regs::SgiCtrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d04usize) as _) }
    }
    #[doc = "Configuration of dummy controls"]
    #[inline(always)]
    pub const fn sgi_dummy_ctrl(self) -> crate::common::Reg<regs::SgiDummyCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d08usize) as _) }
    }
    #[doc = "Sofware Assisted Masking register ."]
    #[inline(always)]
    pub const fn sgi_sfr_sw_mask(
        self,
    ) -> crate::common::Reg<regs::SgiSfrSwMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d0cusize) as _) }
    }
    #[doc = "SFRSEED register for SFRMASK feature."]
    #[inline(always)]
    pub const fn sgi_sfrseed(self) -> crate::common::Reg<regs::SgiSfrseed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d10usize) as _) }
    }
    #[doc = "SHA Control Register"]
    #[inline(always)]
    pub const fn sgi_sha2_ctrl(self) -> crate::common::Reg<regs::SgiSha2Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d14usize) as _) }
    }
    #[doc = "SHA FIFO lower-bank low"]
    #[inline(always)]
    pub const fn sgi_sha_fifo(self) -> crate::common::Reg<regs::SgiShaFifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d18usize) as _) }
    }
    #[doc = "SHA Configuration Reg"]
    #[inline(always)]
    pub const fn sgi_config(self) -> crate::common::Reg<regs::SgiConfig, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d1cusize) as _) }
    }
    #[doc = "SHA Configuration 2 Reg"]
    #[inline(always)]
    pub const fn sgi_config2(self) -> crate::common::Reg<regs::SgiConfig2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d20usize) as _) }
    }
    #[doc = "SGI Auto Mode Control register"]
    #[inline(always)]
    pub const fn sgi_auto_mode(self) -> crate::common::Reg<regs::SgiAutoMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d24usize) as _) }
    }
    #[doc = "SGI Auto Mode Control register"]
    #[inline(always)]
    pub const fn sgi_auto_dma_ctrl(
        self,
    ) -> crate::common::Reg<regs::SgiAutoDmaCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d28usize) as _) }
    }
    #[doc = "SGI internal PRNG SW seeding register"]
    #[inline(always)]
    pub const fn sgi_prng_sw_seed(
        self,
    ) -> crate::common::Reg<regs::SgiPrngSwSeed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d30usize) as _) }
    }
    #[doc = "SGI Key Control SFR"]
    #[inline(always)]
    pub const fn sgi_key_ctrl(self) -> crate::common::Reg<regs::SgiKeyCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d40usize) as _) }
    }
    #[doc = "Wrapped key read SFR"]
    #[inline(always)]
    pub const fn sgi_key_wrap(self) -> crate::common::Reg<regs::SgiKeyWrap, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d50usize) as _) }
    }
    #[doc = "SGI Version"]
    #[inline(always)]
    pub const fn sgi_version(self) -> crate::common::Reg<regs::SgiVersion, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f08usize) as _) }
    }
    #[doc = "Access Error"]
    #[inline(always)]
    pub const fn sgi_access_err(self) -> crate::common::Reg<regs::SgiAccessErr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc0usize) as _) }
    }
    #[doc = "Clear Access Error"]
    #[inline(always)]
    pub const fn sgi_access_err_clr(
        self,
    ) -> crate::common::Reg<regs::SgiAccessErrClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc4usize) as _) }
    }
    #[doc = "Interrupt status"]
    #[inline(always)]
    pub const fn sgi_int_status(self) -> crate::common::Reg<regs::SgiIntStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe0usize) as _) }
    }
    #[doc = "Interrupt enable"]
    #[inline(always)]
    pub const fn sgi_int_enable(self) -> crate::common::Reg<regs::SgiIntEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe4usize) as _) }
    }
    #[doc = "Interrupt status clear"]
    #[inline(always)]
    pub const fn sgi_int_status_clr(
        self,
    ) -> crate::common::Reg<regs::SgiIntStatusClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe8usize) as _) }
    }
    #[doc = "Interrupt status set"]
    #[inline(always)]
    pub const fn sgi_int_status_set(
        self,
    ) -> crate::common::Reg<regs::SgiIntStatusSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "Module ID"]
    #[inline(always)]
    pub const fn sgi_module_id(self) -> crate::common::Reg<regs::SgiModuleId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
