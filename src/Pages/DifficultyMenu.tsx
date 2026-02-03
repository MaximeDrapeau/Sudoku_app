import { ActionIcon, Button, Center, Flex, Stack, Title } from '@mantine/core';
import { IconArrowLeft } from '@tabler/icons-react';
import { invoke } from '@tauri-apps/api/core';
import { useCallback, useContext } from 'react';

import { Page } from '.';
import { SudokuContext } from '../SudokuContext';

export default function DifficultyMenu() {
    const { setCurrentPage, setSudoku, setInitialSudoku } =
        useContext(SudokuContext);

    const returnToMenu = useCallback(() => {
        setCurrentPage(Page.PLAY_CHOICE);
    }, [setCurrentPage]);

    const onPressPlay = useCallback(
        (difficulty: number) => () => {
            invoke('new_sudoku', { difficulty }).then((data) => {
                const sudoku = data as number[][];
                setSudoku(sudoku);
                setInitialSudoku(sudoku);
                setCurrentPage(Page.PLAYING);
            });
        },
        [setCurrentPage, setSudoku, setInitialSudoku],
    );

    return (
        <Center style={{ height: '100vh' }}>
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
            <Flex direction="column" gap="xl">
                <Title>Difficulté</Title>

                <Stack h={300} align="stretch" justify="center" gap="lg">
                    <Button variant="filled" size="lg" onClick={onPressPlay(0)}>
                        Easy
                    </Button>

                    <Button variant="filled" size="lg" onClick={onPressPlay(6)}>
                        Medium
                    </Button>

                    <Button
                        variant="filled"
                        size="lg"
                        onClick={onPressPlay(11)}
                    >
                        Hard
                    </Button>

                    <Button
                        variant="filled"
                        size="lg"
                        onClick={onPressPlay(16)}
                    >
                        Expert
                    </Button>
                </Stack>
            </Flex>
        </Center>
    );
}
