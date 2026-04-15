> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMAXUQ

Compares two unsigned qword integers and stores the maximum value in the destination.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| r64 | r64 |
| m8 | r64 |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It is NOT supported in compatibility mode.

To avoid incorrect results, ensure that the operands are treated strictly as unsigned integers (u64), as the instruction performs an unsigned comparison. Using this instruction for signed integer comparison will yield incorrect maximum values.