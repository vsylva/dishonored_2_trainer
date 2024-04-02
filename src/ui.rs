use crate::hook::{
    BLINK_NO_ANIMATION, HOOK_BEND_TIME, HOOK_BLINK_DISTANCE, HOOK_BLINK_NO_CD,
    HOOK_BLINK_NO_HIT_STUN, HOOK_INSTANT_CHOKE, HOOK_NEVER_FALL, HOOK_UNLIMITED_MANA,
};

pub static mut IS_SHOW_UI: bool = true;

pub unsafe fn window(ui: &hudhook::imgui::Ui) {
    if ui.checkbox("无限法力", HOOK_UNLIMITED_MANA.get_swtich_mut()) {
        HOOK_UNLIMITED_MANA.swtich()
    }

    if ui.checkbox("无限暂停时间", HOOK_BEND_TIME.get_swtich_mut()) {
        HOOK_BEND_TIME.swtich()
    }

    if ui.checkbox("立即击晕", HOOK_INSTANT_CHOKE.get_swtich_mut()) {
        HOOK_INSTANT_CHOKE.swtich()
    }

    if ui.checkbox("永不坠落", HOOK_NEVER_FALL.get_swtich_mut()) {
        HOOK_NEVER_FALL.swtich()
    }

    if ui.checkbox("闪现距离", HOOK_BLINK_DISTANCE.get_swtich_mut()) {
        HOOK_BLINK_DISTANCE.swtich()
    }

    if ui.checkbox("闪现无CD", HOOK_BLINK_NO_CD.get_swtich_mut()) {
        HOOK_BLINK_NO_CD.swtich()
    }

    if ui.checkbox("闪现无硬直", HOOK_BLINK_NO_HIT_STUN.get_swtich_mut()) {
        HOOK_BLINK_NO_HIT_STUN.swtich()
    }

    if ui.checkbox("闪现无动画", BLINK_NO_ANIMATION.get_swtich_mut()) {
        BLINK_NO_ANIMATION.swtich()
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

            ui.window(format!("耻辱2修改器\t[Insert]键打开/关闭菜单"))
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
