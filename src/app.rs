
const ICONCOUNT: usize = 30;
const VELOCITYMIN: f32 = 0.4;
const VELOCITYMAX: f32 = 0.7;
const GAMEWIDTH: f32 = 640.0;
const GAMEHEIGHT: f32 = 480.0;
const ICONSIZE: f32 = 25.0;

use eframe::egui;
use egui::Ui;

pub struct RockPaperScissors<'a> {
    rock: egui::Image<'a>,
    paper: egui::Image<'a>,
    scissors: egui::Image<'a>,
    iconsize: f32,
    fieldh: f32,
    fieldw: f32,
    icons: [IconData; ICONCOUNT],
    finished: bool,
}


impl<'a> RockPaperScissors<'a> {
    pub fn atsize(width: f32, height: f32, size: f32) -> Self {
        let rock_icon = egui::include_image!("../assets/rock.png");


        Self {
            iconsize : size,
            fieldh : height,
            fieldw : width,
            rock: egui::Image::new(rock_icon.clone()),
            paper: egui::Image::new(egui::include_image!("../assets/paper.png")),
            scissors: egui::Image::new(egui::include_image!("../assets/scissors.png")),
            icons: [IconData::default(); ICONCOUNT],
            finished: false,
        }
    }

    pub fn game_restart(game: &mut RockPaperScissors<'_>) {
        for icon in game.icons.iter_mut() {
            icon.icontype = random_icon_type();
            icon.position = random_position(ICONSIZE, GAMEWIDTH, GAMEHEIGHT);
            icon.velocity = random_velocity(VELOCITYMIN, VELOCITYMAX);
        }
        game.finished = false;
    }
}


fn random_icon_type() -> IconType{
    let i = rand::random::<u32>() % 3;
    match i {
        0 => IconType::Rock,
        1 => IconType::Paper,
        2 => IconType::Scissors,
        _ => IconType::Rock,
    }
}

fn random_position(iconsize: f32, width: f32, height: f32) -> egui::Pos2 {
    let mut rng = rand::thread_rng();
    let x: f32 = iconsize/2.0 + rand::Rng::gen::<f32>(&mut rng) * (width - iconsize); 
    let y: f32 = iconsize/2.0 + rand::Rng::gen::<f32>(&mut rng) * (height - iconsize); 
    egui::Pos2::new(x, y)
}

fn random_velocity(velocitymin: f32, velocitymax: f32) -> egui::Vec2 {
    let mut rng = rand::thread_rng();
    let dir = rand::Rng::gen::<f32>(&mut rng) * 360.0;
    let velo = velocitymin + rand::Rng::gen::<f32>(&mut rng) * (velocitymax - velocitymin);
    let x = dir.sin() * velo;
    let y = dir.cos() * velo;
    egui::Vec2::new(x, y)
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum IconType {
    Rock,
    Paper,
    Scissors,
}

impl std::fmt::Display for IconType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            IconType::Rock => write!(f, "Rock"),
            IconType::Paper => write!(f, "Paper"),
            IconType::Scissors => write!(f, "Scissors"),
        }
    }
}

#[derive(Copy, Clone)]
struct IconData {
    icontype: IconType,
    position: egui::Pos2,
    velocity: egui::Vec2,
}

impl IconData {
    fn default() -> Self {
        Self {
            icontype: IconType::Rock,
            position: egui::Pos2::new(0.0, 0.0),
            velocity: egui::Vec2::new(0.0, 0.0),
        }
    }
}


fn moveposition(icon: &mut IconData, xmin: f32, xmax: f32, ymin: f32, ymax: f32) {
    let mut x = icon.position.x + icon.velocity.x;
    let mut y = icon.position.y + icon.velocity.y;
    if x < xmin {
        icon.velocity.x = -icon.velocity.x;
        x = icon.position.x + icon.velocity.x;
    }
    if x > xmax {
        icon.velocity.x = -icon.velocity.x;
        x = icon.position.x + icon.velocity.x;
    }
    if y < ymin {
        icon.velocity.y = -icon.velocity.y;
        y = icon.position.y + icon.velocity.y;
    }
    if y > ymax {
        icon.velocity.y = -icon.velocity.y;
        y = icon.position.y + icon.velocity.y;
    }

    icon.position.x = x;
    icon.position.y = y;
}

fn icon_distace(a: IconData, b: &mut IconData) -> f32 {
    let dx = a.position.x - b.position.x;
    let dy = a.position.y - b.position.y;
    (dx*dx + dy*dy).sqrt()
}

fn paint_icons(game: &mut RockPaperScissors<'_>,ui: &mut Ui) {
    let xmin = game.iconsize / 2.0;
    let xmax = game.fieldw - game.iconsize / 2.0;
    let ymin = game.iconsize / 2.0;
    let ymax = game.fieldh - game.iconsize / 2.0;
    let size = egui::Vec2::new(game.iconsize, game.iconsize);

    // collisions check
    for icon in game.icons {
        for bicon in game.icons.iter_mut() {
            if icon_distace(icon, bicon) < game.iconsize {
                if bicon.icontype == IconType::Rock && icon.icontype == IconType::Paper {
                    bicon.icontype = IconType::Paper;
                } 
                if bicon.icontype == IconType::Paper && icon.icontype == IconType::Scissors {
                    bicon.icontype = IconType::Scissors;
                }
                if bicon.icontype == IconType::Scissors && icon.icontype == IconType::Rock {
                    bicon.icontype = IconType::Rock;
                }
            }
        }
    }

    // game end check
    let firsttype = game.icons[0].icontype;
    game.finished = true;
    for icon in game.icons {
        if icon.icontype != firsttype {
            game.finished = false;
            break;
        }
    }

    for icon in game.icons.iter_mut() {
        if !game.finished {
            moveposition(icon, xmin, xmax, ymin, ymax);
        }

        let rect = egui::Rect::from_center_size(icon.position, size);

        if icon.icontype == IconType::Rock {
            game.rock.paint_at(ui, rect)
        }
        if icon.icontype == IconType::Paper {
            game.paper.paint_at(ui, rect)
        }
        if icon.icontype == IconType::Scissors {
            game.scissors.paint_at(ui, rect)
        }
    }

    if !game.finished {
        return;
    }
    egui::Frame::default()
        .inner_margin(24.0)
        .fill(egui::Color32::from_rgba_unmultiplied(125, 135, 113, 184))
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .rounding(ui.visuals().widgets.noninteractive.rounding)
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {

                ui.label(format!("Game has ended with victory for {}", firsttype));
                if ui.button("New game").clicked() {
                    RockPaperScissors::game_restart(game);
                }
            });
        });
    
}


impl<'a> eframe::App for RockPaperScissors<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui| paint_icons(self, ui));
    }
}