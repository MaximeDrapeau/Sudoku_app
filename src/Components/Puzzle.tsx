import { Button } from '@mantine/core';
import { invoke } from '@tauri-apps/api/core';
import { useCallback, useContext } from 'react';

import { Page, Puzzle as PuzzleProps } from '../Pages';
import { SudokuContext } from '../SudokuContext';

export default function Puzzle({ id, nom, difficulty }: PuzzleProps) {
    const { setCurrentPage, setSudoku, setInitialSudoku } =
        useContext(SudokuContext);

    const onPress = useCallback(async () => {
        const sudoku: number[][] = await invoke('load_sudoku_from_db', { id });
        setSudoku(sudoku);
        setInitialSudoku(sudoku);
        setCurrentPage(Page.PLAYING);
    }, [id, setCurrentPage, setInitialSudoku, setSudoku]);

    return (
        <Button onClick={onPress}>
            {nom}: {difficulty}
        </Button>
    );
}
