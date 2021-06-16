/*
-- Pin Assignments --
MCU    Pin#    Function
A15    2       BUSY
A12    2       WRITE STROBE
A11    3       D7 (MSB)
A10    4       D6
A9     5       D5
A8     6       D4
B15    7       D3
B14    8       D2
B13    9       D1
B12    10      D0 (LSB)
N/A    11      +5V @ 370mA (TYP)
N/A    12      GROUND (COMMON)
N/A    13      NOT USED
A4     14      !RESET
*/

#![deny(unsafe_code)]
#![no_std]

pub mod character;
pub mod command;

// pub fn initialize() {
//
// }
