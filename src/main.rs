use hidapi;

const VENDOR_ID: u16 = 0x046d;
const PRODUCT_ID: u16 = 0xc900;

const MIN_BRIGHTNESS: f32 = 20.0; // 0x14
const MAX_BRIGHTNESS: f32 = 250.0; // 0xfa

fn write(device: &hidapi::HidDevice, mut cmd: Vec<u8>) {
    let mut buf = vec![0x11, 0xff, 0x04];
    buf.append(&mut cmd);
    buf.resize(20, 0);
    println!("Read: {:?}", buf);
    let res = device.write(&buf).unwrap();
    println!("Wrote: {:?} byte(s)", res);
}

fn on(device: &hidapi::HidDevice) {
    write(&device, vec![0x1c, 0x01]);
}

fn off(device: &hidapi::HidDevice) {
    write(&device, vec![0x1c, 0x00]);
}

fn brightness(device: &hidapi::HidDevice, brightness: u8) {
    if brightness > 100 && brightness < 1 {
        return;
    }
    let f_brightness: f32 = brightness.into();
    let f_level = MIN_BRIGHTNESS + ((f_brightness/100.0) * (MAX_BRIGHTNESS - MIN_BRIGHTNESS)).floor();
    println!("brightness {}, level {}", brightness, f_level);
    let level = f_level as u8;
    write(&device, vec![0x4c, 0x00, level]);
}

fn temperature(device: &hidapi::HidDevice, temp: u16) {
    if temp < 2700 && temp < 6500 {
        return;
    }

    let mut cmd = vec![0x9c];
    cmd.extend(temp.to_be_bytes().iter());
    println!("temp {:?}", cmd);
    write(&device, cmd);
}

fn main() {

    let api = hidapi::HidApi::new().unwrap();
    let (vid, pid) = (VENDOR_ID, PRODUCT_ID);
    let device = api.open(vid, pid).unwrap();
    // on(&device);
    // brightness(&device, 20);
    // temperature(&device, 4000);
    // off(&device);
}
