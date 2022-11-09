pub mod smart_pointers; 

// use smart_pointers::sp_box::*;
use smart_pointers::sp_box_two::*;

fn main() {
    // let t: Box<dyn Vehicle>;
    // t = Box::new(Truck);
    // t.drive();

    let t = Truck {
        next_truck: None,
    };

    println!("{:?}", t);

}
