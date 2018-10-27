//Pass in a WebAssembly instance as the first parameter.
//This will create (or add to) a global WebAssembly object containing all the functions exported by the instance.
//This function automatically removes C++ debug markers. If you do not want it to, pass in true as the second parameter.

function loadexports(instance, keepdebugmarkers) {
    if (!window.wasm) {
    window.wasm = {}
    } 
    for (let key in instance.exports) {
        let item = instance.exports[key]
        //We don't want any non-functions like the memory object
        if (typeof item === "function") {
            let name = key
            //Check for C++ Debug Markers, and Remove Them if Needed
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
