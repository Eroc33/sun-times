# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2023-02-16
### Added
 - Now keeping a changelog!
 - altitude function, as a starting point to support arctic regions
 - plot example which uses the altitude function to show a plot of sun up/sun down times
### Changed
 - sun_times now returns an Option. This will be None if the date is not representable in chrono (~5M years from now), or sunsets/rises cannot be calculated due to long arctic/antarctic day/night (outside ~±67° of latitude)
 - sun_times takes a NaiveDate, as chrono's Date<T> has been deprecated
### Fixed
 - Fixed ["day ahead" bug](https://github.com/Eroc33/sun-times/issues/1)
 - Fixed ["negative altitudes" bug](https://github.com/Eroc33/sun-times/issues/2)