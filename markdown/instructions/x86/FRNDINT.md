> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FRNDINT

Rounds the ST(0) floating-point value to the nearest integer, rounding to the nearest even number in the case of a tie. The result is stored in ST(0).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f80 | f80 |

DO NOT support LOCK

This instruction is available in both 32-bit and 64-bit mode. It operates exclusively on the x87 floating-point unit (FPU) register stack.

The instruction SHALL trigger #O if the rounded result cannot be represented within the destination precision. If the input is a NaN, #I is signaled. If the result is not exact, #P is signaled. User MUST ensure that the FPU control word is configured correctly for the desired rounding precision, as this instruction follows the rounding mode specified in the x87 control word.