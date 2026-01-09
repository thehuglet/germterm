use bitflags::bitflags;

bitflags! {
    #[derive(Clone)]
    pub struct Attributes: u8 {
        const BOLD = 0b0000001;
        const ITALIC = 0b0000010;
    }
}
