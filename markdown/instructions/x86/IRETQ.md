> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# IRETQ

Returns control from an interrupt handler to the calling program. It pops the RIP, CS, RFLAGS, RSP, and SS from the stack to restore the processor state. If a privilege level change occurs, the stack pointer is switched to the stack associated with the destination privilege level.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m16/m32/m64 | reg / m64 |
| #I | #I |

DO NOT support LOCK

The instruction SHALL only be executed in 64-bit mode or compatibility mode. In compatibility mode, it behaves as IRET. If executed in 64-bit mode, the instruction MUST pop five qwords from the stack (RIP, CS, RFLAGS, RSP, SS) if a privilege level change occurs, or three qwords (RIP, CS, RFLAGS) if no privilege level change occurs.

To avoid General Protection exceptions (#GP), the CS selector MUST be valid and the Requested Privilege Level (RPL) MUST be less than or equal to the Current Privilege Level (CPL). If the processor is returning to a different privilege level, the RSP and SS values popped from the stack MUST be valid. Failure to ensure the stack contains a valid 64-bit canonical address for RSP will result in a Stack Fault (#SS).