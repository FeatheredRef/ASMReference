> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FCOM

Compares two floating-point values stored in an ST(0) register and a source operand. The instruction sets the floating-point status word flags based on whether the value in ST(0) is smaller than, equal to, or larger than the source operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | ST(0) |
| mN | ST(0) |

DO NOT support LOCK

FCOM is only available in x86-64 when operating in compatibility mode or when using the legacy x87 FPU stack. It is not supported in 64-bit mode for operations other than those targeting the x87 FPU registers.

The instruction will trigger #I if either operand is a Signaling NaN. If either operand is a Quiet NaN, the result is undefined and the condition codes are not updated. Precision and underflow exceptions (#P, #U) may occur if the operands are denormalized.