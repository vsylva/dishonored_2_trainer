use std::arch::asm;

pub unsafe extern "system" fn unlimited_mana() {
    asm!(
        "
        movss xmm1, xmm3
        comiss xmm2, xmm1
        movss xmm7, [rbx + 0x20]
        ",
        options(nomem, nostack)
    );

    asm!(
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        options(nomem, nostack, noreturn)
    );
}

pub(crate) static mut TIME: f32 = f32::MAX;

pub unsafe extern "system" fn bend_time() {
    asm!("push rax", options(nomem, nostack));

    asm!(
        "
        movss xmm7, [rax]
        ",

        in("rax") std::ptr::addr_of_mut!(TIME),
        options(nomem,nostack)
    );

    asm!("pop rax", options(nomem, nostack));

    asm!("movss [rbp + 0x67], xmm7", options(nomem, nostack));

    asm!(
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        options(nomem, nostack, noreturn)
    );
}

pub(crate) static mut BLINK_DISTANCE_FAR: f32 = 60.0;
pub(crate) static mut BLINK_DISTANCE_NEAR: f32 = 20.0;

pub unsafe extern "system" fn blink_distance() {
    asm!("push rax", options(nomem, nostack));
    asm!("push rdx", options(nomem, nostack));

    asm!("cmp eax, -1", options(nomem, nostack));

    asm!(
        "
        jle short 0f
        movss xmm8, [rax]
        0:
        movss xmm4, [r15]
        ",

        in("rax") std::ptr::addr_of_mut!(BLINK_DISTANCE_FAR),
        in("rdx") std::ptr::addr_of_mut!(BLINK_DISTANCE_NEAR),
        options(nomem,nostack)
    );

    asm!("pop rdx", options(nomem, nostack));
    asm!("pop rax", options(nomem, nostack));

    asm!(
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        options(nomem, nostack, noreturn)
    );
}

pub(crate) static mut BLINK_INSTANT: f32 = 0.0;
pub unsafe extern "system" fn blink_instant() {
    asm!("push rax", options(nomem, nostack));

    asm!(
        "
        movss xmm11, [rax]
        ",

        in("rax") std::ptr::addr_of_mut!(BLINK_INSTANT),

        options(nomem,nostack)
    );

    asm!("pop rax", options(nomem, nostack));

    asm!(
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        options(nomem, nostack, noreturn)
    );
}

pub unsafe extern "system" fn blink_no_cd() {
    asm!(
        "
        mov dword ptr [rdx], 0
        mov rax, rdx
        ret
        ",
        options(nomem, nostack)
    );

    asm!(
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        options(nomem, nostack, noreturn)
    );
}
