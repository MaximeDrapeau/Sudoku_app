import { Button } from '@mantine/core';
import { invoke } from '@tauri-apps/api/core';
import { useCallback, useContext } from 'react';

import { Page, Save as SaveProps } from '../Pages';
import { SudokuContext } from '../SudokuContext';

export default function Save({
    id_puzzle,
    nom,
    difficulty,
    date_sauvegarde,
}: SaveProps) {
    const { setCurrentPage, setSudoku, setInitialSudoku } =
        useContext(SudokuContext);

    const onPress = useCallback(async () => {
        const sudokuInitial: number[][] = await invoke('load_sudoku_from_db', {
            id: id_puzzle,
        });
        const sudoku: number[][] = await invoke('load_sauvegarde_from_db', {
            date: date_sauvegarde,
        });
        setSudoku(sudoku);
        setInitialSudoku(sudokuInitial);
        setCurrentPage(Page.PLAYING);
    }, [
        id_puzzle,
        date_sauvegarde,
        setCurrentPage,
        setInitialSudoku,
        setSudoku,
    ]);

    return (
        <Button onClick={onPress}>
            {nom}: {difficulty} - {date_sauvegarde}
        </Button>
    );
}
