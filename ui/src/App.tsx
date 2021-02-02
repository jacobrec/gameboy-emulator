import React, { useState, useEffect } from 'react';
import logo from './logo.svg';
import './App.css';

type RustAPI = any;

function App() {
    const [rust, setRust] = useState<RustAPI | null>(null);
    useEffect(() => {
        const emulatorworker = new Worker(process.env.PUBLIC_URL + '/emulatorworker.js')
        emulatorworker.onmessage = (m) => console.log(m.data);
    });
    return (rust !== null) ? <Main rust={rust}/> : <p>Loading...</p>;
}

function Main(props: {rust: RustAPI}) {
  const [count, setCount] = useState(0);
  useEffect(() => {
      const go = async () => {
        console.log(props);
        //const update = () => setCount(props.rust.check_x());
        //setTimeout(update, 300);
      };
      go();
  });
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
