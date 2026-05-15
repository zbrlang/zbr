const https = require('https');
const fs = require('fs');
const path = require('path');

const VERSION = require('./package.json').version;

const platformMap = {
  'linux-x64':    { name: 'zbr',            url: `https://github.com/zbrlang/zbr/releases/download/v${VERSION}/zbr` },
  'darwin-x64':   { name: 'zbr-darwin-x64', url: `https://github.com/zbrlang/zbr/releases/download/v${VERSION}/zbr-darwin-x64` },
  'darwin-arm64': { name: 'zbr-darwin-arm64', url: `https://github.com/zbrlang/zbr/releases/download/v${VERSION}/zbr-darwin-arm64` },
  'win32-x64':    { name: 'zbr.exe',        url: `https://github.com/zbrlang/zbr/releases/download/v${VERSION}/zbr.exe` },
};

const key = `${process.platform}-${process.arch}`;
const target = platformMap[key];

if (!target) {
  console.error(`Unsupported platform: ${key}`);
  process.exit(1);
}

const binDir = path.join(__dirname, 'bin');
if (!fs.existsSync(binDir)) {
  fs.mkdirSync(binDir);
}

const dest = path.join(binDir, target.name);

console.log(`Downloading ZBR binary for ${key} from ${target.url}...`);

function download(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Handle redirect (common with GitHub Release assets)
        download(response.headers.location, dest).then(resolve).catch(reject);
        return;
      }

      if (response.statusCode !== 200) {
        reject(new Error(`Failed to download binary: ${response.statusCode}`));
        return;
      }

      response.pipe(file);
      file.on('finish', () => {
        file.close();
        // Set execution permissions on Unix
        if (process.platform !== 'win32') {
          fs.chmodSync(dest, 0o755);
        }
        resolve();
      });
    }).on('error', (err) => {
      fs.unlink(dest, () => {}); // Delete partial file
      reject(err);
    });
  });
}

download(target.url, dest)
  .then(() => {
    console.log(`Successfully downloaded ZBR binary to ${dest}`);
  })
  .catch((err) => {
    console.error('Error downloading binary:', err.message);
    process.exit(1);
  });
