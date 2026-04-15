> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMULLW

Multiplies the unsigned 16-bit integers in each lane of the first source operand by the corresponding unsigned 16-bit integers in the second source operand. The lower 16 bits of the result for each lane are stored in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |

DO NOT support LOCK

This instruction is only available when the processor supports the SSE4.1 instruction set. It operates on XMM registers and is not available in 16-bit operation mode.

The instruction performs an unsigned multiplication; if signed multiplication is required, `PMULHW` or `PMULLW` cannot be used interchangeably as they treat the input bit patterns as unsigned. Because only the lower 16 bits of the product are retained, overflow occurs silently for any product exceeding $2^{16}-1$, and the upper 16 bits of the result are discarded.