> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XADD

Adds the contents of a source operand to a destination operand, storing the original value of the destination operand in the source operand and the result of the addition in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | mN |

Support LOCK

XADD requires that both operands be of the same size. In 64-bit mode, if a 32-bit operand size is specified, the upper 32 bits of the destination register are not affected. The instruction is supported in 64-bit mode, 32-bit mode, and compatibility mode.

The instruction updates the EFLAGS register. Specifically, the Carry Flag (CF), Zero Flag (ZF), Sign Flag (SF), Overflow Flag (OF), Auxiliary Flag (AF), and Parity Flag (PF) are affected based on the result of the addition. When using a memory destination with the LOCK prefix, the operation is atomic, preventing other processors from accessing the memory location until the exchange and addition are complete.