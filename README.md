# 概要

Rust を学び始めたのでとりあえずライフゲームを書いてみた

https://github.com/do7be/rust-life-game/assets/9024344/68a8df78-da48-4da5-b247-f2d16d98bb2f

# Wasm 対応

## Build

```
$ wasm-pack build
```

## Setting

```
$ npm init wasm-app www
$ cd www
$ npm i
```

Add to package.json

```
  "dependencies": {
    "hello-wasm-pack": "file:../pkg"
  },
```

```
$ npm i
```

## Start

```
$ cd www
$ npm run start
```
