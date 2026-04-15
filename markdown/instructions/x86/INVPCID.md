> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# INVPCID

Invalidates process-context identifiers (PCIDs) in the translation-lookaside buffer (TLB) based on the provided immediate value and the current state of CR4.PCID.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| imm | #I |

DO NOT support LOCK

The instruction is only available if CR4.PCID is set to 1; otherwise, it may trigger an undefined operation (#UD) exception. It operates in both 64-bit mode and compatibility mode.

The immediate operand determines the scope of the invalidation:
- `0`: Invalidates all TLB entries for the current PCID.
- `1`: Invalidates all TLB entries for all PCIDs.
- `2`: Invalidates TLB entries for the PCID specified in CR3.
- `3`: Invalidates TLB entries for the PCID specified in CR3, but only those not marked as global.

Failure to set CR4.PCID before executing INVPCID WILL result in an #UD exception. When using immediate `1`, the instruction affects all PCIDs, regardless of the current value in CR3.