> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KMOVD

Moves a doubleword immediate value into a 64-bit register.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| imm | r64 |
| reg | #I |
| memory | #I |

DO NOT support LOCK

This instruction SHALL only be used in 64-bit mode. It is NOT supported in compatibility mode.

The immediate value is treated as a signed 32-bit integer and is sign-extended to 64 bits before being stored in the destination register. Failure to account for sign extension when moving values larger than $2^{31}-1$ MAY result in unexpected values in the upper 32 bits of the r64 register.