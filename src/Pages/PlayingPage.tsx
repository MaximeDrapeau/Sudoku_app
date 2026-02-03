import { ActionIcon, Center, Flex, Table, Tooltip } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import {
    IconArrowLeft,
    IconBulb,
    IconDeviceFloppy,
    IconKey,
    IconPencil,
    IconPencilOff,
    IconRestore,
} from '@tabler/icons-react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useCallback, useContext, useEffect, useState } from 'react';

import { Page } from '.';
import SudokuCell from '../Components/SudokuCell';
import { ThreeScene } from '../Components/ThreeScene';
import WinScreen from '../Components/WinScreen';
import { SudokuContext } from '../SudokuContext';

export default function PlayingPage() {
    const [isOpen, { open, close }] = useDisclosure(false);

    const [showVisualizer, setShowVisualizer] = useState(false);

    const [errors, setErrors] = useState<boolean[][]>(
        Array(9)
            .fill(Array(9).fill(0))
            .map((arr) => arr.map(Boolean)),
    );

    const {
        setCurrentPage,
        initialSudoku,
        setInitialSudoku,
        isNoteTaking,
        setIsNoteTaking,
        sudoku,
        setSudoku,
    } = useContext(SudokuContext);

    const returnToMenu = useCallback(() => {
        invoke('reset_state');
        const INIT_ROWS = Array(9).fill(Array(9).fill(0));
        setSudoku(INIT_ROWS);
        setInitialSudoku(INIT_ROWS);
        setErrors(INIT_ROWS.map((arr) => arr.map(Boolean)));
        setCurrentPage(Page.MAIN_MENU);
    }, [setCurrentPage, setSudoku, setInitialSudoku, setErrors]);

    const [visualizerKey, setVisualizerKey] = useState(0);

    function resetVisualizer() {
        setVisualizerKey((k) => k + 1);
    }

    // Listen to sudoku update events
    useEffect(() => {
        const unlisten = listen<{
            grille: number[][];
            grille_initiale: number[][];
            errors: boolean[][];
            is_solved: boolean;
            is_solvable: boolean;
        }>('sudoku', (event) => {
            setSudoku(event.payload.grille);
            setInitialSudoku(event.payload.grille_initiale);
            setErrors(event.payload.errors);
            if (event.payload.is_solved) {
                open();
            }
        });

        return () => {
            unlisten.then((f) => f());
        };
    }, [errors, open, setErrors, setSudoku, setInitialSudoku]);

    return (
        <Center style={{ height: '100vh' }}>
            <WinScreen isOpen={isOpen} onClose={close} />

            <ActionIcon
                variant="transparent"
                size="xl"
                color="gray"
                aria-label="Return to Menu"
                pos="absolute"
                top={5}
                left={5}
                onClick={returnToMenu}
            >
                <IconArrowLeft />
            </ActionIcon>

            <Tooltip label="Solver" withArrow>
                <ActionIcon
                    variant="transparent"
                    size="xl"
                    color="gray"
                    aria-label="Go to Visualizer"
                    pos="absolute"
                    top={5}
                    right={5}
                    onClick={() => setShowVisualizer((v) => !v)}
                >
                    <IconKey />
                </ActionIcon>
            </Tooltip>

            <Tooltip label="indice" withArrow>
                <ActionIcon
                    variant="transparent"
                    size="xl"
                    color="gray"
                    pos="absolute"
                    top={40}
                    right={5}
                    onClick={() => invoke('fill_from_solution')}
                >
                    <IconBulb />
                </ActionIcon>
            </Tooltip>

            <Tooltip label="reinitialiser" withArrow>
                <ActionIcon
                    variant="transparent"
                    size="xl"
                    color="gray"
                    pos="absolute"
                    bottom={40}
                    right={5}
                    onClick={() => {
                        invoke('reset');
                        resetVisualizer();
                    }}
                >
                    <IconRestore />
                </ActionIcon>
            </Tooltip>

            <Tooltip label="sauvegarder" withArrow>
                <ActionIcon
                    variant="transparent"
                    size={70}
                    color="gray"
                    pos="absolute"
                    bottom={5}
                    right={100}
                    onClick={async () => {
                        const id: number = await invoke('import_sudoku', {
                            nom: `autosave-${new Date().toString()}`,
                        });
                        await invoke('sauvegarde_sudoku', { id });
                    }}
                >
                    <IconDeviceFloppy size={70} />
                </ActionIcon>
            </Tooltip>

            <ActionIcon
                variant="transparent"
                size="xl"
                color="gray"
                aria-label="Note Taking Mode"
                pos="absolute"
                bottom={5}
                right={5}
                onClick={() => setIsNoteTaking(!isNoteTaking)}
            >
                {isNoteTaking ? <IconPencil /> : <IconPencilOff />}
            </ActionIcon>

            <Flex direction="row" gap="xl" align="center">
                {/* SUDOKU GRID */}
                <div>
                    <Table
                        withRowBorders
                        withTableBorder
                        withColumnBorders
                        className="sudoku"
                    >
                        <Table.Tbody>
                            {sudoku.map((row, y) => (
                                <Table.Tr key={y}>
                                    {row.map((cell, x) => (
                                        <Table.Td key={x}>
                                            <SudokuCell
                                                value={cell}
                                                col={x}
                                                row={y}
                                                hasError={errors[y][x]}
                                                isDisabled={
                                                    initialSudoku[y][x] !== 0
                                                }
                                            />
                                        </Table.Td>
                                    ))}
                                </Table.Tr>
                            ))}
                        </Table.Tbody>
                    </Table>
                </div>

                {/* Visualizer appears only when button pressed */}
                {showVisualizer && (
                    <div
                        style={{
                            width: '400px',
                            height: '400px',
                            border: '1px solid #333',
                        }}
                    >
                        <ThreeScene key={visualizerKey} />
                    </div>
                )}
            </Flex>
        </Center>
    );
}
