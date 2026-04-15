> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FSTP

Stores the floating-point value from the top of the ST(0) register to the specified destination and then pops that value from the floating-point register stack.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| ST(0) | rN (via ST(i)) |
| ST(0) | m32 (f32) |
| ST(0) | m64 (f64) |
| ST(0) | m80 (f80) |
| ST(0) | m128 (f128) |

DO NOT support LOCK

This instruction is available in 64-bit mode, but it operates on the x87 Floating-Point Unit (FPU) state. In x86-64, while x87 instructions are supported, SSE/AVX are generally preferred.

The precision of the stored value depends on the destination operand size. If the destination is smaller than the 80-bit internal precision of ST(0), the value SHALL be rounded according to the current x87 rounding control word. This process MAY trigger #P, #U, or #O exceptions. If the destination is m32, the value is stored as an f32; if m64, as an f64. Using an incorrect memory size for the intended floating-point format SHALL result in data corruption or incorrect interpretation of the value.