import {
    ActionIcon,
    Button,
    Center,
    Flex,
    Modal,
    Space,
    Table,
    TextInput,
    Tooltip,
} from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import {
    IconArrowLeft,
    IconBallTennis,
    IconRestore,
} from '@tabler/icons-react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useCallback, useContext, useEffect, useState } from 'react';

import { Page } from '.';
import SudokuCell from '../Components/SudokuCell';
import { SudokuContext } from '../SudokuContext';

export default function ImportPage() {
    const [isOpen, { open, close: onClose }] = useDisclosure(false);

    const [errors, setErrors] = useState<boolean[][]>(
        Array(9)
            .fill(Array(9).fill(0))
            .map((arr) => arr.map(Boolean)),
    );

    const {
        setCurrentPage,
        initialSudoku,
        setInitialSudoku,
        sudoku,
        setSudoku,
    } = useContext(SudokuContext);

    const [value, setValue] = useState('');
    const [disabled, setDisabled] = useState(true);
    const [status, setStatus] = useState('');

    const onConfirm = useCallback(async () => {
        const status: string = await invoke('import_new_sudoku', {
            nom: value,
        });
        setStatus(status);
    }, [value]);

    const returnToMenu = useCallback(() => {
        setCurrentPage(Page.MAIN_MENU);
    }, [setCurrentPage]);

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
            let nFilled = 0;
            event.payload.grille.forEach((row) => {
                row.forEach((cell) => {
                    if (cell !== 0) {
                        ++nFilled;
                    }
                });
            });
            setDisabled(
                nFilled < 20 ||
                    !event.payload.is_solvable ||
                    event.payload.errors.some((row) =>
                        row.some((error) => error),
                    ),
            );
        });

        return () => {
            unlisten.then((f) => f());
        };
    }, [errors, setErrors, setSudoku, setInitialSudoku]);

    return (
        <Center style={{ height: '100vh' }}>
            <Modal opened={isOpen} onClose={onClose}>
                <TextInput
                    label="Nom du Sudoku"
                    value={value}
                    onChange={(event) => setValue(event.currentTarget.value)}
                />

                <Space h="lg" />

                <Button
                    variant="filled"
                    size="lg"
                    onClick={onConfirm}
                    color={status === 'success import' ? 'green' : undefined}
                >
                    {status === '' && 'Confirm'}
                    {status === 'success import' && 'Success'}
                </Button>
            </Modal>

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
                    }}
                >
                    <IconRestore />
                </ActionIcon>
            </Tooltip>

            <Tooltip label="fill example" withArrow>
                <ActionIcon
                    variant="transparent"
                    size="xl"
                    color="gray"
                    pos="absolute"
                    bottom={80}
                    right={5}
                    onClick={() => {
                        invoke('load_import_example', { id: 1 });
                    }}
                >
                    <IconBallTennis />
                </ActionIcon>
            </Tooltip>

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
                            {sudoku.map((row, y) =>
                                y < 9 ? (
                                    <Table.Tr key={y}>
                                        {row.map((cell, x) => (
                                            <Table.Td key={x}>
                                                <SudokuCell
                                                    value={cell}
                                                    col={x}
                                                    row={y}
                                                    hasError={errors?.[y]?.[x]}
                                                    isDisabled={
                                                        initialSudoku[y][x] !==
                                                        0
                                                    }
                                                />
                                            </Table.Td>
                                        ))}
                                    </Table.Tr>
                                ) : null,
                            )}
                        </Table.Tbody>
                    </Table>
                </div>

                <Button
                    variant="filled"
                    size="lg"
                    onClick={open}
                    disabled={disabled}
                >
                    Import
                </Button>
            </Flex>
        </Center>
    );
}
