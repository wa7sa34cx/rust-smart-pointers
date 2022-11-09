pub trait Vehicle {
  fn drive(&self);
}

pub struct Truck;

impl Vehicle for Truck {
  fn drive(&self) {
      println!("Truck is driving");
  }
}

