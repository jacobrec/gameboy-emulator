import React, { useState, useEffect } from 'react';
import './App.css';
import Emulator, { Button } from './Emulator';
import { EmulatorScreen } from './EmulatorComponent';
import { useForm } from 'react-hook-form';
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

function GamePad() {
  return (
    <div className="gamepad">
      <Grid 
        container
        direction="row"
        justify="space-evenly"
        alignItems="stretch"
      >
        <Grid item>
          <Grid item className="up-button">
            <UpButton className="direction-pad"/>
          </Grid>
          <Grid item className="left-button">
            <LeftButton className="direction-pad"/>
          </Grid>
          <Grid item className="right-button">
            <RightButton className="direction-pad"/>
          </Grid>
          <Grid item className="down-button">
            <DownButton className="direction-pad"/>
          </Grid>
        </Grid>

        <Grid item>
          <SelectButtonAngled className="direction-pad"/>
          <StartButtonAngled className="direction-pad"/>
        </Grid>

        <Grid item>
            <AButton className="direction-pad"/>
            <BButton className="direction-pad"/>
        </Grid>
       
      </Grid>
    </div>
  );
}

function Screen(props: any) {

  const swtch = (keyString: any) => {
    switch(keyString) {
      case props.buttons.up: return "UP";
      case props.buttons.left: return "LEFT";
      case props.buttons.right: return "RIGHT";
      case props.buttons.down: return "DOWN";
      case props.buttons.a: return "A";
      case props.buttons.b: return "B";
      case props.buttons.start: return "START";
      case props.buttons.select: return "SELECT";

      default: return " "
    }
  };

  return (
    <div className="screen">
      <h1>Key Pressed: {swtch(props.keyPressed)}</h1>
    </div>
  );
}

function ResponsiveDrawer(props: any) {
  const classes = useStyles();
  const [mobileOpen, setMobileOpen] = React.useState(false);

  const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen);
  };

  const swtch = (index: number) => {
    switch(index) {
      case 0: return <GetAppIcon/>;
      case 1: return <SaveIcon/>;
      case 2: return <SettingsIcon/>;

      default: return <h1>No project match</h1>
    }
  };

  const drawerContent = (
    <div>
      <div className={classes.toolbar} />
      <Divider />
      <List>
        {['Load', 'Save', 'Settings'].map((text, index) => (
          <ListItem button key={text}>
            <ListItemIcon>
              {swtch(index)}
            </ListItemIcon>
            <ListItemText primary={text} />
          </ListItem>
        ))}
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
  const [keyPress, setKeyPress] = useState('up');
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
    }
  };


  const handleKeyDown = (event: any) => {
    // console.log(event);
    setKeyPress(event.key);
    w.button_down(decodeButton(event.key))
  }
  const handleKeyUp = (event: any) => {
    // console.log(event);
    setKeyPress(event.key);
    w.button_up(decodeButton(event.key))
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
      <ResponsiveDrawer name={rom.name}/>
      <Grid
        direction="column"
        justify="center"
        alignItems="center"
      >
        <Grid item>
          {
             (rom.constructor === File) ?
              <EmulatorScreen id={"gb-emulator"} rom={rom} />
              : <p>Waiting for ROM</p>
          }
        </Grid>
        <Divider/>
        <Grid item>
          <GamePad />
        </Grid>
      </Grid>
    </div>
  );
}

export default App;
