<div align="center">
    <img src="./src/static/svgs/icon.svg" width=256 alt="Rhyolite">
</div>

<h1 align="center">An Open Source and performant Markdown Editor and knowledge base!</h1>

![Rhyolite Editor Preview](assets/Rhyolite_is_Cool!.png)
![Rhyolite Editor Preview Command Palette](assets/rhyolite_command_preview.png)

#### [Rhyolite](https://rhyolite.xyz/) is a simple markdown editor build using Rust with Freya that allows user to edit and work on markdown files.

> Rhyolite is inspired by volcanic rock rhyolite, which is a type of igneous rock formed from the rapid cooling of lava.
>
> Rhyolite was a fork of [fextify](https://github.com/face-hh/fextify), but was later rewritten from scratch,
> using sveltekit for the frontend and tauri version was changed from v1 to v2. As of now this project does not share any similarities to the [fextify](https://github.com/face-hh/fextify) project.
>
> Update: The project is now completely written in rust, and is being rewritten to use the [FREYA GUI](https://freyaui.dev/) library for the UI, making the app completely native code with rust.

## **Current Updates**

- The app is being migrated from Tauri(Svelte) to Freya(Rust) based UI.(WIP).
- The [0.1.10-freeze](https://github.com/RedddFoxxyy/rhyolite/tree/0.1.10-freeze) branch has the old code for the old tauri version of the app which will not be maintained anymore.


## **For New Contributors**

1. If you want to contribute to the app, you can work on any of the issues that have not been assigned to anyone.
2. Join our **[Discord server](https://discord.gg/K6FAd8FTma)** to collaborate effectively.
3. Checkout the master Branch for latest commits to the app.


## How to Install?

### **Windows**

- Use the `Rhyolite_[version]_[cpu architecture]_en-US.msi` or `Rhyolite_[version]_[cpu architecture]-setup.exe` installer from the [Releases section](https://github.com/RedddFoxxyy/Rhyolite/releases) or from the [official website](https://rhyolite.xyz/).


### **MacOS**

- Use the `Rhyolite_[version]_[cpu architecture].dmg` for Intel Macs from the [Releases section](https://github.com/RedddFoxxyy/Rhyolite/releases) or from the [official website](https://rhyolite.xyz/).

> **Note:** You may encounter issues since the app isn’t signed yet, like macos saying dmg is damaged.


### **Linux**

#### Universal Linux Installer ( Use this if you use a rolling release distro or latest LTS distro )

Run this command in your terminal:

```bash
curl -f https://rhyolite.xyz/install.sh | sh
```

To Uninstall:

```bash
curl -f https://rhyolite.xyz/uninstall.sh | sh
```

#### Debian/Ubuntu

- Install the .deb package from the Releases section.

#### RHEL/Fedora

- Install the .rpm package from the same section.

#### AppImage/Raw Binary

Install `Rhyolite_[version]_.AppImage` using [Gear Lever](https://github.com/mijorus/gearlever) or [AppImageLauncher](https://github.com/TheAssassin/AppImageLauncher), you can also make it executable and run it directly.

#### Flatpak

> Work In Progress!


### **Manual Compilation**

- Linux/macOS users make sure you have rustc, ld/lld/mold, and gcc/clang installed.
- Windows users make sure you have msvc and rustc installed.

- Clone the repo and checkout release branch.

- To build the app manually, run:

```bash
cargo build --release
```


## First Startup

1. Open the Command Palette using CTRL + P.
2. Explore the features or refer to our Discord community for guidance.

## Known Bugs

1. Theming might cause visibility issues.
2. Tab icons occasionally glitch.
3. Large numbers of open tabs can distort the title.
4. The app is still alpha so many unkown/unregisterd bugs.

## Licensing

This project is licensed under the terms of the GPL-3.0 open source license. Please refer to [GPL-3.0](./LICENSE.txt) for the full terms.

```
Copyright (C) 2025  RedddFoxxyy(Suyog Tandel)

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```

## Maintainers

[@RedddFoxxyy](https://github.com/RedddFoxxyy)

## Thanks to all the contributers!!

- This project would have not been possible without the initial guidance and help of [@80avin](https://github.com/80avin), [@RaphGL](https://github.com/RaphGL) and [@prettyblueberry](https://github.com/prettyblueberry).
- Thanks to [marc2332](https://github.com/marc2332) for building [Freya](https://github.com/marc2332/freya) and helping me with using it for rhyolite
