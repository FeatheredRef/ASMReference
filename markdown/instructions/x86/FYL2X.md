> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FYL2X

Computes $y = x \cdot \log_2(x)$ where $x$ is the value in the ST(0) register. The result replaces the value in ST(0).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

This instruction is only available in compatibility mode when operating in x86-64 architecture. It is a floating-point instruction that operates exclusively on the x87 FPU register stack.

The operand $x$ SHALL be a positive finite number. If $x$ is negative, NaN, or infinity, the instruction triggers #I. If $x$ is zero, the result is zero and no exception is triggered. The precision of the result is subject to the current FPU control word; if the result cannot be represented exactly, #P is triggered.