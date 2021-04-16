import React from 'react';
import { render, screen} from '@testing-library/react';
import '@testing-library/jest-dom/extend-expect'
import FileSubmission from './../FileSubmission';
import userEvent from '@testing-library/user-event';

describe('File Submission', () => {

    test('Renders title and input', () => {
    
        render(<FileSubmission/>);
        
        expect(screen.getByText('Load a Game')).toBeInTheDocument();
        expect(screen.getByRole('button')).toBeInTheDocument();
        expect(screen.getByTestId('input')).toBeInTheDocument();
        expect(screen.getByTestId('input')).toBeRequired();
    })

    test('Uploads file', () => {
        const file = new File([' '], 'mock.gb')
        render(<FileSubmission/>);

        const input = screen.getByTestId('input');
        userEvent.upload(input, file)
        
        
        expect(input.files[0]).toStrictEqual(file)
        expect(input.files).toHaveLength(1)
    })
})