use orbclient::Renderer;
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::sync::Arc;

use cell::CheckSet;
use event::Event;
use rect::Rect;
use traits::Place;
use widgets::Widget;

pub struct Grid {
    pub rect: Cell<Rect>,
    space_x: Cell<i32>,
    space_y: Cell<i32>,
    entries: RefCell<BTreeMap<(usize, usize), Arc<Widget>>>,
    focused: Cell<Option<(usize, usize)>>,
}

impl Grid {
    pub fn new() -> Arc<Self> {
        Arc::new(Grid {
                     rect: Cell::new(Rect::default()),
                     space_x: Cell::new(0),
                     space_y: Cell::new(0),
                     entries: RefCell::new(BTreeMap::new()),
                     focused: Cell::new(None),
                 })
    }

    pub fn insert<T: Widget>(&self, col: usize, row: usize, entry: &Arc<T>) {
        self.entries
            .borrow_mut()
            .insert((col, row), entry.clone());
        self.arrange(false);
    }

    pub fn spacing(&self, x: i32, y: i32) -> &Self {
        self.space_x.set(x);
        self.space_y.set(y);
        self
    }

    pub fn arrange(&self, resize: bool) {
        let mut cols = Vec::new();
        let mut rows = Vec::new();
        for (&(col, row), entry) in self.entries.borrow().iter() {
            while col >= cols.len() {
                cols.push(Rect::default());
            }
            while row >= rows.len() {
                rows.push(Rect::default());
            }
            let rect = entry.rect().get();
            if rect.width >= cols[col].width {
                cols[col as usize].width = rect.width;
            }
            if rect.width >= rows[row].width {
                rows[row as usize].width = rect.width;
            }
            if rect.height >= cols[col].height {
                cols[col as usize].height = rect.height;
            }
            if rect.height >= rows[row].height {
                rows[row as usize].height = rect.height;
            }
        }

        let rect = self.rect.get();
        let space_x = self.space_x.get();
        let space_y = self.space_y.get();

        let mut x = rect.x;
        for col in cols.iter_mut() {
            col.x = x;
            x += col.width as i32 + space_x;
        }

        let mut y = rect.y;
        for row in rows.iter_mut() {
            row.y = y;
            y += row.height as i32 + space_y;
        }

        for (&(col, row), entry) in self.entries.borrow().iter() {
            let mut rect = entry.rect().get();
            rect.x = cols[col].x;
            rect.y = rows[row].y;
            if resize {
                rect.width = cols[col].width;
                rect.height = rows[row].height;
            }
            entry.rect().set(rect);
        }
    }
}

impl Place for Grid {
    fn position(&self, x: i32, y: i32) -> &Self {
        let mut rect = self.rect().get();
        rect.x = x;
        rect.y = y;
        self.rect().set(rect);

        self.arrange(false);

        self
    }
}

impl Widget for Grid {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        for (&(col, row), entry) in self.entries.borrow().iter() {
            entry.draw(renderer, self.focused.get() == Some((col, row)));
        }
    }

    fn event(&self, event: Event, mut focused: bool, redraw: &mut bool) -> bool {
        for (&(col, row), entry) in self.entries.borrow().iter() {
            let is_focused = self.focused.get() == Some((col, row));
            if entry.event(event, focused && is_focused, redraw) {
                if self.focused.check_set(Some((col, row))) || !focused {
                    focused = true;
                    *redraw = true;
                }
            } else if is_focused {
                self.focused.set(None);
            }
        }

        focused
    }
}
