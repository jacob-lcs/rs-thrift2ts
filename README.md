# `rs-thrift-gen-ts`

![https://github.com/jacob-lcs/rs-thrift2ts/actions](https://github.com/jacob-lcs/rs-thrift2ts/workflows/CI/badge.svg)

The thrift to typescript tool, written in rust.

# Usage

## Install package

```
# yarn
yarn add rs-thrift-gen-ts

# npm
npm install rs-thrift-gen-ts

# pnpm
pnpm add rs-thrift-gen-ts
```

## Generate ts code

```javascript
const { gen } = require('rs-thrift-gen-ts')

gen({
  path: 'path/to/thrift/file.thrift',
})
```

## Params

| Parameter         | Type             | Default Value | Description                                                                                                                   |
| ----------------- | ---------------- | ------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `ext_name`        | `Option<String>` | `.ts`         | The file extension for the generated file. The default value is `.ts`.                                                        |
| `map_as_object`   | `Option<bool>`   | `false`       | Whether to convert the `map` data type in Thrift to `Object` in JavaScript instead of `Map`. The default value is `false`.    |
| `set_as_array`    | `Option<bool>`   | `false`       | Whether to convert the `set` data type in Thrift to `Array` in JavaScript instead of `Set`. The default value is `false`.     |
| `int64_as_string` | `Option<bool>`   | `false`       | Whether to convert the `i64` data type in Thrift to `string` in JavaScript instead of `bigint`. The default value is `false`. |
| `int64_as_number` | `Option<bool>`   | `false`       | Whether to convert the `i64` data type in Thrift to `number` in JavaScript instead of `bigint`. The default value is `false`. |
| `path`            | `String`         | None          | The path to the Thrift file to be parsed.                                                                                     |

## Support Operating Systems

|                  | node14             | node16             | node18             |
| ---------------- | ------------------ | ------------------ | ------------------ |
| Windows x64      | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| Windows x32      | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| Windows arm64    | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| macOS x64        | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| macOS arm64      | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| Linux x64 gnu    | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| Linux x64 musl   | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| Linux arm gnu    | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| Linux arm64 gnu  | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| Linux arm64 musl | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| Android arm64    | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| Android armv7    | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| FreeBSD x64      | :white_check_mark: | :white_check_mark: | :white_check_mark: |

## Develop requirements

- Install the latest `Rust`
- Install `Node.js@16+`
- Enable corepack with `corepack enable`

## Test in local

- yarn
- yarn build
- yarn test

And you will see:

```bash
$ ava --verbose

  ✔ sync function from native code
  ✔ sleep function from native code (201ms)
  ─

  2 tests passed
✨  Done in 1.12s.
```
