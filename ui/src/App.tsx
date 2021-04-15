import localForage from 'localforage';
import { useState, useEffect} from 'react';
import './App.css';
import { useForm } from 'react-hook-form';
import { DraggableData } from 'react-draggable';

import Modal from '@material-ui/core/Modal';
import Grid from '@material-ui/core/Grid';
import Divider from '@material-ui/core/Divider';

import Emulator, { Button } from './components/Emulator/Emulator';
import { EmulatorScreen } from './components/Emulator/EmulatorComponent';
import FileSubmission from './components/FileSubmission/FileSubmission';
import GamePad from './components/GamePad/GamePad';
import ResponsiveDrawer from './components/ResponsiveDrawer/ResponsiveDrawer';


function App() {
  let w: any = window;
  const { register, handleSubmit} = useForm();
  const [emulator, setEmulator] = useState(new Emulator());
  const [rom, setRom] = useState({name: "No File Selected"});
  const [modalIsOpen, setModalIsOpen] = useState(true);
  const [isDraggableDisabled, setIsDraggableDisabled] = useState(true);
  const [mute, setMute] = useState(true);
  const [controls, setControls] = useState(
    JSON.parse(localStorage.getItem('controls')) || {
    up:'w',
    left:'a',
    down:'s',
    right:'d',
    a:'j',
    b:'k',
    start:' ',
    select:'b',
  });

  const [gamePadLocations, setGamePadLocations] = useState(
    JSON.parse(localStorage.getItem('gamePadLocations')) || {
    "upButton": {x: null, y: null},
    "downButton": {x: null, y: null},
    "leftButton":{x: null, y: null},
    "rightButton": {x: null, y: null},
    "start": {x: null, y: null},
    "select": {x: null, y: null},
    "a": {x: null, y: null},
    "b": {x: null, y: null},
  })

  const onSubmit = (data: any) => {
    setModalIsOpen(false);
    setRom(data.rom[0]);
  };

  const decodeButton = (keyString: string) => {
    switch(keyString) {
      case controls.up: return Button.DUp;
      case controls.left: return Button.DLeft;
      case controls.right: return Button.DRight;
      case controls.down: return Button.DDown;
      case controls.a: return Button.A;
      case controls.b: return Button.B;
      case controls.start: return Button.Start;
      case controls.select: return Button.Select;
      default: return undefined;
    }
  };

  const handleKeyDown = (event: any) => {
    let butt = decodeButton(event.key);
    if (butt !== undefined) {
      w.button_down(butt)
    }
  }
  const handleKeyUp = (event: any) => {

    let butt = decodeButton(event.key);
    if (butt !== undefined) {
      w.button_up(butt)
    }
  }

  function sleep(ms: any) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  const handleClick = (button: Button) => {
    w.button_down(button);
    sleep(85).then(() => {w.button_up(button);});
  }

  const toggleGamePadMove = () => {
    setIsDraggableDisabled(!isDraggableDisabled);
  }

  const toggleMute = () => {
    setMute(!mute);
  }

  const [openKeyBindingModal, setOpenKeyBindingModal] = useState(false);


  const handleClose = () => {
    setOpenKeyBindingModal(false);
  };

  const handleKeyboardChange = () => {
    setOpenKeyBindingModal(true);
  }

  const submittedChanges = (data: any) => {
    let changeControls = (id: string, value: string) => {
      switch(id) {
        case "up": {controls["up"] = value; break;}
        case "down": { controls["down"] = value; break;}
        case "left": { controls["left"] = value; break;}
        case "right": { controls["right"] = value; break;}
        case "a": { controls["a"] = value; break;}
        case "b": { controls["b"] = value; break;}
        case "start": { controls["start"] = value; break;}
        case "select": { controls["select"] = value; break;}
        default: console.log("No such control exists");
      }
    }
    for(const prop in data) {
      if (data[prop].length !== 0) {
        changeControls(prop, data[prop]);
      }
    }
    setOpenKeyBindingModal(false);
  }

  const keyBindingChangeModal = (
    <Modal
    open={openKeyBindingModal}
    onClose={handleClose}
    disableEnforceFocus
    >
      <div className="modal-box">
        <h2>Set Key Bindings</h2>
        <form onSubmit={handleSubmit(submittedChanges)}>
          <input name="up" placeholder="up" ref={register}></input> <br/>
          <input name="down" placeholder="down" ref={register}></input> <br/>
          <input name="left" placeholder="left" ref={register}></input> <br/>
          <input name="right" placeholder="right" ref={register}></input> <br/>
          <input name="a" placeholder="a" ref={register}></input> <br/>
          <input name="b" placeholder="b" ref={register}></input> <br/>
          <input name="start" placeholder="start" ref={register}></input> <br/>
          <input name="select" placeholder="select" ref={register}></input> <br/>
          <button>Save</button>
        </form>
      </div>
    </Modal>
  );

  const handleGamePadChange = (id: string, data: DraggableData) => {

    switch(id) {
      case "upButton": {setGamePadLocations({...gamePadLocations, "upButton": {x: data.x, y: data.y}});
        break;
      }
      case "downButton": {setGamePadLocations({...gamePadLocations, "downButton": {x: data.x, y: data.y}});
        break;
      }
      case "leftButton": {setGamePadLocations({...gamePadLocations, "leftButton": {x: data.x, y: data.y}});
        break;
      }
      case "rightButton": {setGamePadLocations({...gamePadLocations, "rightButton": {x: data.x, y: data.y}});
        break;
      }
      case "start": {setGamePadLocations({...gamePadLocations, "start": {x: data.x, y: data.y}});
        break;
      }
      case "select": {setGamePadLocations({...gamePadLocations, "select": {x: data.x, y: data.y}});
        break;
      }
      case "a": {setGamePadLocations({...gamePadLocations, "a": {x: data.x, y: data.y}});
        break;
      }
      case "b": {setGamePadLocations({...gamePadLocations, "b": {x: data.x, y: data.y}});
        break;
      }
      default: console.log("No such button exists");
    }


  }

  const removeLocalStorage = () => {
    localStorage.clear();
    localForage.clear();
  }

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    window.addEventListener('keyup', handleKeyUp);

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('keyup', handleKeyUp);
    }
  }, [])

  useEffect(() => {
    localStorage.setItem('gamePadLocations', JSON.stringify(gamePadLocations));
    return () => {
      localStorage.setItem('gamePadLocations', JSON.stringify(gamePadLocations));
    }
  }, [gamePadLocations]);

  useEffect(() => {
    localStorage.setItem('controls', JSON.stringify(controls));
  });




  return (
    <div className="App">
      <Modal open={modalIsOpen}>
        <FileSubmission onSubmit={onSubmit}/>
      </Modal>
      <ResponsiveDrawer
        name={rom.name}
        onMute={toggleMute}
        onGamepadChange={toggleGamePadMove}
        onKeyboardChange={handleKeyboardChange}
        onDelete={removeLocalStorage}
        toggle={isDraggableDisabled}
        togglemute={mute}
        modal={keyBindingChangeModal}
      />
      <Grid
        direction="column"
        justify="center"
        alignItems="center"
      >
        <Grid item>
          <div className="screen">
          {
             (rom.constructor === File) ?
              <EmulatorScreen id={"gb-emulator"} rom={rom} />
              : <p>Waiting for ROM</p>
          }
          </div>
        </Grid>
        <Divider/>
        <Grid item>
          <GamePad
          onClick={handleClick}
          disabled={isDraggableDisabled}
          onStop={handleGamePadChange}
          locations={gamePadLocations}
        />
        </Grid>
      </Grid>
    </div>
  );
}

export default App;
