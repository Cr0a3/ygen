use Ygen::prelude::*;

pub fn main() {
    let mut builder = Builder::new();

    let func = builder.add("func");
    func.ret(TypeMetadata::i32);

    func.push( 
        Ir::Return::new(
            Type::i32(5)
        )
    ); // Or func.BuildReturn( Type::i32(5) );

    println!("{}",
        builder.emitToColoredString()
    );
}