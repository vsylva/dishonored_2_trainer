use std::ptr::null_mut;

use crate::asm;

pub(crate) static mut INSTANT_CHOKE_HOOK: ByteHook = ByteHook::new();
pub(crate) static mut NEVER_FALL_HOOK: ByteHook = ByteHook::new();

pub(crate) static mut UNLIMITED_MANA_HOOK: AsmHook = unsafe { ::core::mem::zeroed() };
pub(crate) static mut BEND_TIME_HOOK: AsmHook = unsafe { ::core::mem::zeroed() };

pub(crate) unsafe fn create_hook(mod_addr: *mut ::core::ffi::c_void, mod_data: &[u8]) {
    INSTANT_CHOKE_HOOK
        .set_target_addr(
            mod_addr
                .byte_add(vcheat::pat_find("8B 53 24 85 D2 74 18", mod_data).unwrap())
                .byte_sub(2),
        )
        .get_source(1)
        .set_patch(&[0x77]);

    NEVER_FALL_HOOK
        .set_target_addr(
            mod_addr.byte_add(vcheat::pat_find("89 46 24 F3 0F 10 45 80", mod_data).unwrap()),
        )
        .get_source(3)
        .set_patch(&[0x90, 0x90, 0x90]);

    UNLIMITED_MANA_HOOK = AsmHook::new()
        .get_data(mod_addr, mod_data, "0F 2F D1 F3 0F 10 7B", 8)
        .gen_detour(asm::unlimited_mana as *mut ::core::ffi::c_void)
        .create_hook()
        .to_owned();

    BEND_TIME_HOOK = AsmHook::new()
        .get_data(mod_addr, mod_data, "F3 0F 11 7D 67 C7", 5)
        .gen_detour(asm::bend_time as *mut ::core::ffi::c_void)
        .create_hook()
        .to_owned();
}

#[derive(Clone, Copy)]
pub(crate) struct AsmHook {
    pub(crate) target_addr: *mut std::ffi::c_void,
    target_back_addr: *mut std::ffi::c_void,
    detour_fn_addr: *mut std::ffi::c_void,
    is_enable: bool,
}

impl AsmHook {
    pub(crate) const fn new() -> Self {
        Self {
            target_addr: ::core::ptr::null_mut(),
            target_back_addr: ::core::ptr::null_mut(),
            detour_fn_addr: ::core::ptr::null_mut(),
            is_enable: false,
        }
    }

    pub(crate) unsafe fn get_data(
        &mut self,
        mod_addr: *mut ::core::ffi::c_void,
        mod_data: &[u8],
        pat: &str,
        occupied: usize,
    ) -> &mut Self {
        let pat_offset = vcheat::pat_find(pat, mod_data).unwrap();

        self.target_addr = mod_addr.byte_add(pat_offset);
        self.target_back_addr = self.target_addr.byte_add(occupied);

        self
    }

    pub(crate) unsafe fn gen_detour(
        &mut self,
        detour_fn_addr: *mut ::core::ffi::c_void,
    ) -> &mut Self {
        self.detour_fn_addr = detour_fn_addr;

        let mut detour_fn_end_offset = 0;

        for i in 0..0xFFFF {
            let ptr = detour_fn_addr.cast::<u8>().byte_add(i);

            if ptr.read() == 0x90 {
                let parts = std::slice::from_raw_parts(ptr, 4);

                if parts.iter().all(|nop| *nop == 0x90) {
                    detour_fn_end_offset = i;
                    break;
                }
            }
        }

        let mut jmp_target_addr_shell_code = Vec::new();

        jmp_target_addr_shell_code.push(0xFF);
        jmp_target_addr_shell_code.push(0x25);
        jmp_target_addr_shell_code.push(0x0);
        jmp_target_addr_shell_code.push(0x0);
        jmp_target_addr_shell_code.push(0x0);
        jmp_target_addr_shell_code.push(0x0);

        jmp_target_addr_shell_code
            .extend_from_slice((self.target_back_addr as isize).to_le_bytes().as_ref());

        vcheat::write_mem(
            vcheat::internal::get_proc_handle(),
            detour_fn_addr.byte_add(detour_fn_end_offset),
            &jmp_target_addr_shell_code,
        )
        .unwrap();

        self
    }

    pub(crate) unsafe fn create_hook(&self) -> &Self {
        minhook_raw::create_hook(
            self.target_addr,
            self.detour_fn_addr,
            ::core::ptr::null_mut(),
        );

        self
    }

    pub(crate) fn get_swtich_mut(&mut self) -> &mut bool {
        &mut self.is_enable
    }

    pub(crate) fn swtich(&mut self) {
        if self.is_enable {
            minhook_raw::enable_hook(self.target_addr);
        } else {
            minhook_raw::disable_hook(self.target_addr);
        }
    }
}

#[derive(Clone)]
pub(crate) struct ByteHook {
    target_addr: *mut std::ffi::c_void,
    source: Vec<u8>,
    patch: Vec<u8>,
    is_enable: bool,
}

impl ByteHook {
    pub(crate) const fn new() -> Self {
        Self {
            target_addr: null_mut(),
            source: Vec::new(),
            patch: Vec::new(),
            is_enable: false,
        }
    }

    pub(crate) unsafe fn set_target_addr(
        &mut self,
        target_addr: *mut std::ffi::c_void,
    ) -> &mut Self {
        self.target_addr = target_addr;
        self
    }

    pub(crate) unsafe fn get_source(&mut self, size: usize) -> &mut Self {
        self.source =
            vcheat::read_mem(vcheat::internal::get_proc_handle(), self.target_addr, size).unwrap();
        self
    }

    pub(crate) unsafe fn set_patch(&mut self, patch: &[u8]) -> &mut Self {
        self.patch = patch.to_vec();
        self
    }

    pub(crate) fn get_swtich_mut(&mut self) -> &mut bool {
        &mut self.is_enable
    }

    pub(crate) unsafe fn swtich(&mut self) {
        if self.is_enable {
            vcheat::write_mem(
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
