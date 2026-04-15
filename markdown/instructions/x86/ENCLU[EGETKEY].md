> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLU[EGETKEY]

Retrieves the key associated with the current enclave and stores it in the specified destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | r64 |

DO NOT support LOCK

This instruction is only available when the processor is operating in enclave mode. Attempting to execute this instruction outside of an enclave will result in a general protection exception (#GP).

The destination register must be a 64-bit general-purpose register. Failure to provide a valid register will result in an invalid opcode exception. The retrieved key is specific to the enclave and is not accessible to any code running outside the enclave boundary.