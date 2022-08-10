use tod::{add_todo, delete_todo, load_data, mark_complete, show_json, show_todos, write_data};

fn main() {
    let mut list = load_data("todos.json".to_owned());

    let mut args = std::env::args();
    let action = args.nth(1).expect("[ERROR] Atleast 1 argument required.");
    if action == "list".to_owned() {
        show_todos(&list);
    } else if action == "add".to_owned() {
        let task = args.nth(0).expect("[ERROR] Missing required arguments.");
        add_todo(&mut list, task);
    } else if action == "complete".to_owned() {
        let index = args.nth(0).expect("[ERROR] Missing required arguments.");
        mark_complete(
            &mut list,
            index.parse().expect("[ERROR] Expected an integer"),
        );
    } else if action == "delete".to_owned() {
        let index = args.nth(0).expect("[ERROR] Missing required arguments.");
        delete_todo(
            &mut list,
            index.parse().expect("[ERROR] Missing required argument"),
        )
    } else if action == "json".to_owned() {
        show_json(&list);
    } else {
        panic!("[ERROR] Invalid argument");
    }

    write_data("todos.json".to_owned(), &list);
}
