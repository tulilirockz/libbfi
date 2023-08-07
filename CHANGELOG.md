# Changelog

## [1.0.2](https://github.com/tulilirockz/rBFI/compare/v1.0.1...v1.0.2) (2023-07-31)


### Bug Fixes

* **gh:** use rust-specific release action ([36c24da](https://github.com/tulilirockz/rBFI/commit/36c24da9bfe37b25cc1889933ccbe29e812bfdf0))

## [1.0.1](https://github.com/tulilirockz/rBFI/compare/v1.0.0...v1.0.1) (2023-07-31)


### Bug Fixes

* **gh:** generate shasum at proper place ([499d6bc](https://github.com/tulilirockz/rBFI/commit/499d6bc3cf7b303d88830c5a643a563d09f656c3))
* **gh:** SHA256SUM + build properly uploads now! ([f978e5e](https://github.com/tulilirockz/rBFI/commit/f978e5e89e579cd8d1ea531379b0b5beff6cf261))

## [1.2.0](https://github.com/tulilirockz/rbfi/releases/tag/v1.2.0) - 2023-07-31

### Added
- *(release)* add github releases package action
- add nix and melange package formats + gh action
- static memory in order to be more faithful to initial implementation

### Fixed
- *(gh)* use rust-specific release action
- *(gh)* SHA256SUM + build properly uploads now!
- *(gh)* generate shasum at proper place
- run nix release only if release-please makes release
- fix copyright license and holder

### Other
- *(develop)* release 1.0.1
- *(gh)* install gh cli properly
- test out nix build in separate nix action
- *(develop)* release 1.0.0
- BSD 3 license
- release action and gh metadata
- [**breaking**] modular design for brainfuck interpreter
- better variable naming and less variables in main func
- proper metadata naming
- separate functions for matching brackets
- Fix matching_bracket not working at all (add redundant code tho.)
- Fixed identation and comments
- Merge branch 'develop' of https://github.com/Pranprest/brainfuck-interpreter into develop
- Fixed README.md not being a markdown file
- Remove redundancy in matching_brackets and rearranged use statements
- Project metadata, license and readme
- added main code - first interpretation
- added sample brainfk scripts

## [1.2.0](https://github.com/tulilirockz/rBFI/releases/tag/v1.2.0) - 2023-07-31

### Added
- *(release)* add github releases package action
- add nix and melange package formats + gh action
- static memory in order to be more faithful to initial implementation

### Fixed
- implement release-plz
- *(gh)* move to release-plz action
- *(gh)* use rust-specific release action
- *(gh)* SHA256SUM + build properly uploads now!
- *(gh)* generate shasum at proper place
- run nix release only if release-please makes release
- fix copyright license and holder

### Other
- *(typo)* typo in license field
- add metadata to publish crate to crates.io
- *(develop)* release 1.0.2
- *(develop)* release 1.0.1
- *(gh)* install gh cli properly
- test out nix build in separate nix action
- *(develop)* release 1.0.0
- BSD 3 license
- release action and gh metadata
- [**breaking**] modular design for brainfuck interpreter
- better variable naming and less variables in main func
- proper metadata naming
- separate functions for matching brackets
- Fix matching_bracket not working at all (add redundant code tho.)
- Fixed identation and comments
- Merge branch 'develop' of https://github.com/Pranprest/brainfuck-interpreter into develop
- Fixed README.md not being a markdown file
- Remove redundancy in matching_brackets and rearranged use statements
- Project metadata, license and readme
- added main code - first interpretation
- added sample brainfk scripts

## [1.2.1](https://codeberg.org/tulilirockz/RBFI/compare/v1.2.0...v1.2.1) - 2023-08-03

### Other
- separate functions and everything onto separate traits and structs
- release

## [2.0.0](https://github.com/tulilirockz/libbfi/compare/v1.2.1...v2.0.0) - 2023-08-06

### Added
- [**breaking**] use tokens and macros for generic parsing
- [**breaking**] use tokens and macros for generic parsing

### Other
- move project to github
- join reversed and regular matching functions

## [3.0.0](https://github.com/tulilirockz/LibBFI/compare/v2.0.0...v3.0.0) - 2023-08-07

### Added
- add support for custom brainfuck dialects
- add blub and docs for all suported langs
- session type-like implementation for any brainfuck derivation + respective docs

### Fixed
- make cutoff in multi_char_intruction languages actually work

### Other
- add docsrs documentation and module docs
- *(release)* re-add release-plz to gh workflows

## 1.0.0 (2023-07-31)


### ⚠ BREAKING CHANGES

* modular design for brainfuck interpreter

### Features

* add nix and melange package formats + gh action ([e65340b](https://github.com/tulilirockz/rBFI/commit/e65340bcf7bc9d54bb7c1ec487cc3c5d53e45b56))
* **release:** add github releases package action ([347feaa](https://github.com/tulilirockz/rBFI/commit/347feaa20e42c3bfb1dcec90212d11a657df4512))
* static memory in order to be more faithful to initial implementation ([c9322b4](https://github.com/tulilirockz/rBFI/commit/c9322b4e26daa9503b3c0a1579fdc44f17f6572f))


### Bug Fixes

* run nix release only if release-please makes release ([fd21ded](https://github.com/tulilirockz/rBFI/commit/fd21ded8dfb5e0d6371f9286586bdb85295a7bba))


### Code Refactoring

* modular design for brainfuck interpreter ([c181d5d](https://github.com/tulilirockz/rBFI/commit/c181d5d7aaace92c2de808cd96fcb2838f91d38e))
