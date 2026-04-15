> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FCMOVcc

Copies the value of an f80-bit floating-point operand to an f80-bit destination operand if the specified condition code (cc) is met. If the condition is not met, the destination operand remains unchanged.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m8 | reg |
| #I | #I |

DO NOT support LOCK

The instruction is only available in 32-bit mode and compatibility mode. It is NOT supported in 64-bit mode.

The instruction operates exclusively on the x87 FPU register stack. Since it modifies the FPU state, it MUST be used in contexts where the FPU stack pointer (TOP) is correctly tracked to avoid stack overflow or underflow. Because it is a conditional move, it does not affect the FPU status word or control word, and it does not trigger floating-point exceptions.