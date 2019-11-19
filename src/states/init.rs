use crate::d2::D2;
use crate::d2assetsource;
use crate::d2assetsource::D2AssetSource;
use amethyst::{
    assets::{Loader, ProgressCounter},
    GameData, SimpleState, SimpleTrans, StateData, Trans,
};
use amethyst::prelude::WorldExt;

pub struct InitState {
    pub progress_counter: ProgressCounter,
}

impl InitState {
    pub fn new() -> Self {
        InitState {
            progress_counter: ProgressCounter::new(),
        }
    }
}

impl SimpleState for InitState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut loader = data.world.write_resource::<Loader>();
        let mut mpq_source = D2AssetSource::new("G:\\Diablo II");
        mpq_source.add_mpq("d2data.mpq").expect("whoa");
        mpq_source.add_mpq("d2exp.mpq").expect("whoa");
        mpq_source.add_mpq("patch_d2.mpq").expect("whoa");
        loader.add_source(d2assetsource::SOURCE_NAME, mpq_source);
    }

    fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::Switch(Box::new(D2::new()))
    }
}
