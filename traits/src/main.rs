trait Vehicle: Paint {
    fn park(&self);
    fn get_default_color() -> String {
        "black".to_string()
    }
}

trait Paint {
    fn paint(&self, color: String) {
        println!("Painting Object: {color}");
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}

struct Car {
    info: VehicleInfo,
}

impl Vehicle for Car {
    fn park(&self) {
        println!("Parking Car");
    }
}

impl Paint for Car {}

struct Truck {
    info: VehicleInfo,
}

impl Truck {
    fn unload(&self) {
        println!("Unloading Truck");
    }
}

impl Vehicle for Truck {
    fn park(&self) {
        println!("Parking Truck");
    }
}

impl Paint for Truck {}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("Painting house with {color}")
    }
}

fn main() {
    println!("Hello, world!");

    let truck = Truck {
        info: VehicleInfo {
            make: "Toyota".to_string(),
            model: "Hilux".to_string(),
            year: 1997,
        },
    };

    truck.unload();
    truck.park();

    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_string(),
            model: "Civic".to_string(),
            year: 1995,
        },
    };

    let house = House {};
    let obj = create_paintable_object(false);

    paint_red(&car);
    paint_red(&truck);
    paint_red(&house);
    paint_red(obj.as_ref());

    paint_park_red3(&car);
    paint_park_red3(&truck);
    // uncommenting following will cause error as trait is not bound
    // paint_park_red3(&house);
    // paint_park_red3(&obj);

    let paintable_obj: Vec<&dyn Paint> = vec![&car, &house];
}

fn paint_red(object: &dyn Paint) {
    object.paint("red".to_string());
}

fn paint_red2(object: &impl Paint) {
    object.paint("red".to_string());
}

fn paint_park_red3<T>(object: &T)
where
    T: Vehicle,
{
    object.paint("red".to_string());
    object.park();
}

fn create_paintable_object(vehicle: bool) -> Box<dyn Paint> {
    if vehicle {
        Box::new(Car {
            info: VehicleInfo {
                make: "Toyota".to_string(),
                model: "Corolla".to_string(),
                year: 2020,
            },
        })
    } else {
        Box::new(House {})
    }
}
