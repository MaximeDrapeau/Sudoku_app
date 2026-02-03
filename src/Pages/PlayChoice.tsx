import { ActionIcon, Button, Center, Flex, Stack, Title } from '@mantine/core';
import { IconArrowLeft } from '@tabler/icons-react';
import { invoke } from '@tauri-apps/api/core';
import { useCallback, useContext } from 'react';

import { Page, Puzzle, Save } from '.';
import { SudokuContext } from '../SudokuContext';

export default function PlayChoice() {
    const { setCurrentPage, setPuzzleList, setSaveList } =
        useContext(SudokuContext);

    const returnToMenu = useCallback(() => {
        setCurrentPage(Page.MAIN_MENU);
    }, [setCurrentPage]);

    const onPressGenerate = useCallback(() => {
        setCurrentPage(Page.DIFF_MENU);
    }, [setCurrentPage]);

    const onPressPuzzleList = useCallback(async () => {
        const list: Puzzle[] = await invoke('display_puzzle_from_db');
        setPuzzleList(list);
        setCurrentPage(Page.PUZZLE_LIST);
    }, [setCurrentPage, setPuzzleList]);

    const onPressSaveList = useCallback(async () => {
        const list: Save[] = await invoke('display_sauvegarde_from_db');
        setSaveList(list);
        setCurrentPage(Page.SAVE_LIST);
    }, [setCurrentPage, setSaveList]);

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
                <Title ta="center">Choisir...</Title>

                <Stack h={300} align="stretch" justify="center" gap="lg">
                    <Button
                        variant="filled"
                        size="lg"
                        onClick={onPressGenerate}
                    >
                        Générer
                    </Button>

                    <Button
                        variant="filled"
                        size="lg"
                        onClick={onPressPuzzleList}
                    >
                        Sudokus Initials
                    </Button>

                    <Button
                        variant="filled"
                        size="lg"
                        onClick={onPressSaveList}
                    >
                        Continue from save
                    </Button>
                </Stack>
            </Flex>
        </Center>
    );
}
