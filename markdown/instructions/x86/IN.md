> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# IN

Reads a byte or word from a specified I/O port into the accumulator register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| dx or imm | r8 or r16 |

DO NOT support LOCK

The instruction is available in 64-bit mode, compatibility mode, and 32-bit mode. Access to I/O ports is restricted by the I/O privilege level (IOPL) field in the RFLAGS register and the I/O permission bitmap in the Task State Segment (TSS). If the current privilege level (CPL) is greater than IOPL and the corresponding bit in the I/O permission bitmap is set, a General Protection fault (#GP) SHALL be generated.

When using an immediate value to specify the port, the value MUST be within the range of 0 to 255 (u8). To access ports in the range 256 to 65535, the port address MUST be loaded into the DX register. Attempting to use an immediate value larger than 255 for the port address SHALL result in an invalid operation.