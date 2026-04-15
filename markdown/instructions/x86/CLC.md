> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CLC

The CLC instruction clears the Carry Flag (CF) in the RFLAGS register by setting it to 0.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is available in both 64-bit mode and compatibility mode. It does not have architectural constraints regarding operand size as it operates solely on a specific bit of the RFLAGS register.

The CLC instruction does not affect any flags other than CF. Since it does not access memory or take registers as operands, it cannot trigger general protection faults or alignment check exceptions.