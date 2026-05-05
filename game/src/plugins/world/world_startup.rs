use bevy::image::ImageArrayLayout;
use bevy::image::ImageLoaderSettings;
use bevy::prelude::*;
use bevy::sprite_render::*;

use crate::plugins::MAIN_TILES_TILESET_ASSET;

pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tileset_handle = asset_server
        .load_builder()
        .with_settings::<ImageLoaderSettings>(|s| {
            s.array_layout = Some(ImageArrayLayout::GridCount {
                columns: 2,
                rows: 2,
            })
        })
        .load(MAIN_TILES_TILESET_ASSET);

    const MAP_SIZE: u32 = 64;

    let tile_date = TileData {
        tileset_index: 3,
        color: Color::WHITE,
        visible: true,
        orientation: TileOrientation::Default,
    };

    let tilemap_chunk_date =
        TilemapChunkTileData(vec![Some(tile_date); (MAP_SIZE * MAP_SIZE) as usize]);

    commands.spawn((
        Transform::default(),
        TilemapChunk {
            chunk_size: UVec2::new(MAP_SIZE, MAP_SIZE),
            tileset: tileset_handle,
            alpha_mode: AlphaMode2d::Opaque,
            tile_display_size: UVec2::new(64, 64),
        },
        tilemap_chunk_date,
    ));
}
