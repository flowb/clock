use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    mainspring: f32,
    rate:       f32,
    num_hands:  i8,
    gear_ratio: i8,
    egui:       Egui,
}

fn model(app: &App) -> Model {
    let main_window = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    Model {
        mainspring: 0.0,
        rate: 1.0,
        num_hands: 3,
        gear_ratio: 10,
        egui: Egui::from_window(&app.window(main_window).unwrap()),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    //turn mainspring
    model.mainspring += app.duration.since_prev_update.as_secs_f32() * model.rate;

    //egui
    //model.egui.set_elapsed_time(update.since_start);
    let ctx = model.egui.begin_frame();

    egui::Window::new("Controls").show(&ctx, |ui| {
        ui.label("rate:");
        ui.add(egui::Slider::new(&mut model.rate, -10.0..=10.0));
        ui.label("number of hands:");
        ui.add(egui::Slider::new(&mut model.num_hands, 1..=8));
        ui.label("gear ratio:");
        ui.add(egui::Slider::new(&mut model.gear_ratio,2..=60));
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    //clock face
    let window_bounds = app.window_rect().pad(16.0);
    let face_radius = window_bounds.w().min(window_bounds.h())/2.0;
    let bounding_box = |pos_func| { map_range(pos_func, -1.0,1.0,-face_radius,face_radius)};

    //hands
    for h in 0..model.num_hands {
        let length = (model.num_hands-h) as f32 / model.num_hands as f32;
        let ratio = (model.gear_ratio as f32).pow(h as f32);
        let x = bounding_box((model.mainspring/ratio).sin() * length);
        let y = bounding_box((model.mainspring/ratio).cos() * length);
        let position = Vec2::new(x,y);

        draw.ellipse()
            .no_fill()
            .radius(( face_radius ) * length )
            .stroke_color(SLATEBLUE)
            .stroke_weight(2.0);
        draw.arrow()
            .points(position*-0.25, position)
            .stroke_weight(16.0 / (h+1) as f32)
            .head_width(20.0 / (h+1) as f32)
            .color(MINTCREAM);    
    }
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();

}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}