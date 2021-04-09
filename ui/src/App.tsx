import React, { useState, useEffect } from 'react';
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
import FormControlLabel from '@material-ui/core/FormControlLabel';
import Switch from '@material-ui/core/Switch';

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

  return (
    <div className="gamepad">
      <Draggable disabled={props.disabled} grid={[10,10]}>
        <UpButton className="icon-button up" onClick={() => props.onClick(Button.DUp)}/>
      </Draggable>

      <Draggable disabled={props.disabled} grid={[10,10]}>
        <LeftButton className="icon-button left" onClick={() => props.onClick(Button.DLeft)}/>
      </Draggable>

      <Draggable disabled={props.disabled} grid={[10,10]}>
        <RightButton className="icon-button right" onClick={() => props.onClick(Button.DRight)}/>
      </Draggable>

      <Draggable disabled={props.disabled} grid={[10,10]}>
        <DownButton className="icon-button down" onClick={() => props.onClick(Button.DDown)}/>
      </Draggable>

      <Draggable disabled={props.disabled} grid={[10,10]}>
        <SelectButtonAngled className="select" onClick={() => props.onClick(Button.Select)}/>
      </Draggable>
      <Draggable disabled={props.disabled} grid={[10,10]}>
        <StartButtonAngled className="start" onClick={() => props.onClick(Button.Start)}/>
      </Draggable>

      <Draggable disabled={props.disabled} grid={[10,10]}>
        <AButton className="icon-button a-button" onClick={() => props.onClick(Button.A)}/>
      </Draggable>
      <Draggable disabled={props.disabled} grid={[10,10]}>
        <BButton className="icon-button b-button" onClick={() => props.onClick(Button.B)}/>
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
        <MenuItem text={"Configure GamePad"}>
          <VideogameAssetIcon/>
          <Switch 
            checked={props.toggle}
            onChange={props.onGamepadChange}
          />
        </MenuItem>
        <MenuItem onClick={() => props.onKeyboardChange()} text={"Configure Keyboard"}><KeyboardIcon/></MenuItem>
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
  const [emulator, setEmulator] = useState(new Emulator());
  const [rom, setRom] = useState({name: "No File Selected"});
  const [modalIsOpen, setModalIsOpen] = useState(true);
  const [isDraggableDisabled, setIsDraggableDisabled] = useState(true);
  const [controls, setControls] = useState({
    up:'w',
    left:'a',
    down:'s',
    right:'d',
    a:'j',
    b:'k',
    start:' ',
    select:'b',
  });

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
    // console.log(event);
    // setKeyPress(event.key);
    let butt = decodeButton(event.key);
    if (butt !== undefined) {
        w.button_down(butt)
    }
  }
  const handleKeyUp = (event: any) => {
    // console.log(event);
    // setKeyPress(event.key);
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

  const handleGamepadChange = () => {
    setIsDraggableDisabled(!isDraggableDisabled);
    //needs to save gamepads current positions and save to JSOn file 
  }
  const handleKeyboardChange = () => {
    //need to get keybindings and set controls JSON
    //may need to open modal and save form input 
  }
  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    window.addEventListener('keyup', handleKeyUp);

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('keyup', handleKeyUp);
    }
  }, [])



  return (
    <div className="App">
      <Modal open={modalIsOpen}>
        <FileSubmission onSubmit={onSubmit}/>
      </Modal>
      <ResponsiveDrawer 
        name={rom.name} 
        onGamepadChange={handleGamepadChange} 
        onKeyboardChange={handleKeyboardChange} 
        toggle={isDraggableDisabled}
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
          <GamePad onClick={handleClick} disabled={isDraggableDisabled}/>
        </Grid>
      </Grid>
    </div>
  );
}

export default App;
