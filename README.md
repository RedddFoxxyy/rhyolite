<div align="center">
    <img src="./src-tauri/icons/icon.png" width=256 alt="rhyolite">
</div>

<h1 align="center">An Open Source Markdown Editor Inspired by Obsidian!</h1>

![Rhyolite Editor Preview GIF](assets/readme_gif.gif)
![Rhyolite Editor Preview](assets/Rhyolite_is_Cool!.png)
![Rhyolite Editor Preview Command Palette](assets/rhyolite_command_preview.png)

### [Rhyolite](https://rhyolite.xyz/) is a simple markdown editor build using tauri framework that allows user to edit and work on markdown files.

> Rhyolite is inspired by volcanic rocks, much like Obsidian. The project is my way to contribute and grow as a developer while sharing something meaningful with the open-source community.
>
> Rhyolite was a fork of [fextify](https://github.com/face-hh/fextify), but was later rewritten from scratch,
> using sveltekit for the frontend and tauri version was changed from v1 to v2. As of now this project does not share any similarities to the [fextify](https://github.com/face-hh/fextify) project.

## **Current Updates**

### Changes Underway

- Frontend is being worked on and new features are being added like improving markdown editor based on codemirror.
- Working on a adding support for more markdowns.
- Work on a theme and plugin system is in progress.

## For New Contributors

1. If you want to contribute to the app, you can work on any of the issues that have not been assigned to anyone.
2. Join our **[Discord server](https://discord.gg/K6FAd8FTma)** to collaborate effectively.

---

## How to Install?

### **Windows**

#### 64-bit Systems

- Use the `Rhyolite_[version]_x64_en-US.msi` or `Rhyolite_[version]_x64-setup.exe` installer from the [Releases section](https://github.com/RedddFoxxyy/Rhyolite/releases) or from the [official website](https://rhyolite.xyz/).

#### 32-bit Systems

- Use the `Rhyolite_[version]_x86_en-US.msi` or `Rhyolite_[version]_x86-setup.exe` installer installer from the [Releases section](https://github.com/RedddFoxxyy/Rhyolite/releases) or from the [official website](https://rhyolite.xyz/).

#### ARM64 Systems (Snapdragon processors)

- Use the `Rhyolite_[version]_arm64_en-US.msi` or `Rhyolite_[version]_arm64-setup.exe` installer from the [Releases section](https://github.com/RedddFoxxyy/Rhyolite/releases) or from the [official website](https://rhyolite.xyz/).

---

### **MacOS**

- Use the `Rhyolite_[version]_x64.dmg` for Intel Macs from the [Releases section](https://github.com/RedddFoxxyy/Rhyolite/releases) or from the [official website](https://rhyolite.xyz/).

- Use the `Rhyolite_[version]_aarch64.dmg` for M-series Macs from the [Releases section](https://github.com/RedddFoxxyy/Rhyolite/releases) or from the [official website](https://rhyolite.xyz/).

> **Note:** You may encounter issues since the app isn’t signed yet, like macos saying dmg is damaged.

---

### **Linux**

#### Universal Linux Installer

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

Make the file executable and run:

```bash
chmod +x Rhyolite_[version]_.AppImage
./Rhyolite_[version]_.AppImage
```

#### Flatpack

> Work In Progress!

---

### **Manual Compilation**

- Requires latest version of nodejs and npm.
- Linux users make sure you have rustc, lld(llvm) installed.
- Windows users make sure you have msvc and rustc installed.

- To build the app manually, run:

```bash
npm run tauri build
```

- For debugging, add `--verbose`:

```bash
npm run tauri build --verbose
```

> **Note:** You may face errors when using package manager other than NPM.

---

## First Startup

1. Run Rhyolite.exe or compile using npm run tauri build.
2. Open the Command Palette using CTRL + P.
3. Explore the features or refer to our Discord community for guidance.

## Known Bugs

1. Theming might cause visibility issues.
2. Tab icons occasionally glitch.
3. Large numbers of open tabs can distort the title.

---

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
