> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PANDN

Computes the bitwise AND of the first operand and the NOT of the second operand, storing the result in the destination operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| reg | mN |
| mN | mN |
| imm | #I |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It is not supported in 32-bit mode. The instruction MUST use the same operand size for both source and destination.

When using memory operands, the memory region MUST be aligned to the natural boundary of the operand size to avoid performance penalties or alignment exceptions depending on the processor configuration. Since this instruction performs a read-modify-write operation when the destination is memory, the destination operand MUST be a valid writable memory address.