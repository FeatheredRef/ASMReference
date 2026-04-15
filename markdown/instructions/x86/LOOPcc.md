> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LOOPcc

Decrements the `rcx` register by 1. If the `rcx` register is not 0 and the condition specified by the `cc` suffix is met, the program counter is updated to the target address.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (`rcx`) | imm |

DO NOT support LOCK

This instruction is only available in 32-bit mode and compatibility mode. In 64-bit mode, `LOOPcc` is not supported.

The target address must be within a relative offset range of the current instruction. Since `rcx` is implicitly used as the counter, any manual modification of `rcx` within the loop body will directly affect the iteration count and may lead to infinite loops if `rcx` is set to 0.