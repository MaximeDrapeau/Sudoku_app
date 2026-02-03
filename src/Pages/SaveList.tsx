import {
    ActionIcon,
    Center,
    Flex,
    ScrollArea,
    Space,
    Title,
} from '@mantine/core';
import { Alert } from '@mantine/core';
import { IconArrowLeft } from '@tabler/icons-react';
import { IconInfoCircle } from '@tabler/icons-react';
import { useCallback, useContext } from 'react';

import { Page } from '.';
import Save from '../Components/Save';
import { SudokuContext } from '../SudokuContext';

export default function SaveList() {
    const { setCurrentPage, saveList } = useContext(SudokuContext);

    const returnToChoice = useCallback(() => {
        setCurrentPage(Page.PLAY_CHOICE);
    }, [setCurrentPage]);

    return (
        <Center>
            <Flex direction="column" gap="xl">
                <Space h={100} />
                <Title ta="center">Saves</Title>

                <ActionIcon
                    variant="transparent"
                    size="xl"
                    color="gray"
                    aria-label="Return to Choice"
                    pos="absolute"
                    top={5}
                    left={5}
                    onClick={returnToChoice}
                >
                    <IconArrowLeft />
                </ActionIcon>

                <ScrollArea h={200} type="always">
                    {saveList.length === 0 ? (
                        <Center>
                            <Alert
                                w={200}
                                variant="light"
                                color="blue"
                                title="Alert"
                                icon={<IconInfoCircle />}
                            >
                                No Saves Available
                            </Alert>
                        </Center>
                    ) : null}

                    <Flex direction="column" gap="xl" m={20} miw={300}>
                        {saveList.map((save) => (
                            <Save {...save} />
                        ))}
                    </Flex>
                </ScrollArea>
            </Flex>
        </Center>
    );
}
