use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, ScreenshotCaptured},
};
use wasm_bindgen::JsCast;

#[derive(Component)]
pub struct RenderToCanvas {
    pub canvas_id: String,
}

#[derive(Component)]
pub struct Canvas;

pub struct MultiCanvasPlugin;

impl Plugin for MultiCanvasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (configure_target_canvas, screenshot_observer));
    }
}

pub fn configure_target_canvas(
    mut commands: Commands,
    window: Single<&Window>,
    cameras_to_render: Query<(Entity, &RenderToCanvas, &Camera), Without<Canvas>>,
) {
    let scale_factor = window.resolution.scale_factor();

    for (entity, canvas, camera) in cameras_to_render {
        if let Some(viewport) = &camera.viewport {
            let physical_size = viewport.physical_size;

            let document = web_sys::window().unwrap().document().unwrap();

            let Some(html_canvas_element) = document
                .get_element_by_id(&canvas.canvas_id)
                .and_then(|el| el.dyn_into::<web_sys::HtmlCanvasElement>().ok())
            else {
                continue;
            };

            html_canvas_element.set_width(physical_size.x);
            html_canvas_element.set_height(physical_size.y);

            let logical_width = physical_size.x as f32 / scale_factor;
            let logical_height = physical_size.y as f32 / scale_factor;

            let style = html_canvas_element.style();
            style
                .set_property("width", &format!("{}px", logical_width))
                .unwrap();
            style
                .set_property("height", &format!("{}px", logical_height))
                .unwrap();

            commands.entity(entity).insert(Canvas {});
        }
    }
}

fn screenshot_observer(mut commands: Commands, canvases: Query<(), With<Canvas>>) {
    if !canvases.is_empty() {
        commands
            .spawn(Screenshot::primary_window())
            .observe(draw_to_canvas);
    }
}

fn draw_to_canvas(trigger: On<ScreenshotCaptured>, canvases: Query<(&RenderToCanvas, &Camera)>) {
    let screenshot = trigger.event();
    let image = &screenshot.image;
    let src_width = image.width() as usize;
    let src_height = image.height() as usize;
    let rgba_data = image.data.clone().unwrap();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    for (render_canvas, camera) in canvases {
        let target_size = match camera.physical_viewport_size() {
            Some(tuple) => tuple,
            None => continue,
        };

        let physical_position = match &camera.viewport {
            Some(viewport) => viewport.physical_position,
            None => continue,
        };

        let crop_width = target_size.x as usize;
        let crop_height = target_size.y as usize;

        let start_x = physical_position.x as usize;
        let start_y = physical_position.y as usize;

        let Some(destination_canva) = document
            .get_element_by_id(&render_canvas.canvas_id)
            .and_then(|el| el.dyn_into::<web_sys::HtmlCanvasElement>().ok())
        else {
            continue;
        };

        let context = destination_canva
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let actual_crop_width = crop_width.min(src_width - start_x);
        let actual_crop_height = crop_height.min(src_height - start_y);

        let mut cropped_data = Vec::with_capacity(actual_crop_width * actual_crop_height * 4);

        for y in 0..actual_crop_height {
            let src_y = start_y + y;
            let row_start = (src_y * src_width + start_x) * 4;
            let row_end = row_start + (actual_crop_width * 4);
            cropped_data.extend_from_slice(&rgba_data[row_start..row_end]);
        }

        destination_canva.set_width(actual_crop_width as u32);
        destination_canva.set_height(actual_crop_height as u32);

        let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&cropped_data),
            actual_crop_width as u32,
            actual_crop_height as u32,
        )
        .unwrap();

        context.put_image_data(&image_data, 0.0, 0.0).unwrap();
    }
}
