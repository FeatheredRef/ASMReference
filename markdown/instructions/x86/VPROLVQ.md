> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPROLVQ

Performs a variable precision low-precision multiply-accumulate operation on 64-bit signed integers. It multiplies two signed 64-bit integer values and adds the result to a destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m8 |
| #I | #I |

DO NOT support LOCK

This instruction is ONLY available when the processor is operating in 64-bit mode or compatibility mode. It requires the AVX-512 extension set to be enabled.

To avoid undefined behavior or exceptions, ensure that the destination register is correctly initialized if used as an accumulator. Failure to align memory operands to the required boundary may result in a general protection fault (#GP).