import React from 'react';
import { render, fireEvent } from '@testing-library/react';
import GamePad from './../GamePad';
import userEvent from '@testing-library/user-event';

describe('GamePad', () => {

    const gamePadLocations =  {
        "upButton": {x: null, y: null},
        "downButton": {x: null, y: null},
        "leftButton":{x: null, y: null},
        "rightButton": {x: null, y: null},
        "start": {x: null, y: null},
        "select": {x: null, y: null},
        "a": {x: null, y: null},
        "b": {x: null, y: null},
    };

    const mockOnClick = jest.fn();
    const mockOnStop = jest.fn((button, data) => {
        return {button, data};
    });
    
    const { container } = render(
        <GamePad 
        disabled={false}
        onClick={mockOnClick}
        onStop={mockOnStop}
        locations={gamePadLocations}
        />
    );

    const iconButtons: NodeListOf<Element> = container.querySelectorAll('.icon-button');
    const ssButtons: NodeListOf<Element> = container.querySelectorAll('.start-select-button');

    test('8 buttons render and can be clicked and dragged', () => {
        expect(iconButtons.length).toBe(6);
        expect(ssButtons.length).toBe(2);
        iconButtons.forEach((item) => {
            userEvent.click(item);
            fireEvent.drag(item);
        })
        ssButtons.forEach((item) => {
            userEvent.click(item);
            fireEvent.drag(item);
        })
        expect(mockOnClick.mock.calls.length).toBe(8);
        expect(mockOnStop.mock.calls.length).toBe(16);

    });

})