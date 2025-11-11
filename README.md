# Tiktok Desktop Video Fixer (tdvf)
Tiktok Desktop Video Fixer (AKA tdvf) fixes issues with certain tiktoks where the video and audio playback are broken on desktop.

Specifically, many tiktoks play at seemingly half speed on desktop (though audio is unaffected, but loops midway through). Notably, mobile playback is unaffected. I suspect this is some kind of strange encoding issue. Upon downloading the mp4, one will find that the audio is then also messed up on local playback. Simply throwing the video into an editor and speeding it up 2x will not fix the problem. This utility however fixes this playback issue with some FFmpeg magic.
## Getting Started
These instructions will get tdvf up and running on your local machine.
### Installing FFmpeg
[FFmpeg](http://ffmpeg.org) is the core of this utility, and thus is required for it to run. At the moment, the utility requires a local copy of ffmpeg be installed specifically for the application, however it will download and install automatically if it is not already present. I might revisit this in the future to support pre-existing FFmpeg installs should it be present in the PATH.
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
