> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FDIVRP

Divides the ST(0) floating-point stack value by the floating-point value at the specified memory location or register, rounding the result to the precision specified by the current rounding control field in the floating-point control word. The result is stored in ST(0).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | ST(0) |
| mN | ST(0) |

DO NOT support LOCK

This instruction is only available in x86-64 compatibility mode. In 64-bit mode, x87 floating-point instructions are generally superseded by SSE/AVX, though they remain supported for backward compatibility.

The instruction MUST be used with valid floating-point operands (f16, f32, f64, or f80). Using an invalid operand size or an unsupported memory address will trigger a general-protection exception.

The following floating-point exceptions may occur:
- #Z: Triggered if the divisor is zero.
- #O: Triggered if the result is too large to be represented.
- #U: Triggered if the result is too small to be represented.
- #P: Triggered if the result is rounded to a less precise value.
- #D: Triggered if an operand is a denormalized number.

Ensure the floating-point control word is correctly configured before execution, as the `P` suffix (rounding precision) relies strictly on the current rounding control field. Failure to set the rounding mode correctly may lead to unexpected precision errors.