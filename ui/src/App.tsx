import React, { useState, useEffect } from 'react';
import logo from './logo.svg';
import './App.css';
import Emulator from './Emulator'

function App() {
  const [count, setCount] = useState(0);
  const [emulator, setEmulator] = useState(new Emulator());
  const [intervals, setIntervals] = useState(false);

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
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p> Hello World </p>
        <p> Rust count is at: {count} </p>
      </header>
    </div>
  );
}

export default App;
