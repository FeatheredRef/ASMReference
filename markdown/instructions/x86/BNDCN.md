> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BNDCN

BNDCN clears the bound-checking control flag (BNDCN) in the BNDCN register, effectively disabling bound checking for subsequent instructions.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | r64 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is not supported in compatibility mode.

The BNDCN instruction does not affect the bound registers themselves; it only modifies the control flag that determines whether bound-checking instructions should trigger an exception. Failure to set this flag correctly before performing bound-checked memory accesses may lead to unexpected `#GP` exceptions if bounds are violated.