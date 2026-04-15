> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LOOP

Decrements the ECX (or RCX in 64-bit mode) register by 1. If the resulting value is not zero, the program counter is updated to the target address; otherwise, execution continues with the next instruction.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (RCX/ECX) | imm |

DO NOT support LOCK

The instruction is available in 64-bit mode, but it operates on the RCX register when in 64-bit mode and ECX when in 32-bit mode. The offset provided as an immediate is a signed value relative to the instruction pointer.

To avoid infinite loops or incorrect iteration counts, the programmer SHALL ensure the counter register is initialized to a non-zero value, as the instruction decrements the register before the zero-check. If the register is initialized to 0, it will wrap around to the maximum value of the register size (u64 or u32), resulting in $2^N - 1$ iterations.