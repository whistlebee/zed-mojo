use std::fs;

use zed_extension_api as zed;

struct MojoSdk {
    env_root: String,
    lsp_path: String,
    dap_path: String,
    mojo_path: String,
    lldb_plugin_path: String,
    lldb_visualizers_path: String,
}

struct MojoExtension {
    cached_sdk: Option<MojoSdk>,
}

impl MojoExtension {
    fn binary_extension() -> &'static str {
        let (os, _) = zed::current_platform();
        match os {
            zed::Os::Windows => ".exe",
            zed::Os::Mac | zed::Os::Linux => "",
        }
    }

    fn directory_exists(path: &str) -> bool {
        fs::metadata(path).map_or(false, |stat| stat.is_dir())
    }

    fn library_extension() -> &'static str {
        let (os, _) = zed::current_platform();
        match os {
            zed::Os::Mac => "dylib",
            zed::Os::Linux => "so",
            zed::Os::Windows => "dll",
        }
    }

    fn sdk(&mut self, worktree: &zed::Worktree) -> Result<&MojoSdk, String> {
        if self.cached_sdk.is_some() {
            return Ok(self.cached_sdk.as_ref().unwrap());
        }

        let lsp_path = worktree
            .which("mojo-lsp-server")
            .ok_or_else(|| "Could not find mojo-lsp-server in PATH".to_string())?;

        let dap_path = worktree
            .which("mojo-lldb-dap")
            .ok_or_else(|| "Could not find mojo-lldb-dap in PATH".to_string())?;

        // Heuristic to find libMojoLLDB
        // If path is ~/.pixi/bin/mojo, the env might be ~/.pixi/envs/mojo
        // We will try to guess the env_root to construct lib path
        let mut env_root = None;
        if dap_path.contains("/.pixi/bin/") {
            let base = dap_path.split("/.pixi/bin/").next().unwrap();
            env_root = Some(format!("{}/.pixi/envs/mojo", base));
        } else if dap_path.contains("/.pixi/envs/") {
            if let Some(pos) = dap_path.find("/bin/") {
                env_root = Some(dap_path[..pos].to_string());
            }
        }

        let mojo_path = if let Some(root) = &env_root {
            format!("{}/bin/mojo{}", root, Self::binary_extension())
        } else {
            worktree
                .which("mojo")
                .ok_or_else(|| "Could not find mojo in PATH".to_string())?
        };

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

        self.cached_sdk = Some(MojoSdk {
            env_root: env_root.unwrap_or_default(),
            lsp_path,
            dap_path,
            mojo_path,
            lldb_plugin_path,
            lldb_visualizers_path,
        });

        Ok(self.cached_sdk.as_ref().unwrap())
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
        if !sdk.env_root.is_empty() {
            full_env.push(zed::serde_json::Value::String(format!(
                "CONDA_PREFIX={}",
                sdk.env_root
            )));
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

        if Self::lldb_has_python_scripting_support(sdk)
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

    fn language_server_binary_path(&mut self, worktree: &zed::Worktree) -> zed::Result<String> {
        self.sdk(worktree)
            .map(|sdk| sdk.lsp_path.clone())
            .map_err(Into::into)
    }
}

impl zed::Extension for MojoExtension {
    fn new() -> Self {
        Self { cached_sdk: None }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let mut env = worktree.shell_env();
        env.push(("MODULAR_TELEMETRY_ENABLED".to_string(), "false".to_string()));

        Ok(zed::Command {
            command: self.language_server_binary_path(worktree)?,
            args: vec![],
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

        Ok(zed::DebugAdapterBinary {
            command: Some(command),
            arguments,
            envs: vec![
                ("MODULAR_TELEMETRY_ENABLED".to_string(), "false".to_string()),
                ("CONDA_PREFIX".to_string(), sdk.env_root.clone()),
            ],
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
