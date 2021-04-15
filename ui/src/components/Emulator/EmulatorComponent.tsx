import { useState, useEffect } from 'react';
import { Context } from 'vm';
import Emulator from './Emulator';
import './Emulator.css';


type EmulatorProps = {
    id: string,
    rom: any,
}
export const EmulatorScreen = (props: EmulatorProps) => {
    let w: any = window;
    let [romdata, setRomData] = useState(false)
    let [emulator, setEmulator] = useState(new Emulator())
    let { id, rom } = props;



    useEffect(() => {
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
            // console.log(diff / 1000)
            let data = emulator.update();
            imd.data.set(new Uint8ClampedArray(data.buffer));
            ctx.putImageData(imd, 0, 0);
            ani = requestAnimationFrame(checker);
        };
        if (romdata) {
            ani = requestAnimationFrame(checker);
        }
        return () => {
            // window.clearInterval(inter)
            cancelAnimationFrame(ani);
        }
    })

    if (romdata) {
        console.log(w.rom)
        emulator.load_rom(w.rom)
        w.emu = emulator;
    } else if (rom.constructor === File) {
        rom.arrayBuffer().then((e) => {
            w.rom = new Uint8Array(e);
            setRomData(true);
        })
    }

    return (
        <canvas id={id} width={160} height={144}></canvas>
    )
}

