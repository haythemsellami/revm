# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/haythemsellami/revm/releases/tag/monad-revm-v0.1.0) - 2026-01-16

### Added

- *(precompile)* rug/gmp-based modexp ([#2596](https://github.com/haythemsellami/revm/pull/2596))
- transact multi tx ([#2517](https://github.com/haythemsellami/revm/pull/2517))
- *(docs)* MyEvm example and book cleanup ([#2218](https://github.com/haythemsellami/revm/pull/2218))
- remove specification crate ([#2165](https://github.com/haythemsellami/revm/pull/2165))
- book structure ([#2082](https://github.com/haythemsellami/revm/pull/2082))
- *(examples)* generate block traces ([#895](https://github.com/haythemsellami/revm/pull/895))
- implement EIP-4844 ([#668](https://github.com/haythemsellami/revm/pull/668))
- *(Shanghai)* All EIPs: push0, warm coinbase, limit/measure initcode ([#376](https://github.com/haythemsellami/revm/pull/376))
- Migrate `primitive_types::U256` to `ruint::Uint<256, 4>` ([#239](https://github.com/haythemsellami/revm/pull/239))
- Introduce ByteCode format, Update Readme ([#156](https://github.com/haythemsellami/revm/pull/156))

### Fixed

- Apply spelling corrections from PRs #2926, #2915, #2908 ([#2978](https://github.com/haythemsellami/revm/pull/2978))
- fix typo and update links ([#2387](https://github.com/haythemsellami/revm/pull/2387))
- fix typos ([#620](https://github.com/haythemsellami/revm/pull/620))

### Other

- tests
- Enable optional_no_base_fee by default in monad-revm
- merge with upstream a31fa98efe2208af326cf70f59d374165e0df363 and update GasParams usage
- Merge commit 'a31fa98efe2208af326cf70f59d374165e0df363' into merge-a31fa98efe2208af326cf70f59d374165e0df363
- fix typos, grammar errors, and improve documentation consistency ([#3294](https://github.com/haythemsellami/revm/pull/3294))
- add boundless ([#3043](https://github.com/haythemsellami/revm/pull/3043))
- add SECURITY.md ([#2956](https://github.com/haythemsellami/revm/pull/2956))
- update README.md ([#2842](https://github.com/haythemsellami/revm/pull/2842))
- add rust-version and note about MSRV ([#2789](https://github.com/haythemsellami/revm/pull/2789))
- make crates.io version badge clickable ([#2526](https://github.com/haythemsellami/revm/pull/2526))
- copy edit The Book ([#2463](https://github.com/haythemsellami/revm/pull/2463))
- bump dependency version ([#2431](https://github.com/haythemsellami/revm/pull/2431))
- fixed broken link ([#2421](https://github.com/haythemsellami/revm/pull/2421))
- links to main readme ([#2298](https://github.com/haythemsellami/revm/pull/2298))
- add links to arch page ([#2297](https://github.com/haythemsellami/revm/pull/2297))
- tag v63 revm v20.0.0-alpha.6 ([#2219](https://github.com/haythemsellami/revm/pull/2219))
- rename revm-optimism to op-revm ([#2141](https://github.com/haythemsellami/revm/pull/2141))
- fix README link ([#2139](https://github.com/haythemsellami/revm/pull/2139))
- *(readme)* add tycho-simulation to "Used by" ([#1926](https://github.com/haythemsellami/revm/pull/1926))
- Update README.md examples section ([#1853](https://github.com/haythemsellami/revm/pull/1853))
- Bump new logo ([#1735](https://github.com/haythemsellami/revm/pull/1735))
- *(README)* add rbuilder to used-by ([#1585](https://github.com/haythemsellami/revm/pull/1585))
- added simular to used-by ([#1521](https://github.com/haythemsellami/revm/pull/1521))
- add Trin to used by list ([#1393](https://github.com/haythemsellami/revm/pull/1393))
- Fix typo in readme ([#1185](https://github.com/haythemsellami/revm/pull/1185))
- Add Hardhat to the "Used by" list ([#1164](https://github.com/haythemsellami/revm/pull/1164))
- Add VERBS to used by list ([#1141](https://github.com/haythemsellami/revm/pull/1141))
- license date and revm docs ([#1080](https://github.com/haythemsellami/revm/pull/1080))
- *(docs)* Update the benchmark docs to point to revm package ([#906](https://github.com/haythemsellami/revm/pull/906))
- *(docs)* Update top-level benchmark docs ([#894](https://github.com/haythemsellami/revm/pull/894))
- clang requirement ([#784](https://github.com/haythemsellami/revm/pull/784))
- Readme Updates ([#756](https://github.com/haythemsellami/revm/pull/756))
- Logo ([#743](https://github.com/haythemsellami/revm/pull/743))
- book workflow ([#537](https://github.com/haythemsellami/revm/pull/537))
- add example to revm crate ([#468](https://github.com/haythemsellami/revm/pull/468))
- Update README.md ([#424](https://github.com/haythemsellami/revm/pull/424))
- add no_std to primitives ([#366](https://github.com/haythemsellami/revm/pull/366))
- revm-precompiles to revm-precompile
- Bump v20, changelog ([#350](https://github.com/haythemsellami/revm/pull/350))
- typos ([#232](https://github.com/haythemsellami/revm/pull/232))
- Add support for old forks. ([#191](https://github.com/haythemsellami/revm/pull/191))
- revm bump 1.8. update libs. snailtracer rename ([#159](https://github.com/haythemsellami/revm/pull/159))
- typo fixes
- fix readme typo
- Big Refactor. Machine to Interpreter. refactor instructions. call/create struct ([#52](https://github.com/haythemsellami/revm/pull/52))
- readme. debuger update
- Bump revm v0.3.0. README updated
- readme
- Add time elapsed for tests
- readme updated
- Include Basefee into cost calc. readme change
- Initialize precompile accounts
- Status update. Taking a break
- Merkle calc. Tweaks and debugging for eip158
- Replace aurora bn lib with parity's. All Bn128Add/Mul/Pair tests passes
- TEMP
- one tab removed
- readme
- README Example simplified
- Gas calculation for Call/Create. Example Added
- readme usage
- README changes
- Static gas cost added
- Subroutine changelogs and reverts
- Readme postulates
- Spelling
- Restructure project
- First iteration. Machine is looking okay
