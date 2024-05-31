use crate::hook::{
    BLINK_NO_ANIMATION, HOOK_BEND_TIME, HOOK_BLINK_DISTANCE, HOOK_BLINK_NO_CD,
    HOOK_BLINK_NO_HIT_STUN, HOOK_INSTANT_CHOKE, HOOK_NEVER_FALL, HOOK_UNLIMITED_MANA,
};

pub static mut IS_SHOW_UI: bool = true;

pub unsafe fn on_frame(ui: &hudhook::imgui::Ui) {
    if ui.checkbox("无限法力", &mut HOOK_UNLIMITED_MANA.is_enabled) {
        HOOK_UNLIMITED_MANA.switch()
    }

    if ui.checkbox("无限暂停时间", &mut HOOK_BEND_TIME.is_enabled) {
        HOOK_BEND_TIME.switch()
    }

    if ui.checkbox("立即击晕", &mut HOOK_INSTANT_CHOKE.is_enabled) {
        HOOK_INSTANT_CHOKE.switch()
    }

    if ui.checkbox("永不坠落", &mut HOOK_NEVER_FALL.is_enabled) {
        HOOK_NEVER_FALL.switch()
    }

    if ui.checkbox("闪现距离", &mut HOOK_BLINK_DISTANCE.is_enabled) {
        HOOK_BLINK_DISTANCE.switch()
    }

    if ui.checkbox("闪现无CD", &mut HOOK_BLINK_NO_CD.is_enabled) {
        HOOK_BLINK_NO_CD.switch()
    }

    if ui.checkbox("闪现无硬直", &mut HOOK_BLINK_NO_HIT_STUN.is_enabled) {
        HOOK_BLINK_NO_HIT_STUN.switch()
    }

    if ui.checkbox("闪现无动画", &mut BLINK_NO_ANIMATION.is_enabled) {
        BLINK_NO_ANIMATION.switch()
    }
}

pub struct RenderLoop;

impl hudhook::ImguiRenderLoop for RenderLoop {
    fn initialize<'a>(
        &'a mut self,
        _ctx: &mut hudhook::imgui::Context,
        _render_context: &'a mut dyn hudhook::RenderContext,
    ) {
        _ctx.set_ini_filename(None);

        set_font(_ctx, 25.0);

        _ctx.style_mut().use_light_colors();
    }

    fn before_render<'a>(
        &'a mut self,
        _ctx: &mut hudhook::imgui::Context,
        _render_context: &'a mut dyn hudhook::RenderContext,
    ) {
        unsafe {
            if is_key_down_once(0x2D) {
                IS_SHOW_UI = !IS_SHOW_UI;
            }

            if !IS_SHOW_UI {
                _ctx.io_mut().mouse_draw_cursor = false;
                return;
            }

            _ctx.io_mut().mouse_draw_cursor = true;
        }
    }

    fn render(&mut self, ui: &mut hudhook::imgui::Ui) {
        unsafe {
            if !IS_SHOW_UI {
                return;
            }

            ui.window(format!("[Insert]键"))
                .title_bar(true)
                .size([600.0, 450.0], hudhook::imgui::Condition::FirstUseEver)
                .build(|| on_frame(ui));
        }
    }
}

pub(crate) fn set_font(ctx: &mut hudhook::imgui::Context, font_size: f32) {
    let tf_data = hudhook::imgui::FontSource::TtfData {
        data: include_bytes!("../res/FZHTJW.TTF"),
        size_pixels: font_size,
        config: Some(hudhook::imgui::FontConfig {
            size_pixels: font_size,
            pixel_snap_h: true,
            glyph_ranges: hudhook::imgui::FontGlyphRanges::from_slice(&[
                0x0020, 0x00FF, 0x2000, 0x206F, 0x3000, 0x30FF, 0x31F0, 0x31FF, 0xFF00, 0xFFEF,
                0xFFFD, 0xFFFD, 0x4E00, 0x9FAF, 0,
            ]),
            ..hudhook::imgui::FontConfig::default()
        }),
    };

    ctx.fonts().add_font(&[tf_data]);
}

pub(crate) unsafe fn is_key_down_once(virtual_key_code: i32) -> bool {
    static mut WAS_KEY_DOWN: bool = false;

    if (crate::GetAsyncKeyState(virtual_key_code) & 0x8000) != 0 {
        if !WAS_KEY_DOWN {
            WAS_KEY_DOWN = true;
            return true;
        }
    } else {
        WAS_KEY_DOWN = false;
    }

    false
}
