pub mod smart_pointers; 

use smart_pointers::sp_box::*;

fn main() {
    let t: Box<dyn Vehicle>;
    t = Box::new(Truck);
    t.drive();
}
