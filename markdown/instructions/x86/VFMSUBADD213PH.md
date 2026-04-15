> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUBADD213PH

Performs a fused multiply-subtract and add operation on packed half-precision floating-point values. It computes the result of (a * b) - c + d for each corresponding element of the source operands, where the operands are mapped according to the 213PH pattern.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

This instruction is available only when the processor supports the AVX-512 Fused Multiply-Add (FMA) and FP16 extensions. It requires the processor to be operating in 64-bit mode or compatibility mode.

To avoid precision loss or incorrect results, the user SHALL ensure that the destination register is not used as a source unless the intended behavior is destructive. Incorrect alignment of memory operands (if applicable via surrounding instructions) may trigger a general protection fault. The operation is subject to floating-point exception flags, specifically #P, #O, #U, and #D, based on the MXCSR register settings.