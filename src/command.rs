use crate::character::Character;
use crate::display::VfdDisplay;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum BrightnessLevel {
    Dimmest = 0x1C,
    Dim = 0x1D,
    Bright = 0x1E,
    // Display automatically defaults to "Brightest" after power-up.
    Brightest = 0x1F,
}

/// Holds commands which can be sent to the display.
pub enum Command {
    /// BACK SPACE CURSOR LOCATION ONE POSITION (0x08)
    Backspace,
    /// ADVANCE CURSOR LOCATION ONE POSITION (0x09)
    AdvanceCursor,
    /// LINE FEED (0x0A)
    LineFeed,
    /// CARRIAGE RETURN (0x0D)
    CarriageReturn,
    /// MAKE CURSOR INDICATOR INVISIBLE (0x0E)
    HideCursor,
    /// MAKE CURSOR INDICATOR VISIBLE (0x0F)
    ShowCursor,
    SendCharacter(Character),
    /// NORMAL DATA ENTRY WITH WRAPAROUND TO HOME POSITION (<11>)
    NormalDataEntry,
    /// OVERWRITE OF RIGHT-MOST CHARACTER ON THE BOTTOM LINE ONLY/ AUTOMATIC CARRIAGE RETURN OFF (<12>)
    OverwriteRightMost,
    /// HORIZONTAL SCROLL MODE (from right to left on bottom line only, after line has been filled) (<13>)
    HorizontalScrollMode,
    /// RESET (0x14)
    Reset,
    /// DISPLAY CLEAR (0x15)
    Clear,
    /// CURSOR HOME (0x16)
    CursorHome,
    /// BEGIN USER DEFINED CHARACTER LOADING (0x18)
    StartLoadingCustomCharacter,
    /// BIT 7 HIGH FOR NEXT BYTE ONLY (0x19)
    MsbHighForNextByteOnly,
    SetBrightness(BrightnessLevel),
}

impl Command {
    pub(crate) fn to_byte(&self) -> u8 {
        match self {
            Self::Backspace => 0x08,
            Self::AdvanceCursor => 0x09,
            Self::LineFeed => 0x0A,
            Self::CarriageReturn => 0x0D,
            Self::HideCursor => 0x0E,
            Self::ShowCursor => 0x0F,
            Self::SendCharacter(character) => *character as u8,
            Self::NormalDataEntry => 0x11,
            Self::OverwriteRightMost => 0x12,
            Self::HorizontalScrollMode => 0x13,
            Self::Reset => 0x14,
            Self::Clear => 0x15,
            Self::CursorHome => 0x16,
            Self::StartLoadingCustomCharacter => 0x18,
            Self::MsbHighForNextByteOnly => 0x19,
            Self::SetBrightness(brightness_level) => *brightness_level as u8,
        }
    }
}
