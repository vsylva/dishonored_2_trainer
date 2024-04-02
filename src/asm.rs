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
