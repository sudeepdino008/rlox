

expression     -> assignment
assignment     → identifier "=" assignment | logic_or 
logic_or       -> logic_and ( "or" logic_and )* 
logic_and      -> equality ( "and" equality)* 
equality       → comparison ( ( "!=" | "==" ) comparison )* 
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* 
term           → factor ( ( "-" | "+" ) factor )* 
factor         → unary ( ( "/" | "*" ) unary )* 
unary          → ( "!" | "-" ) unary
               | call 
call           -> primary ( "(" arguments? ")" )*
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" | IDENTIFIER

arguments -> expression ("," expression)*


program -> declaration* EOF ;
declaration -> funDecl | varDecl | statement;
funDecl     -> "fun" function;
function    -> IDENTIFIER "(" parameters? ")" block ;
parameters  -> IDENTIFIER ( "," IDENTIFIER)* ;
varDecl -> "var" identifier ( "=" expression )? ";";
statement -> exprStmt | printStmt | block | ifStmt | whileStmt | breakStmt;
ifStmt -> "if" expression statement ("else" statement)? ; // different from lox in that expression 
                                                             doesn't need to be bracketed
breakStmt -> "break" ";"
block -> "{" declaration* "}"
exprStmt -> expression ";"
printStmt -> "print" expression ";"
whileStmt -> "while" expression block   // slight different from lox, expression doesn't need to  
                                           be parathesized; and it's a block instead of statement (different compared to if)