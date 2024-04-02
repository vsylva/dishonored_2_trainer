use crate::hook::{BEND_TIME_HOOK, INSTANT_CHOKE_HOOK, NEVER_FALL_HOOK, UNLIMITED_MANA_HOOK};

pub static mut IS_SHOW_UI: bool = true;

pub unsafe fn window(ui: &hudhook::imgui::Ui) {
    if ui.checkbox("无限法力", UNLIMITED_MANA_HOOK.get_swtich_mut()) {
        UNLIMITED_MANA_HOOK.swtich()
    }

    if ui.checkbox("无限暂停时间", BEND_TIME_HOOK.get_swtich_mut()) {
        BEND_TIME_HOOK.swtich()
    }

    if ui.checkbox("立即击晕", INSTANT_CHOKE_HOOK.get_swtich_mut()) {
        INSTANT_CHOKE_HOOK.swtich()
    }

    if ui.checkbox("永不坠落", NEVER_FALL_HOOK.get_swtich_mut()) {
        NEVER_FALL_HOOK.swtich()
    }
}

pub struct RenderLoop;

impl hudhook::ImguiRenderLoop for RenderLoop {
    fn initialize<'a>(
        &'a mut self,
        _ctx: &mut hudhook::imgui::Context,
        _loader: hudhook::TextureLoader<'a>,
    ) {
        _ctx.set_ini_filename(None);

        set_font(_ctx, 20.0);
    }

    fn render(&mut self, ui: &mut hudhook::imgui::Ui) {
        unsafe {
            if is_key_down_once(0x2D) {
                IS_SHOW_UI = !IS_SHOW_UI;
            }

            if !IS_SHOW_UI {
                (*hudhook::imgui::sys::igGetIO()).MouseDrawCursor = false;
                return;
            }

            (*hudhook::imgui::sys::igGetIO()).MouseDrawCursor = true;

            ui.window(format!("耻辱2修改器\t[~]键打开/关闭菜单"))
                .title_bar(true)
                .size([500.0, 400.0], hudhook::imgui::Condition::FirstUseEver)
                .resizable(true)
                .collapsible(true)
                .movable(true)
                .build(|| window(ui));
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
