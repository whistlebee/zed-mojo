# zed-mojo

grug like zed editor. zed fast. grug like mojo language. mojo fast. grug want mojo in zed. so grug make extension.

this extension give zed two things:
1. shiny colors. uses `tree-sitter-mojo` grug made.
2. smart brain. talks to `mojo-lsp-server` so zed know what code mean.

## how install

grug not put in big zed extension store yet. must install by hand.

clone this repo. 

open zed. press buttons for command palette. type `zed: install dev extension`. pick folder where you put this code

## language server brain

extension look for `mojo-lsp-server` to be smart. 

it look in `PATH`. it look in project `.pixi` folder. if extension cannot find server, it cry and tell you to install mojo.

## debug bugs

code have bugs. grug write bugs all time. debugger help smash bugs.

to tell zed how to debug mojo, grug make `.zed/debug.json` file in project folder. put this magic text inside:

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

now open `examples/debug_basic.mojo`. run `debugger: start` in zed. pick "Mojo: Debug basic example". set breakpoint. watch code step. fix bug.

change filenames for your project.
