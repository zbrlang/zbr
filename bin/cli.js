#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');
const https = require('https');

const SUPPORTED_TYPES = ['prefix', 'slash', 'sub-slash', 'interaction', 'event'];

const HELP_TEXT = `
Usage: zbr <command>

Commands:
  init <folder>  Initialize a new ZBR project in the specified folder (defaults to current directory)
  run            Start the ZBR runtime engine and bring your bot online
  update         Download and install the latest ZBR runtime engine
  version        Show the current ZBR version
  list           List all commands in your commands/ folder
  new <type>     Scaffold a new command file (prefix, slash, sub-slash, interaction, event)
  help           Show this help message
`;

function readPackageVersion() {
  try {
    const pkgPath = path.join(__dirname, '..', 'package.json');
    const pkgContent = fs.readFileSync(pkgPath, 'utf8');
    const pkg = JSON.parse(pkgContent);
    return pkg.version;
  } catch (err) {
    return null;
  }
}

function version() {
  const binaryPath = getBinaryPath();
  if (!fs.existsSync(binaryPath)) {
    console.error('Runtime engine not found. Run "zbr update" to download it.');
    process.exit(1);
  }
  const { execFileSync } = require('child_process');
  try {
    const output = execFileSync(binaryPath, ['--version'], { encoding: 'utf8' }).trim();
    console.log(`zbr ${output}`);
  } catch (err) {
    console.error('Failed to get version from runtime engine.');
    process.exit(1);
  }
}

function commandFileName(trigger) {
  return `${trigger.replace(/^[/!]/, '')}.zbr`;
}

function headerValue(lines, prefix) {
  for (const line of lines) {
    if (line.startsWith(prefix)) {
      return line.slice(prefix.length).trim();
    }
  }
  return '';
}

function formatTable(rows) {
  const widths = rows[0].map((_, index) =>
    Math.max(...rows.map((row) => row[index].length))
  );

  return rows
    .map((row) =>
      row
        .map((cell, index) => cell.padEnd(widths[index]))
        .join('  ')
    )
    .join('\n');
}

function ensureCommandsDir() {
  if (!fs.existsSync('commands')) {
    fs.mkdirSync('commands');
  }
}

function newCommand(type) {
  const commandType = (type || '').trim().toLowerCase();
  if (!commandType) {
    console.error('Usage: zbr new <type>');
    console.error(`Valid types: ${SUPPORTED_TYPES.join(', ')}`);
    process.exit(1);
  }

  if (!SUPPORTED_TYPES.includes(commandType)) {
    console.error(`Unknown type: ${type}`);
    console.error(`Valid types: ${SUPPORTED_TYPES.join(', ')}`);
    process.exit(1);
  }

  let trigger;
  let name;
  let lines = [];

  switch (commandType) {
    case 'prefix':
      trigger = '!hello';
      name = 'Hello Command';
      lines = [
        `#trigger ${trigger}`,
        `#name ${name}`,
        '#type prefix',
        '',
        'ZsendMessage{Hello! This is an example of a prefix based command in ZBR.}'
      ];
      break;
    case 'slash':
      trigger = '/ping';
      name = 'Ping Command';
      lines = [
        `#trigger ${trigger}`,
        `#name ${name}`,
        '#type slash',
        '#scope guild',
        '#description An example slash based command in ZBR.',
        '#option input|Something to say|string|required',
        '',
        'ZsendMessage{Hello! This is an example of a slash based command in ZBR.}'
      ];
      break;
    case 'sub-slash':
      trigger = '/admin ban';
      name = 'Admin Ban Subcommand';
      lines = [
        `#trigger ${trigger}`,
        `#name ${name}`,
        '#type sub-slash',
        '#scope guild',
        '#description An example sub-slash based command in ZBR.',
        '#option user|User to ban|user|required',
        '',
        'ZsendMessage{Hello! This is an example of a sub-slash based command in ZBR.}'
      ];
      break;
    case 'interaction':
      trigger = 'onInteraction{confirm_action}';
      name = 'Interaction Handler';
      lines = [
        `#trigger ${trigger}`,
        `#name ${name}`,
        '#type interaction',
        '#description An example interaction based command in ZBR.',
        '',
        'ZsendMessage{Hello! This is an example of an interaction based command in ZBR.}'
      ];
      break;
    case 'event':
      trigger = 'onMessage';
      name = 'Message Event Handler';
      lines = [
        `#trigger ${trigger}`,
        `#name ${name}`,
        '#type event',
        '#description An example event based command in ZBR.',
        '',
        'ZsendMessage{Hello! This is an example of an event based command in ZBR.}'
      ];
      break;
  }

  ensureCommandsDir();

  const fileName = `${commandType}.zbr`;
  const filePath = path.join('commands', fileName);

  if (fs.existsSync(filePath)) {
    console.error(`Error: ${filePath} already exists. Aborting.`);
    process.exit(1);
  }

  fs.writeFileSync(filePath, lines.join('\n'));
  console.log(`Created ${filePath}`);
}

function listCommands() {
  if (!fs.existsSync('commands')) {
    console.log('No commands/ directory found. Create one with `zbr init` or add .zbr files to commands/.');
    return;
  }

  const files = fs.readdirSync('commands').filter((file) => file.endsWith('.zbr'));
  if (files.length === 0) {
    console.log('No .zbr command files found in commands/. Create one with `zbr new <type>`.');
    return;
  }

  const rows = [['NAME', 'TRIGGER', 'TYPE']];
  files.sort();

  for (const file of files) {
    const filePath = path.join('commands', file);
    let content;

    try {
      content = fs.readFileSync(filePath, 'utf8');
    } catch (err) {
      continue;
    }

    const lines = content.split(/\r?\n/);
    const trigger = headerValue(lines, '#trigger ');
    const name = headerValue(lines, '#name ');
    const type = headerValue(lines, '#type ');
    rows.push([name || file, trigger || '-', type || 'prefix']);
  }

  console.log(formatTable(rows));
}

function init(targetDir) {
  const root = targetDir && targetDir.length ? path.join('.', targetDir) : '.';
  console.log('Initializing new ZBR project in', root);

  const commandsDir = path.join(root, 'commands');
  if (!fs.existsSync(commandsDir)) {
    fs.mkdirSync(commandsDir, { recursive: true });
    console.log('- Created commands/ directory');
  }

  const config = {
    status: 'online',
    activity: {
      name: 'ZBR Scripting',
      type: 'playing'
    },
    logging: true
  };
  fs.writeFileSync(path.join(root, 'zbr.json'), JSON.stringify(config, null, 2));
  console.log('- Created zbr.json');

  if (!fs.existsSync(path.join(root, '.env'))) {
    const envContent = [
      'DISCORD_TOKEN=YOUR_BOT_TOKEN_HERE',
      'DATABASE_URL=sqlite:./zbr.db',
      ''
    ].join('\n');
    fs.writeFileSync(path.join(root, '.env'), envContent);
    console.log('- Created .env');
  }

  const pingExample = `#trigger !ping\n#name Ping Command\n#type prefix\n\nPong! 🏓\nLatency: Zping{}ms`;
  const helloExample = `#trigger Hello\n#name Welcome Trigger\n#type trigger\n\nHello Zusername{}! 👋\nYou are currently in the <#ZchannelID{}> channel of Zif{ZguildID{}==;a DM;ZserverName{}}.`;

  fs.writeFileSync(path.join(commandsDir, 'ping.zbr'), pingExample);
  fs.writeFileSync(path.join(commandsDir, 'hello.zbr'), helloExample);
  console.log('- Created example commands in commands/');

  console.log('\nProject initialized successfully!');
  console.log('Next steps:');
  console.log('1. Add your bot token to the .env file');
  console.log('2. Run "zbr run" to start your bot');
}

function getBinaryPath() {
  const platform = process.platform;
  const arch = process.arch;

  const binaryMap = {
  'linux-x64':   'zbr-linux-x64',
  'linux-arm64': 'zbr-linux-arm64',
  'darwin-x64':  'zbr-darwin-x64',
  'darwin-arm64':'zbr-darwin-arm64',
  'win32-x64':   'zbr-windows-x64.exe',
  'win32-arm64': 'zbr-windows-arm64.exe',
};

  const key = `${platform}-${arch}`;
  const name = binaryMap[key];

  if (!name) {
    console.error(`Unsupported platform: ${platform}-${arch}`);
    console.error('Please open an issue at https://github.com/zbrlang/zbr');
    process.exit(1);
  }

  return path.join(__dirname, '..', 'bin', name);
}

function run() {
  const binaryPath = getBinaryPath();

  if (!fs.existsSync(binaryPath)) {
    console.error(`Error: Runtime engine not found at ${binaryPath}`);
    console.log('Try running "zbr update" to download the latest version.');
    process.exit(1);
  }

  if (process.platform !== 'win32') {
    try {
      fs.chmodSync(binaryPath, 0o755);
    } catch (err) { }
  }

  const child = spawn(binaryPath, [], {
    stdio: 'inherit',
    env: process.env
  });

  child.on('error', (err) => {
    console.error('Failed to start the runtime engine:', err);
    process.exit(1);
  });

  child.on('exit', (code) => {
    process.exit(code === null ? 1 : code);
  });

  const signals = ['SIGINT', 'SIGTERM', 'SIGHUP'];
  signals.forEach(signal => {
    process.on(signal, () => {
      if (!child.killed) {
        child.kill(signal);
      }
    });
  });
}

function update() {
  const platform = process.platform;
  const arch = process.arch;

  const binaryMap = {
  'linux-x64':   'zbr-linux-x64',
  'linux-arm64': 'zbr-linux-arm64',
  'darwin-x64':  'zbr-darwin-x64',
  'darwin-arm64':'zbr-darwin-arm64',
  'win32-x64':   'zbr-windows-x64.exe',
  'win32-arm64': 'zbr-windows-arm64.exe',
};

  const key = `${platform}-${arch}`;
  const binaryName = binaryMap[key];

  if (!binaryName) {
    console.error(`Unsupported platform: ${platform}-${arch}`);
    process.exit(1);
  }

  const binaryPath = path.join(__dirname, binaryName);
  const url = `https://github.com/zbrlang/zbr/releases/latest/download/${binaryName}`;

  console.log(`Updating ZBR runtime engine for ${key}...`);
  console.log(`Downloading from ${url}`);

  function downloadFile(downloadUrl) {
    https.get(downloadUrl, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        downloadFile(response.headers.location);
        return;
      }

      if (response.statusCode !== 200) {
        console.error(`Failed to download binary: HTTP ${response.statusCode}`);
        process.exit(1);
      }

      const file = fs.createWriteStream(binaryPath);
      response.pipe(file);

      file.on('finish', () => {
        file.close();
        if (process.platform !== 'win32') {
          fs.chmodSync(binaryPath, 0o755);
        }

        // Update package.json version to match the downloaded binary
        try {
          const pkgPath = path.join(__dirname, '..', 'package.json');
          const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'));
          const { execFileSync } = require('child_process');
          const newVersion = execFileSync(binaryPath, ['--version'], { encoding: 'utf8' }).trim().replace(/^v/, '');
          pkg.version = newVersion;
          fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2));
        } catch (err) { }

        console.log('Update successful! ZBR runtime engine is now up to date.');
      });

      file.on('error', (err) => {
        fs.unlink(binaryPath, () => { });
        console.error(`File error: ${err.message}`);
        process.exit(1);
      });
    }).on('error', (err) => {
      console.error(`Update failed: ${err.message}`);
      process.exit(1);
    });
  }

  downloadFile(url);
}

function main() {
  const args = process.argv.slice(2);
  const command = args[0];

  switch (command) {
    case 'init':
      init(args[1]);
      break;
    case 'run':
      run();
      break;
    case 'update':
      update();
      break;
    case 'version':
    case '--version':
    case '-v':
      version();
      break;
    case 'new':
      newCommand(args[1]);
      break;
    case 'list':
      listCommands();
      break;
    case 'help':
    case '--help':
    case '-h':
      console.log(HELP_TEXT);
      break;
    default:
      if (command) {
        console.log(`Unknown command: ${command}`);
      }
      console.log(HELP_TEXT);
      break;
  }
}

main();
