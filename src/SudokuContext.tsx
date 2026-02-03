import { createContext } from 'react';

import { Page, Puzzle, Save } from './Pages';

export const SudokuContext = createContext<{
    currentPage: Page;
    setCurrentPage: (page: Page) => void;

    initialSudoku: number[][];
    setInitialSudoku: (sudoku: number[][]) => void;

    isNoteTaking: boolean;
    setIsNoteTaking: (value: boolean) => void;

    sudoku: number[][];
    setSudoku: (sudoku: number[][]) => void;

    puzzleList: Puzzle[];
    setPuzzleList: (l: Puzzle[]) => void;

    saveList: Save[];
    setSaveList: (l: Save[]) => void;
}>({
    currentPage: Page.LANDING,
    setCurrentPage: () => {},

    initialSudoku: [],
    setInitialSudoku: () => {},

    isNoteTaking: false,
    setIsNoteTaking: () => {},

    sudoku: [],
    setSudoku: () => {},

    puzzleList: [],
    setPuzzleList: () => {},

    saveList: [],
    setSaveList: () => {},
});
