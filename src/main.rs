// get the tessel and accelerometer crate
extern crate rust_tessel;
extern crate rust_accel_mma84;

fn main() {

  // initialize the accelerometer
  let port = rust_tessel::TesselPort::new("a").unwrap();
  let mut accel = rust_accel_mma84::Accelerometer::new(port);
  accel.mode_active();

  // stream accelerometer data
  let mut vals = [0;3];
  loop {
    accel.get_acceleration(&mut vals);
    println!("x: {0:<4} y: {1:<4} z: {2:<4}", vals[0], vals[1], vals[2]);
  }

}
