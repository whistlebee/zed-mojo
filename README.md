# zed-mojo

A Zed extension for the Mojo programming language.

## Features

- **v1.0.0b1 syntax support** - full grammar coverage from `comptime` to ownership keywords and more, powered by [`tree-sitter-mojo`](https://github.com/whistlebee/tree-sitter-mojo)
- **Debugger support** - launch, set breakpoints, step through via `mojo-lldb` adapter
- **Proper indents & folding** - code folds smartly, indent guides follow the language structure
- **Language server integration** - diagnostics, go-to-definition, hover, completions via `mojo-lsp-server`

Often provides better syntax highlighting than the official VSCode Mojo extension.

| Zed Mojo | VSCode Mojo |
| :---: | :---: |
| ![zed-mojo](images/zed-mojo.jpg) | ![vscode-mojo](images/vscode.jpg) |

## Installation

This extension is not yet published in the Zed extension store. To install it manually:

Clone this repository, then open Zed, open the command palette, select `zed: install dev extension`, and pick the folder containing this code.

## Language Server

The extension looks for `mojo-lsp-server` in `PATH` and in the project's `.pixi` directory. If the server is not found, an error message will prompt you to install Mojo.

## Configuration

You can configure the Mojo SDK path in your Zed `settings.json`. This is useful when the Mojo SDK is not on your `PATH`, or you want to use a specific SDK installation.

### Setting a custom SDK path

Add the following to your `settings.json` (or project-level `.zed/settings.json`):

```json
{
  "lsp": {
    "mojo-lsp-server": {
      "settings": {
        "mojo_sdk_path": "/path/to/mojo/sdk"
      }
    }
  }
}
```

When `mojo_sdk_path` is set, all Mojo binaries (`mojo-lsp-server`, `mojo-lldb-dap`, `mojo`, etc.) are resolved relative to this path. If it is not set, the extension falls back to searching your `PATH`.

### Overriding just the LSP binary

If you only need to override the language server binary path:

```json
{
  "lsp": {
    "mojo-lsp-server": {
      "binary": {
        "path": "/custom/path/to/mojo-lsp-server"
      }
    }
  }
}
```

## Formatting

With a saved Mojo file open, run `task: spawn` and select
**Mojo: Format current file**. The task saves the current buffer, runs
`mojo format` on that file, and hides the terminal on success. Errors remain
visible in the task terminal. Reload/rebuild the dev extension after updating
it to load the new task.

The task uses `mojo` from your shell's `PATH`. Zed language tasks do not inherit
the extension's `lsp.mojo-lsp-server.settings.mojo_sdk_path` setting. For a custom
SDK, add this task to your project's `.zed/tasks.json`, replacing the command
with the path to your Mojo executable:

```json
[
  {
    "label": "Mojo: Format current file",
    "command": "\"/path/to/mojo/sdk/bin/mojo\"",
    "args": ["format", "\"$ZED_FILE\""],
    "cwd": "$ZED_WORKTREE_ROOT",
    "save": "current",
    "allow_concurrent_runs": false,
    "reveal": "no_focus",
    "hide": "on_success"
  }
]
```

For a Pixi environment, use `"command": "pixi"` and
`"args": ["run", "mojo", "format", "\"$ZED_FILE\""]` instead.
The Mojo installation must include its formatter; compiler-only Bazel
toolchains can report `unable to resolve Mojo formatter in PATH`.

To bind the task to a shortcut, add an entry to your Zed `keymap.json`:

```json
[
  {
    "context": "Editor && language == Mojo",
    "bindings": {
      "alt-shift-f": [
        "task::Spawn",
        { "task_name": "Mojo: Format current file" }
      ]
    }
  }
]
```

## Organizing imports

Install the native `moff` binary on your shell's `PATH`. Start **Mojo: Start
import worker** once per workspace, then run **Mojo: Organize imports** from
Zed's task picker. The worker keeps the Mojo runtime and organizer loaded; the
organize task saves the current buffer and sends the path through the workspace's
`.zed/moff.in` and `.zed/moff.out` FIFOs. Its terminal remains visible on
failure. Import organization and `mojo format` are separate tasks; neither runs
on save. If the worker is not running, the organize task reports that directly
instead of waiting indefinitely.

Override the task in your project's `.zed/tasks.json` to set source roots or
package classifications:

```json
[
  {
    "label": "Mojo: Organize imports",
    "command": "moff",
    "args": ["request", "--input", "\"$ZED_WORKTREE_ROOT/.zed/moff.in\"", "--output", "\"$ZED_WORKTREE_ROOT/.zed/moff.out\"", "--fix", "\"$ZED_FILE\""],
    "cwd": "$ZED_WORKTREE_ROOT",
    "save": "current",
    "allow_concurrent_runs": false,
    "reveal": "no_focus",
    "hide": "on_success"
  }
]
```

Pass `--src`, `--known-first-party`, and `--known-third-party` on the
**Mojo: Start import worker** task when a project needs explicit package
classification. The one-shot CLI remains available as `moff check --fix PATH`
for scripts and projects that do not keep a worker task running.

## Debugging

To configure debugging for a Mojo project, create a `.zed/debug.json` file in the project root with the following contents:

```json
[
  {
    "label": "Mojo: Debug basic example",
    "adapter": "mojo-lldb",
    "request": "launch",
    "cwd": "$ZED_WORKTREE_ROOT",
    "mojoFile": "$ZED_WORKTREE_ROOT/examples/debug_basic.mojo",
    "args": []
  }
]
```

Open `examples/debug_basic.mojo`, run `debugger: start` in Zed, and select the "Mojo: Debug basic example" configuration. Set breakpoints and step through your code. Adjust the filename and arguments for your own project.
