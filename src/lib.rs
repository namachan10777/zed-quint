use std::fs;
use zed_extension_api::{self as zed, LanguageServerId, Result};

const PACKAGE_NAME: &str = "@informalsystems/quint-language-server";
const SERVER_PATH: &str =
    "node_modules/@informalsystems/quint-language-server/out/src/server.js";

struct QuintExtension {
    did_find_server: bool,
}

impl QuintExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(&mut self, id: &LanguageServerId) -> Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server && server_exists {
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

        if !server_exists
            || zed::npm_package_installed_version(PACKAGE_NAME)?.as_deref()
                != Some(version.as_str())
        {
            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            match zed::npm_install_package(PACKAGE_NAME, &version) {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                            "installed package '{PACKAGE_NAME}' did not contain \
                             expected path '{SERVER_PATH}'"
                        ))?;
                    }
                }
                Err(error) => {
                    if !self.server_exists() {
                        Err(error)?;
                    }
                }
            }
        }

        self.did_find_server = true;
        Ok(SERVER_PATH.to_string())
    }
}

impl zed::Extension for QuintExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // 1. PATH 上にインストール済みのバイナリを優先して使う
        if let Some(path) = worktree.which("quint-language-server") {
            return Ok(zed::Command {
                command: path,
                args: vec!["--stdio".to_string()],
                env: worktree.shell_env(),
            });
        }

        // 2. 見つからなければ npm で管理された server.js を node で起動（自動DL）
        let server_path = self.server_script_path(id)?;
        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                std::env::current_dir()
                    .unwrap()
                    .join(&server_path)
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(QuintExtension);
