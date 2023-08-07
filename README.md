# wslgitcompat
A compatibility layer for applications that use Git, this can route those through WSL.

For example, Obsidian Git allows you specify a `GIT EXECUTABLE`. You can point that to the directory of `wslgitcompat.exe` and it will run `wsl.exe git $args` but also translate paths from Windows to WSL and back and vice versa.
