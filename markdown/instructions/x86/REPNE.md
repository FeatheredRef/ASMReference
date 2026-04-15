> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# REPNE

Repeats the `SCAS` or `CMPS` instruction until `CX/ECX/RCX` is 0 or the Zero Flag (ZF) is set.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| mN | mN |
| reg | mN |

Support LOCK

The instruction is available in 64-bit mode, compatibility mode, and 32-bit protected mode. The behavior depends on the destination index register (`EDI/RDI`) and source index register (`ESI/RSI`) and the Direction Flag (DF). If DF=0, indices are incremented; if DF=1, indices are decremented.

To avoid infinite loops, `RCX` MUST be initialized to a non-zero value unless the intent is to skip the operation. If `RCX` is set to 0, the instruction is treated as a no-operation and `RCX` remains 0. In 64-bit mode, if a 32-bit `ECX` is used, the upper 32 bits of `RCX` MUST be handled carefully to avoid unexpected iteration counts. Ensure that the memory regions pointed to by `RSI` and `RDI` are valid and do not overlap in a way that violates the logic of the search/compare operation to avoid memory access violations.