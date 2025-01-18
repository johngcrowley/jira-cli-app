use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    let db = Rc::new(db::JiraDatabase {
        database: Box::new(db::JSONFileDatabase {
            file_path: "../data/db.json".to_string(),
        }),
    });

    let mut nav = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        if let Some(x) = nav.get_current_page() {
            if let Err(e) = x.draw_page() {
                println!("Error rendering page: {}\nPress any key to continue...", e);
                wait_for_key_press();
            } else {
                let input = io_utils::get_user_input();

                println!("{input}");

                match x.handle_input(input.trim()) {
                    Err(e) => {
                        println!("Error rendering page: {}\nPress any key to continue...", e);
                        wait_for_key_press();
                    }
                    Ok(action) => {
                        if action.is_some() {
                            if let Err(error) = nav.handle_action(action.unwrap()) {
                                println!(
                                    "Error rendering page: {}\nPress any key to continue...",
                                    error
                                );
                                wait_for_key_press();
                            }
                        }
                    }
                }
            }
        } else {
            break;
        }

        // 2. render page
        // 3. get user input
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action
    }
}
