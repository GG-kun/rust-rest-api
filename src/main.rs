use lazy_static::lazy_static; // 1.4.0
use std::sync::RwLock;
use std::fmt;
use regex::Regex;

#[macro_use] extern crate rocket;

#[derive(Clone, Debug)]
struct Element {
    id: String,
    x: String,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"id\": \"{}\", \"x\": \"{}\"}}", self.id, self.x)
    }
}

lazy_static! {
    static ref ELEMENTS: RwLock<Vec<Element>> = RwLock::new(vec![]);
}

fn insert_one(element: &Element) {
    ELEMENTS.write().unwrap().push(element.clone());
}

#[post("/<id>/<x>")]
fn post_one(id: String, x: String) -> String {
    match get_element(&id) {
        Ok(_element) => return "id already in use".to_string(), 
        _ => (),
    };
    let element = Element{id, x};
    insert_one(&element);
    format!("{}", element)
}

fn get_index(id: &String) -> usize {
    for (i, element) in ELEMENTS.read().unwrap().iter().enumerate() {
        if element.id == *id {
            return i
        }
    }
    1usize
}

fn update_one(update: &Element) {
    let id = &update.id;
    let index = get_index(&id);
    ELEMENTS.write().unwrap().remove(index);
    insert_one(update);
}

#[put("/<id>/<x>")]
fn put_one(id: String, x: String) -> String {
    match get_element(&id) {
        Err(err) => return err.to_string(),
        _ => (),
    };
    let element = Element{id, x};
    update_one(&element);
    format!("{}", element)
}

fn get_element(id: &String) -> Result<Element, &'static str> {
    for element in ELEMENTS.read().unwrap().iter() {
        if element.id == *id {
            return Ok(element.clone())
        }
    }
    Err("could not find element")
}

#[get("/<id>")]
fn find_one(id: String) -> String {
    match get_element(&id) {
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

fn delete(update: &Element) {
    let id = &update.id;
    let index = get_index(&id);
    ELEMENTS.write().unwrap().remove(index);
}

#[delete("/<id>")]
fn delete_one(id: String) -> String {
    let element = match get_element(&id) {
        Err(err) => return err.to_string(),
        Ok(element) => element,
    };
    delete(&element);
    format!("{}", element)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![find, find_one, post_one, put_one, delete_one])
}
