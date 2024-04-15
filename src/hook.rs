use crate::asm;

pub(crate) static mut HOOK_INSTANT_CHOKE: ByteHook = ByteHook::new();
pub(crate) static mut HOOK_NEVER_FALL: ByteHook = ByteHook::new();
pub(crate) static mut HOOK_BLINK_NO_HIT_STUN: ByteHook = ByteHook::new();

pub(crate) static mut HOOK_BLINK_DISTANCE: AsmHook = AsmHook::new();
pub(crate) static mut BLINK_NO_ANIMATION: AsmHook = AsmHook::new();
pub(crate) static mut HOOK_BLINK_NO_CD: AsmHook = AsmHook::new();
pub(crate) static mut HOOK_UNLIMITED_MANA: AsmHook = AsmHook::new();
pub(crate) static mut HOOK_BEND_TIME: AsmHook = AsmHook::new();

pub(crate) unsafe fn create_hook(mod_addr: *mut ::core::ffi::c_void, mod_data: &[u8]) {
    HOOK_INSTANT_CHOKE.create(mod_addr, mod_data, "8B 53 24 85 D2 74 18", "77", -2);

    HOOK_BLINK_NO_HIT_STUN.create(
        mod_addr,
        mod_data,
        "48 8B 41 10 48 8B 48 28 48 8B 81 90 00 00 00 48 85 C0 74 0E 48 8B 40 70 48 85 C0",
        "30 C0 C3 90",
        0,
    );

    HOOK_NEVER_FALL.create(mod_addr, mod_data, "89 46 24 F3 0F 10 45 80", "90 90 90", 0);

    HOOK_UNLIMITED_MANA.create(
        mod_addr,
        mod_data,
        "0F 2F D1 F3 0F 10 7B",
        8,
        asm::unlimited_mana as *mut ::core::ffi::c_void,
    );

    HOOK_BEND_TIME.create(
        mod_addr,
        mod_data,
        "F3 0F 11 7D 67 C7",
        5,
        asm::bend_time as *mut ::core::ffi::c_void,
    );

    HOOK_BLINK_DISTANCE.create(
        mod_addr,
        mod_data,
        "83 F8 FF 7E 13 48 8D 14 80 48 8B 43 40 48 8B 48 48",
        5,
        asm::blink_distance as *mut ::core::ffi::c_void,
    );

    BLINK_NO_ANIMATION.create(
        mod_addr,
        mod_data,
        "F3 44 0F 10 5A 70 4C 8D 4D DB 48 8D 55 67",
        6,
        asm::blink_instant as *mut ::core::ffi::c_void,
    );

    HOOK_BLINK_NO_CD.create(
        mod_addr,
        mod_data,
        "F3 0F 10 80 50 01 00 00 F3 0F 11 02 74 0C F3 0F 58 80 04 02 00 00",
        8,
        asm::blink_no_cd as *mut ::core::ffi::c_void,
    );
}

pub struct AsmHook {
    pub target_addr: *mut ::core::ffi::c_void,
    pub is_enabled: bool,
}

impl AsmHook {
    pub const fn new() -> Self {
        Self {
            target_addr: ::core::ptr::null_mut(),
            is_enabled: false,
        }
    }
    pub unsafe fn create(
        &mut self,
        mod_addr: *mut ::core::ffi::c_void,
        mod_data: &[u8],
        pat: &str,
        occupied: usize,
        detour_addr: *mut ::core::ffi::c_void,
    ) {
        let pat_offset = vcheat::pat_find(pat, mod_data).unwrap();

        self.target_addr = mod_addr.byte_add(pat_offset);

        let back_addr = self.target_addr.byte_add(occupied);

        let mut end_offset = 0;

        for i in 0..0xFF {
            let ptr = detour_addr.cast::<u8>().byte_add(i);

            if ptr.read() == 0x90 {
                let parts = std::slice::from_raw_parts(ptr, 4);

                if parts.iter().all(|nop| *nop == 0x90) {
                    end_offset = i;
                    break;
                }
            }
        }

        let mut back_shell_code = Vec::new();

        back_shell_code.push(0xFF);
        back_shell_code.push(0x25);
        back_shell_code.push(0x0);
        back_shell_code.push(0x0);
        back_shell_code.push(0x0);
        back_shell_code.push(0x0);

        back_shell_code.extend_from_slice((back_addr as isize).to_le_bytes().as_ref());

        vcheat::write_mem(
            vcheat::internal::get_proc_handle(),
            detour_addr.byte_add(end_offset),
            &back_shell_code,
        )
        .unwrap();

        minhook_raw::create_hook(self.target_addr, detour_addr, ::core::ptr::null_mut());
    }

    pub fn switch(&mut self) {
        if self.is_enabled {
            minhook_raw::enable_hook(self.target_addr);
        } else {
            minhook_raw::disable_hook(self.target_addr);
        }
    }
}

pub struct ByteHook {
    pub target_addr: *mut ::core::ffi::c_void,
    pub is_enabled: bool,
    pub source: Vec<u8>,
    pub patch: String,
}

impl ByteHook {
    pub const fn new() -> Self {
        Self {
            target_addr: ::core::ptr::null_mut(),
            is_enabled: false,
            source: Vec::new(),
            patch: String::new(),
        }
    }

    pub unsafe fn create<S: AsRef<str>>(
        &mut self,
        mod_addr: *mut ::core::ffi::c_void,
        mod_data: &[u8],
        pat: &str,
        patch: S,
        offset_count: isize,
    ) {
        let pat_offset = vcheat::pat_find(pat, mod_data).unwrap();

        self.target_addr = mod_addr.byte_add(pat_offset);

        self.target_addr = self.target_addr.offset(offset_count);

        let size = patch.as_ref().split_whitespace().count();

        self.source.resize(size, 0);

        ::std::ptr::copy(self.target_addr.cast(), self.source.as_mut_ptr(), size);

        self.patch = patch.as_ref().to_string();
    }

    pub unsafe fn switch(&mut self) {
        if self.is_enabled {
            vcheat::write_mem_hex_str(
                vcheat::internal::get_proc_handle(),
                self.target_addr,
                &self.patch,
            )
            .unwrap();
        } else {
            vcheat::write_mem(
                vcheat::internal::get_proc_handle(),
                self.target_addr,
                &self.source,
            )
            .unwrap();
        }
    }
}
