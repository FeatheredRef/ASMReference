> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSHUFHW

The PSHUFHW instruction shuffles two 16-bit words from a source operand according to an immediate byte value and stores the result in a destination operand. For each 16-bit word in the destination, the instruction selects one of the two 16-bit words from the source based on the corresponding bits of the immediate value.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| imm | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

This instruction SHALL be executed in 64-bit mode or compatibility mode. It requires the SSE3 extension set to be supported by the processor.

The immediate value is used as a control mask where bits [1:0] select the word for the first 16-bit element of the destination, and bits [3:2] select the word for the second 16-bit element. If the selection bits are 0, the first 16-bit word of the source is chosen; if 1, the second 16-bit word is chosen. Bits [7:4] of the immediate value are ignored. Failure to use an SSE3-capable processor SHALL result in an invalid opcode exception.