export const Page = {
    DIFF_MENU: 'diff_menu',
    LANDING: 'landing',
    MAIN_MENU: 'main_menu',
    PLAYING: 'playing',
    IMPORT: 'import',
    PLAY_CHOICE: 'play_choice',
    PUZZLE_LIST: 'puzzle_list',
    SAVE_LIST: 'save_list',
} as const;

export type Page = (typeof Page)[keyof typeof Page];

export type Puzzle = { id: number; nom: string; difficulty: string };

export type Save = {
    id_puzzle: number;
    nom: string;
    difficulty: string;
    date_sauvegarde: string;
};
