import { Center, Flex, Image, Text, Title } from '@mantine/core';
import { useEffect } from 'react';

interface LandingPageProps {
    onContinue(): void;
}

export default function LandingPage({ onContinue }: LandingPageProps) {
    useEffect(() => {
        window.addEventListener('keydown', onContinue);

        return () => {
            window.removeEventListener('keydown', onContinue);
        };
    }, [onContinue]);

    return (
        <Center style={{ height: '100vh' }} onClick={onContinue}>
            <Flex direction="column" gap="lg">
                <Center>
                    <Image src="/Logo.png" h={100} w={100} />
                </Center>

                <Title ta="center">Complete Sudoku</Title>
                <Text ta="center">
                    Appuyez sur n&apos;importe quelle touche ou cliquez pour
                    commencer
                </Text>
            </Flex>
        </Center>
    );
}
