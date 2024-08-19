use cirno_pet::{ImgBuilderInfo, VirtualPetBuilder};

fn main() {
    let mut pet = VirtualPetBuilder::new()
        .vsync()
        .show_hitbox()
        .animation_fps(5)
        .fps(60)
        .window_position(100., 100.)
        .img(
            ImgBuilderInfo::new()
                .filepath("assets/cirno.png")
                .width_slice(220)
                .idle_row(8)
                .move_row(9)
                .height_slice(256)
                .max_animation_frame(7),
        )
        .build();

    pet.run();
}
