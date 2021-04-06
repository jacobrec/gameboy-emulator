import React, { useState, useEffect } from 'react';
import { Context } from 'vm';
import Emulator, { Button } from './Emulator';


type EmulatorProps = {
    id: string,
    callbackRegister: (down: (key: Button) => void, up: (key: Button) => void) => void,
    rom: any,
}
export const EmulatorScreen = (props: EmulatorProps) => {
    let w: any = window;
    let [romdata, setRomData] = useState(new Uint8Array())
    let [emulator, setEmulator] = useState(new Emulator())
    let { id, callbackRegister, rom } = props;
    let pressDown = (key: Button) => {}
    let pressUp = (key: Button) => {}


    if (rom.constructor === File && romdata.length == 0) {
        let starter = async () => {
            let ab = await rom.arrayBuffer();
            setRomData(new Uint8Array(ab));
        };
        starter();
    }

    useEffect(() => {
        // let inter = setInterval(() => {emulator.update();}, 10);
        let d: Document = document;
        let c: HTMLElement | null = d.getElementById(id) as HTMLCanvasElement;
        let ctx: Context = null;
        if (c !== null) {
            let ca: HTMLCanvasElement = c as HTMLCanvasElement;
            ctx = ca.getContext('2d');
        }
        let imd = new ImageData(160, 144);
        let ani = 0;
        let lt = 0;
        const checker = (time: number) => {
            let diff = time - lt;
            lt = time;
            console.log(diff / 1000)
            let data = emulator.update();
            imd.data.set(new Uint8ClampedArray(data.buffer));
            ctx.putImageData(imd, 0, 0);
            w.data = data;
            ani = requestAnimationFrame(checker);
        };
        ani = requestAnimationFrame(checker);
        return () => {
            // window.clearInterval(inter)
            cancelAnimationFrame(ani);
        }
    })

    if (romdata.length > 0) {
        console.log(romdata)
        emulator.load_rom(romdata)
    }

    callbackRegister(pressDown, pressUp);
    return (
        <canvas id={id} width={160} height={144}></canvas>
    )
}