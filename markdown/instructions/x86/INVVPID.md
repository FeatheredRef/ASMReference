> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# INVVPID

Invalidates virtual processor identifier (VPID) cached mappings in the Translation Lookaside Buffer (TLB).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | #I |

DO NOT support LOCK

This instruction SHALL only be executed in VMX root operation. Execution in VMX non-root operation or outside of VMX root operation SHALL result in an invalid opcode exception (#UD).

The `INVVPID` instruction requires the `IA32_VMX_PROC_CONFIG` MSR to be configured correctly. If the VPID enabling bit in the Secondary Processor-Based VM-Execution Controls is not set, the instruction SHALL be treated as a NOP.

To avoid unexpected TLB residency, the software MUST specify the correct invalidation type in the `reg` operand:
- 0: Individual-address invalidation.
- 1: Single-context invalidation.
- 2: All-context invalidation.

Incorrect use of these types may lead to stale translation entries and memory corruption.