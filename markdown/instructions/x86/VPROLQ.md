> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPROLQ

VPROLQ performs a packed multiply-add operation on 64-bit signed integers. It multiplies the 64-bit signed integer elements of the first source operand by the 64-bit signed integer elements of the second source operand and adds the result to the 64-bit signed integer elements of the destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmmN / m64 | zmmN |

DO NOT support LOCK

This instruction is available only in 64-bit mode and requires AVX-512 foundation support. It is not available in compatibility mode.

The operation is performed on packed 64-bit elements; failure to ensure the input registers are correctly aligned to 64-byte boundaries when using memory operands MAY result in performance degradation or alignment faults depending on the processor configuration. Because this instruction utilizes the ZMM registers, the processor MUST be in a state that supports 512-bit vectors to avoid an invalid opcode exception.