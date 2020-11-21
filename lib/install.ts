import fs from 'fs';
import os from 'os';
import path from 'path';

import { family } from 'detect-libc';

import packageJson from '../package.json';

export const name = packageJson.name;
export const version = packageJson.version;
export const manifest = packageJson["prebuild-manifest"];

export const osPlatform = os.platform();
export const osArch = os.arch();
export const osFamily = family || 'unknown';

export const nativeInstallPath = path.join(__dirname, "..", manifest.path);
export const localBuildFilename = `${name}-${version}-${osArch}-${osPlatform}-${osFamily}.node`;
export const localBuildInstallPath = path.join(nativeInstallPath, localBuildFilename);

export function getCompatableRemotes() {
    return manifest.files.filter(file => file.os == osPlatform && file.cpu == osArch && (!file.musl || family == 'musl'));
}

export function findInstalledModule(): string {
    const compatableFiles = [
        localBuildFilename,
        ...getCompatableRemotes().map(x => resolve(x.name))
    ].map(file => path.join(nativeInstallPath, file));
    const installedFiles = compatableFiles.filter(file => fs.existsSync(file));
    if(installedFiles.length === 0) {
        throw new Error("[resvg-node] The native module is not installed.");
    }
    else {
        return installedFiles[0];
    }
}

const templateValues: Record<string, string> = { name, version, platform: osPlatform, arch: osArch, family: osFamily }
export function resolve(value: string) {
    return value.replace(/{(.*?)}/g, (x,g)=> templateValues[g]);
}
