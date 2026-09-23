use std::collections::BTreeMap;
use zellij_tile::prelude::*;

// ============================================================
// POST-APOLLO // CRT VIDEO TERMINAL
// ============================================================

#[derive(Clone, Copy, PartialEq, Eq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Style {
    fg: Color,
    bg: Color,
    bold: bool,
}

#[derive(Clone, Copy)]
struct Cell {
    ch: char,
    style: Style,
}

// ============================================================
// PALETTE
// ============================================================

const CHASSIS_BG: Color = Color {
    r: 0x1B,
    g: 0x06,
    b: 0x23,
};

const BLACK: Color = Color {
    r: 0x08,
    g: 0x01,
    b: 0x0B,
};

const BLACK_INSET: Color = Color {
    r: 0x0D,
    g: 0x02,
    b: 0x12,
};

const RAISED_PURPLE: Color = Color {
    r: 0x26,
    g: 0x0A,
    b: 0x30,
};

const KNOB_BODY: Color = Color {
    r: 0x20,
    g: 0x18,
    b: 0x23,
};

const KNOB_EDGE: Color = Color {
    r: 0x52,
    g: 0x43,
    b: 0x55,
};

const CYAN: Color = Color {
    r: 0x55,
    g: 0xCF,
    b: 0xCA,
};

const CYAN_DIM: Color = Color {
    r: 0x2B,
    g: 0x79,
    b: 0x77,
};

const CYAN_DARK: Color = Color {
    r: 0x16,
    g: 0x43,
    b: 0x43,
};

const ORANGE: Color = Color {
    r: 0xED,
    g: 0x98,
    b: 0x1A,
};

const WHITE: Color = Color {
    r: 0xDC,
    g: 0xF3,
    b: 0xFA,
};

const WHITE_DIM: Color = Color {
    r: 0x99,
    g: 0xAC,
    b: 0xB0,
};

const METAL_SHADOW: Color = Color {
    r: 0x50,
    g: 0x5C,
    b: 0x60,
};

const GREEN: Color = Color {
    r: 0x00,
    g: 0xF7,
    b: 0x82,
};

const RED: Color = Color {
    r: 0xD1,
    g: 0x60,
    b: 0x41,
};

// ============================================================
// STYLES
// ============================================================

const CHASSIS: Style = Style {
    fg: CYAN_DIM,
    bg: CHASSIS_BG,
    bold: false,
};

const EDGE: Style = Style {
    fg: CYAN_DIM,
    bg: CHASSIS_BG,
    bold: false,
};

const EDGE_BRIGHT: Style = Style {
    fg: CYAN,
    bg: CHASSIS_BG,
    bold: false,
};

const LABEL: Style = Style {
    fg: WHITE,
    bg: CHASSIS_BG,
    bold: false,
};

const LABEL_DIM: Style = Style {
    fg: WHITE_DIM,
    bg: CHASSIS_BG,
    bold: false,
};

const ACTIVE: Style = Style {
    fg: ORANGE,
    bg: CHASSIS_BG,
    bold: true,
};

const INSET_CYAN: Style = Style {
    fg: CYAN_DIM,
    bg: BLACK_INSET,
    bold: false,
};

const INSET_ACTIVE: Style = Style {
    fg: ORANGE,
    bg: BLACK_INSET,
    bold: true,
};

const BLACK_CYAN: Style = Style {
    fg: CYAN_DIM,
    bg: BLACK,
    bold: false,
};

const BLACK_CYAN_DARK: Style = Style {
    fg: CYAN_DARK,
    bg: BLACK,
    bold: false,
};

const BLACK_WHITE: Style = Style {
    fg: WHITE,
    bg: BLACK,
    bold: true,
};

const RAISED: Style = Style {
    fg: CYAN_DIM,
    bg: RAISED_PURPLE,
    bold: false,
};

const RAISED_TEXT: Style = Style {
    fg: WHITE,
    bg: RAISED_PURPLE,
    bold: true,
};

const KNOB_FILL: Style = Style {
    fg: KNOB_BODY,
    bg: BLACK_INSET,
    bold: true,
};

const KNOB_BORDER: Style = Style {
    fg: KNOB_EDGE,
    bg: BLACK_INSET,
    bold: true,
};

const KNOB_POINTER: Style = Style {
    fg: WHITE,
    bg: KNOB_BODY,
    bold: true,
};

const VFD: Style = Style {
    fg: CYAN,
    bg: BLACK,
    bold: false,
};

const VFD_ACTIVE: Style = Style {
    fg: ORANGE,
    bg: BLACK,
    bold: true,
};

const VFD_READY: Style = Style {
    fg: GREEN,
    bg: BLACK,
    bold: true,
};

const POWER: Style = Style {
    fg: RED,
    bg: RAISED_PURPLE,
    bold: true,
};

// ============================================================
// PLUGIN
// ============================================================

#[derive(Default)]
struct State {
    region: String,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        self.region = configuration
            .get("region")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());
    }

    fn render(&mut self, rows: usize, cols: usize) {
        match self.region.as_str() {
            "side" => render_side(rows, cols),
            "deck" => render_deck(rows, cols),
            _ => render_unknown(rows, cols),
        }
    }
}

// ============================================================
// CANVAS
// ============================================================

struct Canvas {
    rows: usize,
    cols: usize,
    cells: Vec<Vec<Cell>>,
}

impl Canvas {
    fn new(rows: usize, cols: usize, style: Style) -> Self {
        Self {
            rows,
            cols,
            cells: vec![vec![Cell { ch: ' ', style }; cols]; rows],
        }
    }

    fn set(&mut self, row: usize, col: usize, ch: char, style: Style) {
        if row < self.rows && col < self.cols {
            self.cells[row][col] = Cell { ch, style };
        }
    }

    fn write(&mut self, row: usize, col: usize, text: &str, style: Style) {
        if row >= self.rows {
            return;
        }

        for (offset, ch) in text.chars().enumerate() {
            let x = col + offset;

            if x >= self.cols {
                break;
            }

            self.set(row, x, ch, style);
        }
    }

    fn centered(&mut self, row: usize, text: &str, style: Style) {
        if row >= self.rows {
            return;
        }

        let width = text.chars().count();

        if width >= self.cols {
            self.write(row, 0, text, style);
            return;
        }

        self.write(row, (self.cols - width) / 2, text, style);
    }

    fn fill_rect(
        &mut self,
        row: usize,
        col: usize,
        height: usize,
        width: usize,
        style: Style,
    ) {
        for y in row..row.saturating_add(height) {
            for x in col..col.saturating_add(width) {
                if y < self.rows && x < self.cols {
                    self.set(y, x, ' ', style);
                }
            }
        }
    }

    // Full-block fill.
    //
    // This is intentional: unlike a plain terminal background
    // color, the foreground glyph itself occupies the cell.
    // It gives transparent Kitty a much more substantial
    // "hardware plate" to composite.
    fn solid_rect(
        &mut self,
        row: usize,
        col: usize,
        height: usize,
        width: usize,
        color: Color,
    ) {
        let style = Style {
            fg: color,
            bg: color,
            bold: false,
        };

        for y in row..row.saturating_add(height) {
            for x in col..col.saturating_add(width) {
                if y < self.rows && x < self.cols {
                    self.set(y, x, '█', style);
                }
            }
        }
    }

    fn rect(
        &mut self,
        row: usize,
        col: usize,
        height: usize,
        width: usize,
        style: Style,
    ) {
        if height < 2 || width < 2 {
            return;
        }

        let bottom = row + height - 1;
        let right = col + width - 1;

        if bottom >= self.rows || right >= self.cols {
            return;
        }

        self.set(row, col, '┌', style);
        self.set(row, right, '┐', style);
        self.set(bottom, col, '└', style);
        self.set(bottom, right, '┘', style);

        for x in col + 1..right {
            self.set(row, x, '─', style);
            self.set(bottom, x, '─', style);
        }

        for y in row + 1..bottom {
            self.set(y, col, '│', style);
            self.set(y, right, '│', style);
        }
    }

    fn recessed_rect(
        &mut self,
        row: usize,
        col: usize,
        height: usize,
        width: usize,
    ) {
        self.solid_rect(
            row,
            col,
            height,
            width,
            BLACK_INSET,
        );

        self.rect(
            row,
            col,
            height,
            width,
            Style {
                fg: CYAN_DARK,
                bg: BLACK_INSET,
                bold: false,
            },
        );
    }

    fn render(&self) {
        for row in &self.cells {
            let mut output = String::new();
            let mut previous: Option<Style> = None;

            for cell in row {
                if previous != Some(cell.style) {
                    output.push_str(&ansi(cell.style));
                    previous = Some(cell.style);
                }

                output.push(cell.ch);
            }

            output.push_str("\x1b[0m");
            println!("{output}");
        }
    }
}

fn ansi(style: Style) -> String {
    let weight = if style.bold { 1 } else { 22 };

    format!(
        "\x1b[{weight};38;2;{};{};{};48;2;{};{};{}m",
        style.fg.r,
        style.fg.g,
        style.fg.b,
        style.bg.r,
        style.bg.g,
        style.bg.b,
    )
}

// ============================================================
// RIGHT CONTROL PLATE
// ============================================================

fn render_side(rows: usize, cols: usize) {
    let mut c = Canvas::new(rows, cols, CHASSIS);

    if rows == 0 || cols == 0 {
        return;
    }

    // ========================================================
    // SOLID CHASSIS MATERIAL
    // ========================================================

    c.solid_rect(
        0,
        0,
        rows,
        cols,
        CHASSIS_BG,
    );

    // Screen / hardware seam.
    for y in 0..rows {
        c.set(y, 0, '┃', EDGE_BRIGHT);

        if cols > 1 {
            c.set(y, 1, '│', EDGE);
        }
    }

    // ========================================================
    // SESSION
    // ========================================================

    draw_rotary(
        &mut c,
        1,
        "SESSION",
        "03",
        PointerDirection::Right,
        &[
            ("01", 7, 1),
            ("12", 1, 2),
            ("02", 12, 2),
            ("11", 0, 4),
            ("03", 13, 4),
            ("10", 1, 7),
            ("04", 12, 7),
        ],
        4,
    );

    // ========================================================
    // TAB
    // ========================================================

    draw_rotary(
        &mut c,
        14,
        "TAB",
        "02",
        PointerDirection::UpperRight,
        &[
            ("01", 7, 1),
            ("06", 1, 2),
            ("02", 12, 2),
            ("05", 0, 4),
            ("03", 13, 4),
            ("04", 7, 7),
        ],
        2,
    );

    // ========================================================
    // SLIDERS
    // ========================================================

    if rows > 38 {
        draw_slider(
            &mut c,
            27,
            "HORIZONTAL",
            "L",
            "R",
        );

        draw_slider(
            &mut c,
            34,
            "VERTICAL",
            "UP",
            "DN",
        );
    }

    // ========================================================
    // SPEAKER
    // ========================================================

    if rows > 45 {
        let top = 41;
        let x = 2;

        let width =
            cols.saturating_sub(4);

        let height =
            rows.saturating_sub(top + 2);

        if width >= 10 && height >= 5 {
            c.solid_rect(
                top,
                x,
                height,
                width,
                BLACK,
            );

            c.rect(
                top,
                x,
                height,
                width,
                Style {
                    fg: CYAN_DARK,
                    bg: BLACK,
                    bold: false,
                },
            );

            let grille_width =
                width.saturating_sub(4);

            let grille =
                "━".repeat(grille_width);

            // Dense, full-width speaker slots.
            for y in top + 2..top + height.saturating_sub(1) {
                c.write(
                    y,
                    x + 2,
                    &grille,
                    Style {
                        fg: CYAN_DARK,
                        bg: BLACK,
                        bold: false,
                    },
                );
            }
        }
    }

    c.render();
}

// ============================================================
// ROTARY
// ============================================================

enum PointerDirection {
    Right,
    UpperRight,
}

fn draw_rotary(
    c: &mut Canvas,
    top: usize,
    title: &str,
    value: &str,
    direction: PointerDirection,
    detents: &[(&str, usize, usize)],
    active_detent: usize,
) {
    c.centered(
        top,
        title,
        LABEL,
    );

    c.centered(
        top + 1,
        &format!("CH {value}"),
        ACTIVE,
    );

    // Housing tightened around knob.
    let well_width = 17;
    let well_height = 10;

    let x =
        c.cols.saturating_sub(well_width) / 2;

    let y =
        top + 3;

    if y + well_height > c.rows {
        return;
    }

    c.recessed_rect(
        y,
        x,
        well_height,
        well_width,
    );

    // Detent markings hug the knob rather than floating
    // around a huge instrument gauge.
    for (index, (text, dx, dy)) in detents.iter().enumerate() {
        let style =
            if index == active_detent {
                INSET_ACTIVE
            } else {
                INSET_CYAN
            };

        c.write(
            y + *dy,
            x + *dx,
            text,
            style,
        );
    }

    // ========================================================
    // ROUND DARK KNOB
    // ========================================================

    let knob_x = x + 4;
    let knob_y = y + 3;

    // Drop shadow underneath/right.
    c.write(
        knob_y + 4,
        knob_x + 2,
        "███████",
        Style {
            fg: BLACK,
            bg: BLACK_INSET,
            bold: true,
        },
    );

    // Round-ish face.
    c.write(
        knob_y,
        knob_x + 1,
        "▄█████▄",
        KNOB_FILL,
    );

    c.write(
        knob_y + 1,
        knob_x,
        "█████████",
        KNOB_FILL,
    );

    c.write(
        knob_y + 2,
        knob_x,
        "█████████",
        KNOB_FILL,
    );

    c.write(
        knob_y + 3,
        knob_x + 1,
        "▀███████▀",
        KNOB_FILL,
    );

    // Tiny darker outside edges.
    c.set(
        knob_y + 1,
        knob_x,
        '█',
        KNOB_BORDER,
    );

    c.set(
        knob_y + 2,
        knob_x + 8,
        '█',
        KNOB_BORDER,
    );

    // ========================================================
    // THICK WHITE ROTATING BAR
    // ========================================================

    match direction {
        PointerDirection::Right => {
            c.write(
                knob_y + 1,
                knob_x + 4,
                "●━━━",
                KNOB_POINTER,
            );

            c.write(
                knob_y + 2,
                knob_x + 4,
                "███",
                KNOB_POINTER,
            );
        }

        PointerDirection::UpperRight => {
            c.write(
                knob_y + 2,
                knob_x + 4,
                "●█",
                KNOB_POINTER,
            );

            c.write(
                knob_y + 1,
                knob_x + 5,
                "██",
                KNOB_POINTER,
            );

            c.set(
                knob_y,
                knob_x + 6,
                '█',
                KNOB_POINTER,
            );
        }
    }
}

// ============================================================
// DEEP SLIDER
// ============================================================

fn draw_slider(
    c: &mut Canvas,
    top: usize,
    title: &str,
    left: &str,
    right: &str,
) {
    if top + 6 >= c.rows {
        return;
    }

    c.centered(
        top,
        title,
        LABEL,
    );

    let width = 21;

    let x =
        c.cols.saturating_sub(width) / 2;

    // ========================================================
    // RECESSED MOUNTING WELL
    // ========================================================

    c.solid_rect(
        top + 1,
        x,
        5,
        width,
        BLACK,
    );

    c.rect(
        top + 1,
        x,
        5,
        width,
        Style {
            fg: CYAN_DARK,
            bg: BLACK,
            bold: false,
        },
    );

    // Deep central trench.
    c.write(
        top + 2,
        x + 3,
        "━━━━━━━━━━━━━━━",
        Style {
            fg: BLACK_INSET,
            bg: BLACK,
            bold: true,
        },
    );

    c.write(
        top + 3,
        x + 3,
        "━━━━━━━━━━━━━━━",
        Style {
            fg: CYAN_DARK,
            bg: BLACK,
            bold: false,
        },
    );

    // End marks.
    c.write(
        top + 4,
        x + 1,
        left,
        BLACK_CYAN,
    );

    let right_x =
        x + width
            .saturating_sub(
                right.chars().count() + 1
            );

    c.write(
        top + 4,
        right_x,
        right,
        BLACK_CYAN,
    );

    // ========================================================
    // BIG WHITE SLIDER HANDLE
    // ========================================================

    let handle_x =
        x + 8;

    // shadow
    c.write(
        top + 3,
        handle_x + 1,
        "█████",
        Style {
            fg: METAL_SHADOW,
            bg: BLACK,
            bold: true,
        },
    );

    // upper body
    c.write(
        top + 2,
        handle_x,
        "█████",
        Style {
            fg: WHITE,
            bg: BLACK,
            bold: true,
        },
    );

    // lower body
    c.write(
        top + 3,
        handle_x,
        "█████",
        Style {
            fg: WHITE_DIM,
            bg: BLACK,
            bold: true,
        },
    );

    // physical grip groove
    c.set(
        top + 2,
        handle_x + 2,
        '┃',
        Style {
            fg: METAL_SHADOW,
            bg: WHITE,
            bold: true,
        },
    );

    c.set(
        top + 3,
        handle_x + 2,
        '┃',
        Style {
            fg: METAL_SHADOW,
            bg: WHITE_DIM,
            bold: true,
        },
    );
}

// ============================================================
// BOTTOM RECEIVER
// ============================================================

fn render_deck(rows: usize, cols: usize) {
    let mut c = Canvas::new(rows, cols, CHASSIS);

    if rows == 0 || cols == 0 {
        return;
    }

    // ========================================================
    // SOLID RECEIVER CHASSIS
    // ========================================================

    c.solid_rect(
        0,
        0,
        rows,
        cols,
        CHASSIS_BG,
    );

    for x in 0..cols {
        c.set(
            0,
            x,
            '━',
            EDGE_BRIGHT,
        );
    }

    for y in 0..rows {
        c.set(
            y,
            0,
            '┃',
            EDGE_BRIGHT,
        );

        if cols > 1 {
            c.set(
                y,
                cols - 1,
                '┃',
                EDGE_BRIGHT,
            );
        }
    }

    // ========================================================
    // POWER
    // ========================================================

    c.write(
        1,
        3,
        "POWER",
        LABEL_DIM,
    );

    c.solid_rect(
        2,
        2,
        4,
        9,
        RAISED_PURPLE,
    );

    c.rect(
        2,
        2,
        4,
        9,
        POWER,
    );

    c.write(
        3,
        5,
        "◉",
        POWER,
    );

    c.write(
        4,
        3,
        "QUIT",
        POWER,
    );

    // ========================================================
    // DVD BAY
    // ========================================================

    let bay_x = 14;
    let bay_width = 58;

    if bay_x + bay_width < cols {
        c.solid_rect(
            1,
            bay_x,
            5,
            bay_width,
            BLACK,
        );

        c.rect(
            1,
            bay_x,
            5,
            bay_width,
            BLACK_CYAN_DARK,
        );

        c.write(
            2,
            bay_x + 3,
            "POST-APOLLO",
            Style {
                fg: WHITE_DIM,
                bg: BLACK,
                bold: false,
            },
        );

        c.write(
            2,
            bay_x + 17,
            "DIGITAL VIDEO / DATA DECK",
            BLACK_CYAN,
        );

        c.write(
            3,
            bay_x + 4,
            "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
            BLACK_CYAN_DARK,
        );

        c.write(
            4,
            bay_x + 20,
            "DVD / TERMINAL",
            BLACK_WHITE,
        );

        c.write(
            4,
            bay_x + bay_width - 9,
            "EJECT ▵",
            BLACK_CYAN,
        );
    }

    // ========================================================
    // VFD
    // ========================================================

    let vfd_x = 75;
    let vfd_width = 34;

    if vfd_x + vfd_width < cols {
        c.solid_rect(
            1,
            vfd_x,
            5,
            vfd_width,
            BLACK,
        );

        c.rect(
            1,
            vfd_x,
            5,
            vfd_width,
            BLACK_CYAN_DARK,
        );

        c.write(
            2,
            vfd_x + 2,
            "SESSION",
            VFD,
        );

        c.write(
            2,
            vfd_x + 11,
            "03",
            VFD_ACTIVE,
        );

        c.write(
            2,
            vfd_x + 17,
            "TAB",
            VFD,
        );

        c.write(
            2,
            vfd_x + 22,
            "02",
            VFD_ACTIVE,
        );

        c.write(
            3,
            vfd_x + 2,
            "MODE",
            VFD,
        );

        c.write(
            3,
            vfd_x + 8,
            "NORMAL",
            VFD_ACTIVE,
        );

        c.write(
            4,
            vfd_x + 2,
            "●",
            VFD_READY,
        );

        c.write(
            4,
            vfd_x + 4,
            "SYSTEM READY",
            VFD_READY,
        );
    }

    // ========================================================
    // BUTTON BANK
    // ========================================================

    let row = 6;

    // EDIT
    let g1 = 14;

    c.write(
        row,
        g1,
        "EDIT",
        LABEL_DIM,
    );

    draw_button(
        &mut c,
        row + 1,
        g1,
        10,
        "NEW",
    );

    draw_button(
        &mut c,
        row + 1,
        g1 + 12,
        10,
        "CLOSE",
    );

    // WINDOW
    let g2 = g1 + 29;

    c.write(
        row,
        g2,
        "WINDOW",
        LABEL_DIM,
    );

    draw_button(
        &mut c,
        row + 1,
        g2,
        10,
        "FULL",
    );

    draw_button(
        &mut c,
        row + 1,
        g2 + 12,
        10,
        "FLOAT",
    );

    draw_button(
        &mut c,
        row + 1,
        g2 + 24,
        10,
        "RENAME",
    );

    // SYSTEM
    let g3 = g2 + 41;

    c.write(
        row,
        g3,
        "SYSTEM",
        LABEL_DIM,
    );

    draw_button(
        &mut c,
        row + 1,
        g3,
        12,
        "OPTION",
    );

    // ========================================================
    // RIGHT CONTROL CLUSTER
    // ========================================================

    let cluster_width = 59;

    let cluster_x =
        cols.saturating_sub(
            cluster_width + 3
        );

    if cluster_x > 110 {
        draw_rocker(
            &mut c,
            1,
            cluster_x,
            "CONTROL",
            "PANE",
            "TAB",
            true,
        );

        draw_rocker(
            &mut c,
            1,
            cluster_x + 20,
            "SOURCE",
            "ZELLIJ",
            "MEDIA",
            true,
        );

        draw_mode_knob(
            &mut c,
            1,
            cluster_x + 41,
        );
    }

    c.render();
}

// ============================================================
// BUTTON
// ============================================================

fn draw_button(
    c: &mut Canvas,
    row: usize,
    col: usize,
    width: usize,
    label: &str,
) {
    if row + 2 >= c.rows ||
       col + width >= c.cols
    {
        return;
    }

    c.solid_rect(
        row,
        col,
        3,
        width,
        RAISED_PURPLE,
    );

    c.rect(
        row,
        col,
        3,
        width,
        EDGE,
    );

    // top highlight
    for x in col + 1..col + width - 1 {
        c.set(
            row,
            x,
            '━',
            EDGE_BRIGHT,
        );
    }

    // bottom shadow
    for x in col + 1..col + width - 1 {
        c.set(
            row + 2,
            x,
            '━',
            Style {
                fg: CYAN_DARK,
                bg: RAISED_PURPLE,
                bold: false,
            },
        );
    }

    let len =
        label.chars().count();

    c.write(
        row + 1,
        col + width.saturating_sub(len) / 2,
        label,
        RAISED_TEXT,
    );
}

// ============================================================
// ROCKER SWITCH
// ============================================================

fn draw_rocker(
    c: &mut Canvas,
    row: usize,
    col: usize,
    title: &str,
    left: &str,
    right: &str,
    left_selected: bool,
) {
    let width = 18;

    if row + 6 >= c.rows ||
       col + width >= c.cols
    {
        return;
    }

    c.write(
        row,
        col + 1,
        title,
        LABEL_DIM,
    );

    // Deep mounting recess.
    c.solid_rect(
        row + 1,
        col,
        5,
        width,
        BLACK,
    );

    c.rect(
        row + 1,
        col,
        5,
        width,
        BLACK_CYAN_DARK,
    );

    let left_style =
        if left_selected {
            Style {
                fg: ORANGE,
                bg: BLACK,
                bold: true,
            }
        } else {
            BLACK_CYAN
        };

    let right_style =
        if left_selected {
            BLACK_CYAN
        } else {
            Style {
                fg: ORANGE,
                bg: BLACK,
                bold: true,
            }
        };

    c.write(
        row + 2,
        col + 1,
        left,
        left_style,
    );

    let right_x =
        col + width.saturating_sub(
            right.chars().count() + 1
        );

    c.write(
        row + 2,
        right_x,
        right,
        right_style,
    );

    let left_x =
        col + 2;

    let right_switch_x =
        col + 10;

    // ========================================================
    // STRONGER SEE-SAW PERSPECTIVE
    // ========================================================

    if left_selected {
        // LEFT = physically pushed DOWN.
        c.write(
            row + 4,
            left_x,
            "▄▄▄▄▄▄",
            Style {
                fg: BLACK_INSET,
                bg: BLACK,
                bold: true,
            },
        );

        c.write(
            row + 5,
            left_x,
            "━━━━━━",
            Style {
                fg: CYAN_DARK,
                bg: BLACK,
                bold: false,
            },
        );

        // RIGHT = physically raised.
        c.write(
            row + 3,
            right_switch_x,
            "▀▀▀▀▀▀",
            Style {
                fg: CYAN_DIM,
                bg: RAISED_PURPLE,
                bold: true,
            },
        );

        c.write(
            row + 4,
            right_switch_x,
            "██████",
            Style {
                fg: RAISED_PURPLE,
                bg: BLACK,
                bold: true,
            },
        );

        c.write(
            row + 5,
            right_switch_x,
            "▀▀▀▀▀▀",
            Style {
                fg: CYAN_DARK,
                bg: BLACK,
                bold: true,
            },
        );
    } else {
        // LEFT = raised.
        c.write(
            row + 3,
            left_x,
            "▀▀▀▀▀▀",
            Style {
                fg: CYAN_DIM,
                bg: RAISED_PURPLE,
                bold: true,
            },
        );

        c.write(
            row + 4,
            left_x,
            "██████",
            Style {
                fg: RAISED_PURPLE,
                bg: BLACK,
                bold: true,
            },
        );

        c.write(
            row + 5,
            left_x,
            "▀▀▀▀▀▀",
            Style {
                fg: CYAN_DARK,
                bg: BLACK,
                bold: true,
            },
        );

        // RIGHT = down.
        c.write(
            row + 4,
            right_switch_x,
            "▄▄▄▄▄▄",
            Style {
                fg: BLACK_INSET,
                bg: BLACK,
                bold: true,
            },
        );

        c.write(
            row + 5,
            right_switch_x,
            "━━━━━━",
            Style {
                fg: CYAN_DARK,
                bg: BLACK,
                bold: false,
            },
        );
    }

    // mechanical hinge
    c.set(
        row + 4,
        col + 9,
        '◆',
        Style {
            fg: METAL_SHADOW,
            bg: BLACK,
            bold: false,
        },
    );
}

// ============================================================
// MODE KNOB
// ============================================================

fn draw_mode_knob(
    c: &mut Canvas,
    row: usize,
    col: usize,
) {
    let width = 16;

    if row + 7 >= c.rows ||
       col + width >= c.cols
    {
        return;
    }

    c.write(
        row,
        col + 5,
        "MODE",
        LABEL_DIM,
    );

    c.solid_rect(
        row + 1,
        col,
        7,
        width,
        BLACK,
    );

    c.rect(
        row + 1,
        col,
        7,
        width,
        BLACK_CYAN_DARK,
    );

    c.write(
        row + 2,
        col + 1,
        "MOVE",
        BLACK_CYAN,
    );

    c.write(
        row + 2,
        col + 10,
        "PANE",
        BLACK_CYAN,
    );

    let kx =
        col + 4;

    let ky =
        row + 3;

    // knob shadow
    c.write(
        ky + 3,
        kx + 2,
        "███████",
        Style {
            fg: BLACK_INSET,
            bg: BLACK,
            bold: true,
        },
    );

    // dark round body
    c.write(
        ky,
        kx + 1,
        "▄█████▄",
        Style {
            fg: KNOB_BODY,
            bg: BLACK,
            bold: true,
        },
    );

    c.write(
        ky + 1,
        kx,
        "█████████",
        Style {
            fg: KNOB_BODY,
            bg: BLACK,
            bold: true,
        },
    );

    c.write(
        ky + 2,
        kx + 1,
        "▀███████▀",
        Style {
            fg: KNOB_BODY,
            bg: BLACK,
            bold: true,
        },
    );

    // white pointer
    c.write(
        ky + 1,
        kx + 4,
        "●━━",
        Style {
            fg: WHITE,
            bg: KNOB_BODY,
            bold: true,
        },
    );

    c.write(
        row + 6,
        col + 1,
        "NORMAL",
        Style {
            fg: ORANGE,
            bg: BLACK,
            bold: true,
        },
    );
}

// ============================================================
// FALLBACK
// ============================================================

fn render_unknown(
    rows: usize,
    cols: usize,
) {
    let mut c =
        Canvas::new(
            rows,
            cols,
            CHASSIS,
        );

    c.solid_rect(
        0,
        0,
        rows,
        cols,
        CHASSIS_BG,
    );

    if rows > 0 {
        c.centered(
            rows / 2,
            "POST-APOLLO",
            ACTIVE,
        );
    }

    c.render();
}
