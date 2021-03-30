import React, { useState, useEffect, useRef } from 'react';
import './App.css';
import Emulator from './Emulator';
import { useForm } from 'react-hook-form';
import Modal from 'react-modal';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import Button from '@material-ui/core/Button';
import IconButton from '@material-ui/core/IconButton';
import MenuIcon from '@material-ui/icons/Menu';
import { makeStyles } from '@material-ui/core/styles';

const useStyles = makeStyles((theme) => ({
  root: {
    flexGrow: 1,
  },
  menuButton: {
    marginRight: theme.spacing(2),
  },
  title: {
    flexGrow: 1,
  },
}));


function ButtonAppBar(props: any) {
  const classes = useStyles();

  return (
    <div className={classes.root}>
      <AppBar position="static">
        <Toolbar>
          <IconButton edge="start" className={classes.menuButton} color="inherit" aria-label="menu">
            <MenuIcon />
          </IconButton>
          <Typography variant="h6" className={classes.title}>
            {props.name}
          </Typography>
        </Toolbar>
      </AppBar>
    </div>
  );
}

function FileSubmission(props: any) {
  const { register, handleSubmit, errors } = useForm(); 

  return (
    <Modal
      isOpen={props.isOpen}
    >
      <h2>Load a Game</h2>
      <form onSubmit={handleSubmit(props.onSubmit)} className="form">
        <input required type="file" name="rom" ref={register}/>
        <button>Submit</button>
      </form>
    </Modal>
  );
}

function App() {
  const [count, setCount] = useState(0);
  const [emulator, setEmulator] = useState(new Emulator());
  const [intervals, setIntervals] = useState(false);
  const [rom, setRom] = useState({name: "No File Selected"});

  const [modalIsOpen, setModalIsOpen] = useState(true);

  const onSubmit = (data: any) => {
    console.log(data.rom[0]);
    setModalIsOpen(false);
    setRom(data.rom[0]);
  };



  if (!intervals) {
    setIntervals(true);
    setInterval(() => {
      emulator.update();
    }, 10);
    const checker = () => {
      setCount(emulator.get_screen())
      requestAnimationFrame(checker);
    };
    requestAnimationFrame(checker);
  }

  return (
    <div className="App">
      <ButtonAppBar name={rom.name}/>
      <FileSubmission onSubmit={onSubmit} isOpen={modalIsOpen}/>
      <div>{/* Display screen */}</div>
      <div>{/* modal for settings/other options possibly modal on modile and dashboard otherwise*/}</div>
      <div>{/* controls */}</div>
    </div>
  );
}

export default App;
