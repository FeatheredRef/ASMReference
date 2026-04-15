> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPTERNLOGD

Computes a bitwise logical operation on three 32-bit packed doubleword vector operands. The specific operation is selected by an 8-bit immediate value, which acts as a truth table for the three inputs. The result is stored in the destination operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m32/m64/m128/m256/m512 | xmm/ymm/zmm reg |
| imm | #I |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode or compatibility mode. It requires the AVX-512 foundation (AVX512F) to be supported by the processor. 

When using the instruction with a memory operand, it SHALL be naturally aligned to the size of the vector register used to avoid potential performance penalties or alignment faults. If masking is applied (K-mask), the behavior regarding the destination register depends on the mask bit; if the bit is 0, the destination element is either zeroed or preserved based on the masking type (merging-masking vs zeroing-masking). Failure to specify the correct mask register for the operand width MAY result in undefined behavior.