use std::fs;

use zed_extension_api::{self as zed, settings::LspSettings};

struct MojoSdk {
    env_root: String,
    export_sdk_env: bool,
    lsp_path: String,
    lsp_args: Vec<String>,
    lsp_env: Vec<(String, String)>,
    dap_path: String,
    mojo_path: String,
    lldb_plugin_path: String,
    lldb_visualizers_path: String,
}

struct MojoExtension;

impl MojoExtension {
    fn directory_exists(path: &str) -> bool {
        fs::metadata(path).map_or(false, |stat| stat.is_dir())
    }

    fn library_extension() -> &'static str {
        let (os, _) = zed::current_platform();
        match os {
            zed::Os::Mac => "dylib",
            zed::Os::Linux | _ => "so",
        }
    }

    /// Read the user-configured `mojo_sdk_path` from Zed settings, if any.
    fn configured_sdk_path(worktree: &zed::Worktree) -> Option<String> {
        let lsp_settings = LspSettings::for_worktree("mojo-lsp-server", worktree).ok()?;
        let settings = lsp_settings.settings?;
        settings
            .get("mojo_sdk_path")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    /// Read the user-configured LSP binary path override, if any.
    fn configured_binary_path(worktree: &zed::Worktree) -> Option<String> {
        let lsp_settings = LspSettings::for_worktree("mojo-lsp-server", worktree).ok()?;
        lsp_settings.binary.and_then(|b| b.path)
    }

    /// Read arguments configured for the language-server binary.
    fn configured_binary_arguments(worktree: &zed::Worktree) -> Vec<String> {
        LspSettings::for_worktree("mojo-lsp-server", worktree)
            .ok()
            .and_then(|settings| settings.binary)
            .and_then(|binary| binary.arguments)
            .unwrap_or_default()
    }

    /// Read environment variables configured for the language-server binary.
    fn configured_binary_env(worktree: &zed::Worktree) -> Vec<(String, String)> {
        LspSettings::for_worktree("mojo-lsp-server", worktree)
            .ok()
            .and_then(|settings| settings.binary)
            .and_then(|binary| binary.env)
            .map(|env| env.into_iter().collect())
            .unwrap_or_default()
    }

    /// Zed accepts a worktree-relative binary path in project settings, while
    /// the extension command must contain an executable path. Leave command
    /// names alone so they continue to resolve through PATH.
    fn resolve_binary_path(path: String, worktree: &zed::Worktree) -> String {
        if path.starts_with('/') || !path.contains('/') {
            path
        } else {
            format!("{}/{}", worktree.root_path(), path)
        }
    }

    fn sdk_root_from_binary_path(path: &str) -> Option<String> {
        let root = path.rfind("/bin/").map(|index| path[..index].to_string())?;

        // `pixi global install` puts standalone tools in ~/.pixi/bin. That
        // directory is not a Mojo SDK environment, so exporting it as
        // CONDA_PREFIX/MOJO_SDK_PATH would hide the actual compiler runtime.
        if root.ends_with("/.pixi") {
            None
        } else {
            Some(root)
        }
    }

    fn sdk(&self, worktree: &zed::Worktree) -> Result<MojoSdk, String> {
        let sdk_path_setting = Self::configured_sdk_path(worktree);
        let configured_lsp_args = Self::configured_binary_arguments(worktree);
        let configured_lsp_env = Self::configured_binary_env(worktree);
        let configured_lsp_path = Self::configured_binary_path(worktree)
            .map(|path| Self::resolve_binary_path(path, worktree));

        // If the user provided an explicit SDK path, derive all paths from it.
        if let Some(ref sdk_path) = sdk_path_setting {
            let lib_ext = Self::library_extension();

            let lsp_path = configured_lsp_path
                .clone()
                .unwrap_or_else(|| format!("{}/bin/mojo-lsp-server", sdk_path));
            let dap_path = format!("{}/bin/mojo-lldb-dap", sdk_path);
            let mojo_path = format!("{}/bin/mojo", sdk_path);
            let lldb_plugin_path = format!("{}/lib/libMojoLLDB.{}", sdk_path, lib_ext);
            let lldb_visualizers_path = format!("{}/lib/lldb-visualizers", sdk_path);

            return Ok(MojoSdk {
                env_root: sdk_path.clone(),
                export_sdk_env: true,
                lsp_path,
                lsp_args: configured_lsp_args,
                lsp_env: configured_lsp_env,
                dap_path,
                mojo_path,
                lldb_plugin_path,
                lldb_visualizers_path,
            });
        }

        // Fall back to PATH-based discovery.
        let explicit_lsp_binary = configured_lsp_path.is_some();
        let mut env_root = configured_lsp_path
            .as_deref()
            .and_then(Self::sdk_root_from_binary_path);
        let export_sdk_env = !explicit_lsp_binary && env_root.is_none();

        // Try local project Pixi environment first.
        // Since WASI sandbox denies absolute/relative host filesystem access and ignores hidden directories
        // (like .pixi), we detect if the project is a Pixi project containing Mojo by checking its pixi.toml
        // or pyproject.toml configuration files.
        let mut local_env_exists = false;
        if let Ok(content) = worktree.read_text_file("pixi.toml") {
            if content.contains("mojo") {
                local_env_exists = true;
            }
        } else if let Ok(content) = worktree.read_text_file("pyproject.toml") {
            if content.contains("mojo") && content.contains("tool.pixi") {
                local_env_exists = true;
            }
        }

        if local_env_exists && env_root.is_none() {
            env_root = Some(format!("{}/.pixi/envs/default", worktree.root_path()));
        }

        // Fall back to finding mojo in PATH
        if env_root.is_none() {
            if let Some(mojo_path) = worktree.which("mojo") {
                env_root = Self::sdk_root_from_binary_path(&mojo_path);
            }
        }

        let lsp_path = configured_lsp_path
            .clone()
            .or_else(|| {
                env_root.as_ref().map(|root| format!("{}/bin/mojo-lsp-server", root))
            })
            .or_else(|| worktree.which("mojo-lsp-server"))
            .ok_or_else(|| "Could not find mojo-lsp-server in PATH. Set \"mojo_sdk_path\" in your Zed settings under lsp.mojo-lsp-server.settings.".to_string())?;

        let dap_path = env_root.as_ref().map(|root| format!("{}/bin/mojo-lldb-dap", root))
            .or_else(|| worktree.which("mojo-lldb-dap"))
            .ok_or_else(|| "Could not find mojo-lldb-dap in PATH. Set \"mojo_sdk_path\" in your Zed settings under lsp.mojo-lsp-server.settings.".to_string())?;

        let mojo_path = env_root
            .as_ref()
            .map(|root| format!("{}/bin/mojo", root))
            .or_else(|| worktree.which("mojo"))
            .ok_or_else(|| "Could not find mojo in PATH".to_string())?;

        let lldb_plugin_path = if let Some(root) = &env_root {
            format!("{}/lib/libMojoLLDB.{}", root, Self::library_extension())
        } else {
            "".to_string()
        };

        let lldb_visualizers_path = if let Some(root) = &env_root {
            format!("{}/lib/lldb-visualizers", root)
        } else {
            "".to_string()
        };

        Ok(MojoSdk {
            env_root: env_root.unwrap_or_default(),
            export_sdk_env,
            lsp_path,
            lsp_args: configured_lsp_args,
            lsp_env: configured_lsp_env,
            dap_path,
            mojo_path,
            lldb_plugin_path,
            lldb_visualizers_path,
        })
    }

    fn lldb_has_python_scripting_support(_sdk: &MojoSdk) -> bool {
        // Assume false for now as standard lldb-dap interpreters might lack python scripting
        false
    }

    fn rewrite_mojo_launch_config(
        &mut self,
        config: &str,
        worktree: &zed::Worktree,
    ) -> Result<String, String> {
        let mut json = zed::serde_json::from_str::<zed::serde_json::Value>(config)
            .map_err(|err| err.to_string())?;

        let Some(object) = json.as_object_mut() else {
            return Ok(config.to_string());
        };

        let sdk = self.sdk(worktree)?;

        let Some(mojo_file) = object
            .get("mojoFile")
            .and_then(|value| value.as_str())
            .map(|value| value.to_string())
        else {
            return Ok(config.to_string());
        };

        let args = object
            .get("args")
            .and_then(|value| value.as_array())
            .map(|values| {
                values
                    .iter()
                    .filter_map(|value| value.as_str().map(|value| value.to_string()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let build_args = object
            .get("buildArgs")
            .and_then(|value| value.as_array())
            .map(|values| {
                values
                    .iter()
                    .filter_map(|value| value.as_str().map(|value| value.to_string()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let env = object
            .get("env")
            .and_then(|value| value.as_array())
            .cloned()
            .unwrap_or_else(|| {
                worktree
                    .shell_env()
                    .into_iter()
                    .map(|(key, value)| {
                        zed::serde_json::Value::String(format!("{}={}", key, value))
                    })
                    .collect::<Vec<_>>()
            });

        let mut full_env = vec![
            zed::serde_json::Value::String("LLDB_VSCODE_RIT_TIMEOUT_IN_MS=300000".to_string()),
            zed::serde_json::Value::String("MODULAR_TELEMETRY_ENABLED=false".to_string()),
        ];
        if sdk.export_sdk_env && !sdk.env_root.is_empty() {
            full_env.push(zed::serde_json::Value::String(format!(
                "CONDA_PREFIX={}",
                sdk.env_root
            )));
            full_env.push(zed::serde_json::Value::String(format!(
                "MOJO_SDK_PATH={}",
                sdk.env_root
            )));
            if sdk.env_root.contains(".pixi")
                || sdk.env_root.contains("conda")
                || sdk.env_root.contains("envs")
            {
                full_env.push(zed::serde_json::Value::String(format!(
                    "MODULAR_HOME={}/share/max",
                    sdk.env_root
                )));
            }
        }
        full_env.extend(env);

        object.insert(
            "program".to_string(),
            zed::serde_json::Value::String(sdk.mojo_path.clone()),
        );
        object.insert(
            "args".to_string(),
            zed::serde_json::Value::Array(
                std::iter::once(zed::serde_json::Value::String("run".to_string()))
                    .chain([
                        zed::serde_json::Value::String("--no-optimization".to_string()),
                        zed::serde_json::Value::String("--debug-level".to_string()),
                        zed::serde_json::Value::String("full".to_string()),
                    ])
                    .chain(build_args.into_iter().map(zed::serde_json::Value::String))
                    .chain(std::iter::once(zed::serde_json::Value::String(mojo_file)))
                    .chain(args.into_iter().map(zed::serde_json::Value::String))
                    .collect(),
            ),
        );
        object.insert("env".to_string(), zed::serde_json::Value::Array(full_env));
        object.insert(
            "debuggerRoot".to_string(),
            zed::serde_json::Value::String(worktree.root_path().to_string()),
        );
        object.insert(
            "type".to_string(),
            zed::serde_json::Value::String("mojo-lldb".to_string()),
        );

        if !object.contains_key("runInTerminal") {
            object.insert(
                "runInTerminal".to_string(),
                zed::serde_json::Value::Bool(false),
            );
        }

        if !object.contains_key("stopOnEntry") {
            object.insert(
                "stopOnEntry".to_string(),
                zed::serde_json::Value::Bool(false),
            );
        }

        if !object.contains_key("customFrameFormat") {
            object.insert(
                "customFrameFormat".to_string(),
                zed::serde_json::Value::String(
                    "${function.name-with-args}{${frame.is-artificial} [artificial]}".to_string(),
                ),
            );
        }

        if !object.contains_key("enableSyntheticChildDebugging") {
            object.insert(
                "enableSyntheticChildDebugging".to_string(),
                zed::serde_json::Value::Bool(true),
            );
        }

        if !object.contains_key("enableAutoVariableSummaries") {
            object.insert(
                "enableAutoVariableSummaries".to_string(),
                zed::serde_json::Value::Bool(true),
            );
        }

        if !object.contains_key("commandEscapePrefix") {
            object.insert(
                "commandEscapePrefix".to_string(),
                zed::serde_json::Value::String(":".to_string()),
            );
        }

        if !object.contains_key("timeout") {
            object.insert(
                "timeout".to_string(),
                zed::serde_json::Value::Number(300.into()),
            );
        }

        let mut init_commands = vec![
            zed::serde_json::Value::String(format!("?!plugin load '{}'", sdk.lldb_plugin_path)),
            zed::serde_json::Value::String(
                "?settings set target.show-hex-variable-values-with-leading-zeroes false"
                    .to_string(),
            ),
            zed::serde_json::Value::String(
                "?settings set target.process.optimization-warnings false".to_string(),
            ),
        ];

        if Self::lldb_has_python_scripting_support(&sdk)
            && Self::directory_exists(&sdk.lldb_visualizers_path)
        {
            if let Ok(entries) = fs::read_dir(&sdk.lldb_visualizers_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        init_commands.push(zed::serde_json::Value::String(format!(
                            "?command script import {}",
                            path.to_string_lossy()
                        )));
                    }
                }
            }
        }

        if let Some(existing) = object
            .get("initCommands")
            .and_then(|value| value.as_array())
        {
            init_commands.extend(existing.iter().cloned());
        }

        object.insert(
            "initCommands".to_string(),
            zed::serde_json::Value::Array(init_commands),
        );
        object.remove("buildArgs");
        object.remove("mojoFile");

        zed::serde_json::to_string(&json).map_err(|err| err.to_string())
    }
}

impl zed::Extension for MojoExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let sdk = self.sdk(worktree)?;
        let mut env = worktree.shell_env();
        env.push(("MODULAR_TELEMETRY_ENABLED".to_string(), "false".to_string()));

        // A configured binary path is authoritative. In particular, do not
        // let a parent Pixi/Conda shell make a Bazel mojo-lsp-server use a
        // different SDK than the one its binary belongs to. Apply explicit
        // binary.env settings below so users can still opt into an SDK env.
        if !sdk.export_sdk_env {
            env.retain(|(key, _)| {
                key != "CONDA_PREFIX" && key != "MOJO_SDK_PATH" && key != "MODULAR_HOME"
            });
        }

        for (key, value) in &sdk.lsp_env {
            if let Some((_, existing)) = env.iter_mut().find(|(name, _)| name == key) {
                *existing = value.clone();
            } else {
                env.push((key.clone(), value.clone()));
            }
        }

        if sdk.export_sdk_env && !sdk.env_root.is_empty() {
            env.push(("CONDA_PREFIX".to_string(), sdk.env_root.clone()));
            env.push(("MOJO_SDK_PATH".to_string(), sdk.env_root.clone()));

            if sdk.env_root.contains(".pixi")
                || sdk.env_root.contains("conda")
                || sdk.env_root.contains("envs")
            {
                env.push((
                    "MODULAR_HOME".to_string(),
                    format!("{}/share/max", sdk.env_root),
                ));
            }
        }

        if !sdk.env_root.is_empty() {
            let bin_dir = format!("{}/bin", sdk.env_root);
            // Prepend bin_dir to PATH so the LSP server can invoke 'mojo' helper tools
            let mut path_found = false;
            for (key, value) in env.iter_mut() {
                if key == "PATH" {
                    *value = format!("{}:{}", bin_dir, value);
                    path_found = true;
                    break;
                }
            }
            if !path_found {
                env.push(("PATH".to_string(), bin_dir));
            }
        }

        Ok(zed::Command {
            command: sdk.lsp_path,
            args: sdk.lsp_args,
            env,
        })
    }

    fn get_dap_binary(
        &mut self,
        _adapter_name: String,
        config: zed::DebugTaskDefinition,
        user_provided_debug_adapter_path: Option<String>,
        worktree: &zed::Worktree,
    ) -> Result<zed::DebugAdapterBinary, String> {
        let rewritten_config = self.rewrite_mojo_launch_config(&config.config, worktree)?;
        let sdk = self.sdk(worktree)?;
        let (command, mut arguments) = if let Some(path) = user_provided_debug_adapter_path {
            (path, Vec::new())
        } else {
            (sdk.dap_path.clone(), Vec::new())
        };

        arguments.push("--repl-mode".to_string());
        arguments.push("variable".to_string());

        let request =
            if let Ok(json) = zed::serde_json::from_str::<zed::serde_json::Value>(&config.config) {
                match json.get("request").and_then(|r| r.as_str()) {
                    Some("attach") => zed::StartDebuggingRequestArgumentsRequest::Attach,
                    _ => zed::StartDebuggingRequestArgumentsRequest::Launch,
                }
            } else {
                zed::StartDebuggingRequestArgumentsRequest::Launch
            };

        let mut dap_envs = vec![("MODULAR_TELEMETRY_ENABLED".to_string(), "false".to_string())];
        if sdk.export_sdk_env && !sdk.env_root.is_empty() {
            dap_envs.push(("CONDA_PREFIX".to_string(), sdk.env_root.clone()));
            dap_envs.push(("MOJO_SDK_PATH".to_string(), sdk.env_root.clone()));
            if sdk.env_root.contains(".pixi")
                || sdk.env_root.contains("conda")
                || sdk.env_root.contains("envs")
            {
                dap_envs.push((
                    "MODULAR_HOME".to_string(),
                    format!("{}/share/max", sdk.env_root),
                ));
            }
        }

        Ok(zed::DebugAdapterBinary {
            command: Some(command),
            arguments,
            envs: dap_envs,
            cwd: None,
            connection: None,
            request_args: zed::StartDebuggingRequestArguments {
                configuration: rewritten_config,
                request,
            },
        })
    }

    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        config: zed::serde_json::Value,
    ) -> Result<zed::StartDebuggingRequestArgumentsRequest, String> {
        match config.get("request").and_then(|r| r.as_str()) {
            Some("attach") => Ok(zed::StartDebuggingRequestArgumentsRequest::Attach),
            _ => Ok(zed::StartDebuggingRequestArgumentsRequest::Launch),
        }
    }

    fn dap_config_to_scenario(
        &mut self,
        config: zed::DebugConfig,
    ) -> Result<zed::DebugScenario, String> {
        let config_json = match config.request {
            zed::DebugRequest::Launch(launch) => {
                let mut json = zed::serde_json::json!({
                    "request": "launch",
                    "mojoFile": launch.program,
                    "args": launch.args,
                    "env": launch.envs.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>(),
                });
                if let Some(cwd) = launch.cwd {
                    json.as_object_mut()
                        .unwrap()
                        .insert("cwd".to_string(), zed::serde_json::Value::String(cwd));
                }
                json
            }
            zed::DebugRequest::Attach(attach) => {
                let mut json = zed::serde_json::json!({
                    "request": "attach",
                });
                if let Some(pid) = attach.process_id {
                    json.as_object_mut().unwrap().insert(
                        "pid".to_string(),
                        zed::serde_json::Value::Number(pid.into()),
                    );
                }
                json
            }
        };

        Ok(zed::DebugScenario {
            label: config.label,
            adapter: config.adapter,
            build: None,
            config: zed::serde_json::to_string(&config_json).unwrap(),
            tcp_connection: None,
        })
    }
}

zed::register_extension!(MojoExtension);
