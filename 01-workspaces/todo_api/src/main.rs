use todo_shared::TodoItem;
fn main() {
    let todo_item = TodoItem::new("Going full stack on rust", "Let's go full stack on Rust");
    println!("Created a new todo item : {:?}", todo_item);
}
