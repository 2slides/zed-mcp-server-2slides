use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed_extension_api::{
    serde_json, settings::ContextServerSettings, Command, ContextServerConfiguration,
    ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "2slides-mcp";
const PACKAGE_VERSION: &str = "0.2.1";
const SERVER_PATH: &str = "node_modules/2slides-mcp/dist/cli.js";
const CONTEXT_SERVER_ID: &str = "mcp-server-2slides";

struct TwoSlidesContextServerExtension;

#[derive(Debug, Deserialize, JsonSchema)]
struct TwoSlidesContextServerSettings {
    /// Your 2slides API Key from https://2slides.com/api
    api_key: String,
}

impl zed_extension_api::Extension for TwoSlidesContextServerExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        assert_eq!(
            context_server_id.as_ref(),
            CONTEXT_SERVER_ID,
            "Unexpected context server ID",
        );

        let version = zed_extension_api::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed_extension_api::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        let settings = ContextServerSettings::for_project(CONTEXT_SERVER_ID, project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `api_key` setting".into());
        };

        let settings: TwoSlidesContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        let server_entry = env::current_dir()
            .map_err(|e| e.to_string())?
            .join(SERVER_PATH)
            .to_string_lossy()
            .to_string();

        Ok(Command {
            command: zed_extension_api::node_binary_path()?,
            args: vec![server_entry],
            env: vec![("API_KEY".into(), settings.api_key)],
        })
    }

    fn context_server_configuration(
        &mut self,
        context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        assert_eq!(
            context_server_id.as_ref(),
            CONTEXT_SERVER_ID,
            "Unexpected context server ID",
        );

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

zed_extension_api::register_extension!(TwoSlidesContextServerExtension);

