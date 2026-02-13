#[doc = "Tempsensor Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense0(pub u32);
impl Tempsense0 {
    #[doc = "This bit powers down the temperature sensor."]
    #[must_use]
    #[inline(always)]
    pub const fn power_down(&self) -> super::vals::Tempsense0PowerDown {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tempsense0PowerDown::from_bits(val as u8)
    }
    #[doc = "This bit powers down the temperature sensor."]
    #[inline(always)]
    pub const fn set_power_down(&mut self, val: super::vals::Tempsense0PowerDown) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Starts the measurement process"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_temp(&self) -> super::vals::Tempsense0MeasureTemp {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tempsense0MeasureTemp::from_bits(val as u8)
    }
    #[doc = "Starts the measurement process"]
    #[inline(always)]
    pub const fn set_measure_temp(&mut self, val: super::vals::Tempsense0MeasureTemp) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates that the latest temp is valid"]
    #[must_use]
    #[inline(always)]
    pub const fn finished(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the latest temp is valid"]
    #[inline(always)]
    pub const fn set_finished(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit field contains the last measured temperature count."]
    #[must_use]
    #[inline(always)]
    pub const fn temp_cnt(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the last measured temperature count."]
    #[inline(always)]
    pub const fn set_temp_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[must_use]
    #[inline(always)]
    pub const fn alarm_value(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn set_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for Tempsense0 {
    #[inline(always)]
    fn default() -> Tempsense0 {
        Tempsense0(0)
    }
}
impl core::fmt::Debug for Tempsense0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense0")
            .field("power_down", &self.power_down())
            .field("measure_temp", &self.measure_temp())
            .field("finished", &self.finished())
            .field("temp_cnt", &self.temp_cnt())
            .field("alarm_value", &self.alarm_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense0 {{ power_down: {:?}, measure_temp: {:?}, finished: {=bool:?}, temp_cnt: {=u16:?}, alarm_value: {=u16:?} }}",
            self.power_down(),
            self.measure_temp(),
            self.finished(),
            self.temp_cnt(),
            self.alarm_value()
        )
    }
}
#[doc = "Tempsensor Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense0Clr(pub u32);
impl Tempsense0Clr {
    #[doc = "This bit powers down the temperature sensor."]
    #[must_use]
    #[inline(always)]
    pub const fn power_down(&self) -> super::vals::Tempsense0ClrPowerDown {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tempsense0ClrPowerDown::from_bits(val as u8)
    }
    #[doc = "This bit powers down the temperature sensor."]
    #[inline(always)]
    pub const fn set_power_down(&mut self, val: super::vals::Tempsense0ClrPowerDown) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Starts the measurement process"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_temp(&self) -> super::vals::Tempsense0ClrMeasureTemp {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tempsense0ClrMeasureTemp::from_bits(val as u8)
    }
    #[doc = "Starts the measurement process"]
    #[inline(always)]
    pub const fn set_measure_temp(&mut self, val: super::vals::Tempsense0ClrMeasureTemp) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates that the latest temp is valid"]
    #[must_use]
    #[inline(always)]
    pub const fn finished(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the latest temp is valid"]
    #[inline(always)]
    pub const fn set_finished(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit field contains the last measured temperature count."]
    #[must_use]
    #[inline(always)]
    pub const fn temp_cnt(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the last measured temperature count."]
    #[inline(always)]
    pub const fn set_temp_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[must_use]
    #[inline(always)]
    pub const fn alarm_value(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn set_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for Tempsense0Clr {
    #[inline(always)]
    fn default() -> Tempsense0Clr {
        Tempsense0Clr(0)
    }
}
impl core::fmt::Debug for Tempsense0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense0Clr")
            .field("power_down", &self.power_down())
            .field("measure_temp", &self.measure_temp())
            .field("finished", &self.finished())
            .field("temp_cnt", &self.temp_cnt())
            .field("alarm_value", &self.alarm_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense0Clr {{ power_down: {:?}, measure_temp: {:?}, finished: {=bool:?}, temp_cnt: {=u16:?}, alarm_value: {=u16:?} }}",
            self.power_down(),
            self.measure_temp(),
            self.finished(),
            self.temp_cnt(),
            self.alarm_value()
        )
    }
}
#[doc = "Tempsensor Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense0Set(pub u32);
impl Tempsense0Set {
    #[doc = "This bit powers down the temperature sensor."]
    #[must_use]
    #[inline(always)]
    pub const fn power_down(&self) -> super::vals::Tempsense0SetPowerDown {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tempsense0SetPowerDown::from_bits(val as u8)
    }
    #[doc = "This bit powers down the temperature sensor."]
    #[inline(always)]
    pub const fn set_power_down(&mut self, val: super::vals::Tempsense0SetPowerDown) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Starts the measurement process"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_temp(&self) -> super::vals::Tempsense0SetMeasureTemp {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tempsense0SetMeasureTemp::from_bits(val as u8)
    }
    #[doc = "Starts the measurement process"]
    #[inline(always)]
    pub const fn set_measure_temp(&mut self, val: super::vals::Tempsense0SetMeasureTemp) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates that the latest temp is valid"]
    #[must_use]
    #[inline(always)]
    pub const fn finished(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the latest temp is valid"]
    #[inline(always)]
    pub const fn set_finished(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit field contains the last measured temperature count."]
    #[must_use]
    #[inline(always)]
    pub const fn temp_cnt(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the last measured temperature count."]
    #[inline(always)]
    pub const fn set_temp_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[must_use]
    #[inline(always)]
    pub const fn alarm_value(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn set_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for Tempsense0Set {
    #[inline(always)]
    fn default() -> Tempsense0Set {
        Tempsense0Set(0)
    }
}
impl core::fmt::Debug for Tempsense0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense0Set")
            .field("power_down", &self.power_down())
            .field("measure_temp", &self.measure_temp())
            .field("finished", &self.finished())
            .field("temp_cnt", &self.temp_cnt())
            .field("alarm_value", &self.alarm_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense0Set {{ power_down: {:?}, measure_temp: {:?}, finished: {=bool:?}, temp_cnt: {=u16:?}, alarm_value: {=u16:?} }}",
            self.power_down(),
            self.measure_temp(),
            self.finished(),
            self.temp_cnt(),
            self.alarm_value()
        )
    }
}
#[doc = "Tempsensor Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense0Tog(pub u32);
impl Tempsense0Tog {
    #[doc = "This bit powers down the temperature sensor."]
    #[must_use]
    #[inline(always)]
    pub const fn power_down(&self) -> super::vals::Tempsense0TogPowerDown {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tempsense0TogPowerDown::from_bits(val as u8)
    }
    #[doc = "This bit powers down the temperature sensor."]
    #[inline(always)]
    pub const fn set_power_down(&mut self, val: super::vals::Tempsense0TogPowerDown) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Starts the measurement process"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_temp(&self) -> super::vals::Tempsense0TogMeasureTemp {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tempsense0TogMeasureTemp::from_bits(val as u8)
    }
    #[doc = "Starts the measurement process"]
    #[inline(always)]
    pub const fn set_measure_temp(&mut self, val: super::vals::Tempsense0TogMeasureTemp) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates that the latest temp is valid"]
    #[must_use]
    #[inline(always)]
    pub const fn finished(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the latest temp is valid"]
    #[inline(always)]
    pub const fn set_finished(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit field contains the last measured temperature count."]
    #[must_use]
    #[inline(always)]
    pub const fn temp_cnt(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the last measured temperature count."]
    #[inline(always)]
    pub const fn set_temp_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[must_use]
    #[inline(always)]
    pub const fn alarm_value(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn set_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for Tempsense0Tog {
    #[inline(always)]
    fn default() -> Tempsense0Tog {
        Tempsense0Tog(0)
    }
}
impl core::fmt::Debug for Tempsense0Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense0Tog")
            .field("power_down", &self.power_down())
            .field("measure_temp", &self.measure_temp())
            .field("finished", &self.finished())
            .field("temp_cnt", &self.temp_cnt())
            .field("alarm_value", &self.alarm_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense0Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense0Tog {{ power_down: {:?}, measure_temp: {:?}, finished: {=bool:?}, temp_cnt: {=u16:?}, alarm_value: {=u16:?} }}",
            self.power_down(),
            self.measure_temp(),
            self.finished(),
            self.temp_cnt(),
            self.alarm_value()
        )
    }
}
#[doc = "Tempsensor Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense1(pub u32);
impl Tempsense1 {
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_freq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub const fn set_measure_freq(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tempsense1 {
    #[inline(always)]
    fn default() -> Tempsense1 {
        Tempsense1(0)
    }
}
impl core::fmt::Debug for Tempsense1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense1")
            .field("measure_freq", &self.measure_freq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense1 {{ measure_freq: {=u16:?} }}",
            self.measure_freq()
        )
    }
}
#[doc = "Tempsensor Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense1Clr(pub u32);
impl Tempsense1Clr {
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_freq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub const fn set_measure_freq(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tempsense1Clr {
    #[inline(always)]
    fn default() -> Tempsense1Clr {
        Tempsense1Clr(0)
    }
}
impl core::fmt::Debug for Tempsense1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense1Clr")
            .field("measure_freq", &self.measure_freq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense1Clr {{ measure_freq: {=u16:?} }}",
            self.measure_freq()
        )
    }
}
#[doc = "Tempsensor Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense1Set(pub u32);
impl Tempsense1Set {
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_freq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub const fn set_measure_freq(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tempsense1Set {
    #[inline(always)]
    fn default() -> Tempsense1Set {
        Tempsense1Set(0)
    }
}
impl core::fmt::Debug for Tempsense1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense1Set")
            .field("measure_freq", &self.measure_freq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense1Set {{ measure_freq: {=u16:?} }}",
            self.measure_freq()
        )
    }
}
#[doc = "Tempsensor Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense1Tog(pub u32);
impl Tempsense1Tog {
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_freq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub const fn set_measure_freq(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tempsense1Tog {
    #[inline(always)]
    fn default() -> Tempsense1Tog {
        Tempsense1Tog(0)
    }
}
impl core::fmt::Debug for Tempsense1Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense1Tog")
            .field("measure_freq", &self.measure_freq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense1Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense1Tog {{ measure_freq: {=u16:?} }}",
            self.measure_freq()
        )
    }
}
#[doc = "Tempsensor Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense2(pub u32);
impl Tempsense2 {
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[must_use]
    #[inline(always)]
    pub const fn low_alarm_value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[inline(always)]
    pub const fn set_low_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[must_use]
    #[inline(always)]
    pub const fn panic_alarm_value(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn set_panic_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Tempsense2 {
    #[inline(always)]
    fn default() -> Tempsense2 {
        Tempsense2(0)
    }
}
impl core::fmt::Debug for Tempsense2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense2")
            .field("low_alarm_value", &self.low_alarm_value())
            .field("panic_alarm_value", &self.panic_alarm_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense2 {{ low_alarm_value: {=u16:?}, panic_alarm_value: {=u16:?} }}",
            self.low_alarm_value(),
            self.panic_alarm_value()
        )
    }
}
#[doc = "Tempsensor Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense2Clr(pub u32);
impl Tempsense2Clr {
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[must_use]
    #[inline(always)]
    pub const fn low_alarm_value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[inline(always)]
    pub const fn set_low_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[must_use]
    #[inline(always)]
    pub const fn panic_alarm_value(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn set_panic_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Tempsense2Clr {
    #[inline(always)]
    fn default() -> Tempsense2Clr {
        Tempsense2Clr(0)
    }
}
impl core::fmt::Debug for Tempsense2Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense2Clr")
            .field("low_alarm_value", &self.low_alarm_value())
            .field("panic_alarm_value", &self.panic_alarm_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense2Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense2Clr {{ low_alarm_value: {=u16:?}, panic_alarm_value: {=u16:?} }}",
            self.low_alarm_value(),
            self.panic_alarm_value()
        )
    }
}
#[doc = "Tempsensor Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense2Set(pub u32);
impl Tempsense2Set {
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[must_use]
    #[inline(always)]
    pub const fn low_alarm_value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[inline(always)]
    pub const fn set_low_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[must_use]
    #[inline(always)]
    pub const fn panic_alarm_value(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn set_panic_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Tempsense2Set {
    #[inline(always)]
    fn default() -> Tempsense2Set {
        Tempsense2Set(0)
    }
}
impl core::fmt::Debug for Tempsense2Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense2Set")
            .field("low_alarm_value", &self.low_alarm_value())
            .field("panic_alarm_value", &self.panic_alarm_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense2Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense2Set {{ low_alarm_value: {=u16:?}, panic_alarm_value: {=u16:?} }}",
            self.low_alarm_value(),
            self.panic_alarm_value()
        )
    }
}
#[doc = "Tempsensor Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense2Tog(pub u32);
impl Tempsense2Tog {
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[must_use]
    #[inline(always)]
    pub const fn low_alarm_value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[inline(always)]
    pub const fn set_low_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[must_use]
    #[inline(always)]
    pub const fn panic_alarm_value(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn set_panic_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Tempsense2Tog {
    #[inline(always)]
    fn default() -> Tempsense2Tog {
        Tempsense2Tog(0)
    }
}
impl core::fmt::Debug for Tempsense2Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense2Tog")
            .field("low_alarm_value", &self.low_alarm_value())
            .field("panic_alarm_value", &self.panic_alarm_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense2Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense2Tog {{ low_alarm_value: {=u16:?}, panic_alarm_value: {=u16:?} }}",
            self.low_alarm_value(),
            self.panic_alarm_value()
        )
    }
}
