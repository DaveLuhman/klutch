// Require your modules
const HID = require('node-hid');
const mediaController = require('media-controller');

// Create a media player instance
const player = mediaController.createPlayer('FootPedalController');

// Optional: Set initial button status if needed
mediaController.setButtonStatus({
  play: true,
  pause: true,
  next: false,
  prev: false,
  seek: true,
  shuffle: false,
  loop: 'None'
});

// Define custom functions for media actions
function playPressed() {
  // Simulate play key down event
  // Under the hood, mediaController uses WinRT and SendInput to do this.
  console.log('Play button pressed: simulating play key down');
  // (Add your mediaController play key down call here)
}

function playReleased() {
  // Simulate pause key up event
  console.log('Play button released: simulating pause key up');
  // (Add your mediaController pause key up call here)
}

function seekForward() {
  console.log('Seek forward 10 seconds');
  // Implement a custom call, e.g., via mediaController or using a native addon, to simulate a 10-sec forward seek.
}

function seekBackward() {
  console.log('Seek backward 10 seconds');
  // Same as above but for backward seek.
}

// Open the HID device (the foot pedal) based on VID:1523, PID:255
const devices = HID.devices();
const footPedalInfo = devices.find(dev => dev.vendorId === 1523 && dev.productId === 255);
if (!footPedalInfo) {
  console.error('Foot pedal not found.');
  process.exit(1);
}

const footPedal = new HID.HID(footPedalInfo.path);

// Variables to track play button state to handle press vs release
let playState = false;

// Listen for data events from the HID device
footPedal.on('data', (data) => {
  // Assuming data is a Buffer and the report format is exactly 3 bytes like [0x00, code, 0x00]
  const code = data[1];

  // Check for REW (0x04) having the highest priority
  if (code === 0x04) {
    seekBackward();
    return;
  }
  // Next check for PLAY (0x02)
  if (code === 0x02) {
    // Assuming the device generates an event on press and on lift differently
    // Here we simply toggle an internal state for demonstration
    if (!playState) {
      playState = true;
      playPressed();
    }
    // If already pressed, keep ignoring repeated signals
    return;
  } else {
    // If the PLAY button is released (and if we're tracking that accurately by data or time)
    if (playState) {
      playState = false;
      playReleased();
    }
  }

  // Lowest priority: FWD (0x01)
  if (code === 0x01) {
    seekForward();
    return;
  }
});

// Monitor for device disconnect or errors:
footPedal.on('error', (err) => {
  console.error('Foot pedal error:', err);
  // Optionally, show a notification using Electron's Notification API
});
