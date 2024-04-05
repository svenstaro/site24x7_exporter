# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->

## [Unreleased] - ReleaseDate
- Update support of REST API monitors [#342](https://github.com/svenstaro/site24x7_exporter/pull/342) (thanks @AlKapkone)

## [1.0.1] - 2022-09-10
- Update deps
- Fix release process

## [1.0.0] - 2022-09-07
- Nothing changed in this release but I think this project is stable enough to call it 1.0.0. :)

## [0.6.1] - 2022-09-05
- Update deps

## [0.6.0] - 2021-04-18
- Add more locations to `/geolocation` endpoint [#111](https://github.com/svenstaro/site24x7_exporter/pull/111) (thanks @Whyeasy)

## [0.5.2] - 2021-04-05
- Use `www.` subdomain for querying API which should fix auth token errors in some circumstances (fixes [#3](https://github.com/svenstaro/site24x7_exporter/issues/3))

## [0.5.1] - 2021-01-08
- Change value for monitor that are down to +Infinity

## [0.5.0] - 2021-01-07
- Upgrade all deps to tokio 1.0
- Add proper tests (#74)
- Report NaN as value for monitors that are down
- Stop overwriting previous values with newer invalid values returned by Site24x7

## [0.4.4] - 2020-08-14
- Add more city locations
- Update deps

## [0.4.3] - 2020-07-08
- Add CORS headers to geolocation lookup route

## [0.4.2] - 2020-07-08
- Better logging

## [0.4.1] - 2020-07-07
- Fix datatypes of geolocations for Grafana compatibility

## [0.4.0] - 2020-07-07
- Add separate route to expose geographic information about the locations.

## [0.3.0] - 2020-07-06
- Use more robust way of getting and holding access token.

## [0.2.2] - 2020-07-04
- Show used proxies during startup.

## [0.2.1] - 2020-07-03
- Build Linux target using musl to ensure it's perfectly static.

## [0.2.0] - 2020-07-02
- Make sure deleted monitors are cleaned up properly (#4).
- Better logging and debugging (the latter with `--log.format debug`).

## [0.1.0] - 2020-06-24
- Initial working version.

<!-- next-url -->
[Unreleased]: https://github.com/svenstaro/site24x7_exporter/compare/v1.0.1...HEAD
[1.0.1]: https://github.com/svenstaro/site24x7_exporter/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/svenstaro/site24x7_exporter/compare/v0.6.1...v1.0.0
[0.6.1]: https://github.com/svenstaro/site24x7_exporter/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/svenstaro/site24x7_exporter/compare/v0.5.2...v0.6.0
[0.5.2]: https://github.com/svenstaro/site24x7_exporter/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/svenstaro/site24x7_exporter/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/svenstaro/site24x7_exporter/compare/v0.4.4...v0.5.0
[0.4.4]: https://github.com/svenstaro/site24x7_exporter/compare/v0.4.3...v0.4.4
[0.4.3]: https://github.com/svenstaro/site24x7_exporter/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/svenstaro/site24x7_exporter/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/svenstaro/site24x7_exporter/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/svenstaro/site24x7_exporter/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/svenstaro/site24x7_exporter/compare/0.2.2...v0.3.0
[0.2.2]: https://github.com/svenstaro/site24x7_exporter/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/svenstaro/site24x7_exporter/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/svenstaro/site24x7_exporter/compare/0.2.0...0.2.0
[0.1.0]: https://github.com/svenstaro/site24x7_exporter/compare/0aac075...0.1.0
