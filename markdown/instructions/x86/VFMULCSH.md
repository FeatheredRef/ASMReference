> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMULCSH

Multiplies two floating-point values, converts the result to a smaller precision, and stores the result.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg, reg | reg |
| reg, mN | reg |
| mN, reg | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires AVX support.

The precision of the conversion is determined by the instruction suffix and operand size. Failure to align the destination register size with the conversion target SHALL result in an invalid operation.

Precision loss MAY occur during the conversion from a larger floating-point format to a smaller one. If the result cannot be represented exactly in the target format, the #P exception SHALL be signaled. Underflow (#U) or overflow (#O) MAY be signaled depending on the magnitude of the product relative to the target precision's range.