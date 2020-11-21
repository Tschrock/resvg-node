import { findInstalledModule } from './install';

const addon = require(findInstalledModule());

/**
 * Renders an SVG.
 * @param svgData A string containing the SVG xml.
 * @param options (Optional) The SVG rendering options.
 * @returns A node.js `Buffer` containing the rendered PNG.
 */
export const render = addon.render as (svgData: string, options?: Options) => Buffer;

/**
 * The SVG rendering options.
 */
export interface Options {
    /**
     * The SVG image path.
     *
     * Used to resolve relative image paths.
     *
     * Default: `undefined`
     */
    path?: String;

    /**
     * Font related options.
     */
    font?: FontOptions;

    /**
     * The target DPI.
     *
     * Affects unit conversions.
     *
     * Default: `96.0`
     */
    dpi?: number;

    /**
     * A list of languages.
     *
     * Will be used to resolve a `systemLanguage` conditional attribute.
     *
     * Format: en, en-US.
     *
     * Default: `["en"]`
     */
    languages?: Array<string>;

    /**
     * The default shape rendering method.
     *
     * Will be used when an SVG element's `shape-rendering` property is set to `auto`.
     *
     * Default: GeometricPrecision
     */
    shape_rendering?: ShapeRendering;

    /**
     * The default text rendering method.
     *
     * Will be used when an SVG element's `text-rendering` property is set to `auto`.
     *
     * Default: OptimizeLegibility
     */
    text_rendering?: TextRendering;

    /**
     * The default image rendering method.
     *
     * Will be used when an SVG element's `image-rendering` property is set to `auto`.
     *
     * Default: OptimizeQuality
     */
    image_rendering?: ImageRendering;

    /**
     * The size to render the SVG.
     *
     * Default: Original
     */
    fit_to?: FitTo;

    /**
     * The background color of the SVG.
     *
     * Default: `undefined`
     */
    background?: string;
}

/**
 * The font options.
 */
export interface FontOptions {
    /**
     * If system fonts should be loaded.
     *
     * Default: true
     */
    load_system_fonts?: boolean;

    /**
     * A list of font files to load.
     *
     * Default: `[]`
     */
    font_files?: Array<string>;

    /**
     * A list of font directories to load.
     *
     * Default: `[]`
     */
    font_dirs?: Array<string>;

    /**
     * The default font family.
     *
     * Will be used when no `font-family` attribute is set in the SVG.
     *
     * Default: Times New Roman
     */
    default_font_family?: string;

    /**
     * The default font size.
     *
     * Will be used when no `font-size` attribute is set in the SVG.
     *
     * Default: 12
     */
    default_font_size?: number;

    /**
     * The 'serif' font family.
     *
     * Default: Times New Roman
     */
    serif_family?: string;

    /**
     * The 'sans-serif' font family.
     *
     * Default: Arial
     */
    sans_serif_family?: string;

    /**
     * The 'cursive' font family.
     *
     * Default: Comic Sans MS
     */
    cursive_family?: string;

    /**
     * The 'fantasy' font family.
     *
     * Default: Impact
     */
    fantasy_family?: string;

    /**
     * The 'monospace' font family.
     *
     * Default: Courier New
     */
    monospace_family?: string;
}

/**
 * The shape rendering method.
 */
export enum ShapeRendering {
    OptimizeSpeed = 0,
    CrispEdges = 1,
    GeometricPrecision = 2,
}

/**
 * The text rendering method.
 */
export enum TextRendering {
    OptimizeSpeed = 0,
    OptimizeLegibility = 1,
    GeometricPrecision = 2,
}

/**
 * The image rendering method.
 */
export enum ImageRendering {
    OptimizeQuality = 0,
    OptimizeSpeed = 1,
}

/**
 * The size to render the SVG.
 */
export type FitTo = {
    mode: "original",
} | {
    mode: "width",
    value: number,
} | {
    mode: "height",
    value: number,
} | {
    mode: "zoom",
    value: number,
}
