#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

const HELP_TEXT = `
Usage: zbr <command>

Commands:
  init    Initialize a new ZBR project
  run     Start the ZBR execution engine
  help    Show this help message
`;

function init() {
  console.log('Initializing new ZBR project...');

  // 1. Create commands directory
  if (!fs.existsSync('commands')) {
    fs.mkdirSync('commands');
    console.log('- Created commands/ directory');
  }

  // 2. Create zbr.json (config)
  const config = {
    prefix: "!",
    status: "online",
    activity: {
      name: "ZBR Scripting",
      type: "playing"
    }
  };
  fs.writeFileSync('zbr.json', JSON.stringify(config, null, 2));
  console.log('- Created zbr.json');

  // 3. Create .env
  if (!fs.existsSync('.env')) {
    const envContent = [
      'DISCORD_TOKEN=YOUR_BOT_TOKEN_HERE',
      'DATABASE_URL=sqlite:./zbr.db',
      ''
    ].join('\n');
    fs.writeFileSync('.env', envContent);
    console.log('- Created .env');
  }

  // 4. Create example .zbr files
  const pingExample = `#trigger !ping
#name Ping Command
#type prefix

Pong! 🏓
Latency: Zping{}ms`;

  const helloExample = `#trigger Hello
#name Welcome Trigger
#type trigger

Hello Zusername{}! 👋
You are currently in the <#ZchannelID{}> channel of Zif{ZguildID{}==;a DM;ZserverName{}}.`;

  fs.writeFileSync(path.join('commands', 'ping.zbr'), pingExample);
  fs.writeFileSync(path.join('commands', 'hello.zbr'), helloExample);
  console.log('- Created example commands in commands/');

  console.log('\nProject initialized successfully!');
  console.log('Next steps:');
  console.log('1. Add your bot token to the .env file');
  console.log('2. Run "zbr run" to start your bot');
}

function run() {
  const binaryName = process.platform === 'win32' ? 'zbr.exe' : 'zbr';
  const binaryPath = path.join(__dirname, binaryName);

  if (!fs.existsSync(binaryPath)) {
    console.error(`Error: Execution engine not found at ${binaryPath}`);
    process.exit(1);
  }

  // Grant execution permissions on Unix-like systems
  if (process.platform !== 'win32') {
    try {
      fs.chmodSync(binaryPath, 0o755);
    } catch (err) {}
  }

  const child = spawn(binaryPath, [], {
    stdio: 'inherit',
    env: process.env
  });

  child.on('error', (err) => {
    console.error('Failed to start the execution engine:', err);
    process.exit(1);
  });

  child.on('exit', (code) => {
    process.exit(code === null ? 1 : code);
  });

  // Handle termination signals
  const signals = ['SIGINT', 'SIGTERM', 'SIGHUP'];
  signals.forEach(signal => {
    process.on(signal, () => {
      if (!child.killed) {
        child.kill(signal);
      }
    });
  });
}

function main() {
  const args = process.argv.slice(2);
  const command = args[0];

  switch (command) {
    case 'init':
      init();
      break;
    case 'run':
      run();
      break;
    case 'help':
    case '--help':
    case '-h':
    default:
      if (command && command !== 'help') {
        console.log(`Unknown command: ${command}`);
      }
      console.log(HELP_TEXT);
      break;
  }
}

main();
