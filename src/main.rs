use ncurses::*;
use std::cmp::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;
const KEY_UP: i32 = 259;
const KEY_DOWN: i32 = 258;
const QUIT: i32 = 113;

type Id = usize;

#[derive(Default)]
struct Ui {
    list_curr: Option<Id>,
    row: usize,
    col: usize,
}

impl Ui {
    fn begin(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    fn begin_list(&mut self, id: Id) {
        assert!(self.list_curr.is_none(), "Nested Lists are Not Allowed");
        self.list_curr = Some(id);
    }

    fn list_element(&mut self, label: &str, id: Id) {
        let id_curr = self
            .list_curr
            .expect("Not allowed to create list elements outside of lists");

        self.label(
            label,
            if id_curr == id {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            },
        );
        // Implement further logic for list elements
    }

    fn label(&mut self, text: &str, pair: i16) {
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair));
        self.row += 1;
        // Implement further logic for labels
    }

    fn end_list(&mut self) {
        self.list_curr = None;
        // Implement further logic for ending lists
    }

    fn end(&mut self) {
        // Implement logic for ending UI elements
    }
}

fn main() {
    initscr();
    noecho();
    keypad(stdscr(), true);

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let todos: Vec<String> = vec![
        "Do ProjectEuler".to_string(),
        "Do Ferrous".to_string(),
        "Shave".to_string(),
    ];
    let mut todo_curr: usize = 0;
    let done = Vec::<String>::new();
    let done_curr: usize = 0;

    let mut ui = Ui::default();

    while !quit {
        ui.begin(0, 0); // Set appropriate starting row and col
        {
            ui.begin_list(todo_curr);
            for (index, &ref todo) in todos.iter().enumerate() {
                ui.list_element(&todo, index);
            }
            ui.end_list();

            ui.label("-------------------------------------", REGULAR_PAIR);

            ui.begin_list(done_curr);
            for (index, _done) in done.iter().enumerate() {
                ui.list_element(&_done, index);
            }
            ui.end_list();
        }
        ui.end();

        refresh();
        let key = getch();
        match key {
            KEY_UP => {
                if todo_curr > 0 {
                    todo_curr -= 1;
                }
            }
            KEY_DOWN => {
                if todo_curr < todos.len() - 1 {
                    todo_curr = min(todo_curr + 1, todos.len() - 1);
                }
            }
            QUIT => quit = true,
            _ => {}
        }
    }
    endwin();
}
