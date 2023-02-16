# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - YYYY-MM-DD
### Added
 - Now keeping a changelog!
### Changed
 - sun_times now returns an Option. This will be None if the date is not representable in chrono (~5M years from now), or sunsets/rises cannot be calculated due to long arctic/antarctic day/night (outside ~±67° of latitude)
### Fixed
 - Fixed ["day ahead" bug](https://github.com/Eroc33/sun-times/issues/1)