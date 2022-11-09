pub mod smart_pointers; 

// use smart_pointers::sp_box::*;
// use smart_pointers::sp_box_two::*;
use smart_pointers::sp_rc::*;

// use std::{rc::Rc, thread::JoinHandle};
use std::sync::Arc;

fn main() {
    // Box - usecase 1
    // let t: Box<dyn Vehicle>;
    // t = Box::new(Truck);
    // t.drive();
    
    // Box - usecase 2
    // let t = Truck {
    //     next_truck: None,
    // };
    // println!("{:?}", t);

    // Rc
    // let truck_a = Rc::new(Truck {capacity: 1});
    // let truck_b = Rc::new(Truck {capacity: 2});
    // let truck_c = Rc::new(Truck {capacity: 3});
    // // let facility_one = vec![truck_a, Rc::clone(&truck_b)];
    // // let facility_two = vec![truck_b, truck_c];
    // let facility_one = vec![Rc::clone(&truck_a), Rc::clone(&truck_b)];
    // let facility_two = vec![Rc::clone(&truck_b), Rc::clone(&truck_c)];
    // println!("{:?}", facility_one);
    // println!("{:?}", facility_two);
    // println!("Truck b strong count: {:?}", Rc::strong_count(&truck_b));
    // // std::mem::drop(facility_one);
    // std::mem::drop(facility_two);
    // println!("facility one after drop: {:?}", facility_one);
    // println!("Truck b strong count after drop: {:?}", Rc::strong_count(&truck_b));
    
    // Arc
    let truck_a = Arc::new(Truck {capacity: 1});
    let truck_b = Arc::new(Truck {capacity: 2});
    let truck_c = Arc::new(Truck {capacity: 3});

    let thread = std::thread::spawn(move || -> (Vec<Arc<Truck>>, Vec<Arc<Truck>>) {
        let facility_one = vec![Arc::clone(&truck_a), Arc::clone(&truck_b)];
        let facility_two = vec![Arc::clone(&truck_b), Arc::clone(&truck_c)];
        (facility_one, facility_two)
    });

    let (facility_one , facility_two) = thread.join().unwrap();

    println!("{:?}", facility_one);
    println!("{:?}", facility_two);

    let truck_b = Arc::clone(&facility_one[1]);
    println!("Truck b strong count: {:?}", Arc::strong_count(&truck_b));
    // std::mem::drop(facility_one);
    std::mem::drop(facility_two);
    println!("facility one after drop: {:?}", facility_one);
    println!("Truck b strong count after drop: {:?}", Arc::strong_count(&truck_b));

}
