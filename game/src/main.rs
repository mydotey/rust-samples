use anyhow::Result;
use eframe::egui;
use std::collections::HashMap;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Chinese Chess Game",
        options,
        Box::new(|cc| {
            let mut app = ChineseChessApp::default();
            app.load_textures(&cc.egui_ctx);
            Box::new(app)
        }),
    )
}

struct ChineseChessApp {
    board: [[Option<Piece>; 9]; 10],
    selected_piece: Option<(usize, usize)>,
    current_player: Player,
    textures: HashMap<String, egui::TextureHandle>,
    dark_mode: bool,
}

#[derive(Clone, Copy, PartialEq, Default)]
enum Player {
    #[default]
    Red,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
enum PieceType {
    General,
    Advisor,
    Elephant,
    Horse,
    Chariot,
    Cannon,
    Soldier,
}

#[derive(Clone, Copy)]
struct Piece {
    piece_type: PieceType,
    player: Player,
}

impl Default for ChineseChessApp {
    fn default() -> Self {
        let mut board = [[None; 9]; 10];

        // Initialize red pieces (bottom)
        board[9][0] = Some(Piece {
            piece_type: PieceType::Chariot,
            player: Player::Red,
        });
        board[9][1] = Some(Piece {
            piece_type: PieceType::Horse,
            player: Player::Red,
        });
        board[9][2] = Some(Piece {
            piece_type: PieceType::Elephant,
            player: Player::Red,
        });
        board[9][3] = Some(Piece {
            piece_type: PieceType::Advisor,
            player: Player::Red,
        });
        board[9][4] = Some(Piece {
            piece_type: PieceType::General,
            player: Player::Red,
        });
        board[9][5] = Some(Piece {
            piece_type: PieceType::Advisor,
            player: Player::Red,
        });
        board[9][6] = Some(Piece {
            piece_type: PieceType::Elephant,
            player: Player::Red,
        });
        board[9][7] = Some(Piece {
            piece_type: PieceType::Horse,
            player: Player::Red,
        });
        board[9][8] = Some(Piece {
            piece_type: PieceType::Chariot,
            player: Player::Red,
        });
        board[7][1] = Some(Piece {
            piece_type: PieceType::Cannon,
            player: Player::Red,
        });
        board[7][7] = Some(Piece {
            piece_type: PieceType::Cannon,
            player: Player::Red,
        });
        board[6][0] = Some(Piece {
            piece_type: PieceType::Soldier,
            player: Player::Red,
        });
        board[6][2] = Some(Piece {
            piece_type: PieceType::Soldier,
            player: Player::Red,
        });
        board[6][4] = Some(Piece {
            piece_type: PieceType::Soldier,
            player: Player::Red,
        });
        board[6][6] = Some(Piece {
            piece_type: PieceType::Soldier,
            player: Player::Red,
        });
        board[6][8] = Some(Piece {
            piece_type: PieceType::Soldier,
            player: Player::Red,
        });

        // Initialize black pieces (top)
        board[0][0] = Some(Piece {
            piece_type: PieceType::Chariot,
            player: Player::Black,
        });
        board[0][1] = Some(Piece {
            piece_type: PieceType::Horse,
            player: Player::Black,
        });
        board[0][2] = Some(Piece {
            piece_type: PieceType::Elephant,
            player: Player::Black,
        });
        board[0][3] = Some(Piece {
            piece_type: PieceType::Advisor,
            player: Player::Black,
        });
        board[0][4] = Some(Piece {
            piece_type: PieceType::General,
            player: Player::Black,
        });
        board[0][5] = Some(Piece {
            piece_type: PieceType::Advisor,
            player: Player::Black,
        });
        board[0][6] = Some(Piece {
            piece_type: PieceType::Elephant,
            player: Player::Black,
        });
        board[0][7] = Some(Piece {
            piece_type: PieceType::Horse,
            player: Player::Black,
        });
        board[0][8] = Some(Piece {
            piece_type: PieceType::Chariot,
            player: Player::Black,
        });
        board[2][1] = Some(Piece {
            piece_type: PieceType::Cannon,
            player: Player::Black,
        });
        board[2][7] = Some(Piece {
            piece_type: PieceType::Cannon,
            player: Player::Black,
        });
        board[3][0] = Some(Piece {
            piece_type: PieceType::Soldier,
            player: Player::Black,
        });
        board[3][2] = Some(Piece {
            piece_type: PieceType::Soldier,
            player: Player::Black,
        });
        board[3][4] = Some(Piece {
            piece_type: PieceType::Soldier,
            player: Player::Black,
        });
        board[3][6] = Some(Piece {
            piece_type: PieceType::Soldier,
            player: Player::Black,
        });
        board[3][8] = Some(Piece {
            piece_type: PieceType::Soldier,
            player: Player::Black,
        });

        Self {
            board,
            selected_piece: None,
            current_player: Player::default(),
            textures: HashMap::new(),
            dark_mode: false,
        }
    }
}

impl eframe::App for ChineseChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Chinese Chess Game");

            // Draw the chess board
            let board_size = 400.0;
            let cell_size = board_size / 9.0;
            let (response, painter) = ui.allocate_painter(
                egui::vec2(board_size, board_size * 10.0 / 9.0),
                egui::Sense::click(),
            );

            // Draw board background with toggleable color
            let bg_color = if self.dark_mode {
                egui::Color32::from_rgb(50, 50, 50) // Dark background
            } else {
                egui::Color32::from_rgb(210, 180, 140) // Light background
            };
            painter.rect_filled(response.rect, 0.0, bg_color);

            // Draw grid lines
            for i in 0..10 {
                painter.line_segment(
                    [
                        egui::pos2(
                            response.rect.left(),
                            response.rect.top() + i as f32 * cell_size,
                        ),
                        egui::pos2(
                            response.rect.right(),
                            response.rect.top() + i as f32 * cell_size,
                        ),
                    ],
                    egui::Stroke::new(2.0, egui::Color32::BLACK),
                );
            }
            for j in 0..9 {
                painter.line_segment(
                    [
                        egui::pos2(
                            response.rect.left() + j as f32 * cell_size,
                            response.rect.top(),
                        ),
                        egui::pos2(
                            response.rect.left() + j as f32 * cell_size,
                            response.rect.bottom(),
                        ),
                    ],
                    egui::Stroke::new(2.0, egui::Color32::BLACK),
                );
            }

            // Draw pieces
            for row in 0..10 {
                for col in 0..9 {
                    if let Some(piece) = self.board[row][col] {
                        let x = response.rect.left() + col as f32 * cell_size + cell_size / 2.0;
                        let y = response.rect.top() + row as f32 * cell_size + cell_size / 2.0;

                        let color = match piece.player {
                            Player::Red => egui::Color32::RED,
                            Player::Black => egui::Color32::BLACK,
                        };

                        painter.circle_filled(egui::pos2(x, y), cell_size * 0.4, color);

                        // Draw piece using image texture
                        let texture_name = match (piece.player, piece.piece_type) {
                            (Player::Red, PieceType::General) => "red_general",
                            (Player::Red, PieceType::Advisor) => "red_advisor",
                            (Player::Red, PieceType::Elephant) => "red_elephant",
                            (Player::Red, PieceType::Horse) => "red_horse",
                            (Player::Red, PieceType::Chariot) => "red_chariot",
                            (Player::Red, PieceType::Cannon) => "red_cannon",
                            (Player::Red, PieceType::Soldier) => "red_soldier",
                            (Player::Black, PieceType::General) => "black_general",
                            (Player::Black, PieceType::Advisor) => "black_advisor",
                            (Player::Black, PieceType::Elephant) => "black_elephant",
                            (Player::Black, PieceType::Horse) => "black_horse",
                            (Player::Black, PieceType::Chariot) => "black_chariot",
                            (Player::Black, PieceType::Cannon) => "black_cannon",
                            (Player::Black, PieceType::Soldier) => "black_soldier",
                        };

                        if let Some(texture) = self.textures.get(texture_name) {
                            let size = egui::vec2(cell_size * 0.8, cell_size * 0.8);
                            let rect = egui::Rect::from_center_size(egui::pos2(x, y), size);
                            painter.image(
                                texture.id(),
                                rect,
                                egui::Rect::from_min_max(
                                    egui::pos2(0.0, 0.0),
                                    egui::pos2(1.0, 1.0),
                                ),
                                egui::Color32::WHITE,
                            );
                        }
                    }
                }
            }

            // Handle click events
            if response.clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let col = ((pos.x - response.rect.left()) / cell_size) as usize;
                    let row = ((pos.y - response.rect.top()) / cell_size) as usize;

                    if row < 10 && col < 9 {
                        self.handle_click(row, col);
                    }
                }
            }

            // Display current player and background toggle
            ui.horizontal(|ui| {
                ui.label(format!(
                    "Current player: {}",
                    match self.current_player {
                        Player::Red => "Red",
                        Player::Black => "Black",
                    }
                ));

                // Background color toggle button
                if ui
                    .button(if self.dark_mode {
                        "☀️ Light Mode"
                    } else {
                        "🌙 Dark Mode"
                    })
                    .clicked()
                {
                    self.dark_mode = !self.dark_mode;
                }
            });
        });
    }
}

impl ChineseChessApp {
    fn load_textures(&mut self, ctx: &egui::Context) {
        let piece_names = [
            "red_general",
            "red_advisor",
            "red_elephant",
            "red_horse",
            "red_chariot",
            "red_cannon",
            "red_soldier",
            "black_general",
            "black_advisor",
            "black_elephant",
            "black_horse",
            "black_chariot",
            "black_cannon",
            "black_soldier",
        ];

        for name in piece_names {
            let size = 64;
            let mut image = egui::ColorImage::new([size, size], egui::Color32::TRANSPARENT);

            // Determine colors based on piece name
            let (circle_color, text_color) = if name.starts_with("red") {
                (egui::Color32::RED, egui::Color32::WHITE)
            } else {
                (egui::Color32::BLACK, egui::Color32::WHITE)
            };

            let center = size as f32 / 2.0;
            let radius = size as f32 / 2.5;

            // Draw colored circle
            for y in 0..size {
                for x in 0..size {
                    let dx = x as f32 - center;
                    let dy = y as f32 - center;
                    if dx * dx + dy * dy <= radius * radius {
                        image[(x as usize, y as usize)] = circle_color;
                    }
                }
            }

            // Get Chinese character for the piece
            let chinese_char = match name {
                "red_general" => "帥",    // Red General
                "black_general" => "將",  // Black General
                "red_advisor" => "仕",    // Red Advisor
                "black_advisor" => "士",  // Black Advisor
                "red_elephant" => "相",   // Red Elephant
                "black_elephant" => "象", // Black Elephant
                "red_horse" => "傌",      // Red Horse
                "black_horse" => "馬",    // Black Horse
                "red_chariot" => "俥",    // Red Chariot
                "black_chariot" => "車",  // Black Chariot
                "red_cannon" => "炮",     // Red Cannon
                "black_cannon" => "砲",   // Black Cannon
                "red_soldier" => "兵",    // Red Soldier
                "black_soldier" => "卒",  // Black Soldier
                _ => "?",
            };

            // Draw Chinese character using a simple block pattern for better visibility
            // We'll create a large, bold representation of each character
            let char_width = 20;
            let char_height = 20;
            let char_x = (center - char_width as f32 / 2.0) as usize;
            let char_y = (center - char_height as f32 / 2.0) as usize;

            // Draw a bold representation of each Chinese character
            // Each character is represented by a unique pattern of filled blocks
            match chinese_char {
                "帥" | "將" => {
                    // General - draw a square with center dot
                    for y in 0..char_height {
                        for x in 0..char_width {
                            if (x < 3 || x > char_width - 4) && (y < 3 || y > char_height - 4) {
                                image[(char_x + x, char_y + y)] = text_color;
                            }
                            if x >= char_width / 2 - 2
                                && x <= char_width / 2 + 1
                                && y >= char_height / 2 - 2
                                && y <= char_height / 2 + 1
                            {
                                image[(char_x + x, char_y + y)] = text_color;
                            }
                        }
                    }
                }
                "仕" | "士" => {
                    // Advisor - draw a diamond shape
                    for y in 0..char_height {
                        for x in 0..char_width {
                            let center_x = char_width as isize / 2;
                            let center_y = char_height as isize / 2;
                            if ((x as isize - center_x).abs() + (y as isize - center_y).abs()) <= 6
                            {
                                image[(char_x + x, char_y + y)] = text_color;
                            }
                        }
                    }
                }
                "相" | "象" => {
                    // Elephant - draw a large X shape
                    for y in 0..char_height {
                        for x in 0..char_width {
                            if (x as isize - char_width as isize / 2).abs()
                                == (y as isize - char_height as isize / 2).abs()
                                || (x as isize - char_width as isize / 2).abs()
                                    == (y as isize - char_height as isize / 2).abs() + 1
                            {
                                image[(char_x + x, char_y + y)] = text_color;
                            }
                        }
                    }
                }
                "傌" | "馬" => {
                    // Horse - draw an H-like shape
                    for y in 0..char_height {
                        for x in 0..char_width {
                            if x < 3
                                || x > char_width - 4
                                || (y > char_height / 3
                                    && y < 2 * char_height / 3
                                    && x > char_width / 3
                                    && x < 2 * char_width / 3)
                            {
                                image[(char_x + x, char_y + y)] = text_color;
                            }
                        }
                    }
                }
                "俥" | "車" => {
                    // Chariot - draw a plus sign
                    for y in 0..char_height {
                        for x in 0..char_width {
                            if x == char_width / 2
                                || y == char_height / 2
                                || (x > char_width / 2 - 3
                                    && x < char_width / 2 + 2
                                    && y > char_height / 2 - 3
                                    && y < char_height / 2 + 2)
                            {
                                image[(char_x + x, char_y + y)] = text_color;
                            }
                        }
                    }
                }
                "炮" | "砲" => {
                    // Cannon - draw a circle with dot
                    for y in 0..char_height {
                        for x in 0..char_width {
                            let dx = x as f32 - char_width as f32 / 2.0;
                            let dy = y as f32 - char_height as f32 / 2.0;
                            if dx * dx + dy * dy <= 25.0 {
                                // Circle
                                image[(char_x + x, char_y + y)] = text_color;
                            }
                            if dx * dx + dy * dy <= 4.0 {
                                // Center dot
                                image[(char_x + x, char_y + y)] = text_color;
                            }
                        }
                    }
                }
                "兵" | "卒" => {
                    // Soldier - draw a simple cross
                    for y in 0..char_height {
                        for x in 0..char_width {
                            if x == char_width / 2
                                || y == char_height / 2
                                || (x > char_width / 2 - 2
                                    && x < char_width / 2 + 1
                                    && y > char_height / 2 - 2
                                    && y < char_height / 2 + 1)
                            {
                                image[(char_x + x, char_y + y)] = text_color;
                            }
                        }
                    }
                }
                _ => {
                    // Fallback - draw a question mark
                    for y in 0..char_height {
                        for x in 0..char_width {
                            if (x == char_width / 2 && y < 3 * char_height / 4)
                                || (y == 3 * char_height / 4
                                    && x > char_width / 3
                                    && x < 2 * char_width / 3)
                                || (x + y == char_width && y > char_height / 2)
                            {
                                image[(char_x + x, char_y + y)] = text_color;
                            }
                        }
                    }
                }
            }

            let texture = ctx.load_texture(name, image, egui::TextureOptions::default());
            self.textures.insert(name.to_string(), texture);
        }
    }

    fn handle_click(&mut self, row: usize, col: usize) {
        if let Some((selected_row, selected_col)) = self.selected_piece {
            // Try to move piece
            if self.is_valid_move(selected_row, selected_col, row, col) {
                self.board[row][col] = self.board[selected_row][selected_col].take();
                self.current_player = match self.current_player {
                    Player::Red => Player::Black,
                    Player::Black => Player::Red,
                };
            }
            self.selected_piece = None;
        } else if let Some(piece) = self.board[row][col] {
            // Select piece if it belongs to current player
            if (piece.player == Player::Red && self.current_player == Player::Red)
                || (piece.player == Player::Black && self.current_player == Player::Black)
            {
                self.selected_piece = Some((row, col));
            }
        }
    }

    fn is_valid_move(
        &self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) -> bool {
        // Basic validation - implement proper Chinese chess rules here
        if from_row == to_row && from_col == to_col {
            return false;
        }

        if let Some(piece) = self.board[from_row][from_col] {
            // Check if destination is empty or contains opponent's piece
            if let Some(target_piece) = self.board[to_row][to_col] {
                if target_piece.player == piece.player {
                    return false;
                }
            }

            // TODO: Implement proper movement rules for each piece type
            // For now, allow any move for demonstration
            return true;
        }

        false
    }
}
