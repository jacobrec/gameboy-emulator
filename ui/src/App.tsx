import React, { useState, useEffect, useRef } from 'react';
import './App.css';
import Emulator, { Button } from './Emulator';
import { EmulatorScreen } from './EmulatorComponent';
import { useForm } from 'react-hook-form';
import Draggable from 'react-draggable';

//Material UI imports
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import IconButton from '@material-ui/core/IconButton';
import MenuIcon from '@material-ui/icons/Menu';
import { makeStyles, Theme, createStyles } from '@material-ui/core/styles';
import Modal from '@material-ui/core/Modal';
import Grid from '@material-ui/core/Grid';
import Divider from '@material-ui/core/Divider';
import Drawer from '@material-ui/core/Drawer';
import Hidden from '@material-ui/core/Hidden';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import ListItemIcon from '@material-ui/core/ListItemIcon';
import ListItemText from '@material-ui/core/ListItemText';
import CssBaseline from '@material-ui/core/CssBaseline';
import SaveIcon from '@material-ui/icons/Save';
import SettingsIcon from '@material-ui/icons/Settings';
import GetAppIcon from '@material-ui/icons/GetApp';
import VideogameAssetIcon from '@material-ui/icons/VideogameAsset';
import KeyboardIcon from '@material-ui/icons/Keyboard';
import Switch from '@material-ui/core/Switch';
import DeleteIcon from '@material-ui/icons/Delete';

//Icon imports
import DownButton from './iconComponents/DownButton';
import UpButton from './iconComponents/UpButton';
import LeftButton from './iconComponents/LeftButton';
import RightButton from './iconComponents/RightButton';
import SelectButtonAngled from './iconComponents/SelectButtonAngled';
import StartButtonAngled from './iconComponents/StartButtonAngled';
import AButton from './iconComponents/AButton';
import BButton from './iconComponents/BButton';

const drawerWidth = 240;

interface ControlObj {
  up: string,
  left: string,
  down:string,
  right:string,
  a:string,
  b:string,
  start:string,
  select:string
}

function parse_json(json: any): json is ControlObj {
  return json;
}

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      display: 'flex',
    },
    drawer: {
      [theme.breakpoints.up('sm')]: {
        width: drawerWidth,
        flexShrink: 0,
      },
    },
    appBar: {
      [theme.breakpoints.up('sm')]: {
        zIndex: theme.zIndex.drawer + 1,
      },
    },
    menuButton: {
      marginRight: theme.spacing(2),
      [theme.breakpoints.up('sm')]: {
        display: 'none',
      },
    },
    // necessary for content to be below app bar
    toolbar: theme.mixins.toolbar,
    drawerPaper: {
      width: drawerWidth,
    },
    content: {
      flexGrow: 1,
      padding: theme.spacing(3),
    },
  }),
);

function FileSubmission(props: any) {
  const { register, handleSubmit, errors } = useForm();

  return (
    <div className="modal-box">
      <div className="modal-text">
        <h2>Load a Game</h2>
        <form onSubmit={handleSubmit(props.onSubmit)} className="form">
          <input required type="file" name="rom" ref={register}/>
          <button>Submit</button>
        </form>
      </div>
    </div>
  );
}

function GamePad(props: any) {

  const upButton = useRef(null);
  const leftButton = useRef(null);
  const rightButton = useRef(null);
  const downButton = useRef(null);
  const startButton = useRef(null);
  const selectButton = useRef(null);
  const aButton = useRef(null);
  const bButton = useRef(null);

  

  return (
    <div className="gamepad">
      <Draggable disabled={props.disabled} grid={[10,10]} onStop={() => props.onStop("upButton",upButton.current)}>
        <div 
        className="up" 
        ref={upButton} 
        >
          <UpButton className="icon-button" onClick={() => props.onClick(Button.DUp)} />
        </div>
      </Draggable>

      <Draggable disabled={props.disabled} grid={[10,10]} onStop={() => props.onStop("leftButton",leftButton.current)}>
        <div className="left" ref={leftButton}>
          <LeftButton className="icon-button" onClick={() => props.onClick(Button.DLeft)}/>
        </div>
      </Draggable>

      <Draggable disabled={props.disabled} grid={[10,10]} onStop={() => props.onStop("rightButton",rightButton.current)}>
        <div className="right" ref={rightButton}>
          <RightButton className="icon-button" onClick={() => props.onClick(Button.DRight)}/>
        </div>
      </Draggable>

      <Draggable disabled={props.disabled} grid={[10,10]} onStop={() => props.onStop("downButton",downButton.current)}>
        <div className="down" ref={downButton}>
          <DownButton className="icon-button" onClick={() => props.onClick(Button.DDown)}/>
        </div>
      </Draggable>

      <Draggable disabled={props.disabled} grid={[10,10]} onStop={() => props.onStop("select",selectButton.current)}>
        <div className="select" ref={selectButton}>
          <SelectButtonAngled className="start-select-button" onClick={() => props.onClick(Button.Select)}/>
        </div>
      </Draggable>

      <Draggable disabled={props.disabled} grid={[10,10]} onStop={() => props.onStop("start",startButton.current)}>
        <div className="start" ref={startButton}>
          <StartButtonAngled  className="start-select-button" onClick={() => props.onClick(Button.Start)}/>
        </div>
      </Draggable>

      <Draggable disabled={props.disabled} grid={[10,10]} onStop={() => props.onStop("a",aButton.current)}>
        <div className="a-button" ref={aButton}>
          <AButton className="icon-button" onClick={() => props.onClick(Button.A)}/>
        </div>
      </Draggable>

      <Draggable disabled={props.disabled} grid={[10,10]} onStop={() => props.onStop("b",bButton.current)}>
        <div className="b-button" ref={bButton}>
          <BButton className="icon-button" onClick={() => props.onClick(Button.B)}/>
        </div>
      </Draggable>

    </div>
  );
}

function MenuItem (props: any) {
  let {text, onClick} = props;
  return (
      <ListItem onClick={onClick} button key={text}>
        <ListItemIcon>
          {props.children}
        </ListItemIcon>
        <ListItemText primary={text} />
      </ListItem>

  )
}

function ResponsiveDrawer(props: any) {
  const classes = useStyles();
  const [mobileOpen, setMobileOpen] = React.useState(false);
  
  const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen);
  };

  let w: any = window;
  const drawerContent = (
    <div>
      <div className={classes.toolbar} />
      <Divider />
      <List>
        <MenuItem onClick={() => w.emu.load_save_state()} text={"Load"}><GetAppIcon/></MenuItem>
        <MenuItem onClick={() => w.emu.make_save_state()} text={"Save"}><SaveIcon/></MenuItem>
        <MenuItem text={"Settings"}><SettingsIcon/></MenuItem>
        <ListItem button key={"Configure GamePad"}>
          <ListItemIcon>
            <VideogameAssetIcon/>
          </ListItemIcon>
          <ListItemText primary={"Configure GamePad"} />
          <Switch 
            checked={!props.toggle}
            color="default"
            onChange={props.onGamepadChange}
          />
        </ListItem>
        <MenuItem onClick={props.onKeyboardChange} text={"Configure Keyboard"}><KeyboardIcon/> {props.modal} </MenuItem>
        <MenuItem text={"Delete User Data"} onClick={props.onDelete}><DeleteIcon/></MenuItem>
      </List>
    </div>
  );

  return (
    <div className={classes.root}>
      <CssBaseline />
      <AppBar position="fixed" className={classes.appBar}>
        <Toolbar>
          <IconButton
            color="inherit"
            aria-label="open drawer"
            edge="start"
            onClick={handleDrawerToggle}
            className={classes.menuButton}
          >
            <MenuIcon />
          </IconButton>
          <Typography variant="h6" noWrap>
            {props.name}
          </Typography>
        </Toolbar>
      </AppBar>
      <nav className={classes.drawer} aria-label="mailbox folders">
        <Hidden smUp implementation="css">
          <Drawer
            variant="temporary"
            anchor='left'
            open={mobileOpen}
            onClose={handleDrawerToggle}
            classes={{
              paper: classes.drawerPaper,
            }}
            ModalProps={{
              keepMounted: true, // Better open performance on mobile.
            }}
          >
            {drawerContent}
          </Drawer>
        </Hidden>
        <Hidden xsDown implementation="css">
          <Drawer
            classes={{
              paper: classes.drawerPaper,
            }}
            variant="permanent"
            open
          >
            {drawerContent}
          </Drawer>
        </Hidden>
      </nav>
      <main className={classes.content}>
      </main>
    </div>
  );
}

function App() {
  let w: any = window;
  const { register, handleSubmit} = useForm();
  const [emulator, setEmulator] = useState(new Emulator());
  const [rom, setRom] = useState({name: "No File Selected"});
  const [modalIsOpen, setModalIsOpen] = useState(true);
  const [isDraggableDisabled, setIsDraggableDisabled] = useState(true);
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
    "upButton": {},
    "downButton": {},
    "leftButton": {},
    "rightButton": {},
    "start": {},
    "select": {},
    "a": {},
    "b": {},
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
    // console.log(button);
    w.button_down(button);
    sleep(85).then(() => {w.button_up(button);});
  }

  const toggleGamePadMove = () => {
    setIsDraggableDisabled(!isDraggableDisabled);
  }

  const [openKeyBindingModal, setOpenKeyBindingModal] = useState(false);


  const handleClose = () => {
    setOpenKeyBindingModal(false);
  };

  const handleKeyboardChange = () => {
    //need to get keybindings and set controls JSON
    //may need to open modal and save form input 
    setOpenKeyBindingModal(true);
  }

  const submittedChanges = (data: any) => {
    let changeControls = (id: string, value: string) => {
      switch(id) {
        case "up": { setControls({...controls, "up": value}); break;}
        case "down": { setControls({...controls, "down": value}); break;}
        case "left": { setControls({...controls, "left": value}); break;}
        case "right": { setControls({...controls, "right": value}); break;}
        case "a": { setControls({...controls, "a": value}); break;}
        case "b": { setControls({...controls, "b": value}); break;}
        case "start": { setControls({...controls, "start": value}); break;}
        case "select": { setControls({...controls, "select": value}); break;}
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

  const handleGamePadChange = (id: string, el: HTMLDivElement) => {

    if (el) {
      switch(id) {
        case "upButton": {
          gamePadLocations["upButton"] = {
            offsetTop: el.offsetTop,
            offsetLeft: el.offsetLeft,
          };
          break;
        }
        case "downButton": {
          gamePadLocations["downButton"] = el.getBoundingClientRect();
          break;
        }
        case "leftButton": {
          gamePadLocations["leftButton"] = el.getBoundingClientRect();
          break;
        }
        case "rightButton": {
          gamePadLocations["rightButton"] = el.getBoundingClientRect();
          break;
        }
        case "start": {
          gamePadLocations["start"] = el.getBoundingClientRect();
          break;
        }
        case "select": {
          gamePadLocations["select"] = el.getBoundingClientRect();
          break;
        }
        case "a": {
          gamePadLocations["a"] = el.getBoundingClientRect();
          break;
        }
        case "b": {
          gamePadLocations["b"] = el.getBoundingClientRect();
          break;
        }
        default: console.log("No such button exists");
      }

      console.log(el.offsetParent);
    }

  }

  const removeLocalStorage = () => {
    localStorage.clear();
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
  }, [gamePadLocations]);

  useEffect(() => {
    localStorage.setItem('controls', JSON.stringify(controls));
  }, [controls]);

  


  return (
    <div className="App">
      <Modal open={modalIsOpen}>
        <FileSubmission onSubmit={onSubmit}/>
      </Modal>
      <ResponsiveDrawer 
        name={rom.name} 
        onGamepadChange={toggleGamePadMove} 
        onKeyboardChange={handleKeyboardChange} 
        onDelete={removeLocalStorage}
        toggle={isDraggableDisabled}
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
