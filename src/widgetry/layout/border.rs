use chiropterm::Brush;

use crate::widgetry::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, LayoutHacks};

use super::{Column, Container, Row};

pub type Border<'gamestate, Out> = Widget<'gamestate, BorderState<'gamestate, Out>, Out>;

pub struct BorderState<'gamestate, Out: 'gamestate> {
    column: Column<'gamestate, Out>,

    north: Container<'gamestate, Out>,
    west: Container<'gamestate, Out>,
    center: Container<'gamestate, Out>,
    east: Container<'gamestate, Out>,
    south: Container<'gamestate, Out>,

    pub layout_hacks: LayoutHacks,
}

pub enum BorderSlot {
    North, West, East, South, Center,
}

impl <'gamestate, Out: 'gamestate> BorderState<'gamestate, Out> {
    pub fn set<X: Widgetlike<'gamestate, Out=Out>>(&mut self, slot: BorderSlot, w: Widget<'gamestate, X, Out>) {
        match slot {
            BorderSlot::North => self.north.setup(|x| x.set(w)),
            BorderSlot::West => self.west.setup(|x| x.set(w)),
            BorderSlot::East => self.east.setup(|x| x.set(w)),
            BorderSlot::South => self.south.setup(|x| x.set(w)),
            BorderSlot::Center => self.center.setup(|x| x.set(w)),
        };
    }

    pub fn set_north<X: Widgetlike<'gamestate, Out=Out>>(&mut self, w: Widget<'gamestate, X, Out>) {
        self.set(BorderSlot::North, w)
    }

    pub fn set_west<X: Widgetlike<'gamestate, Out=Out>>(&mut self, w: Widget<'gamestate, X, Out>) {
        self.set(BorderSlot::West, w)
    }

    pub fn set_east<X: Widgetlike<'gamestate, Out=Out>>(&mut self, w: Widget<'gamestate, X, Out>) {
        self.set(BorderSlot::East, w)
    }

    pub fn set_south<X: Widgetlike<'gamestate, Out=Out>>(&mut self, w: Widget<'gamestate, X, Out>) {
        self.set(BorderSlot::South, w)
    }

    pub fn set_center<X: Widgetlike<'gamestate, Out=Out>>(&mut self, w: Widget<'gamestate, X, Out>) {
        self.set(BorderSlot::Center, w)
    }
}

impl <'gamestate, Out: 'gamestate> Widgetlike<'gamestate> for BorderState<'gamestate, Out> {
    type Out = Out;

    fn create() -> Self {
        let row = Row::new();
        let column = Column::new();

        let north = Container::new();
        let west = Container::new();
        let center = Container::new().setup(|l| {
            l.layout_hacks.expand_horizontally = true;
            l.layout_hacks.expand_vertically = true;
        });
        let east = Container::new();
        let south = Container::new();

        row.setup(|r| {
            r.add(west.share());
            r.add(center.share());
            r.add(east.share());
        });
        column.setup(|c| {
            c.add(north.share());
            c.add(row.share());
            c.add(south.share());
        });

        Self {
            column, 
            north, west, east, south, center,
            layout_hacks: LayoutHacks::new(),
        }
    }

    fn draw<'frame>(&self, _selected: bool, brush: Brush, menu: WidgetMenu<'gamestate, 'frame, Self, Self::Out>) { 
        self.column.draw(menu.ui, brush, menu.menu)
    }

    fn estimate_dimensions(&self, ui: &UI, width: isize) -> InternalWidgetDimensions {
        self.column.estimate_dimensions(ui, width).to_internal()
    }

    fn clear_layout_cache(&self, ui: &UI) { 
        self.column.clear_layout_cache_if_needed(ui)
    }

    fn layout_hacks(&self) -> LayoutHacks { 
        self.layout_hacks 
    }
}