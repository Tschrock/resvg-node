import packageJson from '../package.json';
import { build } from './build';
import { download } from './download';

const args = (...flags: string[]) => flags.some(f => process.argv.includes(f));

if (args('--help', '-h')) {
    console.log(`

prebuild.js
Manages downloading/building native modules.

--help, -h          Displays help.
--version, -v       Displays the version.
--help, -h          Displays help.
--install           Downloads or builds the native module.
--download          Downloads the native module.
--build             Builds the native module.
--release           Use the release configuration when building.

`.trim());
    process.exit(0);
}

if (args('--version', '-v')) {
    console.log(packageJson.version);
    process.exit(0);
}

if (args('--install')) {
    console.log("Installing native components...")
    download().then(
        () => console.log('success'),
        () => build(args('--release')).then(
            () => console.log('success'),
            error => console.log(error)
        )
    );
}
else if (args('--download')) {
    console.log("Downloading native components...")
    download().then(
        () => console.log('success'),
        error => console.log(error)
    )
}
else if (args('--build')) {
    console.log("Building native components...")
    build(args('--release')).then(
        () => console.log('success'),
        error => console.log(error)
    )
}
