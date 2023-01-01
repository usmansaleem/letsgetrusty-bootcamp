use std::cell::RefCell;
use std::rc::Rc;

struct Database {
    max_connections: u32,
}

struct AuthService {
    //db: Rc<Database>,
    db: Rc<RefCell<Database>>,
}

struct ContentService {
    //db: Rc<Database>,
    db: Rc<RefCell<Database>>,
}
fn main() {
    println!("Hello, world!");
    //let db = Database {};
    //let auth_service = AuthService { db };
    //let content_service = ContentService { db };

    // Rc example
    //let db = Rc::new(Database {
    //    max_connections: 100,
    //});
    //let auth_service = AuthService { db: Rc::clone(&db) };
    //let content_service = ContentService { db: Rc::clone(&db) };

    // RefCell example
    let db = Rc::new(RefCell::new(Database {
        max_connections: 100,
    }));
    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = ContentService { db: Rc::clone(&db) };

    //db.borrow_mut().max_connections = 200;

    let mut r1 = db.borrow_mut();
    let r2 = db.borrow_mut();
    r1.max_connections = 200;
}
