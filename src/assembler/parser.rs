
struct Parser;

// Set up Instruction structs
//
// First pass:
//  parse directives directly (i.e. set offset to .ORIG value)
//  line by line, match first element, expect other elements to build instructions
//  store LabelAssign line number in symbol table
// Second pass:
//  Update Label References to line number + offset 
//
// Emit bytecode:
//  Convert Instruction to 16-bit bytecode
//  Push bytes to chunk
//  Write chunk to .ch8 file
