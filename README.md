# tiktok-desktop-video-fixer (tdvf)
Fixes issues with certain tiktoks where the video and audio playback are broken on desktop.
## Authors
- **Patrick Sullivan** - [beewyka819](https://github.com/beewyka819)

## Getting Started
These instructions will get tdvf up and running on your local machine.
### Installing
1. Head over to the [Releases](https://github.com/beewyka819/tiktok-desktop-video-fixer/releases) page and download the appropriate file for your system.
2. Extract the archive to a location of your choosing. I recommend keeping the exe contained within its own folder as it will download a local copy of ffmpeg on first run.
### Running the application
Using tdvf is very straightforward.
1. Start by running the executable.
2. In the new window, enter the input file name you wish to fix. The file must exist in the same directory the exe is in.
3. Enter the desired output file name. This file will be created in the same directory as the exe.

You can optionally add tdvf to your PATH, allowing you to run it via the commandline from any directory, in which case the input and output files will be relative to the directory your commandline is in, rather than the tdvf directory. At this time tdvf does not support commandline arguments, so the inputs must still be provided manually.

## License
This work is dual-licensed under [MIT](https://mit-license.org/) and [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0.html)

You can choose between one of them if you use this work.

`SPDX-License-Identifier: MIT OR Apache-2.0`
