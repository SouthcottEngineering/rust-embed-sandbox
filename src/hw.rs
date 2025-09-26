use anyhow::Result;
use std::collections::HashMap;

/// GPIO abstraction trait for testable hardware interactions
pub trait Gpio {
    fn write(&mut self, pin: u8, high: bool) -> Result<()>;
    fn read(&mut self, pin: u8) -> Result<bool>;
}

/// Mock GPIO implementation for testing and emulation
#[derive(Debug, Clone)]
pub struct MockGpio {
    pin_states: HashMap<u8, bool>,
    write_counts: HashMap<u8, usize>,
    read_counts: HashMap<u8, usize>,
    scripted_responses: HashMap<u8, Vec<bool>>,
    response_indices: HashMap<u8, usize>,
    failure_pins: Vec<u8>,
}

impl MockGpio {
    pub fn new() -> Self {
        Self {
            pin_states: HashMap::new(),
            write_counts: HashMap::new(),
            read_counts: HashMap::new(),
            scripted_responses: HashMap::new(),
            response_indices: HashMap::new(),
            failure_pins: Vec::new(),
        }
    }

    /// Set scripted responses for a pin's read operations
    pub fn set_scripted_responses(&mut self, pin: u8, responses: Vec<bool>) {
        self.scripted_responses.insert(pin, responses);
        self.response_indices.insert(pin, 0);
    }

    /// Configure a pin to always fail operations (for testing error handling)
    pub fn set_pin_failure(&mut self, pin: u8) {
        self.failure_pins.push(pin);
    }

    /// Get the number of write operations performed on a pin
    pub fn get_write_count(&self, pin: u8) -> usize {
        self.write_counts.get(&pin).copied().unwrap_or(0)
    }

    /// Get the number of read operations performed on a pin
    pub fn get_read_count(&self, pin: u8) -> usize {
        self.read_counts.get(&pin).copied().unwrap_or(0)
    }

    /// Get the current state of a pin
    pub fn get_pin_state(&self, pin: u8) -> Option<bool> {
        self.pin_states.get(&pin).copied()
    }

    /// Reset all counters and states
    pub fn reset(&mut self) {
        self.pin_states.clear();
        self.write_counts.clear();
        self.read_counts.clear();
        self.scripted_responses.clear();
        self.response_indices.clear();
        self.failure_pins.clear();
    }
}

impl Default for MockGpio {
    fn default() -> Self {
        Self::new()
    }
}

impl Gpio for MockGpio {
    fn write(&mut self, pin: u8, high: bool) -> Result<()> {
        if self.failure_pins.contains(&pin) {
            return Err(anyhow::anyhow!("Simulated GPIO failure on pin {}", pin));
        }

        // Increment write counter
        let count = self.write_counts.entry(pin).or_insert(0);
        *count += 1;

        // Update pin state
        self.pin_states.insert(pin, high);

        Ok(())
    }

    fn read(&mut self, pin: u8) -> Result<bool> {
        if self.failure_pins.contains(&pin) {
            return Err(anyhow::anyhow!("Simulated GPIO failure on pin {}", pin));
        }

        // Increment read counter
        let count = self.read_counts.entry(pin).or_insert(0);
        *count += 1;

        // Check if we have scripted responses
        if let Some(responses) = self.scripted_responses.get(&pin) {
            let index = self.response_indices.get(&pin).copied().unwrap_or(0);
            if index < responses.len() {
                let response = responses[index];
                self.response_indices.insert(pin, index + 1);
                return Ok(response);
            }
        }

        // Default to current pin state or false if not set
        Ok(self.pin_states.get(&pin).copied().unwrap_or(false))
    }
}

/// I2C abstraction trait for expandable hardware interfaces
pub trait I2c {
    fn write(&mut self, address: u8, data: &[u8]) -> Result<()>;
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<usize>;
}

/// Mock I2C implementation for testing
#[derive(Debug, Clone)]
pub struct MockI2c {
    write_log: Vec<(u8, Vec<u8>)>,
    read_responses: HashMap<u8, Vec<u8>>,
    failure_addresses: Vec<u8>,
}

impl MockI2c {
    pub fn new() -> Self {
        Self {
            write_log: Vec::new(),
            read_responses: HashMap::new(),
            failure_addresses: Vec::new(),
        }
    }

    pub fn set_read_response(&mut self, address: u8, data: Vec<u8>) {
        self.read_responses.insert(address, data);
    }

    pub fn set_address_failure(&mut self, address: u8) {
        self.failure_addresses.push(address);
    }

    pub fn get_write_log(&self) -> &[(u8, Vec<u8>)] {
        &self.write_log
    }
}

impl Default for MockI2c {
    fn default() -> Self {
        Self::new()
    }
}

impl I2c for MockI2c {
    fn write(&mut self, address: u8, data: &[u8]) -> Result<()> {
        if self.failure_addresses.contains(&address) {
            return Err(anyhow::anyhow!(
                "Simulated I2C failure on address 0x{:02X}",
                address
            ));
        }

        self.write_log.push((address, data.to_vec()));
        Ok(())
    }

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<usize> {
        if self.failure_addresses.contains(&address) {
            return Err(anyhow::anyhow!(
                "Simulated I2C failure on address 0x{:02X}",
                address
            ));
        }

        if let Some(response) = self.read_responses.get(&address) {
            let bytes_to_copy = std::cmp::min(buffer.len(), response.len());
            buffer[..bytes_to_copy].copy_from_slice(&response[..bytes_to_copy]);
            Ok(bytes_to_copy)
        } else {
            // Return zeros if no response configured
            buffer.fill(0);
            Ok(buffer.len())
        }
    }
}

/// SPI abstraction trait for expandable hardware interfaces
pub trait Spi {
    fn transfer(&mut self, data: &mut [u8]) -> Result<()>;
}

/// Mock SPI implementation for testing
#[derive(Debug, Clone)]
pub struct MockSpi {
    transfer_log: Vec<Vec<u8>>,
    responses: Vec<Vec<u8>>,
    response_index: usize,
    should_fail: bool,
}

impl MockSpi {
    pub fn new() -> Self {
        Self {
            transfer_log: Vec::new(),
            responses: Vec::new(),
            response_index: 0,
            should_fail: false,
        }
    }

    pub fn add_response(&mut self, response: Vec<u8>) {
        self.responses.push(response);
    }

    pub fn set_failure(&mut self, should_fail: bool) {
        self.should_fail = should_fail;
    }

    pub fn get_transfer_log(&self) -> &[Vec<u8>] {
        &self.transfer_log
    }
}

impl Default for MockSpi {
    fn default() -> Self {
        Self::new()
    }
}

impl Spi for MockSpi {
    fn transfer(&mut self, data: &mut [u8]) -> Result<()> {
        if self.should_fail {
            return Err(anyhow::anyhow!("Simulated SPI failure"));
        }

        // Log the input data
        self.transfer_log.push(data.to_vec());

        // If we have a scripted response, use it
        if self.response_index < self.responses.len() {
            let response = &self.responses[self.response_index];
            let bytes_to_copy = std::cmp::min(data.len(), response.len());
            data[..bytes_to_copy].copy_from_slice(&response[..bytes_to_copy]);
            self.response_index += 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_gpio_basic_operations() {
        let mut gpio = MockGpio::new();

        // Test write and read
        gpio.write(1, true).unwrap();
        assert!(gpio.read(1).unwrap());
        assert_eq!(gpio.get_write_count(1), 1);
        assert_eq!(gpio.get_read_count(1), 1);
    }

    #[test]
    fn test_mock_gpio_scripted_responses() {
        let mut gpio = MockGpio::new();
        gpio.set_scripted_responses(2, vec![true, false, true]);

        assert!(gpio.read(2).unwrap());
        assert!(!gpio.read(2).unwrap());
        assert!(gpio.read(2).unwrap());
        assert_eq!(gpio.get_read_count(2), 3);
    }

    #[test]
    fn test_mock_gpio_failure_simulation() {
        let mut gpio = MockGpio::new();
        gpio.set_pin_failure(3);

        assert!(gpio.write(3, true).is_err());
        assert!(gpio.read(3).is_err());
    }

    #[test]
    fn test_mock_i2c_operations() {
        let mut i2c = MockI2c::new();

        // Test write
        i2c.write(0x48, &[0x01, 0x02]).unwrap();
        assert_eq!(i2c.get_write_log().len(), 1);
        assert_eq!(i2c.get_write_log()[0], (0x48, vec![0x01, 0x02]));

        // Test read with response
        i2c.set_read_response(0x48, vec![0xAB, 0xCD]);
        let mut buffer = [0u8; 4];
        let bytes_read = i2c.read(0x48, &mut buffer).unwrap();
        assert_eq!(bytes_read, 2);
        assert_eq!(&buffer[..2], &[0xAB, 0xCD]);
    }

    #[test]
    fn test_mock_spi_transfer() {
        let mut spi = MockSpi::new();
        spi.add_response(vec![0xFF, 0xEE]);

        let mut data = [0x01, 0x02];
        spi.transfer(&mut data).unwrap();

        assert_eq!(data, [0xFF, 0xEE]);
        assert_eq!(spi.get_transfer_log().len(), 1);
        assert_eq!(spi.get_transfer_log()[0], vec![0x01, 0x02]);
    }
}
