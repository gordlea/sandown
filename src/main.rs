use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_pixel_buffer::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugins(PixelBufferPlugins)
        .add_startup_system(
            PixelBufferBuilder::new()
                .with_size(PixelBufferSize::pixel_size((1, 1)))
                .with_render(false) // disable rendering, as we'll do in it egui
                .with_fill(Fill::stretch()) // Pixels will stretch to fill the area
                .setup(),
        )
        // .add_system(setup)
        .add_system(update)
        .run()
}

// pub struct Elements {
//     pub EMPTY: Color,
// }



pub enum Elements {
}

impl Elements {
    pub const EMPTY: u32 = 0xffffffff;
    pub const SAND: u32 = 0xffff00ff;
    pub const WATER: u32 = 0x0000ffff;
}




fn update(mut egui_context: EguiContexts, mut pb: QueryPixelBuffer) {
    // update the frame
    pb.frame().per_pixel(|_, _| Pixel::random());
    // let size = pb.frame().size();
    // pb.frame().set(location, pixel)
    // pb.frame().per_pixel_par(|pos, p| {
    //     if (p.as_color().as_rgba_u32() == Elements::SAND) {
    //         let belowLoc = UVec2::new(pos.x, (pos.y + 1));
    //         if (belowLoc.y < size.y) {
    //             let below = pb.frame().raw_mut()[pos.x + (pos.y + 1) * size.x];
    //         }
    //         pb.frame()
            
    //     }
    // });

    // show ui
    let ctx = egui_context.ctx_mut();
    egui::SidePanel::left("left_panel").show(ctx, |ui| {
        ui.heading("My controls");
        ui.label("Look! Pixels!!")
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        // update Fill component with available size
        pb.update_fill_egui(ui.available_size());

        // get the texture
        let texture = pb.egui_texture();

        // show it
        ui.image(texture.id, texture.size);
    });
}
