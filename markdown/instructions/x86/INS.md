> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# INS

Reads a word from the specified port and stores it into the accumulator register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | r16 |
| rN | r16 |

DO NOT support LOCK

The instruction is available in 64-bit mode, 32-bit mode, and 16-bit mode. In 64-bit mode, the destination register is always AX (the lower 16 bits of RAX).

The port address MUST be between 0 and 65535. If a register is used to specify the port, only the lower 16 bits of the register are used. In 64-bit mode, the `INS` instruction is typically used in compatibility mode; however, direct I/O access depends on the I/O privilege level (IOPL) and the I/O permission bitmap in the TSS. Failure to have sufficient privilege SHALL result in a General Protection Exception (#GP).