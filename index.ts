//import {DataType, define, open} from "ffi-rs";
import path from 'path';
import { suffix } from "bun:ffi";
import { dlopen, FFIType } from "bun:ffi";
//import { bench, run } from 'mitata';

const libPath = path.join(process.cwd(), `./engine/target/debug/engine.${suffix}`);

const lib = dlopen(libPath, {
    add: {
        args: [FFIType.i32, FFIType.i32],
        returns: FFIType.i32,
    },
    start: {
        args: [],
        returns: FFIType.void
    }
});

console.log(lib.symbols.add(4, 13))
lib.symbols.start()


//bench("calling native code", () => lib.symbols.add(4, 13));
//await run();

/*const library_name = "bwin";

open({ library: library_name, path: libPath });

const { add, start } = define({
    add: {
        library: library_name,
        paramsType: [
            DataType.I32,
            DataType.I32
        ],
        retType: DataType.I32,
    },
    start: {
        library: library_name,
        paramsType: [],
        retType: DataType.Void
    }
});*/

//bench("calling native code", () => add([4, 13]));
//await run();
