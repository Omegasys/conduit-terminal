use super::{
    errors::CustomShellResult,
};

#[derive(Clone, Debug, Default)]
pub struct ShellParseResult {
    pub commands: Vec<String>,
    pub output: Vec<u8>,
    pub prompt_detected: bool,
    pub directory: Option<String>,
    pub exit_code: Option<i32>,
}

impl ShellParseResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_command<S: Into<String>>(
        &mut self,
        command: S,
    ) {
        self.commands.push(command.into());
    }

    pub fn add_output(
        &mut self,
        output: &[u8],
    ) {
        self.output.extend_from_slice(output);
    }

    pub fn set_prompt_detected(
        &mut self,
        detected: bool,
    ) {
        self.prompt_detected = detected;
    }

    pub fn set_directory<S: Into<String>>(
        &mut self,
        directory: S,
    ) {
        self.directory = Some(directory.into());
    }

    pub fn set_exit_code(
        &mut self,
        code: i32,
    ) {
        self.exit_code = Some(code);
    }
}

pub struct CustomShellParser {
    buffer: Vec<u8>,
    maximum_buffer_size: usize,
}

impl CustomShellParser {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            maximum_buffer_size: 1024 * 1024,
        }
    }

    pub fn with_maximum_buffer_size(
        maximum: usize,
    ) -> Self {
        Self {
            buffer: Vec::new(),
            maximum_buffer_size: maximum.max(1),
        }
    }

    pub fn feed(
        &mut self,
        bytes: &[u8],
    ) -> CustomShellResult<ShellParseResult> {
        self.buffer.extend_from_slice(bytes);

        if self.buffer.len()
            > self.maximum_buffer_size
        {
            self.buffer.clear();

            return Err(
                super::errors::CustomShellError::BufferTooLarge {
                    maximum: self.maximum_buffer_size,
                },
            );
        }

        let mut result = ShellParseResult::new();

        result.add_output(bytes);

        if self
            .buffer
            .windows(1)
            .any(|window| window == b"\n")
        {
            result.prompt_detected = true;
        }

        Ok(result)
    }

    pub fn buffer(
        &self,
    ) -> &[u8] {
        &self.buffer
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn reset(&mut self) {
        self.clear();
    }
}

impl Default for CustomShellParser {
    fn default() -> Self {
        Self::new()
    }
}
