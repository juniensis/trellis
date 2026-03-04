use trellis_core::terminal::{cell::Cell, style::Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RectangleBorder {
    Char(Cell),
    Ascii(Style),
    UnicodeBox(Style),
    Custom {
        corners: [Cell; 4],
        vert: Cell,
        hori: Cell,
    },
}

impl RectangleBorder {
    pub fn into_components(&self) -> ([Cell; 4], Cell, Cell) {
        match self {
            Self::Char(c) => ([*c, *c, *c, *c], *c, *c),
            Self::Ascii(s) => (
                [Cell::new('+').with_style(*s); 4],
                Cell::new('-').with_style(*s),
                Cell::new('|').with_style(*s),
            ),
            Self::UnicodeBox(s) => (
                [Cell::new('+').with_style(*s); 4],
                Cell::new('-').with_style(*s),
                Cell::new('|').with_style(*s),
            ),
            Self::Custom {
                corners,
                vert,
                hori,
            } => (*corners, *vert, *hori),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Eq, Default)]
pub struct RectangleStyle {
    pub fill: Option<Cell>,
    pub border: Option<RectangleBorder>,
}

impl RectangleStyle {
    pub fn new() -> Self {
        Self {
            fill: None,
            border: None,
        }
    }
    pub fn with_fill(mut self, cell: Cell) -> Self {
        self.fill = Some(cell);
        self
    }
    pub fn with_border(mut self, border: RectangleBorder) -> Self {
        self.border = Some(border);
        self
    }
}

pub enum CircleBorder {
    Char(Cell),
}

pub struct CircleStyle {
    pub fill: Option<Cell>,
    pub border: Option<CircleBorder>,
}
