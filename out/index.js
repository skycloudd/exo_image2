import init, {
    convert,
    convert_gif,
    convert_image_pattern,
} from "./exo_image2.js";

init();

try {
    document.getElementById("generate-button").addEventListener(
        "click",
        function () {
            try {
                run_exo_image(1);
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
                run_exo_image(2);
            } catch (error) {
                alert("Error: " + error);
            }
        },
        false
    );
} catch (error) {}

try {
    document.getElementById("generate-image-pattern-button").addEventListener(
        "click",
        function () {
            try {
                run_exo_image(3);
            } catch (error) {
                alert("Error: " + error);
            }
        },
        false
    );
} catch (error) {}

function run_exo_image(type) {
    let f;

    if (type == 1 || type == 3) {
        f = document.getElementById("image-upload").files[0];
    } else if (type == 2) {
        f = document.getElementById("gif-upload").files[0];
    } else {
        throw "you should not be able to see this";
    }

    if (!f) {
        throw "No file selected";
    }

    let r = new FileReader();

    r.onloadend = function (e) {
        let lvl_name =
            (type == 1 ? "exoimage" : type == 2 ? "exogif" : "exopattern") +
            "-" +
            new Date().toISOString().split(".")[0].replace(/[^\d]/gi, "") +
            ".exolvl";

        let result;

        if (type == 1) {
            result = runConvert(e, lvl_name);
        } else if (type == 2) {
            result = runConvertGif(e, lvl_name);
        } else if (type == 3) {
            result = runConvertImagePattern(e, lvl_name);
        } else {
            throw "you should not be able to see this";
        }

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

function runConvertImagePattern(e, lvl_name) {
    return convert_image_pattern(e.target.result, lvl_name);
}
