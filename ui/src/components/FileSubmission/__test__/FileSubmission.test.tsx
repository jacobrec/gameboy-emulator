import React from 'react';
import { render, screen } from '@testing-library/react';
import FileSubmission from './../FileSubmission';

import { unmountComponentAtNode } from "react-dom";

let container: HTMLDivElement = null;
beforeEach(() => {
  // setup a DOM element as a render target
  container = document.createElement("div");
  document.body.appendChild(container);
});

afterEach(() => {
  // cleanup on exiting
  unmountComponentAtNode(container);
  container.remove();
  container = null;
});