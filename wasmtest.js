

var wasmModule = new WebAssembly.Module(wasmCode);
var wasmInstance = new WebAssembly.Instance(wasmModule, wasmImports);
//log(wasmInstance.exports.getChar(0));

wasmInstance.exports.memory = new WebAssembly.Memory({initial:20});//This line is useless... Huh

const memory = wasmInstance.exports.memory;
//const strBuf = new Uint8Array(memory.buffer, wasmInstance.exports.getStrOffset());
	
	

wasmInstance.exports.memory.grow(9)
log(wasmInstance.exports.memory.buffer.byteLength)
	
	
console.log(memory)
	
var buflen = 100000
	
const strBuf = new Uint8Array(memory.buffer, wasmInstance.exports.getbuf(buflen), buflen);

log(strBuf)
