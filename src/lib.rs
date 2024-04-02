pub mod asm;
pub mod hook;
pub mod ui;

use minhook_raw as minhook;

#[link(name = "user32")]
extern "system" {
    pub(crate) fn GetAsyncKeyState(vKey: i32) -> u16;
}

#[no_mangle]
unsafe extern "system" fn DllMain(
    h_module: isize,
    ul_reason_for_call: u32,
    _lp_reserved: *mut core::ffi::c_void,
) -> i32 {
    if ul_reason_for_call == 1 {
        std::thread::spawn(move || {
            let now = ::std::time::Instant::now();
            let dur = ::std::time::Duration::from_secs(1);

            loop {
                if now.elapsed().as_secs() < 20 {
                    if let Ok(mi) = vcheat::internal::get_mod_info("XAudio2_9.dll") {
                        if mi.handle != 0 {
                            break;
                        }
                    }
                } else {
                    vcheat::internal::free_dll_exit_thread(h_module, 0);
                }

                ::std::thread::sleep(dur);
            }

            let mi = vcheat::internal::get_mod_info("").unwrap();

            let mut mod_data = Vec::<u8>::new();
            mod_data.resize(mi.size as usize, 0);
            ::std::ptr::copy(mi.addr.cast(), mod_data.as_mut_ptr(), mi.size as usize);

            if let Err(_) = vcheat::pat_find("31 2E 37 37 2E 39 2E 30 00", &mod_data) {
                vcheat::internal::free_dll_exit_thread(h_module, 0);
            }

            if minhook::initialize().is_err() {
                vcheat::internal::free_dll_exit_thread(h_module, 0);
            }

            hook::create_hook(mi.addr, &mod_data);

            drop(mod_data);

            if let Err(_) = ::hudhook::Hudhook::builder()
                .with::<hudhook::hooks::dx11::ImguiDx11Hooks>(ui::RenderLoop)
                .with_hmodule(hudhook::windows::Win32::Foundation::HINSTANCE(h_module))
                .build()
                .apply()
            {
                ::hudhook::eject();
            }
        });
    } else if ul_reason_for_call == 0 {
    }

    1
}
