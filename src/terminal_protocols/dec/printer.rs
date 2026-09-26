#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecPrinterMode {
    Normal,
    AutoPrint,
    Controller,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecPrinterAction {
    Start,
    Stop,
    PrintScreen,
    PrintLine,
}

impl DecPrinterAction {
    pub fn mode_for(self) -> Option<DecPrinterMode> {
        match self {
            Self::Start => Some(DecPrinterMode::AutoPrint),
            Self::Stop => Some(DecPrinterMode::Normal),
            Self::PrintScreen | Self::PrintLine => None,
        }
    }
}
