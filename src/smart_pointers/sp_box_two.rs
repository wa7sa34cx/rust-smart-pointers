#[derive(Debug)]
pub struct Truck {
    pub next_truck: Option<Box<Truck>>,
}

