import { fs } from '@dev.sinpy/velox.js';

window.test = function test() {
    fs.openDialog(true).then((val) => console.log(val))
        .catch((err) => console.log(err));
}