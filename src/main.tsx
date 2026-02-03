import { createTheme, MantineProvider } from '@mantine/core';
import ReactDOM from 'react-dom/client';

import App from './App';

import '@mantine/core/styles.css';
import './sudoku.css';

export const theme = createTheme({});

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <MantineProvider theme={theme}>
        <App />
    </MantineProvider>,
);
