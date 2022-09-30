use std::path::Path;
use std::time::Duration;

use bevy::asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::sprite::Rect;
use bevy::utils::BoxedFuture;
use serde::Deserialize;

use crate::animation::{AnimationTag, Frame};

pub struct AtlasLoaderPlugin;

impl Plugin for AtlasLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<AnimationTag>()
            .init_asset_loader::<SpriteSheetLoader>();
    }
}

#[derive(Default)]
struct SpriteSheetLoader;

impl AssetLoader for SpriteSheetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let sheet_data: SheetData = serde_json::from_slice(bytes)?;

            let texture_path = format!("sheet_textures/{}", &sheet_data.meta.image);
            let texture_path = AssetPath::new_ref(Path::new(&texture_path), None);

            let image_handle: Handle<Image> = load_context.get_handle(texture_path.clone());

            let mut texture_atlas = TextureAtlas::new_empty(
                image_handle,
                Vec2::new(sheet_data.meta.size.w, sheet_data.meta.size.h),
            );

            let mut frames = Vec::new();
            for SheetFrame { frame, duration } in sheet_data.frames.iter() {
                let texture_index = texture_atlas.add_texture(Rect {
                    min: Vec2::new(frame.x, frame.y),
                    max: Vec2::new(frame.x + frame.w, frame.y + frame.h),
                });
                frames.push(Frame {
                    texture_index,
                    duration: Duration::from_millis(*duration as u64),
                });
            }

            let mut atlas_asset = LoadedAsset::new(texture_atlas);
            atlas_asset.add_dependency(texture_path);

            for tag in sheet_data.meta.frame_tags.into_iter() {
                let animation_tag = AnimationTag(
                    (tag.from..=tag.to)
                        .into_iter()
                        .map(|idx| frames[idx as usize].clone())
                        .collect(),
                );
                load_context.set_labeled_asset(&tag.name, LoadedAsset::new(animation_tag));
            }

            load_context.set_default_asset(atlas_asset);

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["sheet.json"]
    }
}

#[derive(Debug, Deserialize)]
pub struct SheetData {
    pub frames: Vec<SheetFrame>,
    pub meta: Meta,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    image: String,
    size: Size,
    frame_tags: Vec<MetaTag>,
}

#[derive(Debug, Deserialize)]
pub struct MetaTag {
    name: String,
    from: u32,
    to: u32,
}

#[derive(Debug, Deserialize)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Deserialize)]
pub struct SheetFrame {
    pub frame: BoundingBox,
    pub duration: i32,
}

#[derive(Debug, Deserialize)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
