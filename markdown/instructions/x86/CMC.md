> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CMC

Complements the Carry Flag (CF).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | CF |

DO NOT support LOCK

The CMC instruction is available in 64-bit mode, 32-bit mode, and 16-bit mode. It does not have any architectural constraints regarding the operating mode.

The instruction only affects the CF flag; all other flags in the RFLAGS register remain unchanged. Since it does not take any operands, it cannot trigger memory-related exceptions.