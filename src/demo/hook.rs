use crate::asset_tracking::LoadResource;
use crate::utils::love_to_bevy_coords;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<HookAssets>();
}

#[derive(Component)]
pub struct Hook;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct HookAssets {
    #[dependency]
    hook: Handle<Image>,
}

impl FromWorld for HookAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            hook: assets.load("images/hook_sheet.png"),
        }
    }
}

pub fn hook(
    hook_assets: &HookAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(13, 15), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    (
        Name::new("hook"),
        Hook,
        Sprite::from_atlas_image(
            hook_assets.hook.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Transform::from(Transform::from_translation(
            love_to_bevy_coords(158.0, 30.0).extend(0.0),
        )),
        Anchor::TOP_CENTER,
    )
}
