import { error } from '@sveltejs/kit';
import { GAMES } from '../../../lib/data/games';
import type { GameType } from 'src/types';

/** @type {import('./$types').PageLoad} */
interface loadParams {
	slug: string;
}
export function load({ params }: { params: loadParams }): GameType {
	const game = GAMES.find((game) => game.slug === params.slug);
	if (game) return game;
	throw error(404, 'Not found');
}
