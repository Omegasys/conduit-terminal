use super::{
    capabilities::{
        CustomShellCapabilities,
        ShellColorSupport,
    },
    commands::{
        CustomCommand,
        CustomCommandResult,
    },
    configuration::CustomShellConfiguration,
    environment::CustomShellEnvironment,
    errors::CustomShellResult,
    manifest::{
        CustomShellManifest,
        CustomShellMetadata,
    },
    parser::{
        CustomShellParser,
        ShellParseResult,
    },
    prompt::CustomPrompt,
    shell::{
        CustomShell,
        CustomShellId,
        CustomShellVersion,
    },
};

pub struct ExampleShell {
    manifest: CustomShellManifest,
    capabilities: CustomShellCapabilities,
    configuration: CustomShellConfiguration,
    environment: CustomShellEnvironment,
    prompt: CustomPrompt,
    parser: CustomShellParser,
}

impl ExampleShell {
    pub fn new() -> Self {
        let metadata = CustomShellMetadata::new(
            "Conduit Example Author",
            "Example user-created Conduit shell.",
        )
        .license("GPL-3.0-or-later");

        let mut manifest =
            CustomShellManifest::new(
                CustomShellId::new(
                    "example-shell",
                ),
                "Example Shell",
                metadata,
            );

        manifest.set_version(
            CustomShellVersion::new(
                1,
                0,
                0,
            ),
        );

        let capabilities =
            CustomShellCapabilities {
                color:
                    ShellColorSupport::TrueColor,
                command_completion: true,
                command_aliases: true,
                scripting: true,
                pipelines: true,
                redirection: true,
                job_control: true,
                ..Default::default()
            };

        Self {
            manifest,
            capabilities,
            configuration:
                CustomShellConfiguration::new(),
            environment:
                CustomShellEnvironment::new(),
            prompt:
                CustomPrompt::new("example> "),
            parser:
                CustomShellParser::new(),
        }
    }
}

impl Default for ExampleShell {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomShell for ExampleShell {
    fn id(&self) -> CustomShellId {
        self.manifest.id().clone()
    }

    fn version(&self) -> CustomShellVersion {
        self.manifest.version().clone()
    }

    fn manifest(&self) -> CustomShellManifest {
        self.manifest.clone()
    }

    fn capabilities(
        &self,
    ) -> CustomShellCapabilities {
        self.capabilities.clone()
    }

    fn configuration(
        &self,
    ) -> &CustomShellConfiguration {
        &self.configuration
    }

    fn configuration_mut(
        &mut self,
    ) -> &mut CustomShellConfiguration {
        &mut self.configuration
    }

    fn environment(
        &self,
    ) -> &CustomShellEnvironment {
        &self.environment
    }

    fn environment_mut(
        &mut self,
    ) -> &mut CustomShellEnvironment {
        &mut self.environment
    }

    fn prompt(
        &self,
    ) -> &CustomPrompt {
        &self.prompt
    }

    fn reset(&mut self) {
        self.parser.reset();
        self.prompt.reset();
    }

    fn initialize(
        &mut self,
    ) -> CustomShellResult<()> {
        self.prompt.set_state(
            super::prompt::CustomPromptState::Ready,
        );

        Ok(())
    }

    fn parse_input(
        &mut self,
        input: &[u8],
    ) -> CustomShellResult<ShellParseResult> {
        self.parser.feed(input)
    }

    fn parse_output(
        &mut self,
        output: &[u8],
    ) -> CustomShellResult<ShellParseResult> {
        self.parser.feed(output)
    }

    fn execute_command(
        &mut self,
        _command: CustomCommand,
    ) -> CustomShellResult<CustomCommandResult> {
        Ok(CustomCommandResult::success())
    }
}
