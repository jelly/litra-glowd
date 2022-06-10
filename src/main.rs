use std::sync::{Arc, Mutex};
use hidapi;
use zbus::blocking::ConnectionBuilder;
use zbus::Result;
use zbus_macros::dbus_interface;
use event_listener::Event;

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

struct LitraGlowd {
    done: Event,
    device: Arc<Mutex<hidapi::HidDevice>>,
}

#[dbus_interface(name = "org.jelle.LitraGlowd1")]
impl LitraGlowd {
    fn on(&self) {
        let device = self.device.lock().unwrap();
        write(&device, vec![0x1c, 0x01]);
    }

    fn off(&self) {
        let device = self.device.lock().unwrap();
        write(&device, vec![0x1c, 0x00]);
    }

    fn brightness(&self, brightness: u8) {
        let device = self.device.lock().unwrap();
        if brightness > 100 && brightness < 1 {
            return;
        }
        let f_brightness: f32 = brightness.into();
        let f_level = MIN_BRIGHTNESS + ((f_brightness/100.0) * (MAX_BRIGHTNESS - MIN_BRIGHTNESS)).floor();
        println!("brightness {}, level {}", brightness, f_level);
        let level = f_level as u8;
        write(&device, vec![0x4c, 0x00, level]);
    }

    fn temperature(&self, temp: u16) {
        let device = self.device.lock().unwrap();
        if temp < 2700 && temp < 6500 {
            return;
        }

        let mut cmd = vec![0x9c];
        cmd.extend(temp.to_be_bytes().iter());
        println!("temp {:?}", cmd);
        write(&device, cmd);
    }
}

fn main() -> Result<()> {
    let api = hidapi::HidApi::new().unwrap();
    let (vid, pid) = (VENDOR_ID, PRODUCT_ID);
    let device = Arc::new(Mutex::new(api.open(vid, pid).unwrap()));
    let litraglowd = LitraGlowd {
        done: event_listener::Event::new(),
        device,
    };
    let done_listener = litraglowd.done.listen();
    let _ = ConnectionBuilder::session()?
        .name("org.jelle.LitraGlowd")?
        .serve_at("/org/jelle/LitraGlowd", litraglowd)?
        .build()?;

    done_listener.wait();

    Ok(())
}
