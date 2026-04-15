> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BNDCU

Clears the upper bound of a specified bound register. This instruction sets the upper bound of the specified register to the maximum possible value (all ones), effectively disabling the upper bound check for that register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is not supported in compatibility mode.

The instruction specifically targets the bound registers (BND0-BND3). Attempting to use this instruction on general-purpose registers that are not designated as bound registers will result in an invalid operation. Because BNDCU clears the upper bound, any subsequent BNDCL or BNDCU operations on the same register will overwrite the state. Ensure that the bound register is correctly initialized before performing bounds checks to avoid unintended memory access permissions.