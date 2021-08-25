import * as riscv from "lib-rv32";

function console_log(msg) {
    console.log(msg);
}

const state = riscv.State.new();

function assemble() {
    var program_buffer = document.getElementById("program").value;
    let program = program_buffer.replaceAll('\n', '\\n');

    state.assemble(program);

    document.getElementById("text").value = state.get_text();
};

function run() {
    state.run();
    document.getElementById("state").value = state.get_state();
}

document.getElementById("assemble").onclick = assemble;
document.getElementById("run").onclick = run;
