pub use iced::Font;
use iced::font::Family;
pub use iced::font::Weight;
use iced::Program;
use iced::program::Definition;

pub fn load_fonts<P: Definition>(program: Program<P>) -> Program<P> {
    program
        .font(include_bytes!("resources/fonts/inter-black.ttf"))
        .font(include_bytes!("resources/fonts/inter-bold.ttf"))
        .font(include_bytes!("resources/fonts/inter-extrabold.ttf"))
        .font(include_bytes!("resources/fonts/inter-extralight.ttf"))
        .font(include_bytes!("resources/fonts/inter-light.ttf"))
        .font(include_bytes!("resources/fonts/inter-medium.ttf"))
        .font(include_bytes!("resources/fonts/inter-regular.ttf"))
        .font(include_bytes!("resources/fonts/inter-semibold.ttf"))
        .font(include_bytes!("resources/fonts/inter-thin.ttf"))
}

pub fn inter(weight: Weight) -> Font {
    Font {
        weight,
        family: Family::Name("Inter"),
        ..Font::DEFAULT
    }
}
