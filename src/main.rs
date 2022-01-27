use lazy_static::lazy_static; // 1.4.0
use std::sync::RwLock;
use std::fmt;
use regex::Regex;

#[macro_use] extern crate rocket;

#[derive(Clone, Copy, Debug)]
struct Element {
    id: i32,
    x: i32,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"id\": {}, \"x\": {}}}", self.id, self.x)
    }
}

lazy_static! {
    static ref ELEMENTS: RwLock<Vec<Element>> = RwLock::new(vec![]);
}

fn insert_one(element: Element) {
    ELEMENTS.write().unwrap().push(element);
}

#[post("/<id>/<x>")]
fn post_one(id: i32, x: i32) {
    insert_one(Element{id, x})
}

fn get_index(id: i32) -> usize {
    for (i, element) in ELEMENTS.read().unwrap().iter().enumerate() {
        if element.id == id {
            return i
        }
    }
    1usize
}

fn update_one(update: Element) {
    let index = get_index(update.id);
    ELEMENTS.write().unwrap().remove(index);
    insert_one(update);
}

#[put("/<id>/<x>")]
fn put_one(id: i32, x: i32) -> String {
    match get_element(id) {
        Err(err) => return err.to_string(),
        _ => (),
    };
    let element = Element{id, x};
    update_one(element);
    format!("{}", element)
}

fn get_element(id: i32) -> Result<Element, &'static str> {
    for element in ELEMENTS.read().unwrap().iter() {
        if element.id == id {
            return Ok(*element)
        }
    }
    Err("could not find element")
}

#[get("/<id>")]
fn find_one(id: i32) -> String {
    match get_element(id) {
        Ok(element) => return format!("{}", element),
        Err(err) => return format!("{}", err),
    }
}

#[get("/")]
fn find() -> String {
    let r = Regex::new(r"([\w]*):").unwrap();
    let s = format!("{:?}", ELEMENTS.read().unwrap()).replace("Element", "");
    format!("{}", r.replace_all(&s, "\"$1\":"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![find, find_one, post_one, put_one])
}
