import {DataType, define, open} from "ffi-rs";
import path from 'path';
import { suffix } from "bun:ffi";

const libPath = path.join(process.cwd(), `./engine/target/debug/engine.${suffix}`);

const library_name = "bwin";
open({ library: library_name, path: libPath });

export interface WindowConfig{
    title: string,
    width?: number
    height?: number
}

const native = define({
    start: {
        library: library_name,
        paramsType: [ DataType.String, DataType.I32, DataType.I32 ],
        retType: DataType.Void
    }
});

export const start = (config: WindowConfig) =>{
    native.start([config.title, config.width ?? -1, config.height ?? -1]);
}