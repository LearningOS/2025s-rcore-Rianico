## lab1

在 `process.rs` 中将 0, 1, 2 定义为有具体含义的变量名，在 `sys_trace` 中通过 `match` 分情况进行处理。

在 `TaskManagerInner` 中定义一个类型为 `HashMap<app_id, HashMap<syscall_id, call_cnt>>` 的类型，在 `TaskManager` 中实现 `record_sys_call_cnt` 以及 `get_sys_call_cnt` 两个函数，分别对应保存 syscal 次数以及查询 syscall 次数的功能。

最后，在 `trap_handler` 里的 `Trap::Exception` 埋点，跟踪保存 `syscall` 的状态。

## 简答作业

RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0

1. 
- `ch2b_bad_address` 报 `[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.` ，其在非法的内存地址写入了值。
- `ch2b_bad_register` 报 `[kernel] IllegalInstruction in application, kernel killed it.`，用户程序在 User mode 下访问了 S 态的 `sstatus` 寄存器。
- `ch2b_bad_instructions` 报 `[kernel] IllegalInstruction in application, kernel killed it.`，这是因为其在 U 态下调用了只有 S 态才能调用的 `sret` 指令。

2.1 刚进入 `__restore` 时，`sp` 指向了 `kernel_stack`，存储了需要恢复的 task context，即各种 register 的状态值。`__restore` 可以用于 U mode 发起 `syscall` 或者发生其他 Exception（如调用了 illegal instruction），这两中场景都会触发 trap，在处理完 trap 后需要恢复原来的 task context。

2.2 特殊处理了 `sstatus`，`sepc` 以及 `sscratch`：
- `sstatus` 存储了发生 trap 之前 CPU 所处的特权级以及一些其他信息，例如 `sie` 关于 timer 的状态。U mode 触发的 trap 在处理完后需要将 CPU 重新置为 U mode 以确保安全。
- `sepc` 记录了触发 trap 之前正在执行的指令地址，返回 U mode 时需要知道接下来执行哪一条执行。根据 trap 触发的原因，`sepc` 所存储的指令地址也有所区别；若是发起 `syscall` 触发的 trap，则 `sepc`  存储的会是下一条 U mode 的指令；若是由 Exception 触发的 `trap`，则往往需要在返回 U mode 后重新执行一遍发生 trap 之前的指令。
- `sscratch` 存储了用户/内核栈的地址，会随着 U/S 的切换而切换。

2.3 x2 对应 `sp`，也就是当前的栈帧，`csrrw sp, sscratch, sp` 会将 `sp` 重新置为 trap 发生前的 `sp`。 `x4` 的话对应 thread pointer，当前 rCore 是个单线程实现，不存在线程的切换。

2.4 L60 指令之后，`sp` 被置为 user stack, `sscratch` 被置为 kernel stack。

2.5 发生状态切换在 `csrw sstaus, t0`，从 `__alltraps` 中的 `csrr t0, sstatus` 以及 `sd t0, 32*8(sp)` 可以得知，在发生 trap 的时候，U mode 相关的权限状态被存在了 `32*8(sp)` 的位置，`__restore` 中的 `ld t0, 32*8(sp)` 则是重新将 U mode 相关的 `sstatus` 状态写入了 `t0`。

2.6 执行该行指令之前 `sscratch` 指向 kernel stack，`sp` 指向 user stack，执行完后两者的值互换。

2.7 用户程序调用 `ecall` 时便已经触发 trap 并从 U mode 进入了 S mode。



1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

暂无。

此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

- riscv-asm.pdf

2. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

3. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
