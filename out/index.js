import init, { convert, convert_gif } from "./exo_image2.js";

init();

try {
    document.getElementById("generate-button").addEventListener(
        "click",
        function () {
            try {
                run_exo_image(false);
            } catch (error) {
                alert("Error: " + error);
            }
        },
        false
    );
} catch (error) {}

try {
    document.getElementById("generate-gif-button").addEventListener(
        "click",
        function () {
            try {
                run_exo_image(true);
            } catch (error) {
                alert("Error: " + error);
            }
        },
        false
    );
} catch (error) {}

function run_exo_image(gif) {
    let f = document.getElementById("image-upload").files[0];

    if (!f) {
        throw "No file selected";
    }

    let r = new FileReader();

    r.onloadend = function (e) {
        let lvl_name =
            (gif ? "exogif" : "exoimage") +
            "-" +
            new Date().toISOString().split(".")[0].replace(/[^\d]/gi, "") +
            ".exolvl";

        let result = gif ? runConvertGif(e, lvl_name) : runConvert(e, lvl_name);

        let blob = new Blob([result], { type: "application/octet-stream" });

        let a = document.getElementById("download-a");
        a.href = window.URL.createObjectURL(blob);
        a.download = lvl_name;

        let button = document.getElementById("download-button");

        button.disabled = false;
    };

    r.readAsDataURL(f);
}

function runConvert(e, lvl_name) {
    let should_resize = document.getElementById("resize-checkbox").checked;

    let resize_width = document.getElementById("resize-width").value;
    let resize_height = document.getElementById("resize-height").value;

    return convert(
        e.target.result,
        should_resize,
        resize_width,
        resize_height,
        lvl_name
    );
}

function runConvertGif(e, lvl_name) {
    return convert_gif(e.target.result, lvl_name);
}
