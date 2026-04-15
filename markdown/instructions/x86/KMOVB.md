> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KMOVB

Copies a byte from a source to a destination based on the value of a mask register. The operation is performed for each byte element of the destination; if the corresponding bit in the mask register is set, the byte is copied; otherwise, the destination byte remains unchanged.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m1 |
| m1 | reg |
| m1 | m1 |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode and requires the AVX-512 foundation extensions. It operates on vector registers (zmm/ymm/xmm) and requires the use of opmask registers (k0-k7).

The mask register MUST be specified to avoid undefined behavior regarding the elements not targeted by the mask. If the destination is a memory operand, the memory region MUST be properly aligned to the vector size to avoid performance degradation or alignment check exceptions. Failure to use a valid mask register will result in the instruction failing to update the intended target elements.