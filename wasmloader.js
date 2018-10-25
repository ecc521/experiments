function loadexports(instance, keepdebugmarkers) {
    if (!window.wasm) {
    window.wasm = {}
    } 
    for (let key in instance.exports) {
        let item = instance.exports[key]
        if (typeof item === "function") {
            let name = key
            //Check for C++ Debug Markers, and Remove Them
            if (key.slice(0,3) === "_Z7" && key.slice(-1) === "i" && !keepdebugmarkers) {
                name = key.slice(3, key.length-1)  
            }
            if (window.wasm[name]) {
                console.warn("Overwriting WebAssembly Module Named " + name)    
            }
            window.wasm[name] = item   
        } 
    }  
}
