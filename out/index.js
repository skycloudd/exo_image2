import init, { convert } from "./exo_image2.js";

init();

document.getElementById("generate-button").addEventListener(
    "click",
    function runConvert() {
        var f = document.getElementById("image-upload").files[0];
        var r = new FileReader();

        r.onloadend = function (e) {
            var should_resize =
                document.getElementById("resize-checkbox").checked;

            var resize_width = document.getElementById("resize-width").value;
            var resize_height = document.getElementById("resize-height").value;

            var lvl_name =
                "exoimage-" +
                new Date().toISOString().split(".")[0].replace(/[^\d]/gi, "") +
                ".exolvl";

            var result = convert(
                e.target.result,
                should_resize,
                resize_width,
                resize_height,
                lvl_name
            );

            var blob = new Blob([result], { type: "application/octet-stream" });

            var a = document.getElementById("download-a");
            a.href = window.URL.createObjectURL(blob);
            a.download = lvl_name;

            var button = document.getElementById("download-button");
            button.disabled = false;
        };

        r.readAsDataURL(f);
    },
    false
);
