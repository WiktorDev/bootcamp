use std::cell::RefCell;

thread_local! {
    static ITEMS: RefCell<Vec<String>> = RefCell::default();
}

#[ic_cdk::query]
fn greet(name: String, last_name: i8) -> String {
    format!("Hello, {} {}!", name, last_name)
}

#[ic_cdk::update]
fn add_item(text: String) {
    ITEMS.with(|items| {
        items.borrow_mut().push(text)
    });
}

#[ic_cdk::query]
fn read_items() -> Vec<String> {
    ITEMS.with(|items| {
        items.borrow().clone()
    })
}
