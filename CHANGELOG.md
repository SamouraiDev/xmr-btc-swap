# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- A changelog file.

### Fixed

- Make monero_wallet_rpc readiness check language agnostic. The readiness check was
  failing on non-english language systems preventing users from starting the swap-cli
  and asb.

### Security

- Fixed an issue where Alice would not verify if Bob's Bitcoin lock transaction is semantically correct, i.e. pays the agreed upon amount to an output owned by both of them.
  Fixing this required a **breaking change** on the network layer and hence old versions are not compatible with this version.

[Unreleased]: https://github.com/comit-network/xmr-btc-swap/compare/v0.3...HEAD