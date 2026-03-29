# Registers

For managing states, doing math, or general logic, we mostly use variables of primitive sizes when in high level programming languages. Since
software routines are mostly if not always dependent on these data, they must always be fast. A CPU register is very akin to this, it is the
fastest storage you can directly access on a computer.

Registers, in x86-64 are 64-bit long, and 32-bit long in x86-32. Focusing in x86-64, and neglecting all x86-32, registers can be categorized in
[imm](</arch/imm>), [xmm](</arch/xmm>), [ymm](</arch/ymm>), and [zmm](</arch/zmm>) if you're being size-wise. Also GPR, or SPR if you're being symbolic-wise.

Starting by the symbolic point of view, GPR (General Purpose Registers) are roughly all registers you can freely store data in, regardless of what
it is, in case you're ignoring ABIs. If you don't, you're usually limited by conventions. These registers are all [imm](</arch/imm>) (covered later), and you can
manage their state by using the [stack](</arch/stack>).

In regards to SPR (Specific Purpose Registers), they can also be a GPR. What classify one as SPR is whether you can or cannot freely read and write
it. As priorly mentioned, you can be limited by conventions, [SysV] for instance, and such may rule a behavior. One good example is rsp, which is the
stack pointer. It IS general purpose, you can theoretically do everything you want with it, but you should not, because it is specifically tied to the
function of managing the stack. And this "tie", even if not hard-wired into the silicon, is what define a register as SPR.

Following to the size point of view, GPR are all [imm](</arch/imm>), which ranges in size from 16-bits to 64-bits. There is also the [SIMD](</arch/simd>) registers, which are
[xmm](</arch/xmm>), [ymm](</arch/ymm>), and [zmm](</arch/ymm>). In the same sequence, each having 128-bits, 256-bits, and 512-bits, they being from the features [AVX](</arch/avx>), [AVX2](</arch/avx2>), and [AVX-512](</arch/avx512>).

The SPR, and GPR, have more depth. There are named registers for instance, which specify the SPR, but I will not cover it in this article, as its purpose
is to describe registers per se. If you want to read more about these, check: [Named Registers](</arch/named-registers>). If you want to access all the registers, down here you can see a table covering all of the x86-64 registers.


# x86-64 Registers
| Register    | Bit-length | Category | Feature Specific | Description                                                        |
|-------------|------------|----------|------------------|--------------------------------------------------------------------|
| rax         | 64         | GPR      | —                | Accumulator; holds return value in SysV ABI                        |
| eax         | 32         | GPR      | —                | Lower 32 bits of rax                                               |
| ax          | 16         | GPR      | —                | Lower 16 bits of rax                                               |
| al          | 8          | GPR      | —                | Low byte of ax                                                     |
| ah          | 8          | GPR      | —                | High byte of ax                                                    |
| rbx         | 64         | GPR      | —                | Callee-saved in SysV ABI                                           |
| ebx         | 32         | GPR      | —                | Lower 32 bits of rbx                                               |
| bx          | 16         | GPR      | —                | Lower 16 bits of rbx                                               |
| bl          | 8          | GPR      | —                | Low byte of bx                                                     |
| bh          | 8          | GPR      | —                | High byte of bx                                                    |
| rcx         | 64         | GPR      | —                | 4th argument in SysV ABI; counter in loop and shift instructions   |
| ecx         | 32         | GPR      | —                | Lower 32 bits of rcx                                               |
| cx          | 16         | GPR      | —                | Lower 16 bits of rcx                                               |
| cl          | 8          | GPR      | —                | Low byte of cx; used as shift count operand                        |
| ch          | 8          | GPR      | —                | High byte of cx                                                    |
| rdx         | 64         | GPR      | —                | 3rd argument in SysV ABI; high half of mul/div result with rax     |
| edx         | 32         | GPR      | —                | Lower 32 bits of rdx                                               |
| dx          | 16         | GPR      | —                | Lower 16 bits of rdx                                               |
| dl          | 8          | GPR      | —                | Low byte of dx                                                     |
| dh          | 8          | GPR      | —                | High byte of dx                                                    |
| rsi         | 64         | GPR      | —                | Source index; 2nd argument in SysV ABI                             |
| esi         | 32         | GPR      | —                | Lower 32 bits of rsi                                               |
| si          | 16         | GPR      | —                | Lower 16 bits of rsi                                               |
| sil         | 8          | GPR      | —                | Low byte of si                                                     |
| rdi         | 64         | GPR      | —                | Destination index; 1st argument in SysV ABI                        |
| edi         | 32         | GPR      | —                | Lower 32 bits of rdi                                               |
| di          | 16         | GPR      | —                | Lower 16 bits of rdi                                               |
| dil         | 8          | GPR      | —                | Low byte of di                                                     |
| rbp         | 64         | SPR      | —                | Base (frame) pointer; marks the base of the current stack frame    |
| ebp         | 32         | SPR      | —                | Lower 32 bits of rbp                                               |
| bp          | 16         | SPR      | —                | Lower 16 bits of rbp                                               |
| bpl         | 8          | SPR      | —                | Low byte of bp                                                     |
| rsp         | 64         | SPR      | —                | Stack pointer; always points to the top of the stack               |
| esp         | 32         | SPR      | —                | Lower 32 bits of rsp                                               |
| sp          | 16         | SPR      | —                | Lower 16 bits of rsp                                               |
| spl         | 8          | SPR      | —                | Low byte of sp                                                     |
| r8          | 64         | GPR      | —                | 5th argument in SysV ABI                                           |
| r8d         | 32         | GPR      | —                | Lower 32 bits of r8                                                |
| r8w         | 16         | GPR      | —                | Lower 16 bits of r8                                                |
| r8b         | 8          | GPR      | —                | Low byte of r8                                                     |
| r9          | 64         | GPR      | —                | 6th argument in SysV ABI                                           |
| r9d         | 32         | GPR      | —                | Lower 32 bits of r9                                                |
| r9w         | 16         | GPR      | —                | Lower 16 bits of r9                                                |
| r9b         | 8          | GPR      | —                | Low byte of r9                                                     |
| r10         | 64         | GPR      | —                | Caller-saved scratch register                                      |
| r10d        | 32         | GPR      | —                | Lower 32 bits of r10                                               |
| r10w        | 16         | GPR      | —                | Lower 16 bits of r10                                               |
| r10b        | 8          | GPR      | —                | Low byte of r10                                                    |
| r11         | 64         | GPR      | —                | Clobbered by the syscall instruction                               |
| r11d        | 32         | GPR      | —                | Lower 32 bits of r11                                               |
| r11w        | 16         | GPR      | —                | Lower 16 bits of r11                                               |
| r11b        | 8          | GPR      | —                | Low byte of r11                                                    |
| r12         | 64         | GPR      | —                | Callee-saved in SysV ABI                                           |
| r12d        | 32         | GPR      | —                | Lower 32 bits of r12                                               |
| r12w        | 16         | GPR      | —                | Lower 16 bits of r12                                               |
| r12b        | 8          | GPR      | —                | Low byte of r12                                                    |
| r13         | 64         | GPR      | —                | Callee-saved in SysV ABI                                           |
| r13d        | 32         | GPR      | —                | Lower 32 bits of r13                                               |
| r13w        | 16         | GPR      | —                | Lower 16 bits of r13                                               |
| r13b        | 8          | GPR      | —                | Low byte of r13                                                    |
| r14         | 64         | GPR      | —                | Callee-saved in SysV ABI                                           |
| r14d        | 32         | GPR      | —                | Lower 32 bits of r14                                               |
| r14w        | 16         | GPR      | —                | Lower 16 bits of r14                                               |
| r14b        | 8          | GPR      | —                | Low byte of r14                                                    |
| r15         | 64         | GPR      | —                | Callee-saved in SysV ABI                                           |
| r15d        | 32         | GPR      | —                | Lower 32 bits of r15                                               |
| r15w        | 16         | GPR      | —                | Lower 16 bits of r15                                               |
| r15b        | 8          | GPR      | —                | Low byte of r15                                                    |
| rip         | 64         | SPR      | —                | Instruction pointer; points to the next instruction to be executed |
| rflags      | 64         | SPR      | —                | Status and control flags (carry, zero, sign, overflow, etc.)       |
| cs          | 16         | SPR      | —                | Code segment                                                       |
| ds          | 16         | SPR      | —                | Data segment                                                       |
| es          | 16         | SPR      | —                | Extra segment                                                      |
| fs          | 16         | SPR      | —                | Used for thread-local storage in SysV ABI                          |
| gs          | 16         | SPR      | —                | Used for thread-local storage in Windows ABI                       |
| ss          | 16         | SPR      | —                | Stack segment                                                      |
| cr0         | 64         | SPR      | —                | Controls protected mode, caching, and paging enable                |
| cr1         | 64         | SPR      | —                | Reserved                                                           |
| cr2         | 64         | SPR      | —                | Holds the linear address that triggered the last page fault        |
| cr3         | 64         | SPR      | —                | Points to the page directory; used by the MMU for address translation |
| cr4         | 64         | SPR      | —                | Enables processor extensions such as PAE, SSE, and others          |
| cr8         | 64         | SPR      | —                | Task priority register; controls external interrupt filtering       |
| dr0         | 64         | SPR      | —                | Hardware breakpoint address 0                                      |
| dr1         | 64         | SPR      | —                | Hardware breakpoint address 1                                      |
| dr2         | 64         | SPR      | —                | Hardware breakpoint address 2                                      |
| dr3         | 64         | SPR      | —                | Hardware breakpoint address 3                                      |
| dr4         | 64         | SPR      | —                | Aliased to dr6 when CR4.DE is clear                                |
| dr5         | 64         | SPR      | —                | Aliased to dr7 when CR4.DE is clear                                |
| dr6         | 64         | SPR      | —                | Debug status; reports which breakpoint was triggered               |
| dr7         | 64         | SPR      | —                | Debug control; configures breakpoint conditions and sizes          |
| st0–st7     | 80         | SPR      | x87 FPU          | Extended precision floating-point register stack                   |
| mm0–mm7     | 64         | SPR      | MMX              | Alias the low 64 bits of st0–st7; used for integer SIMD operations |
| xmm0–xmm15 | 128        | SIMD     | SSE              | Lower 128 bits of the corresponding ymm/zmm registers              |
| ymm0–ymm15 | 256        | SIMD     | AVX              | Lower 256 bits of the corresponding zmm registers                  |
| zmm0–zmm31 | 512        | SIMD     | AVX-512          | zmm0–zmm15 alias the corresponding ymm and xmm registers           |
| k0–k7       | 64         | SPR      | AVX-512          | Opmask registers; used to predicate SIMD operations per element    |
