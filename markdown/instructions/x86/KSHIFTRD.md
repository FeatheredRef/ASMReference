> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KSHIFTRD

Shifts the bits of a qword source to the right by a count specified in the CL register. This operation performs a logical shift, filling the most significant bits with zeros.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It SHALL NOT be used in compatibility mode.

The shift count is masked to 6 bits (0-63) for qword operands; any bits in CL beyond the 6th bit are ignored. The Carry Flag (CF) is updated based on the last bit shifted out of the LSB.