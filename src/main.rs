use popper_cpu::cpu::Cpu;
use popper_cpu::parser::bin_parser::BinParser;

fn main() {
    let mut cpu = Cpu::new();
    let bin = "000000010000000000000000000000000010001000100000001000000000000000000000000001000000010000000000000000000000000";
    let mut parser = BinParser::new(bin);

    let ast = parser.compile().unwrap();

    cpu.interpret(ast);

}