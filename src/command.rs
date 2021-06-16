/*
-- Brightness Levels --
Data    Description
1C      Dimmest (12%)
1D      Dim (25%)
1E      Bright (50%)
1F      Brightest (100%)
Display automatically defaults to "Brightest" after power-up.
*/

pub enum BrightnessLevel {
    Dimmest,
    Dim,
    Bright,
    Brightest,
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
    /// NORMAL DATA ENTRY WITH WRAPAROUND TO HOME POSITION (<11>)
    TodoNormalDataEntry,
    /// OVERWRITE OF RIGHT-MOST CHARACTER ON THE BOTTOM LINE ONLY/ AUTOMATIC CARRIAGE RETURN OFF (<12>)
    TodoOverwriteRightMost,
    /// HORIZONTAL SCROLL MODE (from right to left on bottom line only, after line has been filled) (<13>)
    TodoHorizontalScrollMode,
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

// impl Command {
//     pub(crate) fn send<DI>(self, display: &mut DI) -> Result<(), DisplayError>
//         where
//             DI: WriteOnlyDataCommand,
//     {
//         let (data, len) = match self {
//             Self::Backspace => ([0x08, 0, 0], 1),
//             Self::AdvanceCursor => ([0x09, 0, 0], 1),
//             Self::LineFeed => ([0x0A, 0, 0], 1),
//             Self::CarriageReturn => ([0x0D, 0, 0], 1),
//             Self::HideCursor => ([0x0E, 0, 0], 1),
//             Self::ShowCursor => ([0x0F, 0, 0], 1),
//             Self::TodoNormalDataEntry => unimplemented!(),//([0x11, 0, 0], 1),
//             Self::TodoOverwriteRightMost => unimplemented!(),//([0x12, 0, 0], 1),
//             Self::TodoHorizontalScrollMode => unimplemented!(),//([0x13, 0, 0], 1),
//             Self::Reset => ([0x14, 0, 0], 1),
//             Self::Clear => ([0x15, 0, 0], 1),
//             Self::CursorHome => ([0x16, 0, 0], 1),
//             Self::StartLoadingCustomCharacter => ([0x18, 0, 0], 1),
//             Self::MsbHighForNextByteOnly => ([0x19, 0, 0], 1),
//         };
//         display.send_commands(U8(&data[0..len]))
//     }
// }
