def Pat<Copy gr -> gr> {
    asm -> mov $out, $1;
}

def Pat<Copy fp -> fp> f32 {
    asm -> movss $out, $1;
}

def Pat<Copy fp -> fp> f64 {
    asm -> movsd $out, $1;
}

def Pat<Copy imm -> gr> {
    asm -> mov $out, $1;
}

def Pat<Copy mem -> gr> {
    asm -> mov $out, $1;
}

def Pat<Add .., .. -> ..> int
    map %t1, gr
{
    asm -> mov %t1, $1;
    asm -> add %t1, $2;
    asm -> mov $out, %t1;
}

def Pat<Sub .., .. -> ..> int
    map %t1, gr
{
    asm -> mov %t1, $1;
    asm -> sub %t1, $2;
    asm -> mov $out, %t1;
}

def Pat<Ret> {
    asm -> ret;
}

def Pat<GetFramePtr -> gr> {
    asm -> mov $out, Operand::Reg(X86Reg::Rbp()); 
}

def Pat<GetStackPtr -> gr> {
    asm -> mov $out, Operand::Reg(X86Reg::Rsp()); 
}

def Pat<Add fp, fp -> fp> f32 
    map %t1, fp
{
    asm -> mov %t1, $1;
    asm -> add %t1, $2;
    asm -> mov $out, %t1;
}

def Pat<Add fp, fp -> fp> <2 x i64> 
    map %t1, fp
{
    asm -> movdqa %t1, $1;
    asm -> paddq %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Add fp, fp -> fp> <4 x i32> 
    map %t1, fp
{
    asm -> movdqa %t1, $1;
    asm -> paddd %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Sub fp, fp -> fp> <4 x i32> 
    map %t1, fp
{
    asm -> movdqa %t1, $1;
    asm -> psubd %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Sub fp, fp -> fp> <2 x i64> 
    map %t1, fp
{
    asm -> movdqa %t1, $1;
    asm -> psubq %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Sub fp, fp -> fp> <8 x i16> 
    map %t1, fp
{
    asm -> movdqa %t1, $1;
    asm -> psubw %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Sub fp, fp -> fp> <16 x i8> 
    map %t1, fp
{
    asm -> movdqa %t1, $1;
    asm -> psubb %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Copy fp -> fp> <4 x i32> {
    asm -> movdqa $out, $1;
}

def Pat<Br(_)> hook lower_br {}


def Pat<CmpEq .., .. -> ..> int 
    map %t1, gr
    map %t2, gr
{
    asm -> mov %t1, $2;
    asm -> mov %t2, $1;
    asm -> mov $out, Operand::Const(0);
    asm -> cmp %t1, %t1;
    asm -> sete $out;
}

def Pat<CmpNe .., .. -> ..> int 
    map %t1, gr
    map %t2, gr
{
    asm -> mov %t1, $2;
    asm -> mov %t2, $1;
    asm -> mov $out, Operand::Const(0);
    asm -> cmp %t1, %t1;
    asm -> setne $out;
}

def Pat<CmpLt .., .. -> ..> int 
    map %t1, gr
    map %t2, gr
{
    asm -> mov %t1, $2;
    asm -> mov %t2, $1;
    asm -> mov $out, Operand::Const(0);
    asm -> cmp %t1, %t1;
    asm -> setl $out;
}

def Pat<CmpLte .., .. -> ..> int 
    map %t1, gr
    map %t2, gr
{
    asm -> mov %t1, $2;
    asm -> mov %t2, $1;
    asm -> mov $out, Operand::Const(0);
    asm -> cmp %t1, %t1;
    asm -> setle $out;
}

def Pat<CmpGt .., .. -> ..> int 
    map %t1, gr
    map %t2, gr
{
    asm -> mov %t1, $2;
    asm -> mov %t2, $1;
    asm -> mov $out, Operand::Const(0);
    asm -> cmp %t1, %t1;
    asm -> setg $out;
}

def Pat<CmpGte .., .. -> ..> int 
    map %t1, gr
    map %t2, gr
{
    asm -> mov %t1, $2;
    asm -> mov %t2, $1;
    asm -> mov $out, Operand::Const(0);
    asm -> cmp %t1, %t1;
    asm -> setge $out;
}

def Pat<VecInsrt fp, imm, imm -> fp> <8 x i64> 
    map %t1, gr
{
    asm -> mov %t1, $3;
    asm -> movdqa $out, $1;
    asm -> pinsrq $out, $2;
}

def Pat<VecInsrt fp, imm, imm -> fp> <4 x i32> 
    map %t1, gr
{
    asm -> mov %t1, $3;
    asm -> movdqa $out, $1;
    asm -> pinsrd $out, $2;
}

def Pat<VecInsrt fp, imm, imm -> fp> <8 x i16> 
    map %t1, gr
{
    asm -> mov %t1, $3;
    asm -> movdqa $out, $1;
    asm -> pinsrw $out, $2;
}

def Pat<VecInsrt fp, imm, imm -> fp> <16 x i8> 
    map %t1, gr
{
    asm -> mov %t1, $3;
    asm -> movdqa $out, $1;
    asm -> pinsrb $out, $2;
}

def Pat<VecInsrt fp, imm, imm -> fp> <4 x f32> 
    map %t1, gr
{
    asm -> mov %t1, $3;
    asm -> movdqa $out, $1;
    asm -> insertps $out, $2;
}