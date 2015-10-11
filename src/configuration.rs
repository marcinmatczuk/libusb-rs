use std::slice;

use ::interface::Interface;


/// Describes a configuration.
#[derive(Debug)]
pub struct Configuration {
    number: u8,
    description_index: Option<u8>,
    attributes: u8,
    max_power: u8,
    interfaces: Vec<Interface>
}

impl Configuration {
    /// Returns the configuration number.
    pub fn number(&self) -> u8 {
        self.number
    }

    /// Returns the device's maximum power consumption (in milliwatts) in this configuration.
    pub fn max_power(&self) -> u16 {
        self.max_power as u16 * 2
    }

    /// Indicates if the device is self-powered in this configuration.
    pub fn self_powered(&self) -> bool {
        self.attributes & 0x40 != 0
    }

    /// Indicates if the device has remote wakeup capability in this configuration.
    pub fn remote_wakeup(&self) -> bool {
        self.attributes & 0x20 != 0
    }

    /// Returns the index of the string descriptor that describes the configuration.
    pub fn description_string_index(&self) -> Option<u8> {
        self.description_index
    }

    /// Returns a collection of the configuration's interfaces.
    pub fn interfaces(&self) -> &[Interface] {
        &self.interfaces[..]
    }
}


#[doc(hidden)]
pub fn from_libusb(configuration: &::libusb::libusb_config_descriptor) -> Configuration {
    let interfaces = unsafe { slice::from_raw_parts(configuration.interface, configuration.bNumInterfaces as usize) };

    Configuration {
        number:            configuration.bConfigurationValue,
        description_index: match configuration.iConfiguration {
            0 => None,
            n => Some(n)
        },
        attributes:        configuration.bmAttributes,
        max_power:         configuration.bMaxPower,
        interfaces:        interfaces.iter().map(|interface| ::interface::from_libusb(&interface)).collect()
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn it_has_number() {
        assert_eq!(42, ::configuration::from_libusb(&config_descriptor!(bConfigurationValue: 42)).number());
    }

    #[test]
    fn it_has_max_power() {
        assert_eq!(42, ::configuration::from_libusb(&config_descriptor!(bMaxPower: 21)).max_power());
    }

    #[test]
    fn it_interprets_self_powered_bit_in_attributes() {
        assert_eq!(false, ::configuration::from_libusb(&config_descriptor!(bmAttributes: 0b0000_0000)).self_powered());
        assert_eq!(true,  ::configuration::from_libusb(&config_descriptor!(bmAttributes: 0b0100_0000)).self_powered());
    }

    #[test]
    fn it_interprets_remote_wakeup_bit_in_attributes() {
        assert_eq!(false, ::configuration::from_libusb(&config_descriptor!(bmAttributes: 0b0000_0000)).remote_wakeup());
        assert_eq!(true,  ::configuration::from_libusb(&config_descriptor!(bmAttributes: 0b0010_0000)).remote_wakeup());
    }

    #[test]
    fn it_has_description_string_index() {
        assert_eq!(Some(42), ::configuration::from_libusb(&config_descriptor!(iConfiguration: 42)).description_string_index());
    }

    #[test]
    fn it_handles_missing_description_string_index() {
        assert_eq!(None, ::configuration::from_libusb(&config_descriptor!(iConfiguration: 0),).description_string_index());
    }

    #[test]
    fn it_has_interfaces() {
        let interface = interface!(interface_descriptor!(
                bInterfaceNumber:   1,
                bAlternateSetting:  2,
                bInterfaceClass:    3,
                bInterfaceSubClass: 4,
                bInterfaceProtocol: 5,
                iInterface:         6));

        assert_eq!(
            vec!(&::interface::from_libusb(&interface)),
            ::configuration::from_libusb(&config_descriptor!(interface)).interfaces().iter().collect::<Vec<_>>());
    }
}
