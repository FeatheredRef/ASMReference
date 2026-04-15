> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# GETSEC[SMCTRL]

Retrieves the Secure Management Mode (SMM) control settings from the processor and stores the result in the destination register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| mN | #I |
| #I | r64 |

DO NOT support LOCK

This instruction SHALL only be executed in Secure Management Mode (SMM). Execution outside of SMM SHALL result in a general protection exception (#GP).

The destination register MUST be a 64-bit general-purpose register to receive the SMCTRL value. Failure to use a 64-bit register in a 64-bit mode context may lead to unexpected behavior or truncation. Since this instruction accesses hardware-specific security controls, it is only available on processors that support the corresponding Secure Management extensions.