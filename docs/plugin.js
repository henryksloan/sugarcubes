async function choose_jff_file() {
     const element = document.createElement("input");
     element.type = "file";
     element.style.display = "none";
     element.type = "file";
     element.accept = ".jff";
     element.addEventListener(
         "change",
         async () => {
             if (!element.files || !element.files.length) return;
             const file = element.files[0];
             const filename = file.name;
             const content = await file.text();

             wasm_exports.open_jff_file(js_object(content));
         },
         { capture: false, once: true }
     );
     element.click();
};

async function save_jff_file(js_object) {
    if (typeof window.showSaveFilePicker == 'function') {
        // This is the "Save as..." behavior that users expect,
        // but it is currently only available on a few browsers (including Chrome)
        const opts = {
          types: [{
            description: 'JFF file',
            accept: {'text/plain': ['.jff']},
          }],
        };
        try {
            var fileHandle = await window.showSaveFilePicker(opts);
            const writable = await fileHandle.createWritable();
            await writable.write(consume_js_object(js_object));
            await writable.close();
        } catch(err) {}
    } else {
        var blob = new Blob([consume_js_object(js_object)], {type: 'text/plain'});
        saveAs(blob, "sugarcubes.jff");
    }
}

async function choose_multiple_run_file() {
     const element = document.createElement("input");
     element.type = "file";
     element.style.display = "none";
     element.addEventListener(
         "change",
         async () => {
             if (!element.files || !element.files.length) return;
             const file = element.files[0];
             const filename = file.name;
             const content = await file.text();

             wasm_exports.read_multiple_run_inputs(js_object(content));
         },
         { capture: false, once: true }
     );
     element.click();
};

register_plugin = function (importObject) {
    importObject.env.choose_multiple_run_file = choose_multiple_run_file;
    importObject.env.choose_jff_file = choose_jff_file;
    importObject.env.save_jff_file = save_jff_file;
}

miniquad_add_plugin({
    register_plugin,
    // on_init: () => set_wasm(wasm_exports),
    version: "0.0.1",
    name: "js",
});
