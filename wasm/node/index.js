import * as riscv from "lib-rv32";

document.getElementById("console").value = ""
document.getElementById("state").value = ""
document.getElementById("text").value = ""

const state = riscv.State.new();
document.getElementById("console").value = riscv.get_logs()


function assemble() {
    var program_buffer = document.getElementById("program").value;
    let program = program_buffer.replaceAll('\n', '\\n');

    state.assemble(program);

    document.getElementById("text").value = state.get_text();
    document.getElementById("console").value = riscv.get_logs()
};

function run() {
    state.run();
    document.getElementById("state").value = state.get_state();
    document.getElementById("console").value = riscv.get_logs()
}

document.getElementById("assemble").onclick = assemble;
document.getElementById("run").onclick = run;
