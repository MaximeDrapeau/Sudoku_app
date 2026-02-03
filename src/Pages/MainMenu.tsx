import { Button, Center, Flex, Stack, Title } from '@mantine/core';
import { invoke } from '@tauri-apps/api/core';
import { useCallback, useContext } from 'react';

import { Page } from '.';
import { SudokuContext } from '../SudokuContext';

export default function MainMenu() {
    const { setCurrentPage } = useContext(SudokuContext);

    const onPressPlay = useCallback(() => {
        setCurrentPage(Page.PLAY_CHOICE);
    }, [setCurrentPage]);

    const onPressImport = useCallback(() => {
        setCurrentPage(Page.IMPORT);
    }, [setCurrentPage]);

    const onPressQuit = useCallback(() => {
        invoke('exit');
    }, []);

    return (
        <Center style={{ height: '100vh' }}>
            <Flex direction="column" gap="xl">
                <Title>Menu Principal</Title>

                <Stack h={300} align="stretch" justify="center" gap="lg">
                    <Button variant="filled" size="lg" onClick={onPressPlay}>
                        Jouer
                    </Button>

                    <Button variant="default" size="md" onClick={onPressImport}>
                        Importer une grille
                    </Button>

                    <Button variant="default" size="md" onClick={onPressQuit}>
                        Quitter
                    </Button>
                </Stack>
            </Flex>
        </Center>
    );
}
