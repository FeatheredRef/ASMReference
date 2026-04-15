> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRNDSCALESD

Rounds a 64-bit floating-point value to a specific precision and scales the result by a power of 2. The instruction converts a double-precision floating-point value to a scaled double-precision floating-point value based on the rounding mode specified in the immediate operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m64 | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode and 32-bit mode. It requires the AVX instruction set.

The immediate operand MUST specify a valid rounding mode (e.g., Round to Nearest, Round Up, Round Down, or Round Toward Zero). If an invalid immediate is provided, the instruction will trigger an invalid opcode exception. The behavior of the instruction is directly dependent on the rounding mode provided in the immediate, overriding the current rounding control in the MXCSR register.

Precision (#P) and Underflow (#U) exceptions may be triggered depending on the result of the scaling and rounding operation. Ensure that the destination register is not used as a source for a subsequent operation until the instruction has completed to avoid pipeline stalls in specific microarchitectures.