> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# POPCNT

Counts the number of set bits (1s) in the source operand and stores the result in the destination operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| imm | #I |

DO NOT support LOCK

The instruction is available in 64-bit mode and compatibility mode. It requires the SSE4.2 feature flag to be supported by the processor; if the CPU does not support the POPCNT extension, the instruction will trigger an invalid opcode exception.

The destination register is modified regardless of whether the result is zero. Specifically, the destination register is cleared before the population count is added, meaning the instruction does not perform a read-modify-write operation on the destination. To avoid unexpected behavior in legacy environments, software MUST verify CPUID support for the `POPCNT` bit in `ECX` after calling `EAX=1`.