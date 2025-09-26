use my_rust_pi_app::hw::*;
use serial_test::serial;

#[test]
#[serial]
fn gpio_mock_records_operations() {
    let mut gpio = MockGpio::new();
    
    // Test write operations are recorded
    gpio.write(1, true).unwrap();
    gpio.write(1, false).unwrap();
    gpio.write(2, true).unwrap();
    
    assert_eq!(gpio.get_write_count(1), 2);
    assert_eq!(gpio.get_write_count(2), 1);
    assert_eq!(gpio.get_write_count(3), 0);
}

#[test]
#[serial]
fn gpio_mock_scripted_responses() {
    let mut gpio = MockGpio::new();
    gpio.set_scripted_responses(1, vec![true, false, true, false]);
    
    assert_eq!(gpio.read(1).unwrap(), true);
    assert_eq!(gpio.read(1).unwrap(), false);
    assert_eq!(gpio.read(1).unwrap(), true);
    assert_eq!(gpio.read(1).unwrap(), false);
    
    assert_eq!(gpio.get_read_count(1), 4);
}

#[test]
#[serial]
fn gpio_mock_failure_injection() {
    let mut gpio = MockGpio::new();
    gpio.set_pin_failure(5);
    
    // Operations on failed pin should return errors
    assert!(gpio.write(5, true).is_err());
    assert!(gpio.read(5).is_err());
    
    // Operations on normal pins should still work
    assert!(gpio.write(6, true).is_ok());
    assert!(gpio.read(6).is_ok());
}

#[test]
#[serial]
fn i2c_mock_logs_writes() {
    let mut i2c = MockI2c::new();
    
    i2c.write(0x48, &[0x01, 0x02, 0x03]).unwrap();
    i2c.write(0x49, &[0xFF]).unwrap();
    
    let log = i2c.get_write_log();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0], (0x48, vec![0x01, 0x02, 0x03]));
    assert_eq!(log[1], (0x49, vec![0xFF]));
}

#[test]
#[serial]
fn i2c_mock_scripted_reads() {
    let mut i2c = MockI2c::new();
    i2c.set_read_response(0x48, vec![0xAA, 0xBB, 0xCC]);
    
    let mut buffer = [0u8; 5];
    let bytes_read = i2c.read(0x48, &mut buffer).unwrap();
    
    assert_eq!(bytes_read, 3);
    assert_eq!(&buffer[..3], &[0xAA, 0xBB, 0xCC]);
    assert_eq!(&buffer[3..], &[0, 0]); // Rest should be unchanged
}

#[test]
#[serial]
fn i2c_mock_address_failures() {
    let mut i2c = MockI2c::new();
    i2c.set_address_failure(0x50);
    
    assert!(i2c.write(0x50, &[0x01]).is_err());
    
    let mut buffer = [0u8; 1];
    assert!(i2c.read(0x50, &mut buffer).is_err());
    
    // Other addresses should work
    assert!(i2c.write(0x51, &[0x01]).is_ok());
}

#[test]
#[serial]
fn spi_mock_transfer_logging() {
    let mut spi = MockSpi::new();
    
    let mut data1 = [0x01, 0x02];
    let mut data2 = [0x03, 0x04, 0x05];
    
    spi.transfer(&mut data1).unwrap();
    spi.transfer(&mut data2).unwrap();
    
    let log = spi.get_transfer_log();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0], vec![0x01, 0x02]);
    assert_eq!(log[1], vec![0x03, 0x04, 0x05]);
}

#[test]
#[serial]
fn spi_mock_scripted_responses() {
    let mut spi = MockSpi::new();
    spi.add_response(vec![0xAA, 0xBB]);
    spi.add_response(vec![0xCC, 0xDD, 0xEE]);
    
    let mut data1 = [0x01, 0x02];
    spi.transfer(&mut data1).unwrap();
    assert_eq!(data1, [0xAA, 0xBB]);
    
    let mut data2 = [0x03, 0x04, 0x05];
    spi.transfer(&mut data2).unwrap();
    assert_eq!(data2, [0xCC, 0xDD, 0xEE]);
}

#[test]
#[serial]
fn spi_mock_failure_injection() {
    let mut spi = MockSpi::new();
    spi.set_failure(true);
    
    let mut data = [0x01, 0x02];
    assert!(spi.transfer(&mut data).is_err());
    
    // Should work after disabling failure
    spi.set_failure(false);
    assert!(spi.transfer(&mut data).is_ok());
}