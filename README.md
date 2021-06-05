# alias-rs

A simple program that executes commands via alaiases.

## Quick Start
Compile the program:
```
$ cargo run --release
```

Create `aliases.json`:
```
{
    "wsl": "wsl.exe ~",
    "conda": "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe -ExecutionPolicy ByPass -NoExit -Command \"& 'C:\\Users\\User\\miniconda3\\shell\\condabin\\conda-hook.ps1' ; conda activate 'C:\\Users\\User\\miniconda3' \""
}
```

The program `alias.exe` must be in the same directory as `aliases.json`:
| Mode | LastWriteTime | Length | Name |
| ---- | ------------- | ------ | ---- |
| -a---- | 5/06/2021 4:22 PM | 300032 | alias.exe |
| -a---- | 5/06/2021 4:07 PM | 264 | aliases.json |

The program takes only a single argument as the alias:
```
.\alias.exe wsl
dev@DESKTOP-7LLUJG2:~$
```

## Why?
I didn't want to mess around with Windows registry keys and batch files. I want a single json file that contained my required aliases in a structured and readable format (very simple to a `~/.bashrc` file).