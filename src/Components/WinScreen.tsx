import { Center, Modal, Space, Title } from '@mantine/core';

interface WinScreenProps {
    isOpen: boolean;
    onClose: () => void;
}

export default function WinScreen({ isOpen, onClose }: WinScreenProps) {
    return (
        <>
            <Modal opened={isOpen} onClose={onClose}>
                <Center>
                    <Title>You Won!</Title>
                </Center>

                <Space h="lg" />
                <Space h="xl" />
            </Modal>
        </>
    );
}
