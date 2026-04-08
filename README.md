# mint

3rd party mod integration tool for Deep Rock Galactic to download and integrate mods completely
externally of the game. This enables more stable mod usage as well as offline mod usage. Works for
both Steam and Microsoft Store versions.

Original repository: [mint by trumank](https://github.com/trumank/mint).

Alternative [mint-notag by Strappazzon](https://github.com/Strappazzon/drg-mint-notag).

When reporting any issues, look up existing similar ones in the original repo first. Right now my only goals
are:
- Maintain more or less updated dependencies.
- Remove tagging.
- Enable/tweak some minor already existing features.
- Compile distributed binary with stronger optimization.

Mods are added via URL to a .pak or .zip containing a .pak. Mods can also be pulled from mod.io.
Examples:

 - `C:\Path\To\Local\Mod.zip`
 - `https://example.org/some-online-mod-repository/public-mod.pak`
 - `https://mod.io/g/drg/m/sandbox-utilities`

For storing local mods more efficiently you can use 7-Zip:
- Compatible with orig. mint: `7z a -tzip -mx=9 -m0=Deflate:fb=258:pass=15 -mmt=off mod.zip mod.pak`
- Stronger **and** faster compression: `7z a -tzip -mx=9 -m0=LZMA:a=1:d=1g:mf=bt4:fb=273:mc=10000:lc=4 -mmt=off mod.zip mod.pak`

Compression strength compared:
- 31 pak files for total 43.1 MB
- 13.8 MB via Windows transparent NTFS compression (+211%).
- 4.23 MB via first command (+918% / +227%).
- 3.52 MB via second command (+1125% / +293% / +20%).

## Usage

This section assumes that you are on Windows and using the steam version of DRG,
working with either local `.pak`s or mod.io mods.

First, download the [latest release](https://github.com/Fedottt-Bo/mint-no-tag/releases/latest)
and choose the desired variant:
- `mint_no-tag.exe` with just server name tag removed.

Be aware - this version changes MODDING tab in the game menu.

Use original repository for the complete guide with illustrations.

## Building

I only plan supporting native Windows target (msvc).

To build the latest version, install LLVM support in Visual Studio,
download LLVM either manually or in Visual Studio and add it's `bin` to path.

Clang usage is set by `.cargo/config.toml`, you can also change target CPU there.

Last built with:
- rustc `1.96.0-nightly (c75612477 2026-04-07)`
- cargo `1.96.0-nightly (a357df4c2 2026-04-03)`
- Visual Studio 2026 `18.4.2`
- llvm `22.1.2`
