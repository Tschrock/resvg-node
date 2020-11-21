
export declare const GLIBC: 'glibc';
export declare const MUSL: 'musl';
export declare const family: '' | 'glibc' | 'musl';
export declare const version: string;
export declare const method: 'getconf' | 'ldd' | 'filesystem';
export declare const isNonGlibcLinux: boolean;
