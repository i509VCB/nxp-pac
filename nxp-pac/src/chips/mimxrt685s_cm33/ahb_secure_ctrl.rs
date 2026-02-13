#[doc = "LPC_Next0 AHB secure controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSecureCtrl {
    ptr: *mut u8,
}
unsafe impl Send for AhbSecureCtrl {}
unsafe impl Sync for AhbSecureCtrl {}
impl AhbSecureCtrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Memory ROM Rule(n) Register"]
    #[inline(always)]
    pub const fn rom_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RomMemRule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "FLEXSPI0 Region 0 Rule(n) Register"]
    #[inline(always)]
    pub const fn flexspi0_region0_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Flexspi0Region0Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize + n * 4usize) as _) }
    }
    #[doc = "FLEXSPI0 Region 1 Rule 0 Register"]
    #[inline(always)]
    pub const fn flexspi0_region1_rule0(
        self,
    ) -> crate::common::Reg<regs::Flexspi0Region1Rule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "FLEXSPI0 Region 2 Rule 0 Register"]
    #[inline(always)]
    pub const fn flexspi0_region2_rule0(
        self,
    ) -> crate::common::Reg<regs::Flexspi0Region2Rule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "FLEXSPI0 Region 3 Rule 0 Register"]
    #[inline(always)]
    pub const fn flexspi0_region3_rule0(
        self,
    ) -> crate::common::Reg<regs::Flexspi0Region3Rule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "FLEXSPI0 Region 4 Rule 0 Register"]
    #[inline(always)]
    pub const fn flexspi0_region4_rule0(
        self,
    ) -> crate::common::Reg<regs::Flexspi0Region4Rule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "SRAM Partition 00 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram00_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram00Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize + n * 4usize) as _) }
    }
    #[doc = "SRAM Partition 01 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram01_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram01Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "SRAM Partition 02 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram02_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram02Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _) }
    }
    #[doc = "SRAM Partition 03 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram03_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram03Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize + n * 4usize) as _) }
    }
    #[doc = "SRAM Partition 04 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram04_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram04Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize + n * 4usize) as _) }
    }
    #[doc = "SRAM Partition 05 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram05_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram05Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 06 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram06_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram06Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 07 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram07_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram07Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 08 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram08_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram08Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 09 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram09_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram09Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 10 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram10_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram10Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 11 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram11_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram11Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 12 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram12_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram12Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 13 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram13_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram13Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 14 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram14_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram14Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 15 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram15_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram15Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 16 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram16_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram16Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 17 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram17_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram17Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 18 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram18_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram18Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 19 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram19_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram19Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 20 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram20_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram20Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 21 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram21_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram21Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 22 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram22_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram22Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 23 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram23_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram23Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 24 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram24_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram24Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 25 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram25_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram25Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 26 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram26_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram26Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 27 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram27_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram27Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 28 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram28_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram28Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Partition 29 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram29_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ram29Rule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize + n * 4usize) as _)
        }
    }
    #[doc = "Security access rules for HiFi 4 memory sectors (0x24000000--0x240FFFFF). Each sector is 32 Kbytes, there're 4 sectors in total."]
    #[inline(always)]
    pub const fn pif_hifi4_x_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::PifHifi4XMemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
    #[inline(always)]
    pub const fn apb_grp0_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::ApbGrp0MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
    #[inline(always)]
    pub const fn apb_grp0_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::ApbGrp0MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0344usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
    #[inline(always)]
    pub const fn apb_grp1_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::ApbGrp1MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0350usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
    #[inline(always)]
    pub const fn apb_grp1_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::ApbGrp1MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0354usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
    #[inline(always)]
    pub const fn apb_grp1_mem_rule2(
        self,
    ) -> crate::common::Reg<regs::ApbGrp1MemRule2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0358usize) as _) }
    }
    #[doc = "Security access rules for AHB peripheral slaves area 0x40100000--0x4010FFFF"]
    #[inline(always)]
    pub const fn ahb_periph0_slave_rule0(
        self,
    ) -> crate::common::Reg<regs::AhbPeriph0SlaveRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "0x40110000--0x4011FFFF"]
    #[inline(always)]
    pub const fn aips_bridge0_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::AipsBridge0MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[doc = "the memory map is 0x40120000--0x40127FFF"]
    #[inline(always)]
    pub const fn ahb_periph1_slave_rule0(
        self,
    ) -> crate::common::Reg<regs::AhbPeriph1SlaveRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "Security access rules for AIPS Bridge peripherals. Each AIPS bridge sector is 4 Kbytes, there're 16 sectors in total."]
    #[inline(always)]
    pub const fn aips_bridge1_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::AipsBridge1MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
    #[doc = "Security access rules for AIPS Bridge peripherals. Each AIPS bridge sector is 4 Kbytes, there're 16 sectors in total."]
    #[inline(always)]
    pub const fn aips_bridge1_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::AipsBridge1MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a4usize) as _) }
    }
    #[doc = "Security access rules for AHB peripheral slaves area 0x40140000--0x4014BFFF"]
    #[inline(always)]
    pub const fn ahb_periph2_slave_rule0(
        self,
    ) -> crate::common::Reg<regs::AhbPeriph2SlaveRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b0usize) as _) }
    }
    #[doc = "0x40148000--0x4014BFFF"]
    #[inline(always)]
    pub const fn security_ctrl_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::SecurityCtrlMemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c0usize) as _) }
    }
    #[doc = "Security access rules for AHB peripheral slaves area 0x40150000--0x40158FFF"]
    #[inline(always)]
    pub const fn ahb_periph3_slave_rule0(
        self,
    ) -> crate::common::Reg<regs::AhbPeriph3SlaveRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d0usize) as _) }
    }
    #[doc = "most recent security violation address for AHB layer n"]
    #[inline(always)]
    pub const fn sec_vio_addr(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SecVioAddr, crate::common::R> {
        assert!(n < 18usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e00usize + n * 4usize) as _)
        }
    }
    #[doc = "most recent security violation miscellaneous information for AHB layer n"]
    #[inline(always)]
    pub const fn sec_vio_misc_info(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SecVioMiscInfo, crate::common::R> {
        assert!(n < 18usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e80usize + n * 4usize) as _)
        }
    }
    #[doc = "security violation address/information registers valid flags"]
    #[inline(always)]
    pub const fn sec_vio_info_valid(
        self,
    ) -> crate::common::Reg<regs::SecVioInfoValid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f00usize) as _) }
    }
    #[doc = "Secure GPIO mask for port 0 pins. This register is used to block leakage of Secure interface (GPIOs, I2C, UART configured as secure peripherals) pin states to non-secure world."]
    #[inline(always)]
    pub const fn sec_gpio_mask0(self) -> crate::common::Reg<regs::SecGpioMask0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f80usize) as _) }
    }
    #[doc = "Secure GPIO mask for port 1 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask1(self) -> crate::common::Reg<regs::SecGpioMask1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f84usize) as _) }
    }
    #[doc = "Secure GPIO mask for port 2 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask2(self) -> crate::common::Reg<regs::SecGpioMask2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f88usize) as _) }
    }
    #[doc = "Secure GPIO mask for port 3 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask3(self) -> crate::common::Reg<regs::SecGpioMask3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f8cusize) as _) }
    }
    #[doc = "Secure GPIO mask for port 4 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask4(self) -> crate::common::Reg<regs::SecGpioMask4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f90usize) as _) }
    }
    #[doc = "Secure GPIO mask for port 5 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask5(self) -> crate::common::Reg<regs::SecGpioMask5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f94usize) as _) }
    }
    #[doc = "Secure GPIO mask for port 6 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask6(self) -> crate::common::Reg<regs::SecGpioMask6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f98usize) as _) }
    }
    #[doc = "Secure GPIO mask for port 7 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask7(self) -> crate::common::Reg<regs::SecGpioMask7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f9cusize) as _) }
    }
    #[doc = "secure general purpose register 8 used to mask interrupts to DSP for security purpose"]
    #[inline(always)]
    pub const fn sec_dsp_int_mask(
        self,
    ) -> crate::common::Reg<regs::SecDspIntMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa0usize) as _) }
    }
    #[doc = "sec_gp_reg write-lock bits"]
    #[inline(always)]
    pub const fn sec_mask_lock(self) -> crate::common::Reg<regs::SecMaskLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fbcusize) as _) }
    }
    #[doc = "master secure level register"]
    #[inline(always)]
    pub const fn master_sec_level(
        self,
    ) -> crate::common::Reg<regs::MasterSecLevel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd0usize) as _) }
    }
    #[doc = "master secure level anti-pole register"]
    #[inline(always)]
    pub const fn master_sec_level_anti_pol(
        self,
    ) -> crate::common::Reg<regs::MasterSecLevelAntiPol, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd4usize) as _) }
    }
    #[doc = "m33 lock control register"]
    #[inline(always)]
    pub const fn cm33_lock_reg(self) -> crate::common::Reg<regs::Cm33LockReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "secure control duplicate register"]
    #[inline(always)]
    pub const fn misc_ctrl_dp_reg(
        self,
    ) -> crate::common::Reg<regs::MiscCtrlDpReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "secure control register"]
    #[inline(always)]
    pub const fn misc_ctrl_reg(self) -> crate::common::Reg<regs::MiscCtrlReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
