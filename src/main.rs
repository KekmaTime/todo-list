use ncurses::*;
use std::cmp::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;
const KEY_UP: i32 = 259;
const KEY_DOWN: i32 = 258;
const QUIT: i32 = 113;

fn main() {
    initscr();
    noecho();
    keypad(stdscr(), true);

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let todos = vec!["Do ProjectEuler", "Do Ferrous", "Shave"];
    let mut todo_curr: usize = 0;

    refresh_todos(&todos, todo_curr);
    while !quit {

        let key = getch();
        match key {
            KEY_UP => {
                if todo_curr > 0 {
                    todo_curr -= 1;
                    refresh_todos(&todos, todo_curr);
                }
            }
            KEY_DOWN => {
                if todo_curr < todos.len() - 1 {
                    todo_curr = min(todo_curr + 1, todos.len() - 1);
                    refresh_todos(&todos, todo_curr);
                }
            }
            QUIT => quit = true,
            _ => {}
        }
    }
    endwin();
}

fn refresh_todos(todos: &[&str], todo_curr: usize) {
    clear();
    for (index, &todo) in todos.iter().enumerate() {
        let pair = if index == todo_curr {
            HIGHLIGHT_PAIR
        } else {
            REGULAR_PAIR
        };
        attron(COLOR_PAIR(pair));
        mvprintw(index as i32, 1, todo);
        attroff(COLOR_PAIR(pair));
    }
    refresh();  
}
