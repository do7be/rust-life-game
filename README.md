# Rust Life Game

Rust を学び始めたのでとりあえずライフゲームを書いてみた。Wasm 対応もしている。

## Demo

terminal

https://github.com/do7be/rust-life-game/assets/9024344/68a8df78-da48-4da5-b247-f2d16d98bb2f

wasm

https://github.com/do7be/rust-life-game/assets/9024344/c36a848e-3c38-4821-bce5-306494871ffc

## Wasm 対応

### Build

```
$ wasm-pack build
```

### Setting

```
$ npm init wasm-app www
$ cd www
$ npm i
```

**package.json**

```
  "dependencies": {
    "wasm-game-of-life": "file:../pkg"
  },
```

```
$ npm i
```

**www/index.html**

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
    <pre id="game-of-life-pre"></pre>
    <canvas id="game-of-life-canvas"></canvas>
    <script src="./bootstrap.js"></script>
  </body>
</html>
```

**www/index.js**

```
import { WasmLifeGame } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-pre");
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

## Canvas 対応

**index.js**

```
import { WasmLifeGame } from "wasm-game-of-life";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const lifeGame = WasmLifeGame.new(75);

const size = lifeGame.size();

const canvas = document.getElementById("game-of-life-canvas");
canvas.width = (CELL_SIZE + 1) * size + 1;
canvas.height = (CELL_SIZE + 1) * size + 1;

const ctx = canvas.getContext("2d");

const renderLoop = () => {
  lifeGame.tick();

  drawGrid();
  drawCells();

  setTimeout(renderLoop, 500);
};

requestAnimationFrame(renderLoop);

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= size; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * size + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= size; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * size + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const getIndex = (row, column) => {
  return row * size + column;
};

const drawCells = () => {
  const cells = lifeGame.table();

  ctx.beginPath();

  for (let row = 0; row < size; row++) {
    for (let col = 0; col < size; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] ? ALIVE_COLOR : DEAD_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};
```
