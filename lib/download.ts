import http, { IncomingMessage } from 'http';
import https from 'https';
import fs from 'fs';
import { Transform, TransformCallback } from 'stream';
import { getCompatableRemotes, manifest, nativeInstallPath, resolve } from './install';
import path from 'path';

const NS2S = 10 ** 9; // nanoseconds to seconds

class ProgressTracker extends Transform {
    private totalSize = 0;
    private expectedSize = 0;
    private startTime = process.hrtime.bigint();
    private lastUpdateTime = this.startTime;
    constructor() {
        super();
        this.on('pipe', this._onPipe.bind(this));
    }
    private _updateProgress(chunkSize: number) {
        this.totalSize += chunkSize;
        const totalSize = this.totalSize;
        const expectedSize = this.expectedSize;
        const remainingSize = expectedSize > totalSize ? 0 : expectedSize - totalSize;
        const percentage = this.readableEnded ? 100 : expectedSize > 0 ? totalSize / expectedSize : 0;
        const now = process.hrtime.bigint();
        const chunkTime = now - this.lastUpdateTime;
        const totalTime = now - this.startTime;
        const speed = chunkTime > 0 ? chunkSize / (Number(chunkTime) / NS2S) : chunkSize;
        const remainingTime = BigInt(speed > 0 ? (remainingSize / speed) * NS2S : 0);
        const expectedTime = totalTime + remainingTime;
        this.lastUpdateTime = now;
        this.emit('progress', { chunkSize, chunkTime, totalSize, totalTime, remainingSize, remainingTime, expectedSize, expectedTime, speed, percentage });
    }
    _transform(chunk: any, encoding: BufferEncoding, callback: TransformCallback): void {
        this._updateProgress(chunk.length);
        callback(null, chunk);
    }
    private _onPipe(stream: IncomingMessage) {
        if (stream.readable && stream.headers) {
            this.expectedSize = parseInt(stream.headers['content-length'] || '0');
        }
    }
}

const noop = () => { };

async function downloadFile(url: string, dest: string, { progress = noop, timeout = 10000 }, redirects: string[] = []) {
    return new Promise((resolve, reject) => {
        console.log(`Downloading binary from '${url}' to '${dest}'.`);
        fs.mkdirSync(path.dirname(dest), { recursive: true });
        const progressTracker = new ProgressTracker();
        progressTracker.on('progress', progress);
        const request = (url.startsWith('https') ? https : http).get(url);
        request.on('response', (response) => {
            if (response.statusCode && response.statusCode > 300 && response.statusCode < 400 && response.headers.location) {
                if(redirects.length > 5) throw new Error("Too many redirects.");
                if(redirects.includes(response.headers.location)) throw new Error("Redirect loop detected.");
                redirects.push(response.headers.location);
                const oldUrl = new URL(url);
                const newUrl = new URL(response.headers.location, oldUrl);
                resolve(downloadFile(newUrl.toString(), dest, { progress, timeout }, redirects));
            }
            else if(response.statusCode && response.statusCode >= 200 && response.statusCode < 400 ) {
                const file = fs.createWriteStream(dest);
                file.on('error', err => reject(err));
                file.on('finish', () => resolve(file.path));
                response.pipe(progressTracker).pipe(file);
            }
            else {
                reject(new Error(response.statusMessage || "Error downloading binary."));
            }
        });
        request.on('timeout', () => request.destroy(new Error("Timeout expired.")));
        request.on('error', err => {
            fs.unlink(dest, noop);
            reject(err);
        });
        if (timeout) request.setTimeout(timeout);
    });
}

export async function download(): Promise<void> {
    const remotes = getCompatableRemotes();
    if (remotes.length === 0) {
        throw new Error("No compatable pre-builds were found.");
    }
    const remote = remotes[0];
    const remoteName = resolve(remote.name);
    const installLocation = path.join(nativeInstallPath, remoteName);
    const downloadLocation = resolve(manifest.host) + '/' + remoteName;
    await downloadFile(downloadLocation, installLocation, {});
}
