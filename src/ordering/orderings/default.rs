use crate::{
    art::{Art, Pixel},
    ordering::Ordering,
};

pub fn generate_fast(art: &Art) -> Ordering {
    let mut data = Vec::new();
    for (row, y) in art.rows().zip(0..) {
        for (&color, x) in row.iter().zip(0..) {
            if let Some(color) = color {
                data.push(Pixel::new(x, y, color));
            }
        }
    }
    Ordering { data }
}
