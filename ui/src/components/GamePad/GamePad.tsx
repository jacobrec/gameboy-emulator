
import './GamePad.css';
import { Button } from '../Emulator/Emulator';
import Draggable from 'react-draggable';


//Icon imports
import DownButton from './../../iconComponents/DownButton';
import UpButton from './../../iconComponents/UpButton';
import LeftButton from './../../iconComponents/LeftButton';
import RightButton from './../../iconComponents/RightButton';
import SelectButtonAngled from './../../iconComponents/SelectButtonAngled';
import StartButtonAngled from './../../iconComponents/StartButtonAngled';
import AButton from './../../iconComponents/AButton';
import BButton from './../../iconComponents/BButton';

function GamePad(props: any) {

    return (
      <div className="gamepad">
        <Draggable
        disabled={props.disabled}
        grid={[10,10]}
        onStop={(e, data) => props.onStop("upButton", data)}
        defaultPosition={{x: props.locations["upButton"].x, y: props.locations["upButton"].y}}
        >
          <div className="up">
            <UpButton className="icon-button" onClick={() => props.onClick(Button.DUp)} />
          </div>
        </Draggable>
  
        <Draggable
        disabled={props.disabled}
        grid={[10,10]}
        onStop={(e, data) => props.onStop("leftButton",data)}
        defaultPosition={{x: props.locations["leftButton"].x, y: props.locations["leftButton"].y}}
        >
          <div className="left">
            <LeftButton className="icon-button" onClick={() => props.onClick(Button.DLeft)}/>
          </div>
        </Draggable>
  
        <Draggable
        disabled={props.disabled}
        grid={[10,10]}
        onStop={(e, data) => props.onStop("rightButton",data)}
        defaultPosition={{x: props.locations["rightButton"].x, y: props.locations["rightButton"].y}}
        >
          <div className="right">
            <RightButton className="icon-button" onClick={() => props.onClick(Button.DRight)}/>
          </div>
        </Draggable>
  
        <Draggable
        disabled={props.disabled}
        grid={[10,10]}
        onStop={(e, data) => props.onStop("downButton",data)}
        defaultPosition={{x: props.locations["downButton"].x, y: props.locations["downButton"].y}}
        >
          <div className="down">
            <DownButton className="icon-button" onClick={() => props.onClick(Button.DDown)}/>
          </div>
        </Draggable>
  
        <Draggable
        disabled={props.disabled}
        grid={[10,10]}
        onStop={(e, data) => props.onStop("select",data)}
        defaultPosition={{x: props.locations["select"].x, y: props.locations["select"].y}}
        >
          <div className="select">
            <SelectButtonAngled className="start-select-button" onClick={() => props.onClick(Button.Select)}/>
          </div>
        </Draggable>
  
        <Draggable
        disabled={props.disabled}
        grid={[10,10]}
        onStop={(e, data) => props.onStop("start",data)}
        defaultPosition={{x: props.locations["start"].x, y: props.locations["start"].y}}
        >
          <div className="start">
            <StartButtonAngled  className="start-select-button" onClick={() => props.onClick(Button.Start)}/>
          </div>
        </Draggable>
  
        <Draggable
        disabled={props.disabled}
        grid={[10,10]}
        onStop={(e, data) => props.onStop("a",data)}
        defaultPosition={{x: props.locations["a"].x, y: props.locations["a"].y}}
        >
          <div className="a-button">
            <AButton className="icon-button" onClick={() => props.onClick(Button.A)}/>
          </div>
        </Draggable>
  
        <Draggable
        disabled={props.disabled}
        grid={[10,10]}
        onStop={(e, data) => props.onStop("b",data)}
        defaultPosition={{x: props.locations["b"].x, y: props.locations["b"].y}}
        >
          <div className="b-button">
            <BButton className="icon-button" onClick={() => props.onClick(Button.B)}/>
          </div>
        </Draggable>
  
      </div>
    );
}


export default GamePad;