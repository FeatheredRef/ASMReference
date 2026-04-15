> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPSRAVQ

Shifts the 64-bit quadwords in the destination vector operand to the right by the count specified by the immediate value. The shift is arithmetic, meaning the sign bit of each quadword is preserved.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| imm | r128/r256/r512 |
| imm | m16/m32/m64 |

DO NOT support LOCK

This instruction is available only when the processor is operating in 64-bit mode or compatibility mode. It requires AVX-512 support (specifically AVX-512F).

The shift count must be an immediate value between 0 and 63. Providing an immediate value outside of this range SHALL result in an encoding error. Since this is a SIMD operation, the shift is applied independently to each 64-bit element within the vector; bits shifted out are discarded, and no bits are shifted between elements of the vector.