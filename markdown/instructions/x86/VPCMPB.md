> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPCMPB

Compares two bytes in each lane of the source operands according to the specified immediate condition and stores the result in the destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg, mN | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode when using AVX-512 extensions. It REQUIRES the use of ZMM or YMM registers; attempting to use XMM registers will result in an invalid opcode exception if the specific AVX-512 foundation is not supported.

The destination register must not overlap with the source registers to avoid undefined behavior. The immediate byte determines the comparison predicate (e.g., equal, not-equal, less-than, greater-than); using an immediate value that does not correspond to a valid comparison predicate will result in an invalid opcode exception. The mask register (k-register) may be used for conditional updating of the destination elements.