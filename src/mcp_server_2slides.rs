use schemars::JsonSchema;
use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

#[derive(Debug, Deserialize, JsonSchema)]
struct TwoSlidesContextServerSettings {
    /// Your 2slides API Key from https://2slides.com/api
    api_key: String,
}

struct TwoSlidesContextServerExtension;

impl zed::Extension for TwoSlidesContextServerExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = ContextServerSettings::for_project("mcp-server-2slides", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `api_key` setting".into());
        };
        let settings: TwoSlidesContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        Ok(Command {
            command: "npx".into(),
            args: vec!["--yes".into(), "2slides-mcp".into()],
            env: vec![("API_KEY".into(), settings.api_key)],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(TwoSlidesContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(TwoSlidesContextServerExtension);

