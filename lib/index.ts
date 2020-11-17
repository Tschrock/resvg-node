// @ts-ignore
import addon from '../native';

export const render = addon.render as (svgData: string, options?: Options) => Buffer;
export const render_node = addon.render_node;

export interface Options {
    path?: String;
    font?: FontOptions;
    dpi?: number;
    languages?: Array<string>;
    shape_rendering?: ShapeRendering;
    text_rendering?: TextRendering;
    image_rendering?: ImageRendering;
    fit_to?: FitTo;
    background?: string;
}

export interface FontOptions {
    load_system_fonts?: boolean;
    font_files?: Array<string>;
    font_dirs?: Array<string>;
    default_font_family?: string;
    default_font_size?: number;
    serif_family?: string;
    sans_serif_family?: string;
    cursive_family?: string;
    fantasy_family?: string;
    monospace_family?: string;
}

export enum ShapeRendering {
    OptimizeSpeed = 0,
    CrispEdges = 1,
    GeometricPrecision = 2,
}

export enum TextRendering {
    OptimizeSpeed = 0,
    OptimizeLegibility = 1,
    GeometricPrecision = 2,
}

export enum ImageRendering {
    OptimizeQuality = 0,
    OptimizeSpeed = 1,
}

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
