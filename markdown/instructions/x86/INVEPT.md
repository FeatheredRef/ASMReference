> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# INVEPT

Invalidates Extended Page Table (EPT) translations. It flushes the EPT caches (TLBs) based on the specified type of invalidation, either for a single EPTP or for all EPTPs.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m128 | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in VMX root operation. It is NOT supported in VMX non-root operation or in compatibility mode.

The `INVEPT` instruction requires an operand pointing to an EPT pointer descriptor. This descriptor MUST be a 128-bit aligned memory region containing a 64-bit EPT pointer (EPTP) and a 64-bit invalidation type.

The invalidation type MUST be one of the following:
- Single-context invalidation: Invalidates translations associated with the EPTP specified in the descriptor.
- Global invalidation: Invalidates translations for all EPTPs.

Failure to align the memory operand to a 16-byte boundary MAY result in a general-protection exception (#GP). Using an unsupported invalidation type value SHALL result in an invalid-opcode exception (#UD).