> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGATHERPF0QPD

Gathers 64-bit floating-point values from memory into a destination register. It uses a base address, a set of indices from a register, and a scale factor to calculate the effective addresses. The instruction uses a mask register to track which elements have been successfully loaded; elements with a mask bit set to 0 are skipped, and bits are cleared as elements are retrieved.

The table below covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m64 | f64 (zmm/ymm) |
| reg (index) | f64 (zmm/ymm) |
| reg (mask) | reg (mask) |

DO NOT support LOCK

This instruction SHALL be executed only in 64-bit mode. It is NOT supported in compatibility mode.

The instruction behavior is dependent on the mask register. If the mask bit corresponding to an element is 0, the memory access for that element is skipped. If a page fault occurs during the gather operation, the state of the mask register is updated to reflect which elements were successfully loaded before the fault, allowing the handler to resume the operation without duplicating loads.

The scale factor is encoded as an immediate and MUST be 1, 2, 4, or 8. Using an unsupported scale value will result in an invalid opcode. Ensure that the mask register is correctly initialized before execution, as the instruction will not perform any gathers if all mask bits are 0.