> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUB231PD

Computes the fused multiply-subtract of two double-precision floating-point values and subtracts the result from a third double-precision floating-point value. The operation is performed as: $\text{destination} = \text{source1} - (\text{source2} \times \text{source3})$.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | reg |
| reg | reg |

DO NOT support LOCK

This instruction requires AVX support. It is not available in compatibility mode; it requires 64-bit mode or 32-bit mode with AVX enabled.

The destination register must not overlap with any of the source registers to avoid undefined behavior. This instruction utilizes the MXCSR register for rounding control and exception masking.

The instruction will trigger floating-point exceptions such as #O, #U, #P, or #D based on the result of the fused operation and the state of the MXCSR register. Because it is a fused operation, only one rounding step is performed at the end, which prevents intermediate precision loss.