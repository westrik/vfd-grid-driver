use bit_vec::BitVec;
use embedded_hal::digital::v2::OutputPin;

use crate::command::Command;

const DISPLAY_WIDTH: usize = 20;
const DISPLAY_HEIGHT: usize = 2;

#[derive(Debug)]
pub enum DisplayError {
    PinFailure,
    InvalidBit,
}

pub struct VfdDisplay<DP7, DP6, DP5, DP4, DP3, DP2, DP1, DP0, RST, WS> {
    pub d7: DP7,
    pub d6: DP6,
    pub d5: DP5,
    pub d4: DP4,
    pub d3: DP3,
    pub d2: DP2,
    pub d1: DP1,
    pub d0: DP0,
    pub reset: RST,
    pub write_strobe: WS,
}

impl<DP7, DP6, DP5, DP4, DP3, DP2, DP1, DP0, RST, WS>
    VfdDisplay<DP7, DP6, DP5, DP4, DP3, DP2, DP1, DP0, RST, WS>
where
    DP7: OutputPin,
    DP6: OutputPin,
    DP5: OutputPin,
    DP4: OutputPin,
    DP3: OutputPin,
    DP2: OutputPin,
    DP1: OutputPin,
    DP0: OutputPin,
{
    pub fn reset(&mut self) -> Result<(), DisplayError> {
        self.send_command(&Command::Reset)
    }

    pub fn clear(&mut self) -> Result<(), DisplayError> {
        self.send_command(&Command::Clear)
    }
}

pub(crate) trait SendCommand {
    fn send_command(&mut self, command: &Command) -> Result<(), DisplayError>;
    fn send_commands(&mut self, commands: &[Command]) -> Result<(), DisplayError>;
}

impl<DP7, DP6, DP5, DP4, DP3, DP2, DP1, DP0, RST, WS> SendCommand
    for VfdDisplay<DP7, DP6, DP5, DP4, DP3, DP2, DP1, DP0, RST, WS>
where
    DP7: OutputPin,
    DP6: OutputPin,
    DP5: OutputPin,
    DP4: OutputPin,
    DP3: OutputPin,
    DP2: OutputPin,
    DP1: OutputPin,
    DP0: OutputPin,
{
    fn send_command(&mut self, command: &Command) -> Result<(), DisplayError> {
        for (idx, bit) in BitVec::from_bytes(&[command.to_byte()]).iter().enumerate() {
            if bit {
                match idx {
                    0 => self.d7.set_high().map_err(|_| DisplayError::PinFailure)?, // MSB
                    1 => self.d6.set_high().map_err(|_| DisplayError::PinFailure)?,
                    2 => self.d5.set_high().map_err(|_| DisplayError::PinFailure)?,
                    3 => self.d4.set_high().map_err(|_| DisplayError::PinFailure)?,
                    4 => self.d3.set_high().map_err(|_| DisplayError::PinFailure)?,
                    5 => self.d2.set_high().map_err(|_| DisplayError::PinFailure)?,
                    6 => self.d1.set_high().map_err(|_| DisplayError::PinFailure)?,
                    7 => self.d0.set_high().map_err(|_| DisplayError::PinFailure)?, // LSB
                    _ => Err(DisplayError::InvalidBit)?,
                };
            } else {
                match idx {
                    0 => self.d7.set_low().map_err(|_| DisplayError::PinFailure)?, // MSB
                    1 => self.d6.set_low().map_err(|_| DisplayError::PinFailure)?,
                    2 => self.d5.set_low().map_err(|_| DisplayError::PinFailure)?,
                    3 => self.d4.set_low().map_err(|_| DisplayError::PinFailure)?,
                    4 => self.d3.set_low().map_err(|_| DisplayError::PinFailure)?,
                    5 => self.d2.set_low().map_err(|_| DisplayError::PinFailure)?,
                    6 => self.d1.set_low().map_err(|_| DisplayError::PinFailure)?,
                    7 => self.d0.set_low().map_err(|_| DisplayError::PinFailure)?, // LSB
                    _ => Err(DisplayError::InvalidBit)?,
                };
            }
        }
        Ok(())
    }

    fn send_commands(&mut self, commands: &[Command]) -> Result<(), DisplayError> {
        for command in commands {
            self.send_command(command)?;
        }
        Ok(())
    }
}
