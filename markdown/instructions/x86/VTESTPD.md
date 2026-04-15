> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VTESTPD

Performs a bitwise logical AND between two packed double-precision floating-point vectors and sets the CF flag based on the result. The CF flag is set to 1 if any bit of the result is non-zero, and cleared to 0 if all bits of the result are zero. The destination operand is overwritten by the result of the logical AND operation.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m64/m128/m256/m512 | xmm/ymm/zmm reg |

DO NOT support LOCK

The instruction is available only in 64-bit mode or compatibility mode. It requires the AVX extension; if the processor does not support AVX, the instruction will trigger an invalid opcode exception.

To avoid undefined behavior or general protection faults, the destination register MUST match the size of the source operands. When using YMM or ZMM registers, the processor MUST be in a state where AVX or AVX-512 is enabled. Mixing different vector lengths (e.g., using a ZMM source with a YMM destination) is not permitted by the instruction encoding.