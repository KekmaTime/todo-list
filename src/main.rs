use ncurses::*;

fn main() {
    initscr();
    let mut quit = false;
    let todos = vec!["Do ProjectEuler", "Do Ferrous", "Shave"];

    for (row, todo) in todos.iter().enumerate() {
        mvprintw(row as i32, 1, todo);
    }
    refresh();

    while !quit {
        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            _ => {}
        }
    }
    endwin();
}
