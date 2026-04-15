> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# OUT

Outputs data from the accumulator register (AL, AX, or EAX) to a specified I/O port.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| r8 | u8 (imm8 or r8) |
| r16 | u8 (imm8 or r8) |
| r32 | u8 (imm8 or r8) |
| #I | mN |

DO NOT support LOCK

The instruction is available in 64-bit mode, compatibility mode, and 32-bit/16-bit protected modes. The destination port must be specified by an 8-bit immediate or the DX register. If the destination port exceeds 255, the DX register SHALL be used.

The I/O privilege level (IOPL) in RFLAGS and the I/O permission bitmap in the TSS control access to the I/O ports. If the current privilege level (CPL) is greater than the IOPL and the port is not permitted in the bitmap, a General Protection Exception (#GP) SHALL occur.

The size of the data transferred is determined by the accumulator register used: AL for 1 byte, AX for word, and EAX for dword. Using a register other than the accumulator as the source is not supported.