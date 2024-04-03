use crate::hal::generic_context::GenericContext;
use core::arch::asm;

/// General registers of riscv64.
#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct RegistersRV64 {
    pub zero: usize,
    pub ra: usize,
    pub sp: usize,
    pub gp: usize, // only valid for user traps
    pub tp: usize, // only valid for user traps
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0: usize,
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
}

/// Task Context of riscv64
#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct TaskContextRV64 {
    pub ra: usize,
    pub sp: usize,
    pub s0: usize,
    pub s1: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
}

impl GenericContext<TaskContextRV64> for TaskContextRV64 {
    fn goto_trap_return(kstack_ptr: usize) -> TaskContextRV64 {
        let mut cx = TaskContextRV64::zero_init();
        cx.ra = crate::hal::riscv::trap::trap_return as usize;
        cx.sp = kstack_ptr;
        cx
    }

    fn switch(
        current_task_cx_ptr: *mut TaskContextRV64,
        next_task_cx_ptr: *const TaskContextRV64,
    ) -> ! {
        unsafe {
            asm!(
                "
                #save current task cx
                sd ra, 0*8(a0)
                sd sp, 1*8(a0)
                sd s0, 2*8(a0)
                sd s1, 3*8(a0)
                sd s2, 4*8(a0)
                sd s3, 5*8(a0)
                sd s4, 6*8(a0)
                sd s5, 7*8(a0)
                sd s6, 8*8(a0)
                sd s7, 9*8(a0)
                sd s8, 10*8(a0)
                sd s9, 11*8(a0)
                sd s10, 12*8(a0)
                sd s11, 13*8(a0)
                
                #restore next task cx
                ld ra, 0*8(a1)
                ld sp, 1*8(a1)
                ld s0, 2*8(a1)
                ld s1, 3*8(a1)
                ld s2, 4*8(a1)
                ld s3, 5*8(a1)
                ld s4, 6*8(a1)
                ld s5, 7*8(a1)
                ld s6, 8*8(a1)
                ld s7, 9*8(a1)
                ld s8, 10*8(a1)
                ld s9, 11*8(a1)
                ld s10, 12*8(a1)
                ld s11, 13*8(a1)
                ret
                ",
                in("a0") current_task_cx_ptr,
                in("a1") next_task_cx_ptr,
            );
            core::hint::unreachable_unchecked()
        }
    }
}

/// Saved registers when a trap (interrupt or exception) occurs.
#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct TrapFrame {
    /// All general registers.
    pub regs: RegistersRV64,
    /// Supervisor Exception Program Counter.
    pub sepc: usize,
    /// Supervisor Status Register.
    pub sstatus: usize,
}
