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
    "wasm-game-of-life": "file:../pkg"
  },
```

```
$ npm i
```

## www/index.html

```
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>Hello wasm-pack!</title>
    <style>
      body {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
      }
    </style>
  </head>
  <body>
    <noscript>This page contains webassembly and javascript content, please enable javascript in your browser.</noscript>
    <pre id="game-of-life-canvas"></pre>
    <script src="./bootstrap.js"></script>
  </body>
</html>
```

## www/index.js

```
import { WasmLifeGame } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const lifeGame = WasmLifeGame.new(30);

const renderLoop = () => {
  pre.textContent = lifeGame.render();
  lifeGame.tick();

  setTimeout(renderLoop, 500);
};

requestAnimationFrame(renderLoop);
```

## Start

```
$ cd www
$ npm run start
```
