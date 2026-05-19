# Cargo-Solar

The Solar framework is a set of tools for building rust projects. My goal for the design of this framework and this cargo command tool is to allow a user to be able to build a whole project from scratch using all my favorite tools.

This project is mainly for me, and this tool is mainly meant to make it easy for me to build projects with all of my custom made tools that I like to use without having to manually update them each time I make changes to them. This let's me make quick bug fixes that I can release quickly for certain tools and update them on the fly for any of my projects. I'll define out the full requirements for this tool later when its not midnight and i'm not literally falling asleep at the computer. But for now, need to get thoughts on paper (or file, technically).

Also, it's called "solar" because I think it sounds cool, and I needed a name to use as the command for cargo.

Need this tool to do a couple things:
- Be able to initialize new projects with my favorite tools and custom made tools for linting commits, semantic version releases, and eventually tools for other stuff that I don't care to mention right now because they are written down elsewhere.
- Be able to create new projects with a name and calls initialization.
- Be able to update the tools via a command that updates all of my tools to their latest version without breaking anything.

## License

This work is dual-licensed under MIT and Apache 2.0.
You can choose between one of them if you use this work.

`SPDX-License-Identifier: MIT OR Apache-2.0`

Some dependencies may also be licensed under Unicode-3.0, so the license text for this license is included in the LICENSES directory to comply with the terms of this license.