> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SYSCALL

Transitions the processor from ring 3 to ring 0, loads the RIP from the LSTAR MSR, and saves the return address into RCX and the RFLAGS register into R11.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

The instruction SHALL only be executed in 64-bit mode or compatibility mode. It SHALL NOT be used in 32-bit mode. The behavior is dependent on the EFER.SCE (System Call Extensions) bit; if EFER.SCE is 0, executing SYSCALL results in an invalid opcode exception.

The user SHALL ensure that the LSTAR MSR is correctly initialized to the system call handler address before execution. The instruction DOES NOT save the stack pointer (RSP), so the kernel handler MUST swap the user stack for a kernel stack. Note that the RFLAGS register is masked by the SFMASK MSR, which may disable interrupts upon transition to ring 0.