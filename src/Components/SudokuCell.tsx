import { Button, Popover, Table } from '@mantine/core';
import { useLongPress } from '@mantine/hooks';
import { invoke } from '@tauri-apps/api/core';
import React, { useCallback, useContext, useEffect, useState } from 'react';

import { SudokuContext } from '../SudokuContext';

interface SudokuCellProps {
    value: number;
    col: number;
    row: number;
    isDisabled: boolean;
    hasError: boolean;
}

const SELECT_VALUES = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
];

export default function SudokuCell({
    value,
    col,
    row,
    isDisabled,
    hasError,
}: SudokuCellProps) {
    const [isLongPress, setIsLongPress] = useState(false);
    const [noteNumber, setNoteNumber] = useState('');
    const [opened, setOpened] = useState(false);

    const { isNoteTaking } = useContext(SudokuContext);

    const handlers = useLongPress(
        (e) => {
            if (('button' in e && e.button === 0) || !('button' in e)) {
                setIsLongPress(true);
                setOpened(!opened);
            }
        },
        {
            onCancel: (e) => {
                if (('button' in e && e.button === 0) || !('button' in e)) {
                    setOpened(!opened);
                    setIsLongPress(false);
                }
            },
        },
    );

    const updateCell = useCallback(
        (val: number) => () => {
            if (isLongPress || isNoteTaking) {
                if (value !== 0) {
                    invoke('update_cell', {
                        col,
                        row,
                        val: 0,
                    });
                }
                if (noteNumber.includes(val.toString())) {
                    setNoteNumber(noteNumber.replace(val.toString(), ''));
                }
                else {
                    setNoteNumber(noteNumber + val.toString());
                }
            }
            else {
                invoke('update_cell', {
                    col,
                    row,
                    val: val === value ? 0 : val,
                });
                setNoteNumber('');
            }
            setOpened(false);
            setIsLongPress(false);
        },
        [col, row, value, setOpened, isLongPress, noteNumber, isNoteTaking],
    );

    const clearCell = useCallback(
        (e: React.SyntheticEvent) => {
            e.preventDefault();

            invoke('update_cell', {
                col,
                row,
                val: 0,
            });

            setOpened(false);
            setIsLongPress(false);
            setNoteNumber('');
        },
        [setOpened, col, row],
    );

    const onKeyDown = useCallback(
        (e: KeyboardEvent) => {
            if (e.code.startsWith('Digit')) {
                updateCell(parseInt(e.code.replace('Digit', '')))();
            }
        },
        [updateCell],
    );

    useEffect(() => {
        if (opened) {
            window.addEventListener('keydown', onKeyDown);
        }

        return () => {
            window.removeEventListener('keydown', onKeyDown);
        };
    }, [onKeyDown, opened]);

    return (
        <Popover opened={opened} onChange={setOpened}>
            <Popover.Target>
                <div>
                    {noteNumber === '' || isDisabled ? null : (
                        <Table className="number-note" h="3rem" w="3rem">
                            <Table.Tbody>
                                {SELECT_VALUES.map((row, y) => (
                                    <Table.Tr key={y}>
                                        {row.map((value, x) => (
                                            <Table.Td
                                                key={x}
                                                style={
                                                    noteNumber.includes(
                                                        value.toString(),
                                                    )
                                                        ? { opacity: 1 }
                                                        : { opacity: 0 }
                                                }
                                            >
                                                {value}
                                            </Table.Td>
                                        ))}
                                    </Table.Tr>
                                ))}
                            </Table.Tbody>
                        </Table>
                    )}

                    <Button
                        className={
                            hasError ? 'number-button errored' : 'number-button'
                        }
                        h="3rem"
                        w="3rem"
                        variant="transparent"
                        onContextMenu={clearCell}
                        disabled={isDisabled}
                        style={
                            isDisabled
                                ? { cursor: 'pointer', outline: 'unset' }
                                : { outline: 'unset' }
                        }
                        {...handlers}
                    >
                        {value || ''}
                    </Button>
                </div>
            </Popover.Target>

            <Popover.Dropdown>
                <Table withRowBorders withTableBorder withColumnBorders>
                    <Table.Tbody>
                        {SELECT_VALUES.map((row, y) => (
                            <Table.Tr key={y}>
                                {row.map((value, x) => (
                                    <Table.Td key={x}>
                                        <Button
                                            className="number-button"
                                            h="3rem"
                                            w="3rem"
                                            variant="transparent"
                                            onClick={updateCell(value)}
                                        >
                                            {value}
                                        </Button>
                                    </Table.Td>
                                ))}
                            </Table.Tr>
                        ))}
                    </Table.Tbody>
                </Table>
            </Popover.Dropdown>
        </Popover>
    );
}
