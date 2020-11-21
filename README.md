resvg for Node.js
========================

[resvg][resvg] is a small, fast, and portable [SVG][svg] rendering library written in [Rust][rust].

## Installation

```sh
$ npm install resvg-node
```

## Basic Usage

The library currently exports a single function `render()` which will render a SVG string and return a Node.js Buffer containing the image data.
```js
const { render } = require('resvg-node');

const pngData = render('<svg height="100" width="100" xmlns="http://www.w3.org/2000/svg"><circle cx="50" cy="50" r="40" fill="red" /></svg>');
// pngData: <Buffer 89 50 4e 47 0d 0a 1a 0a 00 00 00 0d 49 ...>
```

You can also pass a second parameter with extra rendering and font options.
```js
const { render } = require('resvg-node');

const options = {
    background: "#ff00ff",
    fit_to: {
        mode: "width",
        value: 1000
    }
};

const pngData = render('<svg height="100" width="100" xmlns="http://www.w3.org/2000/svg"><circle cx="50" cy="50" r="40" fill="red" /></svg>', options);
// pngData: <Buffer 89 50 4e 47 0d 0a 1a 0a 00 00 00 0d 49 ...>
```

### SvgOptions

| Option          | Type           | Default            | Description                                                                 |
|-----------------|----------------|--------------------|-----------------------------------------------------------------------------|
| path            | string         |                    | The SVG image path. Used to resolve relative image paths.                   |
| font            | FontOptions    |                    | Font related options.                                                       |
| dpi             | number         | 96.0               | The target DPI. Affects unit conversions.                                   |
| languages       | string[]       | ["en"]             | A list of languages, for resolving `systemLanguage` conditional attributes. |
| shape_rendering | ShapeRendering | GeometricPrecision | The default shape rendering method.                                         |
| text_rendering  | TextRendering  | OptimizeLegibility | The default text rendering method.                                          |
| image_rendering | ImageRendering | OptimizeQuality    | The default image rendering method.                                         |
| fit_to          | FitTo          | Original           | The size to render the SVG.                                                 |
| background      | string         |                    | The background color of the SVG.                                            |

### FontOptions

| Option              | Type     | Default         | Description                         |
|---------------------|----------|-----------------|-------------------------------------|
| load_system_fonts   | boolean  | true            | If system fonts should be loaded.   |
| font_files          | string[] | []              | A list of font files to load.       |
| font_dirs           | string[] | []              | A list of font directories to load. |
| default_font_family | string   | Times New Roman | The default font family.            |
| default_font_size   | number   | 12              | The default font size.              |
| serif_family        | string   | Times New Roman | The 'serif' font family.            |
| sans_serif_family   | string   | Arial           | The 'sans-serif' font family.       |
| cursive_family      | string   | Comic Sans MS   | The 'cursive' font family.          |
| fantasy_family      | string   | Impact          | The 'fantasy' font family.          |
| monospace_family    | string   | Courier New     | The 'monospace' font family.        |

### ShapeRendering

| Name               | Value |
|--------------------|-------|
| OptimizeSpeed      | 0     |
| CrispEdges         | 1     |
| GeometricPrecision | 2     |

### TextRendering

| Name               | Value |
|--------------------|-------|
| OptimizeSpeed      | 0     |
| OptimizeLegibility | 1     |
| GeometricPrecision | 2     |

### ImageRendering

| Name            | Value |
|-----------------|-------|
| OptimizeQuality | 0     |
| OptimizeSpeed   | 1     |

### FitTo

`FitTo` is a bit special, it specifies how the svg should be scaled and has 4 different modes:

#### Original
Renders the SVG at it's original size.
```js
{ mode: "original" }
```
#### Width
Renders the SVG using the specified width.
```js
{ mode: "width", value: 1080 }
```
#### Height
Renders the SVG using the specified width.
```js
{ mode: "height", value: 1080 }
```
#### Zoom
Renders the SVG using the specified scale factor.
```js
{ mode: "zoom", value: 1.5 }
```

## Building
Since `resvg` is a Rust library. you will need to install the [Rust toolchain][rust-install] to build the module. Pre-built binaries are also available for Windows and Linux and will be automatically downloaded when the module is installed.

To download a pre-built release:
```sh
$ npm run download
```

To build the module yourself:
```sh
$ npm run build:rs
```

To build an optimized release:
```sh
$ npm run build:release:rs
```

## License
`resvg` and `resvg-node` are licensed under the MPLv2.0.

[resvg]: https://github.com/RazrFalcon/resvg
[svg]: https://en.wikipedia.org/wiki/Scalable_Vector_Graphics
[rust]: https://www.rust-lang.org/
[rust-install]: https://www.rust-lang.org/tools/install
