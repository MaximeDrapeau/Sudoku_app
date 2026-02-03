import { Container } from '@mantine/core';
import { useState } from 'react';

import { ColorSchemeToggle } from './Components/ColorSchemeToggle';
import { Page, Puzzle, Save } from './Pages';
import DifficultyMenu from './Pages/DifficultyMenu';
import ImportPage from './Pages/ImportPage';
import LandingPage from './Pages/LandingPage';
import MainMenu from './Pages/MainMenu';
import PlayChoice from './Pages/PlayChoice';
import PlayingPage from './Pages/PlayingPage';
import PuzzleList from './Pages/PuzzleList';
import SaveList from './Pages/SaveList';
import { SudokuContext } from './SudokuContext';

const INIT_ROWS = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
];

export default function App() {
    const [sudoku, setSudoku] = useState<number[][]>(INIT_ROWS);
    const [initialSudoku, setInitialSudoku] = useState<number[][]>(INIT_ROWS);
    const [currentPage, setCurrentPage] = useState<Page>(Page.LANDING);
    const [isNoteTaking, setIsNoteTaking] = useState(false);
    const [puzzleList, setPuzzleList] = useState<Puzzle[]>([]);
    const [saveList, setSaveList] = useState<Save[]>([]);

    return (
        <SudokuContext.Provider
            value={{
                currentPage,
                setCurrentPage,

                initialSudoku,
                setInitialSudoku,

                isNoteTaking,
                setIsNoteTaking,

                sudoku,
                setSudoku,

                puzzleList,
                setPuzzleList,

                saveList,
                setSaveList,
            }}
        >
            <Container h="100vh" style={{ overflow: 'hidden' }} fluid>
                {currentPage === Page.LANDING && (
                    <LandingPage
                        onContinue={() => setCurrentPage(Page.MAIN_MENU)}
                    />
                )}
                {currentPage === Page.MAIN_MENU && <MainMenu />}
                {currentPage === Page.PLAYING && <PlayingPage />}
                {currentPage === Page.DIFF_MENU && <DifficultyMenu />}
                {currentPage === Page.IMPORT && <ImportPage />}
                {currentPage === Page.PLAY_CHOICE && <PlayChoice />}
                {currentPage === Page.PUZZLE_LIST && <PuzzleList />}
                {currentPage === Page.SAVE_LIST && <SaveList />}

                <ColorSchemeToggle pos="absolute" left={5} bottom={5} />
            </Container>
        </SudokuContext.Provider>
    );
}
