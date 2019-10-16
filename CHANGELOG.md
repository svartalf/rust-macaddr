# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `MacAddr6::nil` and `MacAddr8::nil` methods to create new nil MAC addresses
- `MacAddr6::broadcast` and `MacAddr8::broadcast` methods to create new nil MAC addresses

### Fixed

- `std::fmt::Display` implementation for `MacAddr8` properly renders address in a canonical form (#1)
