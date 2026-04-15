> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPCMPUB

Compares two unsigned bytes in each lane of the source operands according to the specified predicate and sets the destination mask based on the result.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm/ymm/xmm reg | zmm/ymm/xmm reg |
| zmm/ymm/xmm m | zmm/ymm/xmm reg |
| imm | zmm/ymm/xmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512B extension to be supported by the processor.

The destination register must be different from the first source operand register to avoid undefined behavior in some implementations, although it is generally supported. When using memory operands, the memory must be aligned to the vector size to avoid performance penalties or faults depending on the alignment check settings. If the instruction is used with an immediate value, the immediate is broadcasted across all lanes of the vector.