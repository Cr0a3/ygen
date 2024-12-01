def Pat<Copy gr -> gr> {
    asm -> mov $out, $1;
}

def Pat<Copy gr -> mem> {
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
    map %t1, gr;
{
    asm -> mov %t1, $1;
    asm -> add %t1, $2;
    asm -> mov $out, %t1;
}

def Pat<And .., .. -> ..> int
    map %t1, gr;
{
    asm -> mov %t1, $1;
    asm -> and %t1, $2;
    asm -> mov $out, %t1;
}

def Pat<Or .., .. -> ..> int
    map %t1, gr;
{
    asm -> mov %t1, $1;
    asm -> or %t1, $2;
    asm -> mov $out, %t1;
}

def Pat<Xor .., .. -> ..> int
    map %t1, gr;
{
    asm -> mov %t1, $1;
    asm -> xor %t1, $2;
    asm -> mov $out, %t1;
}

def Pat<Mul .., .. -> ..> int
    map %t1, gr;
{
    asm -> mov %t1, $2;
    asm -> mov $out, $1;
    asm -> imul $out, %t1;
}

def Pat<Sub .., .. -> ..> int
    map %t1, gr;
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
    map %t1, fp;
{
    asm -> movss %t1, $1;
    asm -> addss %t1, $2;
    asm -> movss $out, %t1;
}

def Pat<Add fp, fp -> fp> <2 x i64> 
    map %t1, fp;
{
    asm -> movdqa %t1, $1;
    asm -> paddq %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Add fp, fp -> fp> <4 x i32> 
    map %t1, fp;
{
    asm -> movdqa %t1, $1;
    asm -> paddd %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Sub fp, fp -> fp> <4 x i32> 
    map %t1, fp;
{
    asm -> movdqa %t1, $1;
    asm -> psubd %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Sub fp, fp -> fp> <2 x i64> 
    map %t1, fp;
{
    asm -> movdqa %t1, $1;
    asm -> psubq %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Sub fp, fp -> fp> <8 x i16> 
    map %t1, fp;
{
    asm -> movdqa %t1, $1;
    asm -> psubw %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Sub fp, fp -> fp> <16 x i8> 
    map %t1, fp;
{
    asm -> movdqa %t1, $1;
    asm -> psubb %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Copy fp -> fp> <4 x i32> {
    asm -> movdqa $out, $1;
}

def Pat<Br(_)> hook lower_br; {}
def Pat<BrIfEq(_) gr, ..> 
    map %t1, gr;
    hook lower_breq; 
{
    asm -> mov %t1, $2;
    asm -> cmp $1, %t1;
}

def Pat<CmpEq .., .. -> ..> int 
    map %t1, gr;
    map %t2, gr;
{
    asm -> mov %t1, $2;
    asm -> mov %t2, $1;
    asm -> mov $out, Operand::Const(0);
    asm -> cmp %t1, %t1;
    asm -> sete $out;
}

def Pat<CmpNe .., .. -> ..> int 
    map %t1, gr;
    map %t2, gr;
{
    asm -> mov %t1, $2;
    asm -> mov %t2, $1;
    asm -> mov $out, Operand::Const(0);
    asm -> cmp %t1, %t1;
    asm -> setne $out;
}

def Pat<CmpLt .., .. -> ..> int 
    map %t1, gr;
    map %t2, gr;
{
    asm -> mov %t1, $2;
    asm -> mov %t2, $1;
    asm -> mov $out, Operand::Const(0);
    asm -> cmp %t1, %t1;
    asm -> setl $out;
}

def Pat<CmpLte .., .. -> ..> int 
    map %t1, gr;
    map %t2, gr;
{
    asm -> mov %t1, $2;
    asm -> mov %t2, $1;
    asm -> mov $out, Operand::Const(0);
    asm -> cmp %t1, %t1;
    asm -> setle $out;
}

def Pat<CmpGt .., .. -> ..> int 
    map %t1, gr;
    map %t2, gr;
{
    asm -> mov %t1, $2;
    asm -> mov %t2, $1;
    asm -> mov $out, Operand::Const(0);
    asm -> cmp %t1, %t1;
    asm -> setg $out;
}

def Pat<CmpGte .., .. -> ..> int 
    map %t1, gr;
    map %t2, gr;
{
    asm -> mov %t1, $2;
    asm -> mov %t2, $1;
    asm -> mov $out, Operand::Const(0);
    asm -> cmp %t1, %t1;
    asm -> setge $out;
}

def Pat<VecInsrt fp, imm, imm -> fp> <8 x i64> 
    map %t1, gr;
{
    asm -> mov %t1, $3;
    asm -> movdqa $out, $1;
    asm -> pinsrq $out, $2;
}

def Pat<VecInsrt fp, imm, imm -> fp> <4 x i32> 
    map %t1, gr;
{
    asm -> mov %t1, $3;
    asm -> movdqa $out, $1;
    asm -> pinsrd $out, $2;
}

def Pat<VecInsrt fp, imm, imm -> fp> <8 x i16> 
    map %t1, gr;
{
    asm -> mov %t1, $3;
    asm -> movdqa $out, $1;
    asm -> pinsrw $out, $2;
}

def Pat<VecInsrt fp, imm, imm -> fp> <16 x i8> 
    map %t1, gr;
{
    asm -> mov %t1, $3;
    asm -> movdqa $out, $1;
    asm -> pinsrb $out, $2;
}

def Pat<VecInsrt fp, imm, imm -> fp> <4 x f32> 
    map %t1, gr;
{
    asm -> mov %t1, $3;
    asm -> movdqa $out, $1;
    asm -> insertps $out, $2;
}


def Pat<Shr .., .. -> gr> signed 
    map %t1, gr;
    map %t2, gr;
    overwrite Rcx;
{
    asm -> mov %t1, $2;
    asm -> mov %t2, $1;
    asm -> mov Operand::Reg(X86Reg::Rcx()), %t1;
    asm -> sar %t2, Operand::Reg(X86Reg::Rcx());
    asm -> mov $out, %t2;
}

def Pat<Shr .., .. -> gr> unsigned 
    map %t1, gr;
    map %t2, gr;
    overwrite Rcx;
{
    asm -> mov %t1, $2;
    asm -> mov %t2, $1;
    asm -> mov Operand::Reg(X86Reg::Rcx()), %t1;
    asm -> shr %t2, Operand::Reg(X86Reg::Rcx());
    asm -> mov $out, %t2;
}


def Pat<Shl .., .. -> gr> signed 
    map %t1, gr;
    map %t2, gr;
    overwrite Rcx;
{
    asm -> mov %t1, $2;
    asm -> mov %t2, $1;
    asm -> mov Operand::Reg(X86Reg::Rcx()), %t1;
    asm -> sal %t2, Operand::Reg(X86Reg::Rcx());
    asm -> mov $out, %t2;
} 

def Pat<Shl .., .. -> gr> unsigned 
    map %t1, gr;
    map %t2, gr;
    overwrite Rcx;
{
    asm -> mov %t1, $2;
    asm -> mov %t2, $1;
    asm -> mov Operand::Reg(X86Reg::Rcx()), %t1;
    asm -> shl %t2, Operand::Reg(X86Reg::Rcx());
    asm -> mov $out, %t2;
}

def Pat<Neg .. -> ..> int {
    asm -> mov $out, $1;
    asm -> neg $out;
}

def Pat<I8ToI16 .. -> ..> 
    map %t1, gr;
{
    asm -> mov %t1, $1;
    asm -> movsx $out, %t1;
}

def Pat<I8ToI32 .. -> ..> 
    map %t1, gr;
{
    asm -> mov %t1, $1;
    asm -> movsx $out, %t1;
}

def Pat<I8ToI64 .. -> ..> 
    map %t1, gr;
{
    asm -> mov %t1, $1;
    asm -> movsx $out, %t1;
}

def Pat<I16ToI8 .. -> ..> 
    map %t1, gr;
{
    asm -> mov %t1, $1;
    asm -> movsx $out, %t1;
}

def Pat<I16ToI32 .. -> ..> 
    map %t1, gr;
{
    asm -> mov %t1, $1;
    asm -> movsx $out, %t1;
}

def Pat<I16ToI64 .. -> ..> 
    map %t1, gr;
{
    asm -> mov %t1, $1;
    asm -> movsx $out, %t1;
}

def Pat<I32ToI8 .. -> ..> 
    map %t1, gr;
{
    asm -> mov %t1, $1;
    asm -> movsx $out, %t1;
}

def Pat<I32ToI16 .. -> ..> 
    map %t1, gr;
{
    asm -> mov %t1, $1;
    asm -> movsx $out, %t1;
}

def Pat<I32ToI64 .. -> ..> 
    map %t1, gr;
{
    asm -> mov %t1, $1;
    asm -> movsxd $out, %t1;
}

def Pat<I64ToI8 .. -> ..> 
    map %t1, gr;
{
    asm -> mov %t1, $1;
    asm -> movsx $out, %t1;
}

def Pat<I64ToI16 .. -> ..> 
    map %t1, gr;
{
    asm -> mov %t1, $1;
    asm -> movsx $out, %t1;
}

def Pat<I64ToI32 .. -> ..> 
    map %t1, gr;
{
    asm -> mov %t1, $1;
    asm -> movsx $out, %t1;
}

def Pat<I8ToF32 .. -> ..> 
    map %t1, gr;
    map %t2, fp;
{
    asm -> mov %t1, $1;
    asm -> cvtsi2ss %t2, %t1;
    asm -> movss $out, %t2;
}

def Pat<I16ToF32 .. -> ..> 
    map %t1, gr;
    map %t2, fp;
{
    asm -> mov %t1, $1;
    asm -> cvtsi2ss %t2, %t1;
    asm -> movss $out, %t2;
}

def Pat<I16ToF32 .. -> ..> 
    map %t1, gr;
    map %t2, fp;
{
    asm -> mov %t1, $1;
    asm -> cvtsi2ss %t2, %t1;
    asm -> movss $out, %t2;
}

def Pat<I32ToF32 .. -> ..> 
    map %t1, gr;
    map %t2, fp;
{
    asm -> mov %t1, $1;
    asm -> cvtsi2ss %t2, %t1;
    asm -> movss $out, %t2;
}

def Pat<I64ToF32 .. -> ..> 
    map %t1, gr;
    map %t2, fp;
{
    asm -> mov %t1, $1;
    asm -> cvtsi2ss %t2, %t1;
    asm -> movss $out, %t2;
}

def Pat<I8ToF64 .. -> ..> 
    map %t1, gr;
    map %t2, fp;
{
    asm -> mov %t1, $1;
    asm -> cvtsi2sd %t2, %t1;
    asm -> movsd $out, %t2;
}

def Pat<I16ToF64 .. -> ..> 
    map %t1, gr;
    map %t2, fp;
{
    asm -> mov %t1, $1;
    asm -> cvtsi2sd %t2, %t1;
    asm -> movsd $out, %t2;
}

def Pat<I32ToF64 .. -> ..> 
    map %t1, gr;
    map %t2, fp;
{
    asm -> mov %t1, $1;
    asm -> cvtsi2sd %t2, %t1;
    asm -> movsd $out, %t2;
}

def Pat<I64ToF64 .. -> ..> 
    map %t1, gr;
    map %t2, fp;
{
    asm -> mov %t1, $1;
    asm -> cvtsi2sd %t2, %t1;
    asm -> movsd $out, %t2;
}

def Pat<F32ToI8 .. -> ..> 
    map %t1, gr;
{
    asm -> cvtss2si %t1, %t1;
    asm -> mov $out, %t2;
}

def Pat<F32ToI16 .. -> ..> 
    map %t1, gr;
{
    asm -> cvtss2si %t1, %t1;
    asm -> mov $out, %t2;
}

def Pat<F32ToI32 .. -> ..> 
    map %t1, gr;
{
    asm -> cvtss2si %t1, %t1;
    asm -> mov $out, %t2;
}

def Pat<F32ToI64 .. -> ..> 
    map %t1, gr;
{
    asm -> cvtss2si %t1, %t1;
    asm -> mov $out, %t2;
}

def Pat<F64ToI8 .. -> ..> 
    map %t1, gr;
{
    asm -> cvtsd2si %t1, %t1;
    asm -> mov $out, %t2;
}

def Pat<F64ToI16 .. -> ..> 
    map %t1, gr;
{
    asm -> cvtsd2si %t1, %t1;
    asm -> mov $out, %t2;
}

def Pat<F64ToI32 .. -> ..> 
    map %t1, gr;
{
    asm -> cvtsd2si %t1, %t1;
    asm -> mov $out, %t2;
}

def Pat<F64ToI64 .. -> ..> 
    map %t1, gr;
{
    asm -> cvtsd2si %t1, %t1;
    asm -> mov $out, %t2;
}

def Pat<F32ToF64 .. -> ..> 
    map %t1, fp;
{
    asm -> cvtss2sd %t1, %t1;
    asm -> movsd $out, %t2;
}

def Pat<F64ToF32 .. -> ..> 
    map %t1, fp;
{
    asm -> cvtsd2ss %t1, %t1;
    asm -> movss $out, %t2;
}

def Pat<Div .., .. -> ..> int  
    map %t1, gr;
    hook lower_divei;
    overwrite Rax;
    overwrite Rdx;
{
    asm -> mov %t1, $2;
    asm -> mov Operand::Reg(Reg::Rax()), $1;
}