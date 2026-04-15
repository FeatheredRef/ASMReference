> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BTS

Copies one bit from a specified bit position in the source operand to the CF flag, then sets the bit at that position to 1.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | rN |
| rN | rN |
| imm | mN |
| rN | mN |

Support LOCK

BTS is available in 64-bit mode. When operating on mN, the instruction performs a read-modify-write operation.

The bit index specified by the source operand MUST be less than the bit width of the destination operand; otherwise, the behavior is undefined. For m8, the index MUST be 0-7; for m16, 0-15; for m32, 0-31; and for m64, 0-63.

When using BTS with a memory operand, the LOCK prefix SHALL be used if the operation must be atomic across multiple processors. Without the LOCK prefix, the operation is not guaranteed to be atomic on multiprocessor systems.