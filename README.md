# mint

3rd party mod integration tool for Deep Rock Galactic to download and integrate mods completely
externally of the game. This enables more stable mod usage as well as offline mod usage. Works for
both Steam and Microsoft Store versions.

Original repository: [mint by trumank](https://github.com/trumank/mint).

Alternatives:
- No tag:
  - [mint-notag by Strappazzon](https://github.com/Strappazzon/drg-mint-notag) — based on older stable version of mint, actively maintained.
  - [mint-notag by modcrafts](https://github.com/modcrafts/mint-notag) — currently seems to be abandoned.
- Tagged:
  - [mintfixed by Wasserkleber](https://github.com/Wasserkleber/mintfixed) — simple fork with up-to-date fixes.

When reporting any issues, look up existing similar ones in the original repo first, then in mentioned alternatives. Right now my only goals are:
- Maintain more or less updated dependencies.
- Enable compression-related features: oodle compression, heavier pak file compression, better support for archived mod files.
- Compile distributed binary with stronger optimization.

Mods are added via URL to a .pak or .zip containing a .pak. Mods can also be pulled from mod.io. Examples:
 - `C:\Path\To\Local\Mod.zip`
 - `https://example.org/some-online-mod-repository/public-mod.pak`
 - `https://mod.io/g/drg/m/sandbox-utilities`

For storing local mods more efficiently you can use 7-Zip:
- Compatible with original mint and other forks: `7z a -tzip -mx=9 -m0=Deflate:fb=258:pass=15 -mmt=off mod.zip mod.pak`
- Stronger **and** faster compression: `7z a -tzip -mx=9 -m0=LZMA:a=1:d=1g:mf=bt4:fb=273:mc=10000:lc=4 -mmt=off mod.zip mod.pak`

Compression strength compared:
- 31 pak files for total 43.1 MB
- 13.8 MB via Windows transparent NTFS compression (+211%).
- 4.23 MB via first command (+918% / +227%).
- 3.52 MB via second command (+1125% / +293% / +20%).

## Usage

This section assumes that you are on Windows and using the steam version of DRG,
working with either local `.pak`s or mod.io mods.

First, download the [latest release](https://github.com/Fedottt-Bo/mint-no-tag/releases/latest):
`mint_no-tag.exe`. Be aware - this version changes MODDING tab in the game menu.

~~Then use original repository for the complete guide with illustrations~~ —
partially outdated, as OAuth 2.0 tokens are replaced with OAuth Personal Access Tokens, but general steps should be similar.

## Building

I only plan supporting native Windows target (msvc).

To build the latest version, install LLVM support in Visual Studio,
download LLVM either manually or in Visual Studio and add it's `bin` to PATH.

Clang usage is set by `.cargo/config.toml`, you can also change target CPU there.

Last built with:
- rustc `1.100.0-nightly (787af2b8c 2026-08-25)`
- cargo `1.100.0-nightly (e8cb624d5 2026-08-22)`
- Visual Studio 2026 `18.9.2`
- llvm `23.1.0`
