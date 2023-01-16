use bevy::prelude::*;

use crate::art::{Art, Pixel};

mod orderings;

pub struct OrderingPlugin;

impl Plugin for OrderingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Orderings>()
            .add_plugin(orderings::OrderingsPlugin);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderingType {
    #[default]
    Default = 0,
    SideToSide = 1,
    Spiral = 2,
}

#[derive(Debug, Clone)]
pub struct Ordering {
    data: Vec<Pixel>,
}

#[derive(Debug, Default, Resource)]
pub struct Orderings([Option<Ordering>; 3]);

impl Orderings {
    pub fn get_or_generate(&mut self, typ: OrderingType, art: &Art) -> &Ordering {
        let r = &mut self.0[typ as usize];
        if let Some(ordering) = r {
            ordering
        } else {
            let ordering = match typ {
                OrderingType::Default => orderings::default::generate_fast(art),
                OrderingType::SideToSide => orderings::side_to_side::generate_fast(art),
                OrderingType::Spiral => orderings::spiral::generate_fast(art),
            };
            *r = Some(ordering);
            r.as_ref().unwrap()
        }
    }
}

#[derive(Debug, Resource)]
pub struct CurrentOrdering {
    ordering: Ordering,
    pos: usize,
}

impl CurrentOrdering {
    pub fn init(orderings: &mut Orderings, typ: OrderingType, art: &Art) -> Self {
        Self {
            ordering: orderings.get_or_generate(typ, art).clone(),
            pos: 0,
        }
    }

    pub fn peek(&self) -> Option<Pixel> {
        self.ordering.data.get(self.pos).copied()
    }

    pub fn next(&mut self) -> Option<Pixel> {
        let ret = self.peek();
        self.pos += 1;
        ret
    }
}
