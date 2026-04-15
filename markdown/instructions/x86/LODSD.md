> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LODSD

Loads a double word from the memory location pointed to by the address in the ESI or RSI register into the EAX or RAX register, then increments or decrements the source index register by 4 bytes based on the current direction flag (DF) setting.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m32 | r32 |

DO NOT support LOCK

In 64-bit mode, if the address size is 64 bits, the instruction uses RSI as the source index and loads the value into EAX. If the address size is 32 bits (compatibility mode), it uses ESI and loads the value into EAX. The operation is strictly limited to the source index register and the accumulator register; no other registers or immediate values SHALL be used for these operands.

The Direction Flag (DF) MUST be correctly configured before execution to avoid incorrect memory traversal: if DF=0, the source index is incremented; if DF=1, the source index is decremented. Failure to set DF via `CLD` or `STD` may result in reading from unintended memory addresses.