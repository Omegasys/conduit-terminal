#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolExecutionModel {
    ExternalProcess,
    Embedded,
    Hybrid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolColorSupport {
    None,
    Basic8,
    Standard16,
    Indexed256,
    TrueColor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomProtocolCapabilities {
    pub execution_model: ProtocolExecutionModel,
    pub color: ProtocolColorSupport,

    pub unicode: bool,
    pub stateful: bool,

    pub input_sequences: bool,
    pub output_sequences: bool,

    pub graphics: bool,
    pub hyperlinks: bool,
    pub clipboard: bool,
    pub mouse: bool,
    pub keyboard: bool,

    pub alternate_screen: bool,
    pub synchronized_output: bool,

    pub binary_data: bool,
    pub streaming: bool,
}

impl Default for CustomProtocolCapabilities {
    fn default() -> Self {
        Self {
            execution_model: ProtocolExecutionModel::Embedded,
            color: ProtocolColorSupport::None,

            unicode: true,
            stateful: true,

            input_sequences: true,
            output_sequences: true,

            graphics: false,
            hyperlinks: false,
            clipboard: false,
            mouse: false,
            keyboard: false,

            alternate_screen: false,
            synchronized_output: false,

            binary_data: false,
            streaming: true,
        }
    }
}

impl CustomProtocolCapabilities {
    pub fn terminal() -> Self {
        Self {
            execution_model: ProtocolExecutionModel::Embedded,
            color: ProtocolColorSupport::TrueColor,

            unicode: true,
            stateful: true,

            input_sequences: true,
            output_sequences: true,

            graphics: true,
            hyperlinks: true,
            clipboard: true,
            mouse: true,
            keyboard: true,

            alternate_screen: true,
            synchronized_output: true,

            binary_data: true,
            streaming: true,
        }
    }
}
