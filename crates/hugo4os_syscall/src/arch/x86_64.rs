#[macro_export] macro_rules! syscall {
    (
        $a1:ident
        $(, $a2:ident
            $(, $a3:ident
                $(, $a4:ident
                    $(, $a5:ident
                        $(, $a6:ident
                            $(, $a7:ident
                                $(, $a8:ident
                                    $(, $a9:ident
                                        $(, $a10:ident
                                            $(, $a11:ident
                                                $(, $a12:ident)?
                                            )?
                                        )?
                                    )?
                                )?
                            )?
                        )?
                    )?
                )?
            )?
        )?
    ) => {
        {
            let mut __target: u64 = $a1;
            let __target_ref = &mut __target as *mut u64;
            core::arch::asm!(
                "int 0x80",
                in("eax") __target_ref
                $(, in("rsi") $a2
                    $(, in("rdx") $a3
                        $(, in("rcx") $a4
                            $(, in("r8") $a5
                                $(, in("r9") $a6
                                    $(, in("r10") $a7
                                        $(, in("r11") $a8
                                            $(, in("r12") $a9
                                                $(, in("r13") $a10
                                                    $(, in("r14") $a11
                                                        $(, in("r15") $a12)?
                                                    )?
                                                )?
                                            )?
                                        )?
                                    )?
                                )?
                            )?
                        )?
                    )?
                )?
            );
            __target
        }
    };
}