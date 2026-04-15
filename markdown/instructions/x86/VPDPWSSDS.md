> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPDPWSSDS

Multiplies signed word-sized integers from a source operand with signed word-sized integers from a second source operand, sums the resulting products into signed doubleword-sized integers in the destination operand.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| zmm reg | zmm reg |
| zmm m32 | zmm reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It is NOT supported in compatibility mode.

The destination register is updated in-place; the original values in the destination register are used as the initial accumulators for the dot product. Failure to zero the destination register before use WILL result in an incorrect summation.