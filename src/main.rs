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
    list_curr : Option<Id,
    row : usize,
    col : usize,
}

impl Ui {
    fn begin(&mut self , row : usize , col : usize){
        self.row  = row;
        self.col = col;
    }

    fn begin_list(&mut self, id: Id){
        assert!(self.list_curr.is_none(), "Nested Lists are Not Allowed");
        self.list_curr = Some(id);
    }
    fn list_element(label : &str , id: Id){
        let pair = if index == todo_curr {
            HIGHLIGHT_PAIR
        } else {
            REGULAR_PAIR
        };
        attron(COLOR_PAIR(pair));
        mvprintw(index as i32, 1, todo);
        attroff(COLOR_PAIR(pair)); 
        todo!()
    }
    fn label(&mut self ,text : &str){
        todo!()
    }

    fn end_list(&mut self){
        todo!()
    }
    fn end(&mut self){
        todo!()
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
    let todos : Vec<String> = vec![
        "Do ProjectEuler".to_string(),
        "Do Ferrous".to_string(),
        "Shave".to_string()
    ];
    let mut todo_curr: usize = 0;
    let done = Vec::<String>::new();
    let mut todo_curr : usize = 1;
    let mut done_curr : usize = 0;

    let mut ui = Ui::default();

    while !quit {
        ui.begin();
        {
	        uiui.begin_list(todo_curr);
	        fofor (index, &todo) in todos.iter().enumerate() {
	        	ui.lui.list_element(todo ,index);
	        }
	        uiui.end_list();

	        uiui.label("-------------------------------------");

	        uiui.begin_list(done_curr);
	        fofor (index, done) in dones.iter().enumerate() {
	            uiui.list_element(index);
	        }
	        uiui.end_list();
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
