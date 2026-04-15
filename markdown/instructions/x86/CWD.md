> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CWD

Extends a signed word (16-bit) value in the AX register to a signed double word (32-bit) value across the EAX register. It copies the sign bit of AX into the upper 16 bits of EAX.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (AX) | reg (EAX) |

DO NOT support LOCK

This instruction is available in 32-bit protected mode and 64-bit mode. In 64-bit mode, it operates specifically on the AX and EAX registers to maintain compatibility with 32-bit code. It does not affect the upper 32 bits of the RAX register.

The user SHALL ensure that CWD is used specifically for sign-extending a 16-bit integer to 32-bit before performing operations like `IDIV EAX` to avoid incorrect results due to improper sign extension. Failure to sign-extend will result in the upper 16 bits of EAX containing stale data, leading to an incorrect quotient or a #Z / #O exception.