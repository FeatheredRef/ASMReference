> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FUCOMIP

Compares the value in the ST(0) register with a specified floating-point value and pops the value from ST(0).

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| f80 | ST(0) |
| m80 | ST(0) |

DO NOT support LOCK

This instruction is only available in compatibility mode when targeting x86-64. It operates exclusively on the x87 floating-point stack.

The instruction modifies the FPU status word and the CPU EFLAGS register (CF, ZF, PF). Users MUST ensure that the FPU stack is not empty before execution to avoid a stack underflow exception. Because this instruction pops ST(0), it reduces the current stack top (TOP) by one.

The comparison results in the following EFLAGS states based on the value of ST(0) relative to the source:
- ZF = 1, CF = 0: Values are equal.
- ZF = 0, CF = 1: ST(0) is greater than the source.
- ZF = 0, CF = 0: ST(0) is less than the source.

Possible exceptions include:
- #I: Invalid operation.
- #D: Denormalized operand.