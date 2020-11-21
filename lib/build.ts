import { spawn } from 'child_process';
import fs from 'fs';
import path from 'path';
import { createInterface } from 'readline';

import { localBuildInstallPath } from './install';
import packageJson from '../package.json';

export async function build(release = false): Promise<void> {
    const buildOutput = await cargoBuild(release);
    await fs.promises.mkdir(path.dirname(localBuildInstallPath), { recursive: true });
    await fs.promises.copyFile(buildOutput, localBuildInstallPath);
}

export async function cargoBuild(release = false): Promise<string> {
    return new Promise((resolve, reject) => {
        let outputFile: string;
        const child = spawn(
            'cargo', ['build', '--message-format=json', ...(release ? ['--release'] : [])],
            { cwd: __dirname, stdio: ['inherit', 'pipe', 'inherit'] }
        );
        const lineInterface = createInterface({ input: child.stdout });
        lineInterface.on('line', line => {
            if (line.startsWith('{') && line.endsWith('}')) lineInterface.emit('json', JSON.parse(line));
        });
        lineInterface.on('json', line => {
            if (line.reason === 'compiler-artifact' && line.target && (line.package_id.startsWith(packageJson.name) || line.target.name === packageJson.name)) {
                outputFile = line.filenames[0];
            }
            else if (line.reason === 'build-finished') {
                if (line.success) {
                    resolve(outputFile);
                }
                else {
                    reject(new Error("Build failed."));
                }
            }
        });
        child.on('error', err => {
            reject(err);
        });
    });
}

