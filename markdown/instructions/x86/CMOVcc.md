> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CMOVcc

Copies the value from the source operand to the destination operand if the EFLAGS condition code (cc) is met. If the condition is not met, the destination operand remains unchanged.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| reg | mN |

DO NOT support LOCK

The instruction SHALL NOT be used with an immediate value as a source; such an operation is #I. It is supported in 64-bit mode and compatibility mode.

To avoid unexpected behavior, the programmer MUST ensure that the destination operand is not the same as the source operand if the condition is not met, although this is architecturally transparent. Because CMOVcc reads both the source and the destination regardless of whether the condition is met, it MAY introduce a data dependency on the destination register, which can impact performance in pipeline execution compared to conditional branching.