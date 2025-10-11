use anyhow::Result;
use eframe::egui;
use egui_extras::image::load_svg_bytes;
use resvg::usvg;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

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
            Ok(Box::new(app))
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

                        // Draw colored circle background first
                        painter.circle_filled(egui::pos2(x, y), cell_size * 0.45, color);

                        // Draw piece using image texture on top of the colored circle
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
                            println!(
                                "Drawing texture: {} at position ({}, {}), texture size: {:?}",
                                texture_name,
                                x,
                                y,
                                texture.size()
                            );
                            let size = egui::vec2(cell_size * 0.8, cell_size * 0.8);
                            let rect = egui::Rect::from_center_size(egui::pos2(x, y), size);
                            println!("Drawing rect: {:?}", rect);

                            // Draw a debug rectangle around the image area
                            painter.rect_stroke(
                                rect,
                                0.0,
                                egui::Stroke::new(1.0, egui::Color32::GREEN),
                                egui::StrokeKind::Inside,
                            );

                            painter.image(
                                texture.id(),
                                rect,
                                egui::Rect::from_min_max(
                                    egui::pos2(0.0, 0.0),
                                    egui::pos2(1.0, 1.0),
                                ),
                                egui::Color32::WHITE,
                            );
                        } else {
                            println!("Texture not found: {}", texture_name);
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
            // Get the path to the assets directory relative to the executable
            let mut svg_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            svg_path.push("assets");
            svg_path.push("images");
            svg_path.push(format!("{}.svg", name));

            // Create a simple test texture instead of loading SVG
            // Try to load SVG file
            if let Ok(svg_bytes) = std::fs::read(&svg_path) {
                println!("Loading SVG file: {:?}", svg_path);
                let options = usvg::Options::default();
                match load_svg_bytes(&svg_bytes, &options) {
                    Ok(image) => {
                        let texture = ctx.load_texture(name, image, egui::TextureOptions::LINEAR);
                        self.textures.insert(name.to_string(), texture);
                        println!("Successfully loaded SVG texture for: {}", name);
                    }
                    Err(e) => {
                        println!("Failed to load SVG for {}: {:?}", name, e);
                        // Fallback to test texture
                        self.create_test_texture(ctx, name);
                    }
                }
            } else {
                println!("SVG file not found: {:?}, using test texture", svg_path);
                // Fallback to test texture
                self.create_test_texture(ctx, name);
            }
        }
    }

    fn create_test_texture(&mut self, ctx: &egui::Context, name: &str) {
        // Create a more visible test pattern with different colors and patterns
        let mut image = egui::ColorImage::new([100, 100], vec![egui::Color32::TRANSPARENT]);

        // Different patterns based on piece type for better visibility
        for y in 0..100 {
            for x in 0..100 {
                if name.contains("red") {
                    // Red pieces - solid red background with white pattern
                    image[(x, y)] = egui::Color32::RED;

                    // Add white cross pattern for visibility
                    if x == y || x == 99 - y || x == 50 || y == 50 {
                        image[(x, y)] = egui::Color32::WHITE;
                    }
                } else {
                    // Black pieces - solid blue background with yellow pattern
                    image[(x, y)] = egui::Color32::BLUE;

                    // Add yellow pattern for visibility (checkerboard)
                    if (x / 20 + y / 20) % 2 == 0 {
                        image[(x, y)] = egui::Color32::YELLOW;
                    }
                }
            }
        }

        let texture = ctx.load_texture(name, image, egui::TextureOptions::LINEAR);
        self.textures.insert(name.to_string(), texture);
        println!("Created test texture for: {}", name);
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
