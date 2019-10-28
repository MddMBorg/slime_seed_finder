importScripts("wasm_gui.js");

// Simulate lengthy calculation or an async call
function doCalculation(wasmgui, data, cb) {
    let err = null;
    console.log("Message received from main script");
    console.log(data);
    let fx = data.fx;
    let fy = data.fy;
    let seed = data.seed;
    let FRAG_SIZE = data.FRAG_SIZE;
    let rvec = wasmgui.generate_fragment_slime_map(fx, fy, seed, FRAG_SIZE);
    let result = { rvec: rvec };
    cb(err, result);
}

// Handle incoming messages
self.onmessage = function(msg) {
    const { id, payload } = msg.data;

    Rust.wasm_gui.then(function(wasmgui) {
        doCalculation(wasmgui, payload, function(err, result) {
            const msg = {
                id,
                err,
                payload: result,
            };
            self.postMessage(msg);
        });
    });
};
